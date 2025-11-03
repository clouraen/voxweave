# CLI Video Generation Guide

VoxWeave now supports video generation directly from the command line! This guide explains how to use the CLI to create narrated videos with AI-generated visuals.

## Prerequisites

1. **Build with video generation support:**
   ```bash
   cargo build --features video-generation --release
   ```

2. **Set Z.ai API Key:**
   The video generation feature requires a Z.ai API key. Set it as an environment variable:
   ```bash
   export ZAI_API_KEY=your_api_key_here
   ```

3. **(Optional) Install ffmpeg for subtitle embedding:**
   If you want subtitles embedded directly into the video, install ffmpeg:
   - macOS: `brew install ffmpeg`
   - Ubuntu/Debian: `sudo apt install ffmpeg`
   - Windows: Download from https://ffmpeg.org/

## Usage

### Basic Command

Generate a video from a text or Markdown file:

```bash
voxweave video input.txt
```

This will:
1. Generate audio from the text using the default voice
2. Create subtitle files (.srt)
3. Generate a video with AI visuals matching the content
4. Output: `input.mp4` and `input.srt`

### Command Options

```
Usage: voxweave video [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Input text/markdown file

Options:
  -o, --output <OUTPUT>          Output directory (defaults to same directory as input)
  -v, --voice <VOICE>            Voice identifier (use `list-voices` to see options)
      --style <STYLE>            Video style [default: cyberpunk]
                                 Options: realistic, anime, 3d, cinematic, biotech, 
                                         cyberpunk, educational
      --resolution <RESOLUTION>  Video resolution [default: 1080p]
                                 Options: 720p, 1080p, 4k
      --format <FORMAT>          Video format [default: mp4]
                                 Options: mp4, mov, webm
      --prompt <PROMPT>          Custom visual prompt for video generation
  -s, --speed <SPEED>            Playback speed multiplier [default: 1.0]
  -h, --help                     Print help
```

## Examples

### Example 1: Basic Video Generation

```bash
voxweave video samples/example.txt
```

Creates a cyberpunk-style 1080p MP4 video with default voice.

### Example 2: Custom Style and Resolution

```bash
voxweave video samples/example.txt --style cinematic --resolution 4k
```

Creates a cinematic-style 4K video.

### Example 3: Different Format and Voice

```bash
voxweave video samples/example.txt --format webm --voice af_sky
```

Creates a WebM video using the "af_sky" voice.

### Example 4: Custom Visual Prompt

```bash
voxweave video presentation.md \
  --style educational \
  --prompt "Professional presentation with clean graphics and charts"
```

Generates an educational-style video with custom visual instructions.

### Example 5: Anime Style with Custom Output

```bash
voxweave video story.txt \
  --style anime \
  --resolution 1080p \
  --output ./videos \
  --voice af
```

Creates an anime-style video in the `videos/` directory.

### Example 6: Fast-paced Content

```bash
voxweave video quick-tips.txt \
  --speed 1.2 \
  --style educational \
  --resolution 720p
```

Creates a video with 1.2x speed audio for quick tips content.

## Video Styles Explained

- **`realistic`**: Photorealistic visuals, suitable for documentaries
- **`anime`**: Anime/manga art style
- **`3d`**: 3D rendered graphics
- **`cinematic`**: Movie-like cinematography and lighting
- **`biotech`**: Scientific/biotech themed visuals
- **`cyberpunk`**: Futuristic neon cyberpunk aesthetics (default)
- **`educational`**: Clean, professional educational content style

## Output Files

For an input file `example.txt`, the following files are created:

1. **`example.wav`** - Generated audio file
2. **`example.srt`** - Subtitle file (sentence-based timing)
3. **`example.mp4`** (or `.mov`, `.webm`) - Final video with visuals
4. **`example_subtitled.mp4`** - Video with embedded subtitles (if ffmpeg is available)

## Workflow

The CLI video generation follows this pipeline:

```
1. Text Processing
   └─> Normalize and clean input text
   
2. Audio Generation
   └─> Convert text to speech with selected voice
   └─> Generate subtitle file (.srt)
   
3. Video Generation
   └─> Upload audio to Z.ai API
   └─> Create video generation job
   └─> Poll for completion (with progress updates)
   └─> Download generated video
   
4. Subtitle Integration (if ffmpeg available)
   └─> Embed subtitles directly into video
```

