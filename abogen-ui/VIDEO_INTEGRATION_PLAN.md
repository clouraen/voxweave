# Video Creation Integration Plan with Z.ai Video Models

## Overview

Integrating Z.ai's video generation models ([z.ai/model-api](https://z.ai/model-api)) with the existing TTS/text-to-speech pipeline to create a complete text-to-video workflow.

## Integration Architecture

### Workflow: Text → TTS Audio → Video Generation

```
Text File
    ↓
[Text Processing]
    ↓
[TTS Synthesis] → Audio File (.wav/.mp3/.flac)
    ↓
[Subtitle Generation] → Subtitle File (.srt/.ass/.vtt)
    ↓
[Video Generation] → Video File (.mp4/.mov)
    ↓
Final Output: Video with synchronized audio and subtitles
```

## Implementation Plan

### 1. State Extensions

Add video generation options to `QueuedItem`:

```rust
pub struct QueuedItem {
    // ... existing fields ...
    
    // Video generation options
    pub generate_video: bool,
    pub video_style: VideoStyle,      // Realistic, Anime, 3D, etc.
    pub video_resolution: VideoResolution, // 720p, 1080p, 4K
    pub video_duration: Option<u32>,   // Max duration in seconds
    pub video_prompt: Option<String>,  // Custom video generation prompt
}
```

### 2. New UI Components

Add to MainScreen:
- **"Generate Video"** checkbox
- **Video Style** dropdown (Realistic, Anime, 3D, Cinematic, etc.)
- **Video Resolution** dropdown (720p, 1080p, 4K)
- **Custom Video Prompt** text area (optional)
- **Video Output Format** dropdown (mp4, mov, webm)

### 3. Service Implementation

Create `services/video_generation.rs`:

```rust
pub struct VideoGenerationService {
    api_key: String,
    client: reqwest::Client,
}

impl VideoGenerationService {
    pub async fn generate_video(
        &self,
        audio_path: &Path,
        subtitle_path: Option<&Path>,
        video_config: VideoConfig,
        progress_callback: impl Fn(u8),
    ) -> Result<PathBuf, String> {
        // 1. Upload audio file to Z.ai (if needed)
        // 2. Create video generation request
        // 3. Poll for completion with progress updates
        // 4. Download generated video
        // 5. Combine with subtitles if needed
    }
}
```

### 4. Pipeline Integration

Update `services/tts_stub.rs` or create `services/pipeline.rs`:

```rust
pub async fn process_queue_with_video(
    state: AppState,
    items: Vec<QueuedItem>,
) -> Result<(), String> {
    for item in items {
        // Step 1: TTS Processing (existing)
        let audio_path = tts_process(&item)?;
        
        // Step 2: Subtitle Generation (existing)
        let subtitle_path = generate_subtitles(&item)?;
        
        // Step 3: Video Generation (NEW)
        if item.generate_video {
            let video_path = video_generation_service
                .generate_video(&audio_path, subtitle_path.as_ref(), &item)
                .await?;
            
            // Log completion
            state.logs.write().push(LogEntry {
                message: format!("Video generated: {}", video_path.display()),
                level: LogLevel::Info,
            });
        }
    }
}
```

### 5. API Integration

Z.ai Video Generation API Pattern:

```rust
// Example API call structure
async fn create_video_job(
    audio_url: &str,
    video_prompt: &str,
    style: &str,
    resolution: &str,
) -> Result<String, Error> {
    let response = reqwest::Client::new()
        .post("https://api.z.ai/v1/video/generate")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "audio_url": audio_url,
            "prompt": video_prompt,
            "style": style,
            "resolution": resolution,
            "model": "video-generation-model-name"
        }))
        .send()
        .await?;
    
    let job_id = response.json::<VideoJobResponse>().await?.job_id;
    Ok(job_id)
}

async fn poll_video_status(job_id: &str) -> Result<VideoStatus, Error> {
    // Poll Z.ai API for job status
    // Return progress percentage and download URL when complete
}
```

## User Experience Flow

