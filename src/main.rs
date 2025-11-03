use std::path::{Path, PathBuf};

use voxweave::config::{AppConfig, load_config, save_config};
use voxweave::pipeline::{ConvertRequest, convert_path};
use voxweave::queue::SubtitleGranularity;
use voxweave::tts::{
    EspeakEngine, KokoroEngine, MockSpeechEngine, VoiceEngine, VoiceProfile,
    default_voice_profiles, find_voice,
};
#[cfg(feature = "coqui-tts")]
use voxweave::coqui_tts::CoquiEngine;
use clap::{Args, Parser, Subcommand, ValueEnum};

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Convert(args) => handle_convert(args),
        Commands::ListVoices => handle_list_voices(),
        #[cfg(feature = "video-generation")]
        Commands::Video(args) => handle_video(args),
    }
}

#[derive(Parser)]
#[command(name = "voxweave", version, about = "Rust text-to-speech pipeline CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Convert(ConvertArgs),
    ListVoices,
    #[cfg(feature = "video-generation")]
    Video(VideoArgs),
}

#[cfg(feature = "video-generation")]
#[derive(Args)]
struct VideoArgs {
    /// Input text/markdown file
    input: PathBuf,
    /// Output directory (defaults to the same directory as the input file)
    #[arg(short, long)]
    output: Option<PathBuf>,
    /// Voice identifier (use `list-voices` to see options)
    #[arg(short, long)]
    voice: Option<String>,
    /// Video style (realistic, anime, 3d, cinematic, biotech, cyberpunk, educational)
    #[arg(long, default_value = "cyberpunk")]
    style: String,
    /// Video resolution (720p, 1080p, 4k)
    #[arg(long, default_value = "1080p")]
    resolution: String,
    /// Video format (mp4, mov, webm)
    #[arg(long, default_value = "mp4")]
    format: String,
    /// Custom visual prompt for video generation
    #[arg(long)]
    prompt: Option<String>,
    /// Playback speed multiplier (1.0 = normal)
    #[arg(short, long, default_value_t = 1.0)]
    speed: f32,
}

#[derive(Args)]
struct ConvertArgs {
    /// Input text/markdown file
    input: PathBuf,
    /// Output directory (defaults to the same directory as the input file)
    #[arg(short, long)]
    output: Option<PathBuf>,
    /// Playback speed multiplier (1.0 = normal)
    #[arg(short, long, default_value_t = 1.0)]
    speed: f32,
    /// Voice identifier (use `list-voices` to see options)
    #[arg(short, long)]
    voice: Option<String>,
    /// Subtitle style
    #[arg(long, value_enum, default_value = "sentence")]
    subtitles: SubtitleMode,
    /// Number of words per subtitle when using `--subtitles words`
    #[arg(long, default_value_t = 3)]
    words: u8,
    /// Force replacing single newlines with spaces
    #[arg(long, conflicts_with = "keep_single_newlines")]
    replace_single_newlines: bool,
    /// Force keeping single newlines (overrides config)
    #[arg(long, conflicts_with = "replace_single_newlines")]
    keep_single_newlines: bool,
    /// Average words-per-minute for subtitle timing
    #[arg(long, default_value_t = 150.0)]
    wpm: f32,
    /// Use the mock engine (writes cleaned text instead of generating audio)
    #[arg(long)]
    mock: bool,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum SubtitleMode {
    Disabled,
    Sentence,
    Words,
}

fn handle_convert(args: ConvertArgs) -> anyhow::Result<()> {
    let voices = default_voice_profiles();
    let mut config = load_config().unwrap_or_default();

    let voice = select_voice(&voices, &mut config, args.voice.as_deref())
        .ok_or_else(|| anyhow::anyhow!("voice not found; run `voxweave list-voices`"))?
        .clone();

    let output_dir = args
        .output
        .unwrap_or_else(|| default_output_dir(&args.input));

    let subtitle_granularity = match args.subtitles {
        SubtitleMode::Disabled => SubtitleGranularity::Disabled,
        SubtitleMode::Sentence => SubtitleGranularity::Sentence,
        SubtitleMode::Words => SubtitleGranularity::Words(args.words.max(1)),
    };

    let replace_single_newlines = if args.replace_single_newlines {
        true
    } else if args.keep_single_newlines {
        false
    } else {
        config.replace_single_newlines
    };

    let request = ConvertRequest {
        source: args.input.clone(),
        output_dir: output_dir.clone(),
        voice: voice.clone(),
        speed: args.speed,
        subtitle_granularity,
        replace_single_newlines,
        average_words_per_minute: args.wpm,
    };

    let audio_path = if args.mock {
        let engine = MockSpeechEngine::default();
        convert_path(&engine, &request)?
    } else {
        match voice.engine {
            VoiceEngine::Espeak => {
                let engine = EspeakEngine::default();
                convert_path(&engine, &request)?
            }
            VoiceEngine::Kokoro => {
                let engine = KokoroEngine::default();
                convert_path(&engine, &request)?
            }
            #[cfg(feature = "coqui-tts")]
            VoiceEngine::Coqui => {
                let engine = CoquiEngine::default();
                convert_path(&engine, &request)?
            }
            #[cfg(not(feature = "coqui-tts"))]
            VoiceEngine::Coqui => {
                anyhow::bail!("CoquiTTS support not enabled. Build with --features coqui-tts")
            }
        }
    };

    update_config(&mut config, &voice, &args.input, replace_single_newlines)?;

    println!("✓ Audio saved to {}", audio_path.display());
    if subtitle_granularity != SubtitleGranularity::Disabled {
        let subtitle_path = audio_path.with_extension("srt");
        if subtitle_path.exists() {
            println!("✓ Subtitles saved to {}", subtitle_path.display());
        }
    }
    Ok(())
}

fn handle_list_voices() -> anyhow::Result<()> {
    let voices = default_voice_profiles();
    println!("Available voices:");
    for voice in voices {
        println!(
            "  {:<16} {:<8} {}",
            voice.id,
            voice.engine.as_str(),
            voice.description
        );
    }
    Ok(())
}

fn select_voice<'a>(
    voices: &'a [VoiceProfile],
    config: &mut AppConfig,
    requested: Option<&str>,
) -> Option<&'a VoiceProfile> {
    if let Some(id) = requested {
        return find_voice(voices, id);
    }
    if let Some(default_id) = &config.default_voice {
        if let Some(voice) = find_voice(voices, default_id) {
            return Some(voice);
        }
    }
    voices.first()
}

