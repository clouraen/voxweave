# VoxWeave

VoxWeave is a Rust-first text-to-speech pipeline with a focus on test-driven
development. The project reads plain text or markdown files, normalises and
chunks their content, generates audio via `espeak-ng`,
[`kokoro`](https://github.com/hexgrad/kokoro), or
[CoquiTTS](https://github.com/coqui-ai/TTS) with voice cloning support,
and optionally emits timed SRT subtitles.

## Migration Notes

- The CLI binary is now `voxweave`. Update any scripts that relied on an older binary name.
- Config files live under the application identifier `com/voxweave/voxweave`. Set `VOXWEAVE_CONFIG_DIR` or copy over existing data if you want to keep history.
- Environment overrides now use the `VOXWEAVE_*` prefix; rename any remaining legacy variables to avoid future breakage.

## Requirements

- Rust 1.80+ (edition 2021)
- [`espeak-ng`](https://github.com/espeak-ng/espeak-ng) in `PATH` (or override
  the binary name with `VOXWEAVE_ESPEAK_COMMAND`)
- Optional (for Kokoro voices):
  - Python 3.9+
  - `pip install kokoro numpy torch` (plus any backend-specific dependencies)
  - Override the python executable with `VOXWEAVE_KOKORO_PYTHON` if needed
- Optional (for CoquiTTS with voice cloning):
  - Python 3.9+
  - `pip install TTS torch numpy`
  - Enable with `--features coqui-tts` when building
  - See `COQUI_USAGE_GUIDE.md` for detailed setup
- Optional: set `VOXWEAVE_CONFIG_DIR` when testing to isolate config files

For macOS you can install `espeak-ng` using `brew install espeak-ng`. On Linux
use your distribution's package manager.

## Quick start

```bash
cargo run -- list-voices
cargo run -- convert samples/example.txt --output output_dir
```

Use `--mock` to run the pipeline without synthesising audio (helpful for fast
feedback during development).

To convert with Kokoro voices, pick any voice ID starting with language and
gender markers (for example `af_alloy`):

```bash
cargo run -- convert samples/example.txt --voice af_alloy --output output_dir
```

### Using CoquiTTS with Voice Cloning

Build with CoquiTTS support:
```bash
cargo build --features coqui-tts
```

For detailed CoquiTTS usage including voice cloning, see:
- `COQUI_USAGE_GUIDE.md` - Complete usage guide
- `IMPLEMENTATION_SUMMARY.md` - Implementation details

## Test suite

All modules were developed with TDD. Run the full suite with:

```bash
cargo test
```

## Project structure

- `src/text.rs`: text normalisation helpers
- `src/sanitize.rs`: cross-platform file-name sanitisation
- `src/subtitle.rs`: subtitle chunking and SRT formatting
- `src/pipeline.rs`: high-level conversion orchestration
- `src/tts.rs`: speech engines (mock, `espeak-ng`, Kokoro bridge)
- `src/coqui_tts.rs`: CoquiTTS engine with voice cloning (feature: `coqui-tts`)
- `src/main.rs`: CLI frontend using `clap`
- `python/kokoro_bridge.py`: lightweight Python helper used to run Kokoro
- `abogen-ui/`: Multi-platform UI (desktop, mobile, web)
  - `crates/ui/components/audio_recorder.rs`: Real-time audio recording for voice cloning
  - `crates/ui/services/tts_service.rs`: Multi-engine TTS orchestration

## Environment overrides

### Espeak-ng
- `VOXWEAVE_ESPEAK_COMMAND`: override the `espeak-ng` executable

### Kokoro
- `VOXWEAVE_KOKORO_PYTHON`: path to the python interpreter for Kokoro
- `VOXWEAVE_KOKORO_REPO_ID`: alternate Hugging Face model repo (defaults to
  `hexgrad/Kokoro-82M`)
- `VOXWEAVE_KOKORO_DEVICE`: device passed to `KPipeline` (`cpu`, `cuda`, `mps`, …)
- `VOXWEAVE_KOKORO_SAMPLE_RATE`: override the WAV sample rate (defaults to 24000)
- `VOXWEAVE_KOKORO_SPLIT_PATTERN`: regex forwarded to Kokoro for sentence
  splitting (defaults to `\n+`)

### CoquiTTS (requires `coqui-tts` feature)
- `VOXWEAVE_COQUI_PYTHON`: Python command (default: `python3`)
- `VOXWEAVE_COQUI_MODEL`: TTS model (default: `tts_models/multilingual/multi-dataset/xtts_v2`)
- `VOXWEAVE_COQUI_DEVICE`: Device for inference (`cpu`, `cuda`, `mps`, etc.)
- `VOXWEAVE_COQUI_SAMPLE_RATE`: Sample rate (default: 24000)
- `VOXWEAVE_COQUI_LANGUAGE`: Default language (default: `en`)

## Features

- `default`: Basic TTS with espeak-ng and Kokoro
- `coqui-tts`: Enable CoquiTTS with voice cloning support
  - Adds real-time audio recording via `cpal`
  - Voice cloning from reference audio
  - 16+ language support
- `video-generation`: Enable AI video generation from text
  - Generates narrated videos with AI-generated visuals
  - Multiple video styles (realistic, anime, cinematic, etc.)
  - Subtitle embedding support
  - See `CLI_VIDEO_GENERATION.md` for detailed guide

### Video Generation (CLI)

Build with video generation support:
```bash
cargo build --features video-generation
```

Generate a video from text:
```bash
export ZAI_API_KEY=your_api_key_here
voxweave video input.txt --style cinematic --resolution 1080p
```

For complete video generation documentation, see `CLI_VIDEO_GENERATION.md`.
