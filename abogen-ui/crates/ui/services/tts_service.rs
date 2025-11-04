use crate::state::{AppState, LogEntry, LogLevel, QueuedItem, SaveLocation};
use dioxus::prelude::*;
use std::path::PathBuf;
use voxweave::{
    tts::{KokoroEngine, SpeechEngine, VoiceProfile, VoiceEngine},
    subtitle::{format_srt, generate_subtitles},
    text::clean_text,
    queue::SubtitleGranularity,
};
use std::fs;

#[cfg(feature = "coqui-tts")]
use voxweave::coqui_tts::CoquiEngine;

#[cfg(feature = "video-generation")]
use voxweave::video::VideoGenerationServiceExt;

/// Determine output directory based on save location setting
fn get_output_dir(save_location: &SaveLocation, _file_path: &str) -> PathBuf {
    match save_location {
        SaveLocation::Desktop => {
            // Use Desktop directory
            directories::UserDirs::new()
                .and_then(|dirs| dirs.desktop_dir().map(|p| p.to_path_buf()))
                .unwrap_or_else(|| PathBuf::from("."))
        }
        SaveLocation::Custom(path) => PathBuf::from(path),
    }
}

/// Convert subtitle format enum to granularity
fn subtitle_mode_to_granularity(mode: crate::state::SubtitleMode) -> SubtitleGranularity {
    match mode {
        crate::state::SubtitleMode::Sentence => SubtitleGranularity::Sentence,
        crate::state::SubtitleMode::Paragraph => SubtitleGranularity::Sentence, // Use sentence for paragraphs too
        crate::state::SubtitleMode::None => SubtitleGranularity::Disabled,
    }
}

/// Convert voice ID to VoiceProfile
fn get_voice_profile(voice_id: &str) -> Option<VoiceProfile> {
    // Check if it's a Coqui voice (starts with "coqui-")
    if voice_id.starts_with("coqui-") {
        let lang = voice_id.split('-').nth(1).unwrap_or("en");
        return Some(VoiceProfile::coqui(voice_id, &format!("Coqui {}", voice_id), lang));
    }
    
    // Check if it's a Coqui clone voice (starts with "clone-")
    if voice_id.starts_with("clone-") {
        // Clone voice path should be stored separately and passed in
        // For now, return a placeholder that will need the clone path
        return Some(VoiceProfile::coqui_clone(voice_id, "Cloned Voice", "en", ""));
    }
    
    // Otherwise, assume Kokoro voice
    // Get language code from voice ID (first character)
    let lang_code = voice_id.chars().next()?;
    let lang_name = match lang_code {
        'a' => "a",
        'b' => "b",
        'e' => "e",
        'f' => "f",
        'h' => "h",
        'i' => "i",
        'j' => "j",
        'p' => "p",
        'z' => "z",
        _ => return None,
    };
    
    Some(VoiceProfile::kokoro(voice_id, &format!("Kokoro {}", voice_id), lang_name))
}

/// Convert audio format (WAV to requested format using ffmpeg if needed)
async fn convert_audio_format(
    input_path: &PathBuf,
    output_format: &crate::state::VoiceFormat,
    logs: &mut Signal<Vec<LogEntry>>,
) -> Result<PathBuf, String> {
    match output_format {
        crate::state::VoiceFormat::Wav => Ok(input_path.clone()),
        crate::state::VoiceFormat::Mp3 => {
            let output_path = input_path.with_extension("mp3");
            logs.write().push(LogEntry {
                message: format!("Converting WAV to MP3..."),
                level: LogLevel::Info,
            });
            
            #[cfg(not(target_arch = "wasm32"))]
            {
                let output = std::process::Command::new("ffmpeg")
                    .args(&["-i", input_path.to_str().unwrap(), "-y", output_path.to_str().unwrap()])
                    .output()
                    .map_err(|e| format!("Failed to run ffmpeg: {}", e))?;
                
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("ffmpeg conversion failed: {}", stderr));
                }
                
                // Remove original WAV file
                let _ = fs::remove_file(input_path);
                Ok(output_path)
            }
            
            #[cfg(target_arch = "wasm32")]
            {
                // On web, we can't use ffmpeg, so keep WAV
                logs.write().push(LogEntry {
                    message: "MP3 conversion not available on web platform, keeping WAV".to_string(),
                    level: LogLevel::Notice,
                });
                Ok(input_path.clone())
            }
        }
        crate::state::VoiceFormat::Flac => {
            let output_path = input_path.with_extension("flac");
            logs.write().push(LogEntry {
                message: format!("Converting WAV to FLAC..."),
                level: LogLevel::Info,
            });
            
            #[cfg(not(target_arch = "wasm32"))]
            {
                let output = std::process::Command::new("ffmpeg")
                    .args(&["-i", input_path.to_str().unwrap(), "-y", output_path.to_str().unwrap()])
                    .output()
                    .map_err(|e| format!("Failed to run ffmpeg: {}", e))?;
                
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("ffmpeg conversion failed: {}", stderr));
                }
                
                // Remove original WAV file
                let _ = fs::remove_file(input_path);
                Ok(output_path)
            }
            
            #[cfg(target_arch = "wasm32")]
            {
                // On web, we can't use ffmpeg, so keep WAV
                logs.write().push(LogEntry {
                    message: "FLAC conversion not available on web platform, keeping WAV".to_string(),
                    level: LogLevel::Notice,
                });
                Ok(input_path.clone())
            }
        }
    }
}

