use crate::capability_runtime::{CapabilityDomainId, CapabilityOperation};
use crate::error::{KernelError, Result};
use crate::operator::intent::CapabilityIntent;
use crate::operator::parse_domain;

const PLAUSIBLE_TLDS: &[&str] = &[
    "com", "org", "net", "edu", "gov", "mil", "int", "io", "ai", "app", "dev",
    "co", "uk", "au", "ca", "de", "fr", "jp", "us", "nz", "in", "info", "biz",
    "me", "tv", "cc", "tech", "online", "site", "store", "cloud", "gg", "so",
    "fm", "xyz", "pro", "name", "blog", "page", "shop",
];

/// Deterministic URL plausibility for browser open (Intent also validates).
fn is_plausible_website_url(raw: &str) -> bool {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed.contains(char::is_whitespace) {
        return false;
    }
    let lower = trimmed.to_ascii_lowercase();
    let candidate = if lower.starts_with("http://") || lower.starts_with("https://") {
        trimmed.to_string()
    } else if trimmed.contains('.') {
        format!("https://{trimmed}")
    } else {
        return false;
    };
    let without_scheme = candidate
        .split_once("://")
        .map(|(_, rest)| rest)
        .unwrap_or(candidate.as_str());
    let host = without_scheme
        .split(['/', '?', '#'])
        .next()
        .unwrap_or("")
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']');
    if host.is_empty() || host.contains("..") {
        return false;
    }
    if host.eq_ignore_ascii_case("localhost") {
        return true;
    }
    let labels: Vec<&str> = host.split('.').collect();
    if labels.len() < 2 {
        return false;
    }
    let tld = labels[labels.len() - 1].to_ascii_lowercase();
    if !PLAUSIBLE_TLDS.iter().any(|item| *item == tld) {
        return false;
    }
    let sld = labels[labels.len() - 2];
    if sld.len() < 2 {
        return false;
    }
    labels.iter().all(|label| {
        !label.is_empty()
            && !label.starts_with('-')
            && !label.ends_with('-')
            && label
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-')
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorPlanStep {
    pub domain: CapabilityDomainId,
    pub operation: CapabilityOperation,
    pub text: Option<String>,
    pub query: Option<String>,
    pub path: Option<String>,
    pub hwnd: Option<String>,
    pub pid: Option<u32>,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub monitor_index: Option<i32>,
    pub snap: Option<String>,
    pub title: Option<String>,
    pub category: Option<String>,
    pub priority: Option<String>,
    pub duration: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorPlan {
    pub steps: Vec<OperatorPlanStep>,
    pub composition_id: Option<String>,
}

fn step_from_intent(
    domain: CapabilityDomainId,
    operation: CapabilityOperation,
    intent: &CapabilityIntent,
) -> OperatorPlanStep {
    OperatorPlanStep {
        domain,
        operation,
        text: intent.text.clone(),
        query: intent.query.clone(),
        path: intent.path.clone(),
        hwnd: intent.hwnd.clone(),
        pid: intent.pid,
        x: intent.x,
        y: intent.y,
        width: intent.width,
        height: intent.height,
        monitor_index: intent.monitor_index,
        snap: intent.snap.clone(),
        title: intent.title.clone(),
        category: intent.category.clone(),
        priority: intent.priority.clone(),
        duration: intent.duration.clone(),
    }
}

/// Map CapabilityIntent → ordered Operator plan (composition lives here only).
pub fn plan_capability_intent(intent: &CapabilityIntent) -> Result<OperatorPlan> {
    let domain = parse_domain(&intent.domain)?;
    let op_raw = intent.operation.trim().to_ascii_lowercase();

    // High-level composition: application open = find → focus | launch
    if domain == CapabilityDomainId::application()
        && matches!(op_raw.as_str(), "open" | "open_or_focus")
    {
        return Ok(OperatorPlan {
            composition_id: Some("app.open_or_focus".into()),
            steps: vec![step_from_intent(
                CapabilityDomainId::application(),
                CapabilityOperation::Find,
                intent,
            )],
        });
    }

    // Open then maximize (Intent Grammar fullscreen — Kernel composition only)
    if domain == CapabilityDomainId::application() && op_raw == "open_maximize" {
        return Ok(OperatorPlan {
            composition_id: Some("app.open_maximize".into()),
            steps: vec![step_from_intent(
                CapabilityDomainId::application(),
                CapabilityOperation::Find,
                intent,
            )],
        });
    }

    // Browser open + bring window forward (Intent Grammar foreground)
    if domain == CapabilityDomainId::browser() && op_raw == "open_foreground" {
        let url = intent
            .path
            .as_deref()
            .unwrap_or("")
            .trim();
        if url.is_empty() {
            return Err(KernelError::CapabilityRuntime {
                message: "Which website should I open?".into(),
            });
        }
        let focus = intent
            .title
            .as_deref()
            .or(intent.query.as_deref())
            .unwrap_or("")
            .trim();
        if focus.is_empty() {
            return Err(KernelError::CapabilityRuntime {
                message: "Which window should I bring forward?".into(),
            });
        }
        let mut open_intent = intent.clone();
        open_intent.path = Some(url.to_string());
        open_intent.query = Some(url.to_string());
        let mut focus_intent = intent.clone();
        focus_intent.query = Some(focus.to_string());
        focus_intent.title = Some(focus.to_string());
        return Ok(OperatorPlan {
            composition_id: Some("browser.open_foreground".into()),
            steps: vec![
                step_from_intent(
                    CapabilityDomainId::browser(),
                    CapabilityOperation::Open,
                    &open_intent,
                ),
                step_from_intent(
                    CapabilityDomainId::window(),
                    CapabilityOperation::Focus,
                    &focus_intent,
                ),
            ],
        });
    }

    // Browser open beside = open URL then Window snap composition
    if domain == CapabilityDomainId::browser() && op_raw == "open_beside" {
        let url = intent
            .path
            .as_deref()
            .or(intent.query.as_deref())
            .unwrap_or("")
            .trim();
        if url.is_empty() {
            return Err(KernelError::CapabilityRuntime {
                message: "Which website should I open?".into(),
            });
        }
        let beside = intent.title.as_deref().unwrap_or("").trim();
        if beside.is_empty() {
            return Err(KernelError::CapabilityRuntime {
                message: "Beside which window should I place it?".into(),
            });
        }
        let mut open_intent = intent.clone();
        open_intent.path = Some(url.to_string());
        open_intent.query = Some(url.to_string());
        return Ok(OperatorPlan {
            composition_id: Some("browser.open_beside".into()),
            steps: vec![step_from_intent(
                CapabilityDomainId::browser(),
                CapabilityOperation::Open,
                &open_intent,
            )],
        });
    }

    // Screenshot capture + copy (Operator composition; providers stay independent)
    if domain == CapabilityDomainId::screenshots() && op_raw == "capture_and_copy" {
        let capture_op = if intent.monitor_index.is_some() {
            CapabilityOperation::CaptureMonitor
        } else if intent
            .query
            .as_deref()
            .map(str::trim)
            .filter(|q| !q.is_empty())
            .is_some()
        {
            CapabilityOperation::CaptureWindow
        } else {
            CapabilityOperation::CaptureDesktop
        };
        return Ok(OperatorPlan {
            composition_id: Some("screenshots.capture_and_copy".into()),
            steps: vec![
                step_from_intent(CapabilityDomainId::screenshots(), capture_op, intent),
                step_from_intent(
                    CapabilityDomainId::screenshots(),
                    CapabilityOperation::CopyClipboard,
                    intent,
                ),
            ],
        });
    }

    // Browser focus composes to Window focus (providers stay independent)
    if domain == CapabilityDomainId::browser() && op_raw == "focus" {
        let query = intent.query.as_deref().unwrap_or("").trim();
        if query.is_empty() {
            return Err(KernelError::CapabilityRuntime {
                message: "Which browser should I bring forward?".into(),
            });
        }
        let mut win = intent.clone();
        win.domain = "window".into();
        win.operation = "focus".into();
        return Ok(OperatorPlan {
            composition_id: Some("browser.focus_window".into()),
            steps: vec![step_from_intent(
                CapabilityDomainId::window(),
                CapabilityOperation::Focus,
                &win,
            )],
        });
    }

    let operation = CapabilityOperation::parse(&op_raw).ok_or_else(|| {
        KernelError::CapabilityRuntime {
            message: format!("unknown capability operation '{}'", intent.operation),
        }
    })?;

    // Domain/operation sanity (lightweight — providers still reject mismatches).
    match (domain.as_str(), operation) {
        ("clipboard", CapabilityOperation::Read | CapabilityOperation::Write) => {}
        (
            "application",
            CapabilityOperation::Launch
            | CapabilityOperation::Enumerate
            | CapabilityOperation::Focus
            | CapabilityOperation::Close
            | CapabilityOperation::Minimize
            | CapabilityOperation::Restore
            | CapabilityOperation::Find,
        ) => {}
        (
            "window",
            CapabilityOperation::Enumerate
            | CapabilityOperation::Find
            | CapabilityOperation::Active
            | CapabilityOperation::Bounds
            | CapabilityOperation::Monitors
            | CapabilityOperation::Focus
            | CapabilityOperation::Minimize
            | CapabilityOperation::Restore
            | CapabilityOperation::Maximize
            | CapabilityOperation::Move
            | CapabilityOperation::Resize
            | CapabilityOperation::Center
            | CapabilityOperation::Snap,
        ) => {}
        (
            "notifications",
            CapabilityOperation::Status
            | CapabilityOperation::Show
            | CapabilityOperation::Dismiss,
        ) => {}
        (
            "browser",
            CapabilityOperation::Status | CapabilityOperation::Open | CapabilityOperation::Focus,
        ) => {}
        (
            "screenshots",
            CapabilityOperation::Status
            | CapabilityOperation::CaptureDesktop
            | CapabilityOperation::CaptureWindow
            | CapabilityOperation::CaptureMonitor
            | CapabilityOperation::SavePng
            | CapabilityOperation::CopyClipboard,
        ) => {}
        (d, op) => {
            return Err(KernelError::CapabilityRuntime {
                message: format!(
                    "operation '{}' is not valid for domain '{d}'",
                    op.as_str()
                ),
            });
        }
    }

    if domain.as_str() == "notifications" && operation == CapabilityOperation::Show {
        let title = intent.title.as_deref().unwrap_or("").trim();
        let body = intent.text.as_deref().unwrap_or("").trim();
        if title.is_empty() && body.is_empty() {
            return Err(KernelError::CapabilityRuntime {
                message: "What should the notification say?".into(),
            });
        }
    }

    if domain.as_str() == "browser" && operation == CapabilityOperation::Open {
        let url = intent
            .path
            .as_deref()
            .or(intent.query.as_deref())
            .or(intent.text.as_deref())
            .unwrap_or("")
            .trim();
        if url.is_empty() {
            return Err(KernelError::CapabilityRuntime {
                message: "Which website should I open?".into(),
            });
        }
        if !is_plausible_website_url(url) {
            return Err(KernelError::CapabilityRuntime {
                message: "I couldn’t determine a valid website. Try “open google”, “open github”, “open youtube”, or provide a complete URL.".into(),
            });
        }
    }

    if matches!(
        operation,
        CapabilityOperation::Write
            | CapabilityOperation::Launch
            | CapabilityOperation::Focus
            | CapabilityOperation::Close
            | CapabilityOperation::Minimize
            | CapabilityOperation::Restore
            | CapabilityOperation::Find
            | CapabilityOperation::Maximize
            | CapabilityOperation::Move
            | CapabilityOperation::Resize
            | CapabilityOperation::Center
            | CapabilityOperation::Snap
            | CapabilityOperation::Bounds
    ) && domain.as_str() != "clipboard"
        && intent.query.as_deref().unwrap_or("").is_empty()
        && intent.hwnd.as_deref().unwrap_or("").is_empty()
        && !matches!(
            operation,
            CapabilityOperation::Enumerate
                | CapabilityOperation::Active
                | CapabilityOperation::Monitors
                | CapabilityOperation::Read
        )
    {
        // Window deixis "this" is allowed as query; empty query for window ops means active.
        if domain.as_str() == "window" {
            // bare operate verbs OK (active window)
        } else if domain.as_str() == "application"
            && !matches!(operation, CapabilityOperation::Enumerate)
        {
            return Err(KernelError::CapabilityRuntime {
                message: "application operation requires a query".into(),
            });
        }
    }

    if operation == CapabilityOperation::Write
        && intent.text.as_deref().unwrap_or("").is_empty()
    {
        return Err(KernelError::CapabilityRuntime {
            message: "clipboard write requires text".into(),
        });
    }

    Ok(OperatorPlan {
        composition_id: None,
        steps: vec![step_from_intent(domain, operation, intent)],
    })
}
