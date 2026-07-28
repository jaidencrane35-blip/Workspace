//! Workspace State Composition — Programme III Batch 1.
//!
//! Read-only composition envelope. Gathers source projections, records revisions,
//! calculates freshness/completeness, identifies contradictions, produces a
//! deterministic envelope. Never executes, mutates sources, or silently refreshes.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{Database, WorkspaceStateEnvelopeRepository};
use workspace_domain::{
    ActorContext, AvailabilityStatus, CompletenessStatus, FreshnessStatus, WorkspaceStateConflict,
    WorkspaceStateEnvelope, WorkspaceStateSnapshot, WorkspaceStateSource,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AuditService, WorkspaceCognitiveAgentCastService, WorkspaceCognitiveAutonomyService,
    WorkspaceCognitiveGraphService, WorkspaceCognitiveModelService,
    WorkspaceCognitiveOrchestrationService, WorkspaceLearningAdaptationService,
    WorkspacePlanningService, WorkspaceReasoningMemoryService,
};

pub(crate) struct WorkspaceStateCompositionService;

impl WorkspaceStateCompositionService {
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
    ) -> Result<WorkspaceStateSnapshot> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let envelope = Self::compose_envelope(db, &workspace_id, &now)?;

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            guard
                .run_in_transaction(|database| {
                    let repo = WorkspaceStateEnvelopeRepository::new(database);
                    repo.supersede_current(&workspace_id, &now)?;
                    repo.persist_envelope(&envelope)?;
                    Ok(())
                })
                .map_err(KernelError::from)?;
        }

        Self::audit(
            db,
            actor,
            "workspace.state_envelope.snapshot_generated",
            &envelope.state_id,
            json!({
                "workspace_id": workspace_id,
                "revision": envelope.revision,
                "source_count": envelope.sources.len(),
                "conflict_count": envelope.contradictions.len(),
                "freshness": envelope.freshness.as_str(),
                "completeness": envelope.completeness.as_str(),
                "authority_effect": "none",
            }),
        )?;

        Self::load_snapshot(db, workspace_id)
    }

    pub(crate) fn load_snapshot(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<WorkspaceStateSnapshot> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = WorkspaceStateEnvelopeRepository::new(&guard);
        let mut currents = repo.list_current(&workspace_id)?;
        let history = repo.list_history(&workspace_id)?;
        let history_count = repo.history_count(&workspace_id)?;
        let current = currents.pop();
        Ok(WorkspaceStateSnapshot::assemble(
            workspace_id,
            current,
            history,
            history_count,
            Utc::now().to_rfc3339(),
        ))
    }

    /// Read-only load of durable envelope bodies for historical reconstruction.
    /// Never calls generate / silent refresh.
    pub(crate) fn list_durable_envelopes(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<Vec<WorkspaceStateEnvelope>> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = WorkspaceStateEnvelopeRepository::new(&guard);
        Ok(repo.list_durable_envelopes(&workspace_id)?)
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::WorkspaceStateEnvelopeValidation {
            message: "Unified Workspace State cannot execute".into(),
        })
    }

    pub(crate) fn attempt_mutate_source() -> Result<()> {
        Err(KernelError::WorkspaceStateEnvelopeValidation {
            message: "Unified Workspace State cannot mutate sources".into(),
        })
    }

    pub(crate) fn attempt_repair() -> Result<()> {
        Err(KernelError::WorkspaceStateEnvelopeValidation {
            message: "Unified Workspace State cannot repair sources".into(),
        })
    }

    pub(crate) fn attempt_silent_refresh() -> Result<()> {
        Err(KernelError::WorkspaceStateEnvelopeValidation {
            message: "Unified Workspace State cannot silently refresh foreign services".into(),
        })
    }

    pub(crate) fn attempt_approve() -> Result<()> {
        Err(KernelError::WorkspaceStateEnvelopeValidation {
            message: "Unified Workspace State cannot approve".into(),
        })
    }

    pub(crate) fn attempt_grant() -> Result<()> {
        Err(KernelError::WorkspaceStateEnvelopeValidation {
            message: "Unified Workspace State cannot grant permissions".into(),
        })
    }

    pub(crate) fn generate_with_forced_rollback(
        db: &Arc<Mutex<Database>>,
        _actor: &ActorContext,
        workspace_id: impl Into<String>,
    ) -> Result<()> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let envelope = Self::compose_envelope(db, &workspace_id, &now)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let err = guard.run_in_transaction(|database| {
            let repo = WorkspaceStateEnvelopeRepository::new(database);
            repo.supersede_current(&workspace_id, &now)?;
            repo.persist_envelope(&envelope)?;
            Err(workspace_database::DatabaseError::Migration(
                "forced workspace state envelope transaction rollback".into(),
            ))
        });
        match err {
            Err(_) => Ok(()),
            Ok(()) => Err(KernelError::WorkspaceStateEnvelopeValidation {
                message: "expected forced rollback".into(),
            }),
        }
    }

    fn compose_envelope(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        now: &str,
    ) -> Result<WorkspaceStateEnvelope> {
        // Read-only composition — never call foreign generate mutation paths.
        let mut sources = Vec::new();
        let mut unknowns = Vec::new();
        let mut contradictions = Vec::new();

        // Cognitive model — generate_model is a read assemble from durable rows.
        match WorkspaceCognitiveModelService::generate_model(db, workspace_id) {
            Ok(model) => {
                let available = !model.nodes.is_empty() || !model.relations.is_empty();
                sources.push(WorkspaceStateSource::observe(
                    "cognitive_model",
                    format!("cognitive_model:{workspace_id}"),
                    format!("nodes:{}:relations:{}", model.nodes.len(), model.relations.len()),
                    model.generated_at.clone(),
                    if available {
                        FreshnessStatus::Fresh
                    } else {
                        FreshnessStatus::Unknown
                    },
                    if available {
                        AvailabilityStatus::Available
                    } else {
                        AvailabilityStatus::Unavailable
                    },
                    if available {
                        CompletenessStatus::Partial
                    } else {
                        CompletenessStatus::Unknown
                    },
                    if available {
                        None
                    } else {
                        Some("Cognitive model has no durable nodes/relations".into())
                    },
                ));
                if !available {
                    unknowns.push("cognitive_model contents unknown/empty".into());
                }
            }
            Err(_) => {
                sources.push(WorkspaceStateSource::unavailable(
                    "cognitive_model",
                    now,
                    "Cognitive model unavailable because durable rows could not be loaded.",
                ));
                unknowns.push("cognitive_model unavailable".into());
            }
        }

        Self::push_snapshot_source(
            &mut sources,
            &mut unknowns,
            "planning",
            WorkspacePlanningService::load_snapshot(db, workspace_id).ok(),
            |s| {
                s.current.as_ref().map(|c| {
                    (
                        c.plan.id.clone(),
                        c.plan.id.clone(),
                        s.generated_at.clone(),
                        CompletenessStatus::Complete,
                    )
                })
            },
            now,
        );
        Self::push_snapshot_source(
            &mut sources,
            &mut unknowns,
            "reasoning",
            WorkspaceReasoningMemoryService::load_snapshot(db, workspace_id).ok(),
            |s| {
                s.current.as_ref().map(|c| {
                    (
                        c.id.clone(),
                        c.id.clone(),
                        s.generated_at.clone(),
                        CompletenessStatus::Complete,
                    )
                })
            },
            now,
        );
        Self::push_snapshot_source(
            &mut sources,
            &mut unknowns,
            "cognitive_graph",
            WorkspaceCognitiveGraphService::load_snapshot(db, workspace_id).ok(),
            |s| {
                s.current.as_ref().map(|c| {
                    (
                        c.meta.id.clone(),
                        c.meta.id.clone(),
                        s.generated_at.clone(),
                        CompletenessStatus::Complete,
                    )
                })
            },
            now,
        );
        Self::push_snapshot_source(
            &mut sources,
            &mut unknowns,
            "orchestration",
            WorkspaceCognitiveOrchestrationService::load_snapshot(db, workspace_id).ok(),
            |s| {
                s.current.as_ref().map(|c| {
                    (
                        c.meta.orchestration_id.clone(),
                        c.meta.orchestration_id.clone(),
                        s.generated_at.clone(),
                        CompletenessStatus::Complete,
                    )
                })
            },
            now,
        );
        Self::push_snapshot_source(
            &mut sources,
            &mut unknowns,
            "learning",
            WorkspaceLearningAdaptationService::load_snapshot(db, workspace_id).ok(),
            |s| {
                s.current.as_ref().map(|c| {
                    (
                        c.meta.learning_id.clone(),
                        c.meta.learning_id.clone(),
                        s.generated_at.clone(),
                        CompletenessStatus::Complete,
                    )
                })
            },
            now,
        );
        Self::push_snapshot_source(
            &mut sources,
            &mut unknowns,
            "agent_cast",
            WorkspaceCognitiveAgentCastService::load_snapshot(db, workspace_id).ok(),
            |s| {
                s.current.as_ref().map(|c| {
                    (
                        c.meta.cast_id.clone(),
                        c.meta.cast_id.clone(),
                        s.generated_at.clone(),
                        CompletenessStatus::Complete,
                    )
                })
            },
            now,
        );
        Self::push_snapshot_source(
            &mut sources,
            &mut unknowns,
            "autonomy",
            WorkspaceCognitiveAutonomyService::load_snapshot(db, workspace_id).ok(),
            |s| {
                s.current.as_ref().map(|c| {
                    (
                        c.meta.autonomy_id.clone(),
                        c.meta.autonomy_id.clone(),
                        s.generated_at.clone(),
                        CompletenessStatus::Complete,
                    )
                })
            },
            now,
        );

        // Sources without a side-effect-free load path remain explicitly unavailable.
        for kind in [
            "intent",
            "task_graph",
            "execution",
            "memory",
            "attention",
            "purpose",
            "recommendation_engine",
            "decision_engine",
        ] {
            sources.push(WorkspaceStateSource::unavailable(
                kind,
                now,
                format!(
                    "{kind} unavailable because a side-effect-free revisioned load path is not yet registered for envelope composition."
                ),
            ));
            unknowns.push(format!("{kind} availability unknown — not assumed current"));
        }

        let planning_available = sources.iter().any(|s| {
            s.source_type == "planning" && s.availability_status == AvailabilityStatus::Available
        });
        let reasoning_available = sources.iter().any(|s| {
            s.source_type == "reasoning" && s.availability_status == AvailabilityStatus::Available
        });
        let autonomy_available = sources.iter().any(|s| {
            s.source_type == "autonomy" && s.availability_status == AvailabilityStatus::Available
        });

        if planning_available && !reasoning_available {
            contradictions.push(WorkspaceStateConflict::record_deterministic(
                "planning",
                "reasoning",
                "Planning present while reasoning absent",
                "medium",
                vec![
                    "planning:available".into(),
                    "reasoning:unavailable_or_empty".into(),
                ],
            ));
        }
        if autonomy_available && !planning_available {
            contradictions.push(WorkspaceStateConflict::record_deterministic(
                "autonomy",
                "planning",
                "Autonomy evidence present without planning baseline",
                "high",
                vec![
                    "autonomy:available".into(),
                    "planning:unavailable_or_empty".into(),
                ],
            ));
        }

        WorkspaceStateEnvelope::compose(
            workspace_id,
            now,
            sources,
            contradictions,
            unknowns,
        )
        .map_err(KernelError::from)
    }

    fn push_snapshot_source<T, F>(
        sources: &mut Vec<WorkspaceStateSource>,
        unknowns: &mut Vec<String>,
        source_type: &str,
        loaded: Option<T>,
        current_meta: F,
        now: &str,
    ) where
        F: FnOnce(&T) -> Option<(String, String, String, CompletenessStatus)>,
    {
        match loaded {
            Some(snapshot) => match current_meta(&snapshot) {
                Some((source_id, revision, observed_at, completeness)) => {
                    sources.push(WorkspaceStateSource::observe(
                        source_type,
                        source_id,
                        revision,
                        observed_at,
                        FreshnessStatus::Fresh,
                        AvailabilityStatus::Available,
                        completeness,
                        None,
                    ));
                }
                None => {
                    sources.push(WorkspaceStateSource::observe(
                        source_type,
                        format!("empty:{source_type}"),
                        "none",
                        now,
                        FreshnessStatus::Unknown,
                        AvailabilityStatus::Unavailable,
                        CompletenessStatus::Unknown,
                        Some(format!(
                            "{source_type} snapshot loaded but current channel is empty — not assumed current"
                        )),
                    ));
                    unknowns.push(format!("{source_type} current missing"));
                }
            },
            None => {
                sources.push(WorkspaceStateSource::unavailable(
                    source_type,
                    now,
                    format!("{source_type} unavailable because source revision could not be loaded."),
                ));
                unknowns.push(format!("{source_type} unavailable"));
            }
        }
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
