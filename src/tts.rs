use std::env;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TtsError {
    #[error("unsupported voice: {0}")]
    UnsupportedVoice(String),
    #[error("command execution failed: {0}")]
    CommandFailed(String),
    #[error("audio backend error: {0}")]
    Backend(String),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VoiceEngine {
    Espeak,
    Kokoro,
    Coqui,
}

impl VoiceEngine {
    pub fn as_str(&self) -> &'static str {
        match self {
            VoiceEngine::Espeak => "espeak",
            VoiceEngine::Kokoro => "kokoro",
            VoiceEngine::Coqui => "coqui",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VoiceProfile {
    pub id: String,
    pub description: String,
    pub engine: VoiceEngine,
    pub command: Option<String>,
    pub lang: Option<String>,
}

impl VoiceProfile {
    pub fn builtin(id: &str, description: &str) -> Self {
        Self::espeak(id, description, None)
    }

    pub fn espeak(id: &str, description: &str, command: Option<&str>) -> Self {
        Self {
            id: id.to_owned(),
            description: description.to_owned(),
            engine: VoiceEngine::Espeak,
            command: command.map(|cmd| cmd.to_owned()),
            lang: None,
        }
    }

    pub fn kokoro(id: &str, description: &str, lang: &str) -> Self {
        Self {
            id: id.to_owned(),
            description: description.to_owned(),
            engine: VoiceEngine::Kokoro,
            command: None,
            lang: Some(lang.to_owned()),
        }
    }
    
    pub fn coqui(id: &str, description: &str, lang: &str) -> Self {
        Self {
            id: id.to_owned(),
            description: description.to_owned(),
            engine: VoiceEngine::Coqui,
            command: None,
            lang: Some(lang.to_owned()),
        }
    }
    
    pub fn coqui_clone(id: &str, description: &str, lang: &str, clone_path: &str) -> Self {
        Self {
            id: id.to_owned(),
            description: description.to_owned(),
            engine: VoiceEngine::Coqui,
            command: Some(clone_path.to_owned()),  // Store clone audio path in command field
            lang: Some(lang.to_owned()),
        }
    }
}

pub trait SpeechEngine {
    fn synthesize_to_file(
        &self,
        text: &str,
        voice: &VoiceProfile,
        speed: f32,
        output: &Path,
    ) -> Result<(), TtsError>;
}

#[derive(Default)]
pub struct MockSpeechEngine;

impl SpeechEngine for MockSpeechEngine {
    fn synthesize_to_file(
        &self,
        text: &str,
        _voice: &VoiceProfile,
        _speed: f32,
        output: &Path,
    ) -> Result<(), TtsError> {
        std::fs::write(output, text).map_err(|err| TtsError::Backend(err.to_string()))
    }
}

#[derive(Default)]
pub struct EspeakEngine;

impl EspeakEngine {
    fn resolve_command() -> String {
        env::var("VOXWEAVE_ESPEAK_COMMAND").unwrap_or_else(|_| "espeak".to_string())
    }

    fn normalized_speed(speed: f32) -> i32 {
        let base_rate = 175.0;
        let rate = (base_rate * speed).round();
        rate.clamp(80.0, 450.0) as i32
    }
}

impl SpeechEngine for EspeakEngine {
    fn synthesize_to_file(
        &self,
        text: &str,
        voice: &VoiceProfile,
        speed: f32,
        output: &Path,
    ) -> Result<(), TtsError> {
        if voice.engine != VoiceEngine::Espeak {
            return Err(TtsError::UnsupportedVoice(voice.id.clone()));
        }

        if text.trim().is_empty() {
            std::fs::write(output, b"").map_err(|err| TtsError::Backend(err.to_string()))?;
            return Ok(());
        }

        let command = Self::resolve_command();
        let voice_id = voice.command.as_deref().unwrap_or(&voice.id);
        let rate = Self::normalized_speed(speed);

        let mut child = Command::new(&command)
            .arg("-w")
            .arg(output)
            .arg("-v")
            .arg(voice_id)
            .arg("-s")
            .arg(rate.to_string())
            .arg("--stdin")
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|err| TtsError::CommandFailed(err.to_string()))?;

        if let Some(stdin) = child.stdin.as_mut() {
            stdin
                .write_all(text.as_bytes())
                .map_err(|err| TtsError::Backend(err.to_string()))?;
        } else {
            return Err(TtsError::Backend("failed to open stdin for espeak".into()));
        }

        let status = child
            .wait()
            .map_err(|err| TtsError::CommandFailed(err.to_string()))?;
        if status.success() {
            Ok(())
        } else {
            Err(TtsError::CommandFailed(format!(
                "{command} exited with status {status}"
            )))
        }
    }
}

#[derive(Debug, Clone)]
pub struct KokoroEngine {
    python_command: String,
    repo_id: String,
    device: String,
    sample_rate: u32,
    split_pattern: Option<String>,
}

impl Default for KokoroEngine {
    fn default() -> Self {
        let python_command =
            env::var("VOXWEAVE_KOKORO_PYTHON").unwrap_or_else(|_| "python3".to_string());
        let repo_id = env::var("VOXWEAVE_KOKORO_REPO_ID")
            .unwrap_or_else(|_| "hexgrad/Kokoro-82M".to_string());
        let device = env::var("VOXWEAVE_KOKORO_DEVICE").unwrap_or_else(|_| "cpu".to_string());
        let sample_rate = env::var("VOXWEAVE_KOKORO_SAMPLE_RATE")
            .ok()
            .and_then(|value| value.parse::<u32>().ok())
            .filter(|rate| *rate > 0)
            .unwrap_or(24_000);
        let split_pattern = env::var("VOXWEAVE_KOKORO_SPLIT_PATTERN").ok();
        Self {
            python_command,
            repo_id,
            device,
            sample_rate,
            split_pattern,
        }
    }
}

impl SpeechEngine for KokoroEngine {
    fn synthesize_to_file(
        &self,
        text: &str,
        voice: &VoiceProfile,
        speed: f32,
        output: &Path,
    ) -> Result<(), TtsError> {
        if voice.engine != VoiceEngine::Kokoro {
            return Err(TtsError::UnsupportedVoice(voice.id.clone()));
        }

        let lang = voice
            .lang
            .as_deref()
            .ok_or_else(|| TtsError::Backend("kokoro voice missing language code".into()))?;

        let payload = json!({
            "text": text,
            "voice": &voice.id,
            "lang_code": lang,
            "speed": speed,
            "output": output.to_string_lossy(),
            "sample_rate": self.sample_rate,
            "repo_id": &self.repo_id,
            "device": &self.device,
            "split_pattern": self.split_pattern.as_deref().unwrap_or("\\n+"),
        });

        let mut child = Command::new(&self.python_command)
            .arg("-c")
            .arg(KOKORO_BRIDGE_SCRIPT)
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
                "failed to open stdin for kokoro bridge".into(),
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
            if let Err(err) = serde_json::from_slice::<Value>(&output_data.stdout) {
                eprintln!("warning: failed to decode kokoro metadata: {err}");
            }
        }

        Ok(())
    }
}

pub fn default_voice_profiles() -> Vec<VoiceProfile> {
    let mut voices: Vec<VoiceProfile> = ESPEAK_VOICES
        .iter()
        .map(|(id, description, command)| VoiceProfile::espeak(id, description, *command))
        .collect();

    voices.extend(build_kokoro_voice_profiles());
    voices
}

pub fn find_voice<'a>(voices: &'a [VoiceProfile], id: &str) -> Option<&'a VoiceProfile> {
    voices.iter().find(|voice| voice.id == id)
}

