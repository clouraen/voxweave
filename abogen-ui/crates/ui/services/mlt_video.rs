/// MLT (Media Lovin' Toolkit) XML scripting for video processing
/// Handles merging audio with video and real-time word-highlighted subtitles

use crate::state::{LogEntry, LogLevel, VideoFormat};
use dioxus::prelude::{Signal, WritableExt};
use std::path::{Path, PathBuf};
use std::fs;
use std::process::Command;

/// Word-level subtitle entry for real-time highlighting
#[derive(Debug, Clone)]
pub struct WordSubtitle {
    pub word: String,
    pub start_ms: u64,
    pub end_ms: u64,
}

/// Generate word-level subtitle timing from text and audio duration
pub fn generate_word_timing(text: &str, audio_duration_ms: u64, _average_wpm: f32) -> Vec<WordSubtitle> {
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.is_empty() {
        return Vec::new();
    }
    
    let total_words = words.len() as f32;
    let ms_per_word = if total_words > 0.0 {
        (audio_duration_ms as f32 / total_words).max(50.0) // Minimum 50ms per word
    } else {
        audio_duration_ms as f32
    };
    
    let mut word_timings = Vec::new();
    let mut cursor: u64 = 0;
    
    for word in words {
        let word_ms = (ms_per_word * 1.0).round() as u64; // Base time per word
        let start = cursor;
        let end = cursor + word_ms;
        
        word_timings.push(WordSubtitle {
            word: word.to_string(),
            start_ms: start,
            end_ms: end.min(audio_duration_ms),
        });
        
        cursor = end;
    }
    
    // Adjust last word to match audio duration exactly
    if let Some(last) = word_timings.last_mut() {
        last.end_ms = audio_duration_ms;
    }
    
    word_timings
}

/// Get audio duration from WAV file
pub fn get_audio_duration_ms(audio_path: &Path) -> Result<u64, String> {
    // Use ffprobe to get audio duration
    let output = Command::new("ffprobe")
        .args(&[
            "-v", "error",
            "-show_entries", "format=duration",
            "-of", "default=noprint_wrappers=1:nokey=1",
            audio_path.to_str().unwrap(),
        ])
        .output()
        .map_err(|e| format!("Failed to run ffprobe: {}", e))?;
    
    if !output.status.success() {
        return Err(format!("ffprobe failed: {}", String::from_utf8_lossy(&output.stderr)));
    }
    
    let duration_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let duration_sec: f64 = duration_str
        .parse()
        .map_err(|e| format!("Failed to parse duration: {}", e))?;
    
    Ok((duration_sec * 1000.0) as u64)
}

