//! Derived observation service — read-only interpretation of the audit trail
//! (Sprint 17).
//!
//! Observations are a projection over already-recorded activity, mirroring the
//! way [`super::WorkspaceProjectionService`] projects over authoritative domain
//! state. The audit trail is the durable record of what happened; this service
//! reinterprets it into neutral, consumer-friendly observations without adding
//! any new persistence, subscriber, or mutable state.

use std::sync::{Arc, Mutex};

use workspace_database::Database;
use workspace_domain::{classify_event, neutral_summary, Observation};

use super::AuditService;
use crate::error::{KernelError, Result};

/// Maximum number of audit rows to scan when deriving observations. Bounds the
/// read window so filtered-out bookkeeping rows cannot force an unbounded scan.
const MAX_AUDIT_SCAN: usize = 500;

/// Derives neutral observations from the persisted audit trail.
pub struct ObservationService;

impl ObservationService {
    /// Returns up to `limit` recent observations, most recent first.
    ///
    /// Reads a bounded audit window, keeps only rows that classify as domain
    /// activity (skipping command/read bookkeeping), maps them into validated
    /// observations, then truncates to `limit`.
    pub fn list_recent(
        db: &Arc<Mutex<Database>>,
        limit: usize,
    ) -> Result<Vec<Observation>> {
        let scan = limit.saturating_mul(4).clamp(limit.max(1), MAX_AUDIT_SCAN);
        let audit_events = AuditService::list_recent(db, scan)?;

        let mut observations = Vec::new();
        for event in audit_events {
            let Some((category, importance)) = classify_event(&event.event_type) else {
                continue;
            };

            let observation = Observation {
                id: event.id.to_string(),
                occurred_at: event.timestamp,
                category,
                importance,
                summary: neutral_summary(category, &event.event_type),
                source_event_type: event.event_type,
                actor_type: event.actor_type,
                actor_id: event.actor_id,
                resource_ref: event.resource_ref,
                metadata: event.metadata,
            };

            observation
                .validate()
                .map_err(|error| KernelError::ObservationValidation {
                    message: error.to_string(),
                })?;

            observations.push(observation);

            if observations.len() >= limit {
                break;
            }
        }

        Ok(observations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WorkspaceKernel;

    #[test]
    fn derives_observations_and_excludes_command_bookkeeping() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        kernel.create_workspace("Observed".into()).unwrap();

        let observations =
            ObservationService::list_recent(&kernel.shared_database(), 100).unwrap();

        assert!(observations
            .iter()
            .any(|obs| obs.source_event_type == "workspace.entity.created"));
        assert!(observations
            .iter()
            .all(|obs| obs.source_event_type != "command.executed"));
        assert!(observations.iter().all(|obs| obs.validate().is_ok()));
    }

    #[test]
    fn respects_limit() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        for index in 0..5 {
            kernel.create_workspace(format!("Workspace {index}")).unwrap();
        }

        let observations =
            ObservationService::list_recent(&kernel.shared_database(), 2).unwrap();
        assert_eq!(observations.len(), 2);
    }
}
