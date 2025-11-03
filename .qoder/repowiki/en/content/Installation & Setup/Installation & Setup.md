# Installation & Setup

<cite>
**Referenced Files in This Document**   
- [Cargo.toml](file://Cargo.toml)
- [src/coqui_tts.rs](file://src/coqui_tts.rs)
- [build.sh](file://build.sh)
- [run_demo.sh](file://run_demo.sh)
- [verify.sh](file://verify.sh)
- [test_coqui.py](file://test_coqui.py)
- [QUICKSTART.md](file://QUICKSTART.md)
- [COQUI_USAGE_GUIDE.md](file://COQUI_USAGE_GUIDE.md)
- [abogen-ui/apps/desktop/Cargo.toml](file://abogen-ui/apps/desktop/Cargo.toml)
- [abogen-ui/apps/mobile/Cargo.toml](file://abogen-ui/apps/mobile/Cargo.toml)
- [abogen-ui/apps/web/Cargo.toml](file://abogen-ui/apps/web/Cargo.toml)
</cite>

## Table of Contents
1. [Prerequisites](#prerequisites)
2. [Core Project Setup](#core-project-setup)
3. [CoquiTTS Integration](#coquitts-integration)
4. [Environment Configuration](#environment-configuration)
5. [Building with Feature Flags](#building-with-feature-flags)
6. [UI Application Setup](#ui-application-setup)
7. [Verification and Testing](#verification-and-testing)
8. [Troubleshooting Common Issues](#troubleshooting-common-issues)

## Prerequisites

Before setting up the VoxWeave development environment, ensure the following prerequisites are installed:

- **Rust 1.80+**: Required for building the core project and UI applications. Install via [rustup](https://www.rust-lang.org/tools/install).
- **Python 3.8+ with pip**: Required for CoquiTTS integration and helper scripts. Python 3.9+ is recommended for full compatibility.
- **System Dependencies**:
  - `espeak-ng`: Text-to-speech engine for basic synthesis. Install via package manager:
    - macOS: `brew install espeak-ng`
    - Ubuntu: `sudo apt-get install espeak-ng`
    - Windows: Available via Chocolatey or MSYS2
  - `cpal` and `hound`: Audio recording and WAV file handling (Rust crates, installed automatically)
  - `torch`, `TTS`, `numpy`: Python packages for CoquiTTS backend

**Section sources**
- [FIXES_APPLIED.md](file://FIXES_APPLIED.md#L533-L546)
- [COQUI_USAGE_GUIDE.md](file://COQUI_USAGE_GUIDE.md#L0-L50)

## Core Project Setup

The VoxWeave core project is built using Cargo, Rust's package manager. Follow these steps to set up the development environment:

1. **Clone the repository** (if not already done):
   ```bash
   git clone https://github.com/voxweave/voxweave.git
   cd voxweave
   ```

2. **Build the project in debug mode**:
   ```bash
   cargo build
   ```

3. **Build the project in release mode**:
   ```bash
   cargo build --release
   ```

4. **Run unit tests**:
   ```bash
   cargo test
   ```

The `build.sh` script automates the build and test process:
```bash
./build.sh
```

This script compiles the project in release mode and runs all unit tests to ensure functionality.

**Section sources**
- [build.sh](file://build.sh#L0-L19)
- [Cargo.toml](file://Cargo.toml#L0-L26)

## CoquiTTS Integration

To enable advanced text-to-speech capabilities with voice cloning and multi-language support, integrate CoquiTTS:

1. **Install Python dependencies**:
   ```bash
   pip install TTS torch numpy
   ```

2. **For GPU acceleration**:
   - **NVIDIA GPUs (CUDA)**:
     ```bash
     pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu118
     ```
   - **Apple Silicon (MPS)**: PyTorch automatically uses MPS when available.

3. **Verify CoquiTTS installation** using the provided test script:
   ```bash
   python3 test_coqui.py
   ```
   This script checks for proper installation of `torch`, `TTS`, and `numpy`, and tests model initialization.

4. **Enable the `coqui-tts` feature** when building:
   ```bash
   cargo build --features coqui-tts
   ```

The CoquiEngine implementation in `src/coqui_tts.rs` handles communication with the Python bridge and supports voice cloning via reference audio.

**Section sources**
- [test_coqui.py](file://test_coqui.py#L0-L140)
- [src/coqui_tts.rs](file://src/coqui_tts.rs#L0-L115)
- [COQUI_USAGE_GUIDE.md](file://COQUI_USAGE_GUIDE.md#L0-L100)

## Environment Configuration

Configure VoxWeave behavior using environment variables:

| Environment Variable | Description | Default Value |
|----------------------|-------------|---------------|
| `ZAI_API_KEY` | Required API key for ZAI video features | (none) |
| `VOXWEAVE_COQUI_PYTHON` | Python command for CoquiTTS bridge | `python3` |
| `VOXWEAVE_COQUI_MODEL` | TTS model name | `tts_models/multilingual/multi-dataset/xtts_v2` |
| `VOXWEAVE_COQUI_DEVICE` | Device for TTS synthesis (`cpu`, `cuda`, `mps`) | `cpu` |
| `VOXWEAVE_COQUI_SAMPLE_RATE` | Audio sample rate | `24000` |
| `VOXWEAVE_COQUI_LANGUAGE` | Default language | `en` |

Example configuration for Apple Silicon Macs:
```bash
export VOXWEAVE_COQUI_DEVICE="mps"
export ZAI_API_KEY="your_api_key_here"
```

For NVIDIA GPUs:
```bash
export VOXWEAVE_COQUI_DEVICE="cuda"
```

**Section sources**
- [src/coqui_tts.rs](file://src/coqui_tts.rs#L10-L30)
- [COQUI_USAGE_GUIDE.md](file://COQUI_USAGE_GUIDE.md#L250-L300)

## Building with Feature Flags

VoxWeave supports multiple feature combinations for different use cases:

```bash
# Build with CoquiTTS support
cargo build --features coqui-tts

# Build with video generation (enables tokio and reqwest)
cargo build --features video-generation

# Build with both CoquiTTS and video generation
cargo build --features "coqui-tts video-generation"

# Build UI with CoquiTTS and video features
cd abogen-ui
cargo build --features coqui-tts,video-generation
```

The available features are defined in `Cargo.toml`:
- `coqui-tts`: Enables Coqui TTS engine with Python bridge
- `video-generation`: Enables asynchronous processing and HTTP requests for video features

**Section sources**
- [Cargo.toml](file://Cargo.toml#L20-L26)

## UI Application Setup

The VoxWeave UI is built with Dioxus and supports desktop, web, and mobile platforms:

### Desktop Application
```bash
cd abogen-ui/apps/desktop
cargo build --release --features coqui-tts
```

The desktop app includes all features: video generation, real TTS, ZAI video, and CoquiTTS.

**Section sources**
- [abogen-ui/apps/desktop/Cargo.toml](file://abogen-ui/apps/desktop/Cargo.toml#L0-L13)

### Web Application (Trunk)
```bash
cd abogen-ui/apps/web
trunk serve
```

Uses Trunk for bundling and development server.

**Section sources**
- [abogen-ui/apps/web/Cargo.toml](file://abogen-ui/apps/web/Cargo.toml#L0-L15)

### Mobile Application
```bash
cd abogen-ui/apps/mobile
cargo build --target aarch64-apple-ios  # for iOS
```

**Section sources**
- [abogen-ui/apps/mobile/Cargo.toml](file://abogen-ui/apps/mobile/Cargo.toml#L0-L13)

## Verification and Testing

Verify the installation using the provided scripts:

1. **Run the demo script**:
   ```bash
   ./run_demo.sh
   ```
   This script builds the project, lists available voices, and runs a mock conversion.

2. **Run comprehensive verification**:
   ```bash
   ./verify.sh
   ```
   This script performs five tests:
   - Build compilation
   - Unit tests (17 tests)
   - CLI help command
   - List voices command
   - Text conversion in mock mode

A successful verification outputs:
```
🎉 ALL TESTS PASSED!
Project Status: FULLY OPERATIONAL ✨
```

**Section sources**
- [run_demo.sh](file://run_demo.sh#L0-L80)
- [verify.sh](file://verify.sh#L0-L140)
- [QUICKSTART.md](file://QUICKSTART.md#L0-L222)

## Troubleshooting Common Issues

### Python Dependency Conflicts
- **Issue**: Import errors for `torch` or `TTS`
- **Solution**: Create a virtual environment:
  ```bash
  python -m venv venv
  source venv/bin/activate  # Linux/macOS
  venv\Scripts\activate     # Windows
  pip install TTS torch numpy
  ```

### GPU Acceleration Configuration
- **Issue**: CUDA out of memory
- **Solution**: Switch to CPU mode:
  ```bash
  export VOXWEAVE_COQUI_DEVICE="cpu"
  ```

- **Issue**: MPS not detected on Apple Silicon
- **Solution**: Ensure PyTorch is installed with MPS support:
  ```bash
  pip install torch --index-url https://download.pytorch.org/whl/cpu
  ```

### Missing Input Devices
- **Issue**: "No input device available" when recording
- **Solution**: Check microphone permissions:
  - macOS: System Preferences → Security & Privacy → Microphone
  - Ensure the application has microphone access

### Slow Model Downloads
- **Issue**: CoquiTTS model download is slow (1.8GB)
- **Solution**: The model downloads on first use. Be patient during initial setup.

**Section sources**
- [COQUI_USAGE_GUIDE.md](file://COQUI_USAGE_GUIDE.md#L500-L600)
- [test_coqui.py](file://test_coqui.py#L0-L140)