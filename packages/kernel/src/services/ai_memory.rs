//! Governed AI memory service — intelligence enhancement only (Sprints 60–61).
//!
//! Never calls Permission Gateway decision APIs, grant mutators, or launchers.
//! Memory may influence planning context only.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{AiMemoryRepository, Database};
use workspace_domain::{
    ActorContext, AiMemoryAwareness, IntentContext, MemoryEntry, MemoryEntryId, MemoryMetadata,
    MemoryType, WorkspaceId,
};

use crate::error::{KernelError, Result};
use crate::services::AuditService;

pub(crate) struct AiMemoryService;

impl AiMemoryService {
    pub(crate) fn create(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        memory_type: MemoryType,
        key: impl Into<String>,
        summary: impl Into<String>,
        source: impl Into<String>,
        workspace_id: Option<String>,
        attributes: Option<String>,
    ) -> Result<MemoryEntry> {
        let workspace_id = workspace_id
            .map(WorkspaceId::new)
            .transpose()
            .map_err(KernelError::Domain)?;
        let mut metadata = MemoryMetadata::default_visible();
        if let Some(attributes) = attributes {
            metadata = metadata.with_attribute_json(attributes);
        }
        let entry = MemoryEntry::new(
            memory_type,
            key,
            summary,
            source,
            workspace_id,
            metadata,
        )
        .map_err(|error| KernelError::from(error))?;

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            AiMemoryRepository::new(&guard).upsert(&entry)?;
        }

        Self::audit(db, actor, "ai.memory.created", &entry, true)?;
        Ok(entry)
    }

    pub(crate) fn list_active(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: Option<&str>,
        limit: usize,
    ) -> Result<Vec<MemoryEntry>> {
        let entries = {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            AiMemoryRepository::new(&guard).list_active(workspace_id, limit)?
        };
        Self::audit_access(db, actor, workspace_id, entries.len())?;
        Ok(entries)
    }

    pub(crate) fn assemble_awareness(
        db: &Arc<Mutex<Database>>,
        workspace_id: Option<&str>,
        limit: usize,
    ) -> Result<AiMemoryAwareness> {
        let entries = {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            AiMemoryRepository::new(&guard).list_active(workspace_id, limit)?
        };
        Ok(AiMemoryAwareness::from_entries(entries))
    }

    pub(crate) fn delete(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        id: impl Into<String>,
    ) -> Result<MemoryEntry> {
        let id = MemoryEntryId::new(id.into()).map_err(KernelError::Domain)?;
        let updated_at = Utc::now().to_rfc3339();
        let entry = {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            let repo = AiMemoryRepository::new(&guard);
            let mut entry = repo.get(&id)?.ok_or_else(|| {
                KernelError::AiMemoryValidation {
                    message: format!("memory entry not found: {}", id.as_str()),
                }
            })?;
            if !repo.soft_delete(&id, &updated_at)? {
                return Err(KernelError::AiMemoryValidation {
                    message: format!("memory entry already deleted: {}", id.as_str()),
                });
            }
            entry.mark_deleted();
            entry.updated_at = updated_at;
            entry
        };
        Self::audit(db, actor, "ai.memory.deleted", &entry, true)?;
        Ok(entry)
    }

    pub(crate) fn clear(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        memory_type: Option<MemoryType>,
        workspace_id: Option<&str>,
    ) -> Result<usize> {
        let updated_at = Utc::now().to_rfc3339();
        let cleared = {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            AiMemoryRepository::new(&guard).clear_active(memory_type, workspace_id, &updated_at)?
        };
        let metadata = json!({
            "cleared_count": cleared,
            "memory_type": memory_type.map(|t| t.as_str()),
            "workspace_id": workspace_id,
            "authority_effect": "none",
        })
        .to_string();
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "ai.memory.deleted",
            true,
            metadata,
        )?;
        Ok(cleared)
    }

    fn audit(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        event_type: &str,
        entry: &MemoryEntry,
        success: bool,
    ) -> Result<()> {
        let metadata = json!({
            "memory_id": entry.id.as_str(),
            "memory_type": entry.memory_type.as_str(),
            "key": entry.key,
            "source": entry.source,
            "workspace_id": entry.workspace_id.as_ref().map(|id| id.as_str()),
            "lifecycle": entry.lifecycle.as_str(),
            "confidence_level": entry.metadata.confidence_level,
            "authority_effect": "none",
        })
        .to_string();
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            event_type,
            success,
            metadata,
        )
    }

    fn audit_access(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: Option<&str>,
        count: usize,
    ) -> Result<()> {
        let metadata = json!({
            "workspace_id": workspace_id,
            "entry_count": count,
            "authority_effect": "none",
        })
        .to_string();
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "ai.memory.accessed",
            true,
            metadata,
        )
    }
}
