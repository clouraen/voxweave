use dioxus::prelude::*;

/// Neon-styled button component
#[component]
pub fn NeonButton(
    onclick: EventHandler<MouseEvent>,
    children: Element,
    #[props(optional)] primary: Option<bool>,
    #[props(optional)] disabled: Option<bool>,
) -> Element {
    let is_primary = primary.unwrap_or(false);
    let is_disabled = disabled.unwrap_or(false);
    
    rsx! {
        button {
            class: if is_primary { "neon-btn primary" } else { "neon-btn" },
            disabled: is_disabled,
            onclick: move |e| {
                if !is_disabled {
                    onclick.call(e);
                }
            },
            {children}
        }
    }
}

