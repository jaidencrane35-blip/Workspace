use crate::capability_runtime::{
    CapabilityDomainId, CapabilityOperation, ProviderInvokeResponse,
};
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
        return "That didn’t work.".into();
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
        // Retire defensive invent/pretend chorus while preserving honesty.
        if lower.contains("won't invent")
            || lower.contains("won’t invent")
            || lower.contains("won't pretend")
            || lower.contains("won’t pretend")
            || lower.contains("won't fake")
            || lower.contains("won’t fake")
        {
            let cleaned = stripped
                .replace(" — and I won’t invent it.", ".")
                .replace(" — and I won't invent it.", ".")
                .replace(" — and I won’t pretend it did.", ".")
                .replace(" — and I won't pretend it did.", ".")
                .replace(" — and I won’t invent a program name.", ".")
                .replace(" — and I won't invent a program name.", ".")
                .replace(" and I won’t invent it.", ".")
                .replace(" and I won't invent it.", ".")
                .replace(" and I won’t pretend it did.", ".")
                .replace(" and I won't pretend it did.", ".");
            return cleaned;
        }
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
        return "I can’t do that on the desktop yet.".into();
    }
    if lower.contains("an unknown error")
        || lower.contains("unknown_error")
        || lower == "that action failed."
    {
        return "That didn’t work.".into();
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
        return "That didn’t work. Try again in a moment.".into();
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
        _ => "That didn’t work.",
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

fn step_ok(results: &[ProviderInvokeResponse], index: usize) -> bool {
    results.get(index).is_some_and(|r| r.ok)
}

fn beside_layout_verified(
    enumerate: &ProviderInvokeResponse,
    beside: &str,
) -> bool {
    let Some(items) = enumerate.items.as_ref() else {
        return false;
    };
    let beside_l = beside.to_ascii_lowercase();
    let Some(beside_win) = items
        .iter()
        .find(|i| i.title.to_ascii_lowercase().contains(&beside_l) && !i.minimized)
    else {
        return false;
    };
    // Observable side-by-side: another visible window on the same monitor,
    // horizontally offset (snap left/right produces distinct x).
    items.iter().any(|other| {
        other.hwnd != beside_win.hwnd
            && !other.minimized
            && other.monitor_index == beside_win.monitor_index
            && (other.x - beside_win.x).abs() >= 80
    })
}

/// Compose under the Completion Contract for known multi-step compositions.
fn compose_completion(
    intent: &CapabilityIntent,
    plan: &OperatorPlan,
    results: &[ProviderInvokeResponse],
) -> Option<(bool, String, &'static str)> {
    let composition = plan.composition_id.as_deref()?;
    match composition {
        "browser.open_beside" => {
            let beside = intent.title.as_deref().unwrap_or("the other window");
            let open_ok = step_ok(results, 0);
            let locate_ok = step_ok(results, 1);
            let snap_beside_ok = step_ok(results, 2);
            let snap_opened_ok = step_ok(results, 3);
            let verify_ok = results
                .get(4)
                .is_some_and(|r| r.ok && beside_layout_verified(r, beside));
            let layout_ok =
                open_ok && locate_ok && snap_beside_ok && snap_opened_ok && verify_ok;

            if layout_ok {
                return Some((
                    true,
                    format!("Opened beside “{beside}”."),
                    "completed",
                ));
            }
            if !open_ok {
                let msg = results
                    .first()
                    .and_then(|r| r.message.as_deref())
                    .unwrap_or("I couldn’t open that website.");
                return Some((false, sanitize_owner_message(msg), "failed"));
            }
            if !locate_ok {
                return Some((
                    false,
                    format!(
                        "Opened the site, but I couldn’t find “{beside}” to place beside."
                    ),
                    "partial",
                ));
            }
            if !snap_beside_ok || !snap_opened_ok {
                return Some((
                    false,
                    format!(
                        "Opened the site and found “{beside}”, but I couldn’t finish the side-by-side layout."
                    ),
                    "partial",
                ));
            }
            // Snaps reported ok but desktop observation did not confirm layout.
            Some((
                false,
                format!(
                    "Opened the site next to “{beside}”, but I couldn’t confirm the side-by-side layout."
                ),
                "partial",
            ))
        }
        "browser.open_foreground" => {
            let focus = intent
                .title
                .as_deref()
                .or(intent.query.as_deref())
                .unwrap_or("it");
            let open_ok = step_ok(results, 0);
            let focus_ok = step_ok(results, 1);
            if open_ok && focus_ok {
                return Some((
                    true,
                    format!("Opened {focus} and brought it to the front."),
                    "completed",
                ));
            }
            if open_ok && !focus_ok {
                return Some((
                    false,
                    format!("Opened the site, but I couldn’t bring “{focus}” to the front."),
                    "partial",
                ));
            }
            let msg = results
                .first()
                .and_then(|r| r.message.as_deref())
                .unwrap_or("I couldn’t open that and bring it forward.");
            Some((false, sanitize_owner_message(msg), "failed"))
        }
        "window.focus_minimize" => {
            let target = intent.query.as_deref().unwrap_or("that window");
            let focus_ok = step_ok(results, 0);
            let min_ok = step_ok(results, 1);
            if focus_ok && min_ok {
                return Some((
                    true,
                    format!("Found “{target}” and minimized it."),
                    "completed",
                ));
            }
            if focus_ok && !min_ok {
                return Some((
                    false,
                    format!("Found “{target}”, but I couldn’t minimize it."),
                    "partial",
                ));
            }
            let msg = results
                .first()
                .and_then(|r| r.message.as_deref())
                .unwrap_or("I couldn’t find that window to minimize.");
            Some((false, sanitize_owner_message(msg), "failed"))
        }
        "screenshots.capture_and_copy" => {
            let capture_ok = step_ok(results, 0);
            let copy_ok = step_ok(results, 1);
            if capture_ok && copy_ok {
                let mut reply = results
                    .get(1)
                    .and_then(|r| r.message.clone())
                    .or_else(|| results.first().and_then(|r| r.message.clone()))
                    .unwrap_or_else(|| "Screenshot copied.".into());
                reply = strip_jargon(&reply);
                if let Some(path) = results
                    .first()
                    .and_then(|r| r.text.as_deref())
                    .filter(|p| !p.is_empty() && !p.starts_with("memory://"))
                {
                    if !reply.to_ascii_lowercase().contains("saved") {
                        reply = format!("{reply} Saved to {path}.");
                    }
                }
                return Some((true, reply, "completed"));
            }
            if capture_ok && !copy_ok {
                return Some((
                    false,
                    "I captured the screen, but I couldn’t copy it to the clipboard.".into(),
                    "partial",
                ));
            }
            let msg = results
                .first()
                .and_then(|r| r.message.as_deref())
                .unwrap_or("I couldn’t capture that.");
            Some((false, sanitize_owner_message(msg), "failed"))
        }
        "app.open_maximize" => {
            let target = intent.query.as_deref().unwrap_or("that app");
            // Special execute path: find, focus|launch, maximize
            let open_ok = results.get(1).is_some_and(|r| r.ok);
            let max_ok = results.get(2).is_some_and(|r| r.ok);
            if open_ok && max_ok {
                return Some((
                    true,
                    format!("Opened “{target}” full size."),
                    "completed",
                ));
            }
            if open_ok && !max_ok {
                return Some((
                    false,
                    format!("Opened “{target}”, but I couldn’t make it full size."),
                    "partial",
                ));
            }
            let msg = results
                .last()
                .and_then(|r| r.message.as_deref())
                .unwrap_or("I couldn’t open that full size.");
            Some((false, sanitize_owner_message(msg), "failed"))
        }
        "desktop.open_compound" => {
            let labels = intent
                .title
                .as_deref()
                .unwrap_or("")
                .split(" and ")
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();
            let expected = intent
                .text
                .as_deref()
                .unwrap_or("")
                .split('|')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .count()
                .max(labels.len());

            let mut units_ok = 0usize;
            let mut i = 0usize;
            while i < results.len() {
                let r = &results[i];
                if r.domain == CapabilityDomainId::application()
                    && r.operation == CapabilityOperation::Find
                {
                    if let Some(next) = results.get(i + 1) {
                        if next.ok {
                            units_ok += 1;
                        }
                        i += 2;
                        continue;
                    }
                }
                if r.domain == CapabilityDomainId::browser()
                    && r.operation == CapabilityOperation::Open
                {
                    if r.ok {
                        units_ok += 1;
                    }
                    i += 1;
                    continue;
                }
                i += 1;
            }

            let label_list = if labels.len() >= 2 {
                format!(
                    "{} and {}",
                    labels[..labels.len() - 1]
                        .iter()
                        .map(|l| format!("“{l}”"))
                        .collect::<Vec<_>>()
                        .join(", "),
                    format!("“{}”", labels[labels.len() - 1])
                )
            } else if let Some(one) = labels.first() {
                format!("“{one}”")
            } else {
                "those apps".into()
            };

            if expected > 0 && units_ok >= expected {
                return Some((
                    true,
                    format!("Opened {label_list}."),
                    "completed",
                ));
            }
            if units_ok > 0 {
                return Some((
                    false,
                    format!(
                        "Opened {units_ok} of {expected} — I couldn’t finish opening everything you named."
                    ),
                    "partial",
                ));
            }
            Some((
                false,
                "I couldn’t open what you asked for.".into(),
                "failed",
            ))
        }
        _ => None,
    }
}

/// Compose truthful Conversation reply — no provider jargon.
/// Multi-step compositions follow the Capability Completion Contract.
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

    if let Some((ok, message, status)) = compose_completion(intent, plan, results) {
        return OperatorTurnResult {
            ok,
            message,
            status: Some(status.into()),
            domain: intent.domain.clone(),
            operation: intent.operation.clone(),
            target: last.target.clone().or_else(|| intent.query.clone()),
            preview: last.preview.clone(),
            text: last.text.clone(),
            items: last.items.clone(),
            monitors: last.monitors.clone(),
            composition_id: plan.composition_id.clone(),
        };
    }

    let ok = last.ok;
    let message = if !ok {
        sanitize_owner_message(
            last.message
                .as_deref()
                .unwrap_or("That didn’t work."),
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
            // Composed ops are handled by compose_completion. Fallback must not overclaim.
            ("browser", "open_beside") => {
                if ok {
                    "Opened the site.".into()
                } else {
                    strip_jargon(
                        last.message
                            .as_deref()
                            .unwrap_or("I couldn’t place that beside the other window."),
                    )
                }
            }
            ("browser", "open_foreground") => strip_jargon(
                last.message
                    .as_deref()
                    .unwrap_or(if ok {
                        "Opened in your browser."
                    } else {
                        "I couldn’t open that and bring it forward."
                    }),
            ),
            ("application", "open_maximize") => strip_jargon(
                last.message
                    .as_deref()
                    .unwrap_or(if ok {
                        "Opened that app."
                    } else {
                        "I couldn’t open that full size."
                    }),
            ),
            ("window", "focus_minimize") => strip_jargon(
                last.message
                    .as_deref()
                    .unwrap_or(if ok {
                        "Minimized that window."
                    } else {
                        "I couldn’t find that window to minimize."
                    }),
            ),
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

    fn ok_resp(
        domain: CapabilityDomainId,
        operation: CapabilityOperation,
        message: &str,
    ) -> ProviderInvokeResponse {
        ProviderInvokeResponse {
            domain,
            operation,
            ok: true,
            format: None,
            bytes: None,
            text: None,
            preview: None,
            message: Some(message.into()),
            status: Some("ok".into()),
            target: None,
            items: None,
            monitors: None,
        }
    }

    fn fail_resp(
        domain: CapabilityDomainId,
        operation: CapabilityOperation,
        message: &str,
    ) -> ProviderInvokeResponse {
        ProviderInvokeResponse {
            domain,
            operation,
            ok: false,
            format: None,
            bytes: None,
            text: None,
            preview: None,
            message: Some(message.into()),
            status: Some("error".into()),
            target: None,
            items: None,
            monitors: None,
        }
    }

    #[test]
    fn open_beside_never_claims_layout_after_open_only() {
        let mut beside_intent = intent("browser", "open_beside");
        beside_intent.title = Some("Cursor".into());
        beside_intent.path = Some("https://chatgpt.com".into());
        let plan = OperatorPlan {
            composition_id: Some("browser.open_beside".into()),
            steps: vec![],
        };
        let results = [ok_resp(
            CapabilityDomainId::browser(),
            CapabilityOperation::Open,
            "Opened.",
        )];
        let turn = compose_user_reply(&beside_intent, &plan, &results);
        assert!(!turn.ok);
        assert_eq!(turn.status.as_deref(), Some("partial"));
        assert!(!turn.message.to_ascii_lowercase().contains("opened beside"));
        assert!(turn.message.to_ascii_lowercase().contains("couldn’t find")
            || turn.message.to_ascii_lowercase().contains("couldn't find"));
    }

    #[test]
    fn open_beside_claims_beside_only_when_layout_verified() {
        use crate::capability_runtime::ApplicationWindowItem;
        let mut beside_intent = intent("browser", "open_beside");
        beside_intent.title = Some("Cursor".into());
        let plan = OperatorPlan {
            composition_id: Some("browser.open_beside".into()),
            steps: vec![],
        };
        let enumerate = ProviderInvokeResponse {
            domain: CapabilityDomainId::window(),
            operation: CapabilityOperation::Enumerate,
            ok: true,
            format: None,
            bytes: None,
            text: None,
            preview: None,
            message: Some("listed".into()),
            status: Some("ok".into()),
            target: None,
            items: Some(vec![
                ApplicationWindowItem {
                    hwnd: "1".into(),
                    title: "Cursor".into(),
                    process_id: 1,
                    minimized: false,
                    focused: true,
                    x: 0,
                    y: 0,
                    width: 960,
                    height: 1080,
                    monitor_index: Some(0),
                },
                ApplicationWindowItem {
                    hwnd: "2".into(),
                    title: "ChatGPT - Chrome".into(),
                    process_id: 2,
                    minimized: false,
                    focused: false,
                    x: 960,
                    y: 0,
                    width: 960,
                    height: 1080,
                    monitor_index: Some(0),
                },
            ]),
            monitors: None,
        };
        let results = [
            ok_resp(
                CapabilityDomainId::browser(),
                CapabilityOperation::Open,
                "Opened.",
            ),
            ok_resp(
                CapabilityDomainId::window(),
                CapabilityOperation::Focus,
                "Focused.",
            ),
            ok_resp(
                CapabilityDomainId::window(),
                CapabilityOperation::Snap,
                "Snapped left.",
            ),
            ok_resp(
                CapabilityDomainId::window(),
                CapabilityOperation::Snap,
                "Snapped right.",
            ),
            enumerate,
        ];
        let turn = compose_user_reply(&beside_intent, &plan, &results);
        assert!(turn.ok);
        assert_eq!(turn.status.as_deref(), Some("completed"));
        assert!(turn.message.contains("Opened beside"));
        assert!(turn.message.contains("Cursor"));
    }

    #[test]
    fn open_beside_partial_when_snaps_fail() {
        let mut beside_intent = intent("browser", "open_beside");
        beside_intent.title = Some("Cursor".into());
        let plan = OperatorPlan {
            composition_id: Some("browser.open_beside".into()),
            steps: vec![],
        };
        let results = [
            ok_resp(
                CapabilityDomainId::browser(),
                CapabilityOperation::Open,
                "Opened.",
            ),
            ok_resp(
                CapabilityDomainId::window(),
                CapabilityOperation::Focus,
                "Focused.",
            ),
            fail_resp(
                CapabilityDomainId::window(),
                CapabilityOperation::Snap,
                "snap failed",
            ),
        ];
        let turn = compose_user_reply(&beside_intent, &plan, &results);
        assert!(!turn.ok);
        assert_eq!(turn.status.as_deref(), Some("partial"));
        assert!(!turn.message.to_ascii_lowercase().contains("opened beside"));
        assert!(turn.message.to_ascii_lowercase().contains("side-by-side"));
    }

    #[test]
    fn compound_open_reports_partial_when_second_fails() {
        let mut intent = intent("application", "open_compound");
        intent.text = Some("app:Cursor|app:Google Chrome".into());
        intent.title = Some("Cursor and Google Chrome".into());
        let plan = OperatorPlan {
            composition_id: Some("desktop.open_compound".into()),
            steps: vec![],
        };
        let results = [
            ok_resp(
                CapabilityDomainId::application(),
                CapabilityOperation::Find,
                "found",
            ),
            ok_resp(
                CapabilityDomainId::application(),
                CapabilityOperation::Focus,
                "focused",
            ),
            ok_resp(
                CapabilityDomainId::application(),
                CapabilityOperation::Find,
                "found",
            ),
            fail_resp(
                CapabilityDomainId::application(),
                CapabilityOperation::Launch,
                "launch failed",
            ),
        ];
        let turn = compose_user_reply(&intent, &plan, &results);
        assert!(!turn.ok);
        assert_eq!(turn.status.as_deref(), Some("partial"));
        assert!(turn.message.to_ascii_lowercase().contains("1 of 2"));
    }

    #[test]
    fn capture_and_copy_partial_when_copy_fails() {
        let plan = OperatorPlan {
            composition_id: Some("screenshots.capture_and_copy".into()),
            steps: vec![],
        };
        let results = [
            ok_resp(
                CapabilityDomainId::screenshots(),
                CapabilityOperation::CaptureDesktop,
                "Captured.",
            ),
            fail_resp(
                CapabilityDomainId::screenshots(),
                CapabilityOperation::CopyClipboard,
                "copy failed",
            ),
        ];
        let turn = compose_user_reply(&intent("screenshots", "capture_and_copy"), &plan, &results);
        assert!(!turn.ok);
        assert_eq!(turn.status.as_deref(), Some("partial"));
        assert!(turn.message.to_ascii_lowercase().contains("captured"));
        assert!(turn.message.to_ascii_lowercase().contains("clipboard"));
    }
}
