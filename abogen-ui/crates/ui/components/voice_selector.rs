use dioxus::prelude::*;
use crate::services::voices::get_all_kokoro_voices;

/// Enhanced voice selector with search and categorization
#[component]
pub fn VoiceSelector(
    value: Signal<String>,
    on_change: Option<EventHandler<String>>,
) -> Element {
    let mut is_open = use_signal(|| false);
    let mut search_text = use_signal(|| String::new());
    let mut selected_category = use_signal(|| "All".to_string());
    
    // Get all voices and categorize them
    let all_voices = get_all_kokoro_voices();
    
    // Extract unique categories (languages)
    let categories: Vec<String> = all_voices
        .iter()
        .map(|v| v.language.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    
    // Filter voices based on search and category
    let filtered_voices: Vec<_> = all_voices
        .iter()
        .filter(|voice| {
            let matches_search = search_text.read().is_empty() || 
                voice.display_name.to_lowercase().contains(&search_text.read().to_lowercase()) ||
                voice.id.to_lowercase().contains(&search_text.read().to_lowercase()) ||
                voice.language.to_lowercase().contains(&search_text.read().to_lowercase());
            
            let matches_category = *selected_category.read() == "All" || 
                voice.language == *selected_category.read();
            
            matches_search && matches_category
        })
        .collect();

    rsx! {
        div {
            class: "voice-selector",
            style: "position: relative; width: 100%;",
            
            div {
                style: "display: flex; align-items: center; gap: 8px;",
                input {
                    class: "combo-input",
                    r#type: "text",
                    value: "{search_text.read()}",
                    placeholder: "Search voices...",
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
            
            // Voice selection dropdown
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
                        z-index: 100;
                        box-shadow: 0 4px 20px rgba(255, 58, 212, 0.3);
                    ",
                    
                    // Category filter
                    div {
                        style: "
                            display: flex;
                            overflow-x: auto;
                            padding: 8px;
                            border-bottom: 1px solid rgba(255,58,212,.1);
                        ",
                        button {
                            class: if *selected_category.read() == "All" { "neon-btn primary" } else { "neon-btn" },
                            style: "padding: 6px 12px; margin-right: 4px; font-size: 12px;",
                            onclick: move |_| {
                                *selected_category.write() = "All".to_string();
                            },
                            "All"
                        }
                        {categories.iter().map(|category| {
                            let category_clone = category.clone();
                            rsx! {
                                button {
                                    class: if *selected_category.read() == *category { "neon-btn primary" } else { "neon-btn" },
                                    style: "padding: 6px 12px; margin-right: 4px; font-size: 12px;",
                                    onclick: move |_| {
                                        *selected_category.write() = category_clone.clone();
                                    },
                                    "{category}"
                                }
                            }
                        })}
                    }
                    
                    // Voice list
                    div {
                        style: "
                            max-height: 300px;
                            overflow-y: auto;
                        ",
                        {filtered_voices.iter()
                            .map(|voice_info| {
                                let voice_id = voice_info.id.clone();
                                let display_name = voice_info.display_name.clone();
                                let language = voice_info.language.clone();
                                let current_value = value.read().clone();
                                let is_selected = current_value == voice_id;
                                
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
                                        class: if is_selected { "selected" } else { "" },
                                        onclick: move |_| {
                                            *value.write() = voice_id.to_string();
                                            if let Some(handler) = on_change {
                                                handler.call(voice_id.to_string());
                                            }
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
}