fn build_kokoro_voice_profiles() -> Vec<VoiceProfile> {
    KOKORO_VOICES
        .iter()
        .filter_map(|voice_id| {
            let lang_code = voice_id.chars().next()?;
            let lang_code = lang_code.to_string();
            let description = format!(
                "{} ({})",
                format_voice_name(voice_id),
                kokoro_language_description(&lang_code)
            );
            Some(VoiceProfile::kokoro(voice_id, &description, &lang_code))
        })
        .collect()
}

fn format_voice_name(id: &str) -> String {
    id.split('_')
        .nth(1)
        .map(|name| {
            name.split('-')
                .map(|part| {
                    if part.is_empty() {
                        String::new()
                    } else {
                        let mut chars = part.chars();
                        if let Some(first) = chars.next() {
                            let mut result = first.to_uppercase().collect::<String>();
                            result.push_str(&chars.as_str().to_lowercase());
                            result
                        } else {
                            String::new()
                        }
                    }
                })
                .collect::<Vec<_>>()
                .join("-")
        })
        .unwrap_or_else(|| id.to_string())
}

fn kokoro_language_description(code: &str) -> &'static str {
    for (lang_code, description) in KOKORO_LANGUAGES {
        if *lang_code == code {
            return description;
        }
    }
    "Unknown"
}

const ESPEAK_VOICES: &[(&str, &str, Option<&str>)] = &[
    ("en-us", "English (US)", Some("en-us")),
    ("en-gb", "English (UK)", Some("en-gb")),
    ("en-au", "English (Australia)", Some("en-au")),
    ("es-es", "Spanish", Some("es")),
    ("fr-fr", "French", Some("fr")),
    ("de-de", "German", Some("de")),
    ("it-it", "Italian", Some("it")),
    ("pt-br", "Portuguese (Brazil)", Some("pt-br")),
    ("tr-tr", "Turkish", Some("tr")),
    ("hi-in", "Hindi", Some("hi")),
    ("ja-jp", "Japanese", Some("ja")),
    ("zh-cn", "Chinese (Mandarin)", Some("zh")),
];

