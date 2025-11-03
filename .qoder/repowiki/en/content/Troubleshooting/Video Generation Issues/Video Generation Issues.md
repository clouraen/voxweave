# Video Generation Issues

<cite>
**Referenced Files in This Document**   
- [CLI_VIDEO_GENERATION.md](file://CLI_VIDEO_GENERATION.md)
- [VIDEO_CLI_IMPLEMENTATION.md](file://VIDEO_CLI_IMPLEMENTATION.md)
- [generate_video.sh](file://generate_video.sh)
- [src/video.rs](file://src/video.rs)
- [abogen-ui/crates/ui/services/zai_video.rs](file://abogen-ui/crates/ui/services/zai_video.rs)
- [abogen-ui/crates/ui/services/video_generation.rs](file://abogen-ui/crates/ui/services/video_generation.rs)
- [abogen-ui/crates/ui/services/mlt_video.rs](file://abogen-ui/crates/ui/services/mlt_video.rs)
</cite>

## Table of Contents
1. [Introduction](#introduction)
2. [Z.AI API Integration Issues](#zai-api-integration-issues)
3. [Video Style and Resolution Problems](#video-style-and-resolution-problems)
4. [Custom Prompt Handling](#custom-prompt-handling)
5. [Subtitle and Audio Synchronization](#subtitle-and-audio-synchronization)
6. [Script Execution and Format Conversion](#script-execution-and-format-conversion)
7. [Timeout and Large File Handling](#timeout-and-large-file-handling)
8. [API Request/Response Debugging](#api-requestresponse-debugging)

## Introduction
This document addresses common issues encountered in VoxWeave's AI video pipeline, focusing on troubleshooting video generation problems. The analysis covers Z.AI API integration challenges, video style selection, resolution settings, custom prompt handling, subtitle embedding, and script execution issues. The documentation provides solutions for format conversion failures and timeout errors during video generation.

**Section sources**
- [CLI_VIDEO_GENERATION.md](file://CLI_VIDEO_GENERATION.md#L1-L324)
- [VIDEO_CLI_IMPLEMENTATION.md](file://VIDEO_CLI_IMPLEMENTATION.md#L1-L335)

## Z.AI API Integration Issues

### Authentication Failures
Authentication issues occur when the Z.AI API key is not properly configured. The system checks for both `ZAI_API_KEY` and `OPENAI_API_KEY` environment variables, with Z.AI being the fallback option.

```mermaid
flowchart TD
Start([Start Video Generation]) --> CheckAPIKey["Check for OPENAI_API_KEY"]
CheckAPIKey --> |Found| InitializeSora["Initialize OpenAI Sora Service"]
CheckAPIKey --> |Not Found| CheckZAIKey["Check for ZAI_API_KEY"]
CheckZAIKey --> |Found| InitializeZAI["Initialize Z.AI Service"]
CheckZAIKey --> |Not Found| AuthenticationError["Return Authentication Error"]
AuthenticationError --> DisplayError["Display: 'Neither OPENAI_API_KEY nor ZAI_API_KEY environment variable is set'"]
```

**Diagram sources**
- [src/video.rs](file://src/video.rs#L45-L81)
- [abogen-ui/crates/ui/services/zai_video.rs](file://abogen-ui/crates/ui/services/zai_video.rs#L49-L94)

### Rate Limiting and Invalid API Keys
The system implements retry logic with exponential backoff to handle rate limiting. Invalid API keys result in immediate authentication errors with clear messaging.

```mermaid
sequenceDiagram
participant CLI as "CLI Interface"
participant Service as "VideoGenerationService"
participant API as "Z.AI API"
CLI->>Service : generate_video() with audio_path
Service->>API : POST /upload with audio file
API-->>Service : 401 Unauthorized
Service->>Service : Log error : "Failed to send request to Z.AI"
Service-->>CLI : Return authentication error
CLI->>User : Display : "ZAI_API_KEY environment variable not set"
```

**Diagram sources**
- [abogen-ui/crates/ui/services/video_generation.rs](file://abogen-ui/crates/ui/services/video_generation.rs#L100-L150)
- [src/video.rs](file://src/video.rs#L100-L150)

**Section sources**
- [src/video.rs](file://src/video.rs#L45-L81)
- [abogen-ui/crates/ui/services/zai_video.rs](file://abogen-ui/crates/ui/services/zai_video.rs#L49-L94)

## Video Style and Resolution Problems

### Video Style Selection Issues
The system supports multiple video styles including realistic, anime, 3d, cinematic, biotech, cyberpunk, and educational. Style selection issues typically occur due to case sensitivity or invalid style names.

```mermaid
classDiagram
class VideoStyle {
+Realistic
+Anime
+ThreeD
+Cinematic
+Biotech
+Cyberpunk
+Educational
+as_str() string
+from_str(s string) VideoStyle
}
class VideoConfig {
+style VideoStyle
+resolution VideoResolution
+format VideoFormat
+prompt Option~string~
}
VideoConfig --> VideoStyle : "contains"
```

**Diagram sources**
- [abogen-ui/crates/ui/state.rs](file://abogen-ui/crates/ui/state.rs#L96-L129)
- [src/main.rs](file://src/main.rs#L337-L375)

### Resolution Setting Problems
Resolution settings (720p, 1080p, 4k) must match exactly with supported values. The system defaults to 1080p if an invalid resolution is specified.

```mermaid
flowchart TD
Start["User specifies resolution"] --> ValidateResolution["Validate resolution parameter"]
ValidateResolution --> |Valid| ApplyResolution["Apply specified resolution"]
ValidateResolution --> |Invalid| DefaultResolution["Apply default resolution (1080p)"]
ApplyResolution --> GenerateVideo["Generate video with specified resolution"]
DefaultResolution --> GenerateVideo
GenerateVideo --> Complete["Video generation complete"]
```

**Diagram sources**
- [src/main.rs](file://src/main.rs#L337-L375)
- [abogen-ui/crates/ui/lib.rs](file://abogen-ui/crates/ui/lib.rs#L492-L514)

**Section sources**
- [VIDEO_CLI_IMPLEMENTATION.md](file://VIDEO_CLI_IMPLEMENTATION.md#L100-L150)
- [CLI_VIDEO_GENERATION.md](file://CLI_VIDEO_GENERATION.md#L100-L150)

## Custom Prompt Handling

### Custom Prompt Implementation
Custom prompts are optional parameters that enhance video generation with specific visual instructions. The system handles prompts through the `prompt` field in the video configuration.

```mermaid
sequenceDiagram
participant CLI as "CLI Interface"
participant Config as "VideoConfig"
participant Service as "VideoGenerationService"
CLI->>Config : Set prompt from command line
Config->>Service : Pass config with prompt
Service->>Service : Include prompt in API request
Service->>API : Send request with prompt
API-->>Service : Process with custom prompt
Service-->>CLI : Return generated video
```

**Diagram sources**
- [src/video.rs](file://src/video.rs#L200-L250)
- [VIDEO_CLI_IMPLEMENTATION.md](file://VIDEO_CLI_IMPLEMENTATION.md#L150-L200)

**Section sources**
- [VIDEO_CLI_IMPLEMENTATION.md](file://VIDEO_CLI_IMPLEMENTATION.md#L150-L200)
- [CLI_VIDEO_GENERATION.md](file://CLI_VIDEO_GENERATION.md#L150-L200)

## Subtitle and Audio Synchronization

### Subtitle Embedding Issues
Subtitle embedding relies on ffmpeg for integration into the final video. When ffmpeg is unavailable, subtitles are provided as a separate .srt file.

```mermaid
flowchart TD
Start["Video generation complete"] --> CheckFFmpeg["Check for ffmpeg availability"]
CheckFFmpeg --> |Available| EmbedSubtitles["Embed subtitles using ffmpeg"]
CheckFFmpeg --> |Not Available| SeparateSubtitles["Provide subtitles as separate .srt file"]
EmbedSubtitles --> FinalVideo["Create final video with embedded subtitles"]
SeparateSubtitles --> FinalVideo
FinalVideo --> Complete["Generation complete"]
```

**Diagram sources**
- [abogen-ui/crates/ui/services/video_generation.rs](file://abogen-ui/crates/ui/services/video_generation.rs#L353-L390)
- [abogen-ui/crates/ui/services/mlt_video.rs](file://abogen-ui/crates/ui/services/mlt_video.rs#L277-L323)

### Audio-Video Synchronization Problems
Synchronization issues are prevented through precise timing calculations based on audio duration and word-level subtitle timing.

```mermaid
sequenceDiagram
participant Audio as "Audio Processing"
participant Subtitle as "Subtitle Generation"
participant Video as "Video Composition"
Audio->>Audio : Calculate audio duration
Audio->>Subtitle : Provide duration to subtitle generator
Subtitle->>Subtitle : Generate word-level timings
Subtitle->>Video : Provide timed subtitles
Video->>Video : Synchronize audio and subtitles
Video-->>User : Output synchronized video
```

**Diagram sources**
- [abogen-ui/crates/ui/services/mlt_video.rs](file://abogen-ui/crates/ui/services/mlt_video.rs#L277-L323)
- [abogen-ui/crates/ui/services/tts_service.rs](file://abogen-ui/crates/ui/services/tts_service.rs#L149-L178)

**Section sources**
- [abogen-ui/crates/ui/services/mlt_video.rs](file://abogen-ui/crates/ui/services/mlt_video.rs#L277-L323)
- [abogen-ui/crates/ui/services/tts_service.rs](file://abogen-ui/crates/ui/services/tts_service.rs#L149-L178)

## Script Execution and Format Conversion

### generate_video.sh Script Issues
The generate_video.sh script handles the complete video generation workflow but may fail due to missing API keys or build errors.

```mermaid
flowchart TD
Start["Execute generate_video.sh"] --> CheckAPIKey["Check ZAI_API_KEY"]
CheckAPIKey --> |Not Set| Warning["Display warning: API key not set"]
CheckAPIKey --> |Set| BuildProject["Build VoxWeave with video-generation feature"]
BuildProject --> RunCommand["Run video generation command"]
RunCommand --> Complete["Display completion message"]
```

**Diagram sources**
- [generate_video.sh](file://generate_video.sh#L1-L40)
- [VIDEO_CLI_IMPLEMENTATION.md](file://VIDEO_CLI_IMPLEMENTATION.md#L286-L313)

### Format Conversion Failures
Format conversion (mp4, mov, webm) may fail due to missing codecs or incorrect format specifications in the configuration.

```mermaid
classDiagram
class VideoFormat {
+Mp4
+Mov
+Webm
+as_str() string
}
class VideoGenerationService {
-api_key string
-base_url string
+generate_video() Result~PathBuf, String~
+embed_subtitles() Result~PathBuf, String~
}
class VideoConfig {
+format VideoFormat
}
VideoGenerationService --> VideoConfig : "uses"
VideoConfig --> VideoFormat : "contains"
```

**Diagram sources**
- [src/main.rs](file://src/main.rs#L337-L375)
- [abogen-ui/crates/ui/lib.rs](file://abogen-ui/crates/ui/lib.rs#L492-L514)

**Section sources**
- [generate_video.sh](file://generate_video.sh#L1-L40)
- [src/video.rs](file://src/video.rs#L389-L420)

## Timeout and Large File Handling

### Timeout Errors During Video Generation
The system implements timeout handling with a maximum wait time of 5 minutes for video generation jobs.

```mermaid
sequenceDiagram
participant Service as "VideoGenerationService"
participant API as "Z.AI API"
participant User as "User"
Service->>API : Create video generation job
API-->>Service : Return job_id
loop Poll every 5 seconds
Service->>API : GET /video/status/{job_id}
API-->>Service : Return status and progress
Service->>Service : Update progress (30-80%)
end
alt Timeout after 120 attempts
Service-->>User : Return "Video generation timeout"
else Success
Service->>Service : Download video (80-90%)
Service->>Service : Embed subtitles (90-100%)
Service-->>User : Return video path
end
```

**Diagram sources**
- [abogen-ui/crates/ui/services/zai_video.rs](file://abogen-ui/crates/ui/services/zai_video.rs#L138-L182)
- [src/video.rs](file://src/video.rs#L283-L322)

### Handling Large File Outputs
Large file outputs are managed through direct file system operations and streaming downloads to prevent memory issues.

```mermaid
flowchart TD
Start["Video generation request"] --> CheckFileSize["Check input file size"]
CheckFileSize --> |> 2MB| UseFileURL["Use file:// URL instead of upload"]
CheckFileSize --> |<= 2MB| UploadFile["Upload file to Z.AI"]
UseFileURL --> CreateJob["Create video generation job"]
UploadFile --> CreateJob
CreateJob --> DownloadVideo["Stream download of generated video"]
DownloadVideo --> SaveFile["Save directly to disk"]
SaveFile --> Complete["Return output path"]
```

**Diagram sources**
- [src/video.rs](file://src/video.rs#L150-L200)
- [abogen-ui/crates/ui/services/video_generation.rs](file://abogen-ui/crates/ui/services/video_generation.rs#L200-L250)

**Section sources**
- [src/video.rs](file://src/video.rs#L150-L200)
- [abogen-ui/crates/ui/services/zai_video.rs](file://abogen-ui/crates/ui/services/zai_video.rs#L138-L182)

## API Request/Response Debugging

### Debugging API Request/Response Cycles
The system provides comprehensive logging for API request/response cycles to facilitate debugging.

```mermaid
sequenceDiagram
participant CLI as "CLI Interface"
participant Service as "VideoGenerationService"
participant API as "External Video Service"
CLI->>Service : generate_video() call
Service->>Service : Log : "Starting video generation"
Service->>API : POST /upload with audio
API-->>Service : Return upload URL
Service->>Service : Log : "Audio uploaded successfully"
Service->>API : POST /video/generate with job details
API-->>Service : Return job_id
Service->>Service : Log : "Job created : {job_id}"
loop Poll every second
Service->>API : GET /video/status/{job_id}
API-->>Service : Return status and progress
Service->>Service : Update progress callback
end
Service->>Service : Log : "Video generation completed"
Service->>API : GET /video/download/{video_id}
API-->>Service : Return video bytes
Service->>Service : Save video file
Service-->>CLI : Return video path
```

**Diagram sources**
- [src/video.rs](file://src/video.rs#L45-L81)
- [abogen-ui/crates/ui/services/video_generation.rs](file://abogen-ui/crates/ui/services/video_generation.rs#L100-L150)

### Interpreting Error Codes
External video generation services return specific error codes that are translated into user-friendly messages.

```mermaid
flowchart TD
Start["Receive API response"] --> CheckStatus["Check HTTP status code"]
CheckStatus --> |200-299| ProcessSuccess["Process successful response"]
CheckStatus --> |400-499| HandleClientError["Handle client error"]
CheckStatus --> |500-599| HandleServerError["Handle server error"]
HandleClientError --> |401| AuthError["Display: 'Authentication failed - check API key'"]
HandleClientError --> |403| RateLimitError["Display: 'Rate limit exceeded - wait before retrying'"]
HandleClientError --> |404| NotFoundError["Display: 'Resource not found - check job ID'"]
HandleClientError --> |429| TooManyRequests["Display: 'Too many requests - implement exponential backoff'"]
HandleServerError --> |500| InternalError["Display: 'Internal server error - try again later'"]
HandleServerError --> |502| BadGateway["Display: 'Bad gateway - service temporarily unavailable'"]
HandleServerError --> |503| ServiceUnavailable["Display: 'Service unavailable - check API status'"]
HandleServerError --> |504| GatewayTimeout["Display: 'Gateway timeout - request took too long'"]
```

**Diagram sources**
- [src/video.rs](file://src/video.rs#L49-L94)
- [abogen-ui/crates/ui/services/zai_video.rs](file://abogen-ui/crates/ui/services/zai_video.rs#L49-L94)

**Section sources**
- [src/video.rs](file://src/video.rs#L45-L81)
- [abogen-ui/crates/ui/services/zai_video.rs](file://abogen-ui/crates/ui/services/zai_video.rs#L49-L94)