/// Format subtitles in requested format
fn format_subtitles(
    entries: &[voxweave::subtitle::SubtitleEntry],
    format: &crate::state::SubtitleFormat,
) -> String {
    match format {
        crate::state::SubtitleFormat::Srt => format_srt(entries),
        crate::state::SubtitleFormat::Ass => {
            // Convert SRT format to ASS format (simplified)
            format_ass(entries)
        }
        crate::state::SubtitleFormat::Vtt => {
            // Convert to WebVTT format
            format_vtt(entries)
        }
    }
}

/// Format subtitles as ASS (Advanced SubStation Alpha)
fn format_ass(entries: &[voxweave::subtitle::SubtitleEntry]) -> String {
    let mut output = String::from("[Script Info]\nTitle: Generated Subtitles\n\n[V4+ Styles]\nFormat: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding\nStyle: Default,Arial,20,&H00FFFFFF,&H000000FF,&H00000000,&H80000000,0,0,0,0,100,100,0,0,1,2,0,2,10,10,10,1\n\n[Events]\nFormat: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text\n");
    
    for entry in entries {
        let start = format_timestamp_ass(entry.start_ms);
        let end = format_timestamp_ass(entry.end_ms);
        output.push_str(&format!("Dialogue: 0,{},{},Default,,0,0,0,,{}\n", start, end, entry.text));
    }
    
    output
}

/// Format timestamp for ASS format
fn format_timestamp_ass(ms: u64) -> String {
    let hours = ms / 3_600_000;
    let minutes = (ms % 3_600_000) / 60_000;
    let seconds = (ms % 60_000) / 1_000;
    let centiseconds = (ms % 1_000) / 10;
    format!("{}:{:02}:{:02}.{:02}", hours, minutes, seconds, centiseconds)
}

/// Format subtitles as WebVTT
fn format_vtt(entries: &[voxweave::subtitle::SubtitleEntry]) -> String {
    let mut output = String::from("WEBVTT\n\n");
    
    for entry in entries {
        let start = format_timestamp_vtt(entry.start_ms);
        let end = format_timestamp_vtt(entry.end_ms);
        output.push_str(&format!("{}\n{} --> {}\n{}\n\n", entry.index, start, end, entry.text));
    }
    
    output
}

/// Format timestamp for WebVTT format
fn format_timestamp_vtt(ms: u64) -> String {
    let hours = ms / 3_600_000;
    let minutes = (ms % 3_600_000) / 60_000;
    let seconds = (ms % 60_000) / 1_000;
    let millis = ms % 1_000;
    format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis)
}

