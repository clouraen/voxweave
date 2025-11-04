# Video Processing Cleanup and MLT Package Export

## Overview

Transform VoxWeave from an AI video generation tool to a streamlined narration package creator that produces a ZIP archive containing all assets needed for external video editing. The system will generate audio narration with synchronized subtitles and package them with MLT project files configured for professional video composition using user-provided AI-generated video footage.

## Objectives

### Primary Goals
- Remove all AI video generation functionality and dependencies
- Generate organized ZIP packages containing audio, subtitles, and MLT project files
- Provide MLT configurations with professional subtitle styling for optimal readability
- Support external 5-second boomerang AI video integration through standardized folder structure

### Success Criteria
- Zero dependencies on video generation APIs (Z.AI, Alibaba, etc.)
- Clean removal of video-generation feature flag and related code
- Functional MLT project files that load correctly in Kdenlive and other MLT-compatible editors
- Professional subtitle appearance with high contrast and readability
- Clear folder structure for user-provided video placement

## Current State Analysis

### Components to Remove
- ZAI video service (`src/zai_video.rs`)
- Alibaba video service (`src/aliyun_video.rs`)
- UI video generation service (`abogen-ui/crates/ui/services/video_generation.rs`)
- Video generation feature flag and conditional compilation
- Video generation CLI commands and arguments
- Video-related dependencies in Cargo.toml

### Components to Preserve
- Audio generation pipeline (`src/pipeline.rs`)
- TTS engine integrations (Espeak, Kokoro, CoquiTTS)
- Subtitle generation system (`src/subtitle.rs`)
- Text processing and sanitization
- CLI and UI core functionality

### Components to Create
- MLT project file generator
- ZIP package builder
- Folder structure organizer
- Subtitle styling configuration for MLT

## Proposed Solution

### Architecture Changes

#### Removal Strategy
1. Delete video generation service modules completely
2. Remove video-generation feature flag from all Cargo.toml files
3. Eliminate video-related CLI commands and arguments
4. Strip video generation UI components and state management
5. Clean up video-related configuration and enums
6. Remove external video API dependencies

#### New Package Export Pipeline
Transform the existing conversion pipeline to produce ZIP packages instead of standalone files. The pipeline flow becomes:

Text Input → Audio Generation → Subtitle Generation → MLT Project Creation → ZIP Packaging → Output

### ZIP Package Structure

The generated ZIP archive will follow this standardized structure:

```
project-name.zip
├── audio/
│   └── narration.wav
├── subtitles/
│   └── narration.srt
├── video/
│   ├── README.txt (instructions for user)
│   └── .gitkeep (placeholder)
├── project.mlt
└── README.txt (package overview)
```

#### Directory Purposes

