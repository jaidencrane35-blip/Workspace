//! Workspace Environment Model (Phase 5 / Sprints 119–121).
//!
//! Aggregates WorkspaceState with apps, workflow, task graph, and layout.
//! Primary runtime input is WorkspaceState (not raw observation or DesktopWindowSnapshot).
//! Never enumerates Win32. Never executes or moves windows.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::{ApplicationRepository, Database};
use workspace_domain::{
    build_environment_summary, now_rfc3339, validate_workspace_id, ActorContext,
    ApplicationReference, EnvironmentApplication, EnvironmentGap, EnvironmentLayoutAssociation,
    EnvironmentWindow, EnvironmentWindowGroup, EnvironmentWindowState, IntentContext, Layout,
    ObservationConsumerFreshnessNeed, ObservationRefreshContext, TaskGraph, WorkflowContext,
    WorkspaceEnvironmentState, WorkspaceEnvironmentSummary, WorkspaceId, WorkspaceState,
    WorkspaceStateWindow,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AuditService, CaptureCoordinator, LayoutService, ObservationRefreshPolicyService,
    TaskGraphService, WorkspaceIntentService, WorkspaceObservationService, WorkspaceStateEngine,
};

pub(crate) struct WorkspaceEnvironmentService;

impl WorkspaceEnvironmentService {
    /// Standalone generate for a workspace — loads WorkspaceState via the state engine.
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
    ) -> Result<WorkspaceEnvironmentState> {
        let workspace_id = workspace_id.into();
        let workspace_state = WorkspaceStateEngine::get_current(
            db,
            actor,
            &IntentContext::user_request(),
        )?;
        let workflow =
            WorkspaceIntentService::get_workflow_context_readonly(db, &workspace_id)?;
        let task_graph = TaskGraphService::generate(db, actor, workspace_id.clone()).ok();
        let layout = Self::load_layout(db, &workspace_id);
        let apps = Self::list_apps(db, &workspace_id)?;
        Self::generate_from_state(
            db,
            actor,
            &workspace_id,
            &workspace_state,
            &apps,
            &workflow,
            task_graph.as_ref(),
            layout.as_ref(),
        )
    }

    /// Preferred production path — Environment consumes WorkspaceState.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn generate_from_state(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        workspace_state: &WorkspaceState,
        applications: &[ApplicationReference],
        workflow: &WorkflowContext,
        task_graph: Option<&TaskGraph>,
        layout: Option<&Layout>,
    ) -> Result<WorkspaceEnvironmentState> {
        let workspace_id = validate_workspace_id(workspace_id).map_err(KernelError::from)?;
        let ws = workspace_id.as_str();
        let active_project = workflow
            .active_project_id
            .as_ref()
            .map(|id| id.to_string());
        let active_task = workflow.active_task_id.as_ref().map(|id| id.to_string());
        let windows = &workspace_state.windows;

        let titles_lower: Vec<_> = windows.iter().map(|w| w.title.to_lowercase()).collect();

        let mut env_windows = Vec::new();
        for snap in windows.iter() {
            let matched = match_application(snap, applications);
            let state = window_state_from_state_window(snap);
            let (project_id, task_id) = if matched.is_some() {
                (active_project.clone(), active_task.clone())
            } else {
                (None, None)
            };
            let layout_id = layout.map(|l| l.id.to_string());
            let explanation = match &matched {
                Some(app) => format!(
                    "Window \"{}\" matched registered application \"{}\".",
                    snap.title, app.name
                ),
                None => format!(
                    "Window \"{}\" observed on desktop; no registered Workspace application match.",
                    snap.title
                ),
            };
            env_windows.push(EnvironmentWindow {
                id: format!("env_window:{}", snap.hwnd),
                hwnd: snap.hwnd.clone(),
                title: snap.title.clone(),
                process_id: snap.process_id as u32,
                state,
                matched_application_id: matched.as_ref().map(|a| a.id.to_string()),
                matched_application_name: matched.as_ref().map(|a| a.name.clone()),
                project_id,
                task_id,
                layout_id,
                display_label: display_label_for_window(snap),
                explanation,
                authority_effect: EnvironmentWindow::AUTHORITY_EFFECT_NONE.into(),
            });
        }

        // Prefer WorkspaceState focused_window when present; fall back to scan.
        let focused_window_id = workspace_state
            .focused_window
            .as_ref()
            .map(|window| format!("env_window:{}", window.hwnd))
            .or_else(|| {
                env_windows
                    .iter()
                    .find(|w| w.state == EnvironmentWindowState::Focused)
                    .map(|w| w.id.clone())
            });

        let mut env_apps = Vec::new();
        for app in applications {
            let matched_windows: Vec<_> = env_windows
                .iter()
                .filter(|w| w.matched_application_id.as_deref() == Some(app.id.as_str()))
                .collect();
            let appears_running = !matched_windows.is_empty()
                || app_matches_titles(&app.name, app.identifier.as_deref(), &titles_lower);
            let focused = matched_windows
                .iter()
                .any(|w| w.state == EnvironmentWindowState::Focused);
            let explanation = if appears_running {
                format!(
                    "Application \"{}\" appears running ({} matched window(s)).",
                    app.name,
                    matched_windows.len()
                )
            } else {
                format!(
                    "Application \"{}\" is registered for this Workspace but not observed on the desktop.",
                    app.name
                )
            };
            env_apps.push(EnvironmentApplication {
                application_id: app.id.to_string(),
                name: app.name.clone(),
                identifier: app.identifier.clone(),
                appears_running,
                window_count: matched_windows.len(),
                focused,
                project_id: if appears_running {
                    active_project.clone()
                } else {
                    None
                },
                task_id: if appears_running {
                    active_task.clone()
                } else {
                    None
                },
                explanation,
            });
        }

        let window_groups = build_groups(&env_windows);
        let layout_associations = layout
            .map(|l| {
                vec![EnvironmentLayoutAssociation {
                    layout_id: l.id.to_string(),
                    layout_name: format!("Workspace layout {}", l.id),
                    explanation:
                        "Canvas layout for this Workspace (presentation model — not OS monitor layout)."
                            .into(),
                }]
            })
            .unwrap_or_default();

        let mut gaps = Vec::new();
        for app in &env_apps {
            if !app.appears_running {
                gaps.push(EnvironmentGap {
                    kind: "missing_application".into(),
                    title: format!("Missing: {}", app.name),
                    explanation: app.explanation.clone(),
                    application_id: Some(app.application_id.clone()),
                    project_id: active_project.clone(),
                    task_id: active_task.clone(),
                });
            }
        }

        let has_matched_window = env_windows.iter().any(|w| w.matched_application_id.is_some());
        let disconnected_work = (active_project.is_some() || active_task.is_some())
            && !has_matched_window
            && !windows.is_empty();
        let disconnected_work = disconnected_work
            || ((active_project.is_some() || active_task.is_some())
                && windows.is_empty()
                && task_graph.map(|g| !g.nodes.is_empty()).unwrap_or(false));

        if disconnected_work {
            gaps.push(EnvironmentGap {
                kind: "disconnected_work".into(),
                title: "Active work appears disconnected from desktop".into(),
                explanation: "Current project/task is set, but no open windows match registered Workspace applications."
                    .into(),
                application_id: None,
                project_id: active_project.clone(),
                task_id: active_task.clone(),
            });
        }

        if let Some(graph) = task_graph {
            let open = graph.open_incomplete_nodes();
            if !open.is_empty() && !has_matched_window && !windows.is_empty() {
                gaps.push(EnvironmentGap {
                    kind: "task_graph_unsupported".into(),
                    title: "Task Graph work lacks matching windows".into(),
                    explanation: format!(
                        "{} open Task Graph item(s) have no matching desktop application windows.",
                        open.len()
                    ),
                    application_id: None,
                    project_id: active_project.clone(),
                    task_id: active_task.clone(),
                });
            }
        }

        let running_application_count = env_apps.iter().filter(|a| a.appears_running).count();
        let missing_application_count = env_apps.iter().filter(|a| !a.appears_running).count();
        let summary = build_environment_summary(
            ws,
            env_windows.len(),
            running_application_count,
            missing_application_count,
            disconnected_work,
        );

        let intent = IntentContext::user_request();
        let need = ObservationConsumerFreshnessNeed::for_environment();
        let status = WorkspaceObservationService::get_status(db, actor, &intent)?;
        let refresh_decision = ObservationRefreshPolicyService::decide(
            &status,
            &need.requirement,
            CaptureCoordinator::is_capture_in_progress(),
        );
        Self::audit_freshness_evaluated(db, actor, &intent, &need, &status, refresh_decision.as_str())?;

        let state = WorkspaceEnvironmentState {
            workspace_id: ws.to_string(),
            generated_at: now_rfc3339(),
            active_project_id: active_project,
            active_task_id: active_task,
            windows: env_windows,
            applications: env_apps,
            window_groups,
            layout_associations,
            gaps,
            focused_window_id,
            running_application_count,
            missing_application_count,
            disconnected_work,
            observation_freshness: status.freshness.as_str().into(),
            observation_refresh_decision: refresh_decision.as_str().into(),
            observation_age_seconds: status.age_seconds,
            observation_has_observation: status.has_observation,
            summary,
            authority_effect: WorkspaceEnvironmentState::AUTHORITY_EFFECT_NONE.into(),
        };

        Self::audit_generated(db, actor, &state, workspace_state)?;
        Ok(state)
    }

    pub(crate) fn summary_projection(
        state: &WorkspaceEnvironmentState,
        limit: usize,
    ) -> WorkspaceEnvironmentSummary {
        state.summary_projection(limit)
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::from(
            workspace_domain::WorkspaceEnvironmentError::CannotExecute,
        ))
    }

    fn list_apps(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
    ) -> Result<Vec<ApplicationReference>> {
        let workspace_id = WorkspaceId::new(workspace_id).map_err(KernelError::Domain)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
        ApplicationRepository::new(&guard)
            .list_by_workspace(&workspace_id)
            .map_err(Into::into)
    }

    fn load_layout(db: &Arc<Mutex<Database>>, workspace_id: &str) -> Option<Layout> {
        let workspace_id = WorkspaceId::new(workspace_id).ok()?;
        let guard = db.lock().ok()?;
        LayoutService::load_by_workspace(&guard, &workspace_id).ok()
    }

    fn audit_freshness_evaluated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
        need: &ObservationConsumerFreshnessNeed,
        status: &workspace_domain::WorkspaceObservationStatus,
        decision: &str,
    ) -> Result<()> {
        let _ = ObservationRefreshContext::new()
            .with_consumer(need.consumer_id.clone())
            .with_purpose(
                need.context
                    .clone()
                    .unwrap_or_else(|| "environment_generate".into()),
            );
        AuditService::record_ai_planning_event(
            db,
            actor,
            intent,
            "workspace.environment.freshness_evaluated",
            true,
            json!({
                "consumer": need.consumer_id,
                "requirement": need.requirement.as_str(),
                "decision": decision,
                "freshness": status.freshness.as_str(),
                "age_seconds": status.age_seconds,
                "has_observation": status.has_observation,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }

    fn audit_generated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceEnvironmentState,
        workspace_state: &WorkspaceState,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.environment.generated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "window_count": state.windows.len(),
                "running_application_count": state.running_application_count,
                "missing_application_count": state.missing_application_count,
                "disconnected_work": state.disconnected_work,
                "observation_freshness": state.observation_freshness,
                "observation_refresh_decision": state.observation_refresh_decision,
                "observation_pass_id": workspace_state.metadata.observation_pass_id,
                "latest_delta_reference": workspace_state.metadata.latest_delta_reference,
                "workspace_state_has_changes": workspace_state.metadata.has_changes,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }
}

fn window_state_from_state_window(snap: &WorkspaceStateWindow) -> EnvironmentWindowState {
    if snap.focused {
        EnvironmentWindowState::Focused
    } else if snap.minimized {
        EnvironmentWindowState::Minimized
    } else if snap.visible {
        EnvironmentWindowState::Open
    } else {
        EnvironmentWindowState::Unknown
    }
}

fn display_label_for_window(snap: &WorkspaceStateWindow) -> String {
    match (&snap.monitor_name, snap.monitor_index) {
        (Some(name), Some(index)) => format!("{name} (monitor {index})"),
        (None, Some(index)) => format!("Monitor {index}"),
        _ => "Monitor placement unknown".into(),
    }
}

fn match_application<'a>(
    snap: &WorkspaceStateWindow,
    applications: &'a [ApplicationReference],
) -> Option<&'a ApplicationReference> {
    let title = snap.title.to_lowercase();
    applications.iter().find(|app| {
        app_matches_titles(
            &app.name,
            app.identifier.as_deref(),
            std::slice::from_ref(&title),
        )
    })
}

