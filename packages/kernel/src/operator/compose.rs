use crate::capability_runtime::ProviderInvokeResponse;
use crate::error::KernelError;
use crate::operator::intent::{CapabilityIntent, OperatorTurnResult};
use crate::operator::plan::OperatorPlan;

fn strip_jargon(text: &str) -> String {
    text.replace("Capability Runtime", "Workspace")
        .replace("Window Provider", "Workspace")
        .replace("Application Provider", "Workspace")
        .replace("Notification Provider", "Workspace")
        .replace("Notifications Provider", "Workspace")
        .replace("Browser Provider", "Workspace")
        .replace("Screenshot Provider", "Workspace")
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

/// Owner-facing failure / clarify language — never pass through engineering strings.
/// Diagnostics must log the original error elsewhere; this is Conversation-only.
pub fn sanitize_owner_message(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return "That didn’t work — and I won’t pretend it did.".into();
    }
    let stripped = strip_jargon(trimmed);
    let lower = stripped.to_ascii_lowercase();

    // Already conversational (clarify / first-person / truthful refusal).
    if stripped.starts_with("Which ")
        || stripped.starts_with("What ")
        || stripped.starts_with("Beside ")
        || stripped.starts_with('I')
        || lower.contains("won’t invent")
        || lower.contains("won't invent")
        || lower.contains("won’t pretend")
        || lower.contains("won't pretend")
        || lower.contains("won’t fake")
        || lower.contains("won't fake")
    {
        return stripped;
    }

    if lower.contains("requires a query") {
        return "Which app or window did you mean?".into();
    }
    if lower.contains("requires text") || lower.contains("clipboard write") {
        return "What text should I copy?".into();
    }
    if lower.contains("notification failed")
        || lower.contains("show notification")
        || (lower.contains("notification") && lower.contains("failed"))
    {
        return "I couldn’t show that notification.".into();
    }
    if lower.contains("screenshot") && (lower.contains("fail") || lower.contains("error")) {
        return "I couldn’t capture that.".into();
    }
    if lower.contains("browser") && (lower.contains("fail") || lower.contains("error")) {
        return "I couldn’t open that website.".into();
    }
    if lower.contains("does not support")
        || lower.contains("not valid for domain")
        || lower.contains("unsupported domain")
        || lower.contains("unknown capability")
        || lower.contains("unknown operation")
    {
        return "I can’t do that on the desktop yet — and I won’t invent it.".into();
    }
    if lower.contains("an unknown error")
        || lower.contains("unknown_error")
        || lower == "that action failed."
    {
        return "That didn’t work — and I won’t pretend it did.".into();
    }
    if lower.contains("not permitted") || lower.contains("permission denied") {
        return "I can’t do that without permission.".into();
    }
    if lower.contains("still initializing") || lower.contains("not ready") {
        return "Workspace is still starting — try again in a moment.".into();
    }
    // Engineering residue: HRESULT, paths with ::, "Error:", "failed:"
    if stripped.contains("::")
        || lower.contains("hresult")
        || lower.contains("0x")
        || lower.contains("winrt")
        || lower.contains("error:")
        || lower.contains("failed:")
        || lower.contains("panic")
    {
        return "That didn’t work — and I won’t pretend it did. Try again in a moment.".into();
    }

    stripped
}

fn domain_failure_fallback(domain: &str) -> &'static str {
    match domain.trim().to_ascii_lowercase().as_str() {
        "notifications" | "notification" | "notify" => "I couldn’t show that notification.",
        "screenshots" | "screenshot" | "capture" => "I couldn’t capture that.",
        "browser" | "web" => "I couldn’t open that website.",
        "clipboard" => "I couldn’t use the clipboard just now.",
        "window" => "I couldn’t change that window just now.",
        "application" | "app" => "I couldn’t open or focus that app just now.",
        _ => "That didn’t work — and I won’t pretend it did.",
    }
}

fn looks_conversational(message: &str) -> bool {
    let lower = message.to_ascii_lowercase();
    message.starts_with('I')
        || message.starts_with("Which ")
        || message.starts_with("What ")
        || message.starts_with("Beside ")
        || lower.contains("won’t invent")
        || lower.contains("won't invent")
        || lower.contains("won’t pretend")
        || lower.contains("won't pretend")
        || lower.contains("won’t fake")
        || lower.contains("won't fake")
        || lower.starts_with("workspace is still")
}

