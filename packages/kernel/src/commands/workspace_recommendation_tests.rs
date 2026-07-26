//! Workspace Recommendation Engine tests (Phase 5).

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, AttentionSourceType, ConceptOwnerKind, IntentContext, PLATFORM_CONCEPT_OWNERS,
    RecommendationKind, TaskPriority, WorkspaceTaskPriority,
};

fn seed(kernel: &WorkspaceKernel) -> (String, String) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Recommendation WS".into()))
        .unwrap();
    let workspace_id = workspace.id.to_string();
    let project = CommandHandler::create_project(
        kernel,
        local.clone(),
        intent.clone(),
        workspace_id.clone(),
        "Workspace Platform".into(),
        None,
        None,
    )
    .unwrap();
    let task = CommandHandler::create_task(
        kernel,
        local.clone(),
        intent.clone(),
        project.id.to_string(),
        workspace_id.clone(),
        "Ship Recommendation Engine".into(),
        TaskPriority::High,
    )
    .unwrap();
    CommandHandler::set_active_work(
        kernel,
        local.clone(),
        intent.clone(),
        workspace_id.clone(),
        Some(project.id.to_string()),
        Some(task.id.to_string()),
    )
    .unwrap();
    CommandHandler::create_work_goal(
        kernel,
        local,
        intent,
        workspace_id.clone(),
        "Ship Workspace AI v1".into(),
        Some(project.id.to_string()),
        None,
    )
    .unwrap();
    (workspace_id, project.id.to_string())
}

/// CASE 1 — Recommendations derive only from existing systems.
#[test]
fn case1_recommendations_derive_from_existing_systems() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let state = CommandHandler::generate_workspace_recommendation_engine(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(state.authority_effect, "none");
    assert!(state.evidence.iter().any(|e| e.contains("Attention")));
    assert!(state.evidence.iter().any(|e| e.contains("Purpose")));
    assert!(state.explanation.contains("suggestions only"));
    assert!(PLATFORM_CONCEPT_OWNERS.iter().any(|c| {
        c.concept == "recommendation_candidate"
            && c.owner == "WorkspaceRecommendationEngineService"
            && c.kind == ConceptOwnerKind::Aggregator
    }));
}

/// CASE 2 — Recommendations are deterministic.
#[test]
fn case2_recommendations_are_deterministic() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let a = CommandHandler::generate_workspace_recommendation_engine(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    // Within one snapshot: ids unique; kind priority order is stable.
    let mut ids: Vec<_> = a.candidates.iter().map(|c| c.id.clone()).collect();
    let before = ids.len();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), before);
    let ranks: Vec<_> = a
        .candidates
        .iter()
        .map(|c| match c.kind {
            RecommendationKind::ResolveBlocker => 0u8,
            RecommendationKind::ReviewDecision => 1,
            RecommendationKind::RestoreContext => 2,
            RecommendationKind::ContinueWork => 3,
            RecommendationKind::CompleteTask => 4,
            RecommendationKind::ReorganizeWorkspace => 5,
            RecommendationKind::ExploreOpportunity => 6,
        })
        .collect();
    let mut sorted_ranks = ranks.clone();
    sorted_ranks.sort();
    assert_eq!(ranks, sorted_ranks);
    // Stable kinds survive a second generate despite audit noise.
    let b = CommandHandler::generate_workspace_recommendation_engine(&kernel, local, intent, ws)
        .unwrap();
    assert_eq!(a.label, b.label);
    assert_eq!(a.authority_effect, b.authority_effect);
    let stable = |state: &workspace_domain::WorkspaceRecommendationEngineState| {
        let mut kinds: Vec<_> = state
            .candidates
            .iter()
            .filter(|c| {
                matches!(
                    c.kind,
                    RecommendationKind::ContinueWork
                        | RecommendationKind::CompleteTask
                        | RecommendationKind::ReorganizeWorkspace
                )
            })
            .map(|c| c.kind.as_str().to_string())
            .collect();
        kinds.sort();
        kinds
    };
    assert_eq!(stable(&a), stable(&b));
}

