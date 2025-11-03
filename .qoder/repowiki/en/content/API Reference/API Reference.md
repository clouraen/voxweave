# API Reference

<cite>
**Referenced Files in This Document**   
- [lib.rs](file://src/lib.rs)
- [tts.rs](file://src/tts.rs)
- [video.rs](file://src/video.rs)
- [pipeline.rs](file://src/pipeline.rs)
- [coqui_tts.rs](file://src/coqui_tts.rs)
- [queue.rs](file://src/queue.rs)
- [Cargo.toml](file://Cargo.toml)
</cite>

## Table of Contents
1. [Introduction](#introduction)
2. [Library Interface](#library-interface)
3. [TTS System](#tts-system)
4. [Video Generation Service](#video-generation-service)
5. [Pipeline Module](#pipeline-module)
6. [Feature Flags and Dependencies](#feature-flags-and-dependencies)
7. [Usage Examples](#usage-examples)
8. [Error Handling](#error-handling)
9. [Async Runtime Requirements](#async-runtime-requirements)

## Introduction
The VoxWeave core library provides a modular framework for text-to-speech (TTS) synthesis, video generation, and pipeline orchestration. This API reference documents the public interfaces, data structures, and usage patterns for integrating VoxWeave into custom applications. The library supports multiple TTS engines, voice cloning, subtitle generation, and video creation through third-party APIs.

## Library Interface

The library exports core modules through `src/lib.rs`, providing access to configuration, pipeline processing, text sanitization, subtitle generation, and TTS functionality. The `video` module is conditionally available when the `video-generation` feature is enabled.

```rust
pub mod config;
pub mod pipeline;
pub mod queue;
pub mod sanitize;
pub mod subtitle;
pub mod text;
pub mod tts;
#[cfg(feature = "coqui-tts")]
pub mod coqui_tts;
#[cfg(feature = "video-generation")]
pub mod video;

pub use pipeline::{ConvertRequest, convert_path};
```

The `convert_path` function serves as the primary entry point for converting text files to audio with subtitles, while `ConvertRequest` encapsulates all processing parameters.

**Section sources**
- [lib.rs](file://src/lib.rs#L1-L13)

## TTS System

### SpeechEngine Trait
The `SpeechEngine` trait defines the interface for all text-to-speech engines. Implementations must provide the `synthesize_to_file` method to convert text to audio.

```rust
pub trait SpeechEngine {
    fn synthesize_to_file(
        &self,
        text: &str,
        voice: &VoiceProfile,
        speed: f32,
        output: &Path,
    ) -> Result<(), TtsError>;
}
```

**Section sources**
- [tts.rs](file://src/tts.rs#L80-L87)

### VoiceProfile Struct
The `VoiceProfile` struct defines voice characteristics for speech synthesis, including engine type, language, and optional command parameters.

```rust
pub struct VoiceProfile {
    pub id: String,
    pub description: String,
    pub engine: VoiceEngine,
    pub command: Option<String>,
    pub lang: Option<String>,
}
```

#### VoiceProfile Methods
- `builtin(id, description)` - Creates an eSpeak voice profile
- `espeak(id, description, command)` - Creates an eSpeak profile with custom command
- `kokoro(id, description, lang)` - Creates a Kokoro voice profile
- `coqui(id, description, lang)` - Creates a Coqui TTS profile
- `coqui_clone(id, description, lang, clone_path)` - Creates a voice clone profile using a WAV file

**Section sources**
- [tts.rs](file://src/tts.rs#L36-L75)

### Supported TTS Engines
The `VoiceEngine` enum specifies available speech synthesis backends:

- `Espeak` - Open-source speech synthesizer
- `Kokoro` - Neural TTS with multilingual support
- `Coqui` - XTTS-based engine with voice cloning

Each engine requires specific configuration through environment variables and implements the `SpeechEngine` trait.

**Section sources**
- [tts.rs](file://src/tts.rs#L15-L34)

### TTS Error Types
The `TtsError` enum defines possible failure modes for speech synthesis:

- `UnsupportedVoice(String)` - Requested voice not available
- `CommandFailed(String)` - External command execution failed
- `Backend(String)` - Audio processing or I/O error

**Section sources**
- [tts.rs](file://src/tts.rs#L5-L13)

## Video Generation Service

### VideoGenerationService Struct
The `VideoGenerationService` struct provides an interface to video generation APIs, currently supporting Z.AI and OpenAI Sora.

```rust
pub struct VideoGenerationService {
    api_key: String,
    base_url: String,
    provider: VideoProvider,
}
```

#### Constructor Methods
- `new(api_key)` - Creates service for Z.AI
- `new_sora(api_key)` - Creates service for OpenAI Sora
- `from_env()` - Creates service using environment variables (OPENAI_API_KEY or ZAI_API_KEY)

**Section sources**
- [video.rs](file://src/video.rs#L19-L50)

### Video Generation Methods
The `generate_video` method orchestrates the complete video creation workflow:

```rust
pub async fn generate_video<F, G>(
    &self,
    audio_path: &Path,
    subtitle_path: Option<&Path>,
    config: &VideoConfig,
    progress_callback: Option<F>,
    log_callback: Option<G>,
) -> anyhow::Result<PathBuf>
```

#### Parameters
- `audio_path` - Path to input audio file
- `subtitle_path` - Optional path to SRT subtitle file
- `config` - Video configuration (style, resolution, format)
- `progress_callback` - Optional function to receive progress updates (0-100%)
- `log_callback` - Optional function to receive log messages

#### Workflow Steps
1. Upload audio file to provider
2. Create video generation job
3. Poll job status with progress updates
4. Download generated video
5. Embed subtitles if provided

**Section sources**
- [video.rs](file://src/video.rs#L52-L122)

### Video Configuration
The `VideoConfig` struct specifies video generation parameters:

```rust
pub struct VideoConfig {
    pub style: VideoStyle,
    pub resolution: VideoResolution,
    pub format: VideoFormat,
    pub prompt: Option<String>,
}
```

#### Video Style Options
- `Realistic` - Photorealistic video
- `Anime` - Animated style
- `ThreeD` - 3D rendering style
- `Cinematic` - Film-like quality
- `Biotech` - Scientific/medical visualization
- `Cyberpunk` - Futuristic neon aesthetic
- `Educational` - Instructional content style

#### Resolution Options
- `P720` - 720p HD resolution
- `P1080` - 1080p Full HD resolution
- `P4K` - 4K Ultra HD resolution

#### Format Options
- `Mp4` - MPEG-4 container
- `Mov` - QuickTime format
- `Webm` - Web-optimized format

**Section sources**
- [video.rs](file://src/video.rs#L10-L17)
- [queue.rs](file://src/queue.rs#L5-L75)

### Video Error Handling
The service returns `anyhow::Result` with descriptive error messages for:
- Authentication failures
- API connectivity issues
- Job timeout (5-minute limit)
- File upload/download failures
- Subtitle embedding errors (ffmpeg required)

**Section sources**
- [video.rs](file://src/video.rs#L124-L250)

## Pipeline Module

### ConvertRequest Struct
The `ConvertRequest` struct defines parameters for text-to-audio conversion:

```rust
pub struct ConvertRequest {
    pub source: PathBuf,
    pub output_dir: PathBuf,
    pub voice: VoiceProfile,
    pub speed: f32,
    pub subtitle_granularity: SubtitleGranularity,
    pub replace_single_newlines: bool,
    pub average_words_per_minute: f32,
}
```

**Section sources**
- [pipeline.rs](file://src/pipeline.rs#L10-L19)

### Processing Functions
#### convert_path
Converts a text file to speech with optional subtitles:

```rust
pub fn convert_path<E: SpeechEngine>(
    engine: &E,
    request: &ConvertRequest,
) -> anyhow::Result<PathBuf>
```

**Section sources**
- [pipeline.rs](file://src/pipeline.rs#L21-L55)

#### convert_queue
Processes multiple conversion requests sequentially:

```rust
pub fn convert_queue<E: SpeechEngine>(
    engine: &E,
    queue: &mut ConversionQueue,
) -> anyhow::Result<Vec<PathBuf>>
```

Returns a vector of output audio file paths.

**Section sources**
- [pipeline.rs](file://src/pipeline.rs#L57-L75)

## Feature Flags and Dependencies

### Available Feature Flags
The library uses Cargo feature flags to control optional functionality:

- `coqui-tts` - Enables Coqui TTS engine with voice cloning
- `video-generation` - Enables video generation service (requires tokio and reqwest)

These features are defined in `Cargo.toml`:

```toml
[features]
default = []
coqui-tts = []
video-generation = ["tokio", "reqwest"]
```

**Section sources**
- [Cargo.toml](file://Cargo.toml#L20-L26)
- [lib.rs](file://src/lib.rs#L7-L8)

### Required Dependencies
- `tokio` with async runtime features for video generation
- `reqwest` for HTTP API calls to video providers
- Python environment with TTS package for Coqui engine
- ffmpeg installed for subtitle embedding

## Usage Examples

### Basic Text-to-Speech
```rust
use voxweave::tts::{EspeakEngine, VoiceProfile};
use voxweave::pipeline::{ConvertRequest, convert_path};

let engine = EspeakEngine::default();
let request = ConvertRequest {
    source: "input.txt".into(),
    output_dir: "output/".into(),
    voice: VoiceProfile::builtin("en-us", "US English"),
    speed: 1.0,
    subtitle_granularity: SubtitleGranularity::Sentence,
    replace_single_newlines: false,
    average_words_per_minute: 150.0,
};

let audio_path = convert_path(&engine, &request)?;
```

**Section sources**
- [pipeline.rs](file://src/pipeline.rs#L21-L55)
- [tts.rs](file://src/tts.rs#L36-L75)

### Video Generation with Subtitles
```rust
use voxweave::video::{VideoGenerationService, VideoConfig, VideoStyle, VideoResolution, VideoFormat};
use std::path::Path;

let service = VideoGenerationService::from_env()?;
let config = VideoConfig {
    style: VideoStyle::Cinematic,
    resolution: VideoResolution::P1080,
    format: VideoFormat::Mp4,
    prompt: Some("A cinematic scene with dramatic lighting".to_string()),
};

let video_path = service.generate_video(
    Path::new("audio.wav"),
    Some(Path::new("subtitles.srt")),
    &config,
    Some(|progress| println!("Progress: {}%", progress)),
    Some(|msg, level| println!("[{}] {}", level.as_str(), msg)),
).await?;
```

**Section sources**
- [video.rs](file://src/video.rs#L52-L122)

### Voice Cloning with Coqui TTS
```rust
// Requires coqui-tts feature
#[cfg(feature = "coqui-tts")]
{
    use voxweave::tts::{CoquiEngine, VoiceProfile};
    
    let engine = CoquiEngine::default();
    let voice = VoiceProfile::coqui_clone(
        "my_voice",
        "My cloned voice",
        "en",
        "/path/to/reference.wav"
    );
    
    engine.synthesize_to_file(
        "Hello, this is my cloned voice.",
        &voice,
        1.0,
        Path::new("output.wav")
    )?;
}
```

**Section sources**
- [coqui_tts.rs](file://src/coqui_tts.rs#L1-L115)
- [tts.rs](file://src/tts.rs#L36-L75)

## Error Handling

### TTS Error Handling
Handle speech synthesis errors with pattern matching:

```rust
match engine.synthesize_to_file(&text, &voice, speed, &output_path) {
    Ok(()) => println!("Audio generated successfully"),
    Err(TtsError::UnsupportedVoice(id)) => eprintln!("Voice not supported: {}", id),
    Err(TtsError::CommandFailed(msg)) => eprintln!("Command failed: {}", msg),
    Err(TtsError::Backend(msg)) => eprintln!("Backend error: {}", msg),
}
```

### Video Generation Error Handling
Use Result propagation for video operations:

```rust
match service.generate_video(...).await {
    Ok(path) => println!("Video created at: {}", path.display()),
    Err(e) => eprintln!("Video generation failed: {}", e),
}
```

Common error scenarios include:
- Invalid API keys
- Network connectivity issues
- File permission errors
- Provider rate limiting
- Timeout during long-running jobs

**Section sources**
- [tts.rs](file://src/tts.rs#L5-L13)
- [video.rs](file://src/video.rs#L124-L250)

## Async Runtime Requirements

The video generation module requires an async runtime. When using `video-generation` feature, ensure tokio runtime is available:

```toml
[dependencies]
tokio = { version = "1", features = ["rt", "rt-multi-thread", "fs", "io-util", "time", "process"] }
```

Initialize the runtime in your application:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Your code here
    Ok(())
}
```

Or use a runtime handle for more control:

```rust
let runtime = tokio::runtime::Runtime::new()?;
runtime.block_on(async {
    // Async operations
});
```

**Section sources**
- [Cargo.toml](file://Cargo.toml#L16-L17)
- [video.rs](file://src/video.rs#L52-L122)