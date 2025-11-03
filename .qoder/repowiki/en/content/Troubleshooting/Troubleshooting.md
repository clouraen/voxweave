# Troubleshooting

<cite>
**Referenced Files in This Document**   
- [EXECUTION_STATUS.md](file://EXECUTION_STATUS.md)
- [FIXES_APPLIED.md](file://FIXES_APPLIED.md)
- [verify.sh](file://verify.sh)
- [build_output.txt](file://build_output.txt)
- [test_output.txt](file://test_output.txt)
- [main.rs](file://src/main.rs)
- [kokoro_bridge.py](file://python/kokoro_bridge.py)
- [test_coqui.py](file://test_coqui.py)
- [build.sh](file://build.sh)
- [run_demo.sh](file://run_demo.sh)
- [generate_video.sh](file://generate_video.sh)
- [Cargo.toml](file://Cargo.toml)
</cite>

## Table of Contents
1. [Introduction](#introduction)
2. [Common Setup Issues](#common-setup-issues)
3. [TTS Component Troubleshooting](#tts-component-troubleshooting)
4. [Video Generation Troubleshooting](#video-generation-troubleshooting)
5. [UI Component Issues](#ui-component-issues)
6. [Debugging Procedures](#debugging-procedures)
7. [Performance Troubleshooting](#performance-troubleshooting)
8. [Platform-Specific Issues](#platform-specific-issues)
9. [Escalation Paths and Support](#escalation-paths-and-support)

## Introduction
This document provides comprehensive troubleshooting guidance for the VoxWeave system, focusing on diagnosing and resolving common issues across its components. The information is derived from execution status reports, applied fixes, verification scripts, and system logs. The troubleshooting content is organized by component (TTS, video generation, UI), with specific solutions for common error patterns, setup issues, and performance problems.

## Common Setup Issues

### Missing Dependencies
The VoxWeave system requires specific dependencies for full functionality. Missing dependencies can lead to partial functionality or complete failure.

**Rust and Cargo**: The core application is built with Rust 1.80+ using the 2021 edition. An incompatible edition (e.g., "2024") in `Cargo.toml` will prevent compilation. This was resolved by changing the edition to "2021".

**Optional TTS Dependencies**:
- **espeak-ng**: Required for real TTS synthesis. Install via package managers:
  ```bash
  # macOS
  brew install espeak-ng
  # Linux
  sudo apt-get install espeak-ng
  ```
- **Python 3.9+ with kokoro**: Required for neural TTS functionality.

**Section sources**
- [EXECUTION_STATUS.md](file://EXECUTION_STATUS.md#L1-L345)
- [FIXES_APPLIED.md](file://FIXES_APPLIED.md#L1-L547)
- [Cargo.toml](file://Cargo.toml)

### Incorrect API Keys
The video generation component requires the `ZAI_API_KEY` environment variable to be set. If this key is not set, video generation will fail while audio and subtitle generation may still succeed.

**Solution**: Set the environment variable before running video generation commands:
```bash
export ZAI_API_KEY=your_api_key_here
```

**Section sources**
- [main.rs](file://src/main.rs#L200-L220)
- [generate_video.sh](file://generate_video.sh#L1-L42)

### Python Environment Problems
The Coqui TTS integration requires specific Python packages (`TTS`, `torch`, `numpy`). Missing packages will cause import failures in the Python bridge.

**Diagnosis**: Use the `test_coqui.py` script to verify the Python environment:
```bash
python3 test_coqui.py
```

**Solution**: Install the required packages:
```bash
pip install TTS torch numpy
```

**Section sources**
- [test_coqui.py](file://test_coqui.py#L1-L141)
- [kokoro_bridge.py](file://python/kokoro_bridge.py#L1-L90)

## TTS Component Troubleshooting

### 'ZAI_API_KEY not set' Error
This error occurs specifically during video generation attempts, not during TTS operations. The TTS components (espeak, kokoro, Coqui) do not require this API key.

**Explanation**: The `ZAI_API_KEY` is used exclusively by the video generation service, not the TTS engines. This error indicates that video generation was attempted without the required API key.

**Solution**: Set the `ZAI_API_KEY` environment variable as shown in the Common Setup Issues section.

**Section sources**
- [main.rs](file://src/main.rs#L200-L220)
- [generate_video.sh](file://generate_video.sh#L1-L42)

### Python Import Failures
When using the kokoro or Coqui TTS engines, Python import failures can occur if dependencies are not properly installed.

**Symptoms**: Error messages indicating missing modules like `kokoro` or `TTS`.

**Diagnosis**: Check the Python environment using `test_coqui.py` and examine the `kokoro_bridge.py` script for import statements.

**Solutions**:
1. Verify Python 3.9+ is installed and active
2. Install required packages in a virtual environment
3. Ensure the Python path is correctly configured for the Rust application

**Section sources**
- [kokoro_bridge.py](file://python/kokoro_bridge.py#L1-L90)
- [test_coqui.py](file://test_coqui.py#L1-L141)

### Audio Generation Timeouts
Audio generation timeouts can occur with neural TTS models due to model loading times or processing delays.

**Mitigation**:
- Ensure sufficient memory is available
- Use GPU acceleration when possible (configure in `kokoro_bridge.py`)
- Monitor system resources during generation
- Consider using the mock engine for testing: `--mock` flag

**Section sources**
- [main.rs](file://src/main.rs#L100-L150)
- [kokoro_bridge.py](file://python/kokoro_bridge.py#L1-L90)

## Video Generation Troubleshooting

### Video Generation Failure
Video generation can fail due to several reasons, with the most common being the missing `ZAI_API_KEY`.

**Error Message**: "Video generation failed: Environment variable ZAI_API_KEY not found"

**Solutions**:
1. Set the `ZAI_API_KEY` environment variable
2. Verify the API key is valid and has appropriate permissions
3. Check network connectivity to the ZAI API service

**Diagnostic Steps**:
1. Run `generate_video.sh` with verbose output
2. Check if audio and subtitles are generated (indicating TTS success)
3. Verify API key format and validity

**Section sources**
- [main.rs](file://src/main.rs#L200-L220)
- [generate_video.sh](file://generate_video.sh#L1-L42)

### Style and Resolution Issues
The video generation supports specific styles and resolutions. Invalid values will default to cyberpunk style and 1080p resolution.

**Valid Styles**: realistic, anime, 3d, cinematic, biotech, cyberpunk, educational
**Valid Resolutions**: 720p, 1080p, 4k
**Valid Formats**: mp4, mov, webm

**Solution**: Use only the supported values when specifying video parameters.

**Section sources**
- [main.rs](file://src/main.rs#L230-L280)

## UI Component Issues

### abogen-ui Build Failures
The UI components (desktop, mobile, web) are built with Rust using Cargo. Build failures may occur due to missing dependencies or incompatible Rust versions.

**Solutions**:
1. Ensure Rust 1.80+ is installed
2. Verify Cargo is properly configured
3. Check that all UI-specific dependencies are installed

**Note**: The UI components are separate from the core TTS functionality and may have their own dependency requirements.

**Section sources**
- [abogen-ui/Cargo.toml](file://abogen-ui/Cargo.toml)
- [abogen-ui/apps/desktop/Cargo.toml](file://abogen-ui/apps/desktop/Cargo.toml)

### Service Integration Issues
The UI services (tts_service, video_generation, zai_video) may fail to integrate with the core VoxWeave system.

**Diagnosis**:
1. Verify the core VoxWeave binary is built and accessible
2. Check that environment variables are available to the UI process
3. Ensure API endpoints are correctly configured

**Section sources**
- [abogen-ui/crates/ui/services/tts_service.rs](file://abogen-ui/crates/ui/services/tts_service.rs)
- [abogen-ui/crates/ui/services/video_generation.rs](file://abogen-ui/crates/ui/services/video_generation.rs)

## Debugging Procedures

### Using the verify.sh Script
The `verify.sh` script provides automated verification of the VoxWeave system's core functionality.

**Execution**:
```bash
./verify.sh
```

**Tests Performed**:
1. Build compilation
2. Unit tests execution
3. CLI help command availability
4. Voice listing functionality
5. Text conversion with mock engine

**Expected Output**: All tests should pass with "✅" indicators.

**Section sources**
- [verify.sh](file://verify.sh#L1-L141)

### Interpreting build_output.txt and test_output.txt
These log files provide detailed information about build and test execution.

**build_output.txt**: Indicates successful compilation with message "Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s"

**test_output.txt**: Shows detailed test results, including:
- Compilation status
- Number of tests passed (17/17)
- Individual test results
- Final test outcome: "test result: ok. 17 passed; 0 failed"

**Diagnostic Value**: These files confirm that the code compiles correctly and all unit tests pass, indicating a healthy codebase.

**Section sources**
- [build_output.txt](file://build_output.txt#L1-L2)
- [test_output.txt](file://test_output.txt#L1-L38)

### Enabling Verbose Logging
While not explicitly implemented in the current code, diagnostic information can be obtained through:
- The `--mock` flag for testing without actual audio generation
- The verification scripts that provide detailed output
- Direct examination of the generated audio and subtitle files

**Section sources**
- [main.rs](file://src/main.rs#L100-L150)
- [verify.sh](file://verify.sh#L1-L141)

### Checking Network Connectivity
For video generation, network connectivity to the ZAI API service is essential.

**Diagnostic Steps**:
1. Verify `ZAI_API_KEY` is set
2. Check internet connectivity
3. Test API endpoint accessibility (if known)
4. Monitor for timeout errors in the output

**Section sources**
- [main.rs](file://src/main.rs#L200-L220)

## Performance Troubleshooting

### Slow Processing Times
Slow processing can occur during neural TTS synthesis or video generation.

**Optimization Tips**:
- Use GPU acceleration for Coqui TTS by ensuring CUDA is available
- Close other memory-intensive applications
- Use simpler voice models when possible
- Process smaller text files initially
- Consider using the mock engine for rapid testing

**Section sources**
- [kokoro_bridge.py](file://python/kokoro_bridge.py#L1-L90)
- [test_coqui.py](file://test_coqui.py#L1-L141)

### High Memory Usage
Neural TTS models can consume significant memory.

**Mitigation**:
- Monitor system memory usage during processing
- Ensure sufficient RAM is available (16GB+ recommended for neural TTS)
- Close unnecessary applications
- Process files sequentially rather than in parallel
- Consider using less resource-intensive TTS engines (espeak) for simple tasks

**Section sources**
- [kokoro_bridge.py](file://python/kokoro_bridge.py#L1-L90)

## Platform-Specific Issues

### macOS Issues
**Homebrew Installation**: Use Homebrew to install espeak-ng:
```bash
brew install espeak-ng
```

**Python Environment**: Ensure Python 3.9+ is installed via Homebrew or pyenv.

**Section sources**
- [EXECUTION_STATUS.md](file://EXECUTION_STATUS.md#L1-L345)
- [run_demo.sh](file://run_demo.sh#L1-L81)

### Linux Issues
**Package Installation**: Use the system package manager to install espeak-ng:
```bash
sudo apt-get install espeak-ng
```

**Permissions**: Ensure proper file permissions for reading input files and writing output files.

**Section sources**
- [EXECUTION_STATUS.md](file://EXECUTION_STATUS.md#L1-L345)

### Windows Issues
**Path Handling**: The system includes path sanitization for Windows, replacing invalid characters and handling reserved names.

**Rust Installation**: Ensure Rust is properly installed via rustup.

**Section sources**
- [src/sanitize.rs](file://src/sanitize.rs)

## Escalation Paths and Support

### When to Escalate
Escalate issues when:
- Basic verification tests fail (`verify.sh`)
- Core functionality is broken despite correct setup
- Documentation solutions do not resolve the issue
- Encountering undocumented error messages

### Collecting Diagnostic Information
For support requests, provide:
1. Output from `verify.sh`
2. Contents of `build_output.txt` and `test_output.txt`
3. Exact error messages and command lines used
4. System information (OS, Rust version, Python version)
5. Steps to reproduce the issue

### Available Resources
- `EXECUTION_STATUS.md`: Current system status and verified functionality
- `FIXES_APPLIED.md`: Record of previous issues and their solutions
- `run_demo.sh`: Working example of system functionality
- `test_coqui.py`: Python environment verification script

**Section sources**
- [EXECUTION_STATUS.md](file://EXECUTION_STATUS.md#L1-L345)
- [FIXES_APPLIED.md](file://FIXES_APPLIED.md#L1-L547)
- [verify.sh](file://verify.sh#L1-L141)
- [run_demo.sh](file://run_demo.sh#L1-L81)
- [test_coqui.py](file://test_coqui.py#L1-L141)