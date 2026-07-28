//! Programme II Batch 1 — Cognitive Model commands.

use workspace_domain::{
    Capability, CognitiveModelState, CognitiveNode, CognitiveNodeKind, CognitiveRelation,
    CognitiveRelationKind,
};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceCognitiveModelService;

pub struct CreateCognitiveNode {
    pub workspace_id: String,
    pub kind: String,
    pub title: String,
    pub description: Option<String>,
    pub importance: u8,
    pub confidence: u8,
    pub uncertainty: u8,
    pub external_ref: Option<String>,
    pub parent_id: Option<String>,
}

impl CreateCognitiveNode {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        workspace_id: impl Into<String>,
        kind: impl Into<String>,
        title: impl Into<String>,
        description: Option<String>,
        importance: u8,
        confidence: u8,
        uncertainty: u8,
        external_ref: Option<String>,
        parent_id: Option<String>,
    ) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            kind: kind.into(),
            title: title.into(),
            description,
            importance,
            confidence,
            uncertainty,
            external_ref,
            parent_id,
        }
    }
}

impl Command for CreateCognitiveNode {
    fn name(&self) -> &'static str {
        "CreateCognitiveNode"
    }
}

impl MutationCommand for CreateCognitiveNode {
    type Output = CognitiveNode;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<Self::Output> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        let kind = CognitiveNodeKind::parse(&self.kind).map_err(KernelError::from)?;
        WorkspaceCognitiveModelService::create_node(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            kind,
            self.title.clone(),
            self.description.clone(),
            self.importance,
            self.confidence,
            self.uncertainty,
            self.external_ref.clone(),
            self.parent_id.clone(),
        )
    }
}

pub struct SetCognitiveFocus {
    pub workspace_id: String,
    pub node_id: String,
}

impl SetCognitiveFocus {
    pub fn new(workspace_id: impl Into<String>, node_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            node_id: node_id.into(),
        }
    }
}

impl Command for SetCognitiveFocus {
    fn name(&self) -> &'static str {
        "SetCognitiveFocus"
    }
}

impl MutationCommand for SetCognitiveFocus {
    type Output = CognitiveNode;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<Self::Output> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        WorkspaceCognitiveModelService::set_current_focus(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            self.node_id.clone(),
        )
    }
}

pub struct CreateCognitiveRelation {
    pub workspace_id: String,
    pub from_id: String,
    pub to_id: String,
    pub kind: String,
    pub confidence: u8,
    pub explanation: Option<String>,
}

impl CreateCognitiveRelation {
    pub fn new(
        workspace_id: impl Into<String>,
        from_id: impl Into<String>,
        to_id: impl Into<String>,
        kind: impl Into<String>,
        confidence: u8,
        explanation: Option<String>,
    ) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            from_id: from_id.into(),
            to_id: to_id.into(),
            kind: kind.into(),
            confidence,
            explanation,
        }
    }
}

impl Command for CreateCognitiveRelation {
    fn name(&self) -> &'static str {
        "CreateCognitiveRelation"
    }
}

impl MutationCommand for CreateCognitiveRelation {
    type Output = CognitiveRelation;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<Self::Output> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        let kind = CognitiveRelationKind::parse(&self.kind).map_err(KernelError::from)?;
        WorkspaceCognitiveModelService::create_relation(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            self.from_id.clone(),
            self.to_id.clone(),
            kind,
            self.confidence,
            self.explanation.clone(),
        )
    }
}

pub struct GenerateCognitiveModel {
    pub workspace_id: String,
}

impl GenerateCognitiveModel {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GenerateCognitiveModel {
    fn name(&self) -> &'static str {
        "GenerateCognitiveModel"
    }
}

impl QueryCommand for GenerateCognitiveModel {
    type Output = CognitiveModelState;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Self::Output> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        WorkspaceCognitiveModelService::generate_model(&ctx.database, self.workspace_id)
    }
}
