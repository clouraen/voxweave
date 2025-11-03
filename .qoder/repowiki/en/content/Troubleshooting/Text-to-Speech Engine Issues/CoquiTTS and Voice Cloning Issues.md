# CoquiTTS and Voice Cloning Issues

<cite>
**Referenced Files in This Document**   
- [COQUI_USAGE_GUIDE.md](file://COQUI_USAGE_GUIDE.md)
- [test_coqui.py](file://test_coqui.py)
- [src/coqui_tts.rs](file://src/coqui_tts.rs)
- [src/tts.rs](file://src/tts.rs)
- [python/kokoro_bridge.py](file://python/kokoro_bridge.py)
- [abogen-ui/crates/ui/components/audio_recorder.rs](file://abogen-ui/crates/ui/components/audio_recorder.rs)
</cite>

## Table of Contents
1. [Introduction](#introduction)
2. [Python Dependency Issues](#python-dependency-issues)
3. [Voice Cloning Failures](#voice-cloning-failures)
4. [Device Configuration Problems](#device-configuration-problems)
5. [Model Download and Verification](#model-download-and-verification)
6. [Microphone Permission Issues](#microphone-permission-issues)
7. [COQUI_BRIDGE_SCRIPT Debugging](#coqui_bridge_script-debugging)
8. [CoquiEngine Configuration](#coquiengine-configuration)

## Introduction
This document provides comprehensive troubleshooting guidance for CoquiTTS engine and voice cloning issues in the VoxWeave application. It covers common problems related to Python dependencies, voice cloning, device configuration, model downloads, microphone permissions, and bridge script execution. The documentation references the CoquiEngine's default configuration and explains how environment variables override these defaults to customize behavior.

## Python Dependency Issues
Python dependency problems are common when setting up the CoquiTTS engine. The system requires specific packages to function properly.

### Missing TTS, torch, and numpy Packages
The CoquiTTS integration requires three essential Python packages: TTS, torch, and numpy. When these are missing, the system fails to initialize the TTS engine.

**Installation Commands by Platform:**

For standard installation:
```bash
pip install TTS torch numpy
```

For NVIDIA GPU (CUDA) support:
```bash
pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu118
pip install TTS numpy
```

For Apple Silicon (MPS) support:
```bash
pip install torch torchvision torchaudio
pip install TTS numpy
```

**Verification:**
The `test_coqui.py` script verifies the installation of these dependencies by attempting to import them and testing basic functionality. This script outputs clear success or failure indicators for each required package.

**Section sources**
- [COQUI_USAGE_GUIDE.md](file://COQUI_USAGE_GUIDE.md#L43-L67)
- [test_coqui.py](file://test_coqui.py#L10-L40)

## Voice Cloning Failures
Voice cloning issues typically stem from problems with reference audio files stored in the VoiceProfile::command field.

### File Not Found Errors
The voice cloning system stores the reference audio path in the `command` field of the `VoiceProfile` struct. When this path is invalid or the file doesn't exist, cloning fails.

```rust
pub fn coqui_clone(id: &str, description: &str, lang: &str, clone_path: &str) -> Self {
    Self {
        id: id.to_owned(),
        description: description.to_owned(),
        engine: VoiceEngine::Coqui,
        command: Some(clone_path.to_owned()),  // Store clone audio path in command field
        lang: Some(lang.to_owned()),
    }
}
```

The system checks for file existence before attempting cloning:
```rust
let speaker_wav = voice.command.as_deref();
let _use_cloning = speaker_wav.is_some() && Path::new(speaker_wav.unwrap()).exists();
```

**Troubleshooting Steps:**
1. Verify the file path stored in the VoiceProfile::command field
2. Ensure the file has not been moved or deleted
3. Check file permissions to ensure read access
4. Confirm the path uses the correct format for the operating system

### Invalid WAV Format Issues
The AudioRecorder component saves recordings in WAV format using the hound library. Issues can occur if the WAV file is corrupted or in an unsupported format.

The recording process creates a WAV file with specific parameters:
- Sample rate: Matches the input device's default configuration
- Channels: As specified by the input device
- Bits per sample: 16
- Sample format: Integer

**Section sources**
- [src/tts.rs](file://src/tts.rs#L453-L471)
- [src/coqui_tts.rs](file://src/coqui_tts.rs#L55-L60)
- [abogen-ui/crates/ui/components/audio_recorder.rs](file://abogen-ui/crates/ui/components/audio_recorder.rs#L100-L115)

## Device Configuration Problems
Device configuration issues arise from incorrect settings in the VOXWEAVE_COQUI_DEVICE environment variable.

### Device Options
The CoquiEngine supports multiple device types for TTS processing:
- `cpu`: Standard CPU processing
- `cuda`: NVIDIA GPU acceleration
- `mps`: Apple Silicon GPU acceleration

The default device is set to "cpu" but can be overridden by the environment variable:

```rust
let device = std::env::var("VOXWEAVE_COQUI_DEVICE")
    .unwrap_or_else(|_| "cpu".to_string());
```

### CUDA Out of Memory Issues
When using GPU acceleration, memory limitations can cause failures. This is particularly common with the resource-intensive XTTS v2 model.

**Solutions:**
1. Switch to CPU mode:
```bash
export VOXWEAVE_COQUI_DEVICE="cpu"
```

2. Reduce batch size or processing load
3. Close other GPU-intensive applications
4. Use a machine with more VRAM

The system automatically detects available devices, but explicit configuration ensures optimal performance.

**Section sources**
- [COQUI_USAGE_GUIDE.md](file://COQUI_USAGE_GUIDE.md#L187-L215)
- [src/coqui_tts.rs](file://src/coqui_tts.rs#L30-L35)

## Model Download and Verification
The CoquiTTS system uses the XTTS v2 model, which presents specific challenges during setup.

### 1.8GB XTTS v2 Model Download
The default model `tts_models/multilingual/multi-dataset/xtts_v2` is approximately 1.8GB and downloads automatically on first use. This process can be slow depending on internet connection speed.

The model name can be configured via environment variable:
```bash
export VOXWEAVE_COQUI_MODEL="tts_models/multilingual/multi-dataset/xtts_v2"
```

### Verification with test_coqui.py
The `test_coqui.py` script verifies successful installation by:
1. Testing imports of required packages (torch, TTS, numpy)
2. Attempting to initialize the TTS model
3. Reporting success or failure with detailed error messages

```python
def test_tts_model():
    """Test basic TTS model loading"""
    try:
        from TTS.api import TTS
        import torch
        
        device = "cuda" if torch.cuda.is_available() else "cpu"
        print(f"\nUsing device: {device}")
        
        print("\nTrying to initialize TTS model (this may take a while on first run)...")
        model_name = "tts_models/multilingual/multi-dataset/xtts_v2"
        
        tts = TTS(model_name=model_name, progress_bar=True, gpu=(device != "cpu"))
        print(f"✓ Successfully initialized {model_name}")
        
        return True
    except Exception as e:
        print(f"✗ Model initialization failed: {e}")
        return False
```

**Section sources**
- [test_coqui.py](file://test_coqui.py#L45-L85)
- [COQUI_USAGE_GUIDE.md](file://COQUI_USAGE_GUIDE.md#L135-L144)

## Microphone Permission Issues
Microphone access problems specifically affect audio recording for voice cloning on macOS.

### macOS Microphone Permissions
The AudioRecorder component requires microphone access to capture audio for voice cloning. On macOS, this requires explicit permission.

**Resolution Steps:**
1. Go to System Preferences → Security & Privacy → Microphone
2. Ensure the VoxWeave application has microphone access enabled
3. Restart the application if permission was recently granted

The recording function checks for available input devices:
```rust
let device = host.default_input_device()
    .ok_or_else(|| "No input device available".to_string())?;
```

When permissions are denied, this check fails with "No input device available" even when hardware is present.

**Section sources**
- [abogen-ui/crates/ui/components/audio_recorder.rs](file://abogen-ui/crates/ui/components/audio_recorder.rs#L45-L50)
- [COQUI_USAGE_GUIDE.md](file://COQUI_USAGE_GUIDE.md#L403-L407)

## COQUI_BRIDGE_SCRIPT Debugging
Issues with the Python bridge script require careful analysis of execution, payload formatting, and error output.

### Script Execution
The CoquiEngine executes the COQUI_BRIDGE_SCRIPT via Python:
```rust
let mut child = Command::new(&self.python_command)
    .arg("-c")
    .arg(COQUI_BRIDGE_SCRIPT)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
```

### JSON Payload Formatting
The engine sends configuration via JSON payload:
```rust
let payload = json!({
    "text": text,
    "output": output.to_string_lossy(),
    "sample_rate": self.sample_rate,
    "model_name": &self.model_name,
    "device": &self.device,
    "language": lang,
    "speed": speed,
    "speaker_wav": speaker_wav,
});
```

### Error Output Parsing
The system captures and parses error output:
```rust
let output_data = child.wait_with_output()?;
if !output_data.status.success() {
    let stderr = String::from_utf8_lossy(&output_data.stderr);
    return Err(TtsError::CommandFailed(stderr.trim().to_string()));
}
```

Common error patterns include:
- Import failures: "failed to import coqui dependencies"
- File not found: "Speaker reference audio not found"
- Model loading failures: "Model initialization failed"

**Section sources**
- [src/coqui_tts.rs](file://src/coqui_tts.rs#L84-L113)
- [src/tts.rs](file://src/tts.rs#L472-L522)

## CoquiEngine Configuration
The CoquiEngine's behavior is controlled by both default settings and environment variables.

### Default Configuration
The CoquiEngine has the following defaults:
- Python command: `python3`
- Model: `tts_models/multilingual/multi-dataset/xtts_v2`
- Device: `cpu`
- Sample rate: `24000`
- Language: `en`

```rust
impl Default for CoquiEngine {
    fn default() -> Self {
        let python_command =
            std::env::var("VOXWEAVE_COQUI_PYTHON").unwrap_or_else(|_| "python3".to_string());
        let model_name = std::env::var("VOXWEAVE_COQUI_MODEL")
            .unwrap_or_else(|_| "tts_models/multilingual/multi-dataset/xtts_v2".to_string());
        let device = std::env::var("VOXWEAVE_COQUI_DEVICE")
            .unwrap_or_else(|_| "cpu".to_string());
        let sample_rate = std::env::var("VOXWEAVE_COQUI_SAMPLE_RATE")
            .ok()
            .and_then(|value| value.parse::<u32>().ok())
            .filter(|rate| *rate > 0)
            .unwrap_or(24_000);
        let language = std::env::var("VOXWEAVE_COQUI_LANGUAGE")
            .unwrap_or_else(|_| "en".to_string());
        
        Self {
            python_command,
            model_name,
            device,
            sample_rate,
            language,
        }
    }
}
```

### Environment Variable Overrides
All default values can be overridden using environment variables:

| Environment Variable | Purpose | Default Value |
|----------------------|---------|---------------|
| VOXWEAVE_COQUI_PYTHON | Python executable | python3 |
| VOXWEAVE_COQUI_MODEL | TTS model identifier | tts_models/multilingual/multi-dataset/xtts_v2 |
| VOXWEAVE_COQUI_DEVICE | Processing device | cpu |
| VOXWEAVE_COQUI_SAMPLE_RATE | Audio sample rate | 24000 |
| VOXWEAVE_COQUI_LANGUAGE | Default language | en |

**Section sources**
- [src/coqui_tts.rs](file://src/coqui_tts.rs#L15-L40)
- [COQUI_USAGE_GUIDE.md](file://COQUI_USAGE_GUIDE.md#L225-L250)