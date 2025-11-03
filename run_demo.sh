#!/bin/bash

echo "🎙️  VoxWeave Demo Script"
echo "========================"
echo ""

# Build the project
echo "📦 Building VoxWeave..."
cargo build
if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build successful!"
echo ""

# List available voices
echo "🎵 Available voices:"
./target/debug/voxweave list-voices
echo ""

# Run mock conversion on example file
echo "🔊 Converting example.txt (mock mode)..."
./target/debug/voxweave convert samples/example.txt --mock --voice en-us --output samples/output
if [ $? -eq 0 ]; then
    echo "✅ Mock conversion successful!"
    echo "   Output: samples/output/example.wav"
    echo "   Subtitles: samples/output/example.srt"
else
    echo "❌ Conversion failed!"
fi

echo ""
echo "🏁 Demo complete!"
echo ""
echo "💡 Tips:"
echo "   - To use real TTS, install espeak-ng: brew install espeak-ng (macOS)"
echo "   - Remove --mock flag to use actual speech synthesis"
echo "   - Use --help to see all options"
#!/bin/bash

echo "🎙️  VoxWeave Demo Script"
echo "========================"
echo ""

# Build the project
echo "📦 Building VoxWeave..."
cargo build
if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build successful!"
echo ""

# List available voices
echo "🎵 Available voices:"
./target/debug/voxweave list-voices
echo ""

# Run mock conversion on example file
echo "🔊 Converting example.txt (mock mode)..."
./target/debug/voxweave convert samples/example.txt --mock --voice en-us --output samples/output
if [ $? -eq 0 ]; then
    echo "✅ Mock conversion successful!"
    echo "   Output: samples/output/example.wav"
    echo "   Subtitles: samples/output/example.srt"
else
    echo "❌ Conversion failed!"
fi

echo ""
echo "🏁 Demo complete!"
echo ""
echo "💡 Tips:"
echo "   - To use real TTS, install espeak-ng: brew install espeak-ng (macOS)"
echo "   - Remove --mock flag to use actual speech synthesis"
echo "   - Use --help to see all options"
