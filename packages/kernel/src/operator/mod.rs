//! Kernel Operator — sole execution authority between Intent and Capability Runtime (P12 Finalization).
//!
//! Not AI. Not autonomous. Deterministic validate → plan → execute → compose.
//! Providers remain independent; only the Operator coordinates them.

mod compose;
mod intent;
mod plan;
mod retry;

pub use compose::{compose_failure_reply, compose_user_reply, sanitize_owner_message};
pub use intent::{CapabilityIntent, OperatorTurnResult};
pub use plan::{
    plan_capability_intent, preflight_prepare_coding_workspace, OperatorPlan, OperatorPlanStep,
};
pub use retry::{
    action_attempt_count, execute_interaction_with_retry, is_non_retryable_status,
    is_retryable_status, last_interaction_legs, may_retry, MAX_INTERACTION_ATTEMPTS,
    RETRY_WAIT_DURATION,
};

use crate::capability_runtime::{
    runtime, CapabilityDomainId, CapabilityOperation, ProviderInvokeRequest, ProviderInvokeResponse,
};
use crate::error::{KernelError, Result};

/// Deterministic Kernel Operator.
pub struct KernelOperator;

impl KernelOperator {
    /// Validate + plan a capability intent (no effects).
    pub fn plan(intent: &CapabilityIntent) -> Result<OperatorPlan> {
        plan_capability_intent(intent)
    }

    /// Execute a planned step through Capability Runtime (ports only — permission via caller).
    pub fn invoke_step(step: &OperatorPlanStep) -> Result<ProviderInvokeResponse> {
        runtime().invoke(ProviderInvokeRequest {
            domain: step.domain.clone(),
            operation: step.operation,
            text: step.text.clone(),
            query: step.query.clone(),
            path: step.path.clone(),
            hwnd: step.hwnd.clone(),
            pid: step.pid,
            x: step.x,
            y: step.y,
            width: step.width,
            height: step.height,
            monitor_index: step.monitor_index,
            snap: step.snap.clone(),
            title: step.title.clone(),
            category: step.category.clone(),
            priority: step.priority.clone(),
            duration: step.duration.clone(),
        })
    }

    /// Compose a truthful, jargon-free user reply from step results.
    pub fn compose(
        intent: &CapabilityIntent,
        plan: &OperatorPlan,
        results: &[ProviderInvokeResponse],
    ) -> OperatorTurnResult {
        compose_user_reply(intent, plan, results)
    }

    /// Full turn: plan → execute steps (with open composition) → compose.
    /// Callers that need per-step Permission Gateway should use `plan` + per-command pipeline instead.
    /// P17.S1: hard failures become composed Conversation turns (logged at warn).
    pub fn execute_turn(intent: CapabilityIntent) -> Result<OperatorTurnResult> {
        match Self::execute_turn_inner(intent.clone()) {
            Ok(result) => Ok(result),
            Err(error) => {
                log::warn!(
                    target: "workspace_capability",
                    "operator execute_turn failed (composed): domain={} operation={} err={error:?}",
                    intent.domain,
                    intent.operation
                );
                Ok(compose_failure_reply(&intent, &error))
            }
        }
    }

