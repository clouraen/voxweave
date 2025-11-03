# Core Services

<cite>
**Referenced Files in This Document**   
- [pipeline.rs](file://src/pipeline.rs)
- [tts.rs](file://src/tts.rs)
- [text.rs](file://src/text.rs)
- [subtitle.rs](file://src/subtitle.rs)
- [sanitize.rs](file://src/sanitize.rs)
- [main.rs](file://src/main.rs)
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
This document provides architectural documentation for the core services in the VoxWeave platform, focusing on the service-oriented design of the pipeline module. It details how text-to-speech and subtitle generation workflows are orchestrated through the `ConvertRequest` struct and underlying abstractions. The integration between CLI and UI entry points is explained, along with data flow from input text to final audio and subtitle outputs.

## Project Structure
The VoxWeave platform consists of multiple components organized into distinct directories. The core functionality resides in the `src/` directory, which contains modules for configuration, text processing, TTS engines, subtitle generation, and pipeline orchestration. The `abogen-ui/` directory houses frontend implementations across desktop, mobile, and web platforms, while shared logic is abstracted into reusable crates.

```mermaid
graph TD
subgraph "Core Services"
A[pipeline.rs]
B[tts.rs]
C[text.rs]
D[subtitle.rs]
E[sanitize.rs]
F[queue.rs]
end
subgraph "UI Layer"
G[abogen-ui]
H[tts_service.rs]
I[tts_stub.rs]
end
subgraph "CLI Interface"
J[main.rs]
end
A --> B
A --> C
A --> D
A --> E
J --> A
G --> H
H --> A
I --> A
```

**Diagram sources**
- [pipeline.rs](file://src/pipeline.rs#L1-L140)
- [main.rs](file://src/main.rs#L1-L424)
- [tts_service.rs](file://abogen-ui/crates/ui/services/tts_service.rs#L1-L300)
- [tts_stub.rs](file://abogen-ui/crates/ui/services/tts_stub.rs#L1-L120)

**Section sources**
- [pipeline.rs](file://src/pipeline.rs#L1-L140)
- [main.rs](file://src/main.rs#L1-L424)

## Core Components
The core services of VoxWeave are built around a modular architecture that separates concerns across text processing, speech synthesis, and subtitle generation. The pipeline module acts as the central orchestrator, coordinating these components through well-defined interfaces and data structures.

**Section sources**
- [pipeline.rs](file://src/pipeline.rs#L1-L140)
- [tts.rs](file://src/tts.rs#L1-L523)

## Architecture Overview
VoxWeave's architecture follows a service-oriented design where the pipeline module coordinates text cleaning, audio synthesis, and SRT subtitle generation. The system uses a trait-based abstraction (`SpeechEngine`) to support multiple TTS backends, enabling flexibility in voice synthesis technologies.

```mermaid
sequenceDiagram
participant CLI as CLI Interface
participant Pipeline as Pipeline Module
participant Text as Text Processor
participant TTS as Speech Engine
participant Subtitle as Subtitle Generator
participant FS as File System
CLI->>Pipeline : convert_path(request)
Pipeline->>Text : clean_text(input)
Text-->>Pipeline : cleaned text
Pipeline->>TTS : synthesize_to_file()
TTS-->>Pipeline : audio file
Pipeline->>Subtitle : generate_subtitles()
Subtitle-->>Pipeline : subtitle entries
Pipeline->>FS : write SRT file
FS-->>Pipeline : success
Pipeline-->>CLI : audio path
```

**Diagram sources**
- [pipeline.rs](file://src/pipeline.rs#L1-L140)
- [text.rs](file://src/text.rs#L1-L71)
- [subtitle.rs](file://src/subtitle.rs#L1-L157)
- [tts.rs](file://src/tts.rs#L1-L523)

## Detailed Component Analysis

### ConvertRequest and Pipeline Orchestration
The `ConvertRequest` struct serves as the primary data carrier for conversion operations, encapsulating all necessary parameters for text-to-speech and subtitle generation workflows. It contains source and output paths, voice profile, speed settings, subtitle configuration, and text processing options.

```mermaid
classDiagram
class ConvertRequest {
+source : PathBuf
+output_dir : PathBuf
+voice : VoiceProfile
+speed : f32
+subtitle_granularity : SubtitleGranularity
+replace_single_newlines : bool
+average_words_per_minute : f32
}
class VoiceProfile {
+id : String
+description : String
+engine : VoiceEngine
+command : Option<String>
+lang : Option<String>
}
class SubtitleGranularity {
+Disabled
+Sentence
+Words(count : u8)
}
ConvertRequest --> VoiceProfile : "uses"
ConvertRequest --> SubtitleGranularity : "references"
```

**Diagram sources**
- [pipeline.rs](file://src/pipeline.rs#L1-L140)
- [tts.rs](file://src/tts.rs#L1-L523)
- [queue.rs](file://src/queue.rs#L1-L100)

**Section sources**
- [pipeline.rs](file://src/pipeline.rs#L1-L140)

### Text Processing Pipeline
The text processing component handles cleaning and normalization of input text before synthesis. It removes excessive whitespace, collapses newlines, and optionally replaces single newlines with spaces based on configuration.

```mermaid
flowchart TD
Start([Input Text]) --> CollapseWhitespace["Collapse Whitespace per Line"]
CollapseWhitespace --> ReduceNewlines["Reduce Multiple Newlines to Double"]
ReduceNewlines --> TrimText["Trim Leading/Trailing Whitespace"]
TrimText --> ReplaceNewlines{"Replace Single Newlines?"}
ReplaceNewlines --> |Yes| ReplaceWithSpaces["Replace \\n with Spaces"]
ReplaceNewlines --> |No| Output["Final Cleaned Text"]
ReplaceWithSpaces --> Output
Output --> End([Cleaned Text])
```

**Diagram sources**
- [text.rs](file://src/text.rs#L1-L71)

**Section sources**
- [text.rs](file://src/text.rs#L1-L71)

### Subtitle Generation Workflow
The subtitle generation system creates timed SRT files based on text content and estimated reading speed. It supports different granularity levels including sentence-based and word-grouped subtitles.

```mermaid
sequenceDiagram
participant Pipeline as Pipeline
participant Subtitle as Subtitle Module
participant Formatter as SRT Formatter
Pipeline->>Subtitle : generate_subtitles(text, granularity, wpm)
alt Granularity Enabled
Subtitle->>Subtitle : split_text_by(granularity)
Subtitle->>Subtitle : calculate_duration(words, wpm)
Subtitle-->>Pipeline : SubtitleEntry[]
Pipeline->>Formatter : format_srt(entries)
Formatter-->>Pipeline : SRT string
Pipeline->>FS : write .srt file
else Disabled
Subtitle-->>Pipeline : empty list
end
```

**Diagram sources**
- [subtitle.rs](file://src/subtitle.rs#L1-L157)

**Section sources**
- [subtitle.rs](file://src/subtitle.rs#L1-L157)

### TTS Engine Abstraction
The `SpeechEngine` trait provides a unified interface for different text-to-speech implementations, allowing the pipeline to work with various backend technologies without coupling to specific implementations.

```mermaid
classDiagram
class SpeechEngine {
<<trait>>
+synthesize_to_file(text, voice, speed, output) Result<(), TtsError>
}
class EspeakEngine {
+synthesize_to_file()
}
class KokoroEngine {
+synthesize_to_file()
}
class CoquiEngine {
+synthesize_to_file()
}
class MockSpeechEngine {
+synthesize_to_file()
}
SpeechEngine <|-- EspeakEngine
SpeechEngine <|-- KokoroEngine
SpeechEngine <|-- CoquiEngine
SpeechEngine <|-- MockSpeechEngine
```

**Diagram sources**
- [tts.rs](file://src/tts.rs#L1-L523)

**Section sources**
- [tts.rs](file://src/tts.rs#L1-L523)

## Dependency Analysis
The core services exhibit a layered dependency structure where higher-level components depend on lower-level abstractions. The pipeline module serves as the integration point, depending on text processing, subtitle generation, and TTS engine components.

```mermaid
graph TD
A[pipeline.rs] --> B[text.rs]
A --> C[subtitle.rs]
A --> D[sanitize.rs]
A --> E[tts.rs]
A --> F[queue.rs]
G[main.rs] --> A
H[tts_service.rs] --> A
I[tts_stub.rs] --> A
E --> J[serde]
E --> K[anyhow]
F --> K
```

**Diagram sources**
- [Cargo.toml](file://Cargo.toml#L1-L50)
- [pipeline.rs](file://src/pipeline.rs#L1-L140)
- [tts.rs](file://src/tts.rs#L1-L523)

**Section sources**
- [Cargo.toml](file://Cargo.toml#L1-L50)
- [pipeline.rs](file://src/pipeline.rs#L1-L140)

## Performance Considerations
The pipeline implementation considers several performance aspects including batch processing efficiency and memory usage during file operations. The `convert_queue` function processes items sequentially but could benefit from parallelization for independent tasks.

Key performance characteristics:
- **Memory Usage**: Text content is loaded entirely into memory; large files may impact memory consumption
- **I/O Operations**: Each conversion involves multiple file operations (read input, write audio, write subtitles)
- **Processing Efficiency**: Queue processing occurs synchronously, limiting throughput for multiple files
- **Path Sanitization**: OS-specific name sanitization adds minimal overhead but ensures cross-platform compatibility

Optimization opportunities include:
- Implementing concurrent processing for queue items
- Streaming text processing for large files
- Caching frequently used voice profiles
- Batch file system operations where possible

**Section sources**
- [pipeline.rs](file://src/pipeline.rs#L1-L140)
- [sanitize.rs](file://src/sanitize.rs#L1-L162)

## Troubleshooting Guide
Common issues in the core services typically relate to configuration, file system permissions, or missing dependencies. Error propagation using `anyhow` provides contextual information for debugging.

Frequent error scenarios:
- **Missing Input Files**: Verify source paths exist and are accessible
- **Output Directory Permissions**: Ensure output directories are writable
- **Voice Profile Not Found**: Check voice ID against available options via `list-voices`
- **TTS Engine Dependencies**: Confirm required binaries (espeak, python) are installed and in PATH
- **Environment Variables**: Set required environment variables for specific engines (VOXWEAVE_KOKORO_PYTHON, VOXWEAVE_ESPEAK_COMMAND)

Error handling follows a consistent pattern using `anyhow::Context` to add contextual information while preserving the original error chain.

**Section sources**
- [pipeline.rs](file://src/pipeline.rs#L1-L140)
- [tts.rs](file://src/tts.rs#L1-L523)
- [main.rs](file://src/main.rs#L1-L424)

## Conclusion
The VoxWeave core services demonstrate a well-structured, modular architecture that effectively separates concerns across text processing, speech synthesis, and subtitle generation. The pipeline module successfully orchestrates these components through the `ConvertRequest` abstraction, providing a clean interface for both CLI and UI entry points. The trait-based design of `SpeechEngine` enables extensibility with different TTS backends while maintaining a consistent API. Future improvements could focus on performance optimizations through parallel processing and enhanced error recovery mechanisms.