## Troubleshooting

### "ZAI_API_KEY environment variable not set"

**Solution:** Set your API key before running:
```bash
export ZAI_API_KEY=your_api_key_here
```

To make it permanent, add to your shell config (~/.bashrc, ~/.zshrc):
```bash
echo 'export ZAI_API_KEY=your_api_key_here' >> ~/.zshrc
```

### "Video generation timeout"

The API allows up to 5 minutes for video generation. If it times out:
- Try a shorter input text
- Check the Z.ai API status
- Verify your API key has sufficient quota

### "ffmpeg failed" or subtitle embedding errors

If subtitle embedding fails:
- Install ffmpeg: `brew install ffmpeg` (macOS) or `sudo apt install ffmpeg` (Linux)
- The video will still be created, with subtitles as a separate .srt file

### "voice not found"

**Solution:** List available voices and use a valid identifier:
```bash
voxweave list-voices
```

## Advanced Usage

### Batch Processing Multiple Files

```bash
for file in samples/*.txt; do
  voxweave video "$file" --style educational --resolution 1080p
done
```

### Using with Different Voices

First, list available voices:
```bash
voxweave list-voices
```

Then use a specific voice:
```bash
voxweave video input.txt --voice af_bella
```

### Combining with Text Preprocessing

```bash
# Generate video from preprocessed Markdown
cat article.md | sed 's/# //' > processed.txt
voxweave video processed.txt --style cinematic
```

## Performance Notes

- **720p**: Fastest generation, ~1-2 minutes
- **1080p**: Balanced quality, ~2-4 minutes
- **4K**: Highest quality, ~4-5 minutes

Video generation time depends on:
- Content length
- API server load
- Selected resolution
- Network speed

## Comparison: CLI vs Desktop UI

| Feature | CLI | Desktop UI |
|---------|-----|------------|
| Video generation | ✅ Yes | ✅ Yes |
| Batch processing | ✅ Easy | ❌ Manual |
| Progress visualization | Text-based | Graphical |
| Queue management | Sequential | Visual queue |
| Automation-friendly | ✅ Yes | ❌ No |
| Real-time preview | ❌ No | ✅ Yes |

Use **CLI** for:
- Automation and scripting
- Batch processing
- CI/CD pipelines
- Server environments

Use **Desktop UI** for:
- Interactive editing
- Real-time previews
- Visual queue management
- Drag-and-drop workflows

## Integration Examples

### GitHub Actions Workflow

```yaml
name: Generate Video
on: [push]
jobs:
  video:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Install Rust
        uses: actions-rs/toolchain@v1
      - name: Build VoxWeave
        run: cargo build --release --features video-generation
      - name: Generate Video
        env:
          ZAI_API_KEY: ${{ secrets.ZAI_API_KEY }}
        run: |
          ./target/release/voxweave video content.txt --style educational
```

### Cron Job for Daily Content

```bash
#!/bin/bash
# daily-video.sh

export ZAI_API_KEY="your_key_here"
DATE=$(date +%Y-%m-%d)
INPUT="daily-content-${DATE}.txt"
OUTPUT="videos/${DATE}"

voxweave video "$INPUT" \
  --output "$OUTPUT" \
  --style educational \
  --resolution 1080p

echo "Video generated: ${OUTPUT}/daily-content-${DATE}.mp4"
```

## API Rate Limits

Be aware of Z.ai API rate limits:
- Check your plan's quota
- Implement delays between batch requests if needed
- Monitor API usage in your Z.ai dashboard

## See Also

- [Main README](README.md) - General VoxWeave documentation
- [Quick Start Guide](QUICKSTART.md) - Getting started with VoxWeave
- [Desktop UI Video Guide](abogen-ui/ZAI_VIDEO_FEATURE.md) - Desktop app video features
- [Video Integration Plan](abogen-ui/VIDEO_INTEGRATION_PLAN.md) - Technical implementation details

## Support

For issues or questions:
1. Check the troubleshooting section above
2. Review the main documentation
3. Check Z.ai API documentation for service-specific issues
# CLI Video Generation Guide

