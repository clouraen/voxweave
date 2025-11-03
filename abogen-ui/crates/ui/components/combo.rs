use dioxus::prelude::*;
use crate::services::voices::get_all_kokoro_voices;

/// Combobox component for voice selection
#[component]
pub fn Combo(
    value: Signal<String>,
    options: Vec<&'static str>,
    placeholder: &'static str,
) -> Element {
    let mut is_open = use_signal(|| false);
    let mut search_text = use_signal(|| String::new());
    
    // Get voice information for display
    let all_voices = get_all_kokoro_voices();
    let voice_infos: Vec<_> = options.iter()
        .filter_map(|opt| {
            all_voices.iter().find(|v| v.id == *opt).cloned()
        })
        .collect();
    
    // Get current voice info for display when not searching
    let search_is_empty = search_text.read().is_empty();
    let display_value = if search_is_empty {
        let current_voice_info = voice_infos.iter()
            .find(|v| v.id == value.read().as_str())
            .cloned();
        current_voice_info
            .map(|v| format!("{} ({})", v.display_name, v.language))
            .unwrap_or_else(|| value.read().clone())
    } else {
        search_text.read().clone()
    };

    rsx! {
        div {
            class: "combo",
            style: "position: relative; width: 100%;",
            div {
                style: "display: flex; align-items: center; gap: 8px;",
                input {
                    class: "combo-input",
                    r#type: "text",
                    value: "{display_value}",
                    placeholder: "{placeholder}",
                    oninput: move |e| {
                        let input_val = e.value();
                        *search_text.write() = input_val.clone();
                        // Keep dropdown open while searching
                        *is_open.write() = true;
                    },
                    onfocus: move |_| {
                        *is_open.write() = true;
                    },
                    style: "flex: 1;"
                }
                button {
                    class: "neon-btn",
                    style: "padding: 8px 12px; min-width: auto;",
                    onclick: move |_| {
                        let is_open_val = *is_open.read();
                        *is_open.write() = !is_open_val;
                    },
                    "▾"
                }
            }
            if *is_open.read() {
                div {
                    style: "
                        position: absolute;
                        top: 100%;
                        left: 0;
                        right: 0;
                        margin-top: 4px;
                        background: var(--panel-bg);
                        border: 1px solid var(--neon-magenta);
                        border-radius: 10px;
                        max-height: 300px;
                        overflow-y: auto;
                        z-index: 100;
                        box-shadow: 0 4px 20px rgba(255, 58, 212, 0.3);
                    ",
                    {voice_infos.iter()
                        .filter(|voice_info| {
                            let search = search_text.read().to_lowercase();
                            search.is_empty() ||
                            voice_info.display_name.to_lowercase().contains(&search) ||
                            voice_info.id.to_lowercase().contains(&search) ||
                            voice_info.language.to_lowercase().contains(&search)
                        })
                        .map(|voice_info| {
                            let voice_id = voice_info.id;
                            let display_name = voice_info.display_name.clone();
                            let language = voice_info.language;
                            rsx! {
                                div {
                                    key: "{voice_id}",
                                    style: "
                                        padding: 12px 14px;
                                        cursor: pointer;
                                        color: var(--text);
                                        transition: all 0.1s;
                                        display: flex;
                                        justify-content: space-between;
                                        align-items: center;
                                        border-bottom: 1px solid rgba(255, 58, 212, 0.1);
                                    ",
                                    class: if *value.read() == voice_id { "selected" } else { "" },
                                    onclick: move |_| {
                                        *value.write() = voice_id.to_string();
                                        *is_open.write() = false;
                                        *search_text.write() = String::new();
                                    },
                                    onmouseenter: move |_| {
                                        // Hover effect handled by CSS
                                    },
                                    div {
                                        style: "flex: 1; min-width: 0;",
                                        div {
                                            style: "font-weight: 600; color: var(--neon-cyan); margin-bottom: 4px; font-size: 14px;",
                                            "{display_name}"
                                        }
                                        div {
                                            style: "font-size: 11px; color: var(--neon-magenta);",
                                            "{language}"
                                        }
                                    }
                                    div {
                                        style: "
                                            font-size: 10px;
                                            color: var(--muted);
                                            font-family: monospace;
                                            margin-left: 12px;
                                            opacity: 0.7;
                                        ",
                                        "{voice_id}"
                                    }
                                }
                            }
                        })}
                }
            }
        }
    }
}
