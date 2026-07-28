//! Programme III Batch 11 — Workspace Decision Support contract tests.

use workspace_domain::{
    recovery_must_not_fabricate_decision_support, ActorContext, DecisionSupportCompleteness,
    IntentContext,
};

use crate::commands::handler::CommandHandler;
use crate::services::WorkspaceDecisionSupportService;
use crate::WorkspaceKernel;

fn seed_workspace(kernel: &WorkspaceKernel) -> String {
    CommandHandler::create_workspace(
        kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        "Decision Support WS".into(),
    )
    .unwrap()
    .id
    .to_string()
}

fn seed_upstream_evidence(kernel: &WorkspaceKernel, ws: &str) {
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    for _ in 0..2 {
        CommandHandler::generate_workspace_state_envelope(
            kernel,
            actor.clone(),
            intent.clone(),
            ws.to_string(),
        )
        .unwrap();
    }
    CommandHandler::generate_governance_evaluation(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
    )
    .unwrap();
    CommandHandler::generate_historical_workspace_view(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
    )
    .unwrap();
    CommandHandler::generate_temporal_analysis(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
    )
    .unwrap();
    CommandHandler::generate_workspace_explanation(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
    )
    .unwrap();
    CommandHandler::generate_contextual_workspace_understanding(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
    )
    .unwrap();
    CommandHandler::generate_workspace_knowledge_synthesis(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
    )
    .unwrap();
    CommandHandler::generate_workspace_knowledge_integration(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
    )
    .unwrap();
    CommandHandler::generate_insight_coordination(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
    )
    .unwrap();
}

fn seed_second_workspace_for_cross(kernel: &WorkspaceKernel) -> String {
    let other = seed_workspace(kernel);
    seed_upstream_evidence(kernel, &other);
    other
}

#[test]
fn decision_support_is_non_executing_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let snap = CommandHandler::generate_workspace_decision_support(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(snap.is_non_commandable());
    assert!(recovery_must_not_fabricate_decision_support(&snap));
    let current = snap.current.as_ref().expect("current decision support");
    assert!(current.is_non_executing());
    assert_eq!(current.authority_effect, "none");
    assert!(!current.actionable);
    assert!(!current.contexts.is_empty());
    assert!(current
        .contexts
        .iter()
        .all(|c| !c.participating_evidence.is_empty()));
}

#[test]
fn missing_upstreams_produce_gaps_not_invention() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let snap = CommandHandler::generate_workspace_decision_support(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let current = snap.current.as_ref().unwrap();
    assert!(
        current.completeness == DecisionSupportCompleteness::Unavailable
            || current.completeness == DecisionSupportCompleteness::Partial
            || current.completeness == DecisionSupportCompleteness::Unknown
    );
    assert!(!current.gaps.is_empty());
    assert!(current.gaps.iter().all(|g| !g.actionable));
}

#[test]
fn decision_support_recomputation_supersedes_and_separates_history() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let first = CommandHandler::generate_workspace_decision_support(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let first_id = first.current.as_ref().unwrap().support_id.clone();
    let second = CommandHandler::generate_workspace_decision_support(
        &kernel,
        actor,
        intent,
        ws,
    )
    .unwrap();
    assert_ne!(
        second.current.as_ref().unwrap().support_id,
        first_id
    );
    assert!(second.history_count >= 1);
    assert!(second.history.iter().all(|h| h.is_non_actionable()));
}

#[test]
fn decision_support_restart_continuity_without_fabricating() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let generated = CommandHandler::generate_workspace_decision_support(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws.clone(),
    )
    .unwrap();
    let loaded = CommandHandler::get_workspace_decision_support(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(
        loaded.current.as_ref().map(|c| &c.support_id),
        generated.current.as_ref().map(|c| &c.support_id)
    );
    assert!(recovery_must_not_fabricate_decision_support(&loaded));
}

#[test]
fn empty_decision_support_remains_missing_until_generated() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let snap = CommandHandler::get_workspace_decision_support(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(snap.current.is_none());
    assert_eq!(snap.history_count, 0);
    assert!(recovery_must_not_fabricate_decision_support(&snap));
}

#[test]
fn decision_support_transaction_rollback_leaves_no_partial_write() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::generate_workspace_decision_support(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    WorkspaceDecisionSupportService::generate_with_forced_rollback(
        &kernel.shared_database(),
        &actor,
        ws.clone(),
        workspace_domain::DecisionSupportFrame::all_surfaces(),
    )
    .unwrap();
    let after =
        CommandHandler::get_workspace_decision_support(&kernel, actor, intent, ws).unwrap();
    assert_eq!(
        before.current.as_ref().unwrap().support_id,
        after.current.as_ref().unwrap().support_id
    );
    assert_eq!(before.history_count, after.history_count);
}

