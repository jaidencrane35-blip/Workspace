use std::sync::{Arc, Mutex};

use workspace_database::Database;

use crate::events::types::DomainEvent;
use crate::events::EventBus;
use crate::services::AuditService;

/// Persists domain events from the internal event bus to the audit trail.
pub struct AuditEventSubscriber;

impl AuditEventSubscriber {
    pub fn register(event_bus: &EventBus, database: Arc<Mutex<Database>>) {
        event_bus.subscribe(move |event: &DomainEvent| {
            if let Err(error) = AuditService::record_domain_event(&database, event) {
                log::error!(
                    "audit subscriber failed for {}: {error}",
                    event.name()
                );
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::types::WorkspaceStarted;
    use workspace_database::DatabaseService;
    use tempfile::tempdir;

    #[test]
    fn subscriber_writes_audit_on_publish() {
        let bus = EventBus::new();
        let dir = tempdir().unwrap();
        let db = Arc::new(Mutex::new(
            DatabaseService::initialize(dir.path().join("workspace.db"))
                .unwrap()
                .into_database(),
        ));

        AuditEventSubscriber::register(&bus, Arc::clone(&db));

        bus.publish(DomainEvent::WorkspaceStarted(WorkspaceStarted {
            version: "0.1.0".into(),
        }));

        let records = AuditService::list_recent(&db, 10).unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].event_type, "system.workspace.started");
    }
}
