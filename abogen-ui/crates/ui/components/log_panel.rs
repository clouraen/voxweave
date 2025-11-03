use dioxus::prelude::*;
use crate::state::{LogEntry, LogLevel};

/// Log panel component for displaying processing logs
#[component]
pub fn LogPanel(
    logs: Signal<Vec<LogEntry>>,
) -> Element {
    rsx! {
        div {
            class: "panel",
            style: "
                max-height: 400px;
                overflow-y: auto;
                font-family: 'Courier New', monospace;
                padding: 16px;
            ",
            for log in logs.read().iter() {
                div {
                    class: "log-entry {log.level.class_name()}",
                    "{log.message}"
                }
            }
            if logs.read().is_empty() {
                div {
                    style: "color: var(--muted); font-size: 13px;",
                    "No logs yet..."
                }
            }
        }
    }
}

impl LogLevel {
    fn class_name(&self) -> &'static str {
        match self {
            LogLevel::Info => "info",
            LogLevel::Notice => "notice",
            LogLevel::Error => "error",
        }
    }
}

