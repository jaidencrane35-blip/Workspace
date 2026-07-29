//! Programme IV Batch 4 — Workspace Evidence Coverage Engine contract tests.

use workspace_domain::{
    recovery_must_not_fabricate_evidence_coverage, ActorContext, CoverageScope,
    EvidenceCoverageCompleteness, IntentContext,
};

use crate::commands::handler::CommandHandler;
use crate::services::WorkspaceEvidenceCoverageService;
use crate::WorkspaceKernel;

fn seed_workspace(kernel: &WorkspaceKernel) -> String {
    CommandHandler::create_workspace(
        kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        "Evidence Coverage WS".into(),
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
    CommandHandler::generate_workspace_decision_support(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
    )
    .unwrap();
    CommandHandler::generate_workspace_intelligence_hub(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
    )
    .unwrap();
    CommandHandler::generate_workspace_semantic_query(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
        "knowledge state".into(),
    )
    .unwrap();
    CommandHandler::generate_workspace_evidence_navigation(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
    )
    .unwrap();
    CommandHandler::generate_workspace_evidence_trace(
        kernel,
        actor,
        intent,
        ws.to_string(),
        "knowledge".into(),
    )
    .unwrap();
}

#[test]
fn evidence_coverage_is_non_executing() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let snap = CommandHandler::generate_workspace_evidence_coverage(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(snap.is_non_commandable());
    assert!(recovery_must_not_fabricate_evidence_coverage(&snap));
    let current = snap.current.as_ref().expect("current evidence coverage");
    assert!(current.is_non_executing());
    assert_eq!(current.authority_effect, "none");
    assert!(!current.actionable);
}

#[test]
fn unavailable_upstreams_preserved_as_gaps() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let snap = CommandHandler::generate_workspace_evidence_coverage(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let current = snap.current.as_ref().unwrap();
    assert!(
        current.completeness == EvidenceCoverageCompleteness::Unavailable
            || current.completeness == EvidenceCoverageCompleteness::Partial
            || current.completeness == EvidenceCoverageCompleteness::Unknown
    );
    assert!(!current.gaps.is_empty());
    assert!(current.gaps.iter().all(|g| !g.actionable));
}

#[test]
fn evidence_coverage_recomputation_supersedes_history() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let first = CommandHandler::generate_workspace_evidence_coverage(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let first_id = first.current.as_ref().unwrap().coverage_id.clone();
    let second = CommandHandler::generate_workspace_evidence_coverage(
        &kernel,
        actor,
        intent,
        ws,
    )
    .unwrap();
    assert_ne!(second.current.as_ref().unwrap().coverage_id, first_id);
    assert!(second.history_count >= 1);
    assert!(second.history.iter().all(|h| h.is_non_actionable()));
}

#[test]
fn evidence_coverage_restart_continuity_without_fabricating() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let generated = CommandHandler::generate_workspace_evidence_coverage(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws.clone(),
    )
    .unwrap();
    let loaded = CommandHandler::get_workspace_evidence_coverage(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(
        loaded.current.as_ref().map(|c| &c.coverage_id),
        generated.current.as_ref().map(|c| &c.coverage_id)
    );
    assert!(recovery_must_not_fabricate_evidence_coverage(&loaded));
}

#[test]
fn empty_evidence_coverage_remains_missing_until_generated() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let snap = CommandHandler::get_workspace_evidence_coverage(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(snap.current.is_none());
    assert_eq!(snap.history_count, 0);
    assert!(recovery_must_not_fabricate_evidence_coverage(&snap));
}

#[test]
fn evidence_coverage_transaction_rollback_leaves_no_partial_write() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::generate_workspace_evidence_coverage(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    WorkspaceEvidenceCoverageService::generate_with_forced_rollback(
        &kernel.shared_database(),
        &actor,
        ws.clone(),
        CoverageScope::all_surfaces(),
    )
    .unwrap();
    let after =
        CommandHandler::get_workspace_evidence_coverage(&kernel, actor, intent, ws).unwrap();
    assert_eq!(
        before.current.as_ref().unwrap().coverage_id,
        after.current.as_ref().unwrap().coverage_id
    );
    assert_eq!(before.history_count, after.history_count);
}

#[test]
fn evidence_coverage_negative_authority_guards() {
    assert!(WorkspaceEvidenceCoverageService::attempt_execute().is_err());
    assert!(WorkspaceEvidenceCoverageService::attempt_create_recommendation().is_err());
    assert!(WorkspaceEvidenceCoverageService::attempt_infer_evidence().is_err());
    assert!(WorkspaceEvidenceCoverageService::attempt_measure_truth().is_err());
    assert!(WorkspaceEvidenceCoverageService::attempt_fabricate_completeness().is_err());
    assert!(WorkspaceEvidenceCoverageService::attempt_repair_lineage().is_err());
    assert!(WorkspaceEvidenceCoverageService::attempt_estimate_coverage().is_err());
    assert!(WorkspaceEvidenceCoverageService::attempt_silent_refresh().is_err());
    assert!(WorkspaceEvidenceCoverageService::attempt_emit_command().is_err());
    assert!(CommandHandler::evidence_coverage_attempt_execute().is_err());
}

#[test]
fn evidence_coverage_capability_deny_without_write() {
    use workspace_domain::CapabilitySet;

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let mut ctx =
        kernel.command_context(ActorContext::local_user(), IntentContext::user_request());
    ctx.capability_set = CapabilitySet::new();
    let err = crate::commands::pipeline::CommandPipeline::new(ctx)
        .execute_mutation(
            crate::commands::workspace_evidence_coverage::GenerateWorkspaceEvidenceCoverage::new(
                ws,
            ),
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
fn explain_evidence_coverage_is_non_actionable() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    CommandHandler::generate_workspace_evidence_coverage(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws.clone(),
    )
    .unwrap();
    let explanation = CommandHandler::explain_evidence_coverage(
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
fn evidence_coverage_history_is_non_actionable_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::generate_workspace_evidence_coverage(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let snap =
        CommandHandler::generate_workspace_evidence_coverage(&kernel, actor, intent, ws).unwrap();
    assert!(snap.history.iter().all(|h| {
        h.terminal && !h.actionable && h.authority_effect == "none"
    }));
    let summary = CommandHandler::get_workspace_evidence_coverage_summary(
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
fn evidence_coverage_service_never_calls_foreign_generate() {
    let source = include_str!("../services/workspace_evidence_coverage.rs");
    assert!(!source.contains("WorkspaceEvidenceTraceService::generate"));
    assert!(!source.contains("WorkspaceEvidenceNavigationService::generate"));
    assert!(!source.contains("WorkspaceSemanticQueryService::generate"));
    assert!(!source.contains("WorkspaceIntelligenceHubService::generate"));
    assert!(!source.contains("WorkspaceStateCompositionService::generate"));
    assert!(!source.contains("DecisionEngineService::"));
    assert!(!source.contains("WorkspaceRecommendationEngineService::"));
    assert!(!source.contains("ExecutionLifecycleService::"));
}

#[test]
fn no_execution_paths_in_evidence_coverage_service() {
    let source = include_str!("../services/workspace_evidence_coverage.rs");
    assert!(!source.contains("ExecutionLifecycle"));
    assert!(!source.contains("PermissionGateway"));
    assert!(!source.contains("CommandPipeline"));
}

#[test]
fn no_fabricated_coverage_without_upstream() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let snap = CommandHandler::generate_workspace_evidence_coverage(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let current = snap.current.as_ref().unwrap();
    assert!(current.lineage.contributing_artefacts.is_empty());
    // Empty durable projections are still consulted via load_snapshot — never fabricate artefacts.
    assert!(current.assessment.available_evidence.is_empty());
    assert_eq!(current.metrics.reachable_artefacts, 0);
    assert!(current.metrics.unavailable_snapshots > 0);
}

#[test]
fn no_inferred_completeness_when_sources_unavailable() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let snap = CommandHandler::generate_workspace_evidence_coverage(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let current = snap.current.as_ref().unwrap();
    assert_ne!(current.completeness, EvidenceCoverageCompleteness::Complete);
    assert!(current
        .diagnostics
        .notes
        .iter()
        .any(|n| n.contains("observational")));
}
