use std::path::{Path, PathBuf};
use crate::queue::{VideoStyle, VideoResolution, VideoFormat, LogLevel};

#[cfg(feature = "video-generation")]
use anyhow::Result;

/// Configuration for video generation
#[derive(Debug, Clone)]
pub struct VideoConfig {
    pub style: VideoStyle,
    pub resolution: VideoResolution,
    pub format: VideoFormat,
    pub prompt: Option<String>,
    pub image_urls: Option<Vec<String>>,
}

impl Default for VideoConfig {
    fn default() -> Self {
        Self {
            style: VideoStyle::Cyberpunk,
            resolution: VideoResolution::P1080,
            format: VideoFormat::Mp4,
            prompt: None,
            image_urls: None,
        }
    }
}

/// Z.AI Video Generation Service
#[cfg(feature = "video-generation")]
pub use crate::zai_video::ZAIVideoService as VideoGenerationService;

/// Extension trait for VideoGenerationService to provide CLI-compatible interface
#[cfg(feature = "video-generation")]
pub trait VideoGenerationServiceExt {
    async fn generate_video_from_text<F, G>(
        &self,
        _audio_path: &Path,
        subtitle_path: Option<&Path>,
        config: &VideoConfig,
        progress_callback: Option<F>,
        log_callback: Option<G>,
    ) -> Result<PathBuf>
    where
        F: FnMut(u8),
        G: FnMut(&str, LogLevel);
}

#[cfg(feature = "video-generation")]
impl VideoGenerationServiceExt for VideoGenerationService {
    async fn generate_video_from_text<F, G>(
        &self,
        _audio_path: &Path,
        subtitle_path: Option<&Path>,
        config: &VideoConfig,
        progress_callback: Option<F>,
        log_callback: Option<G>,
    ) -> Result<PathBuf>
    where
        F: FnMut(u8),
        G: FnMut(&str, LogLevel),
    {
        // Build prompt from subtitle or config
        let base_content = if let Some(subtitle_path) = subtitle_path {
            tokio::fs::read_to_string(subtitle_path)
                .await
                .unwrap_or_else(|_| "Generate a video".to_string())
        } else {
            config.prompt.clone().unwrap_or_else(|| "Generate a video".to_string())
        };

        // Construct prompt using style
        let prompt = crate::zai_video::construct_prompt(
            config.style,
            &base_content,
            config.prompt.as_deref(),
        );

        // Determine output directory
        let output_dir = if let Some(subtitle_path) = subtitle_path {
            subtitle_path.parent().unwrap_or_else(|| Path::new("."))
        } else {
            Path::new(".")
        };

        // Call the zai_video Vidu2 generate_video method with optional images
        self.generate_video(
            &prompt,
            config.resolution,
            output_dir,
            config.image_urls.as_deref(),
            progress_callback,
            log_callback,
        ).await
    }
}