fn default_output_dir(input: &Path) -> PathBuf {
    input
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."))
}

fn update_config(
    config: &mut AppConfig,
    voice: &VoiceProfile,
    input: &Path,
    replace_single_newlines: bool,
) -> anyhow::Result<()> {
    config.default_voice = Some(voice.id.clone());
    config.replace_single_newlines = replace_single_newlines;

    if let Some(path_str) = input.to_str() {
        config.recent_files.retain(|item| item != path_str);
        config.recent_files.insert(0, path_str.to_string());
        if config.recent_files.len() > 10 {
            config.recent_files.truncate(10);
        }
    }

    save_config(config)
}

#[cfg(feature = "video-generation")]
fn handle_video(args: VideoArgs) -> anyhow::Result<()> {
    use std::fs;
    use voxweave::pipeline::ConvertRequest;
    use voxweave::queue::SubtitleGranularity;
    use voxweave::video::{VideoConfig, VideoGenerationService};
    use voxweave::queue::LogLevel;

    let voices = default_voice_profiles();
    let mut config = load_config().unwrap_or_default();

    let voice = select_voice(&voices, &mut config, args.voice.as_deref())
        .ok_or_else(|| anyhow::anyhow!("voice not found; run `voxweave list-voices`"))?
        .clone();

    let output_dir = args
        .output
        .unwrap_or_else(|| default_output_dir(&args.input));

    // Read input text
    let _text_content = fs::read_to_string(&args.input)
        .map_err(|e| anyhow::anyhow!("Failed to read input file: {}", e))?;

    // Step 1: Generate audio with subtitles
    println!("📝 Generating audio from text...");
    let request = ConvertRequest {
        source: args.input.clone(),
        output_dir: output_dir.clone(),
        voice: voice.clone(),
        speed: args.speed,
        subtitle_granularity: SubtitleGranularity::Sentence,
        replace_single_newlines: config.replace_single_newlines,
        average_words_per_minute: 150.0,
    };

    let audio_path = match voice.engine {
        VoiceEngine::Espeak => {
            let engine = EspeakEngine::default();
            convert_path(&engine, &request)?
        }
        VoiceEngine::Kokoro => {
            let engine = KokoroEngine::default();
            convert_path(&engine, &request)?
        }
        #[cfg(feature = "coqui-tts")]
        VoiceEngine::Coqui => {
            let engine = CoquiEngine::default();
            convert_path(&engine, &request)?
        }
        #[cfg(not(feature = "coqui-tts"))]
        VoiceEngine::Coqui => {
            anyhow::bail!("CoquiTTS support not enabled. Build with --features coqui-tts")
        }
    };

    let subtitle_path = audio_path.with_extension("srt");
    println!("✓ Audio saved to {}", audio_path.display());
    if subtitle_path.exists() {
        println!("✓ Subtitles saved to {}", subtitle_path.display());
    }

    // Step 2: Generate video using runtime
    println!("🎬 Generating {} video at {}...", args.style, args.resolution);
    println!("⚠️  Video generation requires ZAI_API_KEY environment variable");
    println!("   Set it with: export ZAI_API_KEY=your_api_key_here");
    
    // Use tokio runtime to run async video generation
    let runtime = tokio::runtime::Runtime::new()?;
    let video_result = runtime.block_on(async {
        generate_video_cli(
            &audio_path,
            if subtitle_path.exists() { Some(&subtitle_path) } else { None },
            &args.style,
            &args.resolution,
            &args.format,
            args.prompt.as_deref(),
        ).await
    });

    match video_result {
        Ok(video_path) => {
            println!("✓ Video saved to {}", video_path.display());
            let replace_newlines = config.replace_single_newlines;
            update_config(&mut config, &voice, &args.input, replace_newlines)?;
            Ok(())
        }
        Err(e) => {
            eprintln!("✗ Video generation failed: {}", e);
            eprintln!("  Make sure ZAI_API_KEY is set and valid");
            Err(anyhow::anyhow!("Video generation failed: {}", e))
        }
    }
}

