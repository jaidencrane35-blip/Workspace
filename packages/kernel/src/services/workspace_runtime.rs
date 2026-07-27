//! Workspace Runtime Projection Wiring (Sprints 182–185).
//!
//! Assembles live `WorkspaceRuntimeContext` / health / operator overview from
//! existing foundations. Read-only — never executes, scores, mutates WorkspaceState,
//! grants governance authority, or enters the Permission Gateway.

use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{
    ActorContext, CognitionContextProjection, GovernanceRuntimeSummary, IntentContext,
    OperatorContextProjection, OperatorRuntimeOverview, PublicationReadinessState,
    RuntimeArchitectureReview, RuntimeCapabilityMap, RuntimeConsistencyVerification,
    RuntimeDependencyGraph, RuntimeDiagnosticSnapshot, WorkspaceIntelligenceState,
    WorkspaceRuntimeCoherence, WorkspaceRuntimeContext, WorkspaceRuntimeHealth,
    WorkspaceRuntimeIntegrationContract, WorkspaceRuntimeOperatorView,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AssistantWorkflowStore, AuditService, OrchestratedPlanStore, WorkspaceExperienceService,
    WorkspaceIntelligenceService, WorkspaceObservationService, WorkspaceSessionService,
    WorkspaceStateEngine,
};

pub(crate) struct WorkspaceRuntimeService;

impl WorkspaceRuntimeService {
    /// Project a live operator runtime view from existing cognition foundations.
    pub(crate) fn project(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        workspace_name: impl Into<String>,
        health_label: impl Into<String>,
    ) -> Result<WorkspaceRuntimeOperatorView> {
        let workspace_id = workspace_id.into();
        let workspace_name = workspace_name.into();
        let health_label = health_label.into();

        let intelligence = WorkspaceIntelligenceService::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
            workspace_name,
            health_label,
        )?;

        let session = WorkspaceSessionService::generate_with_inputs(db, actor, &intelligence)?;
        let experience = WorkspaceExperienceService::generate_with_inputs(db, actor, &session)?;
        let experience_summary = experience.summary_projection(8);

        // WorkspaceState is observational presence — soft-fail when no capture yet.
        let workspace_state = WorkspaceStateEngine::get_current(db, actor, intent).ok();

        let governance = Self::governance_summary_from_live(&intelligence);
        let environment = intelligence.environment.clone();
        let attention = intelligence.attention.clone();
        let generated_at = intelligence.generated_at.clone();

        let ctx = WorkspaceRuntimeContext::assemble(
            workspace_id.clone(),
            generated_at,
            workspace_state,
            Some(environment),
            Some(attention),
            Some(intelligence),
            Some(experience_summary),
            governance,
        );

        let observation_status =
            WorkspaceObservationService::get_status(db, actor, intent).ok();
        let health = WorkspaceRuntimeHealth::observe_with_observation_status(
            &ctx,
            observation_status.as_ref(),
        );
        let operator = OperatorContextProjection::from_runtime_context(&ctx, &health);
        let integration = WorkspaceRuntimeIntegrationContract::audit_default(&workspace_id);
        let coherence =
            WorkspaceRuntimeCoherence::review(&ctx, &integration, &health, &operator);

        let cognition = CognitionContextProjection::project_all(&ctx);
        let graph = RuntimeDependencyGraph::canonical(&workspace_id);
        let capabilities = RuntimeCapabilityMap::inventory(&workspace_id);
        let snapshot = RuntimeDiagnosticSnapshot::capture(
            &ctx,
            &health,
            &cognition,
            &graph,
            &capabilities,
            format!("runtime_live:{}", ctx.generated_at),
        );
        let verification = RuntimeConsistencyVerification::verify(
            &graph,
            &capabilities,
            &ctx,
            &integration,
            &snapshot,
            format!("runtime_check:{}", ctx.generated_at),
        );
        let overview = OperatorRuntimeOverview::project(
            &ctx,
            &health,
            &operator,
            &snapshot,
            &verification,
            &graph,
            &capabilities,
            &coherence,
        );
        let review = RuntimeArchitectureReview::review(
            &graph,
            &capabilities,
            &verification,
            &overview,
            &coherence,
        );

        let view = WorkspaceRuntimeOperatorView::compose(
            &ctx,
            health,
            operator,
            overview,
            &coherence,
            &review,
            &verification,
            snapshot.id.clone(),
            observation_status,
        );

        AuditService::record_ai_planning_event(
            db,
            actor,
            intent,
            "workspace.runtime.projected",
            true,
            json!({
                "workspace_id": view.workspace_id,
                "runtime_context_id": view.runtime_context_id,
                "overview_id": view.overview.id,
                "health_overall": view.health.overall.as_str(),
                "coherence_ok": view.coherence_ok,
                "architecture_review_passed": view.architecture_review_passed,
                "consistency_has_errors": view.consistency_has_errors,
                "publication_blocked": view.publication_blocked,
                "authority_effect": view.authority_effect,
            })
            .to_string(),
        )?;

        Ok(view)
    }

    /// Non-authoritative governance labels from live readiness + decision queue.
    /// Never grants Allow, never unblocks publication.
    fn governance_summary_from_live(
        intelligence: &WorkspaceIntelligenceState,
    ) -> GovernanceRuntimeSummary {
        let pending = intelligence.decision_queue.pending_count as u32;
        let high = intelligence.decision_queue.high_priority_count as u32;
        let review_stage = if pending > 0 {
            Some("review_pending".into())
        } else {
            Some("none".into())
        };
        GovernanceRuntimeSummary::from_labels(
            None,
            review_stage,
            Some(intelligence.readiness.overall_status.as_str().into()),
            Some(PublicationReadinessState::Draft),
            high,
            pending,
        )
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        WorkspaceRuntimeOperatorView::attempt_execute().map_err(|error| {
            KernelError::integrity_violation(format!(
                "workspace runtime cannot execute: {error}"
            ))
        })
    }
}
