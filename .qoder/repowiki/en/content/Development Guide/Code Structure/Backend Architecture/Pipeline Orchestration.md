# Pipeline Orchestration

<cite>
**Referenced Files in This Document**   
- [pipeline.rs](file://src/pipeline.rs)
- [text.rs](file://src/text.rs)
- [sanitize.rs](file://src/sanitize.rs)
- [subtitle.rs](file://src/subtitle.rs)
- [queue.rs](file://src/queue.rs)
- [tts.rs](file://src/tts.rs)
- [lib.rs](file://src/lib.rs)
</cite>

## Table of Contents
1. [Introduction](#introduction)
2. [Core Components](#core-components)
3. [Pipeline Execution Flow](#pipeline-execution-flow)
4. [ConvertRequest Structure](#convertrequest-structure)
5. [Batch Processing with convert_queue](#batch-processing-with-convert_queue)
6. [Text Processing Pipeline](#text-processing-pipeline)
7. [Subtitle Generation and Timing](#subtitle-generation-and-timing)
8. [Error Handling and Robustness](#error-handling-and-robustness)
9. [Customization and Configuration](#customization-and-configuration)
10. [Example Walkthrough](#example-walkthrough)

## Introduction
The pipeline orchestration system in VoxWeave coordinates the end-to-end transformation of raw text into synchronized audio and subtitle files. This document details the core functionality implemented in `src/pipeline.rs`, focusing on the `convert_path` function that serves as the central coordinator for text-to-speech conversion and subtitle generation. The system integrates multiple specialized modules to clean text, sanitize filenames, synthesize speech, and generate timed subtitles, providing a robust foundation for batch audio content creation.

## Core Components

The pipeline orchestration system consists of several interconnected components that handle specific aspects of the text-to-audio conversion process. The `convert_path` function acts as the primary orchestrator, coordinating calls to text processing, TTS synthesis, and subtitle generation modules.

**Section sources**
- [pipeline.rs](file://src/pipeline.rs#L0-L140)
- [lib.rs](file://src/lib.rs#L0-L14)

## Pipeline Execution Flow

```mermaid
flowchart TD
A["convert_path()"] --> B["Read input text file"]
B --> C["Clean text with clean_text()"]
C --> D["Sanitize filename with sanitize_name_for_os()"]
D --> E["Synthesize speech via TTS engine"]
E --> F{"Generate subtitles?"}
F --> |Yes| G["Generate subtitles with generate_subtitles()"]
G --> H["Format SRT with format_srt()"]
H --> I["Write subtitle file"]
I --> J["Return audio path"]
F --> |No| J
```

**Diagram sources**
- [pipeline.rs](file://src/pipeline.rs#L39-L83)

**Section sources**
- [pipeline.rs](file://src/pipeline.rs#L39-L83)

## ConvertRequest Structure

The `ConvertRequest` structure encapsulates all parameters needed for a text-to-speech conversion operation. It contains the source file path, output directory, voice profile, speech speed, subtitle configuration, newline handling preferences, and average words per minute for timing calculations.

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
+Words(u8)
}
ConvertRequest --> VoiceProfile : "uses"
ConvertRequest --> SubtitleGranularity : "configures"
```

**Diagram sources**
- [pipeline.rs](file://src/pipeline.rs#L0-L38)
- [queue.rs](file://src/queue.rs#L63-L79)
- [tts.rs](file://src/tts.rs#L0-L523)

**Section sources**
- [pipeline.rs](file://src/pipeline.rs#L0-L38)

## Batch Processing with convert_queue

The `convert_queue` function enables batch processing by dequeuing items from a `ConversionQueue` and applying the `convert_path` function to each. This allows for sequential processing of multiple text-to-speech conversion requests, with each item in the queue containing its own configuration parameters.

```mermaid
sequenceDiagram
participant Queue as ConversionQueue
participant Pipeline as convert_queue()
participant Convert as convert_path()
participant TTS as TTS Engine
Pipeline->>Queue : dequeue()
alt Item available
Queue-->>Pipeline : QueueItem
Pipeline->>Convert : Call with request
Convert->>TTS : Synthesize speech
TTS-->>Convert : Audio file
Convert-->>Pipeline : Audio path
Pipeline->>Queue : dequeue()
Pipeline->>Output : Collect result
else Queue empty
Queue-->>Pipeline : None
Pipeline->>Output : Return collected paths
end
```

**Diagram sources**
- [pipeline.rs](file://src/pipeline.rs#L83-L109)

**Section sources**
- [pipeline.rs](file://src/pipeline.rs#L83-L109)
- [queue.rs](file://src/queue.rs#L100-L132)

## Text Processing Pipeline

The text processing pipeline begins with reading the raw input text and proceeds through cleaning and normalization steps. The `clean_text` function removes extra whitespace, collapses multiple blank lines, and optionally replaces single newlines with spaces based on the `replace_single_newlines` flag.

```mermaid
flowchart TD
A["Raw Input Text"] --> B["split('\n')"]
B --> C["collapse_whitespace() per line"]
C --> D["Join with '\n'"]
D --> E["Replace multiple newlines with '\n\n'"]
E --> F["trim()"]
F --> G{"replace_single_newlines?"}
G --> |Yes| H["replace_single_newlines_with_spaces()"]
G --> |No| I["Cleaned Text"]
H --> I
```

**Diagram sources**
- [text.rs](file://src/text.rs#L0-L71)

**Section sources**
- [text.rs](file://src/text.rs#L0-L71)
- [pipeline.rs](file://src/pipeline.rs#L45-L46)

## Subtitle Generation and Timing

Subtitle generation is controlled by the `subtitle_granularity` parameter and uses the `average_words_per_minute` value to calculate timing. The system can generate subtitles at sentence or word-group levels, with timing based on word count and the specified words-per-minute rate.

```mermaid
flowchart TD
A["generate_subtitles()"] --> B{"Granularity"}
B --> |Disabled| C["Return empty"]
B --> |Sentence| D["split_sentences()"]
B --> |Words(n)| E["split_words(chunk_size=n)"]
D --> F["build_entries()"]
E --> F
F --> G["Calculate duration per chunk"]
G --> H["ms_per_word = 60,000 / average_wpm"]
H --> I["duration = word_count * ms_per_word"]
I --> J["Ensure minimum 500ms duration"]
J --> K["Set start/end timestamps"]
K --> L["Return SubtitleEntry array"]
```

**Diagram sources**
- [subtitle.rs](file://src/subtitle.rs#L0-L157)

**Section sources**
- [subtitle.rs](file://src/subtitle.rs#L0-L157)
- [pipeline.rs](file://src/pipeline.rs#L64-L70)

## Error Handling and Robustness

The pipeline implements comprehensive error handling for various failure scenarios, including file I/O operations, text processing, and speech synthesis. Errors are propagated using the `anyhow` crate, providing contextual information about the source of failures.

```mermaid
flowchart TD
A["convert_path()"] --> B["Read input file"]
B --> C{"Success?"}
C --> |No| D["Return error with context"]
C --> |Yes| E["Create output directory"]
E --> F{"Success?"}
F --> |No| G["Return error with context"]
F --> |Yes| H["Synthesize speech"]
H --> I{"Success?"}
I --> |No| J["Return error with context"]
I --> |Yes| K["Generate subtitles if needed"]
K --> L{"Success?"}
L --> |No| M["Return error with context"]
L --> |Yes| N["Return audio path"]
```

**Diagram sources**
- [pipeline.rs](file://src/pipeline.rs#L40-L83)

**Section sources**
- [pipeline.rs](file://src/pipeline.rs#L40-L83)

## Customization and Configuration

The pipeline offers several customization points that allow users to tailor the conversion process to their specific needs. These include controlling newline behavior, adjusting speech speed, selecting different voice profiles, and configuring subtitle granularity and timing.

```mermaid
classDiagram
class ConvertRequest {
+replace_single_newlines : bool
+speed : f32
+voice : VoiceProfile
+subtitle_granularity : SubtitleGranularity
+average_words_per_minute : f32
}
class SubtitleGranularity {
+Disabled
+Sentence
+Words(u8)
}
class VoiceProfile {
+id : String
+description : String
+engine : VoiceEngine
}
ConvertRequest --> SubtitleGranularity : "configures"
ConvertRequest --> VoiceProfile : "specifies"
```

**Diagram sources**
- [pipeline.rs](file://src/pipeline.rs#L0-L38)
- [queue.rs](file://src/queue.rs#L75-L79)
- [tts.rs](file://src/tts.rs#L0-L523)

**Section sources**
- [pipeline.rs](file://src/pipeline.rs#L0-L38)
- [queue.rs](file://src/queue.rs#L75-L79)

## Example Walkthrough

Consider a conversion request with the following parameters:
- Source: `samples/example.txt`
- Output directory: `output/`
- Voice: "en-us" (English US)
- Speed: 1.2x
- Subtitle granularity: Sentence
- Replace single newlines: false
- Average words per minute: 160

The pipeline would execute as follows:
1. Read the content of `samples/example.txt`
2. Clean the text by removing extra whitespace and blank lines
3. Extract the base filename "example" and sanitize it for the current OS
4. Create the output directory if it doesn't exist
5. Synthesize speech using the specified voice and speed, saving to `output/example.wav`
6. Generate sentence-level subtitles using 160 words per minute for timing calculations
7. Format the subtitles in SRT format and save to `output/example.srt`
8. Return the path to the generated audio file

This end-to-end process transforms raw text into synchronized audio and subtitle files, ready for use in video production or other applications.

**Section sources**
- [pipeline.rs](file://src/pipeline.rs#L39-L83)
- [text.rs](file://src/text.rs#L0-L71)
- [sanitize.rs](file://src/sanitize.rs#L0-L162)
- [subtitle.rs](file://src/subtitle.rs#L0-L157)