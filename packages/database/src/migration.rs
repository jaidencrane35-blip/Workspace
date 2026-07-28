use std::fs;
use std::path::Path;

use crate::connection::Database;
use crate::error::{DatabaseError, Result};

/// A single versioned SQL migration.
#[derive(Debug, Clone)]
pub struct Migration {
    pub version: String,
    pub name: String,
    pub sql: String,
}

/// Applies ordered SQL migrations from the migrations directory.
pub struct MigrationRunner {
    migrations: Vec<Migration>,
}

impl MigrationRunner {
    pub fn new() -> Self {
        Self {
            migrations: Vec::new(),
        }
    }

    /// Loads `.sql` files from a directory, sorted by filename.
    pub fn load_from_dir(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            return Ok(Self::new());
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

            db.transaction(|transaction| {
                transaction
                    .connection()
                    .execute_batch(&migration.sql)
                    .map_err(|error| {
                        DatabaseError::Migration(format!(
                            "migration '{}' failed during apply: {error}",
                            migration.version
                        ))
                    })?;
                transaction.connection().execute(
                    "INSERT INTO _workspace_migrations (version, name) VALUES (?1, ?2)",
                    (&migration.version, &migration.name),
                )?;
                Ok(())
            })?;
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

    #[test]
    fn failed_migration_rolls_back_schema_and_ledger() {
        let db = Database::open_in_memory().unwrap();
        let mut runner = MigrationRunner::new();
        runner.register(Migration {
            version: "998_atomic_failure".into(),
            name: "atomic_failure".into(),
            sql: "CREATE TABLE migration_partial (id TEXT);
                  INSERT INTO missing_table (id) VALUES ('fail');"
                .into(),
        });

        assert!(runner.apply_all(&db).is_err());
        let table_count: i64 = db
            .connection()
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master
                 WHERE type = 'table' AND name = 'migration_partial'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        let ledger_count: i64 = db
            .connection()
            .query_row(
                "SELECT COUNT(*) FROM _workspace_migrations
                 WHERE version = '998_atomic_failure'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(table_count, 0);
        assert_eq!(ledger_count, 0);
    }
}