#[cfg(feature = "video-generation")]
async fn generate_video_cli(
    audio_path: &Path,
    subtitle_path: Option<&Path>,
    style: &str,
    resolution: &str,
    format: &str,
    prompt: Option<&str>,
) -> anyhow::Result<PathBuf> {
    use voxweave::video::{VideoConfig, VideoGenerationService};
    use voxweave::queue::{VideoStyle, VideoResolution, VideoFormat, LogLevel};
    
    // Parse style
    let video_style = match style.to_lowercase().as_str() {
        "realistic" => VideoStyle::Realistic,
        "anime" => VideoStyle::Anime,
        "3d" => VideoStyle::ThreeD,
        "cinematic" => VideoStyle::Cinematic,
        "biotech" => VideoStyle::Biotech,
        "cyberpunk" => VideoStyle::Cyberpunk,
        "educational" => VideoStyle::Educational,
        _ => VideoStyle::Cyberpunk,
    };

    // Parse resolution
    let video_resolution = match resolution.to_lowercase().as_str() {
        "720p" => VideoResolution::P720,
        "1080p" => VideoResolution::P1080,
        "4k" => VideoResolution::P4K,
        _ => VideoResolution::P1080,
    };

    // Parse format
    let video_format = match format.to_lowercase().as_str() {
        "mp4" => VideoFormat::Mp4,
        "mov" => VideoFormat::Mov,
        "webm" => VideoFormat::Webm,
        _ => VideoFormat::Mp4,
    };

    // Create video configuration
    let config = VideoConfig {
        style: video_style,
        resolution: video_resolution,
        format: video_format,
        prompt: prompt.map(|s| s.to_string()),
    };

    // Initialize video generation service
    let service = VideoGenerationService::from_env()?;

    // CLI progress callback
    let progress_callback = |progress: u8| {
        print!("\r   Progress: {}%", progress);
        use std::io::Write;
        std::io::stdout().flush().ok();
    };

    // CLI log callback
    let log_callback = |message: &str, level: LogLevel| {
        match level {
            LogLevel::Info => println!("   {}", message),
            LogLevel::Notice => println!("   ℹ️  {}", message),
            LogLevel::Warning => println!("   ⚠️  {}", message),
            LogLevel::Error => eprintln!("   ❌ {}", message),
        }
    };

    // Generate video
    let video_path = service.generate_video(
        audio_path,
        subtitle_path,
        &config,
        Some(progress_callback),
        Some(log_callback),
    ).await?;

    println!(); // New line after progress
    Ok(video_path)
}
