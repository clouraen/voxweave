/// Alibaba Cloud Aliyun Video Generation Service
/// Generates videos using Alibaba's wan2.5-t2v-preview model via Bailian API

use crate::state::{LogEntry, LogLevel, VideoResolution, VideoFormat};
use dioxus::prelude::{Signal, WritableExt};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AliyunVideoRequest {
    pub model: String, // "wan2.5-t2v-preview"
    pub input: AliyunVideoInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<AliyunVideoParameters>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AliyunVideoInput {
    pub prompt: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AliyunVideoParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>, // e.g., "1920x1080"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>, // Duration in seconds
}

#[derive(Deserialize, Debug)]
pub struct AliyunVideoResponse {
    pub id: String,
    pub status: String,
    #[serde(default)]
    pub output: Option<AliyunVideoOutput>,
    #[serde(default)]
    pub error: Option<AliyunError>,
}

#[derive(Deserialize, Debug)]
pub struct AliyunVideoOutput {
    pub video_url: Option<String>,
    pub task_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct AliyunError {
    pub code: String,
    pub message: String,
}

/// Alibaba Cloud Aliyun Video Generation Service
pub struct AliyunVideoService {
    api_key: String,
    base_url: String,
}

impl AliyunVideoService {
    /// Create a new Aliyun video service
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://bailian.cn-beijing.aliyuncs.com/api/v1/chat/completions".to_string(),
        }
    }

    /// Get API key from environment ALIBABA_API_KEY or return None
    pub fn from_env() -> Option<Self> {
        std::env::var("ALIBABA_API_KEY")
            .ok()
            .map(Self::new)
    }

    /// Generate video using Alibaba wan2.5-t2v-preview model
    pub async fn generate_video(
        &self,
        prompt: &str,
        resolution: VideoResolution,
        output_dir: &Path,
        mut logs: Signal<Vec<LogEntry>>,
        mut progress: Signal<u8>,
    ) -> Result<PathBuf, String> {
        use reqwest::Client;

        let client = Client::new();

        logs.write().push(LogEntry {
            message: "Starting Alibaba wan2.5-t2v-preview video generation...".to_string(),
            level: LogLevel::Info,
        });

        *progress.write() = 10;

        // Convert resolution to string format
        let resolution_str = match resolution {
            VideoResolution::P720 => "1280x720",
            VideoResolution::P1080 => "1920x1080",
            VideoResolution::P4K => "3840x2160",
        };

        // Build request
        let request = AliyunVideoRequest {
            model: "wan2.5-t2v-preview".to_string(),
            input: AliyunVideoInput {
                prompt: prompt.to_string(),
            },
            parameters: Some(AliyunVideoParameters {
                resolution: Some(resolution_str.to_string()),
                duration: None, // Let the model decide based on the prompt
            }),
        };

        logs.write().push(LogEntry {
            message: "Sending video generation request to Alibaba Cloud...".to_string(),
            level: LogLevel::Info,
        });

        // Send request
        let response = client
            .post(&self.base_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Failed to send request to Alibaba Cloud: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("Alibaba Cloud API error ({}): {}", status, text));
        }

        let video_response: AliyunVideoResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse Alibaba Cloud response: {}", e))?;

        // Check for errors in response
        if let Some(error) = video_response.error {
            return Err(format!("Alibaba Cloud error: {} - {}", error.code, error.message));
        }

        logs.write().push(LogEntry {
            message: format!("Video generation job created: ID={}", video_response.id),
            level: LogLevel::Info,
        });

        *progress.write() = 30;

        // Poll for completion
        let video_url = self.poll_video_status(&video_response.id, progress, logs).await?;

        *progress.write() = 80;

        // Download video
        logs.write().push(LogEntry {
            message: "Downloading generated video...".to_string(),
            level: LogLevel::Info,
        });

        let video_bytes = client
            .get(&video_url)
            .send()
            .await
            .map_err(|e| format!("Failed to download video: {}", e))?
            .bytes()
            .await
            .map_err(|e| format!("Failed to read video bytes: {}", e))?;

        // Save video to output directory
        let output_path = output_dir.join("aliyun_video.mp4");

        tokio::fs::write(&output_path, &video_bytes)
            .await
            .map_err(|e| format!("Failed to write video file: {}", e))?;

        logs.write().push(LogEntry {
            message: format!("Video downloaded: {}", output_path.display()),
            level: LogLevel::Info,
        });

        *progress.write() = 90;

        Ok(output_path)
    }

    /// Poll for video generation status
    async fn poll_video_status(
        &self,
        task_id: &str,
        mut progress: Signal<u8>,
        mut logs: Signal<Vec<LogEntry>>,
    ) -> Result<String, String> {
        use reqwest::Client;
        use std::time::Duration;

        let client = Client::new();
        let status_url = format!("{}/status/{}", self.base_url.replace("/chat/completions", ""), task_id);
        let max_attempts = 120; // 10 minutes max (5 second intervals)
        let mut attempts = 0;

        loop {
            if attempts >= max_attempts {
                return Err("Video generation timeout - exceeded maximum wait time".to_string());
            }

            tokio::time::sleep(Duration::from_secs(5)).await;

            let response = client
                .get(&status_url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .send()
                .await
                .map_err(|e| format!("Failed to check status: {}", e))?;

            if !response.status().is_success() {
                attempts += 1;
                continue;
            }

            let status_response: AliyunVideoResponse = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse status response: {}", e))?;

            // Update progress based on status
            let progress_val = match status_response.status.as_str() {
                "pending" | "queued" => 30,
                "processing" => 30 + ((attempts * 50) / max_attempts) as u8,
                "completed" | "success" => 80,
                _ => 30,
            };
            *progress.write() = progress_val;

            match status_response.status.as_str() {
                "completed" | "success" => {
                    if let Some(output) = status_response.output {
                        if let Some(url) = output.video_url {
                            logs.write().push(LogEntry {
                                message: "Video generation completed!".to_string(),
                                level: LogLevel::Info,
                            });
                            return Ok(url);
                        }
                    }
                    return Err("Video completed but no URL provided".to_string());
                }
                "failed" | "error" => {
                    let error_msg = status_response.error
                        .map(|e| format!("{} - {}", e.code, e.message))
                        .unwrap_or_else(|| "Unknown error".to_string());
                    return Err(format!("Video generation failed: {}", error_msg));
                }
                "processing" | "pending" | "queued" => {
                    if attempts % 12 == 0 {
                        // Log every minute
                        logs.write().push(LogEntry {
                            message: format!("Video generation in progress... (attempt {}/{})", attempts + 1, max_attempts),
                            level: LogLevel::Info,
                        });
                    }
                    attempts += 1;
                    continue;
                }
                _ => {
                    attempts += 1;
                    continue;
                }
            }
        }
    }
}

