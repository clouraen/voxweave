use dioxus::prelude::*;

/// Cyberpunk Futurist Teleprompter Component
#[component]
pub fn Teleprompter(
    text: String,
    speed: f64,
    is_playing: bool,
    on_scroll: EventHandler<()>,
    on_pause: EventHandler<()>,
    on_resume: EventHandler<()>,
) -> Element {
    let mut scroll_position = use_signal(|| 0.0);
    
    // Update scroll position when playing
    use_effect(move || {
        if is_playing {
            let interval = 16.0; // ~60fps
            let pixels_per_second = speed * 30.0; // Adjust this multiplier as needed
            let pixels_per_frame = pixels_per_second * (interval / 1000.0);
            
            spawn(async move {
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_millis(interval as u64)).await;
                    if is_playing {
                        scroll_position.write().clone_from(&(scroll_position.read().clone() + pixels_per_frame));
                        on_scroll.call(());
                    }
                }
            });
        }
    });

    rsx! {
        div {
            class: "teleprompter-container",
            style: "
                position: relative;
                width: 100%;
                height: 400px;
                background: rgba(10, 15, 26, 0.9);
                border: 1px solid rgba(39,232,255,.35);
                border-radius: 14px;
                box-shadow: 0 0 20px rgba(39,232,255,.25);
                overflow: hidden;
                margin: 20px 0;
            ",
            
            // Mirror mode indicator
            div {
                class: "mirror-indicator",
                style: "
                    position: absolute;
                    top: 10px;
                    right: 10px;
                    background: rgba(39,232,255,.2);
                    border: 1px solid rgba(39,232,255,.5);
                    border-radius: 8px;
                    padding: 4px 8px;
                    font-size: 12px;
                    color: var(--neon-cyan);
                    z-index: 10;
                ",
                "MIRROR MODE"
            }
            
            // Text display area
            div {
                class: "teleprompter-text",
                style: "
                    position: absolute;
                    top: 0;
                    left: 0;
                    width: 100%;
                    padding: 30px;
                    color: var(--text);
                    font-family: 'Courier New', monospace;
                    font-size: 24px;
                    line-height: 1.5;
                    text-align: center;
                    transform: translateY({-scroll_position.read()}px);
                    transition: transform 0.1s linear;
                ",
                "{text}"
            }
            
            // Current line highlight
            div {
                class: "current-line-highlight",
                style: "
                    position: absolute;
                    top: 50%;
                    left: 0;
                    width: 100%;
                    height: 30px;
                    background: rgba(39,232,255,.1);
                    border-top: 1px solid rgba(39,232,255,.3);
                    border-bottom: 1px solid rgba(39,232,255,.3);
                    transform: translateY(-50%);
                    pointer-events: none;
                "
            }
            
            // Controls
            div {
                class: "teleprompter-controls",
                style: "
                    position: absolute;
                    bottom: 20px;
                    left: 50%;
                    transform: translateX(-50%);
                    display: flex;
                    gap: 10px;
                    z-index: 10;
                ",
                NeonButton {
                    primary: true,
                    onclick: move |_| {
                        if is_playing {
                            on_pause.call(());
                        } else {
                            on_resume.call(());
                        }
                    },
                    if is_playing { "PAUSE" } else { "PLAY" }
                }
                
                NeonButton {
                    onclick: move |_| {
                        scroll_position.write().clone_from(&(scroll_position.read().clone() - 50.0));
                    },
                    "REWIND"
                }
                
                NeonButton {
                    onclick: move |_| {
                        scroll_position.write().clone_from(&(scroll_position.read().clone() + 50.0));
                    },
                    "FAST FORWARD"
                }
            }
        }
    }
}