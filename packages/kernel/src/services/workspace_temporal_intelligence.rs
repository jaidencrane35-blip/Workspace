//! Temporal Intelligence — Programme III Batch 4.
//!
//! Temporal intelligence organises and deepens historical understanding.
//! It does not predict, simulate, correct, replay, or become truth.
//! Reads durable reconstruction + envelopes only — never foreign generate.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{Database, TemporalIntelligenceRepository};
use workspace_domain::{
    ActorContext, TemporalAnalysisStatus, TemporalAnalysisView, TemporalAnalysisWindow,
    TemporalChangeExplanation, TemporalIntelligenceSnapshot,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AuditService, WorkspaceHistoricalReconstructionService, WorkspaceStateCompositionService,
};

pub(crate) struct WorkspaceTemporalIntelligenceService;

impl WorkspaceTemporalIntelligenceService {
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        window: TemporalAnalysisWindow,
    ) -> Result<TemporalIntelligenceSnapshot> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let view = Self::compose_view(db, &workspace_id, &now, window)?;

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            guard
                .run_in_transaction(|database| {
                    let repo = TemporalIntelligenceRepository::new(database);
                    repo.supersede_current(&workspace_id, &now)?;
                    repo.persist_view(&view)?;
                    Ok(())
                })
                .map_err(KernelError::from)?;
        }

        Self::audit(
            db,
            actor,
            "workspace.temporal_intelligence.analysis_generated",
            &view.analysis_id,
            json!({
                "workspace_id": workspace_id,
                "from_revision": view.window.from_revision,
                "to_revision": view.window.to_revision,
                "completeness": view.completeness.as_str(),
                "conflict_count": view.conflict_explanations.len(),
                "gap_count": view.gaps.len(),
                "authority_effect": "none",
            }),
        )?;

        Self::load_snapshot(db, workspace_id)
    }

    pub(crate) fn load_snapshot(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<TemporalIntelligenceSnapshot> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = TemporalIntelligenceRepository::new(&guard);
        let views = repo.list_views(&workspace_id)?;
        let history = repo.list_history(&workspace_id)?;
        let history_count = repo.history_count(&workspace_id)?;
        let current = views
            .into_iter()
            .find(|v| v.status == TemporalAnalysisStatus::Current);
        Ok(TemporalIntelligenceSnapshot::assemble(
            workspace_id,
            current,
            history,
            history_count,
            Utc::now().to_rfc3339(),
        ))
    }

    pub(crate) fn explain(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<TemporalChangeExplanation> {
        let snap = Self::load_snapshot(db, workspace_id)?;
        match snap.current {
            Some(view) => Ok(TemporalChangeExplanation::from_view(&view)),
            None => Ok(TemporalChangeExplanation {
                explanation_id: "temporal_explanation:missing".into(),
                workspace_id: snap.workspace_id,
                from_revision: None,
                to_revision: None,
                completeness: None,
                observed_sequences: vec![],
                conflict_descriptions: vec![],
                evidence_refs: vec![],
                gaps: vec!["No temporal analysis present — AnalysisIncomplete".into()],
                uncertainty: vec![
                    "Missing temporal analysis evidence — never invent causes or forecasts".into(),
                ],
                narrative:
                    "No temporal analysis artefact is available for this workspace.".into(),
                authority_effect: TemporalChangeExplanation::AUTHORITY_EFFECT_NONE.into(),
                actionable: false,
            }),
        }
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::TemporalIntelligenceValidation {
            message: "Temporal intelligence cannot execute or apply corrections".into(),
        })
    }

    pub(crate) fn attempt_replay() -> Result<()> {
        Err(KernelError::TemporalIntelligenceValidation {
            message: "Temporal intelligence has no replay authority".into(),
        })
    }

    pub(crate) fn attempt_simulate() -> Result<()> {
        Err(KernelError::TemporalIntelligenceValidation {
            message: "Temporal intelligence cannot simulate alternate histories".into(),
        })
    }

    pub(crate) fn attempt_forecast() -> Result<()> {
        Err(KernelError::TemporalIntelligenceValidation {
            message: "Temporal intelligence cannot forecast future states".into(),
        })
    }

    pub(crate) fn attempt_mutate_lifecycle() -> Result<()> {
        Err(KernelError::TemporalIntelligenceValidation {
            message: "Temporal intelligence cannot mutate lifecycle state".into(),
        })
    }

    pub(crate) fn attempt_fabricate_cause() -> Result<()> {
        Err(KernelError::TemporalIntelligenceValidation {
            message: "Temporal intelligence cannot invent unsupported causal narratives".into(),
        })
    }

    pub(crate) fn attempt_silent_refresh() -> Result<()> {
        Err(KernelError::TemporalIntelligenceValidation {
            message: "Temporal intelligence cannot silently refresh foreign sources".into(),
        })
    }

    pub(crate) fn generate_with_forced_rollback(
        db: &Arc<Mutex<Database>>,
        _actor: &ActorContext,
        workspace_id: impl Into<String>,
        window: TemporalAnalysisWindow,
    ) -> Result<()> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let view = Self::compose_view(db, &workspace_id, &now, window)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let err = guard.run_in_transaction(|database| {
            let repo = TemporalIntelligenceRepository::new(database);
            repo.supersede_current(&workspace_id, &now)?;
            repo.persist_view(&view)?;
            Err(workspace_database::DatabaseError::Migration(
                "forced temporal intelligence transaction rollback".into(),
            ))
        });
        match err {
            Err(_) => Ok(()),
            Ok(()) => Err(KernelError::TemporalIntelligenceValidation {
                message: "expected forced rollback".into(),
            }),
        }
    }

    fn compose_view(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        now: &str,
        window: TemporalAnalysisWindow,
    ) -> Result<TemporalAnalysisView> {
        // Evidence: reconstruction load_snapshot + durable envelopes — never generate.
        let reconstruction =
            WorkspaceHistoricalReconstructionService::load_snapshot(db, workspace_id)?;
        let envelopes =
            WorkspaceStateCompositionService::list_durable_envelopes(db, workspace_id)?;
        let view = TemporalAnalysisView::analyse(
            workspace_id,
            now,
            window,
            reconstruction.current.as_ref(),
            &envelopes,
        )
        .map_err(KernelError::from)?;
        view.validate().map_err(KernelError::from)?;
        Ok(view)
    }

    fn audit(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        event: &str,
        subject_id: &str,
        metadata: serde_json::Value,
    ) -> Result<()> {
        let _ = subject_id;
        AuditService::record_ai_planning_event(
            db,
            actor,
            &workspace_domain::IntentContext::user_request(),
            event,
            true,
            metadata.to_string(),
        )
    }
}
