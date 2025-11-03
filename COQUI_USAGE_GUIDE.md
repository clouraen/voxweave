# CoquiTTS Integration - Complete Guide

## ✅ Implementation Status

The CoquiTTS integration is now **fully implemented** in VoxWeave! This includes:

- ✅ CoquiEngine backend for TTS synthesis
- ✅ Voice cloning support with reference audio
- ✅ Audio recording component using cpal
- ✅ Integration with UI services
- ✅ Support for 16+ languages
- ✅ Python bridge for XTTS v2 model

---

## 🚀 Getting Started

### 1. Install Python Dependencies

CoquiTTS requires Python 3.9+ and the following packages:

```bash
pip install TTS torch numpy
```

For GPU acceleration (recommended):
```bash
# For CUDA (NVIDIA GPUs)
pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu118

# For MPS (Apple Silicon)
# PyTorch will automatically use MPS if available
```

### 2. Verify Installation

Run the test script to verify your installation:

```bash
cd /Users/cleitonmouraloura/Documents/voxweave
python3 test_coqui.py
```

### 3. Build with CoquiTTS Feature

```bash
# Build the core library
cargo build --features coqui-tts

# Build the UI with CoquiTTS support
cd abogen-ui
cargo build --features coqui-tts

# Build desktop app with all features
cd apps/desktop
cargo build --release --features coqui-tts
```

---

## 🎤 Voice Cloning

### Using the Audio Recorder Component

The `AudioRecorder` component allows you to record your voice for cloning:

1. **Enable coqui-tts feature** when building
2. **Click "Start Recording"** - speaks for 5 seconds
3. **Audio is automatically saved** to Desktop as `voice_clone_<timestamp>.wav`
4. **Use the path** when creating a cloned voice profile

### Programmatic Voice Cloning

```rust
use voxweave::tts::{VoiceProfile, SpeechEngine};
use voxweave::coqui_tts::CoquiEngine;

// Create a cloned voice profile
let voice = VoiceProfile::coqui_clone(
    "my-clone",
    "My Cloned Voice",
    "en",
    "/path/to/reference/audio.wav"
);

// Use it for synthesis
let engine = CoquiEngine::default();
engine.synthesize_to_file(
    "This is synthesized with my cloned voice!",
    &voice,
    1.0,
    &output_path,
)?;
```

---

## 🌍 Supported Languages

CoquiTTS XTTS v2 supports multiple languages:

| Language | Code | Example Voice ID |
|----------|------|------------------|
| English | `en` | `coqui-en` |
| Spanish | `es` | `coqui-es` |
| French | `fr` | `coqui-fr` |
| German | `de` | `coqui-de` |
| Italian | `it` | `coqui-it` |
| Portuguese | `pt` | `coqui-pt` |
| Polish | `pl` | `coqui-pl` |
| Turkish | `tr` | `coqui-tr` |
| Russian | `ru` | `coqui-ru` |
| Dutch | `nl` | `coqui-nl` |
| Czech | `cs` | `coqui-cs` |
| Arabic | `ar` | `coqui-ar` |
| Chinese | `zh-cn` | `coqui-zh-cn` |
| Japanese | `ja` | `coqui-ja` |
| Hungarian | `hu` | `coqui-hu` |
| Korean | `ko` | `coqui-ko` |

---

## ⚙️ Configuration

### Environment Variables

Configure CoquiTTS behavior with these environment variables:

```bash
# Python command (default: python3)
export VOXWEAVE_COQUI_PYTHON="python3"

# TTS model (default: tts_models/multilingual/multi-dataset/xtts_v2)
export VOXWEAVE_COQUI_MODEL="tts_models/multilingual/multi-dataset/xtts_v2"

# Device (cpu, cuda, mps, etc.)
export VOXWEAVE_COQUI_DEVICE="cpu"

# Sample rate (default: 24000)
export VOXWEAVE_COQUI_SAMPLE_RATE="24000"

# Language (default: en)
export VOXWEAVE_COQUI_LANGUAGE="en"
```

### Example Configuration

For Apple Silicon Macs with GPU acceleration:
```bash
export VOXWEAVE_COQUI_DEVICE="mps"
```

For NVIDIA GPUs:
```bash
export VOXWEAVE_COQUI_DEVICE="cuda"
```

---

## 🔧 Architecture

### Components

