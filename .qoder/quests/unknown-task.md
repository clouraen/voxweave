# Kokoro Python Module Import Issue Resolution

## Problem Statement

The application is failing when attempting to use Kokoro TTS with the error:
```
command execution failed: failed to import kokoro dependencies: No module named 'kokoro'
```

This indicates that the Python environment being invoked by the Rust application does not have the `kokoro` package installed, or there is a path/configuration issue preventing the module from being found.

## Root Cause Analysis

The issue stems from one or more of the following:

1. **Missing Python Package**: The `kokoro` package is not installed in the Python environment that the Rust code is invoking
2. **Wrong Python Interpreter**: The Rust code may be invoking a different Python interpreter than expected (e.g., system Python instead of virtual environment Python)
3. **Version Constraint**: For macOS ARM64 environments, the kokoro package has a version constraint (`kokoro<0.7.6`) that must be respected
4. **PATH Configuration**: The Python interpreter specified via `VOXWEAVE_KOKORO_PYTHON` may not be correctly configured
5. **Missing Dependencies**: Additional dependencies (`numpy`, `torch`) required by kokoro may not be installed

## Current System Behavior

### Python Bridge Execution Flow

The Rust application invokes Kokoro through the following mechanism:

1. The `KokoroEngine` struct in `src/tts.rs` reads the Python command from environment variable `VOXWEAVE_KOKORO_PYTHON` (defaults to `python3`)
2. It spawns a subprocess executing: `python3 -c <KOKORO_BRIDGE_SCRIPT>`
3. The bridge script (`python/kokoro_bridge.py`) attempts to import kokoro at runtime
4. If import fails, the script writes to stderr and exits with status code 1
5. The Rust code captures this error and propagates it as `TtsError::CommandFailed`

### Configuration Points

| Environment Variable | Purpose | Default Value |
|---------------------|---------|---------------|
| `VOXWEAVE_KOKORO_PYTHON` | Python interpreter path | `python3` |
| `VOXWEAVE_KOKORO_REPO_ID` | Hugging Face model repo | `hexgrad/Kokoro-82M` |
| `VOXWEAVE_KOKORO_DEVICE` | Compute device | `cpu` |
| `VOXWEAVE_KOKORO_SAMPLE_RATE` | Audio sample rate | `24000` |
| `VOXWEAVE_KOKORO_SPLIT_PATTERN` | Sentence splitting regex | `\n+` |

## Solution Design

### Diagnostic and Resolution Strategy

The solution follows a systematic diagnostic-then-resolution approach with multiple validation stages.

#### Stage 1: Environment Diagnosis

**Objective**: Identify which Python interpreter is being used and verify its package installation state.

**Diagnostic Actions**:
- Determine the Python interpreter path being invoked (from `VOXWEAVE_KOKORO_PYTHON` or default)
- Verify the interpreter exists and is executable
- Check Python version compatibility (must be 3.9+)
- List installed packages in that environment
- Identify if running in a virtual environment
- Detect the platform (macOS ARM64 requires special handling)

**Success Criteria**:
- Python interpreter is found and version >= 3.9
- Clear understanding of which environment is being used (system/venv/conda)
- Package list shows whether kokoro and dependencies are present

#### Stage 2: Dependency Installation

**Objective**: Install all required Python packages with platform-specific constraints.

**Installation Strategy**:

The installation approach varies based on platform and existing environment:

| Platform | Package Specification | Rationale |
|----------|----------------------|-----------|
| macOS ARM64 | `kokoro<0.7.6` | Newer versions lack misaki distribution for ARM64 |
| Other platforms | `kokoro` (latest) | No version constraints |
| All platforms | `numpy torch` | Required dependencies for kokoro |

**Installation Method**:

Install packages using the same Python interpreter that the Rust application invokes:
- If `VOXWEAVE_KOKORO_PYTHON` is set, use that exact path
- Otherwise, use `python3` (the default)
- Consider using virtual environment to isolate dependencies

**Package Installation Order**:
1. Core dependencies: `numpy`, `torch`
2. Kokoro package with platform-specific version constraint
3. Verification: attempt to import kokoro after installation

#### Stage 3: Configuration Validation

**Objective**: Ensure the environment variable configuration correctly points to the Python interpreter with installed packages.

**Validation Actions**:
- If using a virtual environment, ensure `VOXWEAVE_KOKORO_PYTHON` points to the venv's Python binary
- Test the import by manually running the bridge script with the configured interpreter
- Verify that the subprocess invocation from Rust matches the validated configuration

**Configuration Examples**:

For system Python:
```
VOXWEAVE_KOKORO_PYTHON=python3
```

For virtual environment (macOS/Linux):
```
VOXWEAVE_KOKORO_PYTHON=/path/to/venv/bin/python
```

For virtual environment (Windows):
```
VOXWEAVE_KOKORO_PYTHON=C:\path\to\venv\Scripts\python.exe
```

