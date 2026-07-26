//! Sprint 136 — Cognition pipeline separation and automation readiness guards.
//!
//! Verifies cognition layers cannot execute and Experience remains translation-only.
//! Does not introduce automation.

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use crate::services::explanation_resolver::{
    resolve_attention_reason, resolve_attention_reason_traced,
};
use workspace_domain::{AttentionReason, AttentionSignal, AttentionSourceType};

fn assert_cannot_execute(label: &str, result: Result<(), KernelError>) {
    match result {
        Err(err) => {
            let message = err.to_string().to_lowercase();
            assert!(
                message.contains("cannot execute")
                    || message.contains("cannot")
                    || message.contains("grant")
                    || message.contains("authorize"),
                "{label}: expected CannotExecute-style error, got {err}"
            );
        }
        Ok(()) => panic!("{label}: cognition layer must not succeed at attempt_execute"),
    }
}

/// CASE 1 — Core cognition pipeline layers hard-fail attempt_execute.
#[test]
fn case1_cognition_pipeline_layers_cannot_execute() {
    for (label, result) in [
        (
            "observation",
            CommandHandler::workspace_observation_attempt_execute(),
        ),
        (
            "attention",
            CommandHandler::workspace_attention_attempt_execute(),
        ),
        (
            "intelligence",
            CommandHandler::workspace_intelligence_attempt_execute(),
        ),
        (
            "decision_engine",
            CommandHandler::decision_engine_attempt_execute(),
        ),
        (
            "decision_queue",
            CommandHandler::decision_queue_attempt_execute(),
        ),
        (
            "recommendation",
            CommandHandler::workspace_recommendation_engine_attempt_execute(),
        ),
        (
            "experience",
            CommandHandler::workspace_experience_attempt_execute(),
        ),
    ] {
        assert_cannot_execute(label, result);
    }
}

/// CASE 2 — Experience translation does not grant execution authority.
#[test]
fn case2_experience_translation_is_not_execution() {
    let reason = AttentionReason::new(
        AttentionSourceType::TaskGraph,
        AttentionSignal::BlockedTask,
        40,
        "task.base.blocked",
    );
    let display = resolve_attention_reason(&reason);
    let trace = resolve_attention_reason_traced(&reason, Some("test"));
    assert!(display.known);
    assert_eq!(display, trace.display);
    // Translation succeeds; execution remains forbidden on the Experience service.
    assert_cannot_execute(
        "experience_after_translate",
        CommandHandler::workspace_experience_attempt_execute(),
    );
}

/// CASE 3 — Recommendation / Decision / Automation-adjacent cognition stay non-executing.
#[test]
fn case3_recommendation_and_decision_paths_remain_non_executing() {
    assert_cannot_execute(
        "recommendation_engine",
        CommandHandler::workspace_recommendation_engine_attempt_execute(),
    );
    assert_cannot_execute(
        "decision_engine",
        CommandHandler::decision_engine_attempt_execute(),
    );
    assert_cannot_execute(
        "decision_queue",
        CommandHandler::decision_queue_attempt_execute(),
    );
    assert_cannot_execute(
        "adaptation",
        CommandHandler::workspace_adaptation_attempt_execute(),
    );
}

/// CASE 4 — Layer separation: Attention scoring identity is untouched by Experience resolve.
#[test]
fn case4_experience_does_not_mutate_attention_reasoning() {
    let reason = AttentionReason::new(
        AttentionSourceType::DecisionQueue,
        AttentionSignal::OutstandingDecision,
        55,
        "decision.base.outstanding",
    );
    let before = reason.clone();
    let _ = resolve_attention_reason(&reason);
    let _ = resolve_attention_reason_traced(&reason, Some("pipeline_contract"));
    assert_eq!(reason, before);
    assert_eq!(reason.weight, 55);
    assert_eq!(reason.explanation_key, "decision.base.outstanding");
}