/// Generate an Alibaba Aliyun video with composition
pub async fn generate_aliyun_video_with_composition(
    prompt: &str,
    resolution: VideoResolution,
    audio_path: &Path,
    subtitle_path: Option<&Path>,
    subtitle_text: Option<&str>,
    output_dir: &Path,
    mut logs: Signal<Vec<LogEntry>>,
    mut progress: Signal<u8>,
) -> Result<PathBuf, String> {
    let service = AliyunVideoService::from_env()
        .ok_or_else(|| "ALIBABA_API_KEY not found in environment".to_string())?;

    // Generate video with Alibaba
    let aliyun_video_path = service.generate_video(prompt, resolution, output_dir, logs, progress).await?;

    *progress.write() = 95;

    // Step 4: Compose final video with audio and subtitles using MLT
    if subtitle_path.is_some() || subtitle_text.is_some() {
        logs.write().push(LogEntry {
            message: "Composing final video with audio and subtitles using MLT...".to_string(),
            level: LogLevel::Info,
        });

        let (width, height) = match resolution {
            VideoResolution::P720 => (1280, 720),
            VideoResolution::P1080 => (1920, 1080),
            VideoResolution::P4K => (3840, 2160),
        };

        let subtitle_text_str = subtitle_text
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                subtitle_path
                    .and_then(|p| std::fs::read_to_string(p).ok())
                    .unwrap_or_default()
            });

        let final_output = output_dir.join("aliyun_video_final.mp4");

        use crate::services::mlt_video::merge_audio_and_subtitles;
        let final_video = merge_audio_and_subtitles(
            Some(&aliyun_video_path),
            audio_path,
            &subtitle_text_str,
            &final_output,
            width,
            height,
            VideoFormat::Mp4,
            logs,
            progress,
        ).await?;

        logs.write().push(LogEntry {
            message: format!("Final video composed: {}", final_video.display()),
            level: LogLevel::Info,
        });

        Ok(final_video)
    } else {
        // If no subtitles, just return the Alibaba video
        Ok(aliyun_video_path)
    }
}
