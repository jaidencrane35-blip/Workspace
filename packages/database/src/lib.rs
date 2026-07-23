//! Workspace SQLite persistence foundation.

pub mod connection;
pub mod encryption;
pub mod error;
pub mod migration;

pub use connection::Database;
pub use encryption::{EncryptionProvider, EncryptionTier, NoOpEncryptionProvider};
pub use error::DatabaseError;
pub use migration::MigrationRunner;
