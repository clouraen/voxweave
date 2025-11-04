use dioxus::prelude::*;


pub mod components;
pub mod state;
pub mod theme;
pub mod services;

pub use state::*;
pub use components::*;


/// Root application component
#[component]
pub fn App() -> Element {
    // Inject global styles via style tag
    rsx! {
        style { "{theme::get_theme_css()}" }
        
        div {
            style: "min-height: 100vh; display: flex; flex-direction: column;",
            InnerApp {}
        }
    }
}

#[component]
fn InnerApp() -> Element {
    let app_state = AppState::new();
    let current_screen = use_signal(|| Screen::Main);

    rsx! {
        match *current_screen.read() {
            Screen::Main => rsx! {
                MainScreen {
                    state: app_state.clone(),
                    on_start: {
                        let mut is_processing = app_state.is_processing;
                        let mut progress = app_state.progress;
                        let mut logs = app_state.logs;
                        let mut cancel_token = app_state.cancel_token;
                        let mut current_screen = current_screen;
                        move |_| {
                            // Check if queue is not empty and not already processing
                            if !app_state.queue.read().is_empty() && !*is_processing.read() {
                                // Clear any previous cancel token
                                *cancel_token.write() = None;
                                // Reset processing state
                                *is_processing.write() = true;
                                *progress.write() = 0;
                                logs.write().clear();
                                // Switch to processing screen
                                *current_screen.write() = Screen::Processing;
                                
                                // Spawn processing task
                                dioxus::prelude::spawn({
                                    let state = app_state.clone();
                                    let mut is_processing = state.is_processing;
                                    let mut cancel_token = state.cancel_token;
                                    let mut logs = state.logs;
                                    let queue = app_state.queue.read().clone();
                                    let mut current_screen_clone = current_screen;
                                    async move {
                                    // Validate queue is not empty
                                    if queue.is_empty() {
                                        *is_processing.write() = false;
                                        return;
                                    }
                                    
                                    // Clear cancel token at start of new process
                                    *cancel_token.write() = None;
                                    
                                    // Add a small delay to ensure UI has time to render processing screen
                                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                                    
                                    #[cfg(feature = "real-tts")]
                                    let result = crate::services::tts_service::process_queue(state.clone(), queue).await;
                                    #[cfg(not(feature = "real-tts"))]
                                    let result = crate::services::tts_stub::process_queue(state.clone(), queue).await;
                                    
                                    // Reset processing state
                                    *is_processing.write() = false;
                                    
                                    // Check if cancelled
                                    if cancel_token.read().is_some() {
                                        // Was cancelled - already returned to main screen by cancel handler
                                        *cancel_token.write() = None; // Clear for next time
                                    } else {
                                        // Check if there was an error
                                        if let Err(ref e) = result {
                                            // Log error but don't immediately return to main screen
                                            // Let user see the error in logs first
                                            logs.write().push(crate::state::LogEntry {
                                                message: format!("Processing completed with error: {}", e),
                                                level: crate::state::LogLevel::Error,
                                            });
                                            // Wait longer so user can see the error and logs
                                            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                                        } else {
                                            // Success - log completion message
                                            logs.write().push(crate::state::LogEntry {
                                                message: "Processing completed successfully!".to_string(),
                                                level: crate::state::LogLevel::Info,
                                            });
                                            // Brief delay to show completion message
                                            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                                        }
                                        
                                        // Return to main screen after processing completes
                                        *current_screen_clone.write() = Screen::Main;
                                    }
                                }
                                });
                            }
                        }
                    },
                    on_cancel: {
                        let mut current_screen = current_screen;
                        move |_| {
                            *current_screen.write() = Screen::Main;
                        }
                    },
                }
            },
            Screen::Processing => rsx! {
                ProcessingScreen {
                    state: app_state.clone(),
                    on_cancel: {
                        let mut cancel_token = app_state.cancel_token;
                        let mut is_processing = app_state.is_processing;
                        let mut current_screen = current_screen;
                        move |_| {
                            // Set cancel token to stop processing
                            *cancel_token.write() = Some(());
                            // Reset processing state
                            *is_processing.write() = false;
                            // Return to main screen
                            *current_screen.write() = Screen::Main;
                            // Clear cancel token after a brief delay to allow processing to detect cancellation
                            // The next START will clear it anyway
                        }
                    },
                }
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    Main,
    Processing,
}

/// Main screen component
#[component]
fn MainScreen(
    state: AppState,
    on_start: EventHandler,
    on_cancel: EventHandler,
) -> Element {
    let mut queue_modal_open = use_signal(|| false);
    // All Kokoro TTS voices
    const VOICE_OPTIONS: &[&str] = &[
        // American English Female
        "af_alloy", "af_aoede", "af_bella", "af_heart", "af_jessica",
        "af_kore", "af_nicole", "af_nova", "af_river", "af_sarah", "af_sky",
        // American English Male
        "am_adam", "am_echo", "am_eric", "am_fenrir", "am_liam",
        "am_michael", "am_onyx", "am_puck", "am_santa",
        // British English Female
        "bf_alice", "bf_emma", "bf_isabella", "bf_lily",
        // British English Male
        "bm_daniel", "bm_fable", "bm_george", "bm_lewis",
        // Spanish Female
        "ef_dora",
        // Spanish Male
        "em_alex", "em_santa",
        // French Female
        "ff_siwis",
        // Hindi Female
        "hf_alpha", "hf_beta",
        // Hindi Male
        "hm_omega", "hm_psi",
        // Italian Female
        "if_sara",
        // Italian Male
        "im_nicola",
        // Japanese Female
        "jf_alpha", "jf_gongitsune", "jf_nezumi", "jf_tebukuro",
        // Japanese Male
        "jm_kumo",
        // Brazilian Portuguese Female
        "pf_dora",
        // Brazilian Portuguese Male
        "pm_alex", "pm_santa",
        // Mandarin Chinese Female
        "zf_xiaobei", "zf_xiaoni", "zf_xiaoxiao", "zf_xiaoyi",
        // Mandarin Chinese Male
        "zm_yunjian", "zm_yunxi", "zm_yunxia", "zm_yunyang",
    ];

    rsx! {
        QueueModal { state: state.clone(), is_open: queue_modal_open }
        
        div {
            class: "container",
            Header {}
            
            DropZone { state: state.clone() }
            
            div {
                style: "
                    display: grid;
                    grid-template-columns: repeat(3, 1fr);
                    gap: 12px;
                    margin: 20px 0;
                ",
                NeonButton {
                    onclick: move |_| {
                        let selected_file_val = state.selected_file.read().clone();
                        if let Some(file) = selected_file_val {
                            let item = QueuedItem {
                                file: file.clone(),
                                voice: state.voice.read().clone(),
                                speed: *state.speed.read(),
                                subtitle_mode: *state.subtitle_mode.read(),
                                voice_format: *state.voice_format.read(),
                                subtitle_format: *state.subtitle_format.read(),
                                replace_newlines: *state.replace_newlines.read(),
                                use_gpu: *state.use_gpu.read(),
                                generate_video: *state.generate_video.read(),
                                video_style: *state.video_style.read(),
                                video_resolution: *state.video_resolution.read(),
                                video_format: *state.video_format.read(),
                                video_prompt: state.video_prompt.read().clone(),
                                save_location: state.save_location.read().clone(),
                            };
                            let mut queue = state.queue;
                            queue.write().push(item);
                            let mut selected_file = state.selected_file;
                            *selected_file.write() = None;
                        }
                    },
                    "Add to Queue"
                }
                NeonButton {
                    onclick: move |_| {
                        *queue_modal_open.write() = true;
                    },
                    "Manage Queue"
                }
                NeonButton {
                    onclick: move |_| {
                        state.queue.write().clear();
                    },
                    "Clear Queue"
                }
            }
            
            div {
                class: "panel",
                style: "margin-bottom: 20px;",
                Slider {
                    value: state.speed.clone(),
                    min: 0.5,
                    max: 2.0,
                    step: 0.05,
                }
                
                div {
                    style: "
                        display: grid;
                        grid-template-columns: 1fr auto auto;
                        gap: 12px;
                        align-items: center;
                        margin-top: 20px;
                    ",
                    label {
                        style: "font-weight: 600; font-size: 14px; color: var(--text);",
                        "Select voice"
                    }
                    Combo {
                        value: state.voice.clone(),
                        options: VOICE_OPTIONS.to_vec(),
                        placeholder: "af_heart",
                    }
                    button {
                        class: "neon-btn",
                        style: "padding: 8px 12px; min-width: auto;",
                        title: "Tuning",
                        "⚙"
                    }
                    button {
                        class: "neon-btn",
                        style: "padding: 8px 12px; min-width: auto;",
                        title: "Play",
                        "▶"
                    }
                }
                
                div {
                    style: "display: flex; flex-direction: column; gap: 16px; margin-top: 20px;",
                    
                    // Generate subtitles
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
                        option { value: "Sentence", "Sentence" }
                        option { value: "Paragraph", "Paragraph" }
                        option { value: "None", "None" }
                    }
                    
                    // Output voice format
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
                        option { value: "wav", "wav" }
                        option { value: "mp3", "mp3" }
                        option { value: "flac", "flac" }
                    }
                    
                    // Output subtitle format
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
                        option { value: "ass", "ass (centered narrow)" }
                        option { value: "srt", "srt" }
                        option { value: "vtt", "vtt" }
                    }
                    
                    // Replace single newlines
                    select {
                        class: "select",
                        value: if *state.replace_newlines.read() { "Enabled" } else { "Disabled" },
                        onchange: move |e| {
                            *state.replace_newlines.write() = e.value() == "Enabled";
                        },
                        option { value: "Enabled", "Enabled" }
                        option { value: "Disabled", "Disabled" }
                    }
                    
                    // Save location
                    select {
                        class: "select",
                        value: match &*state.save_location.read() {
                            SaveLocation::Desktop => "Desktop",
                            SaveLocation::Custom(_) => "Choose...",
                        },
                        onchange: move |e| {
                            let mut save_location = state.save_location;
                            match e.value().as_str() {
                                "Desktop" => {
                                    *save_location.write() = SaveLocation::Desktop;
                                }
                                "Choose" => {
                                    // Open directory picker for custom save location
                                    #[cfg(not(target_arch = "wasm32"))]
                                    {
                                        spawn(async move {
                                            // Use Dioxus 0.7 directory picker API
                                            // Note: API may vary by version - this is a placeholder
                                            // For now, prompt user or use a text input
                                            log::info!("Directory picker not available in Dioxus 0.7 - using Desktop as default");
                                            // In a real implementation, would use platform-specific APIs
                                            // For now, keep Desktop selected
                                            let mut save_location = state.save_location;
                                            *save_location.write() = SaveLocation::Desktop;
                                        });
                                    }
                                    
                                    #[cfg(target_arch = "wasm32")]
                                    {
                                        // Web: directory selection is limited
                                        // Use a text input or prompt for path
                                        log::info!("Custom location selection on web - would prompt for path");
                                        let mut save_location = state.save_location;
                                        *save_location.write() = SaveLocation::Desktop;
                                    }
                                }
                                _ => {
                                    *save_location.write() = SaveLocation::Desktop;
                                }
                            }
                        },
                        option { value: "Desktop", "Save to Desktop" }
                        option { value: "Choose", "Choose…" }
                    }
                }
                
                CheckBox {
                    checked: state.use_gpu.clone(),
                    label: "Use GPU Acceleration (if available)",
                }
                
                // Video Generation Section
                div {
                    style: "
                        margin-top: 20px;
                        padding-top: 20px;
                        border-top: 1px dashed var(--accent);
                    ",
                    CheckBox {
                        checked: state.generate_video.clone(),
                        label: "🎬 Generate Video with Z.AI CogVideoX-3",
                    }
                    
                    if *state.generate_video.read() {
                        div {
                            style: "
                                display: grid;
                                grid-template-columns: repeat(3, 1fr);
                                gap: 12px;
                                margin-top: 16px;
                            ",
                            // Video Style
                            select {
                                class: "select",
                                value: "{state.video_style.read().as_str()}",
                                onchange: move |e| {
                                    *state.video_style.write() = match e.value().as_str() {
                                        "realistic" => VideoStyle::Realistic,
                                        "anime" => VideoStyle::Anime,
                                        "3d" => VideoStyle::ThreeD,
                                        "cinematic" => VideoStyle::Cinematic,
                                        "biotech" => VideoStyle::Biotech,
                                        "cyberpunk" => VideoStyle::Cyberpunk,
                                        "educational" => VideoStyle::Educational,
                                        _ => VideoStyle::Realistic,
                                    };
                                },
                                option { value: "realistic", "Realistic" }
                                option { value: "anime", "Anime" }
                                option { value: "3d", "3D" }
                                option { value: "cinematic", "Cinematic" }
                                option { value: "biotech", "🎬 BioTech Futurista" }
                                option { value: "cyberpunk", "⚡ Cyberpunk Neon" }
                                option { value: "educational", "📚 Educacional Limpo" }
                            }
                            
                            // Video Resolution
                            select {
                                class: "select",
                                value: "{state.video_resolution.read().as_str()}",
                                onchange: move |e| {
                                    *state.video_resolution.write() = match e.value().as_str() {
                                        "720p" => VideoResolution::P720,
                                        "1080p" => VideoResolution::P1080,
                                        "4k" => VideoResolution::P4K,
                                        _ => VideoResolution::P1080,
                                    };
                                },
                                option { value: "720p", "720p HD" }
                                option { value: "1080p", "1080p Full HD" }
                                option { value: "4k", "4K Ultra HD" }
                            }
                            
                            // Video Format
                            select {
                                class: "select",
                                value: "{state.video_format.read().as_str()}",
                                onchange: move |e| {
                                    *state.video_format.write() = match e.value().as_str() {
                                        "mp4" => VideoFormat::Mp4,
                                        "mov" => VideoFormat::Mov,
                                        "webm" => VideoFormat::Webm,
                                        _ => VideoFormat::Mp4,
                                    };
                                },
                                option { value: "mp4", "MP4" }
                                option { value: "mov", "MOV" }
                                option { value: "webm", "WebM" }
                            }
                        }
                        
                        // Custom Video Prompt
                        textarea {
                            class: "combo-input",
                            placeholder: "Custom video generation prompt (optional)",
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
                                min-height: 80px;
                                resize: vertical;
                                margin-top: 12px;
                                padding: 10px 14px;
                                background: var(--bg-secondary);
                                border: 1px dashed var(--accent);
                                color: var(--text);
                                font-family: 'Rajdhani', sans-serif;
                                font-size: 14px;
                                border-radius: 4px;
                            "
                        }
                    }
                }
            }
            
            div {
                style: "
                    display: flex;
                    justify-content: center;
                    margin: 30px 0;
                    flex-direction: column;
                    align-items: center;
                    gap: 10px;
                ",
                NeonButton {
                    primary: true,
                    disabled: Some(state.queue.read().is_empty() || *state.is_processing.read()),
                    onclick: move |_| {
                        on_start.call(());
                    },
                    "START"
                }
                div {
                    style: "
                        font-size: 12px;
                        color: var(--muted);
                        text-align: center;
                    ",
                    if state.queue.read().is_empty() {
                        "Add items to queue to start"
                    } else if *state.is_processing.read() {
                        "Processing in progress..."
                    } else {
                        "Ready to process {state.queue.read().len()} item(s)"
                    }
                }
            }
        }
    }
}

/// Processing screen component
#[component]
fn ProcessingScreen(
    state: AppState,
    on_cancel: EventHandler,
) -> Element {
    rsx! {
        div {
            class: "container",
            style: "display: flex; flex-direction: column; min-height: 100vh;",
            
            div {
                class: "panel",
                style: "flex: 1; margin-bottom: 20px;",
                h2 {
                    style: "
                        font-size: 24px;
                        font-weight: 700;
                        color: var(--neon-cyan);
                        margin-bottom: 20px;
                        text-transform: uppercase;
                        letter-spacing: 1px;
                    ",
                    "Processing..."
                }
                
                ProgressBar { progress: state.progress.clone() }
                
                LogPanel { logs: state.logs.clone() }
            }
            
            div {
                style: "
                    display: flex;
                    justify-content: center;
                    padding: 20px 0;
                ",
                NeonButton {
                    onclick: move |_| {
                        on_cancel.call(());
                    },
                    "CANCEL"
                }
            }
        }
    }
}

