#!/bin/bash

echo "==========================================="
echo "🧪 VoxWeave - Final Verification Test"
echo "==========================================="
echo ""

# Test 1: Build
echo "Test 1: Building project..."
if cargo build --quiet 2>/dev/null; then
    echo "✅ Build: PASSED"
else
    echo "❌ Build: FAILED"
    exit 1
fi

# Test 2: Unit Tests
echo "Test 2: Running unit tests..."
TEST_OUTPUT=$(cargo test --lib --quiet 2>&1)
if echo "$TEST_OUTPUT" | grep -q "test result: ok"; then
    PASSED=$(echo "$TEST_OUTPUT" | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+")
    echo "✅ Tests: PASSED ($PASSED tests)"
else
    echo "❌ Tests: FAILED"
    exit 1
fi

# Test 3: CLI Help
echo "Test 3: CLI help command..."
if ./target/debug/voxweave --help &>/dev/null; then
    echo "✅ CLI Help: PASSED"
else
    echo "❌ CLI Help: FAILED"
    exit 1
fi

# Test 4: List Voices
echo "Test 4: List voices command..."
VOICE_COUNT=$(./target/debug/voxweave list-voices 2>/dev/null | grep -c "espeak\|kokoro")
if [ "$VOICE_COUNT" -gt 0 ]; then
    echo "✅ List Voices: PASSED ($VOICE_COUNT voices)"
else
    echo "❌ List Voices: FAILED"
    exit 1
fi

# Test 5: Convert with Mock
echo "Test 5: Convert command (mock mode)..."
mkdir -p samples/test_output
if ./target/debug/voxweave convert samples/example.txt --mock --voice en-us --output samples/test_output 2>&1 | grep -q "Audio saved"; then
    echo "✅ Convert: PASSED"
    rm -rf samples/test_output
else
    echo "❌ Convert: FAILED"
    exit 1
fi

echo ""
echo "==========================================="
echo "🎉 ALL TESTS PASSED!"
echo "==========================================="
echo ""
echo "Summary:"
echo "  ✅ Build compilation"
echo "  ✅ Unit tests ($PASSED/17)"
echo "  ✅ CLI interface"
echo "  ✅ Voice listing"
echo "  ✅ Text conversion"
echo ""
echo "Project Status: FULLY OPERATIONAL ✨"
#!/bin/bash

echo "==========================================="
echo "🧪 VoxWeave - Final Verification Test"
echo "==========================================="
echo ""

# Test 1: Build
echo "Test 1: Building project..."
if cargo build --quiet 2>/dev/null; then
    echo "✅ Build: PASSED"
else
    echo "❌ Build: FAILED"
    exit 1
fi

# Test 2: Unit Tests
echo "Test 2: Running unit tests..."
TEST_OUTPUT=$(cargo test --lib --quiet 2>&1)
if echo "$TEST_OUTPUT" | grep -q "test result: ok"; then
    PASSED=$(echo "$TEST_OUTPUT" | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+")
    echo "✅ Tests: PASSED ($PASSED tests)"
else
    echo "❌ Tests: FAILED"
    exit 1
fi

# Test 3: CLI Help
echo "Test 3: CLI help command..."
if ./target/debug/voxweave --help &>/dev/null; then
    echo "✅ CLI Help: PASSED"
else
    echo "❌ CLI Help: FAILED"
    exit 1
fi

# Test 4: List Voices
echo "Test 4: List voices command..."
VOICE_COUNT=$(./target/debug/voxweave list-voices 2>/dev/null | grep -c "espeak\|kokoro")
if [ "$VOICE_COUNT" -gt 0 ]; then
    echo "✅ List Voices: PASSED ($VOICE_COUNT voices)"
else
    echo "❌ List Voices: FAILED"
    exit 1
fi

# Test 5: Convert with Mock
echo "Test 5: Convert command (mock mode)..."
mkdir -p samples/test_output
if ./target/debug/voxweave convert samples/example.txt --mock --voice en-us --output samples/test_output 2>&1 | grep -q "Audio saved"; then
    echo "✅ Convert: PASSED"
    rm -rf samples/test_output
else
    echo "❌ Convert: FAILED"
    exit 1
fi

echo ""
echo "==========================================="
echo "🎉 ALL TESTS PASSED!"
echo "==========================================="
echo ""
echo "Summary:"
echo "  ✅ Build compilation"
echo "  ✅ Unit tests ($PASSED/17)"
echo "  ✅ CLI interface"
echo "  ✅ Voice listing"
echo "  ✅ Text conversion"
echo ""
echo "Project Status: FULLY OPERATIONAL ✨"
