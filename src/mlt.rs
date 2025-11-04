//! MLT project file generator with cyberpunk subtitle styling
//! 
//! This module creates MLT XML project files configured for video composition
//! with professional cyberpunk-themed subtitle styling inspired by
//! Cyberpunk 2077 and Blade Runner 2049.

use std::path::Path;
use anyhow::{Context, Result};

/// Configuration for MLT project generation
#[derive(Debug, Clone)]
pub struct MltConfig {
    /// Frame rate (default: 30 fps)
    pub frame_rate: u32,
    /// Sample rate for audio (default: 48000 Hz)
    pub sample_rate: u32,
    /// Video duration in seconds (from audio length)
    pub duration_seconds: f64,
}

impl Default for MltConfig {
    fn default() -> Self {
        Self {
            frame_rate: 30,
            sample_rate: 48000,
            duration_seconds: 0.0,
        }
    }
}

/// Generate an MLT project file with cyberpunk styling
pub fn create_mlt_project(
    audio_path: &Path,
    subtitle_path: Option<&Path>,
    config: &MltConfig,
) -> Result<String> {
    let audio_rel = audio_path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("narration.wav");
    
    let subtitle_rel = subtitle_path
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str());
    
    let duration_frames = (config.duration_seconds * config.frame_rate as f64).ceil() as u64;
    
    let mut mlt = String::new();
    
    // MLT header
    mlt.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
    mlt.push_str("<mlt LC_NUMERIC=\"C\" version=\"7.0.0\" ");
    mlt.push_str("root=\"\" ");
    mlt.push_str(&format!("producer=\"main_bin\">\n"));
    
    // Profile
    mlt.push_str(&format!("  <profile description=\"HD 1080p 30 fps\" width=\"1920\" height=\"1080\" progressive=\"1\" sample_aspect_num=\"1\" sample_aspect_den=\"1\" display_aspect_num=\"16\" display_aspect_den=\"9\" frame_rate_num=\"{}\" frame_rate_den=\"1\" colorspace=\"709\"/>\n", config.frame_rate));
    
    // Playlist for audio
    mlt.push_str("  <playlist id=\"playlist_audio\">\n");
    mlt.push_str(&format!("    <entry producer=\"audio_producer\" in=\"0\" out=\"{}\"/>\n", duration_frames));
    mlt.push_str("  </playlist>\n");
    
    // Audio producer
    mlt.push_str(&format!("  <producer id=\"audio_producer\">\n"));
    mlt.push_str(&format!("    <property name=\"resource\">audio/{}</property>\n", audio_rel));
    mlt.push_str("    <property name=\"mlt_service\">avformat</property>\n");
    mlt.push_str("    <property name=\"audio_index\">0</property>\n");
    mlt.push_str("  </producer>\n");
    
    // Video playlist (expects user-provided video)
    mlt.push_str("  <playlist id=\"playlist_video\">\n");
    mlt.push_str(&format!("    <entry producer=\"video_producer\" in=\"0\" out=\"{}\"/>\n", duration_frames));
    mlt.push_str("  </playlist>\n");
    
    // Video producer (looping 5-second clip)
    mlt.push_str("  <producer id=\"video_producer\">\n");
    mlt.push_str("    <property name=\"resource\">video/background.mp4</property>\n");
    mlt.push_str("    <property name=\"mlt_service\">avformat</property>\n");
    mlt.push_str("    <property name=\"video_index\">0</property>\n");
    mlt.push_str("    <property name=\"loop\">1</property>\n");
    mlt.push_str("  </producer>\n");
    
    // Subtitle track (if subtitles exist)
    if let Some(sub_file) = subtitle_rel {
        mlt.push_str("  <playlist id=\"playlist_subtitle\">\n");
        mlt.push_str(&format!("    <entry producer=\"subtitle_producer\" in=\"0\" out=\"{}\"/>\n", duration_frames));
        mlt.push_str("  </playlist>\n");
        
        // Subtitle producer with cyberpunk styling
        mlt.push_str("  <producer id=\"subtitle_producer\">\n");
        mlt.push_str("    <property name=\"mlt_service\">subtitle</property>\n");
        mlt.push_str(&format!("    <property name=\"resource\">subtitles/{}</property>\n", sub_file));
        mlt.push_str(&configure_cyberpunk_subtitle_style());
        mlt.push_str("  </producer>\n");
    }
    
    // Main tractor (timeline)
    mlt.push_str("  <tractor id=\"tractor0\" in=\"0\" out=\"");
    mlt.push_str(&duration_frames.to_string());
    mlt.push_str("\">\n");
    mlt.push_str("    <track producer=\"playlist_video\"/>\n");
    mlt.push_str("    <track producer=\"playlist_audio\"/>\n");
    if subtitle_rel.is_some() {
        mlt.push_str("    <track producer=\"playlist_subtitle\"/>\n");
    }
    mlt.push_str("  </tractor>\n");
    
    mlt.push_str("</mlt>\n");
    
    Ok(mlt)
}

