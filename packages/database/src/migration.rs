use std::fs;
use std::path::Path;

use crate::connection::Database;
use crate::error::{DatabaseError, Result};

include!(concat!(env!("OUT_DIR"), "/bundled_migrations.rs"));

/// A single versioned SQL migration.
#[derive(Debug, Clone)]
pub struct Migration {
    pub version: String,
    pub name: String,
    pub sql: String,
}

/// Applies ordered SQL migrations from the migrations directory.
#[derive(Debug)]
pub struct MigrationRunner {
    migrations: Vec<Migration>,
}

impl MigrationRunner {
    pub fn new() -> Self {
        Self {
            migrations: Vec::new(),
        }
    }

    /// Migrations embedded at compile time for installable builds.
    ///
    /// Release binaries must not read SQL from a developer checkout path.
    pub fn bundled() -> Self {
        let mut runner = Self::new();
        for migration in compiled_migrations() {
            runner.register(migration);
        }
        runner
    }

    /// Loads `.sql` files from a directory, sorted by filename.
    ///
    /// Fails closed if the directory is missing or contains no `.sql` files.
    pub fn load_from_dir(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            return Err(DatabaseError::Migration(format!(
                "migrations directory not found: {}",
                path.display()
            )));
        }

        let mut migrations = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_path = entry.path();
            if file_path.extension().and_then(|s| s.to_str()) != Some("sql") {
                continue;
            }

            let file_name = file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or_else(|| DatabaseError::Migration("Invalid migration filename".into()))?
                .to_string();

            let sql = fs::read_to_string(&file_path)?;
            migrations.push(Migration {
                version: file_name.clone(),
                name: file_name,
                sql,
            });
        }

        if migrations.is_empty() {
            return Err(DatabaseError::Migration(format!(
                "no .sql migrations found in {}",
                path.display()
            )));
        }

        migrations.sort_by(|a, b| a.version.cmp(&b.version));

        Ok(Self { migrations })
    }

    pub fn register(&mut self, migration: Migration) {
        self.migrations.push(migration);
        self.migrations.sort_by(|a, b| a.version.cmp(&b.version));
    }

    pub fn apply_all(&self, db: &Database) -> Result<()> {
        db.ensure_migration_table()?;

        for migration in &self.migrations {
            let already_applied: bool = db.connection().query_row(
                "SELECT COUNT(*) > 0 FROM _workspace_migrations WHERE version = ?1",
                [&migration.version],
                |row| row.get(0),
            )?;

            if already_applied {
                continue;
            }

            db.connection().execute_batch(&migration.sql).map_err(|error| {
                DatabaseError::Migration(format!(
                    "migration '{}' failed during apply: {error}",
                    migration.version
                ))
            })?;

            db.connection().execute(
                "INSERT INTO _workspace_migrations (version, name) VALUES (?1, ?2)",
                (&migration.version, &migration.name),
            )?;
        }

        Ok(())
    }
}

impl Default for MigrationRunner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connection::Database;

    #[test]
    fn applies_registered_migration_once() {
        let db = Database::open_in_memory().unwrap();
        let mut runner = MigrationRunner::new();
        runner.register(Migration {
            version: "001_bootstrap".into(),
            name: "bootstrap".into(),
            sql: "SELECT 1;".into(),
        });

        runner.apply_all(&db).unwrap();
        runner.apply_all(&db).unwrap();

        let count: i64 = db
            .connection()
            .query_row(
                "SELECT COUNT(*) FROM _workspace_migrations WHERE version = '001_bootstrap'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn bundled_migrations_include_product_proof_schema() {
        let runner = MigrationRunner::bundled();
        assert!(
            runner.migrations.len() >= 40,
            "expected full migration set, got {}",
            runner.migrations.len()
        );
        assert!(runner
            .migrations
            .iter()
            .any(|migration| migration.version == "001_settings"));
        assert!(runner
            .migrations
            .iter()
            .any(|migration| migration.version == "041_saved_context"));
        assert!(runner
            .migrations
            .iter()
            .any(|migration| migration.version == "044_pilot_measurement"));
        assert!(runner
            .migrations
            .iter()
            .any(|migration| migration.version == "045_workspace_persistent_session"));
        assert!(runner
            .migrations
            .iter()
            .any(|migration| migration.version == "046_workspace_session_recovery_fence"));
    }

    #[test]
    fn load_from_dir_fails_closed_when_missing() {
        let error = MigrationRunner::load_from_dir("definitely-missing-migrations-dir")
            .unwrap_err();
        match error {
            DatabaseError::Migration(message) => {
                assert!(message.contains("not found"));
            }
            other => panic!("expected migration error, got {other:?}"),
        }
    }

    #[test]
    fn migration_failure_returns_clear_error() {
        let db = Database::open_in_memory().unwrap();
        let mut runner = MigrationRunner::new();
        runner.register(Migration {
            version: "999_invalid".into(),
            name: "invalid".into(),
            sql: "CREATE TABLE bad syntax ;".into(),
        });

        let error = runner.apply_all(&db).unwrap_err();
        match error {
            DatabaseError::Migration(message) => {
                assert!(message.contains("999_invalid"));
            }
            other => panic!("expected migration error, got {other:?}"),
        }
    }
}