fn app_matches_titles(name: &str, identifier: Option<&str>, titles_lower: &[String]) -> bool {
    let name_l = name.to_lowercase();
    if !name_l.is_empty() && titles_lower.iter().any(|t| t.contains(&name_l)) {
        return true;
    }
    if let Some(identifier) = identifier {
        let id_l = identifier.to_lowercase();
        let stem = id_l.strip_suffix(".exe").unwrap_or(&id_l);
        if !stem.is_empty() && titles_lower.iter().any(|t| t.contains(stem)) {
            return true;
        }
    }
    false
}

fn build_groups(windows: &[EnvironmentWindow]) -> Vec<EnvironmentWindowGroup> {
    let mut by_app: HashMap<String, Vec<String>> = HashMap::new();
    let mut by_process: HashMap<u32, Vec<String>> = HashMap::new();
    let mut app_labels: HashMap<String, String> = HashMap::new();
    let mut app_projects: HashMap<String, Option<String>> = HashMap::new();

    for window in windows {
        if let Some(app_id) = &window.matched_application_id {
            by_app
                .entry(app_id.clone())
                .or_default()
                .push(window.id.clone());
            app_labels.insert(
                app_id.clone(),
                window
                    .matched_application_name
                    .clone()
                    .unwrap_or_else(|| app_id.clone()),
            );
            app_projects.insert(app_id.clone(), window.project_id.clone());
        } else {
            by_process
                .entry(window.process_id)
                .or_default()
                .push(window.id.clone());
        }
    }

    let mut groups = Vec::new();
    for (app_id, window_ids) in by_app {
        if window_ids.is_empty() {
            continue;
        }
        let label = app_labels
            .get(&app_id)
            .cloned()
            .unwrap_or_else(|| app_id.clone());
        let project_id = app_projects.get(&app_id).cloned().flatten();
        groups.push(EnvironmentWindowGroup {
            id: format!("group:app:{app_id}"),
            label: label.clone(),
            application_id: Some(app_id),
            process_id: None,
            window_ids,
            project_id,
            explanation: format!("Windows grouped by registered application \"{label}\"."),
        });
    }

    for (pid, window_ids) in by_process {
        if window_ids.len() < 2 {
            continue;
        }
        groups.push(EnvironmentWindowGroup {
            id: format!("group:pid:{pid}"),
            label: format!("Process {pid}"),
            application_id: None,
            process_id: Some(pid),
            window_ids,
            project_id: None,
            explanation: format!("Unmatched windows sharing process id {pid}."),
        });
    }

    groups.sort_by(|a, b| a.id.cmp(&b.id));
    groups
}