    fn execute_turn_inner(intent: CapabilityIntent) -> Result<OperatorTurnResult> {
        let plan = Self::plan(&intent)?;
        if plan.steps.is_empty() {
            return Ok(OperatorTurnResult {
                ok: false,
                message: "I need a clearer request before I can act.".into(),
                status: Some("clarify".into()),
                domain: intent.domain.clone(),
                operation: intent.operation.clone(),
                target: intent.query.clone(),
                preview: None,
                text: None,
                items: None,
                monitors: None,
                composition_id: None,
            });
        }

        let mut results: Vec<ProviderInvokeResponse> = Vec::new();

        if plan.composition_id.as_deref() == Some("desktop.prepare_coding_workspace") {
            // C-PROC-002: PCW-001 preflight (zero effects) → PCW-002 open → PCW-003 wait.
            // C-VER-002 is out of scope — never retry Launch/Open.
            if let Err(error) =
                preflight_prepare_coding_workspace(intent.text.as_deref().unwrap_or(""))
            {
                let message = match error {
                    KernelError::CapabilityRuntime { message } => message,
                    other => other.to_public().message,
                };
                return Ok(OperatorTurnResult {
                    ok: false,
                    message,
                    status: Some("clarify".into()),
                    domain: intent.domain.clone(),
                    operation: intent.operation.clone(),
                    target: intent.query.clone(),
                    preview: None,
                    text: None,
                    items: None,
                    monitors: None,
                    composition_id: plan.composition_id.clone(),
                });
            }

            let labels = prepare_target_labels(&intent);
            let mut unit_queries: Vec<String> = Vec::new();
            let mut observed_hwnds: Vec<Option<String>> = Vec::new();

            for step in &plan.steps {
                if step.domain == CapabilityDomainId::application()
                    && step.operation == CapabilityOperation::Find
                {
                    // Reuse C-ACT-001 Find → Focus|Launch (also used by C-CMP-002).
                    let query = step.query.clone().unwrap_or_default();
                    let find = Self::invoke_step(step)?;
                    let has_match =
                        find.ok && find.items.as_ref().is_some_and(|i| !i.is_empty());
                    let hwnd = find
                        .items
                        .as_ref()
                        .and_then(|items| items.first())
                        .map(|item| item.hwnd.clone());
                    // Multiple substring matches: do not treat first-match as identity.
                    let unique_enough = find
                        .items
                        .as_ref()
                        .map(|items| items.len() == 1)
                        .unwrap_or(false);
                    results.push(find);
                    let next_op = if has_match {
                        CapabilityOperation::Focus
                    } else {
                        CapabilityOperation::Launch
                    };
                    let next = Self::invoke_step(&OperatorPlanStep {
                        domain: CapabilityDomainId::application(),
                        operation: next_op,
                        text: None,
                        query: Some(query.clone()),
                        path: None,
                        hwnd: if has_match && unique_enough {
                            hwnd.clone()
                        } else {
                            None
                        },
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
                    })?;
                    let ok = next.ok;
                    results.push(next);
                    unit_queries.push(query);
                    observed_hwnds.push(if has_match && unique_enough {
                        hwnd
                    } else {
                        None
                    });
                    if !ok {
                        // Stop further opens (C-CMP-002 stop-on-failure); still wait what opened.
                        break;
                    }
                } else if step.domain == CapabilityDomainId::browser()
                    && step.operation == CapabilityOperation::Open
                {
                    let result = Self::invoke_step(step)?;
                    let ok = result.ok;
                    let query = step
                        .query
                        .clone()
                        .or_else(|| step.path.clone())
                        .unwrap_or_default();
                    results.push(result);
                    unit_queries.push(query);
                    observed_hwnds.push(None);
                    if !ok {
                        break;
                    }
                } else {
                    let result = Self::invoke_step(step)?;
                    let ok = result.ok;
                    results.push(result);
                    if !ok {
                        break;
                    }
                }
            }

            // PCW-003 — one C-VER-003 window_available wait per opened/attempted target unit.
            let opened_units = observed_hwnds.len();
            for unit in 0..opened_units {
                let label = labels
                    .get(unit)
                    .cloned()
                    .or_else(|| unit_queries.get(unit).cloned())
                    .unwrap_or_else(|| "that window".into());
                let preferred_hwnd = observed_hwnds[unit].clone();
                let wait = Self::invoke_step(&OperatorPlanStep {
                    domain: CapabilityDomainId::window(),
                    operation: CapabilityOperation::WaitCondition,
                    text: None,
                    query: Some(label.clone()),
                    path: None,
                    hwnd: preferred_hwnd.clone(),
                    pid: None,
                    x: None,
                    y: None,
                    width: None,
                    height: None,
                    monitor_index: None,
                    snap: None,
                    title: None,
                    category: Some("window_available".into()),
                    priority: None,
                    duration: None,
                })?;
                results.push(wait.clone());

                // Confirm identity: prefer hwnd; otherwise require a unique Find match.
                let confirmed = confirm_prepare_target_identity(
                    preferred_hwnd.as_deref(),
                    &label,
                    &wait,
                    |query, hwnd| {
                        Self::invoke_step(&OperatorPlanStep {
                            domain: CapabilityDomainId::application(),
                            operation: CapabilityOperation::Find,
                            text: None,
                            query: Some(query.to_string()),
                            path: None,
                            hwnd: hwnd.map(|h| h.to_string()),
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
                        })
                    },
                )?;
                if let Some(verify) = confirmed {
                    results.push(verify);
                }
            }
        } else if plan.composition_id.as_deref() == Some("desktop.open_compound") {
            // P21.S2 — each Find step = open_or_focus unit; each Browser Open = site open.
            for step in &plan.steps {
                if step.domain == CapabilityDomainId::application()
                    && step.operation == CapabilityOperation::Find
                {
                    let query = step.query.clone();
                    let find = Self::invoke_step(step)?;
                    let has_match =
                        find.ok && find.items.as_ref().is_some_and(|i| !i.is_empty());
                    results.push(find);
                    let next_op = if has_match {
                        CapabilityOperation::Focus
                    } else {
                        CapabilityOperation::Launch
                    };
                    let next = Self::invoke_step(&OperatorPlanStep {
                        domain: CapabilityDomainId::application(),
                        operation: next_op,
                        text: None,
                        query,
                        path: None,
                        hwnd: None,
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
                    })?;
                    let ok = next.ok;
                    results.push(next);
                    if !ok {
                        break;
                    }
                } else {
                    let result = Self::invoke_step(step)?;
                    let ok = result.ok;
                    results.push(result);
                    if !ok {
                        break;
                    }
                }
            }
        } else if plan.composition_id.as_deref() == Some("app.open_or_focus")
            || plan.composition_id.as_deref() == Some("app.open_maximize")
        {
            let find = Self::invoke_step(&plan.steps[0])?;
            let has_match = find.ok && find.items.as_ref().is_some_and(|i| !i.is_empty());
            results.push(find);
            let next_op = if has_match {
                CapabilityOperation::Focus
            } else {
                CapabilityOperation::Launch
            };
            let next = Self::invoke_step(&OperatorPlanStep {
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
            })?;
            let maximize_after = plan.composition_id.as_deref() == Some("app.open_maximize") && next.ok;
            results.push(next);
            if maximize_after {
                let maximize = Self::invoke_step(&OperatorPlanStep {
                    domain: CapabilityDomainId::window(),
                    operation: CapabilityOperation::Maximize,
                    text: None,
                    query: intent.query.clone(),
                    path: None,
                    hwnd: None,
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
                })?;
                results.push(maximize);
            }
        } else if plan.composition_id.as_deref() == Some("window.click_control") {
            // C-VER-002 — bounded retry of the same authorized click (reuses Wait Conditions).
            results = execute_interaction_with_retry(
                &intent,
                &plan,
                CapabilityOperation::InvokeControl,
                Self::invoke_step,
            )?;
        } else if plan.composition_id.as_deref() == Some("window.type_control") {
            // C-VER-002 — bounded retry of the same authorized type (reuses Wait Conditions).
            results = execute_interaction_with_retry(
                &intent,
                &plan,
                CapabilityOperation::SetControlValue,
                Self::invoke_step,
            )?;
        } else {
            for step in &plan.steps {
                let result = Self::invoke_step(step)?;
                let ok = result.ok;
                results.push(result);
                if !ok {
                    break;
                }
            }
        }

        Ok(Self::compose(&intent, &plan, &results))
    }
}

