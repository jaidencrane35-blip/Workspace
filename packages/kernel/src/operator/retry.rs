//! C-VER-002 — Bounded retry for authorized interaction compositions.
//!
//! Not an agent loop. Retries only the same authorized action when verification
//! indicates a retryable transient failure. Finite attempts + Wait Conditions.

use crate::capability_runtime::{CapabilityDomainId, CapabilityOperation, ProviderInvokeResponse};
use crate::error::Result;
use crate::operator::intent::CapabilityIntent;
use crate::operator::plan::{OperatorPlan, OperatorPlanStep};

/// Original attempt + one retry. Never indefinite.
pub const MAX_INTERACTION_ATTEMPTS: u32 = 2;

/// Bounded wait between retryable failures (reuses C-VER-003).
pub const RETRY_WAIT_DURATION: &str = "400ms";

/// Statuses that may justify one bounded retry of the same action.
pub fn is_retryable_status(status: Option<&str>) -> bool {
    matches!(
        status,
        Some("control_not_found")
            | Some("control_click_unverified")
            | Some("control_type_unverified")
            | Some("not_found")
            | Some("condition_timeout")
    )
}

/// Statuses that must stop immediately — never auto-retry.
pub fn is_non_retryable_status(status: Option<&str>) -> bool {
    matches!(
        status,
        Some("need_control_name")
            | Some("need_text")
            | Some("need_window")
            | Some("control_click_failed")
            | Some("control_type_failed")
            | Some("need_query")
            | Some("clarify")
            | Some("unsupported")
            | Some("denied")
            | Some("permission_denied")
    )
}

pub fn may_retry(status: Option<&str>) -> bool {
    is_retryable_status(status) && !is_non_retryable_status(status)
}

fn control_name_for_click(intent: &CapabilityIntent) -> Option<String> {
    intent
        .text
        .as_deref()
        .or(intent.title.as_deref())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
}

fn control_name_for_type(intent: &CapabilityIntent) -> Option<String> {
    intent
        .title
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
}

fn wait_control_available_step(intent: &CapabilityIntent, control: &str) -> OperatorPlanStep {
    OperatorPlanStep {
        domain: CapabilityDomainId::window(),
        operation: CapabilityOperation::WaitCondition,
        text: Some(control.to_string()),
        query: intent.query.clone(),
        path: None,
        hwnd: intent.hwnd.clone(),
        pid: intent.pid,
        x: None,
        y: None,
        width: None,
        height: None,
        monitor_index: None,
        snap: None,
        title: None,
        category: Some("control_available".into()),
        priority: None,
        duration: Some(RETRY_WAIT_DURATION.into()),
    }
}

/// Count action invocations (click or type) — used to prove the bound.
pub fn action_attempt_count(
    results: &[ProviderInvokeResponse],
    act: CapabilityOperation,
) -> usize {
    results.iter().filter(|r| r.operation == act).count()
}

/// Locate / act / verify legs for the latest action attempt (skips Wait steps).
pub fn last_interaction_legs(
    results: &[ProviderInvokeResponse],
    act: CapabilityOperation,
) -> Option<(bool, bool, bool)> {
    let act_idx = results.iter().rposition(|r| r.operation == act)?;
    let act_ok = results[act_idx].ok;
    let verify_ok = results
        .get(act_idx + 1)
        .filter(|r| r.operation == CapabilityOperation::FindControl)
        .map(|r| r.ok)
        .unwrap_or(false);
    let locate_ok = results[..act_idx]
        .iter()
        .rev()
        .find(|r| r.operation == CapabilityOperation::FindControl)
        .map(|r| r.ok)
        .unwrap_or(false);
    Some((locate_ok, act_ok, verify_ok))
}