VoxWeave now supports video generation directly from the command line! This guide explains how to use the CLI to create narrated videos with AI-generated visuals.

## Prerequisites

1. **Build with video generation support:**
   ```bash
   cargo build --features video-generation --release
   ```

2. **Set Z.ai API Key:**
   The video generation feature requires a Z.ai API key. Set it as an environment variable:
   ```bash
   export ZAI_API_KEY=your_api_key_here
   ```

3. **(Optional) Install ffmpeg for subtitle embedding:**
   If you want subtitles embedded directly into the video, install ffmpeg:
   - macOS: `brew install ffmpeg`
   - Ubuntu/Debian: `sudo apt install ffmpeg`
   - Windows: Download from https://ffmpeg.org/

## Usage

### Basic Command

Generate a video from a text or Markdown file:

```bash
voxweave video input.txt
```

This will:
1. Generate audio from the text using the default voice
2. Create subtitle files (.srt)
3. Generate a video with AI visuals matching the content
4. Output: `input.mp4` and `input.srt`

### Command Options

```
Usage: voxweave video [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Input text/markdown file

Options:
  -o, --output <OUTPUT>          Output directory (defaults to same directory as input)
  -v, --voice <VOICE>            Voice identifier (use `list-voices` to see options)
      --style <STYLE>            Video style [default: cyberpunk]
                                 Options: realistic, anime, 3d, cinematic, biotech, 
                                         cyberpunk, educational
      --resolution <RESOLUTION>  Video resolution [default: 1080p]
                                 Options: 720p, 1080p, 4k
      --format <FORMAT>          Video format [default: mp4]
                                 Options: mp4, mov, webm
      --prompt <PROMPT>          Custom visual prompt for video generation
  -s, --speed <SPEED>            Playback speed multiplier [default: 1.0]
  -h, --help                     Print help
```

## Examples

### Example 1: Basic Video Generation

```bash
voxweave video samples/example.txt
```

Creates a cyberpunk-style 1080p MP4 video with default voice.

### Example 2: Custom Style and Resolution

```bash
voxweave video samples/example.txt --style cinematic --resolution 4k
```

Creates a cinematic-style 4K video.

### Example 3: Different Format and Voice

```bash
voxweave video samples/example.txt --format webm --voice af_sky
```

Creates a WebM video using the "af_sky" voice.

### Example 4: Custom Visual Prompt

```bash
voxweave video presentation.md \
  --style educational \
  --prompt "Professional presentation with clean graphics and charts"
```

Generates an educational-style video with custom visual instructions.

### Example 5: Anime Style with Custom Output

```bash
voxweave video story.txt \
  --style anime \
  --resolution 1080p \
  --output ./videos \
  --voice af
```

Creates an anime-style video in the `videos/` directory.

### Example 6: Fast-paced Content

```bash
voxweave video quick-tips.txt \
  --speed 1.2 \
  --style educational \
  --resolution 720p
```

Creates a video with 1.2x speed audio for quick tips content.

## Video Styles Explained

- **`realistic`**: Photorealistic visuals, suitable for documentaries
- **`anime`**: Anime/manga art style
- **`3d`**: 3D rendered graphics
- **`cinematic`**: Movie-like cinematography and lighting
- **`biotech`**: Scientific/biotech themed visuals
- **`cyberpunk`**: Futuristic neon cyberpunk aesthetics (default)
- **`educational`**: Clean, professional educational content style

## Output Files

For an input file `example.txt`, the following files are created:

1. **`example.wav`** - Generated audio file
2. **`example.srt`** - Subtitle file (sentence-based timing)
3. **`example.mp4`** (or `.mov`, `.webm`) - Final video with visuals
4. **`example_subtitled.mp4`** - Video with embedded subtitles (if ffmpeg is available)

## Workflow

The CLI video generation follows this pipeline:

```
1. Text Processing
   └─> Normalize and clean input text
   
2. Audio Generation
   └─> Convert text to speech with selected voice
   └─> Generate subtitle file (.srt)
   
3. Video Generation
   └─> Upload audio to Z.ai API
   └─> Create video generation job
   └─> Poll for completion (with progress updates)
   └─> Download generated video
   
4. Subtitle Integration (if ffmpeg available)
   └─> Embed subtitles directly into video
```

