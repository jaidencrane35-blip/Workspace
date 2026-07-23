//! Informational action catalog service (Sprint 52).
//!
//! Lists registered actions and required capabilities without evaluating
//! permission or granting authority.

use workspace_domain::{ActionCatalog, AiActionAwareness};

use crate::error::{KernelError, Result};
use crate::services::AuditService;
use std::sync::{Arc, Mutex};
use workspace_database::Database;
use workspace_domain::{ActorContext, IntentContext};
use serde_json::json;

/// Builds the informational action catalog from the static registry.
pub struct ActionCatalogService;

impl ActionCatalogService {
    pub fn catalog() -> Result<ActionCatalog> {
        let catalog = ActionCatalog::from_registry();
        catalog
            .validate()
            .map_err(|error| KernelError::ActionCatalogValidation {
                message: error.to_string(),
            })?;
        Ok(catalog)
    }

    pub fn ai_awareness() -> Result<AiActionAwareness> {
        Ok(AiActionAwareness::from_catalog(Self::catalog()?))
    }

    pub(crate) fn audit_catalog_viewed(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        catalog: &ActionCatalog,
    ) -> Result<()> {
        let metadata = json!({
            "entry_count": catalog.entries.len(),
            "informational_only": true,
            "grants_authority": false,
        })
        .to_string();

        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "action.catalog.viewed",
            true,
            metadata,
        )
    }
}