/// Execute Find → Act → Find with at most one Wait+retry on retryable failure.
pub fn execute_interaction_with_retry<F>(
    intent: &CapabilityIntent,
    plan: &OperatorPlan,
    act: CapabilityOperation,
    mut invoke: F,
) -> Result<Vec<ProviderInvokeResponse>>
where
    F: FnMut(&OperatorPlanStep) -> Result<ProviderInvokeResponse>,
{
    let find_step = plan
        .steps
        .iter()
        .find(|s| s.operation == CapabilityOperation::FindControl)
        .cloned()
        .ok_or_else(|| crate::error::KernelError::CapabilityRuntime {
            message: "Retry composition missing locate step.".into(),
        })?;
    let act_step = plan
        .steps
        .iter()
        .find(|s| s.operation == act)
        .cloned()
        .ok_or_else(|| crate::error::KernelError::CapabilityRuntime {
            message: "Retry composition missing action step.".into(),
        })?;
    let verify_step = plan
        .steps
        .iter()
        .rev()
        .find(|s| s.operation == CapabilityOperation::FindControl)
        .cloned()
        .unwrap_or_else(|| find_step.clone());

    let control = match act {
        CapabilityOperation::InvokeControl => control_name_for_click(intent),
        CapabilityOperation::SetControlValue => control_name_for_type(intent),
        _ => None,
    };

    let mut results: Vec<ProviderInvokeResponse> = Vec::new();
    let mut attempts = 0u32;

    while attempts < MAX_INTERACTION_ATTEMPTS {
        attempts = attempts.saturating_add(1);

        let locate = invoke(&find_step)?;
        let locate_ok = locate.ok;
        let locate_status = locate.status.clone();
        results.push(locate);

        if !locate_ok {
            if attempts < MAX_INTERACTION_ATTEMPTS && may_retry(locate_status.as_deref()) {
                if let Some(name) = control.as_deref() {
                    let wait = invoke(&wait_control_available_step(intent, name))?;
                    results.push(wait);
                }
                continue;
            }
            break;
        }

        let action = invoke(&act_step)?;
        let action_ok = action.ok;
        let action_status = action.status.clone();
        results.push(action);

        if !action_ok {
            if attempts < MAX_INTERACTION_ATTEMPTS && may_retry(action_status.as_deref()) {
                if let Some(name) = control.as_deref() {
                    let wait = invoke(&wait_control_available_step(intent, name))?;
                    results.push(wait);
                }
                continue;
            }
            break;
        }

        let verify = invoke(&verify_step)?;
        let verify_ok = verify.ok;
        let verify_status = verify.status.clone();
        results.push(verify);

        if verify_ok {
            break;
        }

        if attempts < MAX_INTERACTION_ATTEMPTS && may_retry(verify_status.as_deref()) {
            if let Some(name) = control.as_deref() {
                let wait = invoke(&wait_control_available_step(intent, name))?;
                results.push(wait);
            }
            continue;
        }
        break;
    }

    debug_assert!(action_attempt_count(&results, act) <= MAX_INTERACTION_ATTEMPTS as usize);
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability_runtime::CapabilityDomainId;

    fn resp(op: CapabilityOperation, ok: bool, status: &str) -> ProviderInvokeResponse {
        ProviderInvokeResponse {
            domain: CapabilityDomainId::window(),
            operation: op,
            ok,
            format: None,
            bytes: None,
            text: None,
            preview: None,
            message: Some(status.into()),
            status: Some(status.into()),
            target: Some("Fixture".into()),
            items: None,
            monitors: None,
        }
    }

    fn click_plan() -> (CapabilityIntent, OperatorPlan) {
        let intent = CapabilityIntent {
            domain: "window".into(),
            operation: "click_control".into(),
            query: Some("Fixture Focus".into()),
            text: Some("Save".into()),
            ..Default::default()
        };
        let plan = crate::operator::plan_capability_intent(&intent).unwrap();
        (intent, plan)
    }

    #[test]
    fn classifies_retryable_and_non_retryable() {
        assert!(may_retry(Some("control_not_found")));
        assert!(may_retry(Some("control_click_unverified")));
        assert!(may_retry(Some("control_type_unverified")));
        assert!(!may_retry(Some("control_click_failed")));
        assert!(!may_retry(Some("control_type_failed")));
        assert!(!may_retry(Some("need_control_name")));
        assert!(!may_retry(Some("need_text")));
        assert!(!is_retryable_status(Some("control_clicked")));
    }

    #[test]
    fn first_attempt_success_no_wait_no_retry() {
        let (intent, plan) = click_plan();
        let mut calls = 0u32;
        let results = execute_interaction_with_retry(
            &intent,
            &plan,
            CapabilityOperation::InvokeControl,
            |step| {
                calls += 1;
                match step.operation {
                    CapabilityOperation::FindControl => {
                        Ok(resp(CapabilityOperation::FindControl, true, "control_found"))
                    }
                    CapabilityOperation::InvokeControl => {
                        Ok(resp(CapabilityOperation::InvokeControl, true, "control_clicked"))
                    }
                    other => panic!("unexpected op {other:?}"),
                }
            },
        )
        .unwrap();
        assert_eq!(calls, 3);
        assert_eq!(results.len(), 3);
        assert_eq!(
            action_attempt_count(&results, CapabilityOperation::InvokeControl),
            1
        );
        assert!(!results
            .iter()
            .any(|r| r.operation == CapabilityOperation::WaitCondition));
        assert_eq!(
            last_interaction_legs(&results, CapabilityOperation::InvokeControl),
            Some((true, true, true))
        );
    }

    #[test]
    fn retryable_locate_failure_then_success() {
        let (intent, plan) = click_plan();
        let mut finds = 0u32;
        let results = execute_interaction_with_retry(
            &intent,
            &plan,
            CapabilityOperation::InvokeControl,
            |step| {
                match step.operation {
                    CapabilityOperation::FindControl => {
                        finds += 1;
                        if finds == 1 {
                            Ok(resp(
                                CapabilityOperation::FindControl,
                                false,
                                "control_not_found",
                            ))
                        } else {
                            Ok(resp(CapabilityOperation::FindControl, true, "control_found"))
                        }
                    }
                    CapabilityOperation::WaitCondition => Ok(resp(
                        CapabilityOperation::WaitCondition,
                        true,
                        "condition_met",
                    )),
                    CapabilityOperation::InvokeControl => {
                        Ok(resp(CapabilityOperation::InvokeControl, true, "control_clicked"))
                    }
                    other => panic!("unexpected op {other:?}"),
                }
            },
        )
        .unwrap();
        assert!(results
            .iter()
            .any(|r| r.operation == CapabilityOperation::WaitCondition));
        assert_eq!(
            action_attempt_count(&results, CapabilityOperation::InvokeControl),
            1
        );
        assert_eq!(
            last_interaction_legs(&results, CapabilityOperation::InvokeControl),
            Some((true, true, true))
        );
    }

    #[test]
    fn retry_after_unverified_click_then_completed() {
        let (intent, plan) = click_plan();
        let mut clicks = 0u32;
        let results = execute_interaction_with_retry(
            &intent,
            &plan,
            CapabilityOperation::InvokeControl,
            |step| match step.operation {
                CapabilityOperation::FindControl => {
                    Ok(resp(CapabilityOperation::FindControl, true, "control_found"))
                }
                CapabilityOperation::WaitCondition => Ok(resp(
                    CapabilityOperation::WaitCondition,
                    true,
                    "condition_met",
                )),
                CapabilityOperation::InvokeControl => {
                    clicks += 1;
                    if clicks == 1 {
                        Ok(resp(
                            CapabilityOperation::InvokeControl,
                            false,
                            "control_click_unverified",
                        ))
                    } else {
                        Ok(resp(
                            CapabilityOperation::InvokeControl,
                            true,
                            "control_clicked",
                        ))
                    }
                }
                other => panic!("unexpected op {other:?}"),
            },
        )
        .unwrap();
        assert_eq!(clicks, 2);
        assert_eq!(
            action_attempt_count(&results, CapabilityOperation::InvokeControl),
            2
        );
        assert_eq!(
            last_interaction_legs(&results, CapabilityOperation::InvokeControl),
            Some((true, true, true))
        );
    }

    #[test]
    fn all_attempts_fail_terminates_bounded() {
        let (intent, plan) = click_plan();
        let mut finds = 0u32;
        let results = execute_interaction_with_retry(
            &intent,
            &plan,
            CapabilityOperation::InvokeControl,
            |step| match step.operation {
                CapabilityOperation::FindControl => {
                    finds += 1;
                    Ok(resp(
                        CapabilityOperation::FindControl,
                        false,
                        "control_not_found",
                    ))
                }
                CapabilityOperation::WaitCondition => Ok(resp(
                    CapabilityOperation::WaitCondition,
                    false,
                    "condition_timeout",
                )),
                CapabilityOperation::InvokeControl => panic!("must not click when locate fails"),
                other => panic!("unexpected op {other:?}"),
            },
        )
        .unwrap();
        assert_eq!(finds, 2);
        assert_eq!(
            action_attempt_count(&results, CapabilityOperation::InvokeControl),
            0
        );
        assert!(finds <= MAX_INTERACTION_ATTEMPTS);
    }

    #[test]
    fn non_retryable_failure_does_not_retry() {
        let (intent, plan) = click_plan();
        let mut clicks = 0u32;
        let mut waits = 0u32;
        let results = execute_interaction_with_retry(
            &intent,
            &plan,
            CapabilityOperation::InvokeControl,
            |step| match step.operation {
                CapabilityOperation::FindControl => {
                    Ok(resp(CapabilityOperation::FindControl, true, "control_found"))
                }
                CapabilityOperation::WaitCondition => {
                    waits += 1;
                    Ok(resp(
                        CapabilityOperation::WaitCondition,
                        true,
                        "condition_met",
                    ))
                }
                CapabilityOperation::InvokeControl => {
                    clicks += 1;
                    Ok(resp(
                        CapabilityOperation::InvokeControl,
                        false,
                        "control_click_failed",
                    ))
                }
                other => panic!("unexpected op {other:?}"),
            },
        )
        .unwrap();
        assert_eq!(clicks, 1);
        assert_eq!(waits, 0);
        assert_eq!(
            action_attempt_count(&results, CapabilityOperation::InvokeControl),
            1
        );
    }

    #[test]
    fn max_attempts_never_exceeded() {
        let (intent, plan) = click_plan();
        let mut clicks = 0u32;
        let results = execute_interaction_with_retry(
            &intent,
            &plan,
            CapabilityOperation::InvokeControl,
            |step| match step.operation {
                CapabilityOperation::FindControl => {
                    Ok(resp(CapabilityOperation::FindControl, true, "control_found"))
                }
                CapabilityOperation::WaitCondition => Ok(resp(
                    CapabilityOperation::WaitCondition,
                    true,
                    "condition_met",
                )),
                CapabilityOperation::InvokeControl => {
                    clicks += 1;
                    Ok(resp(
                        CapabilityOperation::InvokeControl,
                        false,
                        "control_click_unverified",
                    ))
                }
                other => panic!("unexpected op {other:?}"),
            },
        )
        .unwrap();
        assert_eq!(clicks, MAX_INTERACTION_ATTEMPTS);
        assert!(clicks <= MAX_INTERACTION_ATTEMPTS);
        assert_eq!(
            action_attempt_count(&results, CapabilityOperation::InvokeControl),
            MAX_INTERACTION_ATTEMPTS as usize
        );
    }
}
