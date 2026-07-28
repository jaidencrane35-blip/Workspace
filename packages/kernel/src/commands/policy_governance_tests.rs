//! Programme III Batch 2 — Policy & Governance Engine contract tests.

use workspace_domain::{
    recovery_must_not_fabricate_policy_governance, ActorContext, IntentContext,
    PolicyEvaluationResult,
};

use crate::commands::handler::CommandHandler;
use crate::services::PolicyGovernanceService;
use crate::WorkspaceKernel;

fn seed_workspace(kernel: &WorkspaceKernel) -> String {
    CommandHandler::create_workspace(
        kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        "Policy Gov WS".into(),
    )
    .unwrap()
    .id
    .to_string()
}

fn seed_state_envelope(kernel: &WorkspaceKernel, ws: &str) {
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    // Generate envelope (partial / unavailable sources expected) as policy context.
    CommandHandler::generate_workspace_state_envelope(
        kernel,
        actor,
        intent,
        ws.to_string(),
    )
    .unwrap();
}

#[test]
fn governance_evaluation_is_advisory_and_non_executing() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_state_envelope(&kernel, &ws);
    let snap = CommandHandler::generate_governance_evaluation(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(snap.is_non_commandable());
    assert!(recovery_must_not_fabricate_policy_governance(&snap));
    let current = snap.current.as_ref().expect("current evaluation");
    assert!(current.is_non_executing());
    assert!(!current.evaluations.is_empty());
    let rec = current.recommendation.as_ref().expect("recommendation");
    assert_eq!(rec.authority_effect, "none");
    assert!(!rec.actionable);
    // Unavailable sources in envelope ⇒ never fabricated Compliant.
    assert_ne!(rec.aggregate_result, PolicyEvaluationResult::Compliant);
}

#[test]
fn missing_state_fails_closed_unknown() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    // No envelope generated — evaluation must be Unknown, not Compliant.
    let snap = CommandHandler::generate_governance_evaluation(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let current = snap.current.as_ref().unwrap();
    assert!(current.context_revision.is_none() || current.context_revision.as_deref() == Some("missing") || current.evaluations.iter().any(|e| e.context_revision == "missing"));
    assert!(current
        .evaluations
        .iter()
        .all(|e| e.result == PolicyEvaluationResult::Unknown));
    assert_eq!(
        current.recommendation.as_ref().unwrap().aggregate_result,
        PolicyEvaluationResult::Unknown
    );
}

#[test]
fn governance_recomputation_supersedes_and_separates_history() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_state_envelope(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let first = CommandHandler::generate_governance_evaluation(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let second = CommandHandler::generate_governance_evaluation(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert_ne!(
        first.current.as_ref().unwrap().meta.evaluation_set_id,
        second.current.as_ref().unwrap().meta.evaluation_set_id
    );
    assert!(second.history_count >= 1);
    assert!(second.history.iter().all(|h| h.is_non_actionable()));
    let summary =
        CommandHandler::get_governance_summary(&kernel, actor, intent, ws, 0).unwrap();
    assert!(summary.history_count >= 1);
    assert!(summary.history.is_empty());
}

#[test]
fn governance_restart_continuity_without_fabricating() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_state_envelope(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let generated = CommandHandler::generate_governance_evaluation(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let loaded =
        CommandHandler::get_governance_evaluation(&kernel, actor, intent, ws).unwrap();
    assert_eq!(
        generated.current.as_ref().unwrap().meta.evaluation_set_id,
        loaded.current.as_ref().unwrap().meta.evaluation_set_id
    );
    assert!(recovery_must_not_fabricate_policy_governance(&loaded));
}

#[test]
fn empty_workspace_evaluation_remains_missing_until_generated() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let snap =
        CommandHandler::get_governance_evaluation(&kernel, actor.clone(), intent.clone(), ws.clone())
            .unwrap();
    assert!(snap.current.is_none());
    assert!(recovery_must_not_fabricate_policy_governance(&snap));
    let generated =
        CommandHandler::generate_governance_evaluation(&kernel, actor, intent, ws).unwrap();
    assert!(generated.current.is_some());
}

#[test]
fn governance_transaction_rollback_leaves_no_partial_write() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_state_envelope(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::generate_governance_evaluation(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    PolicyGovernanceService::generate_with_forced_rollback(
        &kernel.shared_database(),
        &actor,
        ws.clone(),
    )
    .unwrap();
    let after =
        CommandHandler::get_governance_evaluation(&kernel, actor, intent, ws).unwrap();
    assert_eq!(
        before.current.as_ref().unwrap().meta.evaluation_set_id,
        after.current.as_ref().unwrap().meta.evaluation_set_id
    );
    assert_eq!(before.history_count, after.history_count);
}

#[test]
fn governance_negative_authority_guards() {
    assert!(PolicyGovernanceService::attempt_execute().is_err());
    assert!(PolicyGovernanceService::attempt_grant_permission().is_err());
    assert!(PolicyGovernanceService::attempt_bypass_gateway().is_err());
    assert!(PolicyGovernanceService::attempt_create_capability().is_err());
    assert!(PolicyGovernanceService::attempt_mutate_lifecycle().is_err());
    assert!(CommandHandler::policy_governance_attempt_execute().is_err());
}

#[test]
fn governance_capability_deny_without_write() {
    use workspace_domain::CapabilitySet;

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let mut ctx =
        kernel.command_context(ActorContext::local_user(), IntentContext::user_request());
    ctx.capability_set = CapabilitySet::new();
    let err = crate::commands::pipeline::CommandPipeline::new(ctx)
        .execute_mutation(
            crate::commands::policy_governance::GenerateGovernanceEvaluation::new(ws),
        )
        .expect_err("empty capabilities must deny");
    let message = err.to_string().to_lowercase();
    assert!(
        message.contains("denied")
            || message.contains("permission")
            || message.contains("capability")
            || message.contains("not granted"),
        "got {err}"
    );
}

#[test]
fn explain_governance_decision_separates_explanation() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_state_envelope(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::generate_governance_evaluation(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let explanation =
        CommandHandler::explain_governance_decision(&kernel, actor, intent, ws).unwrap();
    assert!(!explanation.policies_involved.is_empty());
    assert!(!explanation.reasoning.is_empty());
    assert_eq!(explanation.authority_effect, "none");
    assert!(!explanation.actionable);
}

#[test]
fn governance_history_is_non_actionable_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_state_envelope(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::generate_governance_evaluation(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let second =
        CommandHandler::generate_governance_evaluation(&kernel, actor, intent, ws).unwrap();
    assert!(second.history.iter().all(|h| {
        h.terminal && !h.actionable && h.authority_effect == "none"
    }));
}
