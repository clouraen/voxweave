/// Audio Recorder Component
/// Captures audio from microphone and saves as .wav file for voice cloning

use dioxus::prelude::*;
use super::NeonButton;
use std::path::PathBuf;

#[cfg(feature = "coqui-tts")]
use std::sync::{Arc, Mutex};

#[cfg(feature = "coqui-tts")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

#[cfg(feature = "coqui-tts")]
use hound::{WavWriter, WavSpec};

#[derive(Debug, Clone, PartialEq)]
pub enum RecordingState {
    Idle,
    Recording,
    Processing,
    Ready(PathBuf),
    Error(String),
}

/// Record audio from microphone to WAV file
#[cfg(feature = "coqui-tts")]
fn record_audio_to_file(output_path: PathBuf, duration_secs: u64) -> Result<(), String> {
    
    // Get default audio host and input device
    let host = cpal::default_host();
    let device = host.default_input_device()
        .ok_or_else(|| "No input device available".to_string())?;
    
    // Get default input configuration
    let config = device.default_input_config()
        .map_err(|e| format!("Failed to get default input config: {}", e))?;
    
    let sample_rate = config.sample_rate().0;
    let channels = config.channels();
    
    // Create WAV spec
    let spec = WavSpec {
        channels,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    
    // Create WAV writer
    let writer = WavWriter::create(&output_path, spec)
        .map_err(|e| format!("Failed to create WAV writer: {}", e))?;
    let writer = Arc::new(Mutex::new(Some(writer)));
    let writer_clone = Arc::clone(&writer);
    
    // Build audio stream
    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => {
            device.build_input_stream(
                &config.into(),
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    if let Ok(mut guard) = writer_clone.lock() {
                        if let Some(writer) = guard.as_mut() {
                            for &sample in data {
                                let amplitude = (sample * i16::MAX as f32) as i16;
                                let _ = writer.write_sample(amplitude);
                            }
                        }
                    }
                },
                |err| eprintln!("Audio stream error: {}", err),
                None,
            )
        }
        cpal::SampleFormat::I16 => {
            device.build_input_stream(
                &config.into(),
                move |data: &[i16], _: &cpal::InputCallbackInfo| {
                    if let Ok(mut guard) = writer_clone.lock() {
                        if let Some(writer) = guard.as_mut() {
                            for &sample in data {
                                let _ = writer.write_sample(sample);
                            }
                        }
                    }
                },
                |err| eprintln!("Audio stream error: {}", err),
                None,
            )
        }
        cpal::SampleFormat::U16 => {
            device.build_input_stream(
                &config.into(),
                move |data: &[u16], _: &cpal::InputCallbackInfo| {
                    if let Ok(mut guard) = writer_clone.lock() {
                        if let Some(writer) = guard.as_mut() {
                            for &sample in data {
                                let amplitude = (sample as i32 - 32768) as i16;
                                let _ = writer.write_sample(amplitude);
                            }
                        }
                    }
                },
                |err| eprintln!("Audio stream error: {}", err),
                None,
            )
        }
        _ => {
            return Err("Unsupported sample format".to_string());
        }
    }.map_err(|e| format!("Failed to build input stream: {}", e))?;
    
    // Start recording
    stream.play()
        .map_err(|e| format!("Failed to start recording: {}", e))?;
    
    // Record for specified duration
    std::thread::sleep(std::time::Duration::from_secs(duration_secs));
    
    // Stop recording
    drop(stream);
    
    // Finalize WAV file
    if let Ok(mut guard) = writer.lock() {
        if let Some(w) = guard.take() {
            w.finalize()
                .map_err(|e| format!("Failed to finalize WAV file: {}", e))?;
        }
    }
    
    Ok(())
}

