//! ZIP package builder for VoxWeave narration projects
//! 
//! Creates organized ZIP archives containing audio, subtitles, MLT project files,
//! and instructional documentation for external video editing.

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use zip::write::{FileOptions, ZipWriter};
use zip::CompressionMethod;

/// Create a complete narration package as a ZIP file
pub fn create_package(
    audio_path: &Path,
    subtitle_path: Option<&Path>,
    mlt_content: &str,
    output_path: &Path,
) -> Result<PathBuf> {
    // Create temporary directory for staging
    let temp_dir = tempfile::tempdir()
        .context("Failed to create temporary directory")?;
    
    let staging_path = temp_dir.path();
    
    // Create directory structure
    create_directory_structure(staging_path)?;
    
    // Copy files to appropriate locations
    copy_files_to_structure(staging_path, audio_path, subtitle_path)?;
    
    // Write MLT project file
    let mlt_path = staging_path.join("project.mlt");
    fs::write(&mlt_path, mlt_content)
        .context("Failed to write MLT project file")?;
    
    // Generate README files
    generate_readme_files(staging_path, audio_path, subtitle_path.is_some())?;
    
    // Create ZIP archive
    let zip_path = compress_to_zip(staging_path, output_path)?;
    
    Ok(zip_path)
}

/// Create the standardized folder structure
fn create_directory_structure(base_path: &Path) -> Result<()> {
    fs::create_dir_all(base_path.join("audio"))
        .context("Failed to create audio directory")?;
    fs::create_dir_all(base_path.join("subtitles"))
        .context("Failed to create subtitles directory")?;
    fs::create_dir_all(base_path.join("video"))
        .context("Failed to create video directory")?;
    
    // Create .gitkeep placeholder in video folder
    File::create(base_path.join("video/.gitkeep"))
        .context("Failed to create .gitkeep file")?;
    
    Ok(())
}

/// Copy audio and subtitle files to the staging structure
fn copy_files_to_structure(
    staging_path: &Path,
    audio_path: &Path,
    subtitle_path: Option<&Path>,
) -> Result<()> {
    // Copy audio file
    let audio_filename = audio_path.file_name()
        .ok_or_else(|| anyhow::anyhow!("Invalid audio filename"))?;
    let dest_audio = staging_path.join("audio").join(audio_filename);
    fs::copy(audio_path, &dest_audio)
        .with_context(|| format!("Failed to copy audio file to {}", dest_audio.display()))?;
    
    // Copy subtitle file if it exists
    if let Some(sub_path) = subtitle_path {
        let sub_filename = sub_path.file_name()
            .ok_or_else(|| anyhow::anyhow!("Invalid subtitle filename"))?;
        let dest_sub = staging_path.join("subtitles").join(sub_filename);
        fs::copy(sub_path, &dest_sub)
            .with_context(|| format!("Failed to copy subtitle file to {}", dest_sub.display()))?;
    }
    
    Ok(())
}

/// Generate instructional README files
fn generate_readme_files(
    staging_path: &Path,
    audio_path: &Path,
    has_subtitles: bool,
) -> Result<()> {
    // Root README
    let root_readme = generate_root_readme(audio_path, has_subtitles);
    fs::write(staging_path.join("README.txt"), root_readme)
        .context("Failed to write root README")?;
    
    // Video folder README
    let video_readme = generate_video_readme();
    fs::write(staging_path.join("video/README.txt"), video_readme)
        .context("Failed to write video README")?;
    
    Ok(())
}

/// Generate the root-level README content
fn generate_root_readme(audio_path: &Path, has_subtitles: bool) -> String {
    let audio_name = audio_path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("narration.wav");
    
    format!(r#"VoxWeave Narration Package
==========================

This package contains all the assets needed to create a narrated video
with cyberpunk-styled subtitles.

📁 PACKAGE CONTENTS
-------------------
audio/          - Generated narration audio file
subtitles/      - Synchronized subtitle file (SRT format){}
video/          - Placeholder for your background video
project.mlt     - MLT project file (open in Kdenlive or compatible editor)
README.txt      - This file

🎬 QUICK START
--------------
1. Extract this ZIP file to a folder
2. Add your 5-second video to the video/ folder
   - Rename it to: background.mp4 (or edit project.mlt to match your filename)
   - Supported formats: MP4, MOV, WebM
   - The video will loop for the duration of the audio

3. Open project.mlt in an MLT-compatible video editor:
   - Kdenlive (recommended): https://kdenlive.org
   - Shotcut: https://shotcut.org
   - OpenShot: https://openshot.org

4. Review the composition:
   - Video track: Your background video (looped)
   - Audio track: {}
   - Subtitle track: Cyberpunk-styled captions{}

5. Render/export the final video

🎨 SUBTITLE STYLING
-------------------
The subtitles feature a cyberpunk/Blade Runner aesthetic:
- Neon cyan text with purple outlines
- Monospace font for technical feel
- Holographic glow effects
- Asymmetric left-aligned layout

You can customize the styling in your MLT editor if desired.

📋 TECHNICAL SPECS
------------------
Audio: 48kHz WAV, stereo
Subtitles: SRT format with timing synchronized to audio
MLT Profile: HD 1080p 30fps (will auto-adjust to your video)
Color Space: sRGB with HDR hints for neon colors

💡 TIPS
-------
- Use a dark, atmospheric 5-second video for best results
- The video will seamlessly loop - choose something that loops well
- For boomerang effects, render your video with forward+reverse motion
- Adjust subtitle glow intensity in your editor for different moods

🔧 TROUBLESHOOTING
------------------
If the MLT project doesn't load:
1. Check that your video file is in video/background.mp4
2. Ensure you're using a recent version of Kdenlive or Shotcut
3. Try manually importing the audio and subtitle files

For more help, visit: https://github.com/yourusername/voxweave

Generated by VoxWeave - Text-to-Speech Narration Pipeline
"#,
        if has_subtitles { "" } else { " (disabled)" },
        audio_name,
        if has_subtitles { "" } else { " (none)" }
    )
}

