/// Z.AI Video Generation Service
/// Generates videos using Z.AI API with TTS narration, background music, and AI visuals

use crate::state::{LogEntry, LogLevel, VideoStyle, VideoResolution, VideoFormat};
use dioxus::prelude::{Signal, WritableExt};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZaiVideoRequest {
    pub text: String,
    pub voice: String,
    pub style: String, // "biotech", "cyberpunk", "educational", etc.
    pub subtitles: bool,
    pub resolution: String, // "1080p", "4k"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ZaiVideoResponse {
    pub video_url: String,
    pub status: String,
    #[serde(default)]
    pub job_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ZaiVideoStatusResponse {
    pub status: String,
    pub video_url: Option<String>,
    pub progress: Option<u8>,
}

/// Z.AI Video Generation Service
pub struct ZaiVideoService {
    api_key: String,
    base_url: String,
}

impl ZaiVideoService {
    /// Create a new Z.AI video service
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://api.z.ai/v1".to_string(),
        }
    }

    /// Get API key from environment or return None
    pub fn from_env() -> Option<Self> {
        std::env::var("ZAI_API_KEY")
            .ok()
            .map(Self::new)
    }

    /// Generate video using Z.AI API
    pub async fn generate_video(
        &self,
        req: ZaiVideoRequest,
        output_dir: &Path,
        mut logs: Signal<Vec<LogEntry>>,
        mut progress: Signal<u8>,
    ) -> Result<PathBuf, String> {
        use reqwest::Client;

        let client = Client::new();

        logs.write().push(LogEntry {
            message: format!("Starting Z.AI video generation with style: {}", req.style),
            level: LogLevel::Info,
        });

        *progress.write() = 10;

        // Step 1: Create video generation job
        logs.write().push(LogEntry {
            message: "Creating video generation job...".to_string(),
            level: LogLevel::Info,
        });

        let create_url = format!("{}/video/generate", self.base_url);
        let response = client
            .post(&create_url)
            .bearer_auth(&self.api_key)
            .json(&req)
            .send()
            .await
            .map_err(|e| format!("Failed to send request to Z.AI: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("Z.AI API error ({}): {}", status, text));
        }

        let video_response: ZaiVideoResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse Z.AI response: {}", e))?;

        logs.write().push(LogEntry {
            message: format!("Video generation job created: {:?}", video_response.status),
            level: LogLevel::Info,
        });

        *progress.write() = 30;

        // Step 2: Poll for completion if job_id is provided
        let video_url = if let Some(job_id) = video_response.job_id {
            logs.write().push(LogEntry {
                message: format!("Polling for video status (Job ID: {})...", job_id),
                level: LogLevel::Info,
            });

            self.poll_video_status(&job_id, progress, logs).await?
        } else {
            // If no job_id, assume video_url is directly available
            video_response.video_url
        };

        *progress.write() = 80;

        // Step 3: Download video
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

        // Use provided output directory (already created by caller)
        let output_path = output_dir.join("zai_video.mp4");

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
        job_id: &str,
        mut progress: Signal<u8>,
        mut logs: Signal<Vec<LogEntry>>,
    ) -> Result<String, String> {
        use reqwest::Client;
        use std::time::Duration;

        let client = Client::new();
        let status_url = format!("{}/video/status/{}", self.base_url, job_id);
        let max_attempts = 120; // 10 minutes max (5 second intervals)
        let mut attempts = 0;

        loop {
            if attempts >= max_attempts {
                return Err("Video generation timeout - exceeded maximum wait time".to_string());
            }

            tokio::time::sleep(Duration::from_secs(5)).await;

            let response = client
                .get(&status_url)
                .bearer_auth(&self.api_key)
                .send()
                .await
                .map_err(|e| format!("Failed to check status: {}", e))?;

            if !response.status().is_success() {
                let status = response.status();
                let text = response.text().await.unwrap_or_default();
                return Err(format!("Status check error ({}): {}", status, text));
            }

            let status_response: ZaiVideoStatusResponse = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse status response: {}", e))?;

            // Update progress if provided
            if let Some(prog) = status_response.progress {
                *progress.write() = (30 + (prog as u32 * 50 / 100)) as u8;
            }

            match status_response.status.as_str() {
                "completed" | "success" => {
                    if let Some(url) = status_response.video_url {
                        logs.write().push(LogEntry {
                            message: "Video generation completed!".to_string(),
                            level: LogLevel::Info,
                        });
                        return Ok(url);
                    } else {
                        return Err("Video completed but no URL provided".to_string());
                    }
                }
                "failed" | "error" => {
                    return Err("Video generation failed on Z.AI side".to_string());
                }
                "processing" | "pending" => {
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

/// Generate a Z.AI video with composition using MLT
pub async fn generate_zai_video_with_composition(
    text: &str,
    voice: &str,
    style: VideoStyle,
    resolution: VideoResolution,
    audio_path: &Path,
    subtitle_path: Option<&Path>,
    subtitle_text: Option<&str>,
    output_dir: &Path,
    mut logs: Signal<Vec<LogEntry>>,
    mut progress: Signal<u8>,
) -> Result<PathBuf, String> {
    let service = ZaiVideoService::from_env()
        .ok_or_else(|| "ZAI_API_KEY not found in environment".to_string())?;

    // Build Z.AI request
    let style_str = match style {
        VideoStyle::Realistic => "realistic",
        VideoStyle::Anime => "anime",
        VideoStyle::ThreeD => "3d",
        VideoStyle::Cinematic => "cinematic",
        VideoStyle::Biotech => "biotech",
        VideoStyle::Cyberpunk => "cyberpunk",
        VideoStyle::Educational => "educational",
    };

    let resolution_str = resolution.as_str();

    // Create enhanced prompt for Z.AI
    let prompt = match style_str {
        "biotech" => Some("Narrate this text with cinematic biotech visuals. Use neon green and blue DNA particles, transparent data layers, and soft laboratory lighting. Sync subtitles word-by-word and overlay holographic effects.".to_string()),
        "cyberpunk" => Some("Narrate this text with cyberpunk neon aesthetics. Use vibrant magenta and cyan lights, futuristic cityscapes, and digital rain effects. Sync subtitles with glitch animations.".to_string()),
        "educational" => Some("Narrate this text with clean educational visuals. Use minimalistic graphics, clear typography, and professional color schemes. Sync subtitles smoothly.".to_string()),
        _ => None,
    };

    let zai_request = ZaiVideoRequest {
        text: text.to_string(),
        voice: voice.to_string(),
        style: style_str.to_string(),
        subtitles: subtitle_path.is_some(),
        resolution: resolution_str.to_string(),
        prompt,
    };

    // Generate video with Z.AI
    let zai_video_path = service.generate_video(zai_request, output_dir, logs, progress).await?;

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

        let final_output = output_dir.join("zai_video_final.mp4");

        use crate::services::mlt_video::merge_audio_and_subtitles;
        let final_video = merge_audio_and_subtitles(
            Some(&zai_video_path),
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
        // If no subtitles, just return the Z.AI video (audio already merged by Z.AI)
        Ok(zai_video_path)
    }
}

