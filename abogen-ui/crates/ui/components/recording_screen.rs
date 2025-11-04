use dioxus::prelude::*;
use crate::components::{Teleprompter, GestureControl, NeonButton, ProgressBar};
use crate::state::{AppState, LogLevel};

/// Cyberpunk Futurist Recording Screen Component
#[component]
pub fn RecordingScreen(
    state: AppState,
    on_stop: EventHandler<()>,
    on_pause: EventHandler<()>,
    on_resume: EventHandler<()>,
) -> Element {
    let is_recording = use_signal(|| true);
    let mut is_paused = use_signal(|| false);
    let mut scroll_speed = use_signal(|| 1.0);
    let teleprompter_text = use_signal(|| "Welcome to the Cyberpunk Teleprompter System. This is a sample text that will scroll as you present. You can control the scrolling speed using the slider below. Use the gesture controls or keyboard arrows to navigate.".to_string());
    
    rsx! {
        div {
            class: "container",
            style: "display: flex; flex-direction: column; min-height: 100vh;",
            
            // Header with recording status
            div {
                class: "panel",
                style: "
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    margin-bottom: 20px;
                ",
                h2 {
                    style: "
                        font-size: 24px;
                        font-weight: 700;
                        color: var(--neon-cyan);
                        text-transform: uppercase;
                        letter-spacing: 1px;
                    ",
                    "🔴 LIVE RECORDING"
                }
                
                div {
                    style: "display: flex; gap: 10px;",
                    NeonButton {
                        primary: true,
                        onclick: move |_| on_stop.call(()),
                        "STOP RECORDING"
                    }
                }
            }
            
            // Main content area
            div {
                style: "
                    display: grid;
                    grid-template-columns: 3fr 1fr;
                    gap: 20px;
                    flex: 1;
                ",
                
                // Left column - Teleprompter and controls
                div {
                    style: "display: flex; flex-direction: column; gap: 20px;",
                    
                    // Teleprompter component
                    Teleprompter {
                        text: teleprompter_text.read().clone(),
                        speed: *scroll_speed.read(),
                        is_playing: *is_recording.read() && !*is_paused.read(),
                        on_scroll: move || {},
                        on_pause: move |_| {
                            *is_paused.write() = true;
                            on_pause.call(());
                        },
                        on_resume: move |_| {
                            *is_paused.write() = false;
                            on_resume.call(());
                        }
                    }
                    
                    // Speed control slider
                    div {
                        class: "panel",
                        style: "padding: 15px;",
                        h3 {
                            style: "color: var(--neon-amber); margin-bottom: 10px;",
                            "Scroll Speed: {*scroll_speed.read():.1}x"
                        }
                        input {
                            r#type: "range",
                            min: "0.1",
                            max: "3.0",
                            step: "0.1",
                            value: "{*scroll_speed.read()}",
                            oninput: move |e| {
                                *scroll_speed.write() = e.value().parse().unwrap_or(1.0);
                            },
                            style: "
                                width: 100%;
                                height: 8px;
                                background: linear-gradient(90deg, #FFB300, #27E8FF);
                                border-radius: 4px;
                                outline: none;
                                -webkit-appearance: none;
                            "
                        }
                    }
                    
                    // Gesture controls
                    GestureControl {
                        on_gesture_up: move |_| {
                            // Increase scroll speed
                            let current = *scroll_speed.read();
                            *scroll_speed.write() = (current + 0.1).min(3.0);
                        },
                        on_gesture_down: move |_| {
                            // Decrease scroll speed
                            let current = *scroll_speed.read();
                            *scroll_speed.write() = (current - 0.1).max(0.1);
                        },
                        on_gesture_left: move |_| {
                            // Rewind teleprompter
                        },
                        on_gesture_right: move |_| {
                            // Fast forward teleprompter
                        },
                        on_gesture_ok: move |_| {
                            // Toggle pause/resume
                            if *is_paused.read() {
                                *is_paused.write() = false;
                                on_resume.call(());
                            } else {
                                *is_paused.write() = true;
                                on_pause.call(());
                            }
                        }
                    }
                }
                
                // Right column - Camera preview and recording info
                div {
                    style: "display: flex; flex-direction: column; gap: 20px;",
                    
                    // Camera preview
                    div {
                        class: "panel",
                        style: "
                            flex: 1;
                            display: flex;
                            align-items: center;
                            justify-content: center;
                            background: #000;
                            position: relative;
                        ",
                        div {
                            style: "
                                width: 100%;
                                height: 200px;
                                background: linear-gradient(45deg, #0A0F1A, #1A1022);
                                border: 1px solid rgba(39,232,255,.3);
                                border-radius: 8px;
                                display: flex;
                                align-items: center;
                                justify-content: center;
                                color: var(--neon-cyan);
                            ",
                            "📹 CAMERA PREVIEW"
                        }
                        
                        // Recording indicator
                        div {
                            style: "
                                position: absolute;
                                top: 10px;
                                left: 10px;
                                width: 12px;
                                height: 12px;
                                background: #FF3355;
                                border-radius: 50%;
                                animation: pulse 1s infinite;
                            "
                        }
                    }
                    
                    // Recording stats
                    div {
                        class: "panel",
                        style: "padding: 15px;",
                        h3 {
                            style: "color: var(--neon-magenta); margin-bottom: 10px;",
                            "Recording Stats"
                        }
                        div {
                            style: "font-size: 14px; color: var(--text);",
                            "Duration: 00:02:45"
                        }
                        div {
                            style: "font-size: 14px; color: var(--text); margin-top: 5px;",
                            "File Size: 124.7 MB"
                        }
                    }
                    
                    // Audio levels
                    div {
                        class: "panel",
                        style: "padding: 15px;",
                        h3 {
                            style: "color: var(--neon-lime); margin-bottom: 10px;",
                            "Audio Levels"
                        }
                        div {
                            style: "display: flex; align-items: center; gap: 10px;",
                            div {
                                style: "
                                    flex: 1;
                                    height: 20px;
                                    background: rgba(255,255,255,.1);
                                    border-radius: 10px;
                                    overflow: hidden;
                                ",
                                div {
                                    style: "
                                        height: 100%;
                                        width: 65%;
                                        background: linear-gradient(90deg, #39FF14, #FFB300);
                                        border-radius: 10px;
                                    "
                                }
                            }
                            div {
                                style: "color: var(--text); font-size: 12px;",
                                "65%"
                            }
                        }
                    }
                }
            }
            
            // Progress and logs
            div {
                class: "panel",
                style: "margin-top: 20px;",
                ProgressBar { progress: state.progress.clone() }
                
                div {
                    style: "margin-top: 15px; max-height: 100px; overflow-y: auto;",
                    // This would be connected to actual logs in a real implementation
                    div {
                        class: "log-entry info",
                        "Teleprompter initialized with cyberpunk theme"
                    }
                    div {
                        class: "log-entry info",
                        "Camera input detected: HD 1080p"
                    }
                    div {
                        class: "log-entry notice",
                        "Recording started at 00:02:45"
                    }
                }
            }
        }
        
        // CSS for pulse animation
        style { {"\
            @keyframes pulse {\
                0% { opacity: 1; }\
                50% { opacity: 0.5; }\
                100% { opacity: 1; }\
            }\
        "} }
    }
}