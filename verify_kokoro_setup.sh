#!/bin/bash
# Kokoro TTS Setup Verification Script
# This script verifies that the Kokoro TTS Python environment is correctly configured

set -e

echo "🔍 VoxWeave Kokoro TTS Setup Verification"
echo "=========================================="
echo ""

# Check Python version
echo "1. Checking Python version..."
PYTHON_CMD="${VOXWEAVE_KOKORO_PYTHON:-python3}"
PYTHON_VERSION=$($PYTHON_CMD --version 2>&1)
echo "   Python command: $PYTHON_CMD"
echo "   $PYTHON_VERSION"

# Extract version number
VERSION_NUM=$(echo $PYTHON_VERSION | grep -oE '[0-9]+\.[0-9]+' | head -1)
MAJOR=$(echo $VERSION_NUM | cut -d. -f1)
MINOR=$(echo $VERSION_NUM | cut -d. -f2)

if [ "$MAJOR" -lt 3 ] || ([ "$MAJOR" -eq 3 ] && [ "$MINOR" -lt 9 ]); then
    echo "   ❌ ERROR: Python 3.9+ required, found $PYTHON_VERSION"
    exit 1
else
    echo "   ✅ Python version is compatible"
fi
echo ""

# Check platform
echo "2. Checking platform..."
PLATFORM=$(uname -s)
ARCH=$(uname -m)
echo "   Platform: $PLATFORM"
echo "   Architecture: $ARCH"

if [ "$PLATFORM" = "Darwin" ] && [ "$ARCH" = "arm64" ]; then
    echo "   ℹ️  macOS ARM64 detected - kokoro<0.7.6 required"
fi
echo ""

# Check Python packages
echo "3. Checking required Python packages..."
PACKAGES=("numpy" "torch" "kokoro")
ALL_INSTALLED=true

for pkg in "${PACKAGES[@]}"; do
    if $PYTHON_CMD -c "import $pkg" 2>/dev/null; then
        VERSION=$($PYTHON_CMD -c "import $pkg; print(getattr($pkg, '__version__', 'unknown'))" 2>/dev/null)
        echo "   ✅ $pkg (version: $VERSION)"
    else
        echo "   ❌ $pkg is NOT installed"
        ALL_INSTALLED=false
    fi
done
echo ""

if [ "$ALL_INSTALLED" = false ]; then
    echo "❌ Missing packages detected!"
    echo ""
    echo "To install required packages:"
    if [ "$PLATFORM" = "Darwin" ] && [ "$ARCH" = "arm64" ]; then
        echo "   pip install -r requirements.txt"
        echo "   (or manually: pip install 'kokoro<0.7.6' numpy torch)"
    else
        echo "   pip install -r requirements.txt"
        echo "   (or manually: pip install kokoro numpy torch)"
    fi
    exit 1
fi

# Test Kokoro import
echo "4. Testing Kokoro TTS import..."
if $PYTHON_CMD -c "from kokoro import KPipeline, model as kokoro_model; print('Success')" 2>/dev/null; then
    echo "   ✅ Kokoro modules imported successfully"
else
    echo "   ❌ Failed to import Kokoro modules"
    echo "   This may indicate a broken installation"
    exit 1
fi
echo ""

# Test the bridge script
echo "5. Testing Kokoro bridge script..."
BRIDGE_SCRIPT="python/kokoro_bridge.py"
if [ -f "$BRIDGE_SCRIPT" ]; then
    echo '{"text":"","output":"/tmp/voxweave_verify.wav"}' | $PYTHON_CMD "$BRIDGE_SCRIPT" 2>/dev/null
    if [ -f "/tmp/voxweave_verify.wav" ]; then
        echo "   ✅ Bridge script executed successfully"
        rm -f "/tmp/voxweave_verify.wav"
    else
        echo "   ❌ Bridge script failed to create output file"
        exit 1
    fi
else
    echo "   ⚠️  Bridge script not found at $BRIDGE_SCRIPT"
    echo "   (Skipping bridge test)"
fi
echo ""

# Check environment variables
echo "6. Checking environment variables..."
ENV_VARS=("VOXWEAVE_KOKORO_PYTHON" "VOXWEAVE_KOKORO_DEVICE" "VOXWEAVE_KOKORO_SAMPLE_RATE")
for var in "${ENV_VARS[@]}"; do
    if [ -n "${!var}" ]; then
        echo "   ✅ $var=${!var}"
    else
        echo "   ℹ️  $var not set (using default)"
    fi
done
echo ""

echo "=========================================="
echo "✅ All checks passed!"
echo ""
echo "Your Kokoro TTS environment is properly configured."
echo "You can now use Kokoro voices with VoxWeave:"
echo ""
echo "   cargo run -- list-voices"
echo "   cargo run -- convert input.txt --voice af_alloy --output output/"
echo ""
