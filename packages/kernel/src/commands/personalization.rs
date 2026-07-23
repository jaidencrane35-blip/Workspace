use crate::commands::context::CommandContext;
use crate::commands::r#trait::{MutationCommand, QueryCommand};
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::AiPersonalizationService;
use workspace_domain::{
    Capability, PreferenceCategory, PreferenceSource, UserPreference, UserPreferenceProfile,
};

/// Creates an explicit user preference.
pub struct CreateUserPreference {
    pub category: PreferenceCategory,
    pub key: String,
    pub value: String,
    pub source: PreferenceSource,
    pub workspace_id: Option<String>,
    pub label: Option<String>,
    pub attributes: Option<String>,
}

impl CreateUserPreference {
    pub fn new(
        category: PreferenceCategory,
        key: String,
        value: String,
        source: PreferenceSource,
        workspace_id: Option<String>,
        label: Option<String>,
        attributes: Option<String>,
    ) -> Self {
        Self {
            category,
            key,
            value,
            source,
            workspace_id,
            label,
            attributes,
        }
    }
}

impl crate::commands::Command for CreateUserPreference {
    fn name(&self) -> &'static str {
        "CreateUserPreference"
    }
}

impl MutationCommand for CreateUserPreference {
    type Output = UserPreference;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::personalization_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<UserPreference> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        AiPersonalizationService::create(
            &ctx.database,
            &ctx.actor_context,
            self.category,
            self.key.clone(),
            self.value.clone(),
            self.source,
            self.workspace_id.clone(),
            self.label.clone(),
            self.attributes.clone(),
        )
    }
}

/// Updates an editable preference.
pub struct UpdateUserPreference {
    pub id: String,
    pub value: Option<String>,
    pub label: Option<Option<String>>,
    pub attributes: Option<Option<String>>,
    pub confidence: Option<u8>,
}

impl UpdateUserPreference {
    pub fn new(
        id: String,
        value: Option<String>,
        label: Option<Option<String>>,
        attributes: Option<Option<String>>,
        confidence: Option<u8>,
    ) -> Self {
        Self {
            id,
            value,
            label,
            attributes,
            confidence,
        }
    }
}

impl crate::commands::Command for UpdateUserPreference {
    fn name(&self) -> &'static str {
        "UpdateUserPreference"
    }
}

impl MutationCommand for UpdateUserPreference {
    type Output = UserPreference;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::personalization_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<UserPreference> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        AiPersonalizationService::update(
            &ctx.database,
            &ctx.actor_context,
            self.id.clone(),
            self.value.clone(),
            self.label.clone(),
            self.attributes.clone(),
            self.confidence,
        )
    }
}

/// Lists active preferences / profile.
pub struct GetPreferenceProfile {
    pub workspace_id: Option<String>,
    pub limit: Option<usize>,
}

impl GetPreferenceProfile {
    pub fn new(workspace_id: Option<String>, limit: Option<usize>) -> Self {
        Self {
            workspace_id,
            limit,
        }
    }
}

impl crate::commands::Command for GetPreferenceProfile {
    fn name(&self) -> &'static str {
        "GetPreferenceProfile"
    }
}

impl QueryCommand for GetPreferenceProfile {
    type Output = UserPreferenceProfile;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::personalization_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<UserPreferenceProfile> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        AiPersonalizationService::get_profile(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.as_deref(),
            self.limit.unwrap_or(50),
        )
    }
}

/// Soft-deletes a preference.
pub struct DeleteUserPreference {
    pub id: String,
}

impl DeleteUserPreference {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

impl crate::commands::Command for DeleteUserPreference {
    fn name(&self) -> &'static str {
        "DeleteUserPreference"
    }
}

impl MutationCommand for DeleteUserPreference {
    type Output = UserPreference;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::personalization_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<UserPreference> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        AiPersonalizationService::delete(&ctx.database, &ctx.actor_context, self.id.clone())
    }
}

/// Enables or disables personalization for planning.
pub struct SetPersonalizationEnabled {
    pub enabled: bool,
}

impl SetPersonalizationEnabled {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

impl crate::commands::Command for SetPersonalizationEnabled {
    fn name(&self) -> &'static str {
        "SetPersonalizationEnabled"
    }
}

impl MutationCommand for SetPersonalizationEnabled {
    type Output = bool;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::personalization_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<bool> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        AiPersonalizationService::set_enabled(&ctx.database, &ctx.actor_context, self.enabled)
    }
}
