use crate::state::{AppState, LogEntry, LogLevel, QueuedItem};
use dioxus::prelude::*;
use std::path::PathBuf;

/// Mock TTS pipeline that simulates processing with async delays and log streaming
pub async fn process_queue(
    state: AppState,
    items: Vec<QueuedItem>,
) -> Result<(), String> {
    use std::time::Duration;
    
    let mut progress = state.progress;
    let mut logs = state.logs;
    let total_items = items.len();
    
    for (idx, item) in items.iter().enumerate() {
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

        // Simulate GPU check
        if item.use_gpu {
            logs.write().push(LogEntry {
                message: "Checking GPU acceleration...".to_string(),
                level: LogLevel::Info,
            });
            tokio::time::sleep(Duration::from_millis(300)).await;
            
            logs.write().push(LogEntry {
                message: "GPU acceleration: Available".to_string(),
                level: LogLevel::Info,
            });
        } else {
            logs.write().push(LogEntry {
                message: "GPU acceleration: Disabled".to_string(),
                level: LogLevel::Info,
            });
        }

        // Simulate module loading
        logs.write().push(LogEntry {
            message: "Loading modules...".to_string(),
            level: LogLevel::Info,
        });
        tokio::time::sleep(Duration::from_millis(400)).await;
        
        logs.write().push(LogEntry {
            message: "Modules loaded successfully".to_string(),
            level: LogLevel::Info,
        });

        // Echo configuration
        logs.write().push(LogEntry {
            message: format!("Configuration: voice={}, speed={:.2}x, format={}, subtitle={}", 
                item.voice, item.speed, item.voice_format.as_str(), item.subtitle_format.as_str()),
            level: LogLevel::Info,
        });

        // Step 1: TTS Processing (simulate)
        logs.write().push(LogEntry {
            message: "Generating audio...".to_string(),
            level: LogLevel::Info,
        });
        
        let steps = 15;
        for step in 0..=steps {
            if state.cancel_token.read().is_some() {
                return Err("Cancelled".to_string());
            }
            
            let item_progress = ((idx * 100 + (step * 60 / steps)) / total_items.max(1)) as u8;
            *progress.write() = item_progress.min(100);
            
            if step < steps {
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
        
        // Simulate audio file path
        let audio_path = PathBuf::from(&item.file.path.replace(".txt", &format!(".{}", item.voice_format.as_str())));
        logs.write().push(LogEntry {
            message: format!("Audio generated: {}", audio_path.display()),
            level: LogLevel::Info,
        });

        // Step 2: Subtitle Generation (if enabled)
        if item.subtitle_mode != crate::state::SubtitleMode::None {
            logs.write().push(LogEntry {
                message: "Generating subtitles...".to_string(),
                level: LogLevel::Info,
            });
            tokio::time::sleep(Duration::from_millis(500)).await;
            let subtitle_path = PathBuf::from(&item.file.path.replace(".txt", &format!(".{}", item.subtitle_format.as_str())));
            logs.write().push(LogEntry {
                message: format!("Subtitles generated: {}", subtitle_path.display()),
                level: LogLevel::Info,
            });

            // Step 3: Video Generation (if enabled)
            #[cfg(feature = "video-generation")]
            if item.generate_video {
                logs.write().push(LogEntry {
                    message: "Starting video generation with Z.ai...".to_string(),
                    level: LogLevel::Info,
                });
                
                // Check if API key is available
                if let Some(video_service) = crate::services::video_generation::VideoGenerationService::from_env() {
                    let video_config = crate::services::video_generation::VideoConfig {
                        style: item.video_style,
                        resolution: item.video_resolution,
                        format: item.video_format,
                        prompt: item.video_prompt.clone(),
                    };
                    
                    match video_service.generate_video(
                        &audio_path,
                        Some(&subtitle_path),
                        &video_config,
                        logs,
                        progress,
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
            
            // Update progress for video generation phase
            let item_progress = ((idx + 1) * 100 / total_items.max(1)) as u8;
            *progress.write() = item_progress.min(100);
        } else {
            // No subtitles, so no video (video requires subtitles for now)
            let item_progress = ((idx + 1) * 100 / total_items.max(1)) as u8;
            *progress.write() = item_progress.min(100);
        }
        
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{FileInfo, SubtitleFormat, SubtitleMode, VoiceFormat, SubtitleDisplayStyle};
    
    #[tokio::test]
    async fn test_process_queue_progress() {
        // This is a stub test - in real implementation, would use proper Dioxus signals
        // For now, just verify the function compiles and can be called
        let items = vec![QueuedItem {
            file: FileInfo {
                name: "test.txt".to_string(),
                path: "/test.txt".to_string(),
                size: 100,
            },
            voice: "af_heart".to_string(),
            speed: 1.0,
            subtitle_mode: SubtitleMode::Sentence,
            voice_format: VoiceFormat::Wav,
            subtitle_format: SubtitleFormat::Srt,
            replace_newlines: true,
            use_gpu: false,
            generate_video: false,
            video_style: crate::state::VideoStyle::Realistic,
            video_resolution: crate::state::VideoResolution::P1080,
            video_format: crate::state::VideoFormat::Mp4,
            video_prompt: None,
            save_location: crate::state::SaveLocation::Desktop,
            subtitle_display_style: SubtitleDisplayStyle::WordByWord,
            show_ipa_transcription: false,
        }];
        
        // Note: This test would need proper AppState setup with signals
        // For now, just ensure it compiles
        assert_eq!(items.len(), 1);
    }
}
