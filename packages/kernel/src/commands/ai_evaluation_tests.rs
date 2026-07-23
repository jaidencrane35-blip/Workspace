//! AI proposal evaluation tests (Sprints 54–55).
//!
//! Evaluation measures quality/outcomes only — never grants authority.

use crate::commands::application::CreateApplication;
use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::services::{AiEvaluationService, AuditService};
use crate::WorkspaceKernel;
use workspace_domain::{
    evaluate_plan, Actor, ActorContext, AiActionProposal, AiApplicationAwareness, AiGoal, AiPlan,
    AiPlanEvaluationReport, AiPlanSubmissionResult, AiProposalAuthorityOutcome,
    AiProposalOutcomeClass, AiProposalQualityIssue, AiProposalSubmission, AiWorkspaceAwareness,
    ApplicationId, CapabilitySet, IntentContext,
};

fn seed_two_apps(kernel: &WorkspaceKernel) -> (String, String, String) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Eval WS".into()))
        .unwrap();

    let app_a = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateApplication::new(
            workspace.id.clone(),
            "Notepad".into(),
            None,
            Some("notepad.exe".into()),
        ))
        .unwrap();
    let app_b = CommandPipeline::new(kernel.command_context(local, intent))
        .execute_mutation(CreateApplication::new(
            workspace.id.clone(),
            "VS Code".into(),
            None,
            Some("code.exe".into()),
        ))
        .unwrap();

    (
        workspace.id.to_string(),
        app_a.id.to_string(),
        app_b.id.to_string(),
    )
}

#[test]
fn case1_proposal_can_be_evaluated_without_execution() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (_ws, app_a, app_b) = seed_two_apps(&kernel);

    let report = CommandHandler::diagnose_ai_plan_evaluation(
        &kernel,
        "ai-eval-1",
        "Prepare my workspace",
        vec![app_a, app_b],
        None,
    )
    .unwrap();

    assert_eq!(report.summary.proposal_count, 2);
    assert_eq!(report.summary.valid_count, 2);
    assert!(report
        .evaluations
        .iter()
        .all(|e| e.outcome == AiProposalOutcomeClass::Created));

    let records = AuditService::list_recent(&kernel.shared_database(), 80).unwrap();
    assert!(records
        .iter()
        .any(|r| r.event_type == "ai.planning.proposal_evaluated"));
    assert!(!records.iter().any(|r| {
        r.command_name.as_deref() == Some("LaunchApplication")
            && r.event_type.starts_with("command.")
            && r.success
            && r.actor_type == workspace_domain::ActorType::AIAssistant
    }));
}

#[test]
fn case2_governance_path_remains_authority() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (_ws, app_a, _) = seed_two_apps(&kernel);

    let result = CommandHandler::submit_ai_plan_simulated(
        &kernel,
        "ai-eval-2",
        "Prepare my workspace",
        vec![app_a],
    )
    .unwrap();

    assert!(matches!(
        result.submissions[0].outcome,
        AiProposalAuthorityOutcome::ApprovalRequired { .. }
    ));

    let records = AuditService::list_recent(&kernel.shared_database(), 80).unwrap();
    assert!(records
        .iter()
        .any(|r| r.event_type == "permission.approval_required"));
    assert!(records
        .iter()
        .any(|r| r.event_type == "ai.planning.proposal_evaluated"));
    assert!(records
        .iter()
        .any(|r| r.event_type == "ai.planning.outcome_recorded"));
}

#[test]
fn case3_poor_proposal_identified_without_correction() {
    let goal = AiGoal::new("Prepare my workspace", "ai-eval-3").unwrap();
    let app = ApplicationId::new("app-1").unwrap();
    let first = AiActionProposal::propose_application_launch(
        &goal,
        &app,
        Some("Launch VS Code".into()),
    )
    .unwrap();
    let duplicate = AiActionProposal::propose_application_launch(
        &goal,
        &app,
        Some("Launch VS Code again".into()),
    )
    .unwrap();
    let plan = AiPlan::new(goal, vec![first, duplicate]);
    let awareness = AiWorkspaceAwareness {
        workspace_id: "ws-1".into(),
        workspace_name: "Dev".into(),
        zone_count: 0,
        layout_id: None,
        applications: vec![AiApplicationAwareness {
            id: app,
            name: "VS Code".into(),
            identifier: Some("code.exe".into()),
            appears_active: true,
        }],
        recent_observation_count: 0,
        environment_window_titles: vec!["VS Code".into()],
    };

    let report = evaluate_plan(&plan, Some(&awareness));
    assert_eq!(report.summary.duplicate_count, 1);
    assert_eq!(report.summary.unnecessary_count, 2);
    assert!(report.evaluations[0].has_issue(|issue| {
        matches!(
            issue,
            AiProposalQualityIssue::Unnecessary { reason } if reason.contains("Already running")
        )
    }));
    // No automatic correction or execution — report only.
    assert_eq!(report.summary.successful_count, 0);
    assert_eq!(
        report.authority_note,
        AiPlanEvaluationReport::AUTHORITY_NOTE
    );
}

