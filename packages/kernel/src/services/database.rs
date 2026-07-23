//! Database service ownership boundary.

use workspace_database::Database;

/// Kernel-owned database service handle.
pub struct DatabaseServiceHandle {
    database: Database,
}

impl DatabaseServiceHandle {
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    pub fn database(&self) -> &Database {
        &self.database
    }

    pub fn into_database(self) -> Database {
        self.database
    }
}
