//! Sprints 187–191 — observation consumer freshness wiring contract tests.

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::services::observation_flight_test_lock;
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, IntentContext, ObservationConsumerFreshnessNeed, ObservationFreshness,
    ObservationRefreshDecision, TaskPriority,
};

fn seed(kernel: &WorkspaceKernel) -> String {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Freshness Wiring WS".into()))
        .unwrap();
    let workspace_id = workspace.id.to_string();
    let project = CommandHandler::create_project(
        kernel,
        local.clone(),
        intent.clone(),
        workspace_id.clone(),
        "Freshness Platform".into(),
        None,
        None,
    )
    .unwrap();
    let task = CommandHandler::create_task(
        kernel,
        local.clone(),
        intent.clone(),
        project.id.to_string(),
        workspace_id.clone(),
        "Wire freshness".into(),
        TaskPriority::High,
    )
    .unwrap();
    CommandHandler::set_active_work(
        kernel,
        local,
        intent,
        workspace_id.clone(),
        Some(project.id.to_string()),
        Some(task.id.to_string()),
    )
    .unwrap();
    workspace_id
}

#[test]
fn case1_environment_evaluates_consumer_freshness_need() {
    let _lock = observation_flight_test_lock()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let env = CommandHandler::generate_workspace_environment(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();

    assert_eq!(
        env.observation_freshness,
        ObservationFreshness::Unavailable.as_str()
    );
    assert_eq!(
        env.observation_refresh_decision,
        ObservationRefreshDecision::ObservationUnavailable.as_str()
    );
    assert!(!env.observation_has_observation);
    assert_eq!(env.authority_effect, "none");
}

#[test]
fn case2_ensure_freshness_uses_manual_trigger_authority_not_silent() {
    let _lock = observation_flight_test_lock()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let _ws = seed(&kernel);
    let result = CommandHandler::ensure_observation_freshness(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        Some(ObservationConsumerFreshnessNeed::ENVIRONMENT_CONSUMER.into()),
    )
    .unwrap();

    assert_eq!(
        result.consumer_id,
        ObservationConsumerFreshnessNeed::ENVIRONMENT_CONSUMER
    );
    assert_eq!(result.authority_effect, "none");
    // In-memory / stub path: may capture or remain unavailable — never execute authority.
    assert!(
        result.trigger_outcome == "accepted_capture"
            || result.trigger_outcome == "unavailable"
            || result.trigger_outcome == "ignored_fresh"
            || result.trigger_outcome == "blocked_capture_in_progress"
            || result.trigger_outcome == "rate_limited"
    );
    assert!(!result.explanation.is_empty());
}

#[test]
fn case3_runtime_health_uses_live_observation_status() {
    let _lock = observation_flight_test_lock()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let view = CommandHandler::generate_workspace_runtime_overview(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();

    assert!(view.observation_status.is_some());
    let status = view.observation_status.as_ref().unwrap();
    assert_eq!(status.authority_effect, "none");
    // Without a capture, freshness is unavailable → runtime observation level unknown.
    assert_eq!(status.freshness, ObservationFreshness::Unavailable);
    assert_eq!(
        view.health.observation_freshness.as_str(),
        workspace_domain::WorkspaceHealthLevel::Unknown.as_str()
    );
    assert!(view.health.stale_observations);
    assert!(CommandHandler::workspace_runtime_attempt_execute().is_err());
}

#[test]
fn case4_canonical_consumer_needs_do_not_execute() {
    let env_need = ObservationConsumerFreshnessNeed::for_environment();
    let intel_need = ObservationConsumerFreshnessNeed::for_intelligence();
    assert_eq!(
        env_need.consumer_id,
        ObservationConsumerFreshnessNeed::ENVIRONMENT_CONSUMER
    );
    assert_eq!(
        intel_need.consumer_id,
        ObservationConsumerFreshnessNeed::INTELLIGENCE_CONSUMER
    );
    assert_eq!(env_need.requirement.as_str(), "not_stale");
    assert_eq!(intel_need.requirement.as_str(), "not_stale");
}
