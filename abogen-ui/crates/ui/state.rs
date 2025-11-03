use dioxus::prelude::*;

/// Represents a file that can be processed
#[derive(Debug, Clone, PartialEq)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
}

/// Represents a queued item with all processing options
#[derive(Debug, Clone)]
pub struct QueuedItem {
    pub file: FileInfo,
    pub voice: String,
    pub speed: f64,
    pub subtitle_mode: SubtitleMode,
    pub voice_format: VoiceFormat,
    pub subtitle_format: SubtitleFormat,
    pub replace_newlines: bool,
    pub use_gpu: bool,
    // Video generation options
    pub generate_video: bool,
    pub video_style: VideoStyle,
    pub video_resolution: VideoResolution,
    pub video_format: VideoFormat,
    pub video_prompt: Option<String>,
    pub save_location: SaveLocation,
}

/// Subtitle generation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubtitleMode {
    Sentence,
    Paragraph,
    None,
}

impl SubtitleMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            SubtitleMode::Sentence => "Sentence",
            SubtitleMode::Paragraph => "Paragraph",
            SubtitleMode::None => "None",
        }
    }
}

/// Voice output format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoiceFormat {
    Wav,
    Mp3,
    Flac,
}

impl VoiceFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            VoiceFormat::Wav => "wav",
            VoiceFormat::Mp3 => "mp3",
            VoiceFormat::Flac => "flac",
        }
    }
}

/// Subtitle output format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubtitleFormat {
    Ass,
    Srt,
    Vtt,
}

impl SubtitleFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            SubtitleFormat::Ass => "ass",
            SubtitleFormat::Srt => "srt",
            SubtitleFormat::Vtt => "vtt",
        }
    }
}

/// Video generation style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoStyle {
    Realistic,
    Anime,
    ThreeD,
    Cinematic,
    Biotech,
    Cyberpunk,
    Educational,
    Wan2_5,
}

impl VideoStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            VideoStyle::Realistic => "realistic",
            VideoStyle::Anime => "anime",
            VideoStyle::ThreeD => "3d",
            VideoStyle::Cinematic => "cinematic",
            VideoStyle::Biotech => "biotech",
            VideoStyle::Cyberpunk => "cyberpunk",
            VideoStyle::Educational => "educational",
            VideoStyle::Wan2_5 => "wan2.5-t2v-preview",
        }
    }
    
    pub fn from_str(s: &str) -> Self {
        match s {
            "realistic" => VideoStyle::Realistic,
            "anime" => VideoStyle::Anime,
            "3d" => VideoStyle::ThreeD,
            "cinematic" => VideoStyle::Cinematic,
            "biotech" => VideoStyle::Biotech,
            "cyberpunk" => VideoStyle::Cyberpunk,
            "educational" => VideoStyle::Educational,
            "wan2.5-t2v-preview" => VideoStyle::Wan2_5,
            _ => VideoStyle::Realistic,
        }
    }
}

/// Video resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoResolution {
    P720,
    P1080,
    P4K,
}

impl VideoResolution {
    pub fn as_str(&self) -> &'static str {
        match self {
            VideoResolution::P720 => "720p",
            VideoResolution::P1080 => "1080p",
            VideoResolution::P4K => "4k",
        }
    }
    
    pub fn from_str(s: &str) -> Self {
        match s {
            "720p" => VideoResolution::P720,
            "1080p" => VideoResolution::P1080,
            "4k" => VideoResolution::P4K,
            _ => VideoResolution::P1080,
        }
    }
}

/// Video output format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoFormat {
    Mp4,
    Mov,
    Webm,
}

impl VideoFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            VideoFormat::Mp4 => "mp4",
            VideoFormat::Mov => "mov",
            VideoFormat::Webm => "webm",
        }
    }
    
    pub fn from_str(s: &str) -> Self {
        match s {
            "mp4" => VideoFormat::Mp4,
            "mov" => VideoFormat::Mov,
            "webm" => VideoFormat::Webm,
            _ => VideoFormat::Mp4,
        }
    }
}

/// Save location preference
#[derive(Debug, Clone, PartialEq)]
pub enum SaveLocation {
    Desktop,
    Custom(String),
}

/// Main application state
#[derive(Clone, PartialEq)]
pub struct AppState {
    pub selected_file: Signal<Option<FileInfo>>,
    pub queue: Signal<Vec<QueuedItem>>,
    pub voice: Signal<String>,
    pub speed: Signal<f64>,
    pub subtitle_mode: Signal<SubtitleMode>,
    pub voice_format: Signal<VoiceFormat>,
    pub subtitle_format: Signal<SubtitleFormat>,
    pub replace_newlines: Signal<bool>,
    pub save_location: Signal<SaveLocation>,
    pub use_gpu: Signal<bool>,
    // Video generation state
    pub generate_video: Signal<bool>,
    pub video_style: Signal<VideoStyle>,
    pub video_resolution: Signal<VideoResolution>,
    pub video_format: Signal<VideoFormat>,
    pub video_prompt: Signal<Option<String>>,
    // Processing state
    pub is_processing: Signal<bool>,
    pub progress: Signal<u8>,
    pub logs: Signal<Vec<LogEntry>>,
    pub cancel_token: Signal<Option<()>>,
}

impl AppState {
    pub fn new() -> Self {
        let use_gpu_default = cfg!(feature = "gpu");
        Self {
            selected_file: use_signal(|| None),
            queue: use_signal(|| Vec::new()),
            voice: use_signal(|| "af_heart".to_string()),
            speed: use_signal(|| 1.0),
            subtitle_mode: use_signal(|| SubtitleMode::Sentence),
            voice_format: use_signal(|| VoiceFormat::Wav),
            subtitle_format: use_signal(|| SubtitleFormat::Ass),
            replace_newlines: use_signal(|| true),
            save_location: use_signal(|| SaveLocation::Desktop),
            use_gpu: use_signal(|| use_gpu_default),
            generate_video: use_signal(|| false),
            video_style: use_signal(|| VideoStyle::Realistic),
            video_resolution: use_signal(|| VideoResolution::P1080),
            video_format: use_signal(|| VideoFormat::Mp4),
            video_prompt: use_signal(|| None),
            is_processing: use_signal(|| false),
            progress: use_signal(|| 0),
            logs: use_signal(|| Vec::new()),
            cancel_token: use_signal(|| None),
        }
    }

}

/// Log entry for processing output
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub message: String,
    pub level: LogLevel,
}

/// Log level for coloring
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Notice,
    Error,
}