/// Audio recorder component for voice cloning
#[component]
#[cfg(feature = "coqui-tts")]
pub fn AudioRecorder(
    on_audio_captured: EventHandler<String>,
    recording_state: Signal<RecordingState>,
) -> Element {
    let mut is_recording = use_signal(|| false);
    let audio_path = use_signal::<Option<String>>(|| None);

    rsx! {
        div {
            class: "panel neon-border",
            style: "
                padding: 20px;
                margin-top: 20px;
            ",
            h3 {
                style: "
                    color: var(--neon-cyan);
                    margin-bottom: 16px;
                ",
                "🎤 Voice Cloning Recorder"
            }
            
            p {
                style: "
                    color: var(--muted);
                    font-size: 14px;
                    margin-bottom: 20px;
                ",
                "Record your voice for voice cloning. Speak clearly for best results."
            }
            
            div {
                style: "
                    display: flex;
                    flex-direction: column;
                    gap: 12px;
                ",
                if !*is_recording.read() {
                    div {
                        NeonButton {
                            primary: true,
                            disabled: None,
                            onclick: move |_| {
                                *is_recording.write() = true;
                                // Start recording
                                dioxus::prelude::spawn({
                                    let mut is_recording = is_recording;
                                    let mut audio_path = audio_path;
                                    let on_audio_captured = on_audio_captured.clone();
                                    let mut recording_state = recording_state;
                                    async move {
                                        *recording_state.write() = RecordingState::Recording;
                                        
                                        // Generate output path
                                        let output_dir = directories::UserDirs::new()
                                            .and_then(|dirs| dirs.desktop_dir().map(|d| d.to_path_buf()))
                                            .unwrap_or_else(|| PathBuf::from("./output"));
                                        
                                        let timestamp = std::time::SystemTime::now()
                                            .duration_since(std::time::UNIX_EPOCH)
                                            .unwrap()
                                            .as_secs();
                                        
                                        let audio_file = output_dir.join(format!("voice_clone_{}.wav", timestamp));
                                        
                                        // Create output directory
                                        std::fs::create_dir_all(&output_dir).ok();
                                        
                                        // Record audio in a separate thread
                                        let audio_file_clone = audio_file.clone();
                                        let record_result = tokio::task::spawn_blocking(move || {
                                            record_audio_to_file(audio_file_clone, 5)
                                        }).await;
                                        
                                        match record_result {
                                            Ok(Ok(())) => {
                                                let audio_path_str = audio_file.to_string_lossy().to_string();
                                                *audio_path.write() = Some(audio_path_str.clone());
                                                *recording_state.write() = RecordingState::Ready(audio_file.clone());
                                                *is_recording.write() = false;
                                                
                                                on_audio_captured.call(audio_path_str);
                                            }
                                            Ok(Err(e)) => {
                                                *recording_state.write() = RecordingState::Error(e);
                                                *is_recording.write() = false;
                                            }
                                            Err(e) => {
                                                *recording_state.write() = RecordingState::Error(format!("Task failed: {}", e));
                                                *is_recording.write() = false;
                                            }
                                        }
                                    }
                                });
                            },
                            "🎙️ Start Recording"
                        }
                    }
                } else {
                    div {
                        style: "
                            text-align: center;
                            padding: 20px;
                        ",
                        div {
                            style: "
                                width: 60px;
                                height: 60px;
                                border-radius: 50%;
                                background: var(--neon-red);
                                margin: 0 auto 16px;
                                display: flex;
                                align-items: center;
                                justify-content: center;
                                animation: pulse 1s infinite;
                            ",
                            span {
                                style: "font-size: 24px;",
                                "⏸"
                            }
                        }
                        p {
                            style: "color: var(--neon-cyan); font-size: 16px;",
                            "Recording... Speak now!"
                        }
                        NeonButton {
                            primary: false,
                            disabled: None,
                            onclick: move |_| {
                                *is_recording.write() = false;
                                *recording_state.write() = RecordingState::Idle;
                            },
                            "⏹ Stop"
                        }
                    }
                }
                
                if let Some(path) = audio_path.read().as_ref() {
                    div {
                        style: "
                            margin-top: 16px;
                            padding: 12px;
                            background: var(--bg-secondary);
                            border: 1px dashed var(--accent);
                            border-radius: 4px;
                        ",
                        p {
                            style: "
                                color: var(--neon-cyan);
                                font-size: 12px;
                                margin-bottom: 8px;
                            ",
                            "✅ Audio captured:"
                        }
                        p {
                            style: "
                                color: var(--muted);
                                font-size: 11px;
                                word-break: break-all;
                            ",
                            "{path}"
                        }
                    }
                }
            }
        }
    }
}

/// Simplified audio recorder without cpal dependency (web version)
#[component]
#[cfg(not(feature = "coqui-tts"))]
pub fn AudioRecorder(
    on_audio_captured: EventHandler<String>,
    recording_state: Signal<RecordingState>,
) -> Element {
    rsx! {
        div {
            class: "panel neon-border",
            style: "
                padding: 20px;
                margin-top: 20px;
            ",
            p {
                style: "color: var(--muted);",
                "Audio recording requires the coqui-tts feature to be enabled."
            }
        }
    }
}

