use dioxus::prelude::*;
use crate::state::AppState;

/// Enhanced video configuration with visual previews
#[component]
pub fn VideoConfiguration(state: AppState) -> Element {
    rsx! {
        div {
            class: "panel",
            style: "
                margin-top: 20px;
                padding-top: 20px;
                border-top: 1px dashed var(--accent);
            ",
            
            // Video Generation Toggle
            div {
                style: "display: flex; align-items: center; margin-bottom: 20px;",
                input {
                    r#type: "checkbox",
                    id: "generate-video",
                    checked: "{state.generate_video.read()}",
                    onchange: move |e| {
                        *state.generate_video.write() = e.value() == "true" || e.value() == "on";
                    },
                    style: "
                        width: 18px;
                        height: 18px;
                        accent-color: var(--neon-cyan);
                        margin-right: 10px;
                    "
                }
                label {
                    r#for: "generate-video",
                    style: "font-weight: 600; font-size: 16px; color: var(--text);",
                    "🎬 Generate Video with Z.AI CogVideoX-3"
                }
            }
            
            // Video Configuration Options (only shown when enabled)
            if *state.generate_video.read() {
                div {
                    style: "display: flex; flex-direction: column; gap: 20px;",
                    
                    // Style Selection with Visual Previews
                    div {
                        style: "display: flex; flex-direction: column;",
                        label {
                            style: "font-weight: 600; font-size: 14px; color: var(--text); margin-bottom: 8px;",
                            "Video Style"
                        }
                        div {
                            style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(120px, 1fr)); gap: 12px;",
                            // Realistic Style
                            div {
                                style: format!(
                                    "
                                    border: 2px solid {};
                                    border-radius: 8px;
                                    padding: 10px;
                                    text-align: center;
                                    cursor: pointer;
                                    transition: all 0.2s;
                                ",
                                    if state.video_style.read().as_str() == "realistic" { "var(--neon-cyan)" } else { "var(--accent)" }
                                ),
                                onclick: move |_| {
                                    *state.video_style.write() = voxweave::queue::VideoStyle::Realistic;
                                },
                                div {
                                    style: "
                                        width: 100%;
                                        height: 60px;
                                        background: linear-gradient(45deg, #8e9eab, #eef2f3);
                                        border-radius: 4px;
                                        margin-bottom: 8px;
                                    "
                                }
                                div {
                                    style: "font-size: 12px; color: var(--text);",
                                    "Realistic"
                                }
                            }
                            // Anime Style
                            div {
                                style: format!(
                                    "
                                    border: 2px solid {};
                                    border-radius: 8px;
                                    padding: 10px;
                                    text-align: center;
                                    cursor: pointer;
                                    transition: all 0.2s;
                                ",
                                    if state.video_style.read().as_str() == "anime" { "var(--neon-cyan)" } else { "var(--accent)" }
                                ),
                                onclick: move |_| {
                                    *state.video_style.write() = voxweave::queue::VideoStyle::Anime;
                                },
                                div {
                                    style: "
                                        width: 100%;
                                        height: 60px;
                                        background: linear-gradient(45deg, #ff9a9e, #fad0c4);
                                        border-radius: 4px;
                                        margin-bottom: 8px;
                                    "
                                }
                                div {
                                    style: "font-size: 12px; color: var(--text);",
                                    "Anime"
                                }
                            }
                            // 3D Style
                            div {
                                style: format!(
                                    "
                                    border: 2px solid {};
                                    border-radius: 8px;
                                    padding: 10px;
                                    text-align: center;
                                    cursor: pointer;
                                    transition: all 0.2s;
                                ",
                                    if state.video_style.read().as_str() == "3d" { "var(--neon-cyan)" } else { "var(--accent)" }
                                ),
                                onclick: move |_| {
                                    *state.video_style.write() = voxweave::queue::VideoStyle::ThreeD;
                                },
                                div {
                                    style: "
                                        width: 100%;
                                        height: 60px;
                                        background: linear-gradient(45deg, #667eea, #764ba2);
                                        border-radius: 4px;
                                        margin-bottom: 8px;
                                    "
                                }
                                div {
                                    style: "font-size: 12px; color: var(--text);",
                                    "3D"
                                }
                            }
                            // Cinematic Style
                            div {
                                style: format!(
                                    "
                                    border: 2px solid {};
                                    border-radius: 8px;
                                    padding: 10px;
                                    text-align: center;
                                    cursor: pointer;
                                    transition: all 0.2s;
                                ",
                                    if state.video_style.read().as_str() == "cinematic" { "var(--neon-cyan)" } else { "var(--accent)" }
                                ),
                                onclick: move |_| {
                                    *state.video_style.write() = voxweave::queue::VideoStyle::Cinematic;
                                },
                                div {
                                    style: "
                                        width: 100%;
                                        height: 60px;
                                        background: linear-gradient(45deg, #1d976c, #93f9b9);
                                        border-radius: 4px;
                                        margin-bottom: 8px;
                                    "
                                }
                                div {
                                    style: "font-size: 12px; color: var(--text);",
                                    "Cinematic"
                                }
                            }
                            // Cyberpunk Style
                            div {
                                style: format!(
                                    "
                                    border: 2px solid {};
                                    border-radius: 8px;
                                    padding: 10px;
                                    text-align: center;
                                    cursor: pointer;
                                    transition: all 0.2s;
                                ",
                                    if state.video_style.read().as_str() == "cyberpunk" { "var(--neon-cyan)" } else { "var(--accent)" }
                                ),
                                onclick: move |_| {
                                    *state.video_style.write() = voxweave::queue::VideoStyle::Cyberpunk;
                                },
                                div {
                                    style: "
                                        width: 100%;
                                        height: 60px;
                                        background: linear-gradient(45deg, #000428, #004e92);
                                        border-radius: 4px;
                                        margin-bottom: 8px;
                                    "
                                }
                                div {
                                    style: "font-size: 12px; color: var(--text);",
                                    "Cyberpunk"
                                }
                            }
                            // Educational Style
                            div {
                                style: format!(
                                    "
                                    border: 2px solid {};
                                    border-radius: 8px;
                                    padding: 10px;
                                    text-align: center;
                                    cursor: pointer;
                                    transition: all 0.2s;
                                ",
                                    if state.video_style.read().as_str() == "educational" { "var(--neon-cyan)" } else { "var(--accent)" }
                                ),
                                onclick: move |_| {
                                    *state.video_style.write() = voxweave::queue::VideoStyle::Educational;
                                },
                                div {
                                    style: "
                                        width: 100%;
                                        height: 60px;
                                        background: linear-gradient(45deg, #f6d365, #fda085);
                                        border-radius: 4px;
                                        margin-bottom: 8px;
                                    "
                                }
                                div {
                                    style: "font-size: 12px; color: var(--text);",
                                    "Educational"
                                }
                            }
                        }
                    }
                    
                    // Resolution and Format Selection
                    div {
                        style: "display: grid; grid-template-columns: 1fr 1fr; gap: 16px;",
                        
                        div {
                            style: "display: flex; flex-direction: column;",
                            label {
                                style: "font-weight: 600; font-size: 14px; color: var(--text); margin-bottom: 8px;",
                                "Resolution"
                            }
                            select {
                                class: "select",
                                value: "{state.video_resolution.read().as_str()}",
                                onchange: move |e| {
                                    *state.video_resolution.write() = match e.value().as_str() {
                                        "720p" => voxweave::queue::VideoResolution::P720,
                                        "1080p" => voxweave::queue::VideoResolution::P1080,
                                        "4k" => voxweave::queue::VideoResolution::P4K,
                                        _ => voxweave::queue::VideoResolution::P1080,
                                    };
                                },
                                option { value: "720p", "720p HD" }
                                option { value: "1080p", "1080p Full HD" }
                                option { value: "4k", "4K Ultra HD" }
                            }
                        }
                        
                        div {
                            style: "display: flex; flex-direction: column;",
                            label {
                                style: "font-weight: 600; font-size: 14px; color: var(--text); margin-bottom: 8px;",
                                "Output Format"
                            }
                            select {
                                class: "select",
                                value: "{state.video_format.read().as_str()}",
                                onchange: move |e| {
                                    *state.video_format.write() = match e.value().as_str() {
                                        "mp4" => voxweave::queue::VideoFormat::Mp4,
                                        "mov" => voxweave::queue::VideoFormat::Mov,
                                        "webm" => voxweave::queue::VideoFormat::Webm,
                                        _ => voxweave::queue::VideoFormat::Mp4,
                                    };
                                },
                                option { value: "mp4", "MP4" }
                                option { value: "mov", "MOV" }
                                option { value: "webm", "WebM" }
                            }
                        }
                    }
                    
                    // Custom Video Prompt
                    div {
                        style: "display: flex; flex-direction: column;",
                        label {
                            style: "font-weight: 600; font-size: 14px; color: var(--text); margin-bottom: 8px;",
                            "Custom Video Prompt (Optional)"
                        }
                        textarea {
                            class: "combo-input",
                            placeholder: "Describe the visual style you want for your video...",
                            value: "{state.video_prompt.read().as_ref().map(|s| s.as_str()).unwrap_or(\"\")}",
                            oninput: move |e| {
                                let val = e.value();
                                if val.is_empty() {
                                    *state.video_prompt.write() = None;
                                } else {
                                    *state.video_prompt.write() = Some(val.clone());
                                }
                            },
                            style: "
                                width: 100%;
                                min-height: 100px;
                                resize: vertical;
                                padding: 12px 16px;
                                background: var(--bg-secondary);
                                border: 1px dashed var(--accent);
                                color: var(--text);
                                font-family: 'Rajdhani', sans-serif;
                                font-size: 14px;
                                border-radius: 8px;
                            "
                        }
                        div {
                            style: "margin-top: 6px; font-size: 12px; color: var(--muted);",
                            "Provide specific visual instructions to guide the AI video generation. For example: 'Create a video with a futuristic cityscape at night, neon lights reflecting on wet streets.'"
                        }
                    }
                    
                    // Advanced Options
                    div {
                        style: "padding-top: 16px; border-top: 1px dashed var(--accent);",
                        div {
                            style: "display: flex; align-items: center;",
                            input {
                                r#type: "checkbox",
                                id: "use-gpu",
                                checked: "{state.use_gpu.read()}",
                                onchange: move |e| {
                                    *state.use_gpu.write() = e.value() == "true" || e.value() == "on";
                                },
                                style: "
                                    width: 18px;
                                    height: 18px;
                                    accent-color: var(--neon-cyan);
                                    margin-right: 10px;
                                "
                            }
                            label {
                                r#for: "use-gpu",
                                style: "font-weight: 600; font-size: 14px; color: var(--text);",
                                "Use GPU Acceleration (if available)"
                            }
                        }
                    }
                }
            }
        }
    }
}