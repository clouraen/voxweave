# VoxWeave CLI Video Generation - Quick Reference

## Setup

```bash
# 1. Build with video support
cargo build --release --features video-generation

# 2. Set API key
export ZAI_API_KEY=your_api_key_here

# 3. (Optional) Make API key permanent
echo 'export ZAI_API_KEY=your_api_key_here' >> ~/.zshrc
```

## Basic Commands

```bash
# Generate video with defaults (cyberpunk style, 1080p, mp4)
voxweave video input.txt

# List available voices
voxweave list-voices

# Generate audio only (no video)
voxweave convert input.txt
```

## Common Options

### Video Styles
```bash
--style realistic      # Photorealistic visuals
--style anime          # Anime/manga art
--style 3d             # 3D rendered graphics
--style cinematic      # Movie-like visuals
--style biotech        # Scientific/biotech themed
--style cyberpunk      # Futuristic neon (default)
--style educational    # Clean, professional
```

### Resolutions
```bash
--resolution 720p      # Fast, smaller files
--resolution 1080p     # Balanced (default)
--resolution 4k        # Highest quality
```

### Formats
```bash
--format mp4           # Default, widely compatible
--format mov           # High quality, Apple
--format webm          # Web-optimized
```

## Examples

### 1. Quick Test
```bash
voxweave video samples/example.txt
```

### 2. Educational Video (4K)
```bash
voxweave video lecture.md \
  --style educational \
  --resolution 4k \
  --voice af_bella
```

### 3. Anime Story Video
```bash
voxweave video story.txt \
  --style anime \
  --resolution 1080p \
  --prompt "Fantasy adventure in vibrant anime style"
```

### 4. Cinematic Presentation
```bash
voxweave video presentation.md \
  --style cinematic \
  --resolution 1080p \
  --format mov
```

### 5. Fast-paced Tutorial
```bash
voxweave video tutorial.txt \
  --style educational \
  --speed 1.2 \
  --resolution 720p
```

## Batch Processing

### Process Directory
```bash
for file in content/*.txt; do
  voxweave video "$file" --style educational
done
```

### With Custom Output
```bash
for file in content/*.txt; do
  voxweave video "$file" \
    --output ./videos \
    --style cinematic \
    --resolution 1080p
done
```

### Parallel Processing (with GNU parallel)
```bash
parallel voxweave video {} --style educational ::: content/*.txt
```

## Output Files

For input `example.txt`:
- `example.wav` - Audio file
- `example.srt` - Subtitle file
- `example.mp4` - Video (or .mov, .webm)
- `example_subtitled.mp4` - With embedded subs (if ffmpeg available)

## Troubleshooting

### No API Key
```
Error: ZAI_API_KEY environment variable not set
```
**Fix**: `export ZAI_API_KEY=your_key`

### Timeout
```
Error: Video generation timeout
```
**Fix**: Try shorter text or lower resolution

### No Voice Found
```
Error: voice not found
```
**Fix**: Run `voxweave list-voices` and use valid voice ID

### ffmpeg Not Found
**Fix**: `brew install ffmpeg` (macOS) or `sudo apt install ffmpeg` (Linux)
Video will still work; subtitles will be external .srt file

## Tips

1. **Start with 720p** for testing, use 4K only for final production
2. **Use custom prompts** to guide visual style
3. **Batch processing**: Add sleep between requests to avoid rate limits
4. **Monitor quota**: Check Z.ai dashboard for API usage

## Full Documentation

- Complete guide: `CLI_VIDEO_GENERATION.md`
- Implementation details: `VIDEO_CLI_IMPLEMENTATION.md`
- General VoxWeave docs: `README.md`

## Help Commands

```bash
voxweave --help                # Main help
voxweave video --help          # Video command help
voxweave list-voices           # Show available voices
voxweave convert --help        # Audio-only help
```
# VoxWeave CLI Video Generation - Quick Reference

## Setup

```bash
# 1. Build with video support
cargo build --release --features video-generation

# 2. Set API key
export ZAI_API_KEY=your_api_key_here

# 3. (Optional) Make API key permanent
echo 'export ZAI_API_KEY=your_api_key_here' >> ~/.zshrc
```

## Basic Commands

```bash
# Generate video with defaults (cyberpunk style, 1080p, mp4)
voxweave video input.txt

# List available voices
voxweave list-voices

# Generate audio only (no video)
voxweave convert input.txt
```

## Common Options

### Video Styles
```bash
--style realistic      # Photorealistic visuals
--style anime          # Anime/manga art
--style 3d             # 3D rendered graphics
--style cinematic      # Movie-like visuals
--style biotech        # Scientific/biotech themed
--style cyberpunk      # Futuristic neon (default)
--style educational    # Clean, professional
```

### Resolutions
```bash
--resolution 720p      # Fast, smaller files
--resolution 1080p     # Balanced (default)
--resolution 4k        # Highest quality
```

### Formats
```bash
--format mp4           # Default, widely compatible
--format mov           # High quality, Apple
--format webm          # Web-optimized
```

## Examples

### 1. Quick Test
```bash
voxweave video samples/example.txt
```

### 2. Educational Video (4K)
```bash
voxweave video lecture.md \
  --style educational \
  --resolution 4k \
  --voice af_bella
```

### 3. Anime Story Video
```bash
voxweave video story.txt \
  --style anime \
  --resolution 1080p \
  --prompt "Fantasy adventure in vibrant anime style"
```

### 4. Cinematic Presentation
```bash
voxweave video presentation.md \
  --style cinematic \
  --resolution 1080p \
  --format mov
```

### 5. Fast-paced Tutorial
```bash
voxweave video tutorial.txt \
  --style educational \
  --speed 1.2 \
  --resolution 720p
```

## Batch Processing

### Process Directory
```bash
for file in content/*.txt; do
  voxweave video "$file" --style educational
done
```

### With Custom Output
```bash
for file in content/*.txt; do
  voxweave video "$file" \
    --output ./videos \
    --style cinematic \
    --resolution 1080p
done
```

### Parallel Processing (with GNU parallel)
```bash
parallel voxweave video {} --style educational ::: content/*.txt
```

## Output Files

For input `example.txt`:
- `example.wav` - Audio file
- `example.srt` - Subtitle file
- `example.mp4` - Video (or .mov, .webm)
- `example_subtitled.mp4` - With embedded subs (if ffmpeg available)

## Troubleshooting

### No API Key
```
Error: ZAI_API_KEY environment variable not set
```
**Fix**: `export ZAI_API_KEY=your_key`

### Timeout
```
Error: Video generation timeout
```
**Fix**: Try shorter text or lower resolution

### No Voice Found
```
Error: voice not found
```
**Fix**: Run `voxweave list-voices` and use valid voice ID

### ffmpeg Not Found
**Fix**: `brew install ffmpeg` (macOS) or `sudo apt install ffmpeg` (Linux)
Video will still work; subtitles will be external .srt file

## Tips

1. **Start with 720p** for testing, use 4K only for final production
2. **Use custom prompts** to guide visual style
3. **Batch processing**: Add sleep between requests to avoid rate limits
4. **Monitor quota**: Check Z.ai dashboard for API usage

## Full Documentation

- Complete guide: `CLI_VIDEO_GENERATION.md`
- Implementation details: `VIDEO_CLI_IMPLEMENTATION.md`
- General VoxWeave docs: `README.md`

## Help Commands

```bash
voxweave --help                # Main help
voxweave video --help          # Video command help
voxweave list-voices           # Show available voices
voxweave convert --help        # Audio-only help
```