1. **CoquiEngine** (`src/coqui_tts.rs`)
   - Implements `SpeechEngine` trait
   - Manages Python bridge communication
   - Handles voice cloning via speaker reference audio

2. **Python Bridge** (inline in `src/tts.rs`)
   - `COQUI_BRIDGE_SCRIPT` constant
   - Accepts JSON payload via stdin
   - Returns metadata via stdout

3. **VoiceProfile** Extensions
   - `VoiceProfile::coqui()` - Standard Coqui voices
   - `VoiceProfile::coqui_clone()` - Voice cloning with reference audio

4. **AudioRecorder Component** (`abogen-ui/crates/ui/components/audio_recorder.rs`)
   - Uses `cpal` for microphone capture
   - Uses `hound` for WAV file writing
   - Provides real-time recording feedback

5. **TTS Service** (`abogen-ui/crates/ui/services/tts_service.rs`)
   - Routes requests to appropriate engine (Kokoro or Coqui)
   - Handles async synthesis in background threads

### Data Flow

```
User Input → TTS Service
              ↓
       Voice Profile Analysis
              ↓
    ┌─────────┴─────────┐
    ↓                   ↓
KokoroEngine      CoquiEngine
    ↓                   ↓
    └─────────┬─────────┘
              ↓
        Audio Output
```

---

## 🧪 Testing

### Run Unit Tests

```bash
# Core library tests
cargo test --features coqui-tts

# Run integration tests (requires Python TTS installed)
cargo test --features coqui-tts -- --ignored
```

### Manual Testing

1. **Test basic synthesis**:
```bash
# Build and run with Coqui enabled
cd abogen-ui/apps/desktop
cargo run --release --features coqui-tts
```

2. **Test voice cloning**:
   - Record audio using the AudioRecorder component
   - Select the recorded file as reference
   - Process text with cloned voice

---

## 📝 Code Examples

### Basic Usage in CLI

```rust
use voxweave::coqui_tts::CoquiEngine;
use voxweave::tts::{SpeechEngine, VoiceProfile};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = CoquiEngine::default();
    let voice = VoiceProfile::coqui("coqui-en", "English", "en");
    let output = PathBuf::from("output.wav");
    
    engine.synthesize_to_file(
        "Hello from CoquiTTS!",
        &voice,
        1.0,
        &output,
    )?;
    
    println!("Audio generated: {:?}", output);
    Ok(())
}
```

### Using in UI Service

```rust
use voxweave::coqui_tts::CoquiEngine;
use voxweave::tts::VoiceEngine;

// In process_queue function
match voice.engine {
    VoiceEngine::Coqui => {
        let engine = CoquiEngine::default();
        engine.synthesize_to_file(&text, &voice, speed, &output)?;
    }
    VoiceEngine::Kokoro => {
        let engine = KokoroEngine::default();
        engine.synthesize_to_file(&text, &voice, speed, &output)?;
    }
    _ => { /* fallback */ }
}
```

---

## 🐛 Troubleshooting

### Issue: "Failed to import coqui dependencies"

**Solution**: Install Python dependencies
```bash
pip install TTS torch numpy
```

### Issue: "No input device available"

**Solution**: Check microphone permissions
- macOS: System Preferences → Security & Privacy → Microphone
- Ensure your app has microphone access

### Issue: "Model download is slow"

**Solution**: The XTTS v2 model is ~1.8GB and downloads on first use. Be patient!

### Issue: "CUDA out of memory"

**Solution**: Switch to CPU mode
```bash
export VOXWEAVE_COQUI_DEVICE="cpu"
```

### Issue: "Speaker reference audio not found"

**Solution**: Verify the audio file path exists and is a valid WAV file

---

## 🎯 Next Steps

Now that CoquiTTS is implemented, you can:

1. **Record your voice** using the AudioRecorder component
2. **Create custom voice profiles** with your recordings
3. **Generate high-quality narration** in 16+ languages
4. **Clone any voice** with just a few seconds of reference audio
5. **Integrate with video generation** for complete multimedia pipelines

---

## 📚 Additional Resources

