/// Kokoro TTS voice information
#[derive(Debug, Clone)]
pub struct VoiceInfo {
    pub id: &'static str,
    pub display_name: String,
    pub language: &'static str,
    pub engine: &'static str, // "kokoro" or "coqui"
}

impl VoiceInfo {
    pub fn new(id: &'static str) -> Self {
        let lang_code = id.chars().next().unwrap_or('a');
        let lang_name = get_language_name(lang_code);
        let display_name = format_voice_name(id);
        
        Self {
            id,
            display_name,
            language: lang_name,
            engine: "kokoro",
        }
    }
    
    pub fn new_coqui(id: &'static str, display_name: String, language: &'static str) -> Self {
        Self {
            id,
            display_name,
            language,
            engine: "coqui",
        }
    }
}

/// Get all Kokoro TTS voices with language information
pub fn get_all_kokoro_voices() -> Vec<VoiceInfo> {
    KOKORO_VOICES
        .iter()
        .map(|voice_id| VoiceInfo::new(*voice_id))
        .collect()
}

/// Get all Coqui TTS voices
#[cfg(feature = "coqui-tts")]
pub fn get_all_coqui_voices() -> Vec<VoiceInfo> {
    COQUI_VOICES
        .iter()
        .map(|(id, name, lang)| VoiceInfo::new_coqui(*id, name.to_string(), *lang))
        .collect()
}

/// Get all available voices (Kokoro + Coqui if enabled)
pub fn get_all_voices() -> Vec<VoiceInfo> {
    let mut voices = get_all_kokoro_voices();
    
    #[cfg(feature = "coqui-tts")]
    {
        voices.extend(get_all_coqui_voices());
    }
    
    voices
}

/// Get voice info by ID
pub fn get_voice_info(voice_id: &str) -> Option<VoiceInfo> {
    // Check Coqui voices first
    #[cfg(feature = "coqui-tts")]
    {
        if let Some((id, name, lang)) = COQUI_VOICES.iter().find(|(id, _, _)| *id == voice_id) {
            return Some(VoiceInfo::new_coqui(*id, name.to_string(), *lang));
        }
    }
    
    // Check Kokoro voices
    KOKORO_VOICES.iter()
        .find(|&&v| v == voice_id)
        .map(|&static_id| VoiceInfo::new(static_id))
}

/// Format voice ID to display name
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

/// Get language name from language code
fn get_language_name(code: char) -> &'static str {
    match code {
        'a' => "American English",
        'b' => "British English",
        'e' => "Spanish",
        'f' => "French",
        'h' => "Hindi",
        'i' => "Italian",
        'j' => "Japanese",
        'p' => "Brazilian Portuguese",
        'z' => "Mandarin Chinese",
        _ => "Unknown",
    }
}

/// All Kokoro TTS voices
const KOKORO_VOICES: &[&str] = &[
    // American English Female
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
    // American English Male
    "am_adam",
    "am_echo",
    "am_eric",
    "am_fenrir",
    "am_liam",
    "am_michael",
    "am_onyx",
    "am_puck",
    "am_santa",
    // British English Female
    "bf_alice",
    "bf_emma",
    "bf_isabella",
    "bf_lily",
    // British English Male
    "bm_daniel",
    "bm_fable",
    "bm_george",
    "bm_lewis",
    // Spanish Female
    "ef_dora",
    // Spanish Male
    "em_alex",
    "em_santa",
    // French Female
    "ff_siwis",
    // Hindi Female
    "hf_alpha",
    "hf_beta",
    // Hindi Male
    "hm_omega",
    "hm_psi",
    // Italian Female
    "if_sara",
    // Italian Male
    "im_nicola",
    // Japanese Female
    "jf_alpha",
    "jf_gongitsune",
    "jf_nezumi",
    "jf_tebukuro",
    // Japanese Male
    "jm_kumo",
    // Brazilian Portuguese Female
    "pf_dora",
    // Brazilian Portuguese Male
    "pm_alex",
    "pm_santa",
    // Mandarin Chinese Female
    "zf_xiaobei",
    "zf_xiaoni",
    "zf_xiaoxiao",
    "zf_xiaoyi",
    // Mandarin Chinese Male
    "zm_yunjian",
    "zm_yunxi",
    "zm_yunxia",
    "zm_yunyang",
];

/// Coqui TTS voices (ID, Display Name, Language)
#[cfg(feature = "coqui-tts")]
const COQUI_VOICES: &[(&str, &str, &str)] = &[
    ("coqui-en", "Coqui English", "English"),
    ("coqui-es", "Coqui Spanish", "Spanish"),
    ("coqui-fr", "Coqui French", "French"),
    ("coqui-de", "Coqui German", "German"),
    ("coqui-it", "Coqui Italian", "Italian"),
    ("coqui-pt", "Coqui Portuguese", "Portuguese"),
    ("coqui-pl", "Coqui Polish", "Polish"),
    ("coqui-tr", "Coqui Turkish", "Turkish"),
    ("coqui-ru", "Coqui Russian", "Russian"),
    ("coqui-nl", "Coqui Dutch", "Dutch"),
    ("coqui-cs", "Coqui Czech", "Czech"),
    ("coqui-ar", "Coqui Arabic", "Arabic"),
    ("coqui-zh-cn", "Coqui Chinese (Mandarin)", "Chinese"),
    ("coqui-ja", "Coqui Japanese", "Japanese"),
    ("coqui-hu", "Coqui Hungarian", "Hungarian"),
    ("coqui-ko", "Coqui Korean", "Korean"),
];

