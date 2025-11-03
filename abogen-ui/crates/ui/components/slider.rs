use dioxus::prelude::*;

/// Speed slider component with numeric readout
#[component]
pub fn Slider(
    value: Signal<f64>,
    min: f64,
    max: f64,
    step: f64,
) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 16px; width: 100%;",
            label {
                style: "font-weight: 600; font-size: 14px; color: var(--text); min-width: 60px;",
                "Speed"
            }
            input {
                r#type: "range",
                min: "{min}",
                max: "{max}",
                step: "{step}",
                value: "{value.read()}",
                    oninput: move |e| {
                        if let Ok(val) = e.value().parse::<f64>() {
                            *value.write() = val;
                        }
                    },
                style: "
                    flex: 1;
                    height: 4px;
                    background: linear-gradient(90deg, #FFB300, #27E8FF);
                    border-radius: 999px;
                    outline: none;
                    -webkit-appearance: none;
                ",
            }
            span {
                style: "
                    min-width: 50px;
                    text-align: right;
                    font-family: 'Courier New', monospace;
                    color: var(--neon-cyan);
                    font-size: 14px;
                    font-weight: 600;
                ",
                "{value.read():.2}x"
            }
        }
    }
}

