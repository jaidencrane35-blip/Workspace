use crate::commands::context::CommandContext;
use crate::commands::r#trait::{MutationCommand, QueryCommand};
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::AiMemoryService;
use workspace_domain::{AiMemoryAwareness, Capability, MemoryEntry, MemoryType};

/// Creates a governed memory entry (informational only).
pub struct CreateMemoryEntry {
    pub memory_type: MemoryType,
    pub key: String,
    pub summary: String,
    pub source: String,
    pub workspace_id: Option<String>,
    pub attributes: Option<String>,
}

impl CreateMemoryEntry {
    pub fn new(
        memory_type: MemoryType,
        key: String,
        summary: String,
        source: String,
        workspace_id: Option<String>,
        attributes: Option<String>,
    ) -> Self {
        Self {
            memory_type,
            key,
            summary,
            source,
            workspace_id,
            attributes,
        }
    }
}

impl crate::commands::Command for CreateMemoryEntry {
    fn name(&self) -> &'static str {
        "CreateMemoryEntry"
    }
}

impl MutationCommand for CreateMemoryEntry {
    type Output = MemoryEntry;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::memory_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<MemoryEntry> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        AiMemoryService::create(
            &ctx.database,
            &ctx.actor_context,
            self.memory_type,
            self.key.clone(),
            self.summary.clone(),
            self.source.clone(),
            self.workspace_id.clone(),
            self.attributes.clone(),
        )
    }
}

/// Lists active memory entries for diagnostic / user visibility.
pub struct ListMemoryEntries {
    pub workspace_id: Option<String>,
    pub limit: Option<usize>,
}

impl ListMemoryEntries {
    pub fn new(workspace_id: Option<String>, limit: Option<usize>) -> Self {
        Self {
            workspace_id,
            limit,
        }
    }
}

impl crate::commands::Command for ListMemoryEntries {
    fn name(&self) -> &'static str {
        "ListMemoryEntries"
    }
}

impl QueryCommand for ListMemoryEntries {
    type Output = Vec<MemoryEntry>;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::memory_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Vec<MemoryEntry>> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        AiMemoryService::list_active(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.as_deref(),
            self.limit.unwrap_or(50),
        )
    }
}

/// Soft-deletes a memory entry.
pub struct DeleteMemoryEntry {
    pub id: String,
}

impl DeleteMemoryEntry {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

impl crate::commands::Command for DeleteMemoryEntry {
    fn name(&self) -> &'static str {
        "DeleteMemoryEntry"
    }
}

impl MutationCommand for DeleteMemoryEntry {
    type Output = MemoryEntry;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::memory_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<MemoryEntry> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        AiMemoryService::delete(&ctx.database, &ctx.actor_context, self.id.clone())
    }
}

/// Clears active memory entries (optional type/workspace filter).
pub struct ClearMemoryEntries {
    pub memory_type: Option<MemoryType>,
    pub workspace_id: Option<String>,
}

impl ClearMemoryEntries {
    pub fn new(memory_type: Option<MemoryType>, workspace_id: Option<String>) -> Self {
        Self {
            memory_type,
            workspace_id,
        }
    }
}

impl crate::commands::Command for ClearMemoryEntries {
    fn name(&self) -> &'static str {
        "ClearMemoryEntries"
    }
}

impl MutationCommand for ClearMemoryEntries {
    type Output = usize;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::memory_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<usize> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        AiMemoryService::clear(
            &ctx.database,
            &ctx.actor_context,
            self.memory_type,
            self.workspace_id.as_deref(),
        )
    }
}

/// Assembles bounded memory awareness for diagnostics / planning preview.
pub struct GetMemoryContext {
    pub workspace_id: Option<String>,
    pub limit: Option<usize>,
}

impl GetMemoryContext {
    pub fn new(workspace_id: Option<String>, limit: Option<usize>) -> Self {
        Self {
            workspace_id,
            limit,
        }
    }
}

impl crate::commands::Command for GetMemoryContext {
    fn name(&self) -> &'static str {
        "GetMemoryContext"
    }
}

impl QueryCommand for GetMemoryContext {
    type Output = AiMemoryAwareness;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::memory_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<AiMemoryAwareness> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        // Access audit via list_active; assemble from the same set.
        let entries = AiMemoryService::list_active(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.as_deref(),
            self.limit.unwrap_or(20),
        )?;
        Ok(AiMemoryAwareness::from_entries(entries))
    }
}
