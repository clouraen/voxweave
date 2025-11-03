use dioxus::prelude::*;
use crate::state::AppState;

/// Queue management modal component
#[component]
pub fn QueueModal(state: AppState, is_open: Signal<bool>) -> Element {
    let queue = state.queue;
    
    rsx! {
        if *is_open.read() {
            div {
                style: "
                    position: fixed;
                    top: 0;
                    left: 0;
                    right: 0;
                    bottom: 0;
                    background: rgba(0, 0, 0, 0.8);
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    z-index: 1000;
                ",
                onclick: move |_| {
                    *is_open.write() = false;
                },
                div {
                    class: "panel",
                    style: "
                        max-width: 600px;
                        width: 90%;
                        max-height: 80vh;
                        overflow-y: auto;
                        position: relative;
                    ",
                    onclick: move |e| {
                        e.stop_propagation();
                    },
                    div {
                        style: "
                            display: flex;
                            justify-content: space-between;
                            align-items: center;
                            margin-bottom: 20px;
                            padding-bottom: 12px;
                            border-bottom: 1px solid var(--accent);
                        ",
                        h2 {
                            style: "
                                margin: 0;
                                font-size: 20px;
                                font-weight: 600;
                                color: var(--neon-cyan);
                            ",
                            "Queue Management"
                        }
                        button {
                            class: "neon-btn",
                            style: "
                                padding: 8px 12px;
                                min-width: auto;
                                font-size: 18px;
                            ",
                            onclick: move |_| {
                                *is_open.write() = false;
                            },
                            "✕"
                        }
                    }
                    
                    if queue.read().is_empty() {
                        div {
                            style: "
                                text-align: center;
                                padding: 40px 20px;
                                color: var(--muted);
                            ",
                            "Queue is empty"
                        }
                    } else {
                        div {
                            style: "display: flex; flex-direction: column; gap: 12px;",
                            for (index, item) in queue.read().iter().enumerate() {
                                div {
                                    key: "{index}",
                                    class: "panel",
                                    style: "
                                        padding: 12px;
                                        display: flex;
                                        justify-content: space-between;
                                        align-items: center;
                                        gap: 12px;
                                    ",
                                    div {
                                        style: "flex: 1; min-width: 0;",
                                        div {
                                            style: "
                                                font-weight: 600;
                                                color: var(--neon-cyan);
                                                margin-bottom: 4px;
                                                overflow: hidden;
                                                text-overflow: ellipsis;
                                                white-space: nowrap;
                                            ",
                                            "{item.file.name}"
                                        }
                                        div {
                                            style: "
                                                font-size: 12px;
                                                color: var(--muted);
                                            ",
                                            "Voice: {item.voice} | Speed: {item.speed:.2}x | {item.voice_format.as_str()} | {item.subtitle_format.as_str()}"
                                        }
                                    }
                                    div {
                                        style: "
                                            display: flex;
                                            gap: 8px;
                                            align-items: center;
                                        ",
                                        if index > 0 {
                                            button {
                                                class: "neon-btn",
                                                style: "padding: 6px 8px; min-width: auto;",
                                                title: "Move up",
                                                onclick: move |_| {
                                                    let mut queue = queue;
                                                    let mut items = queue.read().clone();
                                                    items.swap(index, index - 1);
                                                    *queue.write() = items;
                                                },
                                                "↑"
                                            }
                                        }
                                        if index < queue.read().len() - 1 {
                                            button {
                                                class: "neon-btn",
                                                style: "padding: 6px 8px; min-width: auto;",
                                                title: "Move down",
                                                onclick: move |_| {
                                                    let mut queue = queue;
                                                    let mut items = queue.read().clone();
                                                    items.swap(index, index + 1);
                                                    *queue.write() = items;
                                                },
                                                "↓"
                                            }
                                        }
                                        button {
                                            class: "neon-btn",
                                            style: "
                                                padding: 6px 8px;
                                                min-width: auto;
                                                background: rgba(255, 0, 0, 0.2);
                                                border-color: #ff0000;
                                            ",
                                            title: "Remove",
                                            onclick: move |_| {
                                                let mut queue = queue;
                                                let mut items = queue.read().clone();
                                                items.remove(index);
                                                *queue.write() = items;
                                            },
                                            "✕"
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    div {
                        style: "
                            margin-top: 20px;
                            padding-top: 20px;
                            border-top: 1px solid var(--accent);
                            display: flex;
                            gap: 12px;
                            justify-content: flex-end;
                        ",
                        button {
                            class: "neon-btn",
                            onclick: move |_| {
                                *is_open.write() = false;
                            },
                            "Close"
                        }
                    }
                }
            }
        }
    }
}

