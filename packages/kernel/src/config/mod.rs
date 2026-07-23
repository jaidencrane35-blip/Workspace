use serde::{Deserialize, Serialize};
use workspace_database::{Database, SettingsRepository};

use crate::error::{KernelError, Result};

pub const KEY_THEME: &str = "theme";
pub const KEY_FIRST_RUN: &str = "first_run";
pub const KEY_SETTINGS_VERSION: &str = "settings_version";
pub const KEY_ACTIVE_WORKSPACE_ID: &str = "active_workspace_id";
pub const KEY_PERSONALIZATION_ENABLED: &str = "personalization_enabled";

const DEFAULT_THEME: &str = "system";
const DEFAULT_FIRST_RUN: &str = "true";
const DEFAULT_SETTINGS_VERSION: &str = "1";
const DEFAULT_PERSONALIZATION_ENABLED: &str = "true";

/// Application settings persisted in SQLite.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceSettings {
    pub theme: String,
    pub first_run: bool,
    pub settings_version: u32,
    /// Last active workspace id for session restore (Sprint 38).
    pub active_workspace_id: Option<String>,
    /// When false, planning ignores stored preferences (Sprint 65).
    pub personalization_enabled: bool,
}

impl Default for WorkspaceSettings {
    fn default() -> Self {
        Self {
            theme: DEFAULT_THEME.to_string(),
            first_run: true,
            settings_version: 1,
            active_workspace_id: None,
            personalization_enabled: true,
        }
    }
}

/// Partial settings update from IPC callers.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct SettingsUpdate {
    pub theme: Option<String>,
    pub first_run: Option<bool>,
    /// `None` leave unchanged · `Some("")` clear · `Some(id)` set.
    pub active_workspace_id: Option<String>,
    pub personalization_enabled: Option<bool>,
}

/// Owns configuration reads/writes through the database layer.
pub struct ConfigManager;

impl ConfigManager {
    pub fn load(db: &Database) -> Result<WorkspaceSettings> {
        let repo = SettingsRepository::new(db);
        let defaults = WorkspaceSettings::default();

        let theme = repo
            .get(KEY_THEME)?
            .unwrap_or_else(|| defaults.theme.clone());
        let first_run = parse_bool(
            repo.get(KEY_FIRST_RUN)?
                .unwrap_or_else(|| DEFAULT_FIRST_RUN.to_string())
                .as_str(),
        )?;
        let settings_version = parse_u32(
            repo.get(KEY_SETTINGS_VERSION)?
                .unwrap_or_else(|| DEFAULT_SETTINGS_VERSION.to_string())
                .as_str(),
        )?;
        let active_workspace_id = repo
            .get(KEY_ACTIVE_WORKSPACE_ID)?
            .filter(|value| !value.trim().is_empty());
        let personalization_enabled = parse_bool(
            repo.get(KEY_PERSONALIZATION_ENABLED)?
                .unwrap_or_else(|| DEFAULT_PERSONALIZATION_ENABLED.to_string())
                .as_str(),
        )?;

        Ok(WorkspaceSettings {
            theme,
            first_run,
            settings_version,
            active_workspace_id,
            personalization_enabled,
        })
    }

    pub fn ensure_defaults(db: &Database) -> Result<()> {
        let repo = SettingsRepository::new(db);
        let defaults = WorkspaceSettings::default();

        if repo.get(KEY_THEME)?.is_none() {
            repo.set(KEY_THEME, &defaults.theme)?;
        }
        if repo.get(KEY_FIRST_RUN)?.is_none() {
            repo.set(KEY_FIRST_RUN, DEFAULT_FIRST_RUN)?;
        }
        if repo.get(KEY_SETTINGS_VERSION)?.is_none() {
            repo.set(KEY_SETTINGS_VERSION, DEFAULT_SETTINGS_VERSION)?;
        }
        if repo.get(KEY_PERSONALIZATION_ENABLED)?.is_none() {
            repo.set(
                KEY_PERSONALIZATION_ENABLED,
                DEFAULT_PERSONALIZATION_ENABLED,
            )?;
        }

        Ok(())
    }

    pub fn update(db: &Database, update: SettingsUpdate) -> Result<WorkspaceSettings> {
        let repo = SettingsRepository::new(db);
        let mut current = Self::load(db)?;

        if let Some(theme) = update.theme {
            validate_theme(&theme)?;
            repo.set(KEY_THEME, &theme)?;
            current.theme = theme;
        }

        if let Some(first_run) = update.first_run {
            repo.set(KEY_FIRST_RUN, if first_run { "true" } else { "false" })?;
            current.first_run = first_run;
        }

        if let Some(active) = update.active_workspace_id {
            if active.trim().is_empty() {
                repo.set(KEY_ACTIVE_WORKSPACE_ID, "")?;
                current.active_workspace_id = None;
            } else {
                repo.set(KEY_ACTIVE_WORKSPACE_ID, &active)?;
                current.active_workspace_id = Some(active);
            }
        }

        if let Some(personalization_enabled) = update.personalization_enabled {
            repo.set(
                KEY_PERSONALIZATION_ENABLED,
                if personalization_enabled {
                    "true"
                } else {
                    "false"
                },
            )?;
            current.personalization_enabled = personalization_enabled;
        }

        Ok(current)
    }
}

fn validate_theme(theme: &str) -> Result<()> {
    match theme {
        "system" | "light" | "dark" => Ok(()),
        _ => Err(KernelError::InvalidSettings(format!(
            "unsupported theme: {theme}"
        ))),
    }
}

fn parse_bool(value: &str) -> Result<bool> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(KernelError::InvalidSettings(format!(
            "expected boolean string, got: {value}"
        ))),
    }
}

fn parse_u32(value: &str) -> Result<u32> {
    value
        .parse()
        .map_err(|_| KernelError::InvalidSettings(format!("expected u32, got: {value}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_database::DatabaseService;
    use tempfile::tempdir;

    fn test_db() -> (tempfile::TempDir, Database) {
        let dir = tempdir().unwrap();
        let db = DatabaseService::initialize(dir.path().join("workspace.db"))
            .unwrap()
            .into_database();
        (dir, db)
    }

    #[test]
    fn loads_default_settings() {
        let (_dir, db) = test_db();
        ConfigManager::ensure_defaults(&db).unwrap();

        let settings = ConfigManager::load(&db).unwrap();
        assert_eq!(settings.theme, "system");
        assert!(settings.first_run);
        assert_eq!(settings.settings_version, 1);
        assert!(settings.active_workspace_id.is_none());
        assert!(settings.personalization_enabled);
    }

    #[test]
    fn persists_settings_updates() {
        let (_dir, db) = test_db();
        ConfigManager::ensure_defaults(&db).unwrap();

        let updated = ConfigManager::update(
            &db,
            SettingsUpdate {
                theme: Some("dark".into()),
                first_run: Some(false),
                active_workspace_id: Some("ws-1".into()),
                personalization_enabled: None,
            },
        )
        .unwrap();

        assert_eq!(updated.theme, "dark");
        assert!(!updated.first_run);
        assert_eq!(updated.active_workspace_id.as_deref(), Some("ws-1"));

        let reloaded = ConfigManager::load(&db).unwrap();
        assert_eq!(reloaded, updated);
    }
}
