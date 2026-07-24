mod actor;
mod commands;

use std::sync::{Arc, Mutex};

use commands::audit::get_audit_history;
use commands::automation_contract::{
    approve_automation_contract, create_automation_contract, get_automation_contract,
    list_automation_contracts, pause_automation_contract, prepare_automation_contract_intent,
    request_automation_contract_approval, resume_automation_contract, revoke_automation_contract,
    update_automation_contract,
};
use commands::automation_trigger::{
    accept_automation_intent_proposal, evaluate_triggers, list_automation_intent_proposals,
    list_trigger_events, record_and_evaluate_triggers, record_trigger_event,
    reject_automation_intent_proposal,
};
use commands::decision_engine::{
    dismiss_decision_candidate, generate_decision_engine, postpone_decision_candidate,
    select_decision_candidate,
};
use commands::task_graph::{
    add_task_relationship, create_workspace_task, generate_task_graph,
    get_task_graph_planning_inputs, update_workspace_task_status, validate_task_graph,
};
use commands::decision_queue::{
    accept_decision_item, defer_decision_item, dismiss_decision_item, generate_decision_queue,
    mark_decision_item_viewed, reject_decision_item,
};
use commands::workspace_activity::{
    generate_workspace_activity_graph, get_workspace_activity_timeline,
};
use commands::workspace_continuity::generate_workspace_continuity;
use commands::workspace_environment::generate_workspace_environment;
use commands::workspace_composition::generate_workspace_composition;
use commands::workspace_purpose::generate_workspace_purpose;
use commands::workspace_evolution::generate_workspace_evolution;
use commands::workspace_recommendation::generate_workspace_recommendation_engine;
use commands::workspace_operating_state::generate_workspace_operating_state;
use commands::workspace_pattern::generate_workspace_pattern;
use commands::workspace_attention::generate_workspace_attention;
use commands::layout::{
    create_layout, delete_layout, get_layout, get_layout_snapshot, reset_layout, update_layout,
};
use commands::analytics::get_workspace_metrics;
use commands::context::get_workspace_context;
use commands::desktop_window::get_desktop_windows;
use commands::discovery::{get_action_catalog, get_actor_capabilities};
use commands::memory::{
    clear_memory_entries, create_memory_entry, delete_memory_entry, diagnose_ai_plan_preview,
    get_memory_context, list_memory_entries,
};
use commands::model_provider::{
    diagnose_model_proposal_generation, get_model_provider_metadata, list_model_providers,
    test_model_provider_request,
};
use commands::personalization::{
    compare_personalized_vs_neutral_plan, create_user_preference, delete_user_preference,
    diagnose_ai_plan_with_personalization, get_preference_profile, set_personalization_enabled,
    update_user_preference,
};
use commands::permission_approval::{
    advance_orchestrated_ai_plan, cancel_assistant_workflow, cancel_orchestrated_ai_plan,
    compare_assistant_plan_revisions, confirm_assistant_workflow, create_orchestrated_ai_plan,
    decide_approval, diagnose_ai_plan_evaluation, diagnose_ai_workspace_plan,
    get_ai_evaluation_history, get_assistant_workflow, get_orchestrated_ai_plan,
    get_permission_approvals, record_assistant_explanation_viewed, regenerate_assistant_plan,
    request_ai_application_launch, resume_assistant_workflow, resume_orchestrated_ai_plan,
    revise_assistant_goal, submit_assistant_goal,
};
use commands::observation::get_observations;
use commands::projection::get_workspace_snapshot;
use commands::execution::{
    create_suggestion_intent_request, execute_intent_request, get_execution_outcomes,
    get_execution_state, get_execution_states, request_execution_cancellation,
};
use commands::suggestion::{accept_suggestion, get_suggestions, reject_suggestion};
use commands::suggestion_lifecycle::get_suggestion_lifecycle;
use commands::resources::{
    create_application, create_widget, create_zone, delete_application, delete_widget,
    delete_zone, get_application, get_widget, get_zone, launch_application,
};
use commands::workspace::{create_workspace, get_workspace};
use commands::workspace_intelligence::{
    compare_workspace_intelligence_states, create_project, create_task,
    generate_workspace_intelligence, get_project, get_task, get_workflow_context, list_projects,
    list_tasks, set_active_work, update_task_status,
};
use commands::health::get_workspace_health;
use commands::settings::{get_settings, update_settings};
use commands::status::get_workspace_status;
use tauri::Manager;
use workspace_kernel::WorkspaceKernel;

