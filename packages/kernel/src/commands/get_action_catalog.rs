//! Informational action catalog query (Sprint 52).
//!
//! Returns registered actions and required capabilities. Does not grant
//! authority and does not evaluate whether the actor may execute them.

use crate::commands::context::CommandContext;
use crate::commands::r#trait::QueryCommand;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::ActionCatalogService;
use workspace_domain::{ActionCatalog, Capability};

/// Returns the informational workspace action catalog.
pub struct GetActionCatalog;

impl crate::commands::Command for GetActionCatalog {
    fn name(&self) -> &'static str {
        "GetActionCatalog"
    }
}

impl QueryCommand for GetActionCatalog {
    type Output = ActionCatalog;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        // Same read class as capability discovery — catalog metadata is sensitive.
        Capability::settings_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<ActionCatalog> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }

        let catalog = ActionCatalogService::catalog()?;
        ActionCatalogService::audit_catalog_viewed(
            &ctx.database,
            &ctx.actor_context,
            &catalog,
        )?;
        Ok(catalog)
    }
}