## Troubleshooting

### "ZAI_API_KEY environment variable not set"

**Solution:** Set your API key before running:
```bash
export ZAI_API_KEY=your_api_key_here
```

To make it permanent, add to your shell config (~/.bashrc, ~/.zshrc):
```bash
echo 'export ZAI_API_KEY=your_api_key_here' >> ~/.zshrc
```

### "Video generation timeout"

The API allows up to 5 minutes for video generation. If it times out:
- Try a shorter input text
- Check the Z.ai API status
- Verify your API key has sufficient quota

### "ffmpeg failed" or subtitle embedding errors

If subtitle embedding fails:
- Install ffmpeg: `brew install ffmpeg` (macOS) or `sudo apt install ffmpeg` (Linux)
- The video will still be created, with subtitles as a separate .srt file

### "voice not found"

**Solution:** List available voices and use a valid identifier:
```bash
voxweave list-voices
```

## Advanced Usage

### Batch Processing Multiple Files

```bash
for file in samples/*.txt; do
  voxweave video "$file" --style educational --resolution 1080p
done
```

### Using with Different Voices

First, list available voices:
```bash
voxweave list-voices
```

Then use a specific voice:
```bash
voxweave video input.txt --voice af_bella
```

### Combining with Text Preprocessing

```bash
# Generate video from preprocessed Markdown
cat article.md | sed 's/# //' > processed.txt
voxweave video processed.txt --style cinematic
```

## Performance Notes

- **720p**: Fastest generation, ~1-2 minutes
- **1080p**: Balanced quality, ~2-4 minutes
- **4K**: Highest quality, ~4-5 minutes

Video generation time depends on:
- Content length
- API server load
- Selected resolution
- Network speed

## Comparison: CLI vs Desktop UI

| Feature | CLI | Desktop UI |
|---------|-----|------------|
| Video generation | ✅ Yes | ✅ Yes |
| Batch processing | ✅ Easy | ❌ Manual |
| Progress visualization | Text-based | Graphical |
| Queue management | Sequential | Visual queue |
| Automation-friendly | ✅ Yes | ❌ No |
| Real-time preview | ❌ No | ✅ Yes |

Use **CLI** for:
- Automation and scripting
- Batch processing
- CI/CD pipelines
- Server environments

Use **Desktop UI** for:
- Interactive editing
- Real-time previews
- Visual queue management
- Drag-and-drop workflows

## Integration Examples

### GitHub Actions Workflow

```yaml
name: Generate Video
on: [push]
jobs:
  video:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Install Rust
        uses: actions-rs/toolchain@v1
      - name: Build VoxWeave
        run: cargo build --release --features video-generation
      - name: Generate Video
        env:
          ZAI_API_KEY: ${{ secrets.ZAI_API_KEY }}
        run: |
          ./target/release/voxweave video content.txt --style educational
```

### Cron Job for Daily Content

```bash
#!/bin/bash
# daily-video.sh

export ZAI_API_KEY="your_key_here"
DATE=$(date +%Y-%m-%d)
INPUT="daily-content-${DATE}.txt"
OUTPUT="videos/${DATE}"

voxweave video "$INPUT" \
  --output "$OUTPUT" \
  --style educational \
  --resolution 1080p

echo "Video generated: ${OUTPUT}/daily-content-${DATE}.mp4"
```

## API Rate Limits

Be aware of Z.ai API rate limits:
- Check your plan's quota
- Implement delays between batch requests if needed
- Monitor API usage in your Z.ai dashboard

## See Also

- [Main README](README.md) - General VoxWeave documentation
- [Quick Start Guide](QUICKSTART.md) - Getting started with VoxWeave
- [Desktop UI Video Guide](abogen-ui/ZAI_VIDEO_FEATURE.md) - Desktop app video features
- [Video Integration Plan](abogen-ui/VIDEO_INTEGRATION_PLAN.md) - Technical implementation details

## Support

For issues or questions:
1. Check the troubleshooting section above
2. Review the main documentation
3. Check Z.ai API documentation for service-specific issues
