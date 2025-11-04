//! Z.AI Video Generation Service
//! 
//! Implements video generation using Z.AI's CogVideoX-3 model

use std::path::{Path, PathBuf};
use crate::queue::{VideoStyle, VideoResolution, VideoFormat, LogLevel};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Z.AI Video Generation Request
#[derive(Debug, Serialize)]
struct ZAIVideoRequest {
    model: String,
    prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_url: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fps: Option<i32>,
}

/// Z.AI Video Generation Response
#[derive(Debug, Deserialize)]
struct ZAIVideoResponse {
    id: String,
    model: String,
    task_status: String,
    #[serde(default)]
    request_id: Option<String>,
}

/// Z.AI Video Status Response
#[derive(Debug, Deserialize)]
struct ZAIStatusResponse {
    model: String,
    task_status: String,
    #[serde(default)]
    video_result: Option<Vec<VideoResult>>,
    #[serde(default)]
    request_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct VideoResult {
    url: String,
    #[serde(default)]
    cover_image_url: Option<String>,
}

/// Z.AI Video Generation Service
pub struct ZAIVideoService {
    api_key: String,
    base_url: String,
}

impl ZAIVideoService {
    /// Create a new Z.AI video service
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://api.z.ai/api/paas/v4".to_string(),
        }
    }

    /// Get API key from environment ZAI_API_KEY
    pub fn from_env() -> Result<Self> {
        let api_key = std::env::var("ZAI_API_KEY")
            .context("ZAI_API_KEY environment variable not set")?;
        Ok(Self::new(api_key))
    }

    /// Generate video using Z.AI Vidu 2 model family (image, start-end, reference)
    pub async fn generate_video<F, G>(
        &self,
        prompt: &str,
        resolution: VideoResolution,
        output_dir: &Path,
        image_urls: Option<&[String]>,
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
            cb("Starting Z.AI Vidu 2 video generation...", LogLevel::Info);
        }

        if let Some(ref mut cb) = progress_callback {
            cb(10);
        }

        // Convert resolution to Z.AI format (720p supported by Vidu 2)
        let size = match resolution {
            VideoResolution::P720 => "1280x720",
            VideoResolution::P1080 => "1920x1080",
            VideoResolution::P4K => "3840x2160",
        };

        // Choose Vidu 2 variant based on images provided
        let (model, image_url_field): (String, Option<Vec<String>>) = match image_urls {
            Some(urls) if !urls.is_empty() => {
                match urls.len() {
                    1 => ("vidu2-image".to_string(), Some(vec![urls[0].clone()])),
                    2 => ("vidu2-start-end".to_string(), Some(vec![urls[0].clone(), urls[1].clone()])),
                    _ => ("vidu2-reference".to_string(), Some(urls.to_vec())),
                }
            }
            _ => ("vidu2-reference".to_string(), None), // prompt-only fallback
        };

        // Build request
        let request = ZAIVideoRequest {
            model,
            prompt: prompt.to_string(),
            image_url: image_url_field,
            size: Some(size.to_string()),
            quality: Some("speed".to_string()),
            duration: Some(4),
            fps: Some(30),
        };

        if let Some(ref mut cb) = log_callback {
            cb("Sending video generation request to Z.AI...", LogLevel::Info);
        }

        // Send request
        let response = client
            .post(format!("{}/videos/generations", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Z.AI")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("Z.AI API error ({}): {}", status, text);
        }

        let video_response: ZAIVideoResponse = response
            .json()
            .await
            .context("Failed to parse Z.AI response")?;

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
        let max_attempts = 120; // 10 minutes max (5 second intervals)
        let mut attempts = 0;

        loop {
            if attempts >= max_attempts {
                anyhow::bail!("Video generation timeout - exceeded maximum wait time (10 minutes)");
            }

            tokio::time::sleep(Duration::from_secs(5)).await;

            let response = client
                .get(format!("{}/videos/generations/{}", self.base_url, task_id))
                .header("Authorization", format!("Bearer {}", self.api_key))
                .send()
                .await
                .context("Failed to check video status")?;

            if !response.status().is_success() {
                attempts += 1;
                continue;
            }

            let status_response: ZAIStatusResponse = response
                .json()
                .await
                .context("Failed to parse status response")?;

            // Update progress based on status
            let progress_val = match status_response.task_status.as_str() {
                "PROCESSING" => 30 + ((attempts * 50) / max_attempts) as u8,
                "SUCCESS" => 80,
                _ => 30,
            };
            
            if let Some(ref mut cb) = progress_callback {
                cb(progress_val);
            }

            match status_response.task_status.as_str() {
                "SUCCESS" => {
                    if let Some(video_results) = status_response.video_result {
                        if let Some(first_result) = video_results.first() {
                            if let Some(ref mut cb) = log_callback {
                                cb("Video generation completed!", LogLevel::Info);
                            }
                            return Ok(first_result.url.clone());
                        }
                    }
                    anyhow::bail!("Video completed but no URL provided");
                }
                "FAIL" => {
                    anyhow::bail!("Video generation failed");
                }
                "PROCESSING" => {
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

/// Convert VideoStyle to a prompt modifier for Z.AI
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
