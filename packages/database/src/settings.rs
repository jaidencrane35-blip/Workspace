use std::collections::HashMap;

use crate::connection::Database;
use crate::error::Result;

/// SQLite-backed key/value settings store.
pub struct SettingsRepository<'a> {
    db: &'a Database,
}

impl<'a> SettingsRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn get(&self, key: &str) -> Result<Option<String>> {
        let mut stmt = self
            .db
            .connection()
            .prepare("SELECT value FROM settings WHERE key = ?1")?;

        let mut rows = stmt.query([key])?;
        if let Some(row) = rows.next()? {
            return Ok(Some(row.get(0)?));
        }
        Ok(None)
    }

    pub fn set(&self, key: &str, value: &str) -> Result<()> {
        self.db.connection().execute(
            "INSERT INTO settings (key, value, updated_at)
             VALUES (?1, ?2, datetime('now'))
             ON CONFLICT(key) DO UPDATE SET
                value = excluded.value,
                updated_at = datetime('now')",
            (key, value),
        )?;
        Ok(())
    }

    pub fn get_all(&self) -> Result<HashMap<String, String>> {
        let mut stmt = self
            .db
            .connection()
            .prepare("SELECT key, value FROM settings ORDER BY key")?;

        let rows = stmt.query_map([], |row| Ok((row.get::<_, String>(0)?, row.get(1)?)))?;

        let mut settings = HashMap::new();
        for row in rows {
            let (key, value) = row?;
            settings.insert(key, value);
        }
        Ok(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init::DatabaseService;
    use tempfile::tempdir;

    fn initialized_db() -> (tempfile::TempDir, Database) {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("workspace.db");
        let db = DatabaseService::initialize(&db_path)
            .unwrap()
            .into_database();
        (dir, db)
    }

    #[test]
    fn persists_settings() {
        let (_dir, db) = initialized_db();
        let repo = SettingsRepository::new(&db);

        repo.set("theme", "system").unwrap();
        repo.set("first_run", "true").unwrap();

        assert_eq!(repo.get("theme").unwrap(), Some("system".to_string()));
        assert_eq!(repo.get("first_run").unwrap(), Some("true".to_string()));

        repo.set("first_run", "false").unwrap();
        assert_eq!(repo.get("first_run").unwrap(), Some("false".to_string()));
    }

    #[test]
    fn returns_all_settings() {
        let (_dir, db) = initialized_db();
        let repo = SettingsRepository::new(&db);

        repo.set("theme", "dark").unwrap();
        repo.set("settings_version", "1").unwrap();

        let all = repo.get_all().unwrap();
        assert_eq!(all.get("theme"), Some(&"dark".to_string()));
        assert_eq!(all.get("settings_version"), Some(&"1".to_string()));
    }
}
