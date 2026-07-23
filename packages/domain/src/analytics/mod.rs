//! Deterministic workspace analytics — the "Learn" stage over observations
//! (Sprint 18).
//!
//! Analytics answers "what patterns exist in what happened?" purely by counting
//! and grouping [`Observation`] records. This is deterministic aggregation — no
//! scoring, prediction, ranking, anomaly detection, AI, or recommendations.
//! The type is pure: no database, IO, or UI dependencies.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::observation::{Observation, ObservationCategory};

/// Count of observations belonging to a single category.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryActivity {
    pub category: ObservationCategory,
    pub count: usize,
}

/// Analytics-specific validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum AnalyticsError {
    #[error("Metrics timestamp must not be empty")]
    EmptyTimestamp,

    #[error("Category counts ({sum}) do not match observation count ({observation_count})")]
    InconsistentCounts {
        observation_count: usize,
        sum: usize,
    },

    #[error(
        "Resource creation+deletion ({resource_events}) exceed resource change count ({resource_change_count})"
    )]
    InconsistentResourceCounts {
        resource_change_count: usize,
        resource_events: usize,
    },
}

/// A deterministic summary of workspace activity derived from observations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceMetrics {
    /// RFC 3339 timestamp of when the metrics were computed.
    pub generated_at: String,
    /// Total number of observations aggregated.
    pub observation_count: usize,
    pub lifecycle_activity_count: usize,
    pub resource_change_count: usize,
    pub layout_change_count: usize,
    pub settings_change_count: usize,
    /// Resource-change observations representing creation (`*.created`).
    pub resource_creation_count: usize,
    /// Resource-change observations representing deletion (`*.deleted`).
    pub resource_deletion_count: usize,
    /// Per-category breakdown, ordered deterministically.
    pub activity_by_category: Vec<CategoryActivity>,
}

impl WorkspaceMetrics {
    /// Deterministically aggregates observations into workspace metrics.
    ///
    /// `generated_at` is supplied by the caller to keep aggregation pure and
    /// reproducible for a given set of inputs.
    pub fn from_observations(
        observations: &[Observation],
        generated_at: impl Into<String>,
    ) -> Self {
        let mut lifecycle_activity_count = 0;
        let mut resource_change_count = 0;
        let mut layout_change_count = 0;
        let mut settings_change_count = 0;
        let mut resource_creation_count = 0;
        let mut resource_deletion_count = 0;

        for observation in observations {
            match observation.category {
                ObservationCategory::Lifecycle => lifecycle_activity_count += 1,
                ObservationCategory::ResourceChange => {
                    resource_change_count += 1;
                    if observation.source_event_type.ends_with(".created") {
                        resource_creation_count += 1;
                    } else if observation.source_event_type.ends_with(".deleted") {
                        resource_deletion_count += 1;
                    }
                }
                ObservationCategory::LayoutChange => layout_change_count += 1,
                ObservationCategory::SettingsChange => settings_change_count += 1,
            }
        }

        // Deterministic, fixed category ordering.
        let activity_by_category = vec![
            CategoryActivity {
                category: ObservationCategory::Lifecycle,
                count: lifecycle_activity_count,
            },
            CategoryActivity {
                category: ObservationCategory::ResourceChange,
                count: resource_change_count,
            },
            CategoryActivity {
                category: ObservationCategory::LayoutChange,
                count: layout_change_count,
            },
            CategoryActivity {
                category: ObservationCategory::SettingsChange,
                count: settings_change_count,
            },
        ];

        Self {
            generated_at: generated_at.into(),
            observation_count: observations.len(),
            lifecycle_activity_count,
            resource_change_count,
            layout_change_count,
            settings_change_count,
            resource_creation_count,
            resource_deletion_count,
            activity_by_category,
        }
    }

    pub fn validate(&self) -> Result<(), AnalyticsError> {
        if self.generated_at.trim().is_empty() {
            return Err(AnalyticsError::EmptyTimestamp);
        }

        let sum = self.lifecycle_activity_count
            + self.resource_change_count
            + self.layout_change_count
            + self.settings_change_count;
        if sum != self.observation_count {
            return Err(AnalyticsError::InconsistentCounts {
                observation_count: self.observation_count,
                sum,
            });
        }

        let resource_events = self.resource_creation_count + self.resource_deletion_count;
        if resource_events > self.resource_change_count {
            return Err(AnalyticsError::InconsistentResourceCounts {
                resource_change_count: self.resource_change_count,
                resource_events,
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actor::ActorType;
    use crate::observation::{classify_event, neutral_summary, Observation};

    fn observation(source: &str) -> Observation {
        let (category, importance) = classify_event(source).unwrap();
        Observation {
            id: format!("obs-{source}"),
            occurred_at: "2026-07-23T00:00:00Z".into(),
            category,
            importance,
            source_event_type: source.into(),
            summary: neutral_summary(category, source),
            actor_type: ActorType::LocalUser,
            actor_id: Some("local-user".into()),
            resource_ref: None,
            metadata: None,
        }
    }

    #[test]
    fn aggregates_counts_by_category() {
        let observations = vec![
            observation("system.workspace.started"),
            observation("workspace.entity.created"),
            observation("zone.created"),
            observation("resource.deleted"),
            observation("layout.updated"),
            observation("system.settings.changed"),
        ];

        let metrics = WorkspaceMetrics::from_observations(&observations, "2026-07-23T00:00:00Z");

        assert_eq!(metrics.observation_count, 6);
        assert_eq!(metrics.lifecycle_activity_count, 1);
        assert_eq!(metrics.resource_change_count, 3);
        assert_eq!(metrics.layout_change_count, 1);
        assert_eq!(metrics.settings_change_count, 1);
        assert_eq!(metrics.resource_creation_count, 2);
        assert_eq!(metrics.resource_deletion_count, 1);
        assert!(metrics.validate().is_ok());
    }

    #[test]
    fn empty_observations_produce_zero_metrics() {
        let metrics = WorkspaceMetrics::from_observations(&[], "2026-07-23T00:00:00Z");
        assert_eq!(metrics.observation_count, 0);
        assert_eq!(metrics.activity_by_category.len(), 4);
        assert!(metrics.activity_by_category.iter().all(|entry| entry.count == 0));
        assert!(metrics.validate().is_ok());
    }

    #[test]
    fn activity_by_category_is_deterministically_ordered() {
        let metrics = WorkspaceMetrics::from_observations(&[], "2026-07-23T00:00:00Z");
        let categories: Vec<ObservationCategory> = metrics
            .activity_by_category
            .iter()
            .map(|entry| entry.category)
            .collect();
        assert_eq!(
            categories,
            vec![
                ObservationCategory::Lifecycle,
                ObservationCategory::ResourceChange,
                ObservationCategory::LayoutChange,
                ObservationCategory::SettingsChange,
            ]
        );
    }

    #[test]
    fn validate_rejects_empty_timestamp() {
        let mut metrics = WorkspaceMetrics::from_observations(&[], "2026-07-23T00:00:00Z");
        metrics.generated_at = String::new();
        assert_eq!(metrics.validate(), Err(AnalyticsError::EmptyTimestamp));
    }

    #[test]
    fn validate_rejects_inconsistent_counts() {
        let mut metrics = WorkspaceMetrics::from_observations(
            &[observation("zone.created")],
            "2026-07-23T00:00:00Z",
        );
        metrics.observation_count = 5;
        assert!(matches!(
            metrics.validate(),
            Err(AnalyticsError::InconsistentCounts { .. })
        ));
    }
}
