use crate::commands::context::CommandContext;
use crate::commands::r#trait::{MutationCommand, QueryCommand};
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::AutomationContractService;
use workspace_domain::{
    AutomationContract, AutomationContractIntentRequest, AutomationTriggerKind, Capability,
};

pub struct CreateAutomationContract {
    pub workspace_id: String,
    pub project_id: String,
    pub task_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub trigger_kind: AutomationTriggerKind,
    pub trigger_definition: Option<String>,
    pub intent_statement: String,
    pub required_capabilities: Vec<String>,
}

impl CreateAutomationContract {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        workspace_id: String,
        project_id: String,
        task_id: Option<String>,
        name: String,
        description: Option<String>,
        trigger_kind: AutomationTriggerKind,
        trigger_definition: Option<String>,
        intent_statement: String,
        required_capabilities: Vec<String>,
    ) -> Self {
        Self {
            workspace_id,
            project_id,
            task_id,
            name,
            description,
            trigger_kind,
            trigger_definition,
            intent_statement,
            required_capabilities,
        }
    }
}

impl crate::commands::Command for CreateAutomationContract {
    fn name(&self) -> &'static str {
        "CreateAutomationContract"
    }
}

impl MutationCommand for CreateAutomationContract {
    type Output = AutomationContract;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<AutomationContract> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        AutomationContractService::create(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            self.project_id.clone(),
            self.task_id.clone(),
            self.name.clone(),
            self.description.clone(),
            self.trigger_kind,
            self.trigger_definition.clone(),
            self.intent_statement.clone(),
            self.required_capabilities.clone(),
        )
    }
}

pub struct UpdateAutomationContract {
    pub contract_id: String,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub intent_statement: Option<String>,
    pub trigger_kind: Option<AutomationTriggerKind>,
    pub trigger_definition: Option<String>,
    pub required_capabilities: Option<Vec<String>>,
}

impl UpdateAutomationContract {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        contract_id: String,
        name: Option<String>,
        description: Option<Option<String>>,
        intent_statement: Option<String>,
        trigger_kind: Option<AutomationTriggerKind>,
        trigger_definition: Option<String>,
        required_capabilities: Option<Vec<String>>,
    ) -> Self {
        Self {
            contract_id,
            name,
            description,
            intent_statement,
            trigger_kind,
            trigger_definition,
            required_capabilities,
        }
    }
}

impl crate::commands::Command for UpdateAutomationContract {
    fn name(&self) -> &'static str {
        "UpdateAutomationContract"
    }
}

impl MutationCommand for UpdateAutomationContract {
    type Output = AutomationContract;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<AutomationContract> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        AutomationContractService::update(
            &ctx.database,
            &ctx.actor_context,
            self.contract_id.clone(),
            self.name.clone(),
            self.description.clone(),
            self.intent_statement.clone(),
            self.trigger_kind,
            self.trigger_definition.clone(),
            self.required_capabilities.clone(),
        )
    }
}

pub struct RequestAutomationContractApproval {
    pub contract_id: String,
}

impl RequestAutomationContractApproval {
    pub fn new(contract_id: String) -> Self {
        Self { contract_id }
    }
}

impl crate::commands::Command for RequestAutomationContractApproval {
    fn name(&self) -> &'static str {
        "RequestAutomationContractApproval"
    }
}

impl MutationCommand for RequestAutomationContractApproval {
    type Output = AutomationContract;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<AutomationContract> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        AutomationContractService::request_approval(
            &ctx.database,
            &ctx.actor_context,
            self.contract_id.clone(),
        )
    }
}

pub struct ApproveAutomationContract {
    pub contract_id: String,
}

impl ApproveAutomationContract {
    pub fn new(contract_id: String) -> Self {
        Self { contract_id }
    }
}

impl crate::commands::Command for ApproveAutomationContract {
    fn name(&self) -> &'static str {
        "ApproveAutomationContract"
    }
}

impl MutationCommand for ApproveAutomationContract {
    type Output = AutomationContract;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<AutomationContract> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        AutomationContractService::approve(
            &ctx.database,
            &ctx.actor_context,
            self.contract_id.clone(),
        )
    }
}

pub struct PauseAutomationContract {
    pub contract_id: String,
}

impl PauseAutomationContract {
    pub fn new(contract_id: String) -> Self {
        Self { contract_id }
    }
}

impl crate::commands::Command for PauseAutomationContract {
    fn name(&self) -> &'static str {
        "PauseAutomationContract"
    }
}

impl MutationCommand for PauseAutomationContract {
    type Output = AutomationContract;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<AutomationContract> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        AutomationContractService::pause(
            &ctx.database,
            &ctx.actor_context,
            self.contract_id.clone(),
        )
    }
}

pub struct RevokeAutomationContract {
    pub contract_id: String,
}

impl RevokeAutomationContract {
    pub fn new(contract_id: String) -> Self {
        Self { contract_id }
    }
}

impl crate::commands::Command for RevokeAutomationContract {
    fn name(&self) -> &'static str {
        "RevokeAutomationContract"
    }
}

impl MutationCommand for RevokeAutomationContract {
    type Output = AutomationContract;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<AutomationContract> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        AutomationContractService::revoke(
            &ctx.database,
            &ctx.actor_context,
            self.contract_id.clone(),
        )
    }
}

pub struct GetAutomationContract {
    pub contract_id: String,
}

impl GetAutomationContract {
    pub fn new(contract_id: String) -> Self {
        Self { contract_id }
    }
}

impl crate::commands::Command for GetAutomationContract {
    fn name(&self) -> &'static str {
        "GetAutomationContract"
    }
}

impl QueryCommand for GetAutomationContract {
    type Output = AutomationContract;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<AutomationContract> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        AutomationContractService::get(&ctx.database, self.contract_id)
    }
}

pub struct ListAutomationContracts {
    pub workspace_id: String,
    pub limit: Option<usize>,
}

impl ListAutomationContracts {
    pub fn new(workspace_id: String, limit: Option<usize>) -> Self {
        Self {
            workspace_id,
            limit,
        }
    }
}

impl crate::commands::Command for ListAutomationContracts {
    fn name(&self) -> &'static str {
        "ListAutomationContracts"
    }
}

impl QueryCommand for ListAutomationContracts {
    type Output = Vec<AutomationContract>;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Vec<AutomationContract>> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        AutomationContractService::list(&ctx.database, self.workspace_id, self.limit)
    }
}

/// Integration boundary only — materializes a future Intent template.
/// Does not execute, launch, approve permissions, or grant capabilities.
pub struct PrepareAutomationContractIntent {
    pub contract_id: String,
}

impl PrepareAutomationContractIntent {
    pub fn new(contract_id: String) -> Self {
        Self { contract_id }
    }
}

impl crate::commands::Command for PrepareAutomationContractIntent {
    fn name(&self) -> &'static str {
        "PrepareAutomationContractIntent"
    }
}

impl QueryCommand for PrepareAutomationContractIntent {
    type Output = AutomationContractIntentRequest;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<AutomationContractIntentRequest> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        AutomationContractService::prepare_intent_request(&ctx.database, self.contract_id)
    }
}
