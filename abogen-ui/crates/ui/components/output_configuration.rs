use dioxus::prelude::*;
use crate::state::{AppState, SubtitleMode, VoiceFormat, SubtitleFormat};

/// Enhanced output configuration with tabbed interface
#[component]
pub fn OutputConfiguration(state: AppState) -> Element {
    let mut active_tab = use_signal(|| "audio".to_string());

    rsx! {
        div {
            class: "panel",
            style: "margin-bottom: 20px;",
            
            // Tab headers
            div {
                style: "
                    display: flex;
                    border-bottom: 1px solid var(--accent);
                    margin-bottom: 20px;
                ",
                button {
                    class: if *active_tab.read() == "audio" { "neon-btn primary" } else { "neon-btn" },
                    style: "border-radius: 8px 8px 0 0; border-bottom: none; margin-right: 2px;",
                    onclick: move |_| *active_tab.write() = "audio".to_string(),
                    "Audio Settings"
                }
                button {
                    class: if *active_tab.read() == "subtitles" { "neon-btn primary" } else { "neon-btn" },
                    style: "border-radius: 8px 8px 0 0; border-bottom: none; margin-right: 2px;",
                    onclick: move |_| *active_tab.write() = "subtitles".to_string(),
                    "Subtitle Settings"
                }
                button {
                    class: if *active_tab.read() == "text" { "neon-btn primary" } else { "neon-btn" },
                    style: "border-radius: 8px 8px 0 0; border-bottom: none;",
                    onclick: move |_| *active_tab.write() = "text".to_string(),
                    "Text Processing"
                }
            }
            
            // Tab content
            match active_tab.read().as_str() {
                "audio" => rsx! {
                    div {
                        style: "display: flex; flex-direction: column; gap: 16px;",
                        
                        div {
                            style: "display: flex; flex-direction: column;",
                            label {
                                style: "font-weight: 600; font-size: 14px; color: var(--text); margin-bottom: 8px;",
                                "Output Format"
                            }
                            select {
                                class: "select",
                                value: "{state.voice_format.read().as_str()}",
                                onchange: move |e| {
                                    *state.voice_format.write() = match e.value().as_str() {
                                        "wav" => VoiceFormat::Wav,
                                        "mp3" => VoiceFormat::Mp3,
                                        "flac" => VoiceFormat::Flac,
                                        _ => VoiceFormat::Wav,
                                    };
                                },
                                option { value: "wav", "WAV (Uncompressed, highest quality)" }
                                option { value: "mp3", "MP3 (Compressed, good quality)" }
                                option { value: "flac", "FLAC (Compressed, lossless)" }
                            }
                        }
                        
                        div {
                            style: "margin-top: 10px; padding: 10px; background: rgba(39,232,255,0.1); border-radius: 8px; border: 1px solid rgba(39,232,255,0.3);",
                            style: "font-size: 12px; color: var(--neon-cyan);",
                            "ℹ️ WAV provides the highest quality but largest file size. MP3 offers good compression with minimal quality loss. FLAC provides lossless compression with smaller file sizes."
                        }
                    }
                },
                "subtitles" => rsx! {
                    div {
                        style: "display: flex; flex-direction: column; gap: 16px;",
                        
                        div {
                            style: "display: flex; flex-direction: column;",
                            label {
                                style: "font-weight: 600; font-size: 14px; color: var(--text); margin-bottom: 8px;",
                                "Subtitle Granularity"
                            }
                            select {
                                class: "select",
                                value: "{state.subtitle_mode.read().as_str()}",
                                onchange: move |e| {
                                    *state.subtitle_mode.write() = match e.value().as_str() {
                                        "Sentence" => SubtitleMode::Sentence,
                                        "Paragraph" => SubtitleMode::Paragraph,
                                        _ => SubtitleMode::None,
                                    };
                                },
                                option { value: "Sentence", "Sentence (One subtitle per sentence)" }
                                option { value: "Paragraph", "Paragraph (One subtitle per paragraph)" }
                                option { value: "None", "Disabled (No subtitles)" }
                            }
                        }
                        
                        div {
                            style: "display: flex; flex-direction: column;",
                            label {
                                style: "font-weight: 600; font-size: 14px; color: var(--text); margin-bottom: 8px;",
                                "Subtitle Format"
                            }
                            select {
                                class: "select",
                                value: "{state.subtitle_format.read().as_str()}",
                                onchange: move |e| {
                                    *state.subtitle_format.write() = match e.value().as_str() {
                                        "ass" => SubtitleFormat::Ass,
                                        "srt" => SubtitleFormat::Srt,
                                        "vtt" => SubtitleFormat::Vtt,
                                        _ => SubtitleFormat::Ass,
                                    };
                                },
                                option { value: "ass", "ASS (Advanced SubStation Alpha)" }
                                option { value: "srt", "SRT (SubRip Subtitle)" }
                                option { value: "vtt", "VTT (Web Video Text Tracks)" }
                            }
                        }
                        
                        div {
                            style: "margin-top: 10px; padding: 10px; background: rgba(39,232,255,0.1); border-radius: 8px; border: 1px solid rgba(39,232,255,0.3);",
                            style: "font-size: 12px; color: var(--neon-cyan);",
                            "ℹ️ ASS supports advanced styling and positioning. SRT is widely compatible with most players. VTT is designed for web video."
                        }
                    }
                },
                _ => rsx! {
                    div {
                        style: "display: flex; flex-direction: column; gap: 16px;",
                        
                        div {
                            style: "display: flex; flex-direction: column;",
                            label {
                                style: "font-weight: 600; font-size: 14px; color: var(--text); margin-bottom: 8px;",
                                "Newline Handling"
                            }
                            select {
                                class: "select",
                                value: if *state.replace_newlines.read() { "replace" } else { "keep" },
                                onchange: move |e| {
                                    *state.replace_newlines.write() = e.value() == "replace";
                                },
                                option { value: "replace", "Replace single newlines with spaces" }
                                option { value: "keep", "Preserve single newlines" }
                            }
                        }
                        
                        div {
                            style: "margin-top: 10px; padding: 10px; background: rgba(39,232,255,0.1); border-radius: 8px; border: 1px solid rgba(39,232,255,0.3);",
                            style: "font-size: 12px; color: var(--neon-cyan);",
                            "ℹ️ Replacing single newlines creates longer sentences for better flow. Preserving newlines maintains the original text structure."
                        }
                    }
                }
            }
        }
    }
}