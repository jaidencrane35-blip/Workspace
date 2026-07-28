use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("Migration error: {0}")]
    Migration(String),

    #[error("Record not found: {0}")]
    NotFound(String),

    #[error("Duplicate resource: {0}")]
    DuplicateResource(String),

    #[error("Invalid persistence transition: {0}")]
    InvalidTransition(String),

    /// Authoritative / evaluate-once artifacts cannot be erased or replaced.
    #[error("Immutable artifact violation: {0}")]
    ImmutableArtifact(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Transaction commit failed: {0}")]
    TransactionCommit(String),

    #[error("Transaction rollback failed: {0}")]
    TransactionRollback(String),
}

pub type Result<T> = std::result::Result<T, DatabaseError>;
