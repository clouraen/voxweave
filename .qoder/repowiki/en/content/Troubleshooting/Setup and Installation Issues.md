# Setup and Installation Issues

<cite>
**Referenced Files in This Document**   
- [Cargo.toml](file://Cargo.toml)
- [EXECUTION_STATUS.md](file://EXECUTION_STATUS.md)
- [FIXES_APPLIED.md](file://FIXES_APPLIED.md)
- [build.sh](file://build.sh)
- [verify.sh](file://verify.sh)
- [run_demo.sh](file://run_demo.sh)
- [build_output.txt](file://build_output.txt)
- [README.md](file://README.md)
</cite>

## Table of Contents
1. [Introduction](#introduction)
2. [Rust Edition Incompatibility Issues](#rust-edition-incompatibility-issues)
3. [Missing Build Dependencies and Environment Setup](#missing-build-dependencies-and-environment-setup)
4. [Cargo.toml Configuration Problems](#cargo-toml-configuration-problems)
5. [Script Execution Issues](#script-execution-issues)
6. [Build Output Log Analysis](#build-output-log-analysis)
7. [Validation of Successful Installation](#validation-of-successful-installation)
8. [Platform-Specific Setup for macOS and Linux](#platform-specific-setup-for-macos-and-linux)
9. [Conclusion](#conclusion)

## Introduction
This document provides comprehensive troubleshooting guidance for setting up and installing VoxWeave, a Rust-based text-to-speech pipeline. It addresses common issues encountered during installation, including Rust edition incompatibility, missing dependencies, incorrect Cargo.toml configuration, script execution problems, and platform-specific challenges on macOS and Linux. The content is designed to be accessible to beginners while offering technical depth for advanced users, ensuring a smooth setup process across different environments.

## Rust Edition Incompatibility Issues
One of the most critical issues encountered during VoxWeave setup is Rust edition incompatibility. The project initially specified `edition = "2024"` in its Cargo.toml file, which is not supported by the stable Rust compiler as of the current release.

The solution, as documented in both EXECUTION_STATUS.md and FIXES_APPLIED.md, was to change the edition from "2024" to "2021". This change was necessary because Rust edition 2024 is not yet available in the stable release channel. After this modification, the project successfully compiled with zero errors.

Users encountering compilation failures related to edition incompatibility should verify their Cargo.toml file and ensure it specifies `edition = "2021"`. This is particularly important for developers using older or non-updated Rust toolchains, as the project requires Rust 1.80+ with edition 2021 support.

**Section sources**
- [Cargo.toml](file://Cargo.toml#L4)
- [EXECUTION_STATUS.md](file://EXECUTION_STATUS.md#L7-L10)
- [FIXES_APPLIED.md](file://FIXES_APPLIED.md#L10-L16)

## Missing Build Dependencies and Environment Setup
VoxWeave has several dependencies that must be properly installed for successful compilation and execution. The primary dependencies include Rust 1.80+, Cargo build system, and optional components like espeak-ng for real TTS synthesis and Python 3.9+ with kokoro for neural TTS capabilities.

The README.md file outlines these requirements clearly, specifying that espeak-ng must be in the system PATH or its location can be overridden using the VOXWEAVE_ESPEAK_COMMAND environment variable. For Kokoro voices, Python 3.9+ is required along with specific Python packages (kokoro, numpy, torch). Similarly, CoquiTTS support requires additional Python packages (TTS, torch, numpy) and must be enabled with the `--features coqui-tts` flag during build.

Users may encounter issues if these dependencies are missing or improperly configured. The build process will fail if Rust or Cargo are not properly installed, while optional features will be disabled if their respective dependencies are absent.

**Section sources**
- [README.md](file://README.md#L20-L40)
- [EXECUTION_STATUS.md](file://EXECUTION_STATUS.md#L140-L148)

## Cargo.toml Configuration Problems
The Cargo.toml file is central to Rust project configuration and can be a source of various setup issues. In VoxWeave's case, the primary configuration problem was the use of the unsupported Rust edition 2024, which was corrected to edition 2021.

The Cargo.toml file also defines project dependencies, features, and development dependencies. Users attempting to enable optional features like `coqui-tts` or `video-generation` must ensure they have the necessary dependencies installed and use the appropriate feature flags during compilation.

Additionally, the workspace-level Cargo.toml files in the abogen-ui directory use workspace inheritance for version and edition, which helps maintain consistency across multiple crates. Users should be aware that modifying these workspace settings could affect multiple components of the project.

**Section sources**
- [Cargo.toml](file://Cargo.toml#L1-L27)
- [abogen-ui/Cargo.toml](file://abogen-ui/Cargo.toml#L1-L25)
- [abogen-ui/crates/ui/Cargo.toml](file://abogen-ui/crates/ui/Cargo.toml#L1-L35)

## Script Execution Issues
VoxWeave includes several shell scripts to facilitate setup and verification, including build.sh, verify.sh, and run_demo.sh. Users may encounter issues when executing these scripts, primarily related to file permissions and path configuration.

The build.sh script automates the build process by running `cargo build --release` followed by `cargo test`. The verify.sh script performs a comprehensive verification of the installation by testing build compilation, unit tests, CLI interface, voice listing, and text conversion functionality. The run_demo.sh script provides a complete demonstration of the project's capabilities by building the project, listing voices, and performing a mock conversion.

Common issues include permission denied errors when attempting to execute these scripts. This can be resolved by ensuring the scripts have execute permissions using `chmod +x script_name.sh`. Additionally, users must ensure they are running the scripts from the correct directory and that all required dependencies are available in the system PATH.

**Section sources**
- [build.sh](file://build.sh#L1-L21)
- [verify.sh](file://verify.sh#L1-L141)
- [run_demo.sh](file://run_demo.sh#L1-L81)

## Build Output Log Analysis
The build_output.txt file provides valuable information about the build process and can help diagnose compilation issues. In a successful build, this file contains minimal output, typically indicating that the build completed successfully with timing information.

For example, a successful build shows output like "Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s", indicating that the compilation completed quickly with no errors. When troubleshooting build failures, users should examine this output for error messages, warnings, or unusually long compilation times that might indicate dependency resolution problems.

The verify.sh script also generates detailed output that can be used for troubleshooting. It systematically tests each component of the system and reports success or failure for each test. This structured verification output makes it easier to pinpoint exactly which component is failing and why.

**Section sources**
- [build_output.txt](file://build_output.txt#L1-L2)
- [verify.sh](file://verify.sh#L1-L141)

## Validation of Successful Installation
After resolving setup issues, users can validate a successful installation using several methods provided by the VoxWeave project.

The primary validation method is the verify.sh script, which performs five comprehensive tests:
1. Build compilation
2. Unit test execution
3. CLI help command functionality
4. Voice listing capability
5. Text conversion in mock mode

A successful verification results in "ALL TESTS PASSED" output with green checkmarks for each test. Additionally, the run_demo.sh script provides a practical demonstration of the system's capabilities by building the project, listing available voices, and performing a mock conversion of the example.txt file.

Users can also manually verify installation by running basic commands:
- `cargo build` to compile the project
- `cargo test` to run unit tests (17 tests should pass)
- `./target/debug/voxweave list-voices` to verify voice listing
- `./target/debug/voxweave convert samples/example.txt --mock --voice en-us` to test conversion

**Section sources**
- [verify.sh](file://verify.sh#L1-L141)
- [run_demo.sh](file://run_demo.sh#L1-L81)
- [EXECUTION_STATUS.md](file://EXECUTION_STATUS.md#L40-L100)

## Platform-Specific Setup for macOS and Linux
VoxWeave requires platform-specific setup steps for macOS and Linux, primarily related to package management and dependency installation.

For macOS users, the recommended approach is to use Homebrew to install espeak-ng:
```bash
brew install espeak-ng
```

Linux users should use their distribution's package manager, typically apt for Debian/Ubuntu systems:
```bash
sudo apt-get install espeak-ng
```

The README.md file specifically mentions these installation methods, making it easier for users on these platforms to set up the required dependencies. Additionally, the project's cross-platform file sanitization ensures that output files are compatible across different operating systems, handling platform-specific filename restrictions automatically.

Users on both platforms should ensure they have the latest version of Rust and Cargo installed, which can be managed using rustup. The project's reliance on standard Unix-style shell scripts means that the build and verification process is consistent across macOS and Linux environments.

**Section sources**
- [README.md](file://README.md#L38-L40)
- [EXECUTION_STATUS.md](file://EXECUTION_STATUS.md#L145-L148)

## Conclusion
Setting up and installing VoxWeave involves addressing several common issues, with the most critical being the Rust edition incompatibility that requires changing `edition = "2024"` to `edition = "2021"` in Cargo.toml. The project provides comprehensive tools for troubleshooting and validation, including detailed status reports, automated verification scripts, and demonstration scripts.

By following the guidance in this document, users can successfully resolve compilation failures, configure dependencies properly, execute setup scripts, and validate their installation. The combination of the verify.sh and run_demo.sh scripts provides a robust way to confirm that all components are working correctly, while the detailed error reporting helps diagnose any remaining issues.

The project's well-documented requirements and structured troubleshooting approach make it accessible to developers of all skill levels, ensuring a smooth setup experience across different platforms.