/// Configure cyberpunk-themed subtitle styling
/// Inspired by Cyberpunk 2077 and Blade Runner 2049 aesthetics
fn configure_cyberpunk_subtitle_style() -> String {
    let mut style = String::new();
    
    // Font configuration - monospace for cyberpunk feel
    style.push_str("    <property name=\"family\">Courier New,Consolas,Roboto Mono,monospace</property>\n");
    style.push_str("    <property name=\"size\">44</property>\n");
    style.push_str("    <property name=\"weight\">500</property>\n");
    style.push_str("    <property name=\"letter-spacing\">1.5</property>\n");
    
    // Cyberpunk neon color palette
    style.push_str("    <property name=\"fgcolour\">#00FFFFF2</property>\n"); // Cyan/Teal (95% opacity)
    style.push_str("    <property name=\"bgcolour\">#0A0A1ECC</property>\n"); // Dark box (80% opacity)
    style.push_str("    <property name=\"olcolour\">#8A2BE2</property>\n"); // Purple/Magenta outline
    style.push_str("    <property name=\"outline\">2</property>\n");
    
    // Positioning - bottom-left-center asymmetric layout
    style.push_str("    <property name=\"valign\">bottom</property>\n");
    style.push_str("    <property name=\"halign\">left</property>\n");
    style.push_str("    <property name=\"geometry\">48%/85%/85%/10%</property>\n"); // x/y/width/margin
    
    // Text alignment - left-justified for terminal feel
    style.push_str("    <property name=\"align\">left</property>\n");
    
    // Glow and shadow effects for neon holographic appearance
    style.push_str("    <property name=\"glow\">4</property>\n"); // Cyan glow
    style.push_str("    <property name=\"shadow\">3</property>\n"); // Purple shadow
    style.push_str("    <property name=\"shadowcolour\">#8A2BE2</property>\n");
    
    // Fade transitions - hologram materialization effect
    style.push_str("    <property name=\"fade_in\">150</property>\n");
    style.push_str("    <property name=\"fade_out\">200</property>\n");
    
    // Maximum 2 lines
    style.push_str("    <property name=\"max_lines\">2</property>\n");
    
    style
}

/// Calculate audio duration from WAV file
/// Returns duration in seconds
pub fn get_audio_duration(audio_path: &Path) -> Result<f64> {
    // Read WAV header to get duration
    use std::fs::File;
    use std::io::{Read, Seek, SeekFrom};
    
    let mut file = File::open(audio_path)
        .with_context(|| format!("Failed to open audio file: {}", audio_path.display()))?;
    
    // Read WAV header
    let mut header = [0u8; 44];
    file.read_exact(&mut header)
        .context("Failed to read WAV header")?;
    
    // Check for RIFF header
    if &header[0..4] != b"RIFF" || &header[8..12] != b"WAVE" {
        anyhow::bail!("Invalid WAV file format");
    }
    
    // Get file size
    file.seek(SeekFrom::End(0))?;
    let file_size = file.stream_position()? as u32;
    
    // Extract audio parameters
    let _sample_rate = u32::from_le_bytes([header[24], header[25], header[26], header[27]]);
    let byte_rate = u32::from_le_bytes([header[28], header[29], header[30], header[31]]);
    
    if byte_rate == 0 {
        anyhow::bail!("Invalid byte rate in WAV file");
    }
    
    // Calculate duration: (file_size - header_size) / byte_rate
    let data_size = file_size.saturating_sub(44);
    let duration = data_size as f64 / byte_rate as f64;
    
    Ok(duration)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    
    #[test]
    fn test_mlt_generation_basic() {
        let audio_path = PathBuf::from("audio/test.wav");
        let config = MltConfig {
            duration_seconds: 10.0,
            ..Default::default()
        };
        
        let mlt = create_mlt_project(&audio_path, None, &config).unwrap();
        
        assert!(mlt.contains("<?xml version=\"1.0\""));
        assert!(mlt.contains("<mlt"));
        assert!(mlt.contains("audio/test.wav"));
        assert!(mlt.contains("video/background.mp4"));
        assert!(mlt.contains("</mlt>"));
    }
    
    #[test]
    fn test_mlt_with_subtitles() {
        let audio_path = PathBuf::from("audio/test.wav");
        let subtitle_path = PathBuf::from("subtitles/test.srt");
        let config = MltConfig {
            duration_seconds: 10.0,
            ..Default::default()
        };
        
        let mlt = create_mlt_project(&audio_path, Some(&subtitle_path), &config).unwrap();
        
        assert!(mlt.contains("subtitles/test.srt"));
        assert!(mlt.contains("subtitle_producer"));
        assert!(mlt.contains("#00FFFF")); // Cyan color
        assert!(mlt.contains("Courier New")); // Monospace font
    }
    
    #[test]
    fn test_cyberpunk_styling() {
        let style = configure_cyberpunk_subtitle_style();
        
        // Check for cyberpunk neon colors
        assert!(style.contains("#00FFFF")); // Cyan
        assert!(style.contains("#8A2BE2")); // Purple
        
        // Check for monospace font
        assert!(style.contains("Courier New"));
        assert!(style.contains("monospace"));
        
        // Check for effects
        assert!(style.contains("glow"));
        assert!(style.contains("shadow"));
        assert!(style.contains("fade_in"));
    }
}
