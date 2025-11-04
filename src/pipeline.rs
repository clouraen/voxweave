use std::path::PathBuf;

use anyhow::Context;

use crate::queue::{ConversionQueue, SubtitleGranularity};
use crate::sanitize::sanitize_name_for_os;
use crate::subtitle::{format_srt, generate_subtitles};
use crate::text::clean_text;
use crate::tts::{SpeechEngine, VoiceProfile};
use crate::mlt::{MltConfig, create_mlt_project, get_audio_duration};
use crate::package::create_package;

#[derive(Debug, Clone)]
pub struct ConvertRequest {
    pub source: PathBuf,
    pub output_dir: PathBuf,
    pub voice: VoiceProfile,
    pub speed: f32,
    pub subtitle_granularity: SubtitleGranularity,
    pub replace_single_newlines: bool,
    pub average_words_per_minute: f32,
    pub create_package: bool,
    pub package_name: Option<String>,
}

pub fn convert_path<E: SpeechEngine>(
    engine: &E,
    request: &ConvertRequest,
) -> anyhow::Result<PathBuf> {
    let text = std::fs::read_to_string(&request.source)
        .with_context(|| format!("reading {}", request.source.display()))?;
    std::fs::create_dir_all(&request.output_dir)?;

    let cleaned = clean_text(&text, request.replace_single_newlines);
    let base_name = request
        .source
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("output");
    let sanitized = sanitize_name_for_os(base_name, false /* is_folder irrelevant */);
    let audio_path = request.output_dir.join(format!("{sanitized}.wav"));
    engine
        .synthesize_to_file(&cleaned, &request.voice, request.speed, &audio_path)
        .context("speech synthesis failed")?;

    let subtitle_path = if !cleaned.is_empty() {
        let subtitles = generate_subtitles(
            &cleaned,
            request.subtitle_granularity,
            request.average_words_per_minute,
        );
        if !subtitles.is_empty() {
            let srt = format_srt(&subtitles);
            let subtitle_path = request.output_dir.join(format!("{sanitized}.srt"));
            std::fs::write(&subtitle_path, srt).context("writing subtitles failed")?;
            Some(subtitle_path)
        } else {
            None
        }
    } else {
        None
    };

    // If package creation is requested, create MLT project and ZIP package
    if request.create_package {
        // Get audio duration
        let duration = get_audio_duration(&audio_path)
            .unwrap_or(10.0); // Default to 10 seconds if we can't read duration
        
        // Create MLT project
        let mlt_config = MltConfig {
            duration_seconds: duration,
            ..Default::default()
        };
        
        let mlt_content = create_mlt_project(
            &audio_path,
            subtitle_path.as_deref(),
            &mlt_config,
        )?;
        
        // Determine package name
        let package_name = request.package_name.as_deref()
            .unwrap_or(&sanitized);
        let zip_path = request.output_dir.join(format!("{}.zip", package_name));
        
        // Create ZIP package
        let final_path = create_package(
            &audio_path,
            subtitle_path.as_deref(),
            &mlt_content,
            &zip_path,
        )?;
        
        return Ok(final_path);
    }

    Ok(audio_path)
}

pub fn convert_queue<E: SpeechEngine>(
    engine: &E,
    queue: &mut ConversionQueue,
) -> anyhow::Result<Vec<PathBuf>> {
    let mut outputs = Vec::new();
    while let Some(item) = queue.dequeue() {
        let req = ConvertRequest {
            source: PathBuf::from(item.source_path),
            output_dir: PathBuf::from(item.output_dir),
            voice: item.voice,
            speed: item.speed,
            subtitle_granularity: item.subtitle_granularity,
            replace_single_newlines: item.replace_single_newlines,
            average_words_per_minute: 150.0,
            create_package: false,
            package_name: None,
        };
        outputs.push(convert_path(engine, &req)?);
    }
    Ok(outputs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::queue::{ConversionQueue, QueueItem, SubtitleGranularity};
    use crate::tts::{MockSpeechEngine, VoiceProfile};

    fn temp_input_with_content(content: &str) -> tempfile::NamedTempFile {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        use std::io::Write;
        write!(file, "{content}").unwrap();
        file
    }

    #[test]
    fn convert_path_cleans_text_before_writing() {
        let engine = MockSpeechEngine::default();
        let input = temp_input_with_content(" Hello   world \n");
        let output_dir = tempfile::tempdir().unwrap();

        let request = ConvertRequest {
            source: input.path().to_path_buf(),
            output_dir: output_dir.path().to_path_buf(),
            voice: VoiceProfile::builtin("default", "Default"),
            speed: 1.0,
            subtitle_granularity: SubtitleGranularity::Disabled,
            replace_single_newlines: false,
            average_words_per_minute: 150.0,
            create_package: false,
            package_name: None,
        };

        let output_path = convert_path(&engine, &request).unwrap();
        let written = std::fs::read_to_string(&output_path).unwrap();
        assert_eq!(written, "Hello world");
    }

    #[test]
    fn convert_queue_processes_all_items() {
        let engine = MockSpeechEngine::default();
        let mut queue = ConversionQueue::new();
        let mut _input_files = Vec::new();
        let mut _output_dirs = Vec::new();
        for idx in 0..2 {
            let input = temp_input_with_content(&format!("Text {idx}"));
            let output_dir = tempfile::tempdir().unwrap();
            queue.enqueue(QueueItem {
                source_path: input.path().to_string_lossy().to_string(),
                output_dir: output_dir.path().to_string_lossy().to_string(),
                voice: VoiceProfile::builtin("default", "Default"),
                speed: 1.0,
                subtitle_granularity: SubtitleGranularity::Disabled,
                replace_single_newlines: false,
                create_package: false,
                package_name: None,
            });
            _input_files.push(input);
            _output_dirs.push(output_dir);
        }
        let outputs = convert_queue(&engine, &mut queue).unwrap();
        assert_eq!(outputs.len(), 2);
        for (idx, path) in outputs.iter().enumerate() {
            let written = std::fs::read_to_string(path).unwrap();
            assert_eq!(written, format!("Text {idx}"));
        }
    }
}
