//! Governed AI assistant service — interface layer only (Sprints 58–59).
//!
//! Delegates planning/execution to existing orchestration + Permission Gateway.
//! Never launches, grants, or bypasses the command pipeline.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{
    ActorContext, AiAssistantPlanPreview, AiAssistantWorkflow, AiAssistantWorkflowState,
    AiOrchestratedPlan, IntentContext,
};

use crate::error::{KernelError, Result};
use crate::services::{ActionCatalogService, AuditService};

/// In-memory diagnostic store for assistant workflows (not durable authority).
#[derive(Default)]
pub(crate) struct AssistantWorkflowStore {
    workflows: HashMap<String, AiAssistantWorkflow>,
}

impl AssistantWorkflowStore {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn insert(&mut self, workflow: AiAssistantWorkflow) -> AiAssistantWorkflow {
        self.workflows
            .insert(workflow.id.to_string(), workflow.clone());
        workflow
    }

    pub(crate) fn get(&self, id: &str) -> Option<AiAssistantWorkflow> {
        self.workflows.get(id).cloned()
    }

    pub(crate) fn upsert(&mut self, workflow: AiAssistantWorkflow) -> AiAssistantWorkflow {
        self.workflows
            .insert(workflow.id.to_string(), workflow.clone());
        workflow
    }
}

pub(crate) struct AiAssistantService;

impl AiAssistantService {
    pub(crate) fn store_workflow(
        store: &Arc<Mutex<AssistantWorkflowStore>>,
        workflow: AiAssistantWorkflow,
    ) -> Result<AiAssistantWorkflow> {
        let mut guard = store
            .lock()
            .map_err(|_| KernelError::Config("assistant workflow store lock poisoned".into()))?;
        Ok(guard.insert(workflow))
    }

    pub(crate) fn get_workflow(
        store: &Arc<Mutex<AssistantWorkflowStore>>,
        workflow_id: &str,
    ) -> Result<AiAssistantWorkflow> {
        let guard = store
            .lock()
            .map_err(|_| KernelError::Config("assistant workflow store lock poisoned".into()))?;
        guard
            .get(workflow_id)
            .ok_or_else(|| KernelError::AiAssistantValidation {
                message: format!("assistant workflow not found: {workflow_id}"),
            })
    }

    pub(crate) fn save_workflow(
        store: &Arc<Mutex<AssistantWorkflowStore>>,
        workflow: AiAssistantWorkflow,
    ) -> Result<AiAssistantWorkflow> {
        let mut guard = store
            .lock()
            .map_err(|_| KernelError::Config("assistant workflow store lock poisoned".into()))?;
        Ok(guard.upsert(workflow))
    }

    pub(crate) fn build_plan_preview(plan: &AiOrchestratedPlan) -> AiAssistantPlanPreview {
        AiAssistantPlanPreview::from_orchestrated_plan(plan, |command| {
            ActionCatalogService::ai_awareness()
                .ok()
                .and_then(|awareness| awareness.explain_capability_for_command(command))
        })
    }

    pub(crate) fn audit_goal_received(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workflow: &AiAssistantWorkflow,
    ) -> Result<()> {
        let metadata = json!({
            "workflow_id": workflow.id.as_str(),
            "user_goal": workflow.user_goal,
            "state": workflow.state.as_str(),
        })
        .to_string();
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::ai_suggestion(),
            "ai.assistant.goal_received",
            true,
            metadata,
        )
    }

    pub(crate) fn audit_plan_presented(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workflow: &AiAssistantWorkflow,
    ) -> Result<()> {
        let metadata = json!({
            "workflow_id": workflow.id.as_str(),
            "plan_id": workflow.orchestrated_plan_id.as_ref().map(|id| id.to_string()),
            "action_count": workflow.plan_preview.as_ref().map(|p| p.actions.len()).unwrap_or(0),
            "state": workflow.state.as_str(),
        })
        .to_string();
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::ai_suggestion(),
            "ai.assistant.plan_presented",
            true,
            metadata,
        )
    }

    pub(crate) fn audit_user_confirmed(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workflow: &AiAssistantWorkflow,
    ) -> Result<()> {
        let metadata = json!({
            "workflow_id": workflow.id.as_str(),
            "plan_id": workflow.orchestrated_plan_id.as_ref().map(|id| id.to_string()),
            "state": workflow.state.as_str(),
        })
        .to_string();
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::ai_suggestion(),
            "ai.assistant.user_confirmed",
            true,
            metadata,
        )
    }

    pub(crate) fn audit_cancelled(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workflow: &AiAssistantWorkflow,
    ) -> Result<()> {
        let metadata = json!({
            "workflow_id": workflow.id.as_str(),
            "plan_id": workflow.orchestrated_plan_id.as_ref().map(|id| id.to_string()),
            "state": workflow.state.as_str(),
        })
        .to_string();
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::ai_suggestion(),
            "ai.assistant.cancelled",
            true,
            metadata,
        )
    }

    pub(crate) fn ensure_awaiting_confirmation(workflow: &AiAssistantWorkflow) -> Result<()> {
        if workflow.state != AiAssistantWorkflowState::AwaitingConfirmation {
            return Err(KernelError::AiAssistantValidation {
                message: format!(
                    "assistant workflow is not awaiting confirmation (state={})",
                    workflow.state.as_str()
                ),
            });
        }
        Ok(())
    }
}
