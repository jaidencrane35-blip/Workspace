//! Policy & Governance Engine — Programme III Batch 2.
//!
//! Policy explains authority. It does not become authority.
//! Consumes Workspace State Envelope as context; PermissionGateway remains final.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{Database, PolicyGovernanceRepository};
use workspace_domain::{
    aggregate_results, default_policy_catalog, evaluate_policies, policy_catalog_revision,
    ActorContext, GovernanceExplanation, GovernanceRecommendation, GovernanceEvaluationStatus,
    PolicyGovernanceMeta, PolicyGovernanceSnapshot, PolicyGovernanceView,
};

use crate::error::{KernelError, Result};
use crate::services::{AuditService, WorkspaceStateCompositionService};

pub(crate) struct PolicyGovernanceService;

impl PolicyGovernanceService {
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
    ) -> Result<PolicyGovernanceSnapshot> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let view = Self::compose_view(db, &workspace_id, &now)?;

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            guard
                .run_in_transaction(|database| {
                    let repo = PolicyGovernanceRepository::new(database);
                    for policy in &view.policies {
                        repo.upsert_policy(policy)?;
                    }
                    repo.supersede_current(&workspace_id, &now)?;
                    repo.persist_view(&view)?;
                    Ok(())
                })
                .map_err(KernelError::from)?;
        }

        Self::audit(
            db,
            actor,
            "workspace.policy_governance.evaluation_generated",
            &view.meta.evaluation_set_id,
            json!({
                "workspace_id": workspace_id,
                "context_revision": view.context_revision,
                "aggregate_result": view.meta.aggregate_result,
                "evaluation_count": view.evaluations.len(),
                "authority_effect": "none",
            }),
        )?;

        Self::load_snapshot(db, workspace_id)
    }

    pub(crate) fn load_snapshot(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<PolicyGovernanceSnapshot> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = PolicyGovernanceRepository::new(&guard);
        let metas = repo.list_meta(&workspace_id)?;
        let history = repo.list_history(&workspace_id)?;
        let history_count = repo.history_count(&workspace_id)?;
        let current = metas
            .into_iter()
            .find(|m| m.status == GovernanceEvaluationStatus::Current)
            .map(|m| repo.load_view(&m))
            .transpose()?;
        Ok(PolicyGovernanceSnapshot::assemble(
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
    ) -> Result<GovernanceExplanation> {
        let snap = Self::load_snapshot(db, workspace_id)?;
        match snap.current {
            Some(view) => Ok(GovernanceExplanation::from_view(&view)),
            None => Ok(GovernanceExplanation {
                explanation_id: "governance_explanation:missing".into(),
                workspace_id: snap.workspace_id,
                context_revision: None,
                policies_involved: vec![],
                evidence: vec![],
                reasoning: vec![
                    "No governance evaluation present — EvaluationIncomplete".into(),
                ],
                uncertainty: vec![
                    "Missing evaluation evidence — never assume Compliant".into(),
                ],
                aggregate_result: None,
                authority_effect: GovernanceExplanation::AUTHORITY_EFFECT_NONE.into(),
                actionable: false,
            }),
        }
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::PolicyGovernanceValidation {
            message: "Policy engine cannot execute commands".into(),
        })
    }

    pub(crate) fn attempt_grant_permission() -> Result<()> {
        Err(KernelError::PolicyGovernanceValidation {
            message: "Policy engine cannot grant permissions".into(),
        })
    }

    pub(crate) fn attempt_bypass_gateway() -> Result<()> {
        Err(KernelError::PolicyGovernanceValidation {
            message: "Policy engine cannot bypass PermissionGateway".into(),
        })
    }

    pub(crate) fn attempt_create_capability() -> Result<()> {
        Err(KernelError::PolicyGovernanceValidation {
            message: "Policy engine cannot create capabilities".into(),
        })
    }

    pub(crate) fn attempt_mutate_lifecycle() -> Result<()> {
        Err(KernelError::PolicyGovernanceValidation {
            message: "Policy engine cannot mutate lifecycle state".into(),
        })
    }

    pub(crate) fn generate_with_forced_rollback(
        db: &Arc<Mutex<Database>>,
        _actor: &ActorContext,
        workspace_id: impl Into<String>,
    ) -> Result<()> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let view = Self::compose_view(db, &workspace_id, &now)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let err = guard.run_in_transaction(|database| {
            let repo = PolicyGovernanceRepository::new(database);
            for policy in &view.policies {
                repo.upsert_policy(policy)?;
            }
            repo.supersede_current(&workspace_id, &now)?;
            repo.persist_view(&view)?;
            Err(workspace_database::DatabaseError::Migration(
                "forced policy governance transaction rollback".into(),
            ))
        });
        match err {
            Err(_) => Ok(()),
            Ok(()) => Err(KernelError::PolicyGovernanceValidation {
                message: "expected forced rollback".into(),
            }),
        }
    }

    fn compose_view(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        now: &str,
    ) -> Result<PolicyGovernanceView> {
        // Context source: Workspace State Envelope via load_snapshot only — never generate.
        let state = WorkspaceStateCompositionService::load_snapshot(db, workspace_id)?;
        let envelope = state.current.as_ref();

        let policies = default_policy_catalog();
        let catalog_rev = policy_catalog_revision(&policies);
        let (evaluations, unknowns) =
            evaluate_policies(&policies, envelope, now).map_err(KernelError::from)?;
        let recommendation = GovernanceRecommendation::from_evaluations(&evaluations, now)
            .map_err(KernelError::from)?;
        let aggregate = aggregate_results(&evaluations);

        let meta = PolicyGovernanceMeta::new(
            workspace_id,
            now,
            envelope.map(|e| e.revision.clone()),
            catalog_rev,
            evaluations.len(),
            aggregate.as_str(),
        );

        let view = PolicyGovernanceView {
            meta,
            policies,
            evaluations,
            recommendation: Some(recommendation),
            unknowns,
            context_revision: envelope.map(|e| e.revision.clone()),
            authority_effect: PolicyGovernanceView::AUTHORITY_EFFECT_NONE.into(),
        };
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
