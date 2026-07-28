//! Workspace Cognitive Model service — Programme II Batch 1.
//!
//! DurableStore owner for semantic nodes/relations. Never executes.
//! Goal nodes reference WorkGoals; Intent remains WorkGoal payload SoT.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{CognitiveModelRepository, Database};
use workspace_domain::{
    ActorContext, CognitiveModelState, CognitiveNode, CognitiveNodeKind, CognitiveRelation,
    CognitiveRelationKind, IntentContext,
};

use crate::error::{KernelError, Result};
use crate::services::AuditService;

pub(crate) struct WorkspaceCognitiveModelService;

impl WorkspaceCognitiveModelService {
    pub(crate) fn create_node(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        kind: CognitiveNodeKind,
        title: impl Into<String>,
        description: Option<String>,
        importance: u8,
        confidence: u8,
        uncertainty: u8,
        external_ref: Option<String>,
        parent_id: Option<String>,
    ) -> Result<CognitiveNode> {
        let now = Utc::now().to_rfc3339();
        let node = CognitiveNode::new(
            workspace_id,
            kind,
            title,
            description,
            importance,
            confidence,
            uncertainty,
            external_ref,
            parent_id,
            now,
        )
        .map_err(KernelError::from)?;
        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            CognitiveModelRepository::new(&guard).upsert_node(&node)?;
        }
        Self::audit(db, actor, "workspace.cognitive_model.node_created", &node.id, node.kind.as_str())?;
        Ok(node)
    }

    pub(crate) fn set_current_focus(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        node_id: impl Into<String>,
    ) -> Result<CognitiveNode> {
        let workspace_id = workspace_id.into();
        let node_id = node_id.into();
        let now = Utc::now().to_rfc3339();
        let node = {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            let repo = CognitiveModelRepository::new(&guard);
            let mut node = repo.get_node(&node_id)?.ok_or_else(|| {
                KernelError::CognitiveModelValidation {
                    message: format!("cognitive node not found: {node_id}"),
                }
            })?;
            if node.workspace_id != workspace_id {
                return Err(KernelError::CognitiveModelValidation {
                    message: "cognitive node workspace mismatch".into(),
                });
            }
            repo.clear_focus(&workspace_id, &now)?;
            node.is_current_focus = true;
            node.updated_at = now;
            node.validate().map_err(KernelError::from)?;
            repo.upsert_node(&node)?;
            node
        };
        Self::audit(
            db,
            actor,
            "workspace.cognitive_model.focus_set",
            &node.id,
            node.kind.as_str(),
        )?;
        Ok(node)
    }

    pub(crate) fn create_relation(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        from_id: impl Into<String>,
        to_id: impl Into<String>,
        kind: CognitiveRelationKind,
        confidence: u8,
        explanation: Option<String>,
    ) -> Result<CognitiveRelation> {
        let workspace_id = workspace_id.into();
        let from_id = from_id.into();
        let to_id = to_id.into();
        let now = Utc::now().to_rfc3339();
        let relation = {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            let repo = CognitiveModelRepository::new(&guard);
            let from = repo.get_node(&from_id)?.ok_or_else(|| {
                KernelError::CognitiveModelValidation {
                    message: format!("from node not found: {from_id}"),
                }
            })?;
            let to = repo.get_node(&to_id)?.ok_or_else(|| {
                KernelError::CognitiveModelValidation {
                    message: format!("to node not found: {to_id}"),
                }
            })?;
            if from.workspace_id != workspace_id || to.workspace_id != workspace_id {
                return Err(KernelError::CognitiveModelValidation {
                    message: "relation endpoints must belong to workspace".into(),
                });
            }
            let relation = CognitiveRelation::new(
                workspace_id,
                from_id,
                to_id,
                kind,
                confidence,
                explanation,
                now,
            )
            .map_err(KernelError::from)?;
            repo.upsert_relation(&relation)?;
            relation
        };
        Self::audit(
            db,
            actor,
            "workspace.cognitive_model.relation_created",
            &relation.id,
            relation.kind.as_str(),
        )?;
        Ok(relation)
    }

    pub(crate) fn generate_model(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<CognitiveModelState> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = CognitiveModelRepository::new(&guard);
        let nodes = repo.list_nodes(&workspace_id)?;
        let relations = repo.list_relations(&workspace_id)?;
        Ok(CognitiveModelState::assemble(
            workspace_id,
            nodes,
            relations,
            Utc::now().to_rfc3339(),
        ))
    }

    fn audit(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        event: &str,
        subject_id: &str,
        kind: &str,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            event,
            true,
            json!({
                "subject_id": subject_id,
                "kind": kind,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }
}
