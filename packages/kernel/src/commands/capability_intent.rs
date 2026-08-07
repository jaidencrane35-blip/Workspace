//! Single Conversation → Kernel Operator IPC entry (P12 Finalization).

use crate::capability_runtime::{
    CapabilityDomainId, CapabilityOperation, ProviderInvokeResponse,
};
use crate::commands::application_capability::ExecuteApplicationOperation;
use crate::commands::clipboard::{ReadClipboard, WriteClipboard};
use crate::commands::browser::{BrowserStatus, OpenBrowserUrl};
use crate::commands::notification::{DismissNotification, NotificationStatus, ShowNotification};
use crate::commands::screenshot::{ExecuteScreenshotOperation, ScreenshotStatus};
use crate::commands::pipeline::CommandPipeline;
use crate::commands::window_capability::ExecuteWindowOperation;
use crate::error::{KernelError, Result};
use crate::operator::{
    compose_failure_reply, compose_user_reply, plan_capability_intent, sanitize_owner_message,
    CapabilityIntent, OperatorPlanStep, OperatorTurnResult,
};
use crate::WorkspaceKernel;
use workspace_domain::{ActorContext, IntentContext};

fn app_result_as_response(
    operation: CapabilityOperation,
    result: crate::commands::ApplicationOperationResult,
) -> ProviderInvokeResponse {
    ProviderInvokeResponse {
        domain: CapabilityDomainId::application(),
        operation,
        ok: result.ok,
        format: None,
        bytes: result.items.as_ref().map(|i| i.len()),
        text: None,
        preview: result.preview,
        message: result.message,
        status: result.status,
        target: result.target,
        items: result.items,
        monitors: None,
    }
}

fn window_result_as_response(
    operation: CapabilityOperation,
    result: crate::commands::WindowOperationResult,
) -> ProviderInvokeResponse {
    ProviderInvokeResponse {
        domain: CapabilityDomainId::window(),
        operation,
        ok: result.ok,
        format: None,
        bytes: result.items.as_ref().map(|i| i.len()),
        text: result.text,
        preview: result.preview,
        message: result.message,
        status: result.status,
        target: result.target,
        items: result.items,
        monitors: result.monitors,
    }
}

