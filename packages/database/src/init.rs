use std::path::{Path, PathBuf};

use crate::connection::Database;
use crate::error::Result;
use crate::migration::MigrationRunner;

/// Manages database open, migration, and lifecycle (Sprint 02).
pub struct DatabaseService {
    database: Database,
}

impl DatabaseService {
    /// Opens the database and applies bundled migrations from this crate.
    pub fn initialize(path: impl AsRef<Path>) -> Result<Self> {
        Self::initialize_with_migrations(path, bundled_migrations_dir())
    }

    /// Opens the database and applies migrations from the given directory.
    pub fn initialize_with_migrations(
        path: impl AsRef<Path>,
        migrations_dir: impl AsRef<Path>,
    ) -> Result<Self> {
        let database = Database::open(path)?;
        let runner = MigrationRunner::load_from_dir(migrations_dir)?;
        runner.apply_all(&database)?;

        Ok(Self { database })
    }

    pub fn database(&self) -> &Database {
        &self.database
    }

    pub fn database_mut(&mut self) -> &mut Database {
        &mut self.database
    }

    pub fn into_database(self) -> Database {
        self.database
    }
}

/// Path to SQL migrations shipped with workspace-database.
pub fn bundled_migrations_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("migrations")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::SettingsRepository;
    use tempfile::tempdir;

    #[test]
    fn initializes_and_applies_settings_migration() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("workspace.db");

        let service = DatabaseService::initialize(&db_path).unwrap();

        let repo = SettingsRepository::new(service.database());
        repo.set("theme", "dark").unwrap();
        assert_eq!(repo.get("theme").unwrap(), Some("dark".to_string()));
    }
}
