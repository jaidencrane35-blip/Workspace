//! Kernel Operator — sole execution authority between Intent and Capability Runtime (P12 Finalization).
//!
//! Not AI. Not autonomous. Deterministic validate → plan → execute → compose.
//! Providers remain independent; only the Operator coordinates them.

mod compose;
mod intent;
mod plan;

pub use compose::compose_user_reply;
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
    pub fn execute_turn(intent: CapabilityIntent) -> Result<OperatorTurnResult> {
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

        if plan.composition_id.as_deref() == Some("app.open_or_focus") {
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
            })?;
            results.push(next);
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
