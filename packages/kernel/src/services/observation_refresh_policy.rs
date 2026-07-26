//! Observation refresh decision policy (Sprint 109).
//!
//! Read-only: answers whether a new observation should be requested.
//! Never captures, schedules, or mutates desktop state.

use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{
    decide_observation_refresh, ActorContext, IntentContext, ObservationFreshnessRequirement,
    ObservationRefreshContext, ObservationRefreshDecision, WorkspaceObservationStatus,
};

use crate::error::Result;
use crate::services::{AuditService, CaptureCoordinator, WorkspaceObservationService};

/// Read-only policy service for observation refresh decisions.
pub(crate) struct ObservationRefreshPolicyService;

impl ObservationRefreshPolicyService {
    /// Evaluate whether a refresh should be requested for the current observation.
    ///
    /// Loads status, applies the freshness requirement, and audits the decision.
    /// Does **not** invoke [`CaptureCoordinator`] capture paths.
    pub(crate) fn evaluate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
        requirement: ObservationFreshnessRequirement,
        context: ObservationRefreshContext,
    ) -> Result<ObservationRefreshDecision> {
        let status = WorkspaceObservationService::get_status(db, actor, intent)?;
        let capture_in_progress = CaptureCoordinator::is_capture_in_progress();
        let decision = Self::decide(&status, &requirement, capture_in_progress);
        Self::audit_decision(db, actor, intent, &requirement, &context, &status, &decision)?;
        Ok(decision)
    }

    /// Pure decision helper (no I/O) for tests and callers with a status in hand.
    pub(crate) fn decide(
        status: &WorkspaceObservationStatus,
        requirement: &ObservationFreshnessRequirement,
        capture_in_progress: bool,
    ) -> ObservationRefreshDecision {
        decide_observation_refresh(status, requirement, capture_in_progress)
    }

    fn audit_decision(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
        requirement: &ObservationFreshnessRequirement,
        context: &ObservationRefreshContext,
        status: &WorkspaceObservationStatus,
        decision: &ObservationRefreshDecision,
    ) -> Result<()> {
        let blocked_reason = match decision {
            ObservationRefreshDecision::RefreshBlocked { reason } => Some(reason.as_str()),
            _ => None,
        };
        AuditService::record_ai_planning_event(
            db,
            actor,
            intent,
            "workspace.observation.refresh_evaluated",
            true,
            json!({
                "decision": decision.as_str(),
                "blocked_reason": blocked_reason,
                "requirement": requirement.as_str(),
                "requirement_max_age_seconds": match requirement {
                    ObservationFreshnessRequirement::MaxAgeSeconds { max_age_seconds } => {
                        Some(*max_age_seconds)
                    }
                    _ => None,
                },
                "consumer": context.consumer,
                "purpose": context.purpose,
                "freshness": status.freshness.as_str(),
                "age_seconds": status.age_seconds,
                "has_observation": status.has_observation,
                "capture_in_progress": CaptureCoordinator::is_capture_in_progress(),
                "authority_effect": "none",
            })
            .to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_domain::{
        ObservationFreshness, ObservationRefreshBlockedReason, WorkspaceObservationStatus,
    };
    use workspace_windows_integration::StubDesktopCapturer;

    use crate::services::capture_coordinator::observation_flight_test_lock;
    use crate::services::WorkspaceObservationService;
    use crate::WorkspaceKernel;

    #[test]
    fn evaluate_unavailable_without_observation() {
        let _lock = observation_flight_test_lock().lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let local = ActorContext::local_user();
        let intent = IntentContext::user_request();

        let decision = ObservationRefreshPolicyService::evaluate(
            &kernel.shared_database(),
            &local,
            &intent,
            ObservationFreshnessRequirement::AnyAvailable,
            ObservationRefreshContext::new().with_consumer("test"),
        )
        .unwrap();

        assert_eq!(
            decision,
            ObservationRefreshDecision::ObservationUnavailable
        );
    }

    #[test]
    fn evaluate_fresh_enough_after_capture() {
        let _lock = observation_flight_test_lock().lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let local = ActorContext::local_user();
        let intent = IntentContext::user_request();
        // Seed via observation service so policy tests do not contend on the
        // process-wide CaptureCoordinator flight lock.
        WorkspaceObservationService::capture_with(
            &kernel.shared_database(),
            &local,
            &intent,
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();

        let decision = ObservationRefreshPolicyService::evaluate(
            &kernel.shared_database(),
            &local,
            &intent,
            ObservationFreshnessRequirement::Fresh,
            ObservationRefreshContext::new()
                .with_consumer("environment")
                .with_purpose("generate"),
        )
        .unwrap();

        assert_eq!(decision, ObservationRefreshDecision::FreshEnough);
    }

    #[test]
    fn decide_blocked_when_capture_in_progress() {
        let status = WorkspaceObservationStatus {
            has_observation: true,
            freshness: ObservationFreshness::Fresh,
            pass_id: Some("p".into()),
            captured_at: Some("2026-07-26T12:00:00Z".into()),
            age_seconds: Some(5),
            window_count: Some(1),
            monitor_count: Some(1),
            identity_count: Some(0),
            source: Some("stub".into()),
            last_failure: None,
            authority_effect: "none".into(),
        };
        assert_eq!(
            ObservationRefreshPolicyService::decide(
                &status,
                &ObservationFreshnessRequirement::Fresh,
                true
            ),
            ObservationRefreshDecision::RefreshBlocked {
                reason: ObservationRefreshBlockedReason::CaptureInProgress,
            }
        );
    }
}
