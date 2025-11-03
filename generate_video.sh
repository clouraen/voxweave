#!/bin/bash
# VoxWeave Video Generation Script
# Usage: ./generate_video.sh <input_file> [style] [resolution]

set -e

INPUT_FILE="${1:-abagen_overview.txt}"
STYLE="${2:-cyberpunk}"
RESOLUTION="${3:-1080p}"

echo "🎬 VoxWeave Video Generator"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Input:      $INPUT_FILE"
echo "Style:      $STYLE"
echo "Resolution: $RESOLUTION"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Check if ZAI_API_KEY is set
if [ -z "$ZAI_API_KEY" ]; then
    echo "⚠️  Warning: ZAI_API_KEY not set"
    echo "   Video generation will create audio + subtitles only"
    echo "   To enable video, set: export ZAI_API_KEY=your_key"
    echo ""
fi

# Build with video-generation feature
echo "🔨 Building VoxWeave..."
cargo build --features video-generation --release

echo ""
echo "🚀 Starting video generation..."
echo ""

# Run the video command
cargo run --features video-generation --release -- video "$INPUT_FILE" \
    --style "$STYLE" \
    --resolution "$RESOLUTION"

echo ""
echo "✅ Done!"