/// CASE 3 — Every recommendation has evidence.
#[test]
fn case3_every_recommendation_has_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let _ = CommandHandler::create_workspace_task(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Open Task Graph node".into(),
        Some(project_id),
        WorkspaceTaskPriority::High,
    )
    .unwrap();
    let state = CommandHandler::generate_workspace_recommendation_engine(
        &kernel, local, intent, ws,
    )
    .unwrap();
    assert!(!state.candidates.is_empty());
    assert!(state.candidates.iter().all(|c| {
        !c.reason.is_empty()
            && !c.impact.is_empty()
            && !c.evidence.is_empty()
            && c.authority_effect == "none"
    }));
}

/// CASE 4 — Recommendations cannot modify source systems.
#[test]
fn case4_recommendations_cannot_modify_sources() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::generate_workspace_purpose(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let _ = CommandHandler::generate_workspace_recommendation_engine(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let after = CommandHandler::generate_workspace_purpose(&kernel, local, intent, ws).unwrap();
    assert_eq!(before.label, after.label);
    assert_eq!(before.progress_percent, after.progress_percent);
    assert_eq!(before.authority_effect, after.authority_effect);
}

/// CASE 5 — Attention consumes recommendations correctly.
#[test]
fn case5_attention_consumes_recommendations() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let _ = CommandHandler::create_workspace_task(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Open work for recommendations".into(),
        Some(project_id),
        WorkspaceTaskPriority::High,
    )
    .unwrap();
    let recommendations = CommandHandler::generate_workspace_recommendation_engine(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert!(!recommendations.candidates.is_empty());
    let base = CommandHandler::generate_workspace_attention(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let enriched = crate::services::WorkspaceAttentionService::enrich_with_recommendations(
        &base,
        &recommendations,
    )
    .unwrap();
    assert!(enriched.items.len() >= base.items.len());
    assert!(enriched.items.iter().any(|i| {
        i.source_type == AttentionSourceType::RecommendationEngine
            && i.authority_effect == "none"
            && i.explanation.contains("Suggestion only")
    }));
    let intel =
        CommandHandler::generate_workspace_intelligence(&kernel, local, intent, ws).unwrap();
    assert_eq!(intel.recommendation_engine.authority_effect, "none");
    assert!(intel.attention.item_count >= base.items.len());
}

/// CASE 6 — Assistant only explains recommendations (no accept/execute path).
#[test]
fn case6_assistant_only_explains() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let work = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let assistant = CommandHandler::generate_workspace_intelligence(&kernel, local, intent, ws)
        .unwrap();
    assert_eq!(
        work.recommendation_engine.authority_effect,
        assistant.recommendation_engine.authority_effect
    );
    assert_eq!(
        work.recommendation_engine.authority_effect,
        workspace_domain::WorkspaceRecommendationEngineState::AUTHORITY_EFFECT_NONE
    );
    assert_eq!(
        work.recommendation_engine.workspace_id,
        assistant.recommendation_engine.workspace_id
    );
    assert!(!work.recommendation_engine.summary.is_empty());
    assert!(
        work.recommendation_engine.summary.contains("never execute")
            || work
                .recommendation_engine
                .explanation
                .contains("suggestions only")
    );
}

/// CASE 7 — Purpose and Evolution context appears correctly.
#[test]
fn case7_purpose_and_evolution_context() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let state = CommandHandler::generate_workspace_recommendation_engine(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(state.evidence.iter().any(|e| e.contains("Purpose")));
    assert!(state.evidence.iter().any(|e| e.contains("Evolution")));
    assert!(state
        .candidates
        .iter()
        .any(|c| c.related_purpose_label.is_some()));
}

/// CASE 8 — Recommendations survive regeneration.
#[test]
fn case8_recommendations_survive_regeneration() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let _ = CommandHandler::create_workspace_task(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Persistable open task".into(),
        Some(project_id),
        WorkspaceTaskPriority::Medium,
    )
    .unwrap();
    let a = CommandHandler::generate_workspace_recommendation_engine(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let b = CommandHandler::generate_workspace_recommendation_engine(&kernel, local, intent, ws)
        .unwrap();
    assert_eq!(a.label, b.label);
    assert_eq!(a.authority_effect, "none");
    assert!(a.candidates.iter().any(|c| {
        matches!(
            c.kind,
            RecommendationKind::CompleteTask | RecommendationKind::ContinueWork
        )
    }));
    assert!(b.candidates.iter().any(|c| {
        matches!(
            c.kind,
            RecommendationKind::CompleteTask | RecommendationKind::ContinueWork
        )
    }));
    // Stable CompleteTask ids (task UUIDs) survive regeneration.
    let a_tasks: Vec<_> = a
        .candidates
        .iter()
        .filter(|c| c.kind == RecommendationKind::CompleteTask)
        .map(|c| c.id.clone())
        .collect();
    let b_tasks: Vec<_> = b
        .candidates
        .iter()
        .filter(|c| c.kind == RecommendationKind::CompleteTask)
        .map(|c| c.id.clone())
        .collect();
    assert_eq!(a_tasks, b_tasks);
}

/// CASE 9 — Recommendations cannot bypass Permission Gateway.
#[test]
fn case9_recommendations_cannot_bypass_gateway() {
    match CommandHandler::workspace_recommendation_engine_attempt_execute() {
        Err(KernelError::WorkspaceRecommendationEngineValidation { message }) => {
            assert!(message.contains("cannot execute"));
        }
        other => panic!("expected WorkspaceRecommendationEngineValidation, got {other:?}"),
    }
}

/// CASE 10 — Existing governance / intelligence surfaces remain green.
#[test]
fn case10_governance_surfaces_remain_green() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let intel = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert_eq!(intel.authority_effect, "none");
    assert_eq!(intel.recommendation_engine.authority_effect, "none");
    assert_eq!(intel.evolution.authority_effect, "none");
    assert_eq!(intel.purpose.authority_effect, "none");
    assert_eq!(intel.composition.authority_effect, "none");
    assert_eq!(intel.environment.authority_effect, "none");
    let _ = CommandHandler::generate_workspace_evolution(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let _ = CommandHandler::generate_workspace_purpose(&kernel, local, intent, ws).unwrap();
}

/// CASE 11 — Attention-derived suggestions carry Attention's structured reasons;
/// suggestions from other models carry none.
#[test]
fn case11_attention_reasons_survive_recommendation_generation() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let intel = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let state =
        CommandHandler::generate_workspace_recommendation_engine(&kernel, local, intent, ws)
            .unwrap();

    for candidate in &state.candidates {
        match candidate.related_attention_id {
            Some(ref attention_id) => {
                let source = intel
                    .attention
                    .top_items
                    .iter()
                    .find(|i| i.id.as_str() == attention_id);
                if let Some(source) = source {
                    assert_eq!(
                        candidate.attention_reasons, source.reasons,
                        "candidate {} altered Attention reasons",
                        candidate.id
                    );
                }
            }
            None => assert!(
                candidate.attention_reasons.is_empty(),
                "candidate {} claims Attention reasons without an Attention source",
                candidate.id
            ),
        }
    }
}

