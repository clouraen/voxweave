# Kokoro TTS Troubleshooting Guide

## Common Issues and Solutions

### Issue: "No module named 'kokoro'" Error

**Symptoms:**
```
command execution failed: failed to import kokoro dependencies: No module named 'kokoro'
```

**Diagnosis:**
This error occurs when the Python environment invoked by VoxWeave doesn't have the kokoro package installed.

**Solutions:**

1. **Install kokoro and dependencies:**
   ```bash
   # For macOS ARM64
   pip install 'kokoro<0.7.6' numpy torch
   
   # For other platforms
   pip install kokoro numpy torch
   
   # Or use the requirements file
   pip install -r requirements.txt
   ```

2. **If using a virtual environment:**
   - Activate your virtual environment first:
     ```bash
     source /path/to/venv/bin/activate  # Linux/macOS
     # or
     /path/to/venv/Scripts/activate     # Windows
     ```
   - Then install packages
   - Set the environment variable to point to the venv Python:
     ```bash
     export VOXWEAVE_KOKORO_PYTHON=/path/to/venv/bin/python
     ```

3. **Verify installation:**
   ```bash
   python3 -c "import kokoro; print('Kokoro version:', kokoro.__version__)"
   ```

4. **Check which Python is being used:**
   ```bash
   which python3
   echo $VOXWEAVE_KOKORO_PYTHON
   ```

### Issue: "misaki" Distribution Not Found (macOS ARM64)

**Symptoms:**
```
ERROR: Could not find a version that satisfies the requirement kokoro
ERROR: No matching distribution found for kokoro
```

**Solution:**
Use kokoro version < 0.7.6 which has ARM64 support:
```bash
pip install 'kokoro<0.7.6' numpy torch
```

### Issue: Wrong Python Interpreter Being Used

**Symptoms:**
- Packages installed but still getting import errors
- Works in terminal but not from VoxWeave

**Diagnosis:**
VoxWeave may be using a different Python interpreter than you think.

**Solutions:**

1. **Check the default Python:**
   ```bash
   which python3
   python3 --version
   ```

2. **Set explicit Python path:**
   ```bash
   # Find your Python installation
   which python3
   
   # Set it explicitly
   export VOXWEAVE_KOKORO_PYTHON=$(which python3)
   
   # Or for virtual environment
   export VOXWEAVE_KOKORO_PYTHON=/path/to/venv/bin/python
   ```

3. **Make it permanent (add to shell profile):**
   ```bash
   # For zsh (macOS default)
   echo 'export VOXWEAVE_KOKORO_PYTHON=/path/to/python' >> ~/.zshrc
   source ~/.zshrc
   
   # For bash
   echo 'export VOXWEAVE_KOKORO_PYTHON=/path/to/python' >> ~/.bashrc
   source ~/.bashrc
   ```

### Issue: Torch Installation Failures

**Symptoms:**
```
ERROR: Failed building wheel for torch
```

**Solutions:**

1. **Install PyTorch from official source:**
   Visit https://pytorch.org/get-started/locally/ and follow platform-specific instructions.
   
   For macOS:
   ```bash
   pip3 install torch torchvision torchaudio
   ```

2. **Use CPU-only version (smaller, faster install):**
   ```bash
   pip3 install torch --index-url https://download.pytorch.org/whl/cpu
   ```

### Issue: Permission Denied Errors

**Symptoms:**
```
ERROR: Could not install packages due to an OSError: [Errno 13] Permission denied
```

**Solutions:**

1. **Use user installation:**
   ```bash
   pip install --user kokoro numpy torch
   ```

2. **Use virtual environment (recommended):**
   ```bash
   python3 -m venv voxweave-env
   source voxweave-env/bin/activate
   pip install kokoro numpy torch
   export VOXWEAVE_KOKORO_PYTHON=$(pwd)/voxweave-env/bin/python
   ```

### Verification Steps

Run the verification script to diagnose issues:
```bash
./verify_kokoro_setup.sh
```

The script will check:
- Python version (must be 3.9+)
- Required packages installation
- Kokoro module imports
- Bridge script functionality
- Environment variable configuration

### Manual Testing

Test the bridge script directly:
```bash
cd /path/to/voxweave
echo '{"text":"Hello world","output":"test.wav","voice":"af_alloy","lang_code":"a","speed":1.0}' | python3 python/kokoro_bridge.py
```

If this works, you should see a `test.wav` file created.

### Environment Variables Reference

| Variable | Purpose | Default |
|----------|---------|---------|
| `VOXWEAVE_KOKORO_PYTHON` | Python interpreter path | `python3` |
| `VOXWEAVE_KOKORO_DEVICE` | Compute device | `cpu` |
| `VOXWEAVE_KOKORO_SAMPLE_RATE` | Audio sample rate | `24000` |
| `VOXWEAVE_KOKORO_REPO_ID` | HuggingFace model repo | `hexgrad/Kokoro-82M` |
| `VOXWEAVE_KOKORO_SPLIT_PATTERN` | Sentence splitting regex | `\n+` |

### Getting Help

If you're still experiencing issues after trying these solutions:

1. Run the verification script and save the output:
   ```bash
   ./verify_kokoro_setup.sh > diagnostic.txt 2>&1
   ```

2. Check the VoxWeave logs for detailed error messages

3. Verify your system meets all requirements:
   - Rust 1.80+
   - Python 3.9+
   - Sufficient disk space for models (~500MB for Kokoro)
   - Internet connection for downloading models on first use

4. Open an issue on GitHub with:
   - Your platform (OS, architecture)
   - Python version
   - Output from verification script
   - Full error message

## Platform-Specific Notes

### macOS ARM64 (M1/M2/M3)
- Must use kokoro<0.7.6
- May need to install Xcode Command Line Tools
- MPS device support available for GPU acceleration

### Linux
- May need to install additional dependencies for audio processing
- CUDA available for NVIDIA GPU acceleration

### Windows
- Use Windows path format for VOXWEAVE_KOKORO_PYTHON
- May need Microsoft Visual C++ Build Tools for torch
