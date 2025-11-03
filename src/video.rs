//! Video generation service for VoxWeave
//! 
//! This module provides video generation functionality using the Z.ai API.
//! It can be used by both the CLI and GUI applications.

use std::path::{Path, PathBuf};
use crate::queue::{VideoStyle, VideoResolution, VideoFormat, LogLevel};

#[cfg(feature = "video-generation")]
use anyhow::{Context, Result};
#[cfg(feature = "video-generation")]
use serde_json::json;

/// Configuration for video generation
#[derive(Debug, Clone)]
pub struct VideoConfig {
    pub style: VideoStyle,
    pub resolution: VideoResolution,
    pub format: VideoFormat,
    pub prompt: Option<String>,
}

impl Default for VideoConfig {
    fn default() -> Self {
        Self {
            style: VideoStyle::Cyberpunk,
            resolution: VideoResolution::P1080,
            format: VideoFormat::Mp4,
            prompt: None,
        }
    }
}

/// Z.ai Video Generation Service
#[cfg(feature = "video-generation")]
pub struct VideoGenerationService {
    api_key: String,
    base_url: String,
}

#[cfg(feature = "video-generation")]
impl VideoGenerationService {
    /// Create a new video generation service with the given API key
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://api.z.ai/v1".to_string(),
        }
    }

    /// Get API key from environment variable ZAI_API_KEY
    pub fn from_env() -> Result<Self> {
        let api_key = std::env::var("ZAI_API_KEY")
            .context("ZAI_API_KEY environment variable not set")?;
        Ok(Self::new(api_key))
    }

    /// Generate video from audio file using Z.ai API
    /// 
    /// # Arguments
    /// * `audio_path` - Path to the audio file
    /// * `subtitle_path` - Optional path to subtitle file (.srt)
    /// * `config` - Video generation configuration
    /// * `progress_callback` - Optional callback for progress updates (0-100)
    /// * `log_callback` - Optional callback for log messages
    pub async fn generate_video<F, G>(
        &self,
        audio_path: &Path,
        subtitle_path: Option<&Path>,
        config: &VideoConfig,
        mut progress_callback: Option<F>,
        mut log_callback: Option<G>,
    ) -> Result<PathBuf>
    where
        F: FnMut(u8),
        G: FnMut(&str, LogLevel),
    {
        // Step 1: Upload audio file
        if let Some(ref mut cb) = log_callback {
            cb("Uploading audio to Z.ai...", LogLevel::Info);
        }
        
        let audio_url = self.upload_file(audio_path).await?;
        
        // Step 2: Create video generation job
        if let Some(ref mut cb) = log_callback {
            cb("Creating video generation job...", LogLevel::Info);
        }
        
        let job_id = self.create_video_job(&audio_url, config).await?;
        
        // Step 3: Poll for completion with progress updates
        if let Some(ref mut cb) = log_callback {
            cb(&format!("Video generation started (Job ID: {})", job_id), LogLevel::Info);
        }
        
        let video_url = self.poll_video_status(&job_id, &mut progress_callback, &mut log_callback).await?;
        
        // Step 4: Download generated video
        if let Some(ref mut cb) = log_callback {
            cb("Downloading generated video...", LogLevel::Info);
        }
        
        let video_path = self.download_video(&video_url, audio_path, config).await?;
        
        // Step 5: Combine with subtitles if needed
        if let Some(subtitle_path) = subtitle_path {
            if let Some(ref mut cb) = log_callback {
                cb("Embedding subtitles into video...", LogLevel::Info);
            }
            
            match self.embed_subtitles(&video_path, subtitle_path, config).await {
                Ok(final_video_path) => {
                    if let Some(ref mut cb) = log_callback {
                        cb(&format!("Video with subtitles ready: {}", final_video_path.display()), LogLevel::Info);
                    }
                    Ok(final_video_path)
                }
                Err(e) => {
                    if let Some(ref mut cb) = log_callback {
                        cb(&format!("Subtitle embedding failed: {}. Using video without subtitles.", e), LogLevel::Notice);
                    }
                    Ok(video_path)
                }
            }
        } else {
            Ok(video_path)
        }
    }

    /// Upload file to Z.ai storage
    async fn upload_file(&self, file_path: &Path) -> Result<String> {
        use reqwest::multipart;
        use tokio::fs;
        
        // Read file data
        let file_data = fs::read(file_path)
            .await
            .context("Failed to read audio file")?;
        
        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("audio.wav");
        
        // Create multipart form with file
        let form = multipart::Form::new()
            .part(
                "file",
                multipart::Part::bytes(file_data)
                    .file_name(file_name.to_string())
                    .mime_str("application/octet-stream")
                    .context("Failed to set MIME type")?,
            );
        
        // Upload file to Z.ai
        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/upload", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await
            .context("Failed to upload file to Z.ai")?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Upload failed with status {}: {}", status, error_text);
        }
        
        // Parse response to get file URL
        let upload_response: serde_json::Value = response
            .json()
            .await
            .context("Failed to parse upload response")?;
        
        upload_response
            .get("url")
            .or_else(|| upload_response.get("file_url"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow::anyhow!("Missing URL in upload response"))
    }

    /// Create video generation job
    async fn create_video_job(
        &self,
        audio_url: &str,
        config: &VideoConfig,
    ) -> Result<String> {
        let mut request_body = json!({
            "audio_url": audio_url,
            "style": config.style.as_str(),
            "resolution": config.resolution.as_str(),
            "model": "video-generation",
        });

        if let Some(ref prompt) = config.prompt {
            request_body["prompt"] = json!(prompt);
        }

        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/video/generate", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .context("Failed to create video job")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("API error {}: {}", status, error_text);
        }

        let job_response: serde_json::Value = response
            .json()
            .await
            .context("Failed to parse job response")?;

        job_response
            .get("job_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow::anyhow!("Missing job_id in response"))
    }

    /// Poll video generation status
    async fn poll_video_status<F, G>(
        &self,
        job_id: &str,
        progress_callback: &mut Option<F>,
        _log_callback: &mut Option<G>,
    ) -> Result<String>
    where
        F: FnMut(u8),
        G: FnMut(&str, LogLevel),
    {
        use std::time::Duration;
        
        let client = reqwest::Client::new();
        let max_attempts = 300; // 5 minutes max at 1s intervals
        let mut attempts = 0;

        loop {
            if attempts >= max_attempts {
                anyhow::bail!("Video generation timeout (5 minutes exceeded)");
            }

            tokio::time::sleep(Duration::from_secs(1)).await;
            attempts += 1;

            let response = client
                .get(format!("{}/video/status/{}", self.base_url, job_id))
                .header("Authorization", format!("Bearer {}", self.api_key))
                .send()
                .await
                .context("Failed to check video status")?;

            if !response.status().is_success() {
                continue;
            }

            let status: serde_json::Value = response
                .json()
                .await
                .context("Failed to parse status response")?;

            let status_str = status
                .get("status")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");

            let progress_val = status
                .get("progress")
                .and_then(|v| v.as_u64())
                .map(|p| p.min(100) as u8)
                .unwrap_or(0);

            if let Some(ref mut cb) = progress_callback {
                cb(progress_val);
            }

            match status_str {
                "completed" => {
                    return status
                        .get("video_url")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .ok_or_else(|| anyhow::anyhow!("Missing video_url in response"));
                }
                "failed" => {
                    let error = status
                        .get("error")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown error");
                    anyhow::bail!("Video generation failed: {}", error);
                }
                "processing" | "queued" => {
                    // Continue polling
                }
                _ => {
                    anyhow::bail!("Unknown status: {}", status_str);
                }
            }
        }
    }

    /// Download generated video
    async fn download_video(
        &self,
        video_url: &str,
        audio_path: &Path,
        config: &VideoConfig,
    ) -> Result<PathBuf> {
        use tokio::fs;
        use tokio::io::AsyncWriteExt;

        let client = reqwest::Client::new();
        
        let response = client
            .get(video_url)
            .send()
            .await
            .context("Failed to download video")?;

        if !response.status().is_success() {
            anyhow::bail!("Download failed with status: {}", response.status());
        }

        let video_data = response
            .bytes()
            .await
            .context("Failed to read video data")?;

        // Determine output path
        let audio_stem = audio_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        
        let output_dir = audio_path.parent().unwrap_or_else(|| Path::new("."));
        let video_filename = format!("{}.{}", audio_stem, config.format.as_str());
        let video_path = output_dir.join(video_filename);

        // Write video file
        let mut file = fs::File::create(&video_path)
            .await
            .context("Failed to create video file")?;

        file.write_all(&video_data)
            .await
            .context("Failed to write video file")?;

        Ok(video_path)
    }

    /// Embed subtitles into video file using ffmpeg
    async fn embed_subtitles(
        &self,
        video_path: &Path,
        subtitle_path: &Path,
        config: &VideoConfig,
    ) -> Result<PathBuf> {
        use std::process::Command;
        use tokio::fs;
        
        // Create output path with _subtitled suffix
        let video_stem = video_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        
        let output_dir = video_path.parent().unwrap_or_else(|| Path::new("."));
        let output_filename = format!("{}_subtitled.{}", video_stem, config.format.as_str());
        let output_path = output_dir.join(output_filename);
        
        // Try to use ffmpeg to embed subtitles
        let ffmpeg_result = Command::new("ffmpeg")
            .arg("-i")
            .arg(video_path)
            .arg("-vf")
            .arg(format!("subtitles={}", subtitle_path.display()))
            .arg("-c:v")
            .arg("libx264")
            .arg("-c:a")
            .arg("copy")
            .arg("-y") // Overwrite output file
            .arg(&output_path)
            .output();
        
        match ffmpeg_result {
            Ok(output) if output.status.success() => {
                Ok(output_path)
            }
            Ok(output) => {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                anyhow::bail!("ffmpeg failed: {}", error_msg)
            }
            Err(_) => {
                // ffmpeg not available, copy video and subtitle as external file
                fs::copy(video_path, &output_path)
                    .await
                    .context("Failed to copy video")?;
                
                // Copy subtitle file to same location
                let subtitle_ext = subtitle_path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("srt");
                let subtitle_output = output_dir.join(format!("{}.{}", video_stem, subtitle_ext));
                fs::copy(subtitle_path, &subtitle_output)
                    .await
                    .context("Failed to copy subtitle")?;
                
                Ok(output_path)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_config_default() {
        let config = VideoConfig::default();
        assert_eq!(config.style.as_str(), "cyberpunk");
        assert_eq!(config.resolution.as_str(), "1080p");
        assert_eq!(config.format.as_str(), "mp4");
        assert!(config.prompt.is_none());
    }

    #[test]
    fn test_video_config_custom() {
        let config = VideoConfig {
            style: VideoStyle::Cinematic,
            resolution: VideoResolution::P4K,
            format: VideoFormat::Mov,
            prompt: Some("A futuristic cityscape".to_string()),
        };
        
        assert_eq!(config.style.as_str(), "cinematic");
        assert_eq!(config.resolution.as_str(), "4k");
        assert_eq!(config.format.as_str(), "mov");
        assert_eq!(config.prompt.as_deref(), Some("A futuristic cityscape"));
    }
}
