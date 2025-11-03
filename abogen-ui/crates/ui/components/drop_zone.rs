use dioxus::prelude::*;
use dioxus::html::HasFileData;
use crate::state::{AppState, FileInfo};
use std::path::PathBuf;

/// Drop zone component for file selection
#[component]
pub fn DropZone(state: AppState) -> Element {
    let mut is_dragging = use_signal(|| false);
    let selected_file = state.selected_file;
    let file_input_id = use_signal(|| format!("file-input-{}", std::process::id()));

    // Handle file from path
    let handle_file = move |file_path: PathBuf| {
        if let Ok(metadata) = std::fs::metadata(&file_path) {
            let file_name = file_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();
            
            let file_info = FileInfo {
                name: file_name.clone(),
                path: file_path.to_string_lossy().to_string(),
                size: metadata.len(),
            };
            
            let mut selected_file = state.selected_file;
            *selected_file.write() = Some(file_info);
            
            log::info!("File selected: {} ({})", file_name, metadata.len());
        } else {
            log::warn!("Failed to read file metadata: {}", file_path.display());
        }
    };

    rsx! {
        div {
            style: "position: relative; width: 100%;",
            // Hidden file input
            input {
                id: "{file_input_id.read()}",
                r#type: "file",
                accept: ".txt,.epub,.pdf",
                style: "position: absolute; opacity: 0; width: 0; height: 0; pointer-events: none;",
                onchange: move |evt| {
                    #[cfg(target_arch = "wasm32")]
                    {
                        // Web: files from file input
                        spawn(async move {
                            let files = evt.files();
                            if !files.is_empty() {
                                if let Some(file) = files.first() {
                                    let file_name = file.name().unwrap_or_else(|| "unknown".to_string());
                                    let file_size = file.size().unwrap_or(0);
                                    
                                    let file_info = FileInfo {
                                        name: file_name.clone(),
                                        path: format!("web-file://{}", file_name),
                                        size: file_size,
                                    };
                                    
                                    *state.selected_file.write() = Some(file_info);
                                    log::info!("File selected on web: {}", file_name);
                                }
                            }
                        });
                    }
                    
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        // Desktop: use native file handling
                        let files = evt.files();
                        if !files.is_empty() {
                            if let Some(file) = files.first() {
                                let path = file.path();
                                if path.exists() {
                                    handle_file(path);
                                } else {
                                    log::warn!("Selected file does not exist: {:?}", path);
                                }
                            }
                        }
                    }
                },
            }
            // Clickable label that triggers file input
            label {
                r#for: "{file_input_id.read()}",
                style: "
                    position: absolute;
                    width: 100%;
                    height: 100%;
                    cursor: pointer;
                    z-index: 10;
                ",
            }
            div {
                class: "panel dashed",
                style: format!(
                    "
                    position: relative;
                    min-height: 150px;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    justify-content: center;
                    gap: 12px;
                    cursor: pointer;
                    transition: all 0.2s;
                    border-color: {};
                    box-shadow: {};
                ",
                    if *is_dragging.read() { "var(--neon-cyan)" } else { "var(--neon-magenta)" },
                    if *is_dragging.read() { "0 0 20px rgba(39,232,255,.5)" } else { "none" }
                ),
                ondragover: move |e| {
                    e.stop_propagation();
                    *is_dragging.write() = true;
                },
                ondragleave: move |_| {
                    *is_dragging.write() = false;
                },
                ondrop: move |e| {
                    e.stop_propagation();
                    *is_dragging.write() = false;
                    
                    // Handle file drop
                    // Dioxus 0.7 drag data API
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        // Desktop: get files from drop event
                        let drag_data = e.data();
                        
                        // Extract file paths from drag data
                        // Dioxus 0.7 provides files() method that returns Vec<FileData>
                        let files = drag_data.files();
                        if !files.is_empty() {
                            // Get first file and extract its path
                            if let Some(file_data) = files.first() {
                                // FileData path() returns PathBuf directly
                                let path = file_data.path();
                                if let Ok(canonical_path) = std::fs::canonicalize(&path) {
                                    handle_file(canonical_path);
                                } else if path.exists() {
                                    // Try using the path as-is if it exists
                                    handle_file(path);
                                } else {
                                    log::warn!("File dropped but path does not exist: {:?}", path);
                                }
                            }
                        } else {
                            log::info!("File dropped but no files found in drag data");
                        }
                    }
                    
                    #[cfg(target_arch = "wasm32")]
                    {
                        // Web: handle file drop using FileReader API
                        spawn(async move {
                            // The drop event should provide files via e.data()
                            let drag_data = e.data();
                            let files = drag_data.files();
                            
                            if !files.is_empty() {
                                if let Some(file_data) = files.first() {
                                    // For web, FileData provides file information
                                    // Extract file name and size
                                    let file_name = file_data.name().unwrap_or_else(|| "unknown".to_string());
                                    let file_size = file_data.size().unwrap_or(0);
                                    
                                    let file_info = FileInfo {
                                        name: file_name.clone(),
                                        path: format!("web-file://{}", file_name),
                                        size: file_size,
                                    };
                                    
                                    let mut selected_file = state.selected_file;
                                    *selected_file.write() = Some(file_info);
                                    log::info!("File dropped on web: {}", file_name);
                                }
                            }
                        });
                    }
                },
                label {
                    style: "
                        font-weight: 600; 
                        font-size: 16px; 
                        color: var(--neon-magenta); 
                        text-transform: uppercase; 
                        letter-spacing: 1px;
                        pointer-events: none;
                        z-index: 1;
                    ",
                    "Textbox"
                }
                if let Some(file) = selected_file.read().clone() {
                    div {
                        style: "text-align: center;",
                        div {
                            style: "color: var(--neon-cyan); font-size: 14px; margin-bottom: 4px;",
                            "{file.name}"
                        }
                        div {
                            style: "color: var(--muted); font-size: 12px;",
                            "{format_size(file.size)}"
                        }
                    }
                } else {
                    div {
                        style: "text-align: center; color: var(--text); font-size: 14px;",
                        "Drag and drop your file here or click to browse."
                        br {}
                        span {
                            style: "color: var(--muted); font-size: 12px;",
                            "(.txt, .epub, .pdf)"
                        }
                    }
                }
            }
        }
    }
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}