fn execute_step(
    kernel: &WorkspaceKernel,
    actor: ActorContext,
    intent_ctx: IntentContext,
    step: &OperatorPlanStep,
) -> Result<ProviderInvokeResponse> {
    let pipeline = || CommandPipeline::new(kernel.command_context(actor.clone(), intent_ctx.clone()));

    match step.domain.as_str() {
        "clipboard" => match step.operation {
            CapabilityOperation::Read => {
                let result = pipeline().execute_query(ReadClipboard)?;
                Ok(ProviderInvokeResponse {
                    domain: CapabilityDomainId::clipboard(),
                    operation: CapabilityOperation::Read,
                    ok: true,
                    format: Some(result.format),
                    bytes: Some(result.bytes),
                    text: Some(result.text),
                    preview: Some(result.preview),
                    message: None,
                    status: Some("read".into()),
                    target: None,
                    items: None,
                    monitors: None,
                })
            }
            CapabilityOperation::Write => {
                let text = step.text.clone().unwrap_or_default();
                let result = pipeline().execute_mutation(WriteClipboard::new(text))?;
                Ok(ProviderInvokeResponse {
                    domain: CapabilityDomainId::clipboard(),
                    operation: CapabilityOperation::Write,
                    ok: true,
                    format: Some(result.format),
                    bytes: Some(result.bytes),
                    text: None,
                    preview: Some(result.preview),
                    message: Some(result.message),
                    status: Some("written".into()),
                    target: None,
                    items: None,
                    monitors: None,
                })
            }
            other => Err(KernelError::CapabilityRuntime {
                message: format!("clipboard does not support '{}'", other.as_str()),
            }),
        },
        "application" => {
            let result = pipeline().execute_mutation(ExecuteApplicationOperation::new(
                step.operation,
                step.query.clone(),
                step.path.clone(),
                step.hwnd.clone(),
            ))?;
            Ok(app_result_as_response(step.operation, result))
        }
        "window" => {
            let result = pipeline().execute_mutation(ExecuteWindowOperation::new(
                step.operation,
                step.query.clone(),
                step.path.clone(),
                step.hwnd.clone(),
                step.pid,
                step.x,
                step.y,
                step.width,
                step.height,
                step.monitor_index,
                step.snap.clone(),
            ))?;
            Ok(window_result_as_response(step.operation, result))
        }
        "browser" => match step.operation {
            CapabilityOperation::Status => {
                let result = pipeline().execute_query(BrowserStatus)?;
                Ok(ProviderInvokeResponse {
                    domain: CapabilityDomainId::browser(),
                    operation: CapabilityOperation::Status,
                    ok: result.available,
                    format: Some(result.default_handler),
                    bytes: None,
                    text: Some(result.browsers),
                    preview: Some(result.message.clone()),
                    message: Some(result.message),
                    status: Some(if result.available {
                        "available".into()
                    } else {
                        "unavailable".into()
                    }),
                    target: None,
                    items: None,
                    monitors: None,
                })
            }
            CapabilityOperation::Open => {
                let result = pipeline().execute_mutation(OpenBrowserUrl::new(
                    step.path
                        .clone()
                        .or_else(|| step.query.clone())
                        .or_else(|| step.text.clone()),
                ))?;
                Ok(ProviderInvokeResponse {
                    domain: CapabilityDomainId::browser(),
                    operation: CapabilityOperation::Open,
                    ok: result.ok,
                    format: None,
                    bytes: None,
                    text: result.target.clone(),
                    preview: result.preview,
                    message: Some(result.message),
                    status: Some(result.status),
                    target: result.target,
                    items: None,
                    monitors: None,
                })
            }
            other => Err(KernelError::CapabilityRuntime {
                message: format!("browser does not support '{}'", other.as_str()),
            }),
        },
        "screenshots" => match step.operation {
            CapabilityOperation::Status => {
                let result = pipeline().execute_query(ScreenshotStatus)?;
                Ok(ProviderInvokeResponse {
                    domain: CapabilityDomainId::screenshots(),
                    operation: CapabilityOperation::Status,
                    ok: result.available,
                    format: Some(format!("{} monitors", result.monitor_count)),
                    bytes: Some(result.monitor_count),
                    text: Some(format!(
                        "window={}, clipboard={}",
                        result.can_capture_window, result.can_copy_clipboard
                    )),
                    preview: Some(result.message.clone()),
                    message: Some(result.message),
                    status: Some(if result.available {
                        "available".into()
                    } else {
                        "unavailable".into()
                    }),
                    target: None,
                    items: None,
                    monitors: None,
                })
            }
            CapabilityOperation::CaptureDesktop
            | CapabilityOperation::CaptureWindow
            | CapabilityOperation::CaptureMonitor
            | CapabilityOperation::SavePng
            | CapabilityOperation::CopyClipboard => {
                let result = pipeline().execute_mutation(ExecuteScreenshotOperation::new(
                    step.operation,
                    step.query.clone(),
                    step.path.clone(),
                    step.monitor_index,
                ))?;
                Ok(ProviderInvokeResponse {
                    domain: CapabilityDomainId::screenshots(),
                    operation: step.operation,
                    ok: result.ok,
                    format: Some("png".into()),
                    bytes: None,
                    text: result.path.clone(),
                    preview: result.preview,
                    message: Some(result.message),
                    status: Some(result.status),
                    target: result.target,
                    items: None,
                    monitors: None,
                })
            }
            other => Err(KernelError::CapabilityRuntime {
                message: format!("screenshots does not support '{}'", other.as_str()),
            }),
        },
        "notifications" => match step.operation {
            CapabilityOperation::Status => {
                let result = pipeline().execute_query(NotificationStatus)?;
                Ok(ProviderInvokeResponse {
                    domain: CapabilityDomainId::notifications(),
                    operation: CapabilityOperation::Status,
                    ok: result.available,
                    format: Some(result.platform),
                    bytes: None,
                    text: Some(result.permission),
                    preview: Some(result.message.clone()),
                    message: Some(result.message),
                    status: Some(if result.available {
                        "available".into()
                    } else {
                        "unavailable".into()
                    }),
                    target: None,
                    items: None,
                    monitors: None,
                })
            }
            CapabilityOperation::Show => {
                let result = pipeline().execute_mutation(ShowNotification::new(
                    step.title.clone(),
                    step.text.clone(),
                    step.category.clone(),
                    step.priority.clone(),
                    step.duration.clone(),
                ))?;
                Ok(ProviderInvokeResponse {
                    domain: CapabilityDomainId::notifications(),
                    operation: CapabilityOperation::Show,
                    ok: result.ok,
                    format: step.category.clone(),
                    bytes: None,
                    text: result.target.clone(),
                    preview: result.preview,
                    message: Some(result.message),
                    status: Some(result.status),
                    target: result.target,
                    items: None,
                    monitors: None,
                })
            }
            CapabilityOperation::Dismiss => {
                let result =
                    pipeline().execute_mutation(DismissNotification::new(step.query.clone()))?;
                Ok(ProviderInvokeResponse {
                    domain: CapabilityDomainId::notifications(),
                    operation: CapabilityOperation::Dismiss,
                    ok: result.ok,
                    format: None,
                    bytes: None,
                    text: result.target.clone(),
                    preview: result.preview,
                    message: Some(result.message),
                    status: Some(result.status),
                    target: result.target,
                    items: None,
                    monitors: None,
                })
            }
            other => Err(KernelError::CapabilityRuntime {
                message: format!("notifications does not support '{}'", other.as_str()),
            }),
        },
        other => Err(KernelError::CapabilityRuntime {
            message: format!("unsupported domain '{other}'"),
        }),
    }
}

