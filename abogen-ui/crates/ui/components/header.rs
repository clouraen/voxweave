use dioxus::prelude::*;

/// Header component with app logo, version, and caret
#[component]
pub fn Header() -> Element {
    rsx! {
        div {
            class: "panel",
            style: "display: flex; align-items: center; justify-content: space-between; padding: 16px 20px; margin-bottom: 20px;",
            div {
                style: "display: flex; align-items: center; gap: 12px;",
                div {
                    style: "width: 32px; height: 32px; background: linear-gradient(135deg, var(--neon-cyan), var(--neon-magenta)); border-radius: 6px;",
                    // Logo placeholder
                }
                span {
                    style: "font-size: 18px; font-weight: 600; color: var(--text);",
                    "abogen v1.1.0"
                }
            }
            div {
                style: "width: 24px; height: 24px; display: flex; align-items: center; justify-content: center; color: var(--muted);",
                "▾"
            }
        }
    }
}

