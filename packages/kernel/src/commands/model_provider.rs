use crate::commands::context::CommandContext;
use crate::commands::r#trait::QueryCommand;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::ModelProviderService;
use workspace_domain::{Capability, ModelProviderDescriptor, ModelRequest, ModelResponse};

/// Lists registered model providers (metadata only).
pub struct ListModelProviders;

impl crate::commands::Command for ListModelProviders {
    fn name(&self) -> &'static str {
        "ListModelProviders"
    }
}

impl QueryCommand for ListModelProviders {
    type Output = Vec<ModelProviderDescriptor>;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::settings_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Vec<ModelProviderDescriptor>> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        Ok(ModelProviderService::list_providers())
    }
}

/// Returns metadata for one provider.
pub struct GetModelProviderMetadata {
    pub provider_id: String,
}

impl GetModelProviderMetadata {
    pub fn new(provider_id: String) -> Self {
        Self { provider_id }
    }
}

impl crate::commands::Command for GetModelProviderMetadata {
    fn name(&self) -> &'static str {
        "GetModelProviderMetadata"
    }
}

impl QueryCommand for GetModelProviderMetadata {
    type Output = ModelProviderDescriptor;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::settings_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<ModelProviderDescriptor> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        ModelProviderService::get_provider_metadata(&self.provider_id)
    }
}

/// Diagnostic provider invoke (intelligence only — no execution).
pub struct TestModelProviderRequest {
    pub task: String,
    pub application_ids: Vec<String>,
    pub preferred_provider_id: Option<String>,
}

impl TestModelProviderRequest {
    pub fn new(
        task: String,
        application_ids: Vec<String>,
        preferred_provider_id: Option<String>,
    ) -> Self {
        Self {
            task,
            application_ids,
            preferred_provider_id,
        }
    }
}

impl crate::commands::Command for TestModelProviderRequest {
    fn name(&self) -> &'static str {
        "TestModelProviderRequest"
    }
}

impl QueryCommand for TestModelProviderRequest {
    type Output = ModelResponse;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::settings_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<ModelResponse> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        let request = ModelRequest::planning_assist(
            self.task,
            self.application_ids,
            Some("operator_console diagnostic".into()),
        )
        .map_err(KernelError::from)?;
        ModelProviderService::invoke_with_audit(
            &ctx.database,
            &ctx.actor_context,
            &request,
            self.preferred_provider_id.as_deref(),
        )
    }
}
