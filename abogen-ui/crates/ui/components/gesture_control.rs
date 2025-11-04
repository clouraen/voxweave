use dioxus::prelude::*;

/// Gesture Recognition Control Component for Teleprompter
#[component]
pub fn GestureControl(
    on_gesture_up: EventHandler<()>,
    on_gesture_down: EventHandler<()>,
    on_gesture_left: EventHandler<()>,
    on_gesture_right: EventHandler<()>,
    on_gesture_ok: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            class: "gesture-control-panel",
            style: "
                background: rgba(10, 15, 26, 0.8);
                border: 1px solid rgba(255,58,212,.35);
                border-radius: 14px;
                padding: 20px;
                margin: 20px 0;
                box-shadow: 0 0 15px rgba(255,58,212,.2);
            ",
            
            h3 {
                style: "
                    color: var(--neon-cyan);
                    text-align: center;
                    margin-bottom: 15px;
                    font-size: 18px;
                ",
                "🎮 Gesture Controls"
            }
            
            div {
                style: "
                    display: grid;
                    grid-template-columns: repeat(3, 1fr);
                    grid-template-rows: repeat(3, 1fr);
                    gap: 10px;
                    max-width: 300px;
                    margin: 0 auto;
                ",
                
                // Empty top-left cell
                div {}
                
                // Up gesture button
                NeonButton {
                    primary: true,
                    onclick: move |_| on_gesture_up.call(()),
                    style: "height: 50px;",
                    "↑"
                }
                
                // Empty top-right cell
                div {}
                
                // Left gesture button
                NeonButton {
                    primary: true,
                    onclick: move |_| on_gesture_left.call(()),
                    style: "height: 50px;",
                    "←"
                }
                
                // OK gesture button (center)
                NeonButton {
                    primary: true,
                    onclick: move |_| on_gesture_ok.call(()),
                    style: "height: 50px;",
                    "OK"
                }
                
                // Right gesture button
                NeonButton {
                    primary: true,
                    onclick: move |_| on_gesture_right.call(()),
                    style: "height: 50px;",
                    "→"
                }
                
                // Empty bottom-left cell
                div {}
                
                // Down gesture button
                NeonButton {
                    primary: true,
                    onclick: move |_| on_gesture_down.call(()),
                    style: "height: 50px;",
                    "↓"
                }
                
                // Empty bottom-right cell
                div {}
            }
            
            div {
                style: "
                    margin-top: 20px;
                    text-align: center;
                    font-size: 14px;
                    color: var(--muted);
                ",
                "Use hand gestures or keyboard arrow keys to control the teleprompter"
            }
        }
    }
}