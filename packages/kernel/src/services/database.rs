//! Database service ownership boundary.

use std::sync::{Arc, Mutex};

use workspace_database::Database;

/// Kernel-owned database service handle (shared for audit subscribers).
pub struct DatabaseServiceHandle {
    inner: Arc<Mutex<Database>>,
}

impl DatabaseServiceHandle {
    pub fn new(database: Database) -> Self {
        Self {
            inner: Arc::new(Mutex::new(database)),
        }
    }

    pub fn shared(&self) -> Arc<Mutex<Database>> {
        Arc::clone(&self.inner)
    }

    pub fn with_database<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Database) -> R,
    {
        let guard = self
            .inner
            .lock()
            .expect("database lock poisoned");
        f(&guard)
    }

    pub fn into_database(self) -> Database {
        match Arc::try_unwrap(self.inner) {
            Ok(mutex) => mutex.into_inner().expect("database mutex poisoned"),
            Err(_) => panic!("database handle still shared"),
        }
    }
}
