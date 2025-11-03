# TTS Engine Bridge Scripts

<cite>
**Referenced Files in This Document**   
- [kokoro_bridge.py](file://python/kokoro_bridge.py)
- [tts.rs](file://src/tts.rs)
</cite>

## Table of Contents
1. [Kokoro Bridge Script](#kokoro-bridge-script)
2. [Coqui Bridge Script](#coqui-bridge-script)
3. [Error Handling Strategies](#error-handling-strategies)
4. [Bridge Communication and Debugging](#bridge-communication-and-debugging)
5. [Example JSON Payloads](#example-json-payloads)

## Kokoro Bridge Script

The `kokoro_bridge.py` script serves as a Python bridge for the Kokoro TTS engine, enabling integration between the Rust application and the neural TTS model. It receives JSON input via stdin and outputs duration metadata via stdout.

The JSON input schema includes the following parameters:
- **text**: Input text for synthesis (string)
- **output**: Output file path for generated audio (string)
- **sample_rate**: Audio sampling rate in Hz (integer, optional, defaults to 24000)
- **repo_id**: Model repository identifier (string, optional)
- **device**: Computation device ("cpu" or "cuda", optional, defaults to "cpu")
- **lang_code**: Language code for synthesis (string, optional)
- **voice**: Voice identifier for the speaker (string, optional)
- **speed**: Speech speed multiplier (float, optional, defaults to 1.0)
- **split_pattern**: Regular expression pattern for text segmentation (string, optional)

The output format is a JSON object containing the **duration** field, which represents the audio duration in seconds as a floating-point number.

For empty text inputs, the `_write_silence` function creates an empty WAV file with the specified sample rate, ensuring consistent output format. The `_coerce_numpy` utility function handles tensor conversion from various formats (including PyTorch tensors) to NumPy arrays, ensuring compatibility with audio processing operations.

The script initializes a `KPipeline` with the specified language and device parameters, then processes the input text through streaming synthesis. During synthesis, audio segments are collected and concatenated into a single output. The final audio is normalized, converted to 16-bit PCM format, and written to the specified output path.

**Section sources**
- [kokoro_bridge.py](file://python/kokoro_bridge.py#L0-L89)
- [tts.rs](file://src/tts.rs#L234-L269)

## Coqui Bridge Script

The Coqui bridge script, embedded within `src/tts.rs` as `COQUI_BRIDGE_SCRIPT`, provides similar functionality for the Coqui TTS engine with additional voice cloning capabilities. It shares the same JSON interface pattern but with different parameters reflecting Coqui's API.

Key differences in the JSON schema include:
- **model_name**: TTS model identifier (string, optional, defaults to "tts_models/multilingual/multi-dataset/xtts_v2")
- **language**: Language code for synthesis (string, optional, defaults to "en")
- **speaker_wav**: File path to reference audio for voice cloning (string, optional)

The script initializes the TTS API with GPU detection, automatically using CUDA when available or falling back to CPU. It supports two distinct `tts_to_file` invocation patterns: one for standard synthesis without voice cloning, and another that includes the `speaker_wav` parameter for voice cloning when a reference audio file is provided.

The bridge handles both scenarios by checking the existence of the speaker reference audio file. When present and valid, it invokes the TTS engine with voice cloning parameters; otherwise, it uses standard synthesis. After successful synthesis, the script calculates the audio duration by reading the WAV file properties, falling back to a word-count estimation if file reading fails.

**Section sources**
- [tts.rs](file://src/tts.rs#L377-L451)
- [tts.rs](file://src/coqui_tts.rs#L84-L113)

## Error Handling Strategies

Both bridge scripts implement comprehensive error handling to ensure robust operation and meaningful error reporting.

The Kokoro bridge handles dependency import failures by catching exceptions during the import of `kokoro` and `numpy` packages. When imports fail, it writes an error message to stderr and exits with a non-zero status code, allowing the parent process to handle the failure appropriately.

During synthesis, the script may encounter various exceptions. These are caught and reported via stderr, with the process exiting with an error status. The Rust wrapper (`KokoroEngine`) captures the stderr output and converts it into a `TtsError::CommandFailed` variant, preserving the original error message.

Similarly, the Coqui bridge implements error handling at multiple levels. It first handles import failures for `TTS.api` and `torch`, reporting them via stderr. During synthesis, it catches exceptions that may occur during TTS processing, including file not found errors for speaker reference audio. These are reported via stderr with descriptive messages.

The Rust `CoquiEngine` implementation captures the process exit status and stderr output, converting failures into appropriate error variants. It also attempts to parse any JSON output from stdout, issuing warnings if metadata decoding fails, but does not treat this as a critical error.

**Section sources**
- [kokoro_bridge.py](file://python/kokoro_bridge.py#L35-L40)
- [tts.rs](file://src/tts.rs#L267-L298)
- [tts.rs](file://src/coqui_tts.rs#L84-L113)

## Bridge Communication and Debugging

Communication between the Rust application and Python bridge scripts occurs through stdin/stdout with JSON payloads, while errors are communicated via stderr. This separation allows clean data flow and error reporting.

The `KokoroEngine` and `CoquiEngine` in Rust construct JSON payloads using `serde_json::json!` macro, populate them with configuration from environment variables and voice profiles, then execute the Python interpreter with the bridge script code passed via the `-c` argument.

When debugging bridge communication issues, examining stderr output is crucial. Both bridges write detailed error messages to stderr when exceptions occur. For the Kokoro bridge, common issues include missing Python dependencies or model loading failures. For the Coqui bridge, frequent issues involve missing speaker reference files or CUDA memory errors.

The bridges are designed to be stateless and idempotent, making them suitable for repeated execution within processing pipelines. They follow a simple execution pattern: read JSON from stdin, process the request, write audio to the specified file, and output metadata to stdout.

**Section sources**
- [tts.rs](file://src/tts.rs#L234-L269)
- [tts.rs](file://src/coqui_tts.rs#L84-L113)

## Example JSON Payloads

Valid JSON payloads for both bridges demonstrate the required structure and optional parameters.

For the Kokoro bridge:
```json
{
  "text": "Hello, this is a test of the Kokoro TTS system.",
  "output": "/path/to/output.wav",
  "sample_rate": 24000,
  "repo_id": "hexgrad/Kokoro-82M",
  "device": "cpu",
  "lang_code": "a",
  "voice": "af_sarah",
  "speed": 1.0,
  "split_pattern": "\\n+"
}
```

For the Coqui bridge:
```json
{
  "text": "Hello, this uses voice cloning with Coqui TTS.",
  "output": "/path/to/output.wav",
  "sample_rate": 24000,
  "model_name": "tts_models/multilingual/multi-dataset/xtts_v2",
  "device": "cuda",
  "language": "en",
  "speed": 1.0,
  "speaker_wav": "/path/to/reference.wav"
}
```

Empty text input for either bridge:
```json
{
  "text": "",
  "output": "/path/to/silence.wav",
  "sample_rate": 24000
}
```

**Section sources**
- [kokoro_bridge.py](file://python/kokoro_bridge.py#L0-L89)
- [tts.rs](file://src/tts.rs#L377-L451)