/// Compose Owner-visible failure from a KernelError (P17.S1).
/// Caller must log the original error for diagnostics before/after this call.
pub fn compose_failure_reply(intent: &CapabilityIntent, error: &KernelError) -> OperatorTurnResult {
    let message = match error {
        KernelError::CapabilityRuntime { message } => sanitize_owner_message(message),
        KernelError::WindowsIntegration { message } => {
            let sanitized = sanitize_owner_message(message);
            if looks_conversational(&sanitized) {
                sanitized
            } else {
                domain_failure_fallback(&intent.domain).into()
            }
        }
        KernelError::PermissionDenied(_) => "I can’t do that without permission.".into(),
        KernelError::NotReady => {
            "Workspace is still starting — try again in a moment.".into()
        }
        other => {
            let sanitized = sanitize_owner_message(&other.to_public().message);
            if looks_conversational(&sanitized) {
                sanitized
            } else {
                domain_failure_fallback(&intent.domain).into()
            }
        }
    };

    OperatorTurnResult {
        ok: false,
        message,
        status: Some("failed".into()),
        domain: intent.domain.clone(),
        operation: intent.operation.clone(),
        target: intent.query.clone(),
        preview: None,
        text: None,
        items: None,
        monitors: None,
        composition_id: None,
    }
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
        sanitize_owner_message(
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
            ("browser", "open_foreground") => {
                if ok {
                    let focus = intent
                        .title
                        .as_deref()
                        .or(intent.query.as_deref())
                        .unwrap_or("it");
                    format!("Opened {focus} and brought it to the front.")
                } else {
                    strip_jargon(
                        last.message
                            .as_deref()
                            .unwrap_or("I couldn’t open that and bring it forward."),
                    )
                }
            }
            ("application", "open_maximize") => {
                if ok {
                    let target = intent.query.as_deref().unwrap_or("that app");
                    format!("Opened “{target}” full size.")
                } else {
                    strip_jargon(
                        last.message
                            .as_deref()
                            .unwrap_or("I couldn’t open that full size."),
                    )
                }
            }
            ("window", "focus_minimize") => {
                if ok {
                    let target = intent.query.as_deref().unwrap_or("that window");
                    format!("Found “{target}” and minimized it.")
                } else {
                    strip_jargon(
                        last.message
                            .as_deref()
                            .unwrap_or("I couldn’t find that window to minimize."),
                    )
                }
            }
            ("screenshots", "status") => strip_jargon(
                last.message
                    .as_deref()
                    .unwrap_or(if ok {
                        "I can take screenshots on this PC."
                    } else {
                        "I can’t take screenshots on this PC right now."
                    }),
            ),
            ("screenshots", "capture_desktop")
            | ("screenshots", "capture_window")
            | ("screenshots", "capture_monitor")
            | ("screenshots", "save_png")
            | ("screenshots", "copy_clipboard")
            | ("screenshots", "capture_and_copy") => {
                let mut reply = strip_jargon(
                    last.message
                        .as_deref()
                        .unwrap_or(if ok {
                            "Screenshot ready."
                        } else {
                            "I couldn’t capture that."
                        }),
                );
                if ok {
                    if let Some(path) = last.text.as_deref().filter(|p| !p.is_empty()) {
                        if !reply.to_ascii_lowercase().contains("saved")
                            && intent.operation != "copy_clipboard"
                            && !path.starts_with("memory://")
                        {
                            reply = format!("{reply} Saved to {path}.");
                        }
                    }
                }
                reply
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability_runtime::{CapabilityDomainId, CapabilityOperation};

    fn intent(domain: &str, operation: &str) -> CapabilityIntent {
        CapabilityIntent {
            domain: domain.into(),
            operation: operation.into(),
            ..Default::default()
        }
    }

    #[test]
    fn sanitizes_notification_failed_strings() {
        let msg = sanitize_owner_message("Notification failed: show notification: WinRT boom");
        assert!(msg.contains("couldn’t show") || msg.contains("couldn't show"));
        assert!(!msg.to_ascii_lowercase().contains("winrt"));
        assert!(!msg.to_ascii_lowercase().contains("notification failed"));
    }

    #[test]
    fn sanitizes_engineering_plan_strings() {
        assert_eq!(
            sanitize_owner_message("application operation requires a query"),
            "Which app or window did you mean?"
        );
        assert_eq!(
            sanitize_owner_message("clipboard write requires text"),
            "What text should I copy?"
        );
    }

    #[test]
    fn preserves_clarify_questions() {
        assert_eq!(
            sanitize_owner_message("Which website should I open?"),
            "Which website should I open?"
        );
    }

    #[test]
    fn never_emits_unknown_error_phrase() {
        let msg = sanitize_owner_message("An unknown error occurred.");
        assert!(!msg.to_ascii_lowercase().contains("unknown error"));
    }

    #[test]
    fn compose_failure_from_windows_integration_uses_domain_voice() {
        let result = compose_failure_reply(
            &intent("notifications", "show"),
            &KernelError::WindowsIntegration {
                message: "Notification failed: show notification: 0x80004005".into(),
            },
        );
        assert!(!result.ok);
        assert!(
            result.message.contains("couldn’t show") || result.message.contains("couldn't show")
        );
        assert!(!result.message.contains("0x"));
        assert_eq!(result.status.as_deref(), Some("failed"));
    }

    #[test]
    fn soft_fail_provider_message_is_sanitized() {
        let plan = OperatorPlan {
            composition_id: None,
            steps: vec![],
        };
        let results = [ProviderInvokeResponse {
            domain: CapabilityDomainId::notifications(),
            operation: CapabilityOperation::Show,
            ok: false,
            format: None,
            bytes: None,
            text: None,
            preview: None,
            message: Some("Notification failed: show notification: boom".into()),
            status: Some("error".into()),
            target: None,
            items: None,
            monitors: None,
        }];
        let turn = compose_user_reply(&intent("notifications", "show"), &plan, &results);
        assert!(!turn.ok);
        assert!(!turn.message.to_ascii_lowercase().contains("notification failed"));
    }
}
