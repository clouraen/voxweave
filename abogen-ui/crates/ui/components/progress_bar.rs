use dioxus::prelude::*;

/// Progress bar component with percentage display
#[component]
pub fn ProgressBar(
    progress: Signal<u8>,
) -> Element {
    let pct = *progress.read();
    rsx! {
        div {
            class: "progress",
            style: "margin: 20px 0;",
            div {
                class: "bar",
                style: "width: {pct}%; display: flex; align-items: center; justify-content: center; color: var(--text); font-size: 12px; font-weight: 600;",
                if pct > 0 {
                    "{pct}%"
                }
            }
        }
    }
}

