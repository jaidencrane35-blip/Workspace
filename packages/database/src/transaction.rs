use rusqlite::Connection;

/// Active database transaction scope.
pub struct Transaction<'conn> {
    connection: &'conn Connection,
}

impl<'conn> Transaction<'conn> {
    pub(crate) fn new(connection: &'conn Connection) -> Self {
        Self { connection }
    }

    pub fn connection(&self) -> &Connection {
        self.connection
    }
}
