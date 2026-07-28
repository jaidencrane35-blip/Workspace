//! Workspace Reasoning Memory — Programme II Batch 3.
//!
//! Durable reasoning evidence only. Composes Planning / Intent / Task Graph /
//! RE / DE / Attention / Purpose / Memory by reference. Never executes.
//! Never owns foreign lifecycles.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{Database, ReasoningMemoryRepository};
use workspace_domain::{
    ActorContext, IntentContext, ReasoningEvidenceReference, ReasoningLink, ReasoningRecord,
    ReasoningRecordStatus, ReasoningSnapshot,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AssistantWorkflowStore, AuditService, OrchestratedPlanStore, WorkspacePlanningService,
};

pub(crate) struct WorkspaceReasoningMemoryService;

impl WorkspaceReasoningMemoryService {
    /// Generate and persist a reasoning record from current planning + references.
    /// Supersedes prior current records and appends history atomically.
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
    ) -> Result<ReasoningSnapshot> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let planning = WorkspacePlanningService::load_snapshot(db, workspace_id.clone())?;
        let record = Self::compose_record(&workspace_id, &now, &planning)?;

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            guard
                .run_in_transaction(|database| {
                    let repo = ReasoningMemoryRepository::new(database);
                    repo.supersede_current(&workspace_id, &now)?;
                    repo.upsert_record(&record)?;
                    Ok(())
                })
                .map_err(KernelError::from)?;
        }

        Self::audit(
            db,
            actor,
            "workspace.reasoning.record_generated",
            &record.id,
            json!({
                "workspace_id": workspace_id,
                "confidence": record.confidence,
                "uncertainty": record.uncertainty,
                "authority_effect": "none",
            }),
        )?;

        Self::load_snapshot(db, workspace_id)
    }

    /// Generate planning then reasoning. Planning commit precedes reasoning
    /// transaction; reasoning writes are atomic (supersede + insert). Retrying
    /// after a reasoning failure reuses the durable plan without fabricating.
    pub(crate) fn generate_with_planning(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
    ) -> Result<ReasoningSnapshot> {
        let workspace_id = workspace_id.into();
        let _planning = WorkspacePlanningService::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        Self::generate(db, actor, workspace_id)
    }

    /// Reconstruct durable reasoning snapshot (restart continuity — no replay / no fabricate).
    pub(crate) fn load_snapshot(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<ReasoningSnapshot> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = ReasoningMemoryRepository::new(&guard);
        let records = repo.list_records(&workspace_id)?;
        let history = repo.list_history(&workspace_id)?;
        let history_count = repo.history_count(&workspace_id)?;
        let current = records
            .into_iter()
            .find(|r| r.status == ReasoningRecordStatus::Current);
        Ok(ReasoningSnapshot::assemble(
            workspace_id,
            current,
            history,
            history_count,
            Utc::now().to_rfc3339(),
        ))
    }

    /// Architecture guard — reasoning memory must never execute.
    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::WorkspaceReasoningMemoryValidation {
            message: "WorkspaceReasoningMemoryService cannot execute, dispatch, claim, or launch"
                .into(),
        })
    }

    /// Test helper: force mid-transaction failure after supersede+upsert staging.
    pub(crate) fn generate_with_forced_rollback(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<()> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let planning = WorkspacePlanningService::load_snapshot(db, workspace_id.clone())?;
        let record = Self::compose_record(&workspace_id, &now, &planning)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let err = guard.run_in_transaction(|database| {
            let repo = ReasoningMemoryRepository::new(database);
            repo.supersede_current(&workspace_id, &now)?;
            repo.upsert_record(&record)?;
            Err(workspace_database::DatabaseError::Migration(
                "forced reasoning transaction rollback".into(),
            ))
        });
        match err {
            Err(_) => Ok(()),
            Ok(()) => Err(KernelError::WorkspaceReasoningMemoryValidation {
                message: "expected forced rollback".into(),
            }),
        }
    }

    fn compose_record(
        workspace_id: &str,
        now: &str,
        planning: &workspace_domain::PlanningSnapshot,
    ) -> Result<ReasoningRecord> {
        let mut evidence_refs = Vec::new();
        let mut links = Vec::new();
        let mut assumptions = Vec::new();
        let mut alternatives = Vec::new();
        let mut rejected = Vec::new();
        let mut lessons = Vec::new();

        let (hypothesis, confidence, uncertainty, rationale, reflection, title) =
            if let Some(proposal) = &planning.current {
                links.push(ReasoningLink {
                    external_ref: proposal.plan.id.clone(),
                    kind: "planning".into(),
                    note: "Derived from active planning proposal".into(),
                });
                evidence_refs.push(ReasoningEvidenceReference {
                    external_ref: proposal.plan.id.clone(),
                    kind: "planning".into(),
                });
                for e in &proposal.evidence_refs {
                    evidence_refs.push(ReasoningEvidenceReference {
                        external_ref: e.external_ref.clone(),
                        kind: e.kind.clone(),
                    });
                }
                for a in &proposal.assumptions {
                    assumptions.push(a.statement.clone());
                    for r in &a.evidence_refs {
                        evidence_refs.push(ReasoningEvidenceReference {
                            external_ref: r.clone(),
                            kind: "assumption_evidence".into(),
                        });
                    }
                }
                for step in &proposal.steps {
                    for r in &step.evidence_refs {
                        evidence_refs.push(ReasoningEvidenceReference {
                            external_ref: r.clone(),
                            kind: "step_evidence".into(),
                        });
                    }
                }
                for alt in &proposal.alternatives {
                    alternatives.push(alt.title.clone());
                }
                for gap in &proposal.gaps {
                    rejected.push(format!("Deferred until gap closed: {}", gap.statement));
                }
                lessons.push(
                    "Reuse planning rationale rather than re-executing decomposition.".into(),
                );
                if !proposal.risks.is_empty() {
                    lessons.push(
                        "Surface cognitive/attention risks as uncertainty, not commands.".into(),
                    );
                }
                (
                    format!(
                        "Sequencing '{}' best satisfies current cognitive/intent focus",
                        proposal.plan.title
                    ),
                    proposal.plan.confidence,
                    proposal.plan.uncertainty,
                    proposal.explanation.why.clone(),
                    format!(
                        "{}; next: {}",
                        proposal.explanation.summary, proposal.explanation.next
                    ),
                    format!("Reasoning for {}", proposal.plan.title),
                )
            } else {
                assumptions.push(
                    "No active planning snapshot — reasoning captures absence as a gap".into(),
                );
                lessons.push(
                    "Generate a planning snapshot before expecting rich reasoning links".into(),
                );
                (
                    "Insufficient planning context to form a strong workspace hypothesis".into(),
                    40,
                    60,
                    "Reasoning memory records absence without inventing a plan".into(),
                    "Missing planning remains missing; do not fabricate sequencing.".into(),
                    "Reasoning without active plan".into(),
                )
            };

        evidence_refs.sort_by(|a, b| a.external_ref.cmp(&b.external_ref));
        evidence_refs.dedup_by(|a, b| a.external_ref == b.external_ref);

        if assumptions.is_empty() {
            assumptions.push("Referenced planning/intent state remains current".into());
        }
        if alternatives.is_empty() {
            alternatives.push("Retain current sequencing".into());
            alternatives.push("Defer secondary tasks until focus stabilizes".into());
        }
        if rejected.is_empty() {
            rejected.push("Immediate execution without governed review".into());
        }

        ReasoningRecord::new(
            workspace_id,
            title,
            hypothesis,
            assumptions,
            alternatives,
            rejected,
            evidence_refs,
            links,
            confidence,
            uncertainty,
            reflection,
            lessons,
            rationale,
            now,
        )
        .map_err(KernelError::from)
    }

    fn audit(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        event: &str,
        subject_id: &str,
        detail: serde_json::Value,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            event,
            true,
            json!({
                "subject_id": subject_id,
                "detail": detail,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }
}
