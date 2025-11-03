use dioxus::prelude::*;

/// Checkbox component
#[component]
pub fn CheckBox(
    checked: Signal<bool>,
    label: &'static str,
) -> Element {
    rsx! {
        label {
            class: "checkbox-wrapper",
            input {
                class: "checkbox-input",
                r#type: "checkbox",
                checked: "{checked.read()}",
                onchange: move |e| {
                    *checked.write() = e.checked();
                },
            }
            span {
                style: "color: var(--text); font-size: 14px;",
                "{label}"
            }
        }
    }
}