- [CoquiTTS GitHub](https://github.com/coqui-ai/TTS)
- [XTTS v2 Documentation](https://docs.coqui.ai/en/latest/models/xtts.html)
- [PyTorch Installation](https://pytorch.org/get-started/locally/)
- [VoxWeave Documentation](../README.md)

---

**Status**: ✅ **Production Ready**

All components are implemented and tested. Voice cloning and multi-language synthesis are fully functional!
# CoquiTTS Integration - Complete Guide

## ✅ Implementation Status

The CoquiTTS integration is now **fully implemented** in VoxWeave! This includes:

- ✅ CoquiEngine backend for TTS synthesis
- ✅ Voice cloning support with reference audio
- ✅ Audio recording component using cpal
- ✅ Integration with UI services
- ✅ Support for 16+ languages
- ✅ Python bridge for XTTS v2 model

---

## 🚀 Getting Started

### 1. Install Python Dependencies

CoquiTTS requires Python 3.9+ and the following packages:

```bash
pip install TTS torch numpy
```

For GPU acceleration (recommended):
```bash
# For CUDA (NVIDIA GPUs)
pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu118

# For MPS (Apple Silicon)
# PyTorch will automatically use MPS if available
```

### 2. Verify Installation

Run the test script to verify your installation:

```bash
cd /Users/cleitonmouraloura/Documents/voxweave
python3 test_coqui.py
```

### 3. Build with CoquiTTS Feature

```bash
# Build the core library
cargo build --features coqui-tts

# Build the UI with CoquiTTS support
cd abogen-ui
cargo build --features coqui-tts

# Build desktop app with all features
cd apps/desktop
cargo build --release --features coqui-tts
```

---

## 🎤 Voice Cloning

### Using the Audio Recorder Component

The `AudioRecorder` component allows you to record your voice for cloning:

1. **Enable coqui-tts feature** when building
2. **Click "Start Recording"** - speaks for 5 seconds
3. **Audio is automatically saved** to Desktop as `voice_clone_<timestamp>.wav`
4. **Use the path** when creating a cloned voice profile

### Programmatic Voice Cloning

```rust
use voxweave::tts::{VoiceProfile, SpeechEngine};
use voxweave::coqui_tts::CoquiEngine;

// Create a cloned voice profile
let voice = VoiceProfile::coqui_clone(
    "my-clone",
    "My Cloned Voice",
    "en",
    "/path/to/reference/audio.wav"
);

// Use it for synthesis
let engine = CoquiEngine::default();
engine.synthesize_to_file(
    "This is synthesized with my cloned voice!",
    &voice,
    1.0,
    &output_path,
)?;
```

---

## 🌍 Supported Languages

CoquiTTS XTTS v2 supports multiple languages:

| Language | Code | Example Voice ID |
|----------|------|------------------|
| English | `en` | `coqui-en` |
| Spanish | `es` | `coqui-es` |
| French | `fr` | `coqui-fr` |
| German | `de` | `coqui-de` |
| Italian | `it` | `coqui-it` |
| Portuguese | `pt` | `coqui-pt` |
| Polish | `pl` | `coqui-pl` |
| Turkish | `tr` | `coqui-tr` |
| Russian | `ru` | `coqui-ru` |
| Dutch | `nl` | `coqui-nl` |
| Czech | `cs` | `coqui-cs` |
| Arabic | `ar` | `coqui-ar` |
| Chinese | `zh-cn` | `coqui-zh-cn` |
| Japanese | `ja` | `coqui-ja` |
| Hungarian | `hu` | `coqui-hu` |
| Korean | `ko` | `coqui-ko` |

---

## ⚙️ Configuration

### Environment Variables

Configure CoquiTTS behavior with these environment variables:

```bash
# Python command (default: python3)
export VOXWEAVE_COQUI_PYTHON="python3"

# TTS model (default: tts_models/multilingual/multi-dataset/xtts_v2)
export VOXWEAVE_COQUI_MODEL="tts_models/multilingual/multi-dataset/xtts_v2"

# Device (cpu, cuda, mps, etc.)
export VOXWEAVE_COQUI_DEVICE="cpu"

# Sample rate (default: 24000)
export VOXWEAVE_COQUI_SAMPLE_RATE="24000"

# Language (default: en)
export VOXWEAVE_COQUI_LANGUAGE="en"
```

### Example Configuration

For Apple Silicon Macs with GPU acceleration:
```bash
export VOXWEAVE_COQUI_DEVICE="mps"
```

For NVIDIA GPUs:
```bash
export VOXWEAVE_COQUI_DEVICE="cuda"
```

---

## 🔧 Architecture

### Components

1. **CoquiEngine** (`src/coqui_tts.rs`)
   - Implements `SpeechEngine` trait
   - Manages Python bridge communication
   - Handles voice cloning via speaker reference audio

2. **Python Bridge** (inline in `src/tts.rs`)
   - `COQUI_BRIDGE_SCRIPT` constant
   - Accepts JSON payload via stdin
   - Returns metadata via stdout

3. **VoiceProfile** Extensions
   - `VoiceProfile::coqui()` - Standard Coqui voices
   - `VoiceProfile::coqui_clone()` - Voice cloning with reference audio

4. **AudioRecorder Component** (`abogen-ui/crates/ui/components/audio_recorder.rs`)
   - Uses `cpal` for microphone capture
   - Uses `hound` for WAV file writing
   - Provides real-time recording feedback

5. **TTS Service** (`abogen-ui/crates/ui/services/tts_service.rs`)
   - Routes requests to appropriate engine (Kokoro or Coqui)
   - Handles async synthesis in background threads

### Data Flow

```
User Input → TTS Service
              ↓
       Voice Profile Analysis
              ↓
    ┌─────────┴─────────┐
    ↓                   ↓
KokoroEngine      CoquiEngine
    ↓                   ↓
    └─────────┬─────────┘
              ↓
        Audio Output
```

---

## 🧪 Testing

### Run Unit Tests

```bash
# Core library tests
cargo test --features coqui-tts

# Run integration tests (requires Python TTS installed)
cargo test --features coqui-tts -- --ignored
```

### Manual Testing

1. **Test basic synthesis**:
```bash
# Build and run with Coqui enabled
cd abogen-ui/apps/desktop
cargo run --release --features coqui-tts
```

2. **Test voice cloning**:
   - Record audio using the AudioRecorder component
   - Select the recorded file as reference
   - Process text with cloned voice

---

## 📝 Code Examples

### Basic Usage in CLI

```rust
use voxweave::coqui_tts::CoquiEngine;
use voxweave::tts::{SpeechEngine, VoiceProfile};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = CoquiEngine::default();
    let voice = VoiceProfile::coqui("coqui-en", "English", "en");
    let output = PathBuf::from("output.wav");
    
    engine.synthesize_to_file(
        "Hello from CoquiTTS!",
        &voice,
        1.0,
        &output,
    )?;
    
    println!("Audio generated: {:?}", output);
    Ok(())
}
```

### Using in UI Service

```rust
use voxweave::coqui_tts::CoquiEngine;
use voxweave::tts::VoiceEngine;

// In process_queue function
match voice.engine {
    VoiceEngine::Coqui => {
        let engine = CoquiEngine::default();
        engine.synthesize_to_file(&text, &voice, speed, &output)?;
    }
    VoiceEngine::Kokoro => {
        let engine = KokoroEngine::default();
        engine.synthesize_to_file(&text, &voice, speed, &output)?;
    }
    _ => { /* fallback */ }
}
```

---

## 🐛 Troubleshooting

### Issue: "Failed to import coqui dependencies"

**Solution**: Install Python dependencies
```bash
pip install TTS torch numpy
```

### Issue: "No input device available"

**Solution**: Check microphone permissions
- macOS: System Preferences → Security & Privacy → Microphone
- Ensure your app has microphone access

### Issue: "Model download is slow"

**Solution**: The XTTS v2 model is ~1.8GB and downloads on first use. Be patient!

### Issue: "CUDA out of memory"

**Solution**: Switch to CPU mode
```bash
export VOXWEAVE_COQUI_DEVICE="cpu"
```

### Issue: "Speaker reference audio not found"

**Solution**: Verify the audio file path exists and is a valid WAV file

---

## 🎯 Next Steps

Now that CoquiTTS is implemented, you can:

1. **Record your voice** using the AudioRecorder component
2. **Create custom voice profiles** with your recordings
3. **Generate high-quality narration** in 16+ languages
4. **Clone any voice** with just a few seconds of reference audio
5. **Integrate with video generation** for complete multimedia pipelines

---

## 📚 Additional Resources

- [CoquiTTS GitHub](https://github.com/coqui-ai/TTS)
- [XTTS v2 Documentation](https://docs.coqui.ai/en/latest/models/xtts.html)
- [PyTorch Installation](https://pytorch.org/get-started/locally/)
- [VoxWeave Documentation](../README.md)

---

**Status**: ✅ **Production Ready**

All components are implemented and tested. Voice cloning and multi-language synthesis are fully functional!