#### Stage 4: Integration Testing

**Objective**: Verify end-to-end functionality of Kokoro TTS from the Rust application.

**Test Scenarios**:
1. List Kokoro voices to verify engine initialization
2. Perform a simple TTS conversion using a Kokoro voice
3. Verify audio file generation and metadata output
4. Confirm no import errors in stderr

**Test Command Examples**:
```
cargo run -- list-voices
cargo run -- convert samples/example.txt --voice af_alloy --output output_dir
```

**Expected Outcomes**:
- Kokoro voices appear in the voice list
- Audio file is generated without errors
- No Python import errors in console output

### Enhanced Error Reporting

To improve debuggability for future occurrences, the bridge script error handling should provide more context.

**Current Error Output**:
```
failed to import kokoro dependencies: No module named 'kokoro'
```

**Enhanced Error Output** (proposed improvements):
- Python interpreter path being used
- Python version
- Installed packages in the environment
- Suggestions for resolution based on platform

**Error Message Format**:
```
Failed to import kokoro dependencies: No module named 'kokoro'
Python: /usr/bin/python3 (version 3.11.5)
Suggestion: Install kokoro with: pip install kokoro numpy torch
For macOS ARM64: pip install 'kokoro<0.7.6' numpy torch
```

### Fallback and Degradation Strategy

**Graceful Degradation**:
When Kokoro is unavailable, the application should:
- Continue to function with espeak-ng voices
- Clearly communicate to the user that Kokoro voices are unavailable
- Provide actionable installation instructions
- Not fail catastrophically when listing voices

**Voice List Behavior**:
- If Kokoro import fails during voice listing, only espeak voices should be shown
- A warning message should indicate Kokoro is unavailable with installation instructions

## Implementation Checklist

### User Actions Required

1. **Identify Python Interpreter**
   - Check if `VOXWEAVE_KOKORO_PYTHON` is set: `echo $VOXWEAVE_KOKORO_PYTHON`
   - If not set, verify default `python3` location: `which python3`
   - Check Python version: `python3 --version` (must be >= 3.9)

2. **Install Required Packages**
   
   For macOS ARM64:
   ```
   pip install 'kokoro<0.7.6' numpy torch
   ```
   
   For other platforms:
   ```
   pip install kokoro numpy torch
   ```
   
   If using a virtual environment:
   ```
   source /path/to/venv/bin/activate
   pip install <packages as above>
   ```

3. **Configure Environment Variable** (if using virtual environment)
   ```
   export VOXWEAVE_KOKORO_PYTHON=/path/to/venv/bin/python
   ```
   
   Add to shell profile for persistence (`.bashrc`, `.zshrc`, etc.)

4. **Verify Installation**
   ```
   $VOXWEAVE_KOKORO_PYTHON -c "import kokoro; print('Kokoro installed successfully')"
   ```

5. **Test Integration**
   ```
   cargo run -- list-voices
   cargo run -- convert samples/example.txt --voice af_alloy --output output_dir
   ```

### Development Team Actions (Future Enhancements)

1. **Create Requirements File**
   - Add `requirements.txt` or `requirements-macos-arm64.txt` to the repository
   - Document platform-specific constraints
   - Include in installation documentation

2. **Improve Error Diagnostics**
   - Enhance `kokoro_bridge.py` to output detailed diagnostic information on import failure
   - Include Python version, interpreter path, and platform info in error messages
   - Provide platform-specific installation suggestions

3. **Add Setup Validation Script**
   - Create a setup script or cargo build step that validates Python environment
   - Check for required packages before compilation
   - Provide clear setup instructions if dependencies are missing

4. **Update Documentation**
   - Add troubleshooting section for Kokoro import issues
   - Document platform-specific installation procedures
   - Include virtual environment setup recommendations

## Success Criteria

The issue is considered resolved when:

1. ✅ The Python environment contains kokoro, numpy, and torch packages
2. ✅ The `VOXWEAVE_KOKORO_PYTHON` variable (if set) points to the correct interpreter
3. ✅ The bridge script can successfully import kokoro when invoked
4. ✅ The Rust application can list Kokoro voices without errors
5. ✅ TTS conversion with Kokoro voices produces audio files successfully
6. ✅ No import errors appear in stderr during normal operation

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Virtual environment not activated | High | High | Document need to set `VOXWEAVE_KOKORO_PYTHON` when using venv |
| Platform-specific package issues | Medium | Medium | Document macOS ARM64 version constraint |
| Torch installation size/complexity | Medium | Low | Provide CPU-only installation option |
| Conflicting Python versions | Low | High | Clear documentation on Python 3.9+ requirement |

## Related Documentation

Files to update with resolution details:
- `README.md`: Add Kokoro installation troubleshooting section
- `QUICKSTART.md`: Include Python setup verification steps
- Create new: `PYTHON_SETUP.md` with detailed environment configuration guide
