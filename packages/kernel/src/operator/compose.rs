use crate::capability_runtime::ProviderInvokeResponse;
use crate::operator::intent::{CapabilityIntent, OperatorTurnResult};
use crate::operator::plan::OperatorPlan;

fn strip_jargon(text: &str) -> String {
    text.replace("Capability Runtime", "Workspace")
        .replace("Window Provider", "Workspace")
        .replace("Application Provider", "Workspace")
        .replace("Notification Provider", "Workspace")
        .replace("Notifications Provider", "Workspace")
        .replace("Browser Provider", "Workspace")
        .replace("Provider Registry", "Workspace")
        .replace("Capability Router", "Workspace")
        .replace("Kernel Operator", "Workspace")
        .replace("Capability Provider", "Workspace")
        .replace("Provider", "Workspace")
        .replace("Runtime", "Workspace")
        .replace("Registry", "Workspace")
        .replace("Router", "Workspace")
        .replace("IPC", "connection")
        .replace("Kernel", "Workspace")
}

/// Compose truthful Conversation reply — no provider jargon.
pub fn compose_user_reply(
    intent: &CapabilityIntent,
    plan: &OperatorPlan,
    results: &[ProviderInvokeResponse],
) -> OperatorTurnResult {
    let last = match results.last() {
        Some(r) => r,
        None => {
            return OperatorTurnResult {
                ok: false,
                message: "Nothing happened.".into(),
                status: Some("empty".into()),
                domain: intent.domain.clone(),
                operation: intent.operation.clone(),
                target: intent.query.clone(),
                preview: None,
                text: None,
                items: None,
                monitors: None,
                composition_id: plan.composition_id.clone(),
            };
        }
    };

    let ok = last.ok;
    let message = if !ok {
        strip_jargon(
            last.message
                .as_deref()
                .unwrap_or("That didn’t work — and I won’t pretend it did."),
        )
    } else {
        match (intent.domain.as_str(), intent.operation.as_str()) {
            ("clipboard", "read") => {
                let text = last.text.as_deref().unwrap_or("");
                if text.is_empty() {
                    "Clipboard is empty (or has no text).".into()
                } else {
                    format!(
                        "Clipboard ({}, {} bytes):\n{}",
                        last.format.as_deref().unwrap_or("text"),
                        last.bytes.unwrap_or(0),
                        last.preview.as_deref().unwrap_or(text)
                    )
                }
            }
            ("clipboard", "write") => strip_jargon(
                last.message
                    .as_deref()
                    .unwrap_or("Copied."),
            ),
            ("window", "enumerate") => {
                let count = last.items.as_ref().map(|i| i.len()).unwrap_or(0);
                if count == 0 {
                    "No open windows found.".into()
                } else {
                    let titles = last
                        .items
                        .as_ref()
                        .map(|items| {
                            items
                                .iter()
                                .take(12)
                                .map(|i| format!("• {}", i.title))
                                .collect::<Vec<_>>()
                                .join("\n")
                        })
                        .unwrap_or_default();
                    format!("Open windows ({count}):\n{titles}")
                }
            }
            ("window", "monitors") => {
                let lines = last
                    .monitors
                    .as_ref()
                    .map(|monitors| {
                        monitors
                            .iter()
                            .map(|m| {
                                format!(
                                    "• Monitor {}: {}{}",
                                    m.index,
                                    m.name,
                                    if m.is_primary { " (primary)" } else { "" }
                                )
                            })
                            .collect::<Vec<_>>()
                            .join("\n")
                    })
                    .unwrap_or_else(|| "(none)".into());
                format!(
                    "{}\n{lines}",
                    last.message.as_deref().unwrap_or("Monitors attached.")
                )
            }
            ("application", "enumerate") => {
                let titles = last
                    .items
                    .as_ref()
                    .map(|items| {
                        items
                            .iter()
                            .take(12)
                            .map(|i| format!("• {}", i.title))
                            .collect::<Vec<_>>()
                            .join("\n")
                    })
                    .unwrap_or_else(|| "(none)".into());
                format!(
                    "{}\n{titles}",
                    last.message.as_deref().unwrap_or("Applications listed.")
                )
            }
            ("notifications", "status") => strip_jargon(
                last.message
                    .as_deref()
                    .unwrap_or("I checked desktop notification support."),
            ),
            ("notifications", "show") => {
                if ok {
                    strip_jargon(
                        last.message
                            .as_deref()
                            .unwrap_or("Desktop notification shown."),
                    )
                } else {
                    strip_jargon(
                        last.message
                            .as_deref()
                            .unwrap_or("I couldn’t show that notification."),
                    )
                }
            }
            ("notifications", "dismiss") => strip_jargon(
                last.message
                    .as_deref()
                    .unwrap_or("I couldn’t dismiss that notification."),
            ),
            ("browser", "status") => {
                let browsers = last.text.as_deref().unwrap_or("your system default");
                if ok {
                    format!(
                        "Here’s what I can use on this PC: {browsers}."
                    )
                } else {
                    strip_jargon(
                        last.message
                            .as_deref()
                            .unwrap_or("I can’t reach a browser on this PC right now."),
                    )
                }
            }
            ("browser", "open") => strip_jargon(
                last.message
                    .as_deref()
                    .unwrap_or(if ok {
                        "Opened in your browser."
                    } else {
                        "I couldn’t open that website."
                    }),
            ),
            ("browser", "open_beside") => {
                if ok {
                    let beside = intent.title.as_deref().unwrap_or("the other window");
                    format!("Opened beside “{beside}”.")
                } else {
                    strip_jargon(
                        last.message
                            .as_deref()
                            .unwrap_or("I couldn’t place that beside the other window."),
                    )
                }
            }
            _ => strip_jargon(
                last.message
                    .as_deref()
                    .or(last.text.as_deref())
                    .unwrap_or(if ok {
                        "Done."
                    } else {
                        "That action didn’t succeed."
                    }),
            ),
        }
    };

    OperatorTurnResult {
        ok,
        message,
        status: last.status.clone(),
        domain: intent.domain.clone(),
        operation: intent.operation.clone(),
        target: last.target.clone().or_else(|| intent.query.clone()),
        preview: last.preview.clone(),
        text: last.text.clone(),
        items: last.items.clone(),
        monitors: last.monitors.clone(),
        composition_id: plan.composition_id.clone(),
    }
}
