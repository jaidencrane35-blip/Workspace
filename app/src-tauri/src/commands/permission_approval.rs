use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{
    AiAssistantWorkflow, AiOrchestratedPlan, AiPlanEvaluationReport, AiPlanSubmissionResult,
    AiProposalEvaluation, ApplicationLaunchResult, ApprovalDecisionResult,
    PermissionApprovalRequest,
};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

const DIAGNOSTIC_AI_ACTOR_ID: &str = "diagnostic-ai";

#[tauri::command]
pub fn get_permission_approvals(
    limit: Option<usize>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Vec<PermissionApprovalRequest>> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_permission_approvals(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            limit,
        ) {
            Ok(items) => IpcResponse::success(items),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn decide_approval(
    request_id: String,
    decision: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<ApprovalDecisionResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::decide_approval(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            request_id,
            decision,
        ) {
            Ok(result) => IpcResponse::success(result),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

/// Diagnostic AI simulation: AI actor proposes launch through the governed path.
///
/// Expected default outcome: ApprovalRequired (AI has zero capabilities).
/// Not a chatbot — architecture validation only.
#[tauri::command]
pub fn request_ai_application_launch(
    id: String,
    reason: Option<String>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<ApplicationLaunchResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::submit_ai_application_launch(
            &kernel,
            DIAGNOSTIC_AI_ACTOR_ID,
            id,
            reason.or_else(|| {
                Some("Diagnostic AI simulation — propose launch for approval".into())
            }),
        ) {
            Ok(result) => IpcResponse::success(result),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

/// Diagnostic AI planning: goal → proposals → governed submissions (no auto-retry).
///
/// Default: each proposal hits ApprovalRequired. Not a chatbot or autonomous agent.
#[tauri::command]
pub fn diagnose_ai_workspace_plan(
    goal: Option<String>,
    application_ids: Option<Vec<String>>,
    workspace_id: Option<String>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AiPlanSubmissionResult> {
    let goal = goal.unwrap_or_else(|| "Prepare my workspace".into());
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::submit_ai_plan(
            &kernel,
            DIAGNOSTIC_AI_ACTOR_ID,
            goal,
            application_ids.unwrap_or_default(),
            workspace_id,
        ) {
            Ok(result) => IpcResponse::success(result),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

/// Diagnostic AI evaluation: plan → quality/outcome measurement (no execution, no authority).
#[tauri::command]
pub fn diagnose_ai_plan_evaluation(
    goal: Option<String>,
    application_ids: Option<Vec<String>>,
    workspace_id: Option<String>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AiPlanEvaluationReport> {
    let goal = goal.unwrap_or_else(|| "Prepare my workspace".into());
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::diagnose_ai_plan_evaluation(
            &kernel,
            DIAGNOSTIC_AI_ACTOR_ID,
            goal,
            application_ids.unwrap_or_default(),
            workspace_id,
        ) {
            Ok(report) => IpcResponse::success(report),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

/// Diagnostic: create a multi-step AI plan (no execution).
#[tauri::command]
pub fn create_orchestrated_ai_plan(
    goal: Option<String>,
    application_ids: Option<Vec<String>>,
    workspace_id: Option<String>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AiOrchestratedPlan> {
    let goal = goal.unwrap_or_else(|| "Prepare my coding workspace".into());
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::create_orchestrated_ai_plan(
            &kernel,
            DIAGNOSTIC_AI_ACTOR_ID,
            goal,
            application_ids.unwrap_or_default(),
            workspace_id,
        ) {
            Ok(plan) => IpcResponse::success(plan),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

/// Diagnostic: preview / load an orchestrated plan by id.
#[tauri::command]
pub fn get_orchestrated_ai_plan(
    plan_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AiOrchestratedPlan> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_orchestrated_ai_plan(&kernel, plan_id) {
            Ok(plan) => IpcResponse::success(plan),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

/// Diagnostic: advance runnable steps through Permission Gateway (pauses on approval).
#[tauri::command]
pub fn advance_orchestrated_ai_plan(
    plan_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AiOrchestratedPlan> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::advance_orchestrated_ai_plan(&kernel, plan_id) {
            Ok(plan) => IpcResponse::success(plan),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

/// Diagnostic: resume after human DecideApproval (allow-once or deny).
#[tauri::command]
pub fn resume_orchestrated_ai_plan(
    plan_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AiOrchestratedPlan> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::resume_orchestrated_ai_plan(&kernel, plan_id) {
            Ok(plan) => IpcResponse::success(plan),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

/// Diagnostic: cancel an orchestrated plan safely.
#[tauri::command]
pub fn cancel_orchestrated_ai_plan(
    plan_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AiOrchestratedPlan> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::cancel_orchestrated_ai_plan(&kernel, plan_id) {
            Ok(plan) => IpcResponse::success(plan),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

/// Diagnostic: submit a natural-language assistant goal → governed plan preview.
#[tauri::command]
pub fn submit_assistant_goal(
    goal: Option<String>,
    application_ids: Option<Vec<String>>,
    workspace_id: Option<String>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AiAssistantWorkflow> {
    let goal = goal.unwrap_or_else(|| "Prepare my coding workspace".into());
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::submit_assistant_goal(
            &kernel,
            DIAGNOSTIC_AI_ACTOR_ID,
            goal,
            application_ids.unwrap_or_default(),
            workspace_id,
        ) {
            Ok(workflow) => IpcResponse::success(workflow),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn get_assistant_workflow(
    workflow_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AiAssistantWorkflow> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_assistant_workflow(&kernel, workflow_id) {
            Ok(workflow) => IpcResponse::success(workflow),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

/// Diagnostic: user confirms assistant plan → Permission Gateway path.
#[tauri::command]
pub fn confirm_assistant_workflow(
    workflow_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AiAssistantWorkflow> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::confirm_assistant_workflow(&kernel, workflow_id) {
            Ok(workflow) => IpcResponse::success(workflow),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn resume_assistant_workflow(
    workflow_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AiAssistantWorkflow> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::resume_assistant_workflow(&kernel, workflow_id) {
            Ok(workflow) => IpcResponse::success(workflow),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn cancel_assistant_workflow(
    workflow_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AiAssistantWorkflow> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::cancel_assistant_workflow(&kernel, workflow_id) {
            Ok(workflow) => IpcResponse::success(workflow),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

/// Diagnostic: derived AI proposal evaluation history from operational audits.
#[tauri::command]
pub fn get_ai_evaluation_history(
    limit: Option<usize>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Vec<AiProposalEvaluation>> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_ai_evaluation_history(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            limit,
        ) {
            Ok(items) => IpcResponse::success(items),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}
