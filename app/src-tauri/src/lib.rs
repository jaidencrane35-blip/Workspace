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
use commands::workspace_recommendation::{
    accept_recommendation, confirm_recommendation_decision, decline_recommendation_decision,
    generate_workspace_recommendation_engine, present_recommendation, reject_recommendation,
    revoke_recommendation_adapter_preparation,
};
use commands::workspace_operating_state::generate_workspace_operating_state;
use commands::workspace_pattern::generate_workspace_pattern;
use commands::workspace_adaptation::{
    accept_adaptation_proposal, generate_workspace_adaptation, reject_adaptation_proposal,
    review_adaptation_proposal,
};
use commands::workspace_readiness::generate_workspace_readiness;
use commands::workspace_runtime::generate_workspace_runtime_overview;
use commands::workspace_session::{compare_workspace_sessions, generate_workspace_session};
use commands::workspace_experience::{
    compare_workspace_experiences, generate_workspace_experience,
};
use commands::workspace_work_context::{
    compare_workspace_work_contexts, generate_workspace_work_context,
    validate_workspace_work_context,
};
use commands::workspace_navigation::{
    compare_workspace_navigation, generate_workspace_navigation, validate_workspace_navigation,
};
use commands::workspace_milestone::{
    compare_workspace_milestones, generate_workspace_milestones, validate_workspace_milestones,
};
use commands::workspace_working_style::{
    compare_workspace_working_styles, generate_workspace_working_style,
    validate_workspace_working_style,
};
use commands::workspace_transition::{
    compare_workspace_transitions, generate_workspace_transitions, validate_workspace_transitions,
};
use commands::workspace_interaction::{
    compare_workspace_interactions, generate_workspace_interactions,
    select_workspace_interaction, validate_workspace_interactions,
};
use commands::workspace_profile::{
    compare_workspace_profile, compare_workspace_profile_states, create_workspace_profile,
    generate_workspace_profile_state, get_workspace_profile, list_workspace_profiles,
    update_workspace_profile, validate_workspace_profile_state,
};
use commands::workspace_attention::generate_workspace_attention;
use commands::layout::{
    create_layout, delete_layout, get_layout, get_layout_snapshot, reset_layout, update_layout,
};
use commands::analytics::get_workspace_metrics;
use commands::context::get_workspace_context;
use commands::workspace_observation::{
    capture_workspace_observation, ensure_observation_freshness, get_latest_observation_delta,
    get_latest_workspace_observation, get_observation_scheduler_status,
    get_workspace_observation_by_id, get_workspace_observation_status,
};
use commands::workspace_state::{get_workspace_runtime_state, get_workspace_state};
use commands::saved_context::{get_saved_context_capture_scope, save_workspace_context};
use commands::pilot_measurement::{
    get_pilot_measurement_scope, get_pilot_measurement_snapshot, grant_pilot_consent,
    record_pilot_baseline, record_pilot_interview, record_pilot_leave_resume,
    withdraw_pilot_consent,
};
use commands::resume::{
    delete_saved_context, execute_resume_plan, get_saved_context, list_saved_contexts,
    resolve_resume_plan,
};
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
use commands::shell::exit_workspace;
use commands::status::get_workspace_status;
use tauri::Manager;
use workspace_kernel::WorkspaceKernel;

fn init_logging() {
    // Product builds stay quiet (no console under windows_subsystem).
    // Engineers opt into verbose logs with RUST_LOG / WORKSPACE_DEV_LOG=1.
    let default_filter = if cfg!(debug_assertions) {
        if std::env::var_os("WORKSPACE_DEV_LOG").is_some() {
            "info"
        } else {
            "warn"
        }
    } else {
        "error"
    };
    let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(default_filter))
        .format_timestamp_secs()
        .try_init();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_logging();
    log::info!("workspace application starting");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // Runtime / settings / shell
            get_workspace_status,
            get_workspace_health,
            get_settings,
            update_settings,
            exit_workspace,
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
            capture_workspace_observation,
            get_latest_workspace_observation,
            get_workspace_observation_by_id,
            get_workspace_observation_status,
            get_observation_scheduler_status,
            get_latest_observation_delta,
            ensure_observation_freshness,
            get_saved_context_capture_scope,
            save_workspace_context,
            list_saved_contexts,
            get_saved_context,
            delete_saved_context,
            resolve_resume_plan,
            execute_resume_plan,
            get_pilot_measurement_scope,
            get_pilot_measurement_snapshot,
            grant_pilot_consent,
            withdraw_pilot_consent,
            record_pilot_baseline,
            record_pilot_leave_resume,
            record_pilot_interview,
            get_workspace_state,
            get_workspace_runtime_state,
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
            present_recommendation,
            accept_recommendation,
            reject_recommendation,
            confirm_recommendation_decision,
            decline_recommendation_decision,
            revoke_recommendation_adapter_preparation,
            generate_workspace_operating_state,
            generate_workspace_pattern,
            generate_workspace_adaptation,
            review_adaptation_proposal,
            accept_adaptation_proposal,
            reject_adaptation_proposal,
            generate_workspace_readiness,
            generate_workspace_runtime_overview,
            generate_workspace_session,
            compare_workspace_sessions,
            generate_workspace_experience,
            compare_workspace_experiences,
            generate_workspace_work_context,
            compare_workspace_work_contexts,
            validate_workspace_work_context,
            generate_workspace_navigation,
            compare_workspace_navigation,
            validate_workspace_navigation,
            generate_workspace_milestones,
            compare_workspace_milestones,
            validate_workspace_milestones,
            generate_workspace_working_style,
            compare_workspace_working_styles,
            validate_workspace_working_style,
            generate_workspace_transitions,
            compare_workspace_transitions,
            validate_workspace_transitions,
            generate_workspace_interactions,
            compare_workspace_interactions,
            validate_workspace_interactions,
            select_workspace_interaction,
            create_workspace_profile,
            update_workspace_profile,
            list_workspace_profiles,
            get_workspace_profile,
            generate_workspace_profile_state,
            compare_workspace_profile,
            compare_workspace_profile_states,
            validate_workspace_profile_state,
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
            // Quarantine / CommandHandler parity — unused by React today.
            // Canonical quarantine + constitution tiers: docs/03-Engineering/ipc-tiers.json
            // (get_action_catalog is Developer-tier; OperatorConsole calls it — not quarantine.)
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
            std::fs::create_dir_all(&app_data_dir).unwrap_or_else(|error| {
                panic!(
                    "failed to create app data directory {}: {error}",
                    app_data_dir.display()
                );
            });
            let db_path = app_data_dir.join("workspace.db");
            log::info!(
                "initializing workspace kernel with database {}",
                db_path.display()
            );

            let kernel = WorkspaceKernel::initialize(db_path).unwrap_or_else(|error| {
                log::error!("workspace kernel initialization failed: {error}");
                panic!("failed to initialize workspace kernel: {error}");
            });

            app.manage(Arc::new(Mutex::new(kernel)));
            log::info!("workspace application setup complete");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