#[test]
fn decision_support_negative_authority_guards() {
    assert!(WorkspaceDecisionSupportService::attempt_execute().is_err());
    assert!(WorkspaceDecisionSupportService::attempt_create_task().is_err());
    assert!(WorkspaceDecisionSupportService::attempt_mutate_lifecycle().is_err());
    assert!(WorkspaceDecisionSupportService::attempt_grant_permissions().is_err());
    assert!(WorkspaceDecisionSupportService::attempt_approve_policy().is_err());
    assert!(WorkspaceDecisionSupportService::attempt_create_recommendation().is_err());
    assert!(WorkspaceDecisionSupportService::attempt_make_decision().is_err());
    assert!(WorkspaceDecisionSupportService::attempt_invent_missing_evidence().is_err());
    assert!(
        WorkspaceDecisionSupportService::attempt_convert_tradeoff_to_recommendation().is_err()
    );
    assert!(WorkspaceDecisionSupportService::attempt_convert_comparison_to_ranking().is_err());
    assert!(WorkspaceDecisionSupportService::attempt_silent_refresh().is_err());
    assert!(WorkspaceDecisionSupportService::attempt_emit_command().is_err());
    assert!(CommandHandler::decision_support_attempt_execute().is_err());
}

#[test]
fn decision_support_capability_deny_without_write() {
    use workspace_domain::CapabilitySet;

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let mut ctx =
        kernel.command_context(ActorContext::local_user(), IntentContext::user_request());
    ctx.capability_set = CapabilitySet::new();
    let err = crate::commands::pipeline::CommandPipeline::new(ctx)
        .execute_mutation(
            crate::commands::workspace_decision_support::GenerateWorkspaceDecisionSupport::new(ws),
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
fn explain_decision_support_is_non_actionable() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    CommandHandler::generate_workspace_decision_support(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws.clone(),
    )
    .unwrap();
    let explanation = CommandHandler::explain_workspace_decision_support(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(!explanation.actionable);
    assert_eq!(explanation.authority_effect, "none");
}

#[test]
fn decision_support_history_is_non_actionable_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::generate_workspace_decision_support(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let snap =
        CommandHandler::generate_workspace_decision_support(&kernel, actor, intent, ws).unwrap();
    assert!(snap.history.iter().all(|h| {
        h.terminal && !h.actionable && h.authority_effect == "none"
    }));
    let summary = CommandHandler::get_workspace_decision_support_summary(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        snap.workspace_id.clone(),
        10,
    )
    .unwrap();
    assert!(summary.history_count >= summary.history.len());
}

#[test]
fn dependencies_are_meaning_only_not_executional() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let snap = CommandHandler::generate_workspace_decision_support(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let current = snap.current.as_ref().unwrap();
    assert!(!current.dependencies.is_empty());
    for dep in &current.dependencies {
        assert!(!matches!(
            dep.kind.as_str(),
            "should_execute" | "requires_action" | "causes" | "triggers" | "authorises"
        ));
        assert!(!dep.actionable);
    }
}

#[test]
fn tradeoffs_are_descriptive_with_policy_and_contextual() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let snap = CommandHandler::generate_workspace_decision_support(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let current = snap.current.as_ref().unwrap();
    assert!(!current.tradeoffs.is_empty());
    for tradeoff in &current.tradeoffs {
        assert!(!tradeoff.actionable);
        assert_eq!(tradeoff.authority_effect, "none");
        let blob = format!(
            "{} {} {}",
            tradeoff.title, tradeoff.description, tradeoff.evidence_quality_notes.join(" ")
        )
        .to_lowercase();
        assert!(!blob.contains("recommended option"));
        assert!(!blob.contains("best choice"));
    }
}

#[test]
fn cross_workspace_included_as_evidence_reference_when_present() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let _other = seed_second_workspace_for_cross(&kernel);
    CommandHandler::generate_cross_workspace_intelligence(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
    )
    .unwrap();
    let snap = CommandHandler::generate_workspace_decision_support(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let current = snap.current.as_ref().unwrap();
    assert!(current
        .provenance_links
        .iter()
        .any(|p| p.origin_domain == "cross_workspace_intelligence"));
}

fn dependency_is_meaning_only(kind: &str) -> bool {
    matches!(
        kind,
        "depends_on_evidence" | "relates_to" | "supported_by" | "references"
    )
}

#[test]
fn evidence_bundle_partitions_supporting_and_unavailable() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let snap = CommandHandler::generate_workspace_decision_support(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let current = snap.current.as_ref().unwrap();
    assert!(!current.evidence_bundles.is_empty());
    let bundle = &current.evidence_bundles[0];
    assert!(!bundle.supporting.is_empty());
    assert!(bundle.is_non_actionable());
    assert!(current
        .dependencies
        .iter()
        .all(|d| dependency_is_meaning_only(&d.kind)));
}