/// Generate the video folder README content
fn generate_video_readme() -> String {
    r#"📹 Background Video Instructions
================================

This folder is where you should place your background video.

REQUIRED FILE
-------------
Name your video file: background.mp4

Alternatively, you can use a different name and update the project.mlt file
to reference it (search for "video/background.mp4" and replace).

VIDEO REQUIREMENTS
------------------
✓ Duration: 5 seconds (will loop automatically)
✓ Format: MP4, MOV, or WebM
✓ Resolution: Any (MLT will scale to match project)
✓ Recommended: 1080p or higher for best quality

SUGGESTED CONTENT
-----------------
For cyberpunk narration videos, consider:
- Futuristic cityscape with neon lights
- Abstract tech patterns or data visualizations
- Atmospheric sci-fi environments
- Looping animations or cinemagraphs

BOOMERANG EFFECT
----------------
For a smooth boomerang loop:
1. Record 2.5 seconds of footage
2. Reverse and append to create 5-second clip
3. Ensure first and last frames match for seamless loop

WHERE TO GET VIDEOS
-------------------
- Generate with AI: Runway, Pika Labs, Stable Video Diffusion
- Stock footage: Pexels, Pixabay, Videezy (filter for looping)
- Create your own: Blender, After Effects, DaVinci Resolve

Once you've added your video, open project.mlt in Kdenlive!
"#.to_string()
}

/// Compress the staging directory into a ZIP archive
fn compress_to_zip(source_dir: &Path, output_path: &Path) -> Result<PathBuf> {
    let zip_file = File::create(output_path)
        .with_context(|| format!("Failed to create ZIP file: {}", output_path.display()))?;
    
    let mut zip = ZipWriter::new(zip_file);
    let options = FileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o644);
    
    // Walk through the source directory and add files
    add_directory_to_zip(&mut zip, source_dir, source_dir, options)?;
    
    zip.finish()
        .context("Failed to finalize ZIP archive")?;
    
    Ok(output_path.to_path_buf())
}

/// Recursively add directory contents to ZIP archive
fn add_directory_to_zip<W: Write + io::Seek>(
    zip: &mut ZipWriter<W>,
    base_path: &Path,
    current_path: &Path,
    options: FileOptions,
) -> Result<()> {
    for entry in fs::read_dir(current_path)
        .with_context(|| format!("Failed to read directory: {}", current_path.display()))? {
        let entry = entry?;
        let path = entry.path();
        let name = path.strip_prefix(base_path)
            .unwrap_or(&path)
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid UTF-8 in path"))?;
        
        if path.is_file() {
            zip.start_file(name, options)
                .with_context(|| format!("Failed to start ZIP entry: {}", name))?;
            let mut file = File::open(&path)?;
            io::copy(&mut file, zip)
                .with_context(|| format!("Failed to write file to ZIP: {}", name))?;
        } else if path.is_dir() {
            // Create directory entry
            zip.add_directory(format!("{}/", name), options)
                .with_context(|| format!("Failed to add directory to ZIP: {}", name))?;
            // Recurse
            add_directory_to_zip(zip, base_path, &path, options)?;
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn test_create_directory_structure() {
        let temp_dir = tempfile::tempdir().unwrap();
        create_directory_structure(temp_dir.path()).unwrap();
        
        assert!(temp_dir.path().join("audio").is_dir());
        assert!(temp_dir.path().join("subtitles").is_dir());
        assert!(temp_dir.path().join("video").is_dir());
        assert!(temp_dir.path().join("video/.gitkeep").is_file());
    }
    
    #[test]
    fn test_readme_generation() {
        let readme = generate_root_readme(Path::new("test.wav"), true);
        assert!(readme.contains("VoxWeave"));
        assert!(readme.contains("cyberpunk"));
        assert!(readme.contains("Kdenlive"));
    }
    
    #[test]
    fn test_video_readme() {
        let readme = generate_video_readme();
        assert!(readme.contains("background.mp4"));
        assert!(readme.contains("5 seconds"));
        assert!(readme.contains("boomerang"));
    }
}