/// Process queue with actual TTS pipeline
pub async fn process_queue(
    state: AppState,
    items: Vec<QueuedItem>,
) -> Result<(), String> {
    
    let mut progress = state.progress;
    let mut logs = state.logs;
    let total_items = items.len();
    
    // Engines are initialized as needed per voice in the loop below
    
    for (idx, item) in items.iter().enumerate() {
        // Yield periodically to keep UI responsive
        tokio::task::yield_now().await;
        
        // Check for cancellation
        if state.cancel_token.read().is_some() {
            logs.write().push(LogEntry {
                message: "Processing cancelled by user".to_string(),
                level: LogLevel::Notice,
            });
            return Err("Cancelled".to_string());
        }

        // Add initial log
        logs.write().push(LogEntry {
            message: format!("Processing item {}/{}: {}", idx + 1, total_items, item.file.name),
            level: LogLevel::Info,
        });

        // Determine output directory
        let output_dir = get_output_dir(&item.save_location, &item.file.path);
        let source_path = PathBuf::from(&item.file.path);
        
        // Check if source file exists
        if !source_path.exists() && !item.file.path.starts_with("web-file://") {
            logs.write().push(LogEntry {
                message: format!("Source file not found: {}", item.file.path),
                level: LogLevel::Error,
            });
            // Update progress before continuing to next item
            *progress.write() = ((idx + 1) * 100 / total_items.max(1)).min(100) as u8;
            continue;
        }

        // Read source file
        logs.write().push(LogEntry {
            message: "Reading source file...".to_string(),
            level: LogLevel::Info,
        });
        
        let text = if item.file.path.starts_with("web-file://") {
            // Web file - this is a placeholder, would need actual file content
            logs.write().push(LogEntry {
                message: "Web file processing not fully implemented".to_string(),
                level: LogLevel::Notice,
            });
            continue;
        } else {
            fs::read_to_string(&source_path)
                .map_err(|e| format!("Failed to read source file: {}", e))?
        };

        // Clean text
        let cleaned = clean_text(&text, item.replace_newlines);
        
        // Get base name for output files
        let base_name = source_path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("output");
        
        // Get voice profile
        let voice = get_voice_profile(&item.voice)
            .ok_or_else(|| format!("Invalid voice: {}", item.voice))?;
        
        // Note: For voice cloning, the clone_audio_path would need to be
        // passed from the UI state and set in the voice.command field
        
        let goto_next_item = false;

        // Create output directory
        fs::create_dir_all(&output_dir)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;

        // Generate audio
        logs.write().push(LogEntry {
            message: "Generating audio with Kokoro TTS...".to_string(),
            level: LogLevel::Info,
        });
        
        // Yield to keep UI responsive
        tokio::task::yield_now().await;
        
        let audio_path_wav = output_dir.join(format!("{}.wav", base_name));
        
        // Run blocking TTS synthesis in a separate thread to avoid freezing UI
        let cleaned_clone = cleaned.clone();
        let voice_clone = voice.clone();
        let audio_path_clone = audio_path_wav.clone();
        let speed = item.speed as f32;
        
        // Determine which engine to use based on voice profile
        let synthesis_result = match voice.engine {
            #[cfg(feature = "coqui-tts")]
            VoiceEngine::Coqui => {
                tokio::task::spawn_blocking(move || -> Result<(), String> {
                    use voxweave::coqui_tts::CoquiEngine;
                    // Create a new engine in the blocking thread
                    let engine = CoquiEngine::default();
                    engine.synthesize_to_file(&cleaned_clone, &voice_clone, speed, &audio_path_clone)
                        .map_err(|e| e.to_string())
                }).await
                .map_err(|e| format!("TTS task join failed: {}", e))?
            }
            VoiceEngine::Kokoro => {
                tokio::task::spawn_blocking(move || -> Result<(), String> {
                    use voxweave::tts::KokoroEngine;
                    // Create a new engine in the blocking thread
                    let engine = KokoroEngine::default();
                    engine.synthesize_to_file(&cleaned_clone, &voice_clone, speed, &audio_path_clone)
                        .map_err(|e| e.to_string())
                }).await
                .map_err(|e| format!("TTS task join failed: {}", e))?
            }
            _ => {
                // Fallback to Kokoro for other engines
                tokio::task::spawn_blocking(move || -> Result<(), String> {
                    use voxweave::tts::KokoroEngine;
                    let engine = KokoroEngine::default();
                    engine.synthesize_to_file(&cleaned_clone, &voice_clone, speed, &audio_path_clone)
                        .map_err(|e| e.to_string())
                }).await
                .map_err(|e| format!("TTS task join failed: {}", e))?
            }
        };
        
        synthesis_result?;
        
        let progress_base = (idx * 100) / total_items.max(1);
        *progress.write() = (progress_base + 40).min(100) as u8;
        
        logs.write().push(LogEntry {
            message: format!("Audio generated: {}", audio_path_wav.display()),
            level: LogLevel::Info,
        });

        // Convert audio format if needed
        #[allow(unused_variables)]
        let audio_path = convert_audio_format(&audio_path_wav, &item.voice_format, &mut logs).await?;
        
        *progress.write() = (progress_base + 50).min(100) as u8;

        // Generate subtitles if enabled
        #[allow(unused_variables)]
        let subtitle_path = if item.subtitle_mode != crate::state::SubtitleMode::None {
            logs.write().push(LogEntry {
                message: "Generating subtitles...".to_string(),
                level: LogLevel::Info,
            });
            
            let granularity = subtitle_mode_to_granularity(item.subtitle_mode);
            let entries = generate_subtitles(&cleaned, granularity, 150.0);
            
            if !entries.is_empty() {
                let subtitle_content = format_subtitles(&entries, &item.subtitle_format);
                let subtitle_path = output_dir.join(format!("{}.{}", base_name, item.subtitle_format.as_str()));
                fs::write(&subtitle_path, subtitle_content)
                    .map_err(|e| format!("Failed to write subtitle file: {}", e))?;
                
                logs.write().push(LogEntry {
                    message: format!("Subtitles generated: {}", subtitle_path.display()),
                    level: LogLevel::Info,
                });
                
                Some(subtitle_path)
            } else {
                None
            }
        } else {
            None
        };

        *progress.write() = (progress_base + 70).min(100) as u8;

        // Video generation (if enabled)
        #[cfg(feature = "video-generation")]
        if item.generate_video {
            logs.write().push(LogEntry {
                message: "Starting video generation...".to_string(),
                level: LogLevel::Info,
            });
            
            // Video generation using Z.AI Vidu 2
            #[cfg(feature = "video-generation")]
            {
                if let Ok(video_service) = voxweave::video::VideoGenerationService::from_env() {
                    let video_config = voxweave::video::VideoConfig {
                        style: item.video_style,
                        resolution: item.video_resolution,
                        format: item.video_format,
                        prompt: item.video_prompt.clone(),
                        image_urls: None,
                    };
                    
                    let mut progress_logs = logs.clone();
                    let mut progress_progress = progress.clone();
                    
                    let progress_cb = move |p: u8| {
                        *progress_progress.write() = p;
                    };
                    
                    let log_cb = move |msg: &str, level: voxweave::queue::LogLevel| {
                        progress_logs.write().push(LogEntry {
                            message: msg.to_string(),
                            level: match level {
                                voxweave::queue::LogLevel::Info => LogLevel::Info,
                                voxweave::queue::LogLevel::Notice => LogLevel::Notice,
                                voxweave::queue::LogLevel::Warning => LogLevel::Notice,
                                voxweave::queue::LogLevel::Error => LogLevel::Error,
                            },
                        });
                    };
                    
                    match video_service.generate_video_from_text(
                        &audio_path,
                        subtitle_path.as_ref().map(|p| p.as_path()),
                        &video_config,
                        Some(progress_cb),
                        Some(log_cb),
                    ).await {
                        Ok(video_path) => {
                            logs.write().push(LogEntry {
                                message: format!("Video generated successfully: {}", video_path.display()),
                                level: LogLevel::Info,
                            });
                        }
                        Err(e) => {
                            logs.write().push(LogEntry {
                                message: format!("Video generation failed: {}", e),
                                level: LogLevel::Error,
                            });
                        }
                    }
                } else {
                    logs.write().push(LogEntry {
                        message: "ZAI_API_KEY not found. Video generation skipped.".to_string(),
                        level: LogLevel::Notice,
                    });
                }
            }
        }
        
        // Skip remaining processing if Z.AI video generation succeeded
        if goto_next_item {
            *progress.write() = ((idx + 1) * 100 / total_items.max(1)).min(100) as u8;
            logs.write().push(LogEntry {
                message: format!("Completed: {}", item.file.name),
                level: LogLevel::Info,
            });
            continue;
        }
        
        *progress.write() = ((idx + 1) * 100 / total_items.max(1)).min(100) as u8;
        
        logs.write().push(LogEntry {
            message: format!("Completed: {}", item.file.name),
            level: LogLevel::Info,
        });
    }

    // Final progress
    *progress.write() = 100;
    logs.write().push(LogEntry {
        message: "All items processed successfully!".to_string(),
        level: LogLevel::Info,
    });

    Ok(())
}

/// Extract subtitle text from SRT/ASS/VTT format (simplified)
fn extract_subtitle_text(subtitle_content: &str) -> String {
    // Simple extraction - get all text from subtitle entries
    // This is a simplified version - for production, should parse properly
    let mut text_parts = Vec::new();
    
    for line in subtitle_content.lines() {
        // Skip timestamp lines and index numbers
        if !line.trim().is_empty()
            && !line.contains("-->")
            && !line.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false)
            && !line.starts_with("WEBVTT")
            && !line.starts_with("[Script Info]")
            && !line.starts_with("[V4+ Styles]")
            && !line.starts_with("[Events]")
            && !line.starts_with("Format:")
            && !line.starts_with("Style:")
            && !line.starts_with("Dialogue:")
        {
            text_parts.push(line.trim());
        }
    }
    
    text_parts.join(" ")
}

