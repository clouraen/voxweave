# Video Generation

<cite>
**Referenced Files in This Document**   
- [src/video.rs](file://src/video.rs)
- [VIDEO_CLI_IMPLEMENTATION.md](file://VIDEO_CLI_IMPLEMENTATION.md)
- [abogen-ui/ZAI_VIDEO_FEATURE.md](file://abogen-ui/ZAI_VIDEO_FEATURE.md)
- [src/queue.rs](file://src/queue.rs)
- [src/lib.rs](file://src/lib.rs)
</cite>

## Table of Contents
1. [Introduction](#introduction)
2. [Project Structure](#project-structure)
3. [Core Components](#core-components)
4. [Architecture Overview](#architecture-overview)
5. [Detailed Component Analysis](#detailed-component-analysis)
6. [Dependency Analysis](#dependency-analysis)
7. [Performance Considerations](#performance-considerations)
8. [Troubleshooting Guide](#troubleshooting-guide)
9. [Conclusion](#conclusion)

## Introduction

The VoxWeave video generation system enables automated creation of narrated videos from text content using the Z.AI API. This documentation details the implementation of the `VideoGenerationService` in `src/video.rs`, which orchestrates the complete pipeline: audio upload, job creation, status polling, video download, and subtitle embedding via ffmpeg. The system supports multiple video styles, resolutions, and formats, and allows for custom visual prompts. Both CLI and UI implementations are covered, with integration details from `VIDEO_CLI_IMPLEMENTATION.md` and `ZAI_VIDEO_FEATURE.md`. The service includes robust error handling for API key issues, timeouts, and HTTP status codes, and provides examples of API interactions.

## Project Structure

The video generation functionality is distributed across multiple components in the VoxWeave repository. The core logic resides in `src/video.rs`, while configuration and data structures are defined in `src/queue.rs`. The UI integration is handled in the `abogen-ui` crate, specifically in `services/zai_video.rs` and `services/mlt_video.rs`. The CLI implementation is integrated into `src/main.rs` and documented in `VIDEO_CLI_IMPLEMENTATION.md`.

```mermaid
graph TB
subgraph "Core Library"
video_rs[src/video.rs]
queue_rs[src/queue.rs]
lib_rs[src/lib.rs]
end
subgraph "CLI Interface"
main_rs[src/main.rs]
VIDEO_CLI_IMPLEMENTATION_MD[VIDEO_CLI_IMPLEMENTATION.md]
end
subgraph "UI Interface"
abogen_ui[abogen-ui/]
ZAI_VIDEO_FEATURE_MD[ZAI_VIDEO_FEATURE.md]
end
video_rs --> queue_rs
lib_rs --> video_rs
main_rs --> video_rs
abogen_ui --> video_rs
```

**Diagram sources**
- [src/video.rs](file://src/video.rs#L1-L462)
- [src/queue.rs](file://src/queue.rs#L1-L154)
- [src/lib.rs](file://src/lib.rs#L1-L13)
- [VIDEO_CLI_IMPLEMENTATION.md](file://VIDEO_CLI_IMPLEMENTATION.md#L1-L334)
- [abogen-ui/ZAI_VIDEO_FEATURE.md](file://abogen-ui/ZAI_VIDEO_FEATURE.md#L1-L168)

**Section sources**
- [src/video.rs](file://src/video.rs#L1-L462)
- [src/queue.rs](file://src/queue.rs#L1-L154)
- [src/lib.rs](file://src/lib.rs#L1-L13)

## Core Components

The `VideoGenerationService` in `src/video.rs` is the central component for video generation. It handles the complete pipeline from audio upload to final video delivery with subtitles. The service supports two providers: Z.AI and OpenAI Sora, with Z.AI being the primary provider. The service is configured through the `VideoConfig` struct, which specifies the video style, resolution, format, and optional custom prompt. The service integrates with the existing TTS pipeline to generate synchronized audio and subtitles.

**Section sources**
- [src/video.rs](file://src/video.rs#L15-L462)
- [src/queue.rs](file://src/queue.rs#L2-L49)

## Architecture Overview

The video generation architecture follows a modular design with clear separation of concerns. The CLI and UI both interface with the `VideoGenerationService`, which handles the interaction with the Z.AI API. The service manages the complete workflow: uploading the audio file, creating a video generation job, polling for completion, downloading the generated video, and optionally embedding subtitles using ffmpeg.

```mermaid
graph TD
A[Input Text] --> B[Generate Audio & Subtitles]
B --> C[Upload Audio to Z.AI]
C --> D[Create Video Generation Job]
D --> E[Poll Job Status]
E --> F{Job Complete?}
F --> |Yes| G[Download Video]
F --> |No| E
G --> H{Embed Subtitles?}
H --> |Yes| I[Run ffmpeg with subtitles filter]
H --> |No| J[Return Video Path]
I --> J
J --> K[Final Video Output]
```

**Diagram sources**
- [src/video.rs](file://src/video.rs#L150-L462)
- [VIDEO_CLI_IMPLEMENTATION.md](file://VIDEO_CLI_IMPLEMENTATION.md#L50-L80)

## Detailed Component Analysis

### Video Generation Service Analysis

The `VideoGenerationService` implements a comprehensive pipeline for video generation. It begins by uploading the audio file to Z.AI, either via direct upload or by providing a file URL for large files. It then creates a video generation job with the specified style, resolution, and prompt. The service polls the job status until completion, downloads the generated video, and optionally embeds subtitles using ffmpeg.

#### For Object-Oriented Components:
```mermaid
classDiagram
class VideoGenerationService {
+api_key : String
+base_url : String
+provider : VideoProvider
+new(api_key : String) VideoGenerationService
+new_sora(api_key : String) VideoGenerationService
+from_env() Result~VideoGenerationService~
+generate_video(audio_path : &Path, subtitle_path : Option~&Path~, config : &VideoConfig, progress_callback : Option~F~, log_callback : Option~G~) Result~PathBuf~
}
class VideoConfig {
+style : VideoStyle
+resolution : VideoResolution
+format : VideoFormat
+prompt : Option~String~
}
class VideoProvider {
+ZAI
+OpenAISora
}
VideoGenerationService --> VideoConfig : "uses"
VideoGenerationService --> VideoProvider : "uses"
```

**Diagram sources**
- [src/video.rs](file://src/video.rs#L15-L462)

#### For API/Service Components:
```mermaid
sequenceDiagram
participant Client as "CLI/UI"
participant Service as "VideoGenerationService"
participant ZAI as "Z.AI API"
participant FFmpeg as "ffmpeg"
Client->>Service : generate_video()
Service->>Service : upload_file()
Service->>ZAI : POST /upload
ZAI-->>Service : File URL
Service->>Service : create_video_job()
Service->>ZAI : POST /videos/generations
ZAI-->>Service : Job ID
Service->>Service : poll_video_status()
loop Poll every 1s
Service->>ZAI : GET /videos/generations/{job_id}
ZAI-->>Service : Status (PROCESSING)
end
ZAI-->>Service : Status (SUCCESS) with video URL
Service->>Service : download_video()
Service->>ZAI : GET video URL
ZAI-->>Service : Video data
Service->>Service : embed_subtitles()?
Service->>FFmpeg : Run subtitles filter
FFmpeg-->>Service : Subtitled video
Service-->>Client : Final video path
```

**Diagram sources**
- [src/video.rs](file://src/video.rs#L150-L462)

#### For Complex Logic Components:
```mermaid
flowchart TD
Start([Start]) --> Upload["Upload Audio File"]
Upload --> CheckSize{"File > 2MB?"}
CheckSize --> |Yes| UseFileURL["Use file:// URL"]
CheckSize --> |No| ReadFile["Read File Data"]
ReadFile --> CreateForm["Create Multipart Form"]
CreateForm --> SendUpload["Send POST /upload"]
SendUpload --> UploadSuccess{"Upload Success?"}
UploadSuccess --> |No| UseFileURL
UploadSuccess --> |Yes| GetURL["Extract URL from Response"]
UseFileURL --> GetURL
GetURL --> CreateJob["Create Video Job"]
CreateJob --> SendJob["Send POST /videos/generations"]
SendJob --> JobSuccess{"Job Created?"}
JobSuccess --> |No| Error["Return API Error"]
JobSuccess --> |Yes| GetJobID["Extract Job ID"]
GetJobID --> PollLoop["Poll Job Status"]
PollLoop --> GetStatus["GET /videos/generations/{job_id}"]
GetStatus --> CheckStatus{"Status = SUCCESS?"}
CheckStatus --> |No| Wait["Wait 1s"]
Wait --> PollLoop
CheckStatus --> |Yes| GetVideoURL["Extract Video URL"]
GetVideoURL --> Download["Download Video"]
Download --> SaveFile["Save to Disk"]
SaveFile --> EmbedSubs{"Embed Subtitles?"}
EmbedSubs --> |Yes| RunFFmpeg["Run ffmpeg with subtitles filter"]
RunFFmpeg --> CheckFFmpeg{"FFmpeg Success?"}
CheckFFmpeg --> |No| Warn["Log Warning, Use Video Without Subtitles"]
CheckFFmpeg --> |Yes| UseSubtitled["Use Subtitled Video"]
Warn --> End["Return Video Path"]
UseSubtitled --> End
EmbedSubs --> |No| End
```

**Diagram sources**
- [src/video.rs](file://src/video.rs#L150-L462)

**Section sources**
- [src/video.rs](file://src/video.rs#L15-L462)

### Video Styles and Configurations

The system supports multiple video styles, resolutions, and formats through dedicated enum types in `src/queue.rs`. These configurations are used by both the CLI and UI to provide consistent options for video generation.

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
+as_str() String
}
class VideoResolution {
+P720
+P1080
+P4K
+as_str() String
}
class VideoFormat {
+Mp4
+Mov
+Webm
+as_str() String
}
```

**Diagram sources**
- [src/queue.rs](file://src/queue.rs#L2-L49)

**Section sources**
- [src/queue.rs](file://src/queue.rs#L2-L49)

## Dependency Analysis

The video generation system has several key dependencies that enable its functionality. The core dependencies are managed through Cargo features, allowing the video generation module to be conditionally compiled.

```mermaid
graph TD
A[VideoGenerationService] --> B[reqwest]
A --> C[tokio]
A --> D[anyhow]
A --> E[serde_json]
A --> F[ffmpeg]
B --> G[HTTP Client]
C --> H[Async Runtime]
D --> I[Error Handling]
E --> J[JSON Serialization]
F --> K[Subtitle Embedding]
```

**Diagram sources**
- [Cargo.toml](file://Cargo.toml#L1-L20)
- [src/video.rs](file://src/video.rs#L1-L462)

**Section sources**
- [VIDEO_CLI_IMPLEMENTATION.md](file://VIDEO_CLI_IMPLEMENTATION.md#L200-L230)

## Performance Considerations

Video generation performance is influenced by several factors, including resolution, content length, and API server load. Generation times typically range from 1-2 minutes for 720p, 2-4 minutes for 1080p, and 4-5 minutes for 4K resolution. The system implements a 5-minute timeout for video generation jobs to prevent indefinite waiting. Batch processing is supported through the CLI, allowing multiple videos to be generated sequentially. For optimal performance, users should select the appropriate resolution for their needs and consider adding delays between batch requests to avoid rate limits.

**Section sources**
- [VIDEO_CLI_IMPLEMENTATION.md](file://VIDEO_CLI_IMPLEMENTATION.md#L300-L320)

## Troubleshooting Guide

The video generation system includes comprehensive error handling for common issues. Key error scenarios and their solutions include:

**Section sources**
- [src/video.rs](file://src/video.rs#L150-L462)
- [VIDEO_CLI_IMPLEMENTATION.md](file://VIDEO_CLI_IMPLEMENTATION.md#L250-L280)

### API Key Errors
When the ZAI_API_KEY environment variable is not set, the system returns a clear error message. Users should ensure the API key is properly set in their environment.

### Timeout Errors
If video generation exceeds the 5-minute timeout, the system returns a timeout error. Users can try with shorter input text or verify their network connection.

### HTTP Status Errors
API errors from Z.AI are propagated with the HTTP status code and error message, allowing users to diagnose issues such as rate limiting or invalid requests.

### FFmpeg Errors
When subtitle embedding fails due to missing or misconfigured ffmpeg, the system logs a warning and returns the video without subtitles, ensuring partial functionality.

## Conclusion

The VoxWeave video generation system provides a robust and flexible solution for creating narrated videos from text content using the Z.AI API. The `VideoGenerationService` in `src/video.rs` implements a complete pipeline that handles audio upload, job creation, status polling, video download, and subtitle embedding. The system supports multiple video styles, resolutions, and formats, and allows for custom visual prompts. Both CLI and UI implementations provide feature parity, with the CLI enabling automation and batch processing. The architecture is modular and well-documented, with comprehensive error handling and performance considerations.