1. **User selects text file** (existing)
2. **User configures TTS settings** (existing)
3. **User enables "Generate Video"** checkbox (NEW)
4. **User selects video style and resolution** (NEW)
5. **User clicks START** (existing)
6. **Processing Screen shows**:
   - "Generating audio..." (existing)
   - "Generating subtitles..." (existing)
   - "Generating video..." (NEW)
   - "Combining video with audio..." (NEW)
   - "Rendering final video..." (NEW)
7. **Output**: Video file with synchronized audio and subtitles

## Technical Considerations

### API Requirements
- Z.ai API key (environment variable or config)
- HTTP client (reqwest or ureq)
- Async video generation with progress polling
- File upload/download handling

### Dependencies to Add

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json", "multipart"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
```

### Video Processing Pipeline

1. **TTS Phase**: Generate audio from text
2. **Subtitle Phase**: Generate subtitles with timings
3. **Video Generation Phase**:
   - Upload audio to Z.ai (or provide URL)
   - Create video generation job with:
     - Audio reference
     - Visual style prompt
     - Resolution settings
     - Custom prompt (optional)
   - Poll for completion (stream progress)
   - Download generated video
4. **Post-Processing Phase**:
   - Combine video with audio (ensure sync)
   - Burn subtitles into video (if needed)
   - Apply any final effects

## UI Updates Needed

### MainScreen Additions

```rust
// New section in MainScreen
div {
    class: "panel",
    style: "margin-top: 20px;",
    CheckBox {
        checked: state.generate_video,
        label: "Generate Video with Z.ai",
    }
    
    if *state.generate_video.read() {
        // Video configuration options
        select {
            class: "select",
            value: "{state.video_style.read().as_str()}",
            onchange: move |e| {
                // Update video style
            },
            option { value: "realistic", "Realistic" }
            option { value: "anime", "Anime" }
            option { value: "3d", "3D" }
            option { value: "cinematic", "Cinematic" }
        }
        
        select {
            class: "select",
            value: "{state.video_resolution.read().as_str()}",
            onchange: move |e| {
                // Update resolution
            },
            option { value: "720p", "720p HD" }
            option { value: "1080p", "1080p Full HD" }
            option { value: "4k", "4K Ultra HD" }
        }
        
        textarea {
            class: "combo-input",
            placeholder: "Custom video generation prompt (optional)",
            value: "{state.video_prompt.read()}",
            oninput: move |e| {
                *state.video_prompt.write() = e.value().clone();
            },
            style: "min-height: 80px; resize: vertical;"
        }
    }
}
```

## Benefits of Integration

1. **Unified Workflow**: Text-to-speech and text-to-video in one application
2. **Automated Pipeline**: No need to manually combine audio with video
3. **Synchronized Output**: Audio, subtitles, and video all in sync
4. **Professional Results**: Leverage Z.ai's advanced video generation models
5. **Batch Processing**: Queue multiple files for video generation

## Implementation Steps

1. ✅ Add video generation fields to `AppState` and `QueuedItem`
2. ✅ Create `video_generation.rs` service with Z.ai API integration
3. ✅ Add video configuration UI components
4. ✅ Integrate video generation into processing pipeline
5. ✅ Add progress tracking for video generation phase
6. ✅ Handle video file downloads and storage
7. ✅ Add error handling and retry logic
8. ✅ Update logging to show video generation progress

## API Documentation Reference

- Z.ai Video Generation API: [z.ai/model-api](https://z.ai/model-api)
- Likely endpoints:
  - `POST /v1/video/generate` - Create video generation job
  - `GET /v1/video/status/{job_id}` - Check job status
  - `GET /v1/video/download/{job_id}` - Download completed video

## Security Considerations

- Store API key securely (environment variable, not in code)
- Validate video generation requests
- Set rate limits to avoid API abuse
- Handle API errors gracefully
- Implement retry logic with exponential backoff

## Cost Considerations

- Z.ai API likely has usage-based pricing
- Video generation may be more expensive than TTS
- Consider showing estimated costs before generation
- Implement budget/warning thresholds

## Next Steps

1. Research Z.ai API documentation for exact endpoints
2. Get API key and test connection
3. Implement basic video generation service
4. Integrate into existing pipeline
5. Test end-to-end workflow
6. Add UI controls for video settings

---

This integration would transform abogen from a TTS tool into a complete **text-to-video content creation platform**, significantly expanding its capabilities and use cases.

