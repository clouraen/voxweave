# Voice Recording with Teleprompter Feature Design

## Overview
This document outlines the design for replacing the current Text-to-Speech (TTS) functionality with a voice recording feature that includes a teleprompter to assist users in recording their voiceovers. This feature will allow users to record their own voice instead of generating synthetic speech, with a visual aid to help them read and record text smoothly.

## Requirements
- Replace TTS audio generation with voice recording capability
- Implement a teleprompter interface to guide users during recording
- Maintain existing text processing pipeline (normalization, sanitization)
- Preserve subtitle generation functionality with accurate timing
- Support existing video generation workflow
- Provide recording controls (start, stop, pause)
- Allow users to re-record segments if needed
- Support all existing output formats (WAV, MP3, FLAC)
- Maintain compatibility with existing queue processing system
- Cross-platform support (desktop, web, mobile)

## Architecture Changes

### Core Pipeline Modifications
The existing pipeline in `pipeline.rs` will be extended to support voice recording:
1. Process text through existing normalization and sanitization steps
2. Replace TTS generation with voice recording workflow
3. Generate accurate subtitles based on recorded audio timing
4. Continue with video generation as before

A new `VoiceRecordingEngine` trait will be created to mirror the existing `SpeechEngine` trait but for recorded audio.

### Component Structure
```
VoiceRecordingPipeline
├── TextProcessor (existing)
├── TeleprompterController
├── AudioRecorder
├── SubtitleGenerator
└── VideoGenerator (existing)
```

## Teleprompter Design

### Interface Elements
- Scrollable text display with current reading position highlighting
- Adjustable text size and contrast for visibility
- Progress indicator showing completed sections
- Recording status indicators
- Control buttons (record, pause, stop)
- Text segmentation visualization

### Functionality
- Text scrolling synchronized with user reading pace
- Highlighting of current sentence/phrase being read
- Ability to jump to specific sections for re-recording
- Visual cues for pauses and emphasis
- Adjustable scroll speed based on user preference
- Support for different text segmentation modes (sentence, paragraph)
- Real-time word highlighting during recording

### Implementation Details
The teleprompter will be implemented as a new UI component that:
1. Takes processed text from the text normalization pipeline
2. Segments text into readable chunks (sentences or paragraphs)
3. Displays text with visual highlighting of the current reading position
4. Scrolls text at an adjustable pace
5. Integrates with recording controls

## Audio Recording Integration

### Recording Workflow
1. User prepares script in teleprompter
2. User initiates recording
3. Teleprompter begins scrolling as user reads
4. Audio is captured through system microphone
5. Recording stops when user completes script or manually stops
6. Audio is processed and synchronized with subtitle timing

### Technical Considerations
- Cross-platform audio recording using existing `cpal` integration
- Audio format compatibility with existing pipeline (WAV as intermediate format)
- Real-time monitoring of recording levels
- Handling of recording interruptions or errors
- Support for converting recorded audio to user-selected output formats (MP3, FLAC)

The existing `AudioRecorder` component in the UI already implements basic recording functionality using `cpal` that can be extended for this feature.

### Timing Synchronization
For accurate subtitle generation, the system will:
1. Record word-level timing during the recording process
2. Use the existing `WordSubtitle` structure from `mlt_video.rs`
3. Generate accurate subtitle timings based on actual word pronunciation times
4. Support all existing subtitle formats (SRT, ASS, VTT)

## UI Modifications

### Desktop Application Changes
- Add toggle to switch between TTS and voice recording modes
- Replace TTS configuration panel with recording controls when in recording mode
- Add teleprompter display area
- Implement recording status panel
- Add audio level visualization
- Integrate with existing `AudioRecorder` component

### Mobile Application Changes
- Optimize teleprompter for smaller screens
- Implement touch-friendly recording controls
- Add recording preview functionality
- Add mode toggle for TTS vs recording

### Web Application Changes
- Browser-compatible audio recording implementation using Web Audio API
- Responsive teleprompter design
- Streamlined recording workflow
- Mode toggle for TTS vs recording