**audio/** - Contains the generated WAV audio file with the narrated text

**subtitles/** - Contains SRT subtitle files with timing synchronized to the audio narration

**video/** - Empty directory with instructions where users should place their 5-second boomerang AI video. The README inside explains expected format and naming conventions.

**project.mlt** - MLT project file configured to combine audio, video, and subtitles with professional styling

**README.txt** - Root-level documentation explaining the package contents and usage workflow

### MLT Project Configuration

#### Project Structure
The MLT file will define a video composition project with three primary tracks:
- Video track: References the user-provided video file in the video/ folder
- Audio track: References the generated narration WAV file
- Subtitle overlay: Embedded subtitle track with professional styling

#### Subtitle Styling Specifications

The MLT configuration will include subtitle appearance settings inspired by Cyberpunk 2077 and Blade Runner 2049 aesthetics, combining futuristic design with optimal readability:

**Font Configuration**
- Font family: Monospace (Courier New, Consolas, or Roboto Mono for cyberpunk authenticity)
- Font size: Proportional to video resolution (e.g., 44pt for 1080p)
- Font weight: Medium with slight letter-spacing for futuristic feel
- Character spacing: 1.5px for enhanced technical aesthetic

**Color and Contrast - Cyberpunk Neon Palette**
- Text color: Cyan/Teal (RGB: 0, 255, 255, Opacity: 95%) - Primary neon accent
- Alternative color: Hot Pink (RGB: 255, 0, 128) - For emphasis or alternating lines
- Outline/Stroke: Deep purple/magenta, 2-pixel width (RGB: 138, 43, 226)
- Background: Semi-transparent dark box with subtle gradient (RGB: 10, 10, 30, Opacity: 80%)
- Neon glow effect: Cyan outer glow, 4-pixel blur radius for holographic appearance
- Drop shadow: Dark purple, 3-pixel offset with 2-pixel blur for depth

**Positioning and Layout**
- Vertical alignment: Bottom of frame with 10% margin from edge
- Horizontal alignment: Centered with slight left offset (48% from left) for asymmetric cyberpunk aesthetic
- Text alignment: Left-justified for technical/terminal feel
- Maximum width: 85% of video frame width
- Text box style: Subtle corner brackets/frame elements (optional decorative elements)

**Visual Effects - Blade Runner Atmosphere**
- Scan line overlay: Subtle horizontal lines for retro-futuristic CRT effect
- Flicker: Very subtle opacity variation (98%-100%) for holographic realism
- Border accent: Thin cyan line on left edge of text box (2px width)

**Timing and Transitions**
- Subtitle duration: Matches SRT timing exactly
- Fade in: 150ms with slight horizontal slide from left (10px) - Hologram materialization effect
- Fade out: 200ms with glow intensification before disappearing - Energy dissipation effect
- Persistence: Subtle glow lingers 50ms after text fades

#### Video Track Configuration

The video track will be configured to accommodate external footage:

**Expected Video Specifications**
- File format: MP4, MOV, or WebM
- Resolution: Any (MLT will scale to project resolution)
- Duration: 5 seconds (looped or extended as needed)
- Location: `video/background.mp4` (or user-specified filename)

**Looping Behavior**
The MLT configuration will set up the 5-second video to loop continuously for the duration of the audio narration, creating a seamless background.

#### Audio Track Configuration

**Audio Properties**
- File reference: Relative path to `audio/narration.wav`
- Volume: 100% (0 dB)
- Fade in: 50ms at start
- Fade out: 200ms at end

**Synchronization**
Audio serves as the timing master - the video loops and subtitles sync to the audio timeline.

### CLI Interface Changes

#### Updated Command Structure

**Convert Command** (Modified)
Generates a ZIP package instead of separate audio/subtitle files.

```
voxweave convert input.txt [OPTIONS]
```

**Options:**
- `--output, -o <DIR>` - Output directory for the ZIP file (default: same as input file)
- `--voice, -v <VOICE>` - Voice profile identifier
- `--speed, -s <SPEED>` - Playback speed multiplier
- `--subtitles <MODE>` - Subtitle granularity: disabled, sentence, words
- `--words <COUNT>` - Words per subtitle when using words mode
- `--wpm <WPM>` - Average words per minute for subtitle timing
- `--package-name <NAME>` - Custom name for the ZIP package (default: input filename)
- `--no-zip` - Generate files separately without packaging (for development/testing)

**Removed Commands:**
- `video` command entirely removed
- All video-related subcommands eliminated

#### CLI Output Behavior

After processing, the CLI will output:
```
✓ Audio generated: narration.wav
✓ Subtitles generated: narration.srt
✓ MLT project created: project.mlt
✓ Package created: project-name.zip
  Location: /path/to/output/project-name.zip
  
Next steps:
1. Extract the ZIP file
2. Add your 5-second video to the video/ folder
3. Open project.mlt in Kdenlive or another MLT editor
4. Render the final video
```

### UI Interface Changes

#### Updated Workflow

**Main Screen**
- Remove all video style, resolution, and format selectors
- Remove video generation progress indicators
- Add ZIP package export options
- Add package naming field

**Processing Screen**
- Update progress steps to reflect new pipeline:
  1. Processing text
  2. Generating audio
  3. Creating subtitles
  4. Building MLT project
  5. Packaging files

**Output Screen**
- Display path to generated ZIP file
- Show package contents summary
- Provide instructions for adding video and opening in MLT editor
- Add "Open output folder" button

#### Removed UI Components
- Video style selection dropdown
- Resolution picker
- Video format selector
- Video generation progress bar
- Video preview functionality
- API key configuration for video services

### Implementation Components

#### MLT Generator Module

**Responsibilities:**
- Generate valid MLT XML project files
- Configure video, audio, and subtitle tracks
- Apply professional subtitle styling
- Set up video looping for 5-second clips
- Create relative file path references

**Key Methods:**
- `create_mlt_project(audio_path, subtitle_path, config)` - Main project generator
- `configure_subtitle_style()` - Returns subtitle styling XML
- `create_video_track(duration_seconds)` - Sets up video track with looping
- `create_audio_track(audio_path)` - Configures audio timeline
- `create_subtitle_track(srt_path, style)` - Embeds subtitle overlay

#### ZIP Packager Module

**Responsibilities:**
- Create standardized folder structure
- Copy generated files to appropriate directories
- Generate instructional README files
- Create ZIP archive
- Clean up temporary files

**Key Methods:**
- `create_package(audio_path, subtitle_path, mlt_path, output_path)` - Main packaging function
- `create_directory_structure(temp_dir)` - Sets up folder hierarchy
- `generate_readme_files(temp_dir)` - Creates instructional documentation
- `compress_to_zip(source_dir, output_zip)` - Creates final archive

#### Updated Pipeline Module

**Changes to Existing Pipeline:**
- Modify `convert_path` to call MLT generator after subtitle creation
- Add ZIP packaging step as final stage
- Update return type to return ZIP file path instead of audio path
- Remove any video generation branches

**Integration Points:**
- Call MLT generator with audio and subtitle paths
- Pass generated MLT file to ZIP packager
- Return final ZIP package path to caller

### Configuration and Defaults

#### MLT Configuration Defaults

**Project Settings**
- Profile: Automatic (matches video resolution when imported)
- Frame rate: 30 fps (configurable)
- Sample rate: 48000 Hz (matches audio)
- Color space: sRGB with HDR color grading hints for neon colors

**Subtitle Defaults**
- Style: Cyberpunk neon holographic (as specified above)
- Primary color scheme: Cyan/teal with purple accents
- Position: Bottom-left-center (asymmetric)
- Maximum lines: 2 simultaneous lines
- Glow intensity: Medium (adjustable in MLT editor)

**Visual Theme Elements**
- Color palette: Cyberpunk noir (cyan, magenta, purple, dark blue)
- Typography: Monospace/technical fonts
- Effects: Neon glow, subtle scan lines, holographic flicker

#### Package Naming Convention

Default package name derivation:
1. Use input filename stem (without extension)
2. Sanitize for filesystem compatibility
3. Append timestamp if name collision detected
4. Format: `{input-name}-{timestamp}.zip`

User can override with `--package-name` option.

### Error Handling

#### MLT Generation Errors
- Invalid subtitle file format: Log warning, create project without subtitles
- Missing audio file: Fail with clear error message
- XML generation failure: Fail with detailed error context

#### ZIP Packaging Errors
- Insufficient disk space: Fail early with space check
- Permission issues: Fail with actionable error message
- Temporary directory cleanup: Warn but don't fail if cleanup issues

#### User Communication
All errors should provide:
- Clear description of what failed
- Why it failed (when possible)
- Actionable steps to resolve
- Fallback options (e.g., --no-zip flag to get raw files)

### Dependencies

#### Dependencies to Remove
- All video generation API clients
- Video processing libraries
- HTTP client dependencies used only for video APIs

#### Dependencies to Add
- `zip` crate for ZIP archive creation
- XML generation library for MLT files (or use built-in XML string formatting)

#### Dependencies to Retain
- Audio processing dependencies (espeak, etc.)
- Subtitle generation dependencies
- Text processing dependencies
- CLI and UI framework dependencies

### Migration and Cleanup

#### Code Removal Checklist
- Delete `src/video.rs` entirely
- Delete `src/zai_video.rs` entirely
- Delete `src/aliyun_video.rs` entirely
- Delete `abogen-ui/crates/ui/services/video_generation.rs`
- Remove video-related service modules from service mod files
- Remove video enums from queue.rs and state.rs
- Remove video CLI args and handlers from main.rs
- Clean up video feature flag conditionals

#### Configuration File Updates
- Remove video-generation feature from all Cargo.toml files
- Remove video API dependencies
- Add ZIP and MLT generation dependencies
- Update feature flag documentation

#### Documentation Updates
- Update README to reflect new ZIP package workflow
- Remove video generation guides
- Add MLT editor compatibility information
- Document expected video file specifications
- Provide troubleshooting guide for MLT editors

### Testing Strategy

#### Unit Tests
- MLT XML generation produces valid XML structure
- Subtitle styling configuration is correctly formatted
- ZIP package contains expected directory structure
- File path references in MLT are correctly relativized

#### Integration Tests
- Full pipeline from text to ZIP produces valid package
- Generated MLT files open in Kdenlive without errors
- ZIP extraction preserves folder structure
- README files are generated with correct content

#### Manual Validation
- Extract ZIP and verify folder structure
- Add test video file to video/ folder
- Open MLT project in Kdenlive
- Verify subtitles appear with correct styling
- Render final video to confirm functionality

### User Documentation

#### README Content Structure

**Root README.txt:**
- Package overview and purpose
- Contents manifest
- Quick start instructions
- Video file requirements
- Editing workflow guidance

**video/README.txt:**
- Explains purpose of this folder
- Specifies expected video format and duration
- Provides naming convention guidance
- Suggests video file sources

#### Usage Guide Updates

**Workflow Documentation:**
1. Run voxweave convert on text file
2. Extract generated ZIP package
3. Obtain or generate 5-second AI video (external tool)
4. Place video in video/ folder as background.mp4
5. Open project.mlt in MLT-compatible editor
6. Adjust timing or styling if needed
7. Render final narrated video

**MLT Editor Compatibility:**
- Kdenlive: Full support
- Shotcut: Partial support (may need manual subtitle styling)
- OpenShot: Limited support
- MLT command-line tools: Full support

### Future Enhancements

#### Potential Extensions
- Multiple subtitle style presets (cyberpunk neon [default], blade runner noir, casual, formal, cinematic)
- Configurable color schemes (cyan/magenta, amber/purple, green/red)
- Adjustable glow intensity and flicker effects
- Configurable video loop patterns (boomerang, forward, reverse)
- Support for multiple video clips with transitions
- Custom MLT templates with theme variations
- Web-based ZIP package preview with live style editor
- Direct integration with MLT editor APIs
- Additional cyberpunk UI elements (corner brackets, scanlines, glitch effects)

#### Backward Compatibility
No backward compatibility required since this is a breaking change that fundamentally alters the tool's purpose. Users relying on video generation will need to transition to external video creation workflows.
- Explains purpose of this folder
- Specifies expected video format and duration
- Provides naming convention guidance
- Suggests video file sources

#### Usage Guide Updates

**Workflow Documentation:**
1. Run voxweave convert on text file
2. Extract generated ZIP package
3. Obtain or generate 5-second AI video (external tool)
4. Place video in video/ folder as background.mp4
5. Open project.mlt in MLT-compatible editor
6. Adjust timing or styling if needed
7. Render final narrated video

**MLT Editor Compatibility:**
- Kdenlive: Full support
- Shotcut: Partial support (may need manual subtitle styling)
- OpenShot: Limited support
- MLT command-line tools: Full support

### Future Enhancements

#### Potential Extensions
- Multiple subtitle style presets (cyberpunk neon [default], blade runner noir, casual, formal, cinematic)
- Configurable color schemes (cyan/magenta, amber/purple, green/red)
- Adjustable glow intensity and flicker effects
- Configurable video loop patterns (boomerang, forward, reverse)
- Support for multiple video clips with transitions
- Custom MLT templates with theme variations
- Web-based ZIP package preview with live style editor
- Direct integration with MLT editor APIs
- Additional cyberpunk UI elements (corner brackets, scanlines, glitch effects)

#### Backward Compatibility
No backward compatibility required since this is a breaking change that fundamentally alters the tool's purpose. Users relying on video generation will need to transition to external video creation workflows.