const KOKORO_LANGUAGES: &[(&str, &str)] = &[
    ("a", "American English"),
    ("b", "British English"),
    ("e", "Spanish"),
    ("f", "French"),
    ("h", "Hindi"),
    ("i", "Italian"),
    ("j", "Japanese"),
    ("p", "Brazilian Portuguese"),
    ("z", "Mandarin Chinese"),
];

const KOKORO_VOICES: &[&str] = &[
    "af_alloy",
    "af_aoede",
    "af_bella",
    "af_heart",
    "af_jessica",
    "af_kore",
    "af_nicole",
    "af_nova",
    "af_river",
    "af_sarah",
    "af_sky",
    "am_adam",
    "am_echo",
    "am_eric",
    "am_fenrir",
    "am_liam",
    "am_michael",
    "am_onyx",
    "am_puck",
    "am_santa",
    "bf_alice",
    "bf_emma",
    "bf_isabella",
    "bf_lily",
    "bm_daniel",
    "bm_fable",
    "bm_george",
    "bm_lewis",
    "ef_dora",
    "em_alex",
    "em_santa",
    "ff_siwis",
    "hf_alpha",
    "hf_beta",
    "hm_omega",
    "hm_psi",
    "if_sara",
    "im_nicola",
    "jf_alpha",
    "jf_gongitsune",
    "jf_nezumi",
    "jf_tebukuro",
    "jm_kumo",
    "pf_dora",
    "pm_alex",
    "pm_santa",
    "zf_xiaobei",
    "zf_xiaoni",
    "zf_xiaoxiao",
    "zf_xiaoyi",
    "zm_yunjian",
    "zm_yunxi",
    "zm_yunxia",
    "zm_yunyang",
];

const KOKORO_BRIDGE_SCRIPT: &str = include_str!("../python/kokoro_bridge.py");

// CoquiTTS XTTS Bridge Script (inline)
#[cfg(feature = "coqui-tts")]
pub const COQUI_BRIDGE_SCRIPT: &str = r#"
import json
import sys
import wave
import os

SAMPLE_RATE_FALLBACK = 24000

def _write_silence(path, sample_rate):
    with wave.open(path, "wb") as wav:
        wav.setnchannels(1)
        wav.setsampwidth(2)
        wav.setframerate(sample_rate)
        wav.writeframes(b"")

def main():
    payload = json.load(sys.stdin)
    text = payload.get("text", "")
    output_path = payload["output"]
    sample_rate = int(payload.get("sample_rate") or SAMPLE_RATE_FALLBACK)

    if not text.strip():
        _write_silence(output_path, sample_rate)
        sys.stdout.write(json.dumps({"duration": 0.0}))
        sys.stdout.flush()
        return

    try:
        from TTS.api import TTS
        import torch
    except Exception as exc:
        sys.stderr.write(f"failed to import coqui dependencies: {exc}\n")
        sys.stderr.flush()
        sys.exit(1)

    model_name = payload.get("model_name") or "tts_models/multilingual/multi-dataset/xtts_v2"
    device = payload.get("device") or ("cuda" if torch.cuda.is_available() else "cpu")
    language = payload.get("language") or "en"
    speed = float(payload.get("speed", 1.0))
    
    speaker_wav = payload.get("speaker_wav")
    use_voice_cloning = speaker_wav is not None and os.path.exists(speaker_wav)
    
    try:
        tts = TTS(model_name=model_name, progress_bar=False, gpu=device != "cpu")
        
        if use_voice_cloning:
            if not os.path.exists(speaker_wav):
                raise FileNotFoundError(f"Speaker reference audio not found: {speaker_wav}")
            
            tts.tts_to_file(
                text=text,
                file_path=output_path,
                speaker_wav=speaker_wav,
                language=language,
                speed=speed,
            )
        else:
            tts.tts_to_file(
                text=text,
                file_path=output_path,
                language=language,
                speed=speed,
            )
        
        try:
            with wave.open(output_path, "rb") as wav:
                frames = wav.getnframes()
                sample_rate = wav.getframerate()
                duration = float(frames) / float(sample_rate)
        except Exception:
            duration = len(text.split()) / 3.0
        
        sys.stdout.write(json.dumps({"duration": duration, "success": True}))
        sys.stdout.flush()
        
    except Exception as exc:
        sys.stderr.write(f"TTS generation failed: {exc}\n")
        sys.stderr.flush()
        sys.exit(1)

if __name__ == "__main__":
    main()
"#;
