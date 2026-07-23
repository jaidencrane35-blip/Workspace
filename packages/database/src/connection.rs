use std::path::{Path, PathBuf};

use rusqlite::Connection;

use crate::encryption::{EncryptionProvider, NoOpEncryptionProvider};
use crate::error::Result;
use crate::transaction::Transaction;

/// SQLite database handle for Workspace local-first storage (DEC-010).
pub struct Database {
    connection: Connection,
    path: PathBuf,
    encryption: Box<dyn EncryptionProvider>,
}

impl Database {
    /// Opens or creates a SQLite database at the given path.
    /// Uses Tier 0 encryption (OS file protection) via NoOp provider (DEC-015).
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        Self::open_with_encryption(path, Box::new(NoOpEncryptionProvider))
    }

    /// Opens a database with a custom encryption provider (future Tier 1/2).
    pub fn open_with_encryption(
        path: impl AsRef<Path>,
        encryption: Box<dyn EncryptionProvider>,
    ) -> Result<Self> {
        let path = path.as_ref().to_path_buf();

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let connection = Connection::open(&path)?;

        Ok(Self {
            connection,
            path,
            encryption,
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn connection(&self) -> &Connection {
        &self.connection
    }

    pub fn connection_mut(&mut self) -> &mut Connection {
        &mut self.connection
    }

    pub fn encryption(&self) -> &dyn EncryptionProvider {
        self.encryption.as_ref()
    }

    /// Opens an in-memory database for tests.
    pub fn open_in_memory() -> Result<Self> {
        let connection = Connection::open_in_memory()?;
        Ok(Self {
            connection,
            path: PathBuf::from(":memory:"),
            encryption: Box::new(NoOpEncryptionProvider),
        })
    }
}

impl Database {
    /// Runs `operation` inside a transaction. Commits on success; rolls back on error.
    pub fn transaction<F, T>(&self, operation: F) -> Result<T>
    where
        F: FnOnce(&Transaction<'_>) -> Result<T>,
    {
        self.connection().execute_batch("BEGIN IMMEDIATE")?;
        let tx = Transaction::new(self.connection());

        match operation(&tx) {
            Ok(value) => {
                self.connection().execute_batch("COMMIT")?;
                Ok(value)
            }
            Err(error) => {
                let _ = self.connection().execute_batch("ROLLBACK");
                Err(error)
            }
        }
    }

    pub(crate) fn ensure_migration_table(&self) -> Result<()> {
        self.connection.execute_batch(
            "CREATE TABLE IF NOT EXISTS _workspace_migrations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                version TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                applied_at TEXT NOT NULL DEFAULT (datetime('now'))
            );",
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encryption::EncryptionTier;
    use crate::migration::MigrationRunner;
    use tempfile::tempdir;

    #[test]
    fn opens_database_file() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("workspace.db");
        let db = Database::open(&db_path).unwrap();
        assert_eq!(db.path(), db_path.as_path());
        assert_eq!(db.encryption().tier(), EncryptionTier::Tier0);
    }

    #[test]
    fn runs_migration_bootstrap() {
        let db = Database::open_in_memory().unwrap();
        let runner = MigrationRunner::new();
        runner.apply_all(&db).unwrap();

        let count: i64 = db
            .connection()
            .query_row(
                "SELECT COUNT(*) FROM _workspace_migrations",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn transaction_commits_on_success() {
        let db = Database::open_in_memory().unwrap();
        let runner = MigrationRunner::load_from_dir(crate::init::bundled_migrations_dir()).unwrap();
        runner.apply_all(&db).unwrap();

        db.transaction(|tx| {
            tx.connection().execute(
                "INSERT INTO workspaces (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
                ("tx-ws", "Transactional", "2026-07-23T10:00:00Z", "2026-07-23T10:00:00Z"),
            )?;
            Ok(())
        })
        .unwrap();

        let count: i64 = db
            .connection()
            .query_row("SELECT COUNT(*) FROM workspaces", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn transaction_rolls_back_on_error() {
        let db = Database::open_in_memory().unwrap();
        let runner = MigrationRunner::load_from_dir(crate::init::bundled_migrations_dir()).unwrap();
        runner.apply_all(&db).unwrap();

        let result: crate::error::Result<()> = db.transaction(|tx| {
            tx.connection().execute(
                "INSERT INTO workspaces (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
                ("rollback-ws", "Rollback", "2026-07-23T10:00:00Z", "2026-07-23T10:00:00Z"),
            )?;
            Err(crate::error::DatabaseError::Migration(
                "forced rollback".into(),
            ))
        });

        assert!(result.is_err());

        let count: i64 = db
            .connection()
            .query_row("SELECT COUNT(*) FROM workspaces", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }
}