fn prepare_target_labels(intent: &CapabilityIntent) -> Vec<String> {
    intent
        .title
        .as_deref()
        .unwrap_or("")
        .split(" and ")
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

/// PCW-003/004 identity confirmation — hwnd preferred; never first-substring alone.
fn confirm_prepare_target_identity<F>(
    preferred_hwnd: Option<&str>,
    label: &str,
    wait: &ProviderInvokeResponse,
    mut find: F,
) -> Result<Option<ProviderInvokeResponse>>
where
    F: FnMut(&str, Option<&str>) -> Result<ProviderInvokeResponse>,
{
    let wait_met = wait.ok
        && wait
            .status
            .as_deref()
            .is_some_and(|s| s == "condition_met" || s == "found");

    if let Some(hwnd) = preferred_hwnd {
        if wait_met
            && wait
                .items
                .as_ref()
                .is_some_and(|items| items.iter().any(|item| item.hwnd == hwnd))
        {
            return Ok(Some(ProviderInvokeResponse {
                domain: CapabilityDomainId::application(),
                operation: CapabilityOperation::Find,
                ok: true,
                format: Some("prepare_identity".into()),
                bytes: Some(1),
                text: None,
                preview: Some(label.into()),
                message: Some(format!("verified hwnd for “{label}”.")),
                status: Some("identity_confirmed".into()),
                target: Some(label.into()),
                items: wait.items.clone(),
                monitors: None,
            }));
        }
        let observed = find(label, Some(hwnd))?;
        let ok = observed.ok
            && observed
                .items
                .as_ref()
                .is_some_and(|items| items.iter().any(|item| item.hwnd == hwnd));
        return Ok(Some(ProviderInvokeResponse {
            domain: CapabilityDomainId::application(),
            operation: CapabilityOperation::Find,
            ok,
            format: Some("prepare_identity".into()),
            bytes: observed.items.as_ref().map(|i| i.len()),
            text: None,
            preview: Some(label.into()),
            message: Some(if ok {
                format!("verified hwnd for “{label}”.")
            } else {
                format!("could not confirm “{label}”.")
            }),
            status: Some(if ok {
                "identity_confirmed".into()
            } else {
                "identity_unconfirmed".into()
            }),
            target: Some(label.into()),
            items: observed.items,
            monitors: None,
        }));
    }

    if !wait_met {
        return Ok(Some(ProviderInvokeResponse {
            domain: CapabilityDomainId::application(),
            operation: CapabilityOperation::Find,
            ok: false,
            format: Some("prepare_identity".into()),
            bytes: Some(0),
            text: None,
            preview: Some(label.into()),
            message: Some(format!("“{label}” was not observed in time.")),
            status: Some("identity_unconfirmed".into()),
            target: Some(label.into()),
            items: None,
            monitors: None,
        }));
    }

    // No hwnd: require exactly one Find match — refuse first-of-many substring hits.
    let observed = find(label, None)?;
    let count = observed.items.as_ref().map(|i| i.len()).unwrap_or(0);
    let ok = observed.ok && count == 1;
    Ok(Some(ProviderInvokeResponse {
        domain: CapabilityDomainId::application(),
        operation: CapabilityOperation::Find,
        ok,
        format: Some("prepare_identity".into()),
        bytes: Some(count),
        text: None,
        preview: Some(label.into()),
        message: Some(if ok {
            format!("verified unique window for “{label}”.")
        } else if count > 1 {
            format!("multiple windows matched “{label}” — left unconfirmed.")
        } else {
            format!("could not confirm “{label}”.")
        }),
        status: Some(if ok {
            "identity_confirmed".into()
        } else {
            "identity_unconfirmed".into()
        }),
        target: Some(label.into()),
        items: observed.items,
        monitors: None,
    }))
}

pub fn parse_domain(raw: &str) -> Result<CapabilityDomainId> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "clipboard" => Ok(CapabilityDomainId::clipboard()),
        "application" | "app" => Ok(CapabilityDomainId::application()),
        "window" => Ok(CapabilityDomainId::window()),
        "notifications" | "notification" | "notify" => Ok(CapabilityDomainId::notifications()),
        "browser" | "web" => Ok(CapabilityDomainId::browser()),
        "screenshots" | "screenshot" | "capture" => Ok(CapabilityDomainId::screenshots()),
        other => Err(KernelError::CapabilityRuntime {
            message: format!("unknown capability domain '{other}'"),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plans_application_open_composition() {
        let plan = KernelOperator::plan(&CapabilityIntent {
            domain: "application".into(),
            operation: "open".into(),
            query: Some("notepad".into()),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(plan.composition_id.as_deref(), Some("app.open_or_focus"));
        assert_eq!(plan.steps.len(), 1);
        assert_eq!(plan.steps[0].operation, CapabilityOperation::Find);
    }

    #[test]
    fn plans_browser_open_foreground_composition() {
        let plan = KernelOperator::plan(&CapabilityIntent {
            domain: "browser".into(),
            operation: "open_foreground".into(),
            path: Some("https://chatgpt.com".into()),
            query: Some("ChatGPT".into()),
            title: Some("ChatGPT".into()),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(
            plan.composition_id.as_deref(),
            Some("browser.open_foreground")
        );
        assert_eq!(plan.steps.len(), 2);
    }

    #[test]
    fn plans_compound_open_decomposition() {
        let plan = KernelOperator::plan(&CapabilityIntent {
            domain: "application".into(),
            operation: "open_compound".into(),
            text: Some("app:Cursor|app:Google Chrome".into()),
            title: Some("Cursor and Google Chrome".into()),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(
            plan.composition_id.as_deref(),
            Some("desktop.open_compound")
        );
        assert_eq!(plan.steps.len(), 2);
        assert_eq!(plan.steps[0].operation, CapabilityOperation::Find);
        assert_eq!(plan.steps[0].query.as_deref(), Some("Cursor"));
        assert_eq!(plan.steps[1].operation, CapabilityOperation::Find);
        assert_eq!(plan.steps[1].query.as_deref(), Some("Google Chrome"));
    }

    #[test]
    fn plans_prepare_coding_workspace_composition() {
        let plan = KernelOperator::plan(&CapabilityIntent {
            domain: "application".into(),
            operation: "prepare_coding_workspace".into(),
            text: Some("app:Cursor|app:Notepad".into()),
            title: Some("Cursor and Notepad".into()),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(
            plan.composition_id.as_deref(),
            Some("desktop.prepare_coding_workspace")
        );
        assert_eq!(plan.steps.len(), 2);
        assert_eq!(plan.steps[0].query.as_deref(), Some("Cursor"));
        assert_eq!(plan.steps[1].query.as_deref(), Some("Notepad"));
    }

    #[test]
    fn plans_prepare_coding_workspace_single_app_uses_find_step() {
        let plan = KernelOperator::plan(&CapabilityIntent {
            domain: "application".into(),
            operation: "prepare_coding_workspace".into(),
            text: Some("app:Notepad".into()),
            title: Some("Notepad".into()),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(
            plan.composition_id.as_deref(),
            Some("desktop.prepare_coding_workspace")
        );
        assert_eq!(plan.steps.len(), 1);
        assert_eq!(plan.steps[0].operation, CapabilityOperation::Find);
    }

    #[test]
    fn prepare_preflight_rejects_kernel_unexecutable_before_effects() {
        let err = preflight_prepare_coding_workspace("app:Cursor|app:Visual Studio Code")
            .expect_err("VS Code is Intent-known but not launch_alias");
        let msg = err.to_string().to_ascii_lowercase();
        assert!(msg.contains("visual studio code") || msg.contains("nothing was opened"));
        assert!(preflight_prepare_coding_workspace("app:Cursor|app:Notepad").is_ok());
    }

    #[test]
    fn prepare_preflight_rejects_unknown_app() {
        assert!(preflight_prepare_coding_workspace("app:NoSuchCodingAppZZZ").is_err());
    }

    #[test]
    fn plans_browser_open_beside_completion_contract() {
        let plan = KernelOperator::plan(&CapabilityIntent {
            domain: "browser".into(),
            operation: "open_beside".into(),
            path: Some("https://chatgpt.com".into()),
            query: Some("https://chatgpt.com".into()),
            title: Some("Cursor".into()),
            snap: Some("Chrome".into()),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(plan.composition_id.as_deref(), Some("browser.open_beside"));
        assert!(
            plan.steps.len() >= 5,
            "open_beside must plan open+locate+snaps+verify, got {}",
            plan.steps.len()
        );
        assert_eq!(plan.steps[0].operation, CapabilityOperation::Open);
        assert_eq!(plan.steps[0].domain, CapabilityDomainId::browser());
        assert_eq!(plan.steps[1].operation, CapabilityOperation::Focus);
        assert_eq!(plan.steps[2].operation, CapabilityOperation::Snap);
        assert_eq!(plan.steps[2].snap.as_deref(), Some("left"));
        assert_eq!(plan.steps[3].operation, CapabilityOperation::Snap);
        assert_eq!(plan.steps[3].snap.as_deref(), Some("right"));
        assert_eq!(plan.steps[4].operation, CapabilityOperation::Enumerate);
    }

    #[test]
    fn plans_application_open_maximize_composition() {
        let plan = KernelOperator::plan(&CapabilityIntent {
            domain: "application".into(),
            operation: "open_maximize".into(),
            query: Some("Cursor".into()),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(plan.composition_id.as_deref(), Some("app.open_maximize"));
    }

    #[test]
    fn plans_window_focus_minimize_composition() {
        let plan = KernelOperator::plan(&CapabilityIntent {
            domain: "window".into(),
            operation: "focus_minimize".into(),
            query: Some("ChatGPT".into()),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(
            plan.composition_id.as_deref(),
            Some("window.focus_minimize")
        );
        assert_eq!(plan.steps.len(), 2);
    }

    #[test]
    fn plans_window_enumerate() {
        let plan = KernelOperator::plan(&CapabilityIntent {
            domain: "window".into(),
            operation: "enumerate".into(),
            ..Default::default()
        })
        .unwrap();
        assert!(plan.composition_id.is_none());
        assert_eq!(plan.steps[0].operation, CapabilityOperation::Enumerate);
    }
}
