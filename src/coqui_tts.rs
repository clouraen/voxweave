use crate::tts::{SpeechEngine, TtsError, VoiceProfile, VoiceEngine, COQUI_BRIDGE_SCRIPT};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use serde_json::json;

#[derive(Debug, Clone)]
pub struct CoquiEngine {
    python_command: String,
    model_name: String,
    device: String,
    sample_rate: u32,
    language: String,
}

#[cfg(feature = "coqui-tts")]
impl Default for CoquiEngine {
    fn default() -> Self {
        let python_command =
            std::env::var("VOXWEAVE_COQUI_PYTHON").unwrap_or_else(|_| "python3".to_string());
        let model_name = std::env::var("VOXWEAVE_COQUI_MODEL")
            .unwrap_or_else(|_| "tts_models/multilingual/multi-dataset/xtts_v2".to_string());
        let device = std::env::var("VOXWEAVE_COQUI_DEVICE")
            .unwrap_or_else(|_| "cpu".to_string());
        let sample_rate = std::env::var("VOXWEAVE_COQUI_SAMPLE_RATE")
            .ok()
            .and_then(|value| value.parse::<u32>().ok())
            .filter(|rate| *rate > 0)
            .unwrap_or(24_000);
        let language = std::env::var("VOXWEAVE_COQUI_LANGUAGE")
            .unwrap_or_else(|_| "en".to_string());
        
        Self {
            python_command,
            model_name,
            device,
            sample_rate,
            language,
        }
    }
}

#[cfg(feature = "coqui-tts")]
impl SpeechEngine for CoquiEngine {
    fn synthesize_to_file(
        &self,
        text: &str,
        voice: &VoiceProfile,
        speed: f32,
        output: &Path,
    ) -> Result<(), TtsError> {
        if voice.engine != VoiceEngine::Coqui {
            return Err(TtsError::UnsupportedVoice(voice.id.clone()));
        }

        let lang = voice
            .lang
            .as_deref()
            .unwrap_or(&self.language);

        // Check if voice cloning is enabled (clone path stored in command field)
        let speaker_wav = voice.command.as_deref();
        let _use_cloning = speaker_wav.is_some() && Path::new(speaker_wav.unwrap()).exists();

        let payload = json!({
            "text": text,
            "output": output.to_string_lossy(),
            "sample_rate": self.sample_rate,
            "model_name": &self.model_name,
            "device": &self.device,
            "language": lang,
            "speed": speed,
            "speaker_wav": speaker_wav,
        });

        let mut child = Command::new(&self.python_command)
            .arg("-c")
            .arg(COQUI_BRIDGE_SCRIPT)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| TtsError::CommandFailed(err.to_string()))?;

        if let Some(stdin) = child.stdin.as_mut() {
            let data =
                serde_json::to_vec(&payload).map_err(|err| TtsError::Backend(err.to_string()))?;
            stdin
                .write_all(&data)
                .map_err(|err| TtsError::Backend(err.to_string()))?;
        } else {
            return Err(TtsError::Backend(
                "failed to open stdin for coqui bridge".into(),
            ));
        }

        let output_data = child
            .wait_with_output()
            .map_err(|err| TtsError::CommandFailed(err.to_string()))?;

        if !output_data.status.success() {
            let stderr = String::from_utf8_lossy(&output_data.stderr);
            return Err(TtsError::CommandFailed(stderr.trim().to_string()));
        }

        if !output_data.stdout.is_empty() {
            if let Err(err) = serde_json::from_slice::<serde_json::Value>(&output_data.stdout) {
                eprintln!("warning: failed to decode coqui metadata: {err}");
            }
        }

        Ok(())
    }
}

