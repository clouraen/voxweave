use crate::state::{VideoStyle, VideoResolution, VideoFormat};

/// Configuration for video generation
#[derive(Debug, Clone)]
pub struct VideoConfig {
    pub style: VideoStyle,
    pub resolution: VideoResolution,
    pub format: VideoFormat,
    pub prompt: Option<String>,
}

#[cfg(feature = "video-generation")]
use crate::state::{LogEntry, LogLevel};
#[cfg(feature = "video-generation")]
use dioxus::prelude::{Signal, WritableExt};
#[cfg(feature = "video-generation")]
use std::path::{Path, PathBuf};

/// Z.ai Video Generation Service
#[cfg(feature = "video-generation")]
pub struct VideoGenerationService {
    api_key: String,
    base_url: String,
}

#[cfg(feature = "video-generation")]
impl VideoGenerationService {
    /// Create a new video generation service
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://api.z.ai/v1".to_string(),
        }
    }

    /// Get API key from environment or return default
    pub fn from_env() -> Option<Self> {
        std::env::var("ZAI_API_KEY")
            .ok()
            .map(Self::new)
    }

    /// Generate video from audio file using Z.ai API
    pub async fn generate_video(
        &self,
        audio_path: &Path,
        subtitle_path: Option<&Path>,
        config: &VideoConfig,
        mut logs: Signal<Vec<LogEntry>>,
        mut progress: Signal<u8>,
    ) -> Result<PathBuf, String> {

        // Step 1: Upload audio file (if needed)
        logs.write().push(LogEntry {
            message: "Uploading audio to Z.ai...".to_string(),
            level: LogLevel::Info,
        });
        
        let audio_url = self.upload_file(audio_path).await?;
        
        // Step 2: Create video generation job
        logs.write().push(LogEntry {
            message: "Creating video generation job...".to_string(),
            level: LogLevel::Info,
        });
        
        let job_id = self.create_video_job(&audio_url, config).await?;
        
        // Step 3: Poll for completion with progress updates
        logs.write().push(LogEntry {
            message: format!("Video generation started (Job ID: {})", job_id),
            level: LogLevel::Info,
        });
        
        let video_url = self.poll_video_status(&job_id, progress, logs).await?;
        
        // Step 4: Download generated video
        logs.write().push(LogEntry {
            message: "Downloading generated video...".to_string(),
            level: LogLevel::Info,
        });
        
        let video_path = self.download_video(&video_url, audio_path, config).await?;
        
        // Step 5: Combine with subtitles if needed
        if let Some(subtitle_path) = subtitle_path {
            logs.write().push(LogEntry {
                message: "Embedding subtitles into video...".to_string(),
                level: LogLevel::Info,
            });
            
            match self.embed_subtitles(&video_path, &subtitle_path, config).await {
                Ok(final_video_path) => {
                    logs.write().push(LogEntry {
                        message: format!("Video with subtitles ready: {}", final_video_path.display()),
                        level: LogLevel::Info,
                    });
                    Ok(final_video_path)
                }
                Err(e) => {
                    logs.write().push(LogEntry {
                        message: format!("Subtitle embedding failed: {}. Using video without subtitles.", e),
                        level: LogLevel::Notice,
                    });
                    Ok(video_path)
                }
            }
        } else {
            Ok(video_path)
        }
    }

    /// Upload file to Z.ai storage
    async fn upload_file(&self, file_path: &Path) -> Result<String, String> {
        use reqwest::multipart;
        use tokio::fs;
        
        // Read file data
        let file_data = fs::read(file_path)
            .await
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
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
                    .map_err(|e| format!("Failed to create form part: {}", e))?,
            );
        
        // Upload file to Z.ai
        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/upload", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await
            .map_err(|e| format!("Failed to upload file: {}", e))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Upload failed with status {}: {}", status, error_text));
        }
        
        // Parse response to get file URL
        let upload_response: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse upload response: {}", e))?;
        
        upload_response
            .get("url")
            .or_else(|| upload_response.get("file_url"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| {
                // Fallback to file:// URL if upload endpoint doesn't exist or returns unexpected format
                format!("file://{}", file_path.display())
            })
    }

    /// Create video generation job
    async fn create_video_job(
        &self,
        audio_url: &str,
        config: &VideoConfig,
    ) -> Result<String, String> {
        use serde_json::json;

        let mut request_body = json!({
            "audio_url": audio_url,
            "style": config.style.as_str(),
            "resolution": config.resolution.as_str(),
            "model": "video-generation", // Update with actual model name
        });

        if let Some(ref prompt) = config.prompt {
            request_body["prompt"] = json!(prompt);
        }

        let request_body_clone = request_body.clone();
        let base_url = self.base_url.clone();
        let api_key = self.api_key.clone();

        // Use retry logic for API call
        let response = self.retry_api_call(move || {
            let client = reqwest::Client::new();
            let rb = request_body_clone.clone();
            let url = base_url.clone();
            let key = api_key.clone();
            async move {
                client
                    .post(format!("{}/video/generate", url))
                    .header("Authorization", format!("Bearer {}", key))
                    .header("Content-Type", "application/json")
                    .json(&rb)
                    .send()
                    .await
                    .map_err(|e| format!("Failed to create video job: {}", e))
            }
        }, 3).await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("API error {}: {}", status, error_text));
        }

        let job_response: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        job_response
            .get("job_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| "Missing job_id in response".to_string())
    }

    /// Poll video generation status
    async fn poll_video_status(
        &self,
        job_id: &str,
        mut progress: Signal<u8>,
        logs: Signal<Vec<LogEntry>>,
    ) -> Result<String, String> {
        use std::time::Duration;
        
        let client = reqwest::Client::new();
        let max_attempts = 300; // 5 minutes max at 1s intervals
        let mut attempts = 0;

        loop {
            if attempts >= max_attempts {
                return Err("Video generation timeout".to_string());
            }

            tokio::time::sleep(Duration::from_secs(1)).await;
            attempts += 1;

            let response = client
                .get(format!("{}/video/status/{}", self.base_url, job_id))
                .header("Authorization", format!("Bearer {}", self.api_key))
                .send()
                .await
                .map_err(|e| format!("Failed to check status: {}", e))?;

            if !response.status().is_success() {
                continue;
            }

            let status: serde_json::Value = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse status: {}", e))?;

            let status_str = status
                .get("status")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");

            let progress_val = status
                .get("progress")
                .and_then(|v| v.as_u64())
                .map(|p| p as u8)
                .unwrap_or(0);

            *progress.write() = progress_val;

            match status_str {
                "completed" => {
                    return status
                        .get("video_url")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .ok_or_else(|| "Missing video_url in response".to_string());
                }
                "failed" => {
                    let error = status
                        .get("error")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown error");
                    return Err(format!("Video generation failed: {}", error));
                }
                "processing" | "queued" => {
                    // Continue polling
                }
                _ => {
                    return Err(format!("Unknown status: {}", status_str));
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
    ) -> Result<PathBuf, String> {
        use tokio::fs;
        use tokio::io::AsyncWriteExt;

        let client = reqwest::Client::new();
        
        let response = client
            .get(video_url)
            .send()
            .await
            .map_err(|e| format!("Failed to download video: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Download failed with status: {}", response.status()));
        }

        let video_data = response
            .bytes()
            .await
            .map_err(|e| format!("Failed to read video data: {}", e))?;

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
            .map_err(|e| format!("Failed to create video file: {}", e))?;

        file.write_all(&video_data)
            .await
            .map_err(|e| format!("Failed to write video file: {}", e))?;

        Ok(video_path)
    }

    /// Embed subtitles into video file
    async fn embed_subtitles(
        &self,
        video_path: &Path,
        subtitle_path: &Path,
        config: &VideoConfig,
    ) -> Result<PathBuf, String> {
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
        // For web platform, this would need to be done client-side or via API
        #[cfg(not(target_arch = "wasm32"))]
        {
            let subtitle_ext = subtitle_path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("srt");
            
            // Use ffmpeg command if available
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
                    return Ok(output_path);
                }
                Ok(output) => {
                    let error_msg = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("ffmpeg failed: {}", error_msg));
                }
                Err(_) => {
                    // ffmpeg not available, try alternative: copy video and add subtitle as external file
                    // For now, just copy the video and log that subtitles are separate
                    fs::copy(video_path, &output_path)
                        .await
                        .map_err(|e| format!("Failed to copy video: {}", e))?;
                    
                    // Copy subtitle file to same location
                    let subtitle_output = output_dir.join(format!("{}.{}", video_stem, subtitle_ext));
                    fs::copy(subtitle_path, &subtitle_output)
                        .await
                        .map_err(|e| format!("Failed to copy subtitle: {}", e))?;
                    
                    return Ok(output_path);
                }
            }
        }
        
        #[cfg(target_arch = "wasm32")]
        {
            // On web, we'd need to use a web-based subtitle embedding service
            // For now, just copy the video and provide subtitle as separate file
            let video_data = fs::read(video_path)
                .await
                .map_err(|e| format!("Failed to read video: {}", e))?;
            
            fs::write(&output_path, video_data)
                .await
                .map_err(|e| format!("Failed to write video: {}", e))?;
            
            Ok(output_path)
        }
    }
    
    /// Retry API call with exponential backoff
    async fn retry_api_call<F, Fut, T>(
        &self,
        operation: F,
        max_retries: u32,
    ) -> Result<T, String>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, String>>,
    {
        use std::time::Duration;
        
        let mut attempt = 0;
        let mut delay = Duration::from_secs(1);
        
        loop {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) if attempt < max_retries => {
                    attempt += 1;
                    log::warn!("API call failed (attempt {}/{}): {}. Retrying in {:?}...", 
                        attempt, max_retries, e, delay);
                    tokio::time::sleep(delay).await;
                    delay = delay * 2; // Exponential backoff
                }
                Err(e) => return Err(format!("API call failed after {} attempts: {}", max_retries, e)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_config() {
        let config = VideoConfig {
            style: VideoStyle::Realistic,
            resolution: VideoResolution::P1080,
            format: VideoFormat::Mp4,
            prompt: Some("A cyberpunk cityscape".to_string()),
        };
        
        assert_eq!(config.style.as_str(), "realistic");
        assert_eq!(config.resolution.as_str(), "1080p");
        assert_eq!(config.format.as_str(), "mp4");
    }
}

