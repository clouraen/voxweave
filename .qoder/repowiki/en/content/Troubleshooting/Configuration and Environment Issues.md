# Configuration and Environment Issues

<cite>
**Referenced Files in This Document**   
- [config.rs](file://src/config.rs)
- [coqui_tts.rs](file://src/coqui_tts.rs)
- [build_output.txt](file://build_output.txt)
- [EXECUTION_STATUS.md](file://EXECUTION_STATUS.md)
- [COQUI_USAGE_GUIDE.md](file://COQUI_USAGE_GUIDE.md)
</cite>

## Table of Contents
1. [Introduction](#introduction)
2. [Configuration File Management](#configuration-file-management)
3. [Environment Variable Configuration](#environment-variable-configuration)
4. [Feature Flag Activation](#feature-flag-activation)
5. [Cross-Platform Path Handling](#cross-platform-path-handling)
6. [Configuration Persistence](#configuration-persistence)
7. [Error Log Interpretation](#error-log-interpretation)
8. [Factory Reset Procedure](#factory-reset-procedure)
9. [Conclusion](#conclusion)

## Introduction
This document provides comprehensive troubleshooting guidance for configuration and environment issues in the VoxWeave application. It addresses common problems related to configuration files, environment variables, feature flags, and cross-platform compatibility. The document focuses on the implementation details found in config.rs and related files, providing solutions for configuration-related issues that may arise during development and production use.

## Configuration File Management

VoxWeave's configuration system is implemented in config.rs, which defines the AppConfig structure and associated functions for loading, saving, and managing application settings. The system handles missing or malformed configuration files by returning default values when the configuration file does not exist.

The configuration file is stored as config.json in a platform-specific directory determined by the directories crate. On most systems, this will be in the standard configuration directory (e.g., ~/.config/voxweave/config.json on Linux). Users can override this location by setting the VOXWEAVE_CONFIG_DIR environment variable.

When troubleshooting configuration file issues, verify that:
- The configuration directory exists and is writable
- The config.json file has proper JSON formatting
- File permissions allow read/write access by the application
- The file is not locked by another process

**Section sources**
- [config.rs](file://src/config.rs#L0-L117)

## Environment Variable Configuration

VoxWeave uses several environment variables to configure its behavior, particularly for the Coqui-TTS integration. The most critical variables include:

- **VOXWEAVE_CONFIG_DIR**: Overrides the default configuration directory path
- **VOXWEAVE_COQUI_PYTHON**: Specifies the Python command to use (default: python3)
- **VOXWEAVE_COQUI_DEVICE**: Sets the processing device (cpu, cuda, mps)
- **VOXWEAVE_COQUI_MODEL**: Specifies the TTS model to use
- **VOXWEAVE_COQUI_SAMPLE_RATE**: Sets the audio sample rate
- **VOXWEAVE_COQUI_LANGUAGE**: Default language for speech synthesis

When troubleshooting environment variable issues:
1. Verify variables are set before launching the application
2. Check for typos in variable names
3. Ensure values are appropriate for the target system
4. Validate that Python dependencies are installed when using Coqui-TTS

For Apple Silicon Macs, set VOXWEAVE_COQUI_DEVICE="mps" to enable GPU acceleration. For NVIDIA GPUs, use "cuda" instead.

**Section sources**
- [coqui_tts.rs](file://src/coqui_tts.rs#L0-L115)
- [COQUI_USAGE_GUIDE.md](file://COQUI_USAGE_GUIDE.md#L465-L487)

## Feature Flag Activation

Feature flags in VoxWeave are managed through Cargo.toml feature definitions and conditional compilation. The coqui-tts feature must be enabled to access Coqui-TTS functionality. This feature enables the CoquiEngine implementation and related components.

To activate the coqui-tts feature:
```bash
cargo build --features coqui-tts
```

For the UI components:
```bash
cd abogen-ui
cargo build --features coqui-tts
```

When the coqui-tts feature is not enabled, related functionality will be unavailable. Ensure that:
- The feature is specified in build commands
- Dependencies (cpal, hound) are available
- Python environment is properly configured
- Required Python packages (TTS, torch, numpy) are installed

Feature activation impacts available functionality, particularly voice cloning and neural TTS capabilities.

**Section sources**
- [coqui_tts.rs](file://src/coqui_tts.rs#L10-L15)
- [COQUI_TTS_IMPLEMENTATION.md](file://COQUI_TTS_IMPLEMENTATION.md#L25-L35)

## Cross-Platform Path Handling

VoxWeave implements cross-platform path handling through the config_path function in config.rs. This function uses the directories crate to determine appropriate configuration directories for different operating systems.

The system handles platform-specific path issues by:
- Using PathBuf for portable path manipulation
- Creating configuration directories automatically
- Handling different path separators (forward slash on Unix-like systems, backslash on Windows)
- Managing platform-specific reserved filenames

When troubleshooting path-related issues:
- Verify the application has permission to access the configuration directory
- Check for invalid characters in file paths
- Ensure sufficient disk space is available
- Validate that network drives are accessible if used

The configuration system automatically creates parent directories when needed, reducing path-related errors.

**Section sources**
- [config.rs](file://src/config.rs#L20-L35)

## Configuration Persistence

Configuration persistence in VoxWeave is implemented through the save_config and load_config functions in config.rs. These functions handle reading and writing the AppConfig structure to JSON format.

The persistence mechanism ensures settings are maintained across application restarts by:
- Automatically saving to the platform-appropriate configuration directory
- Creating backup directories when needed
- Using pretty-printed JSON for readability
- Validating file existence before loading

If configuration changes are not persisting:
1. Check file permissions on the configuration directory
2. Verify disk space availability
3. Ensure the application has proper write privileges
4. Confirm the VOXWEAVE_CONFIG_DIR environment variable points to a writable location

The system includes safeguards to prevent data loss during write operations.

**Section sources**
- [config.rs](file://src/config.rs#L40-L55)

## Error Log Interpretation

Configuration-related errors can be identified in build_output.txt and EXECUTION_STATUS.md. These files provide insights into configuration issues during build and execution phases.

In build_output.txt, look for:
- Compilation errors related to missing features
- Dependency resolution issues
- Feature flag activation problems

EXECUTION_STATUS.md contains detailed execution information, including:
- Test results for configuration functionality
- Build success/failure status
- Environment setup verification
- Feature availability confirmation

When diagnosing configuration issues:
1. Check EXECUTION_STATUS.md for test results (e.g., config_round_trip_persists_data)
2. Review build_output.txt for compilation warnings
3. Verify that all configuration tests pass
4. Confirm the presence of configuration files in expected locations

These logs help identify whether issues stem from configuration problems or other sources.

**Section sources**
- [build_output.txt](file://build_output.txt#L0-L1)
- [EXECUTION_STATUS.md](file://EXECUTION_STATUS.md#L0-L344)

## Factory Reset Procedure

To reset VoxWeave to factory defaults:
1. Locate the configuration directory using the same logic as config_path()
2. Delete the config.json file
3. Restart the application

Alternatively, temporarily set VOXWEAVE_CONFIG_DIR to a new directory:
```bash
export VOXWEAVE_CONFIG_DIR=/tmp/voxweave-clean
```

The application will create a new configuration file with default settings. The default configuration includes:
- replace_single_newlines: false
- default_voice: None
- recent_files: empty list

After reset, customize settings as needed and they will be saved to the new configuration file.

**Section sources**
- [config.rs](file://src/config.rs#L10-L15)

## Conclusion
VoxWeave's configuration system provides robust settings management across different environments and platforms. By understanding the configuration file structure, environment variable requirements, and feature flag system, users can effectively troubleshoot common issues. The implementation in config.rs ensures reliable configuration persistence, while the documentation in COQUI_USAGE_GUIDE.md provides clear guidance for environment setup. When encountering configuration problems, consult the execution status and build output logs for diagnostic information, and use the factory reset procedure when necessary to restore default settings.