## Data Flow

```
graph TD
    A[Input Text] --> B[Text Processing]
    B --> C[Teleprompter Display]
    C --> D[Voice Recording]
    D --> E[Audio Processing]
    E --> F[Word Timing Analysis]
    F --> G[Subtitle Generation]
    G --> H[Video Generation]
    H --> I[Output Files]
```

## New Components

### VoiceRecordingEngine Trait
A new trait similar to `SpeechEngine` but for voice recording:

```rust
pub trait VoiceRecordingEngine {
    fn record_to_file(
        &self,
        text: &str,
        config: &VoiceRecordingConfig,
        output: &Path,
    ) -> Result<RecordingMetadata, VoiceRecordingError>;
}
```

### VoiceRecordingConfig
Configuration struct for voice recording parameters:

```rust
pub struct VoiceRecordingConfig {
    pub input_device: Option<String>,
    pub scroll_speed: f32,
    pub teleprompter_mode: TeleprompterMode,
    // ... other configuration options
}
```

### RecordingMetadata
Metadata returned after recording completion:

```rust
pub struct RecordingMetadata {
    pub duration: f32,
    pub word_timings: Vec<WordTiming>,
    // ... other metadata
}
```

### TeleprompterMode
Enumeration for different teleprompter display modes:

```rust
pub enum TeleprompterMode {
    Sentence,
    Paragraph,
    WordByWord,
}
```

## Integration with Existing Systems

### Queue Processing
The existing queue processing system in `tts_service.rs` will be extended to support voice recording mode. The `process_queue` function will need to:
1. Detect when voice recording mode is selected
2. Use the new `VoiceRecordingEngine` instead of `SpeechEngine`
3. Maintain compatibility with all existing output formats
4. Support all existing configuration options

### Subtitle Generation
Subtitle generation will be based on actual word timings from the recorded audio rather than estimated timings. The system will:
1. Use the existing `WordSubtitle` structure from `mlt_video.rs`
2. Generate accurate timings during the recording process
3. Convert word timings to the appropriate subtitle format (SRT, ASS, VTT)
4. Maintain compatibility with all existing subtitle configuration options

### File Format Conversion
The existing audio format conversion logic in `tts_service.rs` will be reused for converting recorded WAV files to user-selected output formats (MP3, FLAC).

## Configuration Options

### Teleprompter Settings
- Text size and font
- Scroll speed adjustment
- Highlight color and style
- Background contrast options
- Text segmentation mode (sentence, paragraph, word-by-word)
- Auto-scroll vs manual scroll options

### Recording Settings
- Audio input device selection
- Recording quality parameters
- File format options
- Automatic gain control
- Pre-recording countdown
- Pause detection sensitivity

## Implementation Plan

### Phase 1: Core Recording Functionality
- Implement `VoiceRecordingEngine` trait
- Create voice recording pipeline in `pipeline.rs`
- Add voice recording support to queue processing
- Implement basic teleprompter component

### Phase 2: UI Integration
- Add mode toggle between TTS and voice recording
- Implement recording controls for all platforms
- Integrate teleprompter display component
- Add teleprompter customization options
- Implement word timing analysis

### Phase 3: Advanced Features
- Implement segment re-recording capability
- Add real-time audio monitoring
- Optimize teleprompter scrolling algorithms
- Add pre-recording countdown
- Implement pause detection
- Add audio level visualization

## Testing Strategy

### Unit Tests
- Voice recording functionality
- Teleprompter text scrolling logic
- Integration with existing pipeline components
- Audio format conversion
- Word timing accuracy

### Integration Tests
- End-to-end recording workflow
- Cross-platform recording compatibility
- Subtitle timing accuracy
- Queue processing with voice recording mode
- Mode switching between TTS and recording

## Dependencies
- `cpal` for cross-platform audio recording (already used in project)
- `hound` for WAV file handling (already used in project)
- Existing text processing components
- Subtitle generation components
- Video generation pipeline
- FFmpeg for audio format conversion (already used in project)
- Existing word timing analysis from `mlt_video.rs`
