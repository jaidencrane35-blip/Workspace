//! Deterministic workspace analytics service — the "Learn" stage (Sprint 18).
//!
//! Consumes [`ObservationService`] output and aggregates it into
//! [`WorkspaceMetrics`]. It performs no mutation, executes no commands, grants
//! no permissions, and never touches repositories or the database API directly —
//! it delegates reads to `ObservationService` and aggregation to the pure domain
//! type.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use workspace_database::Database;
use workspace_domain::WorkspaceMetrics;

use super::ObservationService;
use crate::error::{KernelError, Result};

/// Aggregates observations into deterministic workspace metrics.
pub struct WorkspaceAnalyticsService;

impl WorkspaceAnalyticsService {
    /// Computes metrics over up to `limit` recent observations.
    pub fn compute(db: &Arc<Mutex<Database>>, limit: usize) -> Result<WorkspaceMetrics> {
        let observations = ObservationService::list_recent(db, limit)?;
        let metrics = WorkspaceMetrics::from_observations(&observations, Utc::now().to_rfc3339());

        metrics
            .validate()
            .map_err(|error| KernelError::AnalyticsValidation {
                message: error.to_string(),
            })?;

        Ok(metrics)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WorkspaceKernel;

    #[test]
    fn computes_metrics_from_observations() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        kernel.create_workspace("Analyzed".into()).unwrap();

        let metrics = WorkspaceAnalyticsService::compute(&kernel.shared_database(), 200).unwrap();

        assert!(metrics.observation_count > 0);
        assert!(metrics.resource_change_count >= 1);
        assert!(metrics.resource_creation_count >= 1);
        assert!(metrics.validate().is_ok());
    }

    #[test]
    fn category_counts_sum_to_observation_count() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        for index in 0..3 {
            kernel.create_workspace(format!("Workspace {index}")).unwrap();
        }

        let metrics = WorkspaceAnalyticsService::compute(&kernel.shared_database(), 200).unwrap();

        let sum = metrics.lifecycle_activity_count
            + metrics.resource_change_count
            + metrics.layout_change_count
            + metrics.settings_change_count;
        assert_eq!(sum, metrics.observation_count);
    }
}