fn clarify(intent: &CapabilityIntent, message: impl Into<String>) -> OperatorTurnResult {
    OperatorTurnResult {
        ok: false,
        message: sanitize_owner_message(&message.into()),
        status: Some("clarify".into()),
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

/// Kernel Operator entry: plan (Operator) → per-step Permission Gateway → compose.
/// P17.S1: Owner-visible failures always return `Ok(OperatorTurnResult)` via compose —
/// engineering detail is logged; never forwarded as IPC failure payload text.
pub fn execute_capability_intent(
    kernel: &WorkspaceKernel,
    actor: ActorContext,
    intent_ctx: IntentContext,
    intent: CapabilityIntent,
) -> Result<OperatorTurnResult> {
    match execute_capability_intent_inner(kernel, actor, intent_ctx, intent.clone()) {
        Ok(result) => Ok(result),
        Err(error) => {
            log::warn!(
                target: "workspace_capability",
                "capability intent failed (composed for Conversation): domain={} operation={} err={error:?}",
                intent.domain,
                intent.operation
            );
            Ok(compose_failure_reply(&intent, &error))
        }
    }
}

fn execute_capability_intent_inner(
    kernel: &WorkspaceKernel,
    actor: ActorContext,
    intent_ctx: IntentContext,
    intent: CapabilityIntent,
) -> Result<OperatorTurnResult> {
    let plan = match plan_capability_intent(&intent) {
        Ok(plan) => plan,
        Err(KernelError::CapabilityRuntime { message }) => {
            return Ok(clarify(&intent, message));
        }
        Err(other) => return Err(other),
    };

    if plan.steps.is_empty() {
        return Ok(clarify(
            &intent,
            "I need a clearer request before I can act.",
        ));
    }

    let mut results = Vec::new();

    if plan.composition_id.as_deref() == Some("browser.open_beside") {
        let opened = execute_step(kernel, actor.clone(), intent_ctx.clone(), &plan.steps[0])?;
        results.push(opened);
        let beside = intent.title.clone().unwrap_or_default();
        // Prefer Intent snap hint, then common Windows browser window titles.
        let browser_candidates: Vec<String> = {
            let mut list = Vec::new();
            if let Some(hint) = intent.snap.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
                list.push(hint.to_string());
            }
            for name in ["Chrome", "Edge", "Firefox", "Brave"] {
                if !list.iter().any(|n| n.eq_ignore_ascii_case(name)) {
                    list.push(name.to_string());
                }
            }
            list
        };
        let mut snapped_left = false;
        for browser_query in browser_candidates {
            let left = execute_step(
                kernel,
                actor.clone(),
                intent_ctx.clone(),
                &OperatorPlanStep {
                    domain: CapabilityDomainId::window(),
                    operation: CapabilityOperation::Snap,
                    text: None,
                    query: Some(browser_query),
                    path: None,
                    hwnd: None,
                    pid: None,
                    x: None,
                    y: None,
                    width: None,
                    height: None,
                    monitor_index: None,
                    snap: Some("left".into()),
                    title: None,
                    category: None,
                    priority: None,
                    duration: None,
                },
            )?;
            let ok = left.ok;
            results.push(left);
            if ok {
                snapped_left = true;
                break;
            }
        }
        if !snapped_left {
            // Fall back to active window after URL open.
            let left = execute_step(
                kernel,
                actor.clone(),
                intent_ctx.clone(),
                &OperatorPlanStep {
                    domain: CapabilityDomainId::window(),
                    operation: CapabilityOperation::Snap,
                    text: None,
                    query: Some("this".into()),
                    path: None,
                    hwnd: None,
                    pid: None,
                    x: None,
                    y: None,
                    width: None,
                    height: None,
                    monitor_index: None,
                    snap: Some("left".into()),
                    title: None,
                    category: None,
                    priority: None,
                    duration: None,
                },
            )?;
            results.push(left);
        }
        if !beside.is_empty() {
            let right = execute_step(
                kernel,
                actor,
                intent_ctx,
                &OperatorPlanStep {
                    domain: CapabilityDomainId::window(),
                    operation: CapabilityOperation::Snap,
                    text: None,
                    query: Some(beside),
                    path: None,
                    hwnd: None,
                    pid: None,
                    x: None,
                    y: None,
                    width: None,
                    height: None,
                    monitor_index: None,
                    snap: Some("right".into()),
                    title: None,
                    category: None,
                    priority: None,
                    duration: None,
                },
            )?;
            results.push(right);
        }
        return Ok(compose_user_reply(&intent, &plan, &results));
    }

    if plan.composition_id.as_deref() == Some("app.open_or_focus") {
        let found = execute_step(kernel, actor.clone(), intent_ctx.clone(), &plan.steps[0])?;
        let has_match = found.ok && found.items.as_ref().is_some_and(|i| !i.is_empty());
        results.push(found);
        let next_op = if has_match {
            CapabilityOperation::Focus
        } else {
            CapabilityOperation::Launch
        };
        let next = execute_step(
            kernel,
            actor,
            intent_ctx,
            &OperatorPlanStep {
                domain: CapabilityDomainId::application(),
                operation: next_op,
                text: None,
                query: intent.query.clone(),
                path: intent.path.clone(),
                hwnd: intent.hwnd.clone(),
                pid: None,
                x: None,
                y: None,
                width: None,
                height: None,
                monitor_index: None,
                snap: None,
                title: None,
                category: None,
                priority: None,
                duration: None,
            },
        )?;
        results.push(next);
    } else {
        for step in &plan.steps {
            let result = execute_step(kernel, actor.clone(), intent_ctx.clone(), step)?;
            let ok = result.ok;
            results.push(result);
            if !ok {
                break;
            }
        }
    }

    Ok(compose_user_reply(&intent, &plan, &results))
}
