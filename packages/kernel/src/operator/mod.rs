//! Kernel Operator — sole execution authority between Intent and Capability Runtime (P12 Finalization).
//!
//! Not AI. Not autonomous. Deterministic validate → plan → execute → compose.
//! Providers remain independent; only the Operator coordinates them.

mod compose;
mod intent;
mod plan;

pub use compose::{compose_failure_reply, compose_user_reply, sanitize_owner_message};
pub use intent::{CapabilityIntent, OperatorTurnResult};
pub use plan::{plan_capability_intent, OperatorPlan, OperatorPlanStep};

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

        if plan.composition_id.as_deref() == Some("desktop.open_compound") {
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