#[test]
fn case4_denied_proposal_records_outcome_and_stays_blocked() {
    let goal = AiGoal::new("Prepare my workspace", "ai-eval-4").unwrap();
    let app = ApplicationId::new("app-1").unwrap();
    let proposal =
        AiActionProposal::propose_application_launch(&goal, &app, Some("Launch".into())).unwrap();
    let request = proposal
        .to_action_request(&goal.requesting_actor_id)
        .unwrap();
    let result = AiPlanSubmissionResult {
        plan: AiPlan::new(goal, vec![proposal.clone()]),
        submissions: vec![AiProposalSubmission {
            proposal,
            request,
            outcome: AiProposalAuthorityOutcome::Denied {
                reason: "capability missing".into(),
            },
        }],
    };

    let report = AiEvaluationService::evaluate_submission_result(&result, None).unwrap();
    assert_eq!(report.summary.rejected_count, 1);
    assert_eq!(
        report.evaluations[0].outcome,
        AiProposalOutcomeClass::Denied
    );
    assert_eq!(
        report.evaluations[0].outcome_detail.as_deref(),
        Some("capability missing")
    );
}

#[test]
fn case5_successful_approved_action_outcome_recorded() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let goal = AiGoal::new("Prepare my workspace", "ai-eval-5").unwrap();
    let app = ApplicationId::new("app-1").unwrap();
    let proposal =
        AiActionProposal::propose_application_launch(&goal, &app, Some("Launch".into())).unwrap();
    let request = proposal
        .to_action_request(&goal.requesting_actor_id)
        .unwrap();
    let result = AiPlanSubmissionResult {
        plan: AiPlan::new(goal.clone(), vec![proposal.clone()]),
        submissions: vec![AiProposalSubmission {
            proposal,
            request,
            outcome: AiProposalAuthorityOutcome::Allowed,
        }],
    };

    let actor = ActorContext::new(Actor::ai_assistant("ai-eval-5").unwrap());
    let report = AiEvaluationService::evaluate_submission_result(&result, None).unwrap();
    AiEvaluationService::audit_report(&kernel.shared_database(), &actor, &report).unwrap();

    assert_eq!(report.summary.successful_count, 1);
    assert_eq!(
        report.evaluations[0].outcome,
        AiProposalOutcomeClass::Succeeded
    );

    let records = AuditService::list_recent(&kernel.shared_database(), 40).unwrap();
    assert!(records.iter().any(|r| {
        r.event_type == "ai.planning.outcome_recorded"
            && r.metadata
                .as_deref()
                .is_some_and(|m| m.contains("succeeded"))
    }));
}

#[test]
fn case6_evaluation_cannot_influence_permissions() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (_ws, app_a, _) = seed_two_apps(&kernel);

    let before = CapabilitySet::for_actor_type(workspace_domain::ActorType::AIAssistant);
    assert_eq!(before.iter().count(), 0);

    let _report = CommandHandler::diagnose_ai_plan_evaluation(
        &kernel,
        "ai-eval-6",
        "Prepare my workspace",
        vec![app_a.clone()],
        None,
    )
    .unwrap();

    let after = CapabilitySet::for_actor_type(workspace_domain::ActorType::AIAssistant);
    assert_eq!(before, after);
    assert_eq!(after.iter().count(), 0);

    // Governance path still requires approval — evaluation did not grant launch.
    let result = CommandHandler::submit_ai_plan_simulated(
        &kernel,
        "ai-eval-6",
        "Prepare my workspace",
        vec![app_a],
    )
    .unwrap();
    assert!(matches!(
        result.submissions[0].outcome,
        AiProposalAuthorityOutcome::ApprovalRequired { .. }
            | AiProposalAuthorityOutcome::Denied { .. }
    ));
}

#[test]
fn evaluation_history_lists_audited_evaluations() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (_ws, app_a, _) = seed_two_apps(&kernel);

    CommandHandler::diagnose_ai_plan_evaluation(
        &kernel,
        "ai-eval-hist",
        "Prepare my workspace",
        vec![app_a],
        None,
    )
    .unwrap();

    let history = CommandHandler::get_ai_evaluation_history(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        Some(20),
    )
    .unwrap();

    assert!(!history.is_empty());
    assert!(history.iter().all(|e| e.validate().is_ok()));
}