/// Generate MLT XML script for video with audio and word-highlighted subtitles
pub fn generate_mlt_xml(
    video_path: Option<&Path>,
    audio_path: &Path,
    _subtitle_text: &str,
    word_timings: &[WordSubtitle],
    _output_path: &Path,
    width: u32,
    height: u32,
    _format: VideoFormat,
) -> Result<String, String> {
    use std::fmt::Write;
    
    let mut xml = String::new();
    
    // MLT XML header
    writeln!(xml, "<?xml version=\"1.0\" encoding=\"utf-8\"?>").unwrap();
    writeln!(xml, "<mlt>").unwrap();
    
    // Main playlist
    writeln!(xml, "  <playlist id=\"main\">").unwrap();
    
    // Add video producer if provided
    if let Some(video) = video_path {
        writeln!(xml, "    <producer id=\"video\" resource=\"{}\">", 
            escape_xml(video.to_str().unwrap())).unwrap();
        writeln!(xml, "      <property name=\"mlt_service\">avformat</property>").unwrap();
        writeln!(xml, "    </producer>").unwrap();
        writeln!(xml, "    <entry producer=\"video\" in=\"0\" out=\"9999999\"/>").unwrap();
    } else {
        // Create color producer as background
        writeln!(xml, "    <producer id=\"background\" resource=\"color:black\">").unwrap();
        writeln!(xml, "      <property name=\"mlt_service\">color</property>").unwrap();
        writeln!(xml, "      <property name=\"length\">9999999</property>").unwrap();
        writeln!(xml, "      <property name=\"width\">{}</property>", width).unwrap();
        writeln!(xml, "      <property name=\"height\">{}</property>", height).unwrap();
        writeln!(xml, "    </producer>").unwrap();
        writeln!(xml, "    <entry producer=\"background\" in=\"0\" out=\"9999999\"/>").unwrap();
    }
    
    // Add audio producer
    writeln!(xml, "    <producer id=\"audio\" resource=\"{}\">",
        escape_xml(audio_path.to_str().unwrap())).unwrap();
    writeln!(xml, "      <property name=\"mlt_service\">avformat</property>").unwrap();
    writeln!(xml, "    </producer>").unwrap();
    writeln!(xml, "    <entry producer=\"audio\" in=\"0\" out=\"9999999\"/>").unwrap();
    
    writeln!(xml, "  </playlist>").unwrap();
    
    // Create track for subtitles with word-by-word highlighting
    writeln!(xml, "  <playlist id=\"subtitles\">").unwrap();
    
    for (idx, word_timing) in word_timings.iter().enumerate() {
        let start_sec = word_timing.start_ms as f64 / 1000.0;
        let end_sec = word_timing.end_ms as f64 / 1000.0;
        let duration = end_sec - start_sec;
        
        // Build text with previous words (normal) and current word (highlighted)
        let previous_words: Vec<String> = word_timings.iter()
            .take(idx)
            .map(|w| w.word.clone())
            .collect();
        let previous_text = previous_words.join(" ");
        
        // Create display text: previous words in white, current word highlighted in cyan
        let display_text = if previous_text.is_empty() {
            format!("<span foreground=\"#00FFFF\" weight=\"bold\">{}</span>", word_timing.word)
        } else {
            format!("{} <span foreground=\"#00FFFF\" weight=\"bold\">{}</span>", 
                previous_text, word_timing.word)
        };
        
        writeln!(xml, "    <producer id=\"word_{}_{}\" resource=\"text:{}\">",
            word_timing.start_ms,
            word_timing.end_ms,
            escape_xml(&display_text)).unwrap();
        writeln!(xml, "      <property name=\"mlt_service\">pango</property>").unwrap();
        writeln!(xml, "      <property name=\"family\">Arial</property>").unwrap();
        writeln!(xml, "      <property name=\"size\">{}</property>", (height as f32 * 0.08) as u32).unwrap();
        writeln!(xml, "      <property name=\"weight\">bold</property>").unwrap();
        writeln!(xml, "      <property name=\"fgcolour\">#FFFFFF</property>").unwrap(); // Default white
        writeln!(xml, "      <property name=\"bgcolour\">#00000080</property>").unwrap(); // Semi-transparent black background
        writeln!(xml, "      <property name=\"align\">center</property>").unwrap();
        writeln!(xml, "      <property name=\"valign\">bottom</property>").unwrap();
        writeln!(xml, "      <property name=\"pad\">10</property>").unwrap();
        writeln!(xml, "      <property name=\"out\">{}</property>", (duration * 25.0) as u64).unwrap(); // 25fps
        writeln!(xml, "    </producer>").unwrap();
        
        // Add entry with timing
        writeln!(xml, "    <entry producer=\"word_{}_{}\" in=\"0\" out=\"{}\"/>",
            word_timing.start_ms,
            word_timing.end_ms,
            (duration * 25.0) as u64).unwrap();
    }
    
    writeln!(xml, "  </playlist>").unwrap();
    
    // Create multitrack composition
    writeln!(xml, "  <tractor id=\"main\" title=\"Video with Subtitles\">").unwrap();
    writeln!(xml, "    <property name=\"width\">{}</property>", width).unwrap();
    writeln!(xml, "    <property name=\"height\">{}</property>", height).unwrap();
    writeln!(xml, "    <property name=\"aspect_ratio\">1</property>").unwrap();
    writeln!(xml, "    <property name=\"frame_rate_num\">25</property>").unwrap();
    writeln!(xml, "    <property name=\"frame_rate_den\">1</property>").unwrap();
    
    // Track 0: Main video/audio
    writeln!(xml, "    <track producer=\"main\"/>").unwrap();
    
    // Track 1: Subtitles overlay
    writeln!(xml, "    <track producer=\"subtitles\"/>").unwrap();
    
    writeln!(xml, "  </tractor>").unwrap();
    
    writeln!(xml, "</mlt>").unwrap();
    
    Ok(xml)
}

