# Kokoro TTS Bridge Issues

<cite>
**Referenced Files in This Document**   
- [kokoro_bridge.py](file://python/kokoro_bridge.py)
- [tts.rs](file://src/tts.rs)
- [config.rs](file://src/config.rs)
</cite>

## Table of Contents
1. [Introduction](#introduction)
2. [Python Dependency Issues](#python-dependency-issues)
3. [Environment Variable Configuration](#environment-variable-configuration)
4. [Voice and Language Configuration](#voice-and-language-configuration)
5. [Bridge Script Execution Problems](#bridge-script-execution-problems)
6. [Audio Processing Issues](#audio-processing-issues)
7. [Error Handling and Debugging](#error-handling-and-debugging)

## Introduction
This document addresses troubleshooting issues related to the Kokoro TTS bridge in the VoxWeave application. The Kokoro TTS system uses a Python bridge to generate speech from text, and various configuration, dependency, and execution issues can arise during its operation. This guide covers common problems including Python dependency errors, environment variable configuration, voice ID formatting, and audio processing issues.

## Python Dependency Issues

The Kokoro TTS bridge relies on specific Python packages that must be properly installed and accessible. The `kokoro_bridge.py` script imports the kokoro package and numpy, and failures in these imports are a common source of issues.

When the Python interpreter cannot find the required packages, the bridge script outputs an error message to stderr in the format: "failed to import kokoro dependencies: {exception}". This error propagates through the Rust application and appears in the final `TtsError::CommandFailed` variant.

Common dependency problems include:
- Missing kokoro package installation
- Numpy import errors due to incompatible versions
- Torch dependency issues
- Virtual environment not activated when running the application

The bridge script uses a try-except block to catch import exceptions and exit with status code 1, ensuring that the parent Rust process can handle the failure appropriately.

**Section sources**
- [kokoro_bridge.py](file://python/kokoro_bridge.py#L15-L25)
- [tts.rs](file://src/tts.rs#L245-L252)

## Environment Variable Configuration

The Kokoro TTS engine uses several environment variables to control its behavior and locate necessary resources. These variables are read during the initialization of the `KokoroEngine` struct in the Rust codebase.

The `VOXWEAVE_KOKORO_PYTHON` environment variable specifies the Python interpreter path used to execute the bridge script. If not set, it defaults to "python3". This setting is crucial when multiple Python versions are installed or when using virtual environments, as it ensures the correct Python interpreter with the required packages is used.

Other important configuration variables include:
- `VOXWEAVE_KOKORO_REPO_ID`: Specifies the Hugging Face model repository ID (defaults to "hexgrad/Kokoro-82M")
- `VOXWEAVE_KOKORO_DEVICE`: Controls the execution device ("cpu", "cuda", "mps", etc.)
- `VOXWEAVE_KOKORO_SAMPLE_RATE`: Sets the audio sample rate (defaults to 24000)
- `VOXWEAVE_KOKORO_SPLIT_PATTERN`: Defines the regex pattern for text splitting

Virtual environment issues commonly occur when the environment variables point to a system Python interpreter while the required packages are installed in a virtual environment, or vice versa. Users should ensure consistency between their environment variable settings and package installation locations.

**Section sources**
- [tts.rs](file://src/tts.rs#L194-L212)
- [README.md](file://README.md#L100-L115)

## Voice and Language Configuration

The Kokoro TTS system has specific requirements for voice IDs and language code mapping. Voice IDs follow a standardized format consisting of a language code followed by an underscore and the voice name (e.g., "af_alloy" for American English female).

The available voices are defined in the `KOKORO_VOICES` constant, which contains 56 different voice options across multiple languages and genders. Each voice ID's first character represents the language code according to the `KOKORO_LANGUAGES` mapping:
- "a": American English
- "b": British English
- "e": Spanish
- "f": French
- "h": Hindi
- "i": Italian
- "j": Japanese
- "p": Brazilian Portuguese
- "z": Mandarin Chinese

The voice system constructs `VoiceProfile` objects that include the voice ID, description, engine type, and language code. The language code is extracted from the first character of the voice ID and mapped to its corresponding language name for display purposes.

Incorrect voice ID formats or unsupported language codes will result in configuration errors or fallback behaviors.

**Section sources**
- [tts.rs](file://src/tts.rs#L341-L451)
- [tts.rs](file://src/tts.rs#L296-L339)

## Bridge Script Execution Problems

The communication between the Rust application and the Python bridge script occurs through stdin/stdout pipes. The Rust code serializes a JSON payload containing the text, voice parameters, and output path, which is then passed to the Python script via stdin.

Common execution failures include:
- stdin/stdout communication issues between Rust and Python processes
- Incorrect JSON payload formatting
- Process spawning failures due to incorrect Python path
- Permission issues when writing output files
- Text encoding problems

The Rust code uses `Command::new` to spawn the Python process with the bridge script as an argument. It configures stdin, stdout, and stderr to be piped for inter-process communication. If stdin cannot be opened for the bridge process, the system returns a `TtsError::Backend` with the message "failed to open stdin for kokoro bridge".

The bridge script reads the JSON payload from stdin using `json.load(sys.stdin)` and writes the result to stdout as JSON. Any errors are written to stderr, which are then captured by the Rust parent process.

**Section sources**
- [tts.rs](file://src/tts.rs#L234-L269)
- [kokoro_bridge.py](file://python/kokoro_bridge.py#L10-L12)

## Audio Processing Issues

The Kokoro TTS pipeline involves several audio processing steps that can encounter issues, particularly related to audio concatenation and sample rate conversion.

The bridge script processes text input and generates audio segments, which are then concatenated into a single audio array using `numpy.concatenate`. If no audio segments are produced, it creates an empty array. The audio is then clipped to the range [-1.0, 1.0] and converted to 16-bit PCM format for WAV output.

Sample rate conversion issues can occur when:
- The requested sample rate doesn't match the output format requirements
- The fallback sample rate (24000) is incompatible with downstream processing
- Sample rate conversion introduces audio artifacts

The script handles empty text input by writing silence to the output file using the `_write_silence` function, which creates an empty WAV file with the specified sample rate.

Audio concatenation problems may arise from:
- Incompatible audio segment formats
- Memory limitations when processing long texts
- NumPy array dimension mismatches
- Floating-point precision issues during audio processing

**Section sources**
- [kokoro_bridge.py](file://python/kokoro_bridge.py#L30-L89)
- [kokoro_bridge.py](file://python/kokoro_bridge.py#L5-L8)

## Error Handling and Debugging

The Kokoro TTS system implements comprehensive error handling across both the Rust and Python components. Errors are propagated from the Python bridge script back to the Rust application through stderr, where they are captured and returned as `TtsError::CommandFailed` variants.

The error flow follows this pattern:
1. Python script encounters an error and writes to stderr
2. Rust code captures stderr output from the completed process
3. The stderr content is trimmed and returned as the error message
4. The error propagates through the application to the user interface

Common error patterns in the bridge script's output include:
- "failed to import kokoro dependencies: {exception}" - Python package import issues
- Various exception messages during TTS generation
- Empty stderr output when the process fails without explicit error reporting

Debugging steps for troubleshooting include:
1. Verifying Python package installation with `pip list`
2. Testing the bridge script independently with sample JSON input
3. Checking environment variable settings
4. Validating voice ID formats against the `KOKORO_VOICES` list
5. Ensuring the output directory is writable
6. Confirming the Python interpreter can access all required packages

The system also includes fallback mechanisms, such as using the default sample rate when none is specified and writing silence for empty input text.

**Section sources**
- [tts.rs](file://src/tts.rs#L267-L298)
- [kokoro_bridge.py](file://python/kokoro_bridge.py#L20-L25)