fn init_logging() {
    let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_secs()
        .try_init();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_logging();
    log::info!("workspace application starting");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // Runtime / settings
            get_workspace_status,
            get_workspace_health,
            get_settings,
            update_settings,
            // Product shell (Canvas + Diagnostic)
            create_workspace,
            get_workspace,
            create_zone,
            create_layout,
            update_layout,
            get_layout,
            get_workspace_context,
            get_suggestions,
            accept_suggestion,
            reject_suggestion,
            get_suggestion_lifecycle,
            get_desktop_windows,
            create_application,
            launch_application,
            get_permission_approvals,
            decide_approval,
            request_ai_application_launch,
            diagnose_ai_workspace_plan,
            diagnose_ai_plan_preview,
            diagnose_ai_plan_evaluation,
            get_ai_evaluation_history,
            create_memory_entry,
            list_memory_entries,
            get_memory_context,
            delete_memory_entry,
            clear_memory_entries,
            list_model_providers,
            get_model_provider_metadata,
            test_model_provider_request,
            diagnose_model_proposal_generation,
            create_user_preference,
            update_user_preference,
            get_preference_profile,
            delete_user_preference,
            set_personalization_enabled,
            diagnose_ai_plan_with_personalization,
            compare_personalized_vs_neutral_plan,
            create_orchestrated_ai_plan,
            get_orchestrated_ai_plan,
            advance_orchestrated_ai_plan,
            resume_orchestrated_ai_plan,
            cancel_orchestrated_ai_plan,
            submit_assistant_goal,
            get_assistant_workflow,
            confirm_assistant_workflow,
            resume_assistant_workflow,
            cancel_assistant_workflow,
            revise_assistant_goal,
            regenerate_assistant_plan,
            compare_assistant_plan_revisions,
            record_assistant_explanation_viewed,
            create_project,
            list_projects,
            get_project,
            create_task,
            list_tasks,
            get_task,
            update_task_status,
            set_active_work,
            get_workflow_context,
            generate_workspace_intelligence,
            compare_workspace_intelligence_states,
            create_automation_contract,
            list_automation_contracts,
            get_automation_contract,
            update_automation_contract,
            request_automation_contract_approval,
            approve_automation_contract,
            pause_automation_contract,
            resume_automation_contract,
            revoke_automation_contract,
            prepare_automation_contract_intent,
            record_trigger_event,
            evaluate_triggers,
            record_and_evaluate_triggers,
            list_trigger_events,
            list_automation_intent_proposals,
            accept_automation_intent_proposal,
            reject_automation_intent_proposal,
            generate_decision_queue,
            mark_decision_item_viewed,
            defer_decision_item,
            dismiss_decision_item,
            accept_decision_item,
            reject_decision_item,
            generate_workspace_activity_graph,
            get_workspace_activity_timeline,
            generate_workspace_continuity,
            generate_workspace_attention,
            generate_workspace_environment,
            generate_workspace_composition,
            generate_workspace_purpose,
            generate_workspace_evolution,
            generate_workspace_recommendation_engine,
            generate_workspace_operating_state,
            generate_workspace_pattern,
            generate_decision_engine,
            select_decision_candidate,
            dismiss_decision_candidate,
            postpone_decision_candidate,
            generate_task_graph,
            create_workspace_task,
            update_workspace_task_status,
            add_task_relationship,
            validate_task_graph,
            get_task_graph_planning_inputs,
            create_suggestion_intent_request,
            execute_intent_request,
            get_execution_outcomes,
            get_execution_states,
            request_execution_cancellation,
            // Registered for CommandHandler parity / future UI — unused by React today
            // (see docs/03-Engineering/IPC-SURFACE.md)
            delete_zone,
            get_zone,
            delete_application,
            get_application,
            create_widget,
            delete_widget,
            get_widget,
            delete_layout,
            reset_layout,
            get_layout_snapshot,
            get_workspace_snapshot,
            get_actor_capabilities,
            get_action_catalog,
            get_audit_history,
            get_observations,
            get_workspace_metrics,
            get_execution_state,
        ])
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to resolve app data directory");
            let db_path = app_data_dir.join("workspace.db");

            let kernel = WorkspaceKernel::initialize(db_path).unwrap_or_else(|error| {
                log::error!("workspace kernel initialization failed: {error}");
                panic!("failed to initialize workspace kernel");
            });

            app.manage(Arc::new(Mutex::new(kernel)));
            log::info!("workspace application setup complete");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
