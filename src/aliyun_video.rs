//! Alibaba Cloud Aliyun Video Generation Service
//! Generates videos using Alibaba's wan2.5-i2v-preview model via Bailian API
//! This is a CLI-compatible version without Dioxus dependencies.

use crate::queue::{VideoResolution, VideoStyle, LogLevel};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[cfg(feature = "video-generation")]
use anyhow::{Context, Result};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AliyunVideoRequest {
    pub model: String, // "wan2.5-i2v-preview"
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
#[cfg(feature = "video-generation")]
pub struct AliyunVideoService {
    api_key: String,
    base_url: String,
}

#[cfg(feature = "video-generation")]
impl AliyunVideoService {
    /// Create a new Aliyun video service
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://bailian.cn-beijing.aliyuncs.com/api/v1/chat/completions".to_string(),
        }
    }

    /// Get API key from environment ALIBABA_API_KEY
    pub fn from_env() -> Result<Self> {
        let api_key = std::env::var("ALIBABA_API_KEY")
            .context("ALIBABA_API_KEY environment variable not set")?;
        Ok(Self::new(api_key))
    }

    /// Generate video using Alibaba wan2.5-i2v-preview model
    /// 
    /// # Arguments
    /// * `prompt` - Text description for video generation
    /// * `resolution` - Video resolution
    /// * `output_dir` - Directory to save the generated video
    /// * `progress_callback` - Optional callback for progress updates (0-100)
    /// * `log_callback` - Optional callback for log messages
    pub async fn generate_video<F, G>(
        &self,
        prompt: &str,
        resolution: VideoResolution,
        output_dir: &Path,
        mut progress_callback: Option<F>,
        mut log_callback: Option<G>,
    ) -> Result<PathBuf>
    where
        F: FnMut(u8),
        G: FnMut(&str, LogLevel),
    {
        use reqwest::Client;

        let client = Client::new();

        if let Some(ref mut cb) = log_callback {
            cb("Starting Alibaba wan2.5-i2v-preview video generation...", LogLevel::Info);
        }

        if let Some(ref mut cb) = progress_callback {
            cb(10);
        }

        // Convert resolution to string format
        let resolution_str = match resolution {
            VideoResolution::P720 => "1280x720",
            VideoResolution::P1080 => "1920x1080",
            VideoResolution::P4K => "3840x2160",
        };

        // Build request
        let request = AliyunVideoRequest {
            model: "wan2.5-i2v-preview".to_string(),
            input: AliyunVideoInput {
                prompt: prompt.to_string(),
            },
            parameters: Some(AliyunVideoParameters {
                resolution: Some(resolution_str.to_string()),
                duration: None, // Let the model decide based on the prompt
            }),
        };

        if let Some(ref mut cb) = log_callback {
            cb("Sending video generation request to Alibaba Cloud...", LogLevel::Info);
        }

        // Send request
        let response = client
            .post(&self.base_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Alibaba Cloud")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("Alibaba Cloud API error ({}): {}", status, text);
        }

        let video_response: AliyunVideoResponse = response
            .json()
            .await
            .context("Failed to parse Alibaba Cloud response")?;

        // Check for errors in response
        if let Some(error) = video_response.error {
            anyhow::bail!("Alibaba Cloud error: {} - {}", error.code, error.message);
        }

        if let Some(ref mut cb) = log_callback {
            cb(&format!("Video generation job created: ID={}", video_response.id), LogLevel::Info);
        }

        if let Some(ref mut cb) = progress_callback {
            cb(30);
        }

        // Poll for completion
        let video_url = self.poll_video_status(&video_response.id, &mut progress_callback, &mut log_callback).await?;

        if let Some(ref mut cb) = progress_callback {
            cb(80);
        }

        // Download video
        if let Some(ref mut cb) = log_callback {
            cb("Downloading generated video...", LogLevel::Info);
        }

        let video_bytes = client
            .get(&video_url)
            .send()
            .await
            .context("Failed to download video")?
            .bytes()
            .await
            .context("Failed to read video bytes")?;

        // Save video to output directory
        let output_path = output_dir.join("video.mp4");

        tokio::fs::write(&output_path, &video_bytes)
            .await
            .context("Failed to write video file")?;

        if let Some(ref mut cb) = log_callback {
            cb(&format!("Video downloaded: {}", output_path.display()), LogLevel::Info);
        }

        if let Some(ref mut cb) = progress_callback {
            cb(100);
        }

        Ok(output_path)
    }

    /// Poll for video generation status
    async fn poll_video_status<F, G>(
        &self,
        task_id: &str,
        progress_callback: &mut Option<F>,
        log_callback: &mut Option<G>,
    ) -> Result<String>
    where
        F: FnMut(u8),
        G: FnMut(&str, LogLevel),
    {
        use reqwest::Client;
        use std::time::Duration;

        let client = Client::new();
        let status_url = format!("{}/status/{}", self.base_url.replace("/chat/completions", ""), task_id);
        let max_attempts = 120; // 10 minutes max (5 second intervals)
        let mut attempts = 0;

        loop {
            if attempts >= max_attempts {
                anyhow::bail!("Video generation timeout - exceeded maximum wait time (10 minutes)");
            }

            tokio::time::sleep(Duration::from_secs(5)).await;

            let response = client
                .get(&status_url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .send()
                .await
                .context("Failed to check video status")?;

            if !response.status().is_success() {
                attempts += 1;
                continue;
            }

            let status_response: AliyunVideoResponse = response
                .json()
                .await
                .context("Failed to parse status response")?;

            // Update progress based on status
            let progress_val = match status_response.status.as_str() {
                "pending" | "queued" => 30,
                "processing" => 30 + ((attempts * 50) / max_attempts) as u8,
                "completed" | "success" => 80,
                _ => 30,
            };
            
            if let Some(ref mut cb) = progress_callback {
                cb(progress_val);
            }

            match status_response.status.as_str() {
                "completed" | "success" => {
                    if let Some(output) = status_response.output {
                        if let Some(url) = output.video_url {
                            if let Some(ref mut cb) = log_callback {
                                cb("Video generation completed!", LogLevel::Info);
                            }
                            return Ok(url);
                        }
                    }
                    anyhow::bail!("Video completed but no URL provided");
                }
                "failed" | "error" => {
                    let error_msg = status_response.error
                        .map(|e| format!("{} - {}", e.code, e.message))
                        .unwrap_or_else(|| "Unknown error".to_string());
                    anyhow::bail!("Video generation failed: {}", error_msg);
                }
                "processing" | "pending" | "queued" => {
                    if attempts % 12 == 0 {
                        // Log every minute
                        if let Some(ref mut cb) = log_callback {
                            cb(&format!("Video generation in progress... (attempt {}/{})", attempts + 1, max_attempts), LogLevel::Info);
                        }
                    }
                    attempts += 1;
                }
                _ => {
                    attempts += 1;
                }
            }
        }
    }
}

