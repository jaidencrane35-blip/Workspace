//! Configuration service ownership boundary.

use workspace_database::Database;

use crate::config::{ConfigManager, SettingsUpdate, WorkspaceSettings};
use crate::error::Result;

/// Kernel-owned configuration service.
pub struct ConfigurationService;

impl ConfigurationService {
    pub(crate) fn initialize(db: &Database) -> Result<()> {
        ConfigManager::ensure_defaults(db)
    }

    pub fn load(db: &Database) -> Result<WorkspaceSettings> {
        ConfigManager::load(db)
    }

    pub(crate) fn update(db: &Database, update: SettingsUpdate) -> Result<WorkspaceSettings> {
        ConfigManager::update(db, update)
    }
}
