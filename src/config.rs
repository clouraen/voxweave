use std::fs;
use std::path::PathBuf;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::tts::VoiceProfile;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    pub replace_single_newlines: bool,
    pub default_voice: Option<String>,
    pub recent_files: Vec<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            replace_single_newlines: false,
            default_voice: None,
            recent_files: Vec::new(),
        }
    }
}

pub fn config_path() -> anyhow::Result<PathBuf> {
    if let Some(overridden) = std::env::var_os("VOXWEAVE_CONFIG_DIR") {
        let base = PathBuf::from(overridden);
        fs::create_dir_all(&base)?;
        return Ok(base.join("config.json"));
    }
    let project_dirs = ProjectDirs::from("com", "voxweave", "voxweave")
        .ok_or_else(|| anyhow::anyhow!("unable to determine configuration directory"))?;
    let path = project_dirs.config_dir().join("config.json");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(path)
}

pub fn load_config() -> anyhow::Result<AppConfig> {
    let path = config_path()?;
    if !path.exists() {
        return Ok(AppConfig::default());
    }
    let data = fs::read_to_string(path)?;
    let cfg = serde_json::from_str(&data)?;
    Ok(cfg)
}

pub fn save_config(cfg: &AppConfig) -> anyhow::Result<()> {
    let path = config_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let data = serde_json::to_string_pretty(cfg)?;
    fs::write(path, data)?;
    Ok(())
}

pub fn resolve_voice<'a>(
    voices: &'a [VoiceProfile],
    config: &AppConfig,
) -> Option<&'a VoiceProfile> {
    if let Some(default_id) = &config.default_voice {
        voices.iter().find(|voice| &voice.id == default_id)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;
    use std::sync::Mutex;

    static CONFIG_TEST_LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

    #[test]
    fn default_config_is_returned_when_missing() {
        let _guard = CONFIG_TEST_LOCK.lock().unwrap();
        let dir = tempfile::tempdir().unwrap();
        unsafe {
            std::env::set_var("VOXWEAVE_CONFIG_DIR", dir.path());
        }
        let cfg = load_config().unwrap();
        assert_eq!(cfg, AppConfig::default());
        unsafe {
            std::env::remove_var("VOXWEAVE_CONFIG_DIR");
        }
    }

    #[test]
    fn config_round_trip_persists_data() {
        let _guard = CONFIG_TEST_LOCK.lock().unwrap();
        let dir = tempfile::tempdir().unwrap();
        unsafe {
            std::env::set_var("VOXWEAVE_CONFIG_DIR", dir.path());
        }

        let mut cfg = AppConfig::default();
        cfg.replace_single_newlines = true;
        cfg.default_voice = Some("test-voice".into());
        cfg.recent_files.push("book.epub".into());

        save_config(&cfg).unwrap();
        let path = config_path().unwrap();
        assert!(path.exists(), "config file should exist after saving");

        let loaded = load_config().unwrap();
        assert_eq!(loaded, cfg);
        unsafe {
            std::env::remove_var("VOXWEAVE_CONFIG_DIR");
        }
    }
}