/// Convert VideoStyle to a prompt modifier for Alibaba API
pub fn style_to_prompt_modifier(style: VideoStyle) -> &'static str {
    match style {
        VideoStyle::Realistic => "Photorealistic cinematic video",
        VideoStyle::Anime => "Anime-style animated video",
        VideoStyle::ThreeD => "3D rendered video with high-quality graphics",
        VideoStyle::Cinematic => "Cinematic movie-style video with dramatic lighting",
        VideoStyle::Biotech => "Futuristic biotech laboratory with DNA particles and holographic displays",
        VideoStyle::Cyberpunk => "Cyberpunk neon cityscape with vibrant colors and futuristic aesthetics",
        VideoStyle::Educational => "Clean professional educational video with clear visuals",
        VideoStyle::Wan2_5 => "High-quality AI-generated video",
    }
}

/// Construct a full prompt from style and base content
pub fn construct_prompt(style: VideoStyle, base_content: &str, custom_prompt: Option<&str>) -> String {
    if let Some(custom) = custom_prompt {
        // Custom prompt completely overrides
        custom.to_string()
    } else {
        // Combine style modifier with base content
        let modifier = style_to_prompt_modifier(style);
        format!("{} depicting: {}", modifier, base_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_to_prompt_modifier() {
        assert_eq!(style_to_prompt_modifier(VideoStyle::Realistic), "Photorealistic cinematic video");
        assert_eq!(style_to_prompt_modifier(VideoStyle::Anime), "Anime-style animated video");
        assert_eq!(style_to_prompt_modifier(VideoStyle::Cyberpunk), "Cyberpunk neon cityscape with vibrant colors and futuristic aesthetics");
    }

    #[test]
    fn test_construct_prompt_with_custom() {
        let custom_prompt = Some("Epic space battle");
        let result = construct_prompt(VideoStyle::Cinematic, "base content", custom_prompt);
        assert_eq!(result, "Epic space battle");
    }

    #[test]
    fn test_construct_prompt_without_custom() {
        let result = construct_prompt(VideoStyle::Cyberpunk, "a futuristic city", None);
        assert!(result.contains("Cyberpunk neon cityscape"));
        assert!(result.contains("a futuristic city"));
    }

    #[test]
    fn test_resolution_conversion() {
        assert_eq!(VideoResolution::P720.as_str(), "720p");
        assert_eq!(VideoResolution::P1080.as_str(), "1080p");
        assert_eq!(VideoResolution::P4K.as_str(), "4k");
    }
}
