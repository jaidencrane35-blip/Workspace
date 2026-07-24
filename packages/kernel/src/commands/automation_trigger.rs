use crate::commands::context::CommandContext;
use crate::commands::r#trait::{MutationCommand, QueryCommand};
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::TriggerEvaluatorService;
use workspace_domain::{
    AutomationIntentProposal, AutomationIntentProposalStatus, Capability, TriggerEvaluationResult,
    TriggerEvent, TriggerEventType,
};

pub struct RecordTriggerEvent {
    pub workspace_id: String,
    pub event_type: TriggerEventType,
    pub source: String,
    pub context: String,
    pub project_id: Option<String>,
    pub task_id: Option<String>,
}

impl RecordTriggerEvent {
    pub fn new(
        workspace_id: String,
        event_type: TriggerEventType,
        source: String,
        context: String,
        project_id: Option<String>,
        task_id: Option<String>,
    ) -> Self {
        Self {
            workspace_id,
            event_type,
            source,
            context,
            project_id,
            task_id,
        }
    }
}

impl crate::commands::Command for RecordTriggerEvent {
    fn name(&self) -> &'static str {
        "RecordTriggerEvent"
    }
}

impl MutationCommand for RecordTriggerEvent {
    type Output = TriggerEvent;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<TriggerEvent> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        TriggerEvaluatorService::record_event(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            self.event_type,
            self.source.clone(),
            self.context.clone(),
            self.project_id.clone(),
            self.task_id.clone(),
        )
    }
}

pub struct EvaluateTriggers {
    pub trigger_event_id: String,
}

impl EvaluateTriggers {
    pub fn new(trigger_event_id: String) -> Self {
        Self { trigger_event_id }
    }
}

impl crate::commands::Command for EvaluateTriggers {
    fn name(&self) -> &'static str {
        "EvaluateTriggers"
    }
}

impl MutationCommand for EvaluateTriggers {
    type Output = TriggerEvaluationResult;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<TriggerEvaluationResult> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        TriggerEvaluatorService::evaluate(
            &ctx.database,
            &ctx.actor_context,
            &ctx.capability_set,
            self.trigger_event_id.clone(),
        )
    }
}

pub struct RecordAndEvaluateTriggers {
    pub workspace_id: String,
    pub event_type: TriggerEventType,
    pub source: String,
    pub context: String,
    pub project_id: Option<String>,
    pub task_id: Option<String>,
}

impl RecordAndEvaluateTriggers {
    pub fn new(
        workspace_id: String,
        event_type: TriggerEventType,
        source: String,
        context: String,
        project_id: Option<String>,
        task_id: Option<String>,
    ) -> Self {
        Self {
            workspace_id,
            event_type,
            source,
            context,
            project_id,
            task_id,
        }
    }
}

impl crate::commands::Command for RecordAndEvaluateTriggers {
    fn name(&self) -> &'static str {
        "RecordAndEvaluateTriggers"
    }
}

impl MutationCommand for RecordAndEvaluateTriggers {
    type Output = TriggerEvaluationResult;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<TriggerEvaluationResult> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        TriggerEvaluatorService::record_and_evaluate(
            &ctx.database,
            &ctx.actor_context,
            &ctx.capability_set,
            self.workspace_id.clone(),
            self.event_type,
            self.source.clone(),
            self.context.clone(),
            self.project_id.clone(),
            self.task_id.clone(),
        )
    }
}

pub struct ListTriggerEvents {
    pub workspace_id: String,
    pub limit: Option<usize>,
}

impl ListTriggerEvents {
    pub fn new(workspace_id: String, limit: Option<usize>) -> Self {
        Self {
            workspace_id,
            limit,
        }
    }
}

impl crate::commands::Command for ListTriggerEvents {
    fn name(&self) -> &'static str {
        "ListTriggerEvents"
    }
}

impl QueryCommand for ListTriggerEvents {
    type Output = Vec<TriggerEvent>;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Vec<TriggerEvent>> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        TriggerEvaluatorService::list_events(&ctx.database, self.workspace_id, self.limit)
    }
}

pub struct ListAutomationIntentProposals {
    pub workspace_id: String,
    pub status: Option<AutomationIntentProposalStatus>,
    pub limit: Option<usize>,
}

impl ListAutomationIntentProposals {
    pub fn new(
        workspace_id: String,
        status: Option<AutomationIntentProposalStatus>,
        limit: Option<usize>,
    ) -> Self {
        Self {
            workspace_id,
            status,
            limit,
        }
    }
}

impl crate::commands::Command for ListAutomationIntentProposals {
    fn name(&self) -> &'static str {
        "ListAutomationIntentProposals"
    }
}

impl QueryCommand for ListAutomationIntentProposals {
    type Output = Vec<AutomationIntentProposal>;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Vec<AutomationIntentProposal>> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        TriggerEvaluatorService::list_proposals(
            &ctx.database,
            self.workspace_id,
            self.status,
            self.limit,
        )
    }
}

pub struct AcceptAutomationIntentProposal {
    pub proposal_id: String,
}

impl AcceptAutomationIntentProposal {
    pub fn new(proposal_id: String) -> Self {
        Self { proposal_id }
    }
}

impl crate::commands::Command for AcceptAutomationIntentProposal {
    fn name(&self) -> &'static str {
        "AcceptAutomationIntentProposal"
    }
}

impl MutationCommand for AcceptAutomationIntentProposal {
    type Output = AutomationIntentProposal;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<AutomationIntentProposal> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        TriggerEvaluatorService::accept_proposal(
            &ctx.database,
            &ctx.actor_context,
            self.proposal_id.clone(),
        )
    }
}

pub struct RejectAutomationIntentProposal {
    pub proposal_id: String,
}

impl RejectAutomationIntentProposal {
    pub fn new(proposal_id: String) -> Self {
        Self { proposal_id }
    }
}

impl crate::commands::Command for RejectAutomationIntentProposal {
    fn name(&self) -> &'static str {
        "RejectAutomationIntentProposal"
    }
}

impl MutationCommand for RejectAutomationIntentProposal {
    type Output = AutomationIntentProposal;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<AutomationIntentProposal> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        TriggerEvaluatorService::reject_proposal(
            &ctx.database,
            &ctx.actor_context,
            self.proposal_id.clone(),
        )
    }
}