/// Escape XML special characters
fn escape_xml(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
}

/// Escape text for Pango markup (preserve markup tags but escape content)
fn escape_for_pango(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}

/// Render video using MLT XML script
pub async fn render_mlt_video(
    mlt_xml_path: &Path,
    output_path: &Path,
    format: VideoFormat,
    mut logs: Signal<Vec<LogEntry>>,
    mut progress: Signal<u8>,
) -> Result<PathBuf, String> {
    logs.write().push(LogEntry {
        message: format!("Rendering video with MLT (format: {})...", format.as_str()),
        level: LogLevel::Info,
    });
    
    // Determine output format and codec
    let (video_codec, audio_codec, container) = match format {
        VideoFormat::Mp4 => ("libx264", "aac", "mp4"),
        VideoFormat::Mov => ("libx264", "aac", "mov"),
        VideoFormat::Webm => ("libvpx-vp9", "libopus", "webm"),
    };
    
    // Build melt command - load MLT XML file
    let mlt_path_str = mlt_xml_path.to_str().unwrap().to_string();
    let output_path_str = output_path.to_str().unwrap().to_string();
    
    // Run melt in blocking task
    let result = tokio::task::spawn_blocking(move || {
        use std::process::Command;
        
        // Use melt to render the XML file
        let output = Command::new("melt")
            .arg(&mlt_path_str)
            .arg("-consumer")
            .arg(format!("avformat:{}", output_path_str))
            .arg(format!("vcodec={}", video_codec))
            .arg(format!("acodec={}", audio_codec))
            .arg(format!("container={}", container))
            .arg("real_time=-1") // Disable real-time rendering for faster processing
            .output()
            .map_err(|e| format!("Failed to run melt: {}", e))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            return Err(format!("MLT render failed. Stderr: {} Stdout: {}", stderr, stdout));
        }
        
        Ok(())
    }).await
    .map_err(|e| format!("MLT render task failed: {}", e))?;
    
    result?;
    
    *progress.write() = 100;
    
    logs.write().push(LogEntry {
        message: format!("Video rendered successfully: {}", output_path.display()),
        level: LogLevel::Info,
    });
    
    Ok(output_path.to_path_buf())
}

/// Merge audio with video and add word-highlighted subtitles using MLT
pub async fn merge_audio_and_subtitles(
    video_path: Option<&Path>, // Optional video background (can be None for audio-only)
    audio_path: &Path,
    subtitle_text: &str,
    output_path: &Path,
    width: u32,
    height: u32,
    format: VideoFormat,
    mut logs: Signal<Vec<LogEntry>>,
    mut progress: Signal<u8>,
) -> Result<PathBuf, String> {
    logs.write().push(LogEntry {
        message: "Generating word-level subtitle timing...".to_string(),
        level: LogLevel::Info,
    });
    
    // Get audio duration
    let audio_duration_ms = get_audio_duration_ms(audio_path)?;
    
    // Generate word-level timing
    let word_timings = generate_word_timing(subtitle_text, audio_duration_ms, 150.0);
    
    logs.write().push(LogEntry {
        message: format!("Generated {} word-level subtitle entries", word_timings.len()),
        level: LogLevel::Info,
    });
    
    *progress.write() = 20;
    
    // Generate MLT XML script
    logs.write().push(LogEntry {
        message: "Generating MLT XML script...".to_string(),
        level: LogLevel::Info,
    });
    
    let mlt_xml = generate_mlt_xml(
        video_path,
        audio_path,
        subtitle_text,
        &word_timings,
        output_path,
        width,
        height,
        format,
    )?;
    
    *progress.write() = 40;
    
    // Save MLT XML to temporary file
    let mlt_xml_path = output_path.with_extension("mlt");
    fs::write(&mlt_xml_path, mlt_xml)
        .map_err(|e| format!("Failed to write MLT XML: {}", e))?;
    
    logs.write().push(LogEntry {
        message: format!("MLT XML script saved: {}", mlt_xml_path.display()),
        level: LogLevel::Info,
    });
    
    *progress.write() = 50;
    
    // Render video using MLT
    let final_video = render_mlt_video(&mlt_xml_path, output_path, format, logs, progress).await?;
    
    // Clean up temporary MLT XML file
    let _ = fs::remove_file(&mlt_xml_path);
    
    Ok(final_video)
}