/// CASE 12 — Same Attention input yields the same reasons in the same order.
#[test]
fn case12_recommendation_reason_ordering_is_deterministic() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let generate = || {
        CommandHandler::generate_workspace_recommendation_engine(
            &kernel,
            ActorContext::local_user(),
            IntentContext::user_request(),
            ws.clone(),
        )
        .unwrap()
    };
    let projection = |state: &workspace_domain::WorkspaceRecommendationEngineState| {
        state
            .candidates
            .iter()
            .map(|c| {
                (
                    c.id.clone(),
                    c.attention_reasons
                        .iter()
                        .map(|r| (r.signal.as_str(), r.source.as_str(), r.weight))
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(projection(&generate()), projection(&generate()));
}

/// CASE 13 — Generate projects durable Available lifecycle overlays.
#[test]
fn case13_generate_projects_lifecycle_available() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let state = CommandHandler::generate_workspace_recommendation_engine(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(!state.candidates.is_empty());
    for candidate in &state.candidates {
        assert_eq!(
            candidate.lifecycle_state.as_deref(),
            Some("available"),
            "candidate {} missing Available overlay",
            candidate.id
        );
        assert_eq!(candidate.authority_effect, "none");
    }
}

/// CASE 14 — Present / accept / reject record human decisions only; accept never executes.
#[test]
fn case14_recommendation_review_records_decision_only() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let state = CommandHandler::generate_workspace_recommendation_engine(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let id = state.candidates[0].id.clone();

    let presented = CommandHandler::present_recommendation(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        id.clone(),
    )
    .unwrap();
    assert_eq!(presented.lifecycle_state, "presented");
    assert!(presented.outcome.is_none());
    assert_eq!(presented.authority_effect, "none");

    let accepted = CommandHandler::accept_recommendation(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        id.clone(),
    )
    .unwrap();
    assert_eq!(accepted.lifecycle_state, "accepted");
    assert_eq!(accepted.authority_effect, "none");
    let outcome = accepted.outcome.expect("accept must record outcome");
    assert_eq!(outcome.user_decision.as_str(), "accepted");
    assert_eq!(outcome.authority_effect, "none");
    assert!(!outcome.is_system_failure());

    let after = CommandHandler::generate_workspace_recommendation_engine(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let item = after
        .candidates
        .iter()
        .find(|c| c.id == id)
        .expect("accepted candidate still projected");
    assert_eq!(item.lifecycle_state.as_deref(), Some("accepted"));
    assert_eq!(item.lifecycle_resolution_type.as_deref(), Some("accepted"));

    assert_cannot_execute(CommandHandler::workspace_recommendation_engine_attempt_execute());

    // Reject path on a different candidate (or re-seed) — use second if present.
    if let Some(other) = state.candidates.get(1).map(|c| c.id.clone()) {
        let rejected = CommandHandler::reject_recommendation(
            &kernel,
            local,
            intent,
            ws,
            other.clone(),
        )
        .unwrap();
        assert_eq!(rejected.lifecycle_state, "rejected");
        assert_eq!(rejected.authority_effect, "none");
        let reject_outcome = rejected.outcome.expect("reject must record outcome");
        assert_eq!(reject_outcome.user_decision.as_str(), "rejected");
        assert!(!reject_outcome.is_system_failure());
    }
}

/// CASE 15 — Accept from Available auto-presents then accepts; still no execution.
#[test]
fn case15_accept_from_available_auto_presents() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let state = CommandHandler::generate_workspace_recommendation_engine(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let id = state.candidates[0].id.clone();
    let accepted = CommandHandler::accept_recommendation(
        &kernel,
        local,
        intent,
        ws,
        id,
    )
    .unwrap();
    assert_eq!(accepted.lifecycle_state, "accepted");
    assert!(accepted.outcome.is_some());
    assert_eq!(accepted.authority_effect, "none");
    assert_cannot_execute(CommandHandler::workspace_recommendation_engine_attempt_execute());
}

fn assert_cannot_execute(result: Result<(), KernelError>) {
    match result {
        Err(err) => {
            let message = err.to_string().to_lowercase();
            assert!(
                message.contains("cannot execute")
                    || message.contains("cannot")
                    || message.contains("authorize"),
                "expected CannotExecute-style error, got {err}"
            );
        }
        Ok(()) => panic!("must not succeed at attempt_execute"),
    }
}

/// CASE 19 — Explanation views are non-authoritative and grounded in evidence/keys.
#[test]
fn case19_explanation_views_are_non_authoritative() {
    use workspace_domain::RecommendationProvenance;

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let state = CommandHandler::generate_workspace_recommendation_engine(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(!state.candidates.is_empty());
    for candidate in &state.candidates {
        let view = candidate
            .explanation
            .as_ref()
            .expect("explanation view projected");
        assert_eq!(view.authority_effect, "none");
        assert_eq!(candidate.authority_effect, "none");
        assert_eq!(view.why_suggested, candidate.reason);
        assert!(!view.lifecycle_note.is_empty());
        assert_eq!(view.evidence_summaries.len(), candidate.evidence.len());
        // Catalog keys only — no free-form CoT field on the view.
        assert_eq!(
            view.explanation_keys.len(),
            candidate.attention_reasons.len()
        );
        let provenance = RecommendationProvenance::from_recommendation_item(candidate);
        assert_eq!(provenance.explanation_keys, view.explanation_keys);
        assert_eq!(provenance.source_evidence, candidate.evidence);
    }
    assert_cannot_execute(CommandHandler::workspace_recommendation_engine_attempt_execute());
}

/// CASE 20 — Provenance immutable across lifecycle; terminal stays terminal; no handoff merge.
#[test]
fn case20_provenance_immutable_and_terminal_stays_terminal() {
    use workspace_domain::RecommendationProvenance;

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let state = CommandHandler::generate_workspace_recommendation_engine(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let id = state.candidates[0].id.clone();
    let before = RecommendationProvenance::from_recommendation_item(&state.candidates[0]);
    let before_evidence = state.candidates[0].evidence.clone();
    let before_reasons = state.candidates[0].attention_reasons.clone();

    CommandHandler::accept_recommendation(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        id.clone(),
    )
    .unwrap();

    let after = CommandHandler::generate_workspace_recommendation_engine(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();
    let item = after
        .candidates
        .iter()
        .find(|c| c.id == id)
        .expect("accepted candidate retained in full snapshot");
    assert_eq!(item.lifecycle_state.as_deref(), Some("accepted"));
    assert!(!item.is_active_lifecycle());
    assert_eq!(item.evidence, before_evidence);
    assert_eq!(item.attention_reasons, before_reasons);
    let after_prov = RecommendationProvenance::from_recommendation_item(item);
    assert_eq!(after_prov.source_evidence, before.source_evidence);
    assert_eq!(after_prov.reasoning_origins, before.reasoning_origins);
    assert_eq!(after_prov.explanation_keys, before.explanation_keys);
    let view = item.explanation.as_ref().expect("explanation after accept");
    assert_eq!(view.authority_effect, "none");
    assert!(view.lifecycle_note.to_lowercase().contains("decision record"));
    // Recommendation accept must not imply Decision Engine handoff.
    assert!(!view.lifecycle_note.to_lowercase().contains("planner"));
    assert_cannot_execute(CommandHandler::workspace_recommendation_engine_attempt_execute());
    assert_cannot_execute(CommandHandler::decision_engine_attempt_execute());
}

/// CASE 16 — Open overlays whose source vanished expire with an outcome (no execute).
#[test]
fn case16_orphan_overlays_expire_on_regenerate() {
    use workspace_database::RecommendationLifecycleRepository;
    use workspace_domain::{
        RecommendationLifecycleOverlay, RecommendationLifecycleState,
    };

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let _ = CommandHandler::generate_workspace_recommendation_engine(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();

    let orphan_id = "recommendation:orphan:continuity-test".to_string();
    {
        let db = kernel.shared_database();
        let guard = db.lock().unwrap();
        RecommendationLifecycleRepository::new(&guard)
            .upsert_overlay(&RecommendationLifecycleOverlay {
                workspace_id: ws.clone(),
                native_id: orphan_id.clone(),
                lifecycle_state: RecommendationLifecycleState::Available,
                created_at: "t0".into(),
                presented_at: None,
                resolved_at: None,
                resolution_type: None,
                actor_id: Some(local.actor.id.to_string()),
                outcome: None,
                content_fingerprint: Some("stale".into()),
                updated_at: "t0".into(),
                authority_effect: RecommendationLifecycleOverlay::AUTHORITY_EFFECT_NONE.into(),
            })
            .unwrap();
    }

    let _ = CommandHandler::generate_workspace_recommendation_engine(
        &kernel,
        local,
        intent,
        ws.clone(),
    )
    .unwrap();

    let db = kernel.shared_database();
    let guard = db.lock().unwrap();
    let overlay = RecommendationLifecycleRepository::new(&guard)
        .get_overlay(&ws, &orphan_id)
        .unwrap()
        .expect("orphan overlay retained as Expired");
    assert_eq!(
        overlay.lifecycle_state,
        RecommendationLifecycleState::Expired
    );
    assert_eq!(overlay.authority_effect, "none");
    let outcome = overlay.outcome.expect("expire records outcome");
    assert_eq!(outcome.user_decision.as_str(), "expired");
    assert!(!outcome.is_system_failure());
    assert_cannot_execute(CommandHandler::workspace_recommendation_engine_attempt_execute());
}

/// CASE 17 — Material content change supersedes open overlays, then opens Available.
#[test]
fn case17_content_change_supersedes_open_overlay() {
    use workspace_database::RecommendationLifecycleRepository;
    use workspace_domain::RecommendationLifecycleState;

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let state = CommandHandler::generate_workspace_recommendation_engine(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let id = state.candidates[0].id.clone();

    {
        let db = kernel.shared_database();
        let guard = db.lock().unwrap();
        let mut overlay = RecommendationLifecycleRepository::new(&guard)
            .get_overlay(&ws, &id)
            .unwrap()
            .expect("overlay from generate");
        overlay.content_fingerprint = Some("old-fingerprint-before-change".into());
        overlay.lifecycle_state = RecommendationLifecycleState::Presented;
        overlay.presented_at = Some("t-presented".into());
        RecommendationLifecycleRepository::new(&guard)
            .upsert_overlay(&overlay)
            .unwrap();
    }

    let after = CommandHandler::generate_workspace_recommendation_engine(
        &kernel,
        local,
        intent,
        ws.clone(),
    )
    .unwrap();
    let item = after
        .candidates
        .iter()
        .find(|c| c.id == id)
        .expect("live candidate");
    assert_eq!(item.lifecycle_state.as_deref(), Some("available"));
    assert!(item.is_active_lifecycle());

    let db = kernel.shared_database();
    let guard = db.lock().unwrap();
    let overlay = RecommendationLifecycleRepository::new(&guard)
        .get_overlay(&ws, &id)
        .unwrap()
        .unwrap();
    assert_eq!(
        overlay.lifecycle_state,
        RecommendationLifecycleState::Available
    );
    assert_eq!(
        overlay.content_fingerprint.as_deref(),
        Some(item.continuity_fingerprint().as_str())
    );
    assert_eq!(overlay.authority_effect, "none");
    assert_cannot_execute(CommandHandler::workspace_recommendation_engine_attempt_execute());
}

/// CASE 18 — Terminal resolutions stay out of active summary surfaces.
#[test]
fn case18_terminal_excluded_from_active_summary() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let state = CommandHandler::generate_workspace_recommendation_engine(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert!(!state.candidates.is_empty());
    let id = state.candidates[0].id.clone();
    let before_active = state
        .summary_projection(12)
        .top_candidates
        .len();

    CommandHandler::accept_recommendation(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        id.clone(),
    )
    .unwrap();

    let after = CommandHandler::generate_workspace_recommendation_engine(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();
    let accepted = after
        .candidates
        .iter()
        .find(|c| c.id == id)
        .expect("accepted remains in full snapshot for history");
    assert_eq!(accepted.lifecycle_state.as_deref(), Some("accepted"));
    assert!(!accepted.is_active_lifecycle());

    let summary = after.summary_projection(12);
    assert!(
        summary
            .top_candidates
            .iter()
            .all(|c| c.id != id && c.is_active_lifecycle()),
        "accepted must not appear in active top_candidates"
    );
    assert!(summary.candidate_count <= before_active);
    assert_eq!(summary.authority_effect, "none");
    assert_cannot_execute(CommandHandler::workspace_recommendation_engine_attempt_execute());
}
