# IPC Surface Inventory

| Field | Value |
|-------|-------|
| **Purpose** | Document which Tauri IPC commands the React shell uses vs which remain registered for CommandHandler parity |
| **Owner** | Lead Software Engineer |
| **Dependencies** | `app/src-tauri/src/lib.rs`, `app/src` |
| **Update Process** | Update when React invokes change or IPC is added/removed |

---

## Registered Product and Diagnostic Contracts

This table records the intended consumer and contract purpose. Actual React
usage is verified independently by `pnpm verify:ipc-contract`; registration
does not imply that the current UI invokes a command.

| Command | Consumer |
|---------|----------|
| `get_workspace_status` | App bootstrap |
| `get_workspace_health` | App bootstrap / Diagnostic |
| `get_settings` / `update_settings` | App + Diagnostic |
| `create_workspace` / `get_workspace` | Canvas + Diagnostic |
| `create_zone` | Canvas + Diagnostic |
| `create_layout` / `get_layout` / `update_layout` | Canvas (`layoutPersistence.ts`) |
| `get_workspace_context` | App bootstrap / Diagnostic |
| `get_suggestions` / `accept_suggestion` / `reject_suggestion` | Diagnostic |
| `get_suggestion_lifecycle` | Diagnostic |
| `create_suggestion_intent_request` / `execute_intent_request` | Diagnostic |
| `get_execution_outcomes` / `get_execution_states` | Diagnostic |
| `request_execution_cancellation` | Diagnostic |
| `capture_workspace_observation` | Diagnostic: governed desktop perception capture via CaptureCoordinator (`desktop.read`; rejects concurrent captures) |
| `get_latest_workspace_observation` | Diagnostic: latest observation snapshot (`desktop.read`) |
| `get_workspace_observation_by_id` | Diagnostic: observation snapshot by pass id (`desktop.read`) |
| `get_workspace_observation_status` | Diagnostic: observation freshness/status metadata only (`desktop.read`; never triggers capture) |
| `get_observation_scheduler_status` | Diagnostic: observation scheduler runtime health (`desktop.read`; no control) |
| `get_latest_observation_delta` | Diagnostic: latest vs previous observation delta facts (`desktop.read`; empty when &lt;2 snapshots) |
| `ensure_observation_freshness` | Diagnostic: Manual TriggerAuthority ensure for consumer freshness need (never Event/Plugin; never silent) |
| `get_workspace_state` | Diagnostic: canonical WorkspaceState projection from latest observation + delta (`desktop.read`) |
| `create_application` / `launch_application` | Diagnostic (governed launch) |
| `capture_desktop_arrangement` | Capture observed windows into a named DesktopArrangement (`desktop.write`) — Workspace arrangements rail |
| `restore_desktop_arrangement` | Governed restore via PermissionGateway → WindowController (`desktop.restore`) — explicit Restore button |
| `get_desktop_arrangement` / `list_desktop_arrangements` | Read saved arrangements (`desktop.read`) — arrangements rail |
| `request_ai_application_launch` | Diagnostic AI simulation (`AiActionRequest` → pipeline → ApprovalRequired) |
| `diagnose_ai_workspace_plan` | Diagnostic AI planning (workspace context → proposals → governed submissions) |
| `diagnose_ai_plan_evaluation` | Diagnostic AI evaluation (plan quality/outcomes; no authority) |
| `get_ai_evaluation_history` | Diagnostic evaluation history from operational audits |
| `create_orchestrated_ai_plan` | Diagnostic multi-step plan create (no execution) |
| `get_orchestrated_ai_plan` | Diagnostic multi-step plan preview |
| `advance_orchestrated_ai_plan` | Diagnostic advance steps through Permission Gateway |
| `resume_orchestrated_ai_plan` | Diagnostic resume after human approval decision |
| `cancel_orchestrated_ai_plan` | Diagnostic cancel multi-step plan |
| `submit_assistant_goal` | Product + diagnostic: goal → governed plan preview |
| `get_assistant_workflow` | Product + diagnostic: workflow preview/refresh |
| `confirm_assistant_workflow` | Product + diagnostic: confirm → Permission Gateway path |
| `resume_assistant_workflow` | Product + diagnostic: resume after human permission decision |
| `cancel_assistant_workflow` | Product + diagnostic: cancel assistant workflow |
| `revise_assistant_goal` | Product + diagnostic: revise goal → regenerate plan |
| `regenerate_assistant_plan` | Product + diagnostic: regenerate plan for same goal |
| `compare_assistant_plan_revisions` | Product + diagnostic: explain differences between revisions |
| `record_assistant_explanation_viewed` | Product + diagnostic: audit explanation view (`authority_effect: none`) |
| `create_project` / `list_projects` / `get_project` | Product + diagnostic: durable project intent |
| `create_task` / `list_tasks` / `get_task` / `update_task_status` | Product + diagnostic: durable task intent |
| `set_active_work` / `get_workflow_context` | Product + diagnostic: active project/task context |
| `generate_workspace_intelligence` | Product + diagnostic: read-only workspace understanding |
| `compare_workspace_intelligence_states` | Product + diagnostic: compare intelligence snapshots |
| `create_automation_contract` / `list_automation_contracts` / `get_automation_contract` | Product: governed automation contract definitions |
| `update_automation_contract` | Product: edit definition (material edits invalidate approval) |
| `request_automation_contract_approval` / `approve_automation_contract` | Product: definition approval lifecycle (≠ execution) |
| `pause_automation_contract` / `resume_automation_contract` / `revoke_automation_contract` | Product: pause/resume/revoke definitions |
| `prepare_automation_contract_intent` | Boundary only: materialize future Intent template (no execute) |
| `record_trigger_event` / `evaluate_triggers` / `record_and_evaluate_triggers` | Product: governed trigger evaluation (proposals only) |
| `list_trigger_events` / `list_automation_intent_proposals` | Product: inspect trigger events and intent proposals |
| `accept_automation_intent_proposal` / `reject_automation_intent_proposal` | Product: proposal review status (≠ execution) |
| `generate_decision_queue` | Product: aggregate Workspace Decision Queue (read-only aggregation) |
| `mark_decision_item_viewed` / `defer_decision_item` / `dismiss_decision_item` | Product: lifecycle overlay only (≠ source mutation for dismiss) |
| `accept_decision_item` / `reject_decision_item` | Product: delegate to source or return gateway handoff (≠ execute) |
| `generate_workspace_activity_graph` | Product: aggregate Workspace Activity Graph (read-only timeline + relationships) |
| `get_workspace_activity_timeline` | Product: same generate; return timeline slice only |
| `generate_workspace_continuity` | Product: aggregate Workspace Continuity (read-only resume narrative) |
| `generate_workspace_attention` | Product: aggregate Workspace Attention (read-only prioritization) |
| `generate_decision_engine` | Product: aggregate Decision Engine recommendations (never executes) |
| `select_decision_candidate` | Product: accept recommendation → planner handoff only |
| `dismiss_decision_candidate` / `postpone_decision_candidate` | Product: Decision Engine lifecycle overlay |
| `generate_task_graph` | Product: aggregate / refresh Workspace Task Graph |
| `create_workspace_task` / `update_workspace_task_status` | Product: graph node lifecycle (≠ execute) |
| `add_task_relationship` | Product: dependency edges with cycle prevention |
| `validate_task_graph` | Product/diagnostic: integrity check |
| `get_task_graph_planning_inputs` | Product: open incomplete nodes for Planner |
| `generate_workspace_environment` | Product: aggregate live desktop Environment Model (never moves windows) |
| `generate_workspace_composition` | Product: aggregate logical working environment Composition (never launches or groups) |
| `generate_workspace_purpose` | Product: aggregate why-work-exists Purpose Model (never executes or owns goals) |
| `generate_workspace_evolution` | Product: aggregate how-work-changed Evolution Model (never stores a second history) |
| `generate_workspace_recommendation_engine` | Product: aggregate what-might-help-next suggestions (never executes) |
| `present_recommendation` | Product: mark recommendation Presented (lifecycle overlay only) |
| `accept_recommendation` | Product: record human accept decision + outcome (never executes) |
| `reject_recommendation` | Product: record human reject decision + outcome (never executes) |
| `confirm_recommendation_decision` / `decline_recommendation_decision` | Product: explicit decision confirmation lifecycle (never executes) |
| `revoke_recommendation_adapter_preparation` | Registered reversible adapter-preparation boundary; not currently invoked by React |
| `generate_workspace_operating_state` | Product: aggregate what-is-happening-now snapshot (never executes or persists) |
| `generate_workspace_pattern` | Product: aggregate recurring structures (never predicts, profiles, or executes) |
| `generate_workspace_adaptation` | Product: aggregate possible improvement proposals (never applies) |
| `review_adaptation_proposal` | Product: mark adaptation reviewed (audit only) |
| `accept_adaptation_proposal` | Product: accept adaptation → Intent handoff only (never executes) |
| `reject_adaptation_proposal` | Product: reject adaptation (audit only; no workspace mutation) |
| `generate_workspace_readiness` | Product: aggregate preparedness for current work (never prepares/executes) |
| `generate_workspace_runtime_overview` | Product/diagnostic: live runtime context/health/operator overview (never executes) |
| `generate_workspace_session` | Product: project runtime working session from Intelligence (never executes) |
| `compare_workspace_sessions` | Product: compare two session snapshots (informational) |
| `generate_workspace_experience` | Product: present Session as calm Work experience (never executes) |
| `compare_workspace_experiences` | Product: compare two experience snapshots (informational) |
| `generate_workspace_work_context` | Product: classify semantic kind-of-work (never executes) |
| `compare_workspace_work_contexts` | Product: compare two work-context snapshots (informational) |
| `validate_workspace_work_context` | Product: validate work-context invariants (informational) |
| `generate_workspace_navigation` | Product: project inspection paths over understanding (never executes) |
| `compare_workspace_navigation` | Product: compare two navigation snapshots (informational) |
| `validate_workspace_navigation` | Product: validate navigation invariants (informational) |
| `generate_workspace_milestones` | Product: project progress toward meaningful outcomes (never plans/executes) |
| `compare_workspace_milestones` | Product: compare two milestone snapshots (informational) |
| `validate_workspace_milestones` | Product: validate milestone invariants (informational) |
| `generate_workspace_working_style` | Product: project observable operating patterns (never profiles/executes) |
| `compare_workspace_working_styles` | Product: compare two working-style snapshots (informational) |
| `validate_workspace_working_style` | Product: validate working-style invariants (informational) |
| `generate_workspace_transitions` | Product: explain movement between work states (never restores/executes) |
| `compare_workspace_transitions` | Product: compare two transition snapshots (informational) |
| `validate_workspace_transitions` | Product: validate transition invariants (informational) |
| `generate_workspace_interactions` | Product: project interaction opportunities (never executes) |
| `compare_workspace_interactions` | Product: compare two interaction snapshots (informational) |
| `validate_workspace_interactions` | Product: validate interaction invariants (informational) |
| `select_workspace_interaction` | Product: select opportunity → Intent handoff only (never executes) |
| `create_workspace_profile` | Product: create durable Environment Profile (never executes) |
| `update_workspace_profile` | Product: update durable Environment Profile (never executes) |
| `list_workspace_profiles` | Product: list profiles for a workspace |
| `get_workspace_profile` | Product: get one profile by id |
| `generate_workspace_profile_state` | Product: profile read model + alignment compare (informational) |
| `compare_workspace_profile` | Product: compare one profile vs current workspace (informational) |
| `compare_workspace_profile_states` | Product: compare two profile state snapshots (informational) |
| `validate_workspace_profile_state` | Product: validate profile state invariants (informational) |
| `create_memory_entry` / `list_memory_entries` / `get_memory_context` | Diagnostic governed memory (informational) |
| `delete_memory_entry` / `clear_memory_entries` | Diagnostic memory lifecycle |
| `diagnose_ai_plan_preview` | Diagnostic memory-aware plan (no execution) |
| `list_model_providers` / `get_model_provider_metadata` | Diagnostic model provider inventory |
| `test_model_provider_request` | Diagnostic provider invoke (intelligence only) |
| `diagnose_model_proposal_generation` | Diagnostic provider → proposals (no execution) |
| `create_user_preference` / `update_user_preference` / `get_preference_profile` | Diagnostic personalization CRUD |
| `delete_user_preference` / `set_personalization_enabled` | Diagnostic preference lifecycle / toggle |
| `diagnose_ai_plan_with_personalization` | Diagnostic preference-aware plan (no execution) |
| `compare_personalized_vs_neutral_plan` | Diagnostic personalized vs neutral compare |
| `get_action_catalog` | Diagnostic action catalog (existence ≠ authorization) |
| `get_permission_approvals` / `decide_approval` | Diagnostic (allow once / deny) |

---

## Registered, Not Currently Invoked by React

The verifier currently reports 182 registered commands, 142 commands invoked
by React, and 40 registered commands without a React caller. These remain
available for kernel parity, external consumers, or future UI wiring; they are
not evidence of an active UI flow.

- `delete_zone`, `get_zone`
- `delete_application`, `get_application`
- `create_widget`, `delete_widget`, `get_widget`
- `delete_layout`, `reset_layout`, `get_layout_snapshot`
- `get_workspace_snapshot`
- `get_actor_capabilities`, `get_audit_history`, `get_observations`, `get_workspace_metrics`
- `get_execution_state` (singular; UI uses list)
- `add_task_relationship`, `update_workspace_task_status`
- `capture_workspace_observation`, `get_latest_workspace_observation`, `get_workspace_observation_by_id`, `get_latest_observation_delta`
- `compare_workspace_intelligence_states`
- `delete_memory_entry`, `list_memory_entries`
- `evaluate_triggers`, `record_trigger_event`, `list_trigger_events`
- `get_assistant_workflow`, `get_orchestrated_ai_plan`
- `get_automation_contract`, `update_automation_contract`, `prepare_automation_contract_intent`
- `get_workspace_activity_timeline`
- `get_workspace_profile`, `list_workspace_profiles`, `update_workspace_profile`
- `list_tasks`, `update_task_status`
- `revoke_recommendation_adapter_preparation`

---

## Contract Ownership and Compatibility

- Rust domain models are canonical for serialized response fields.
- `app/src/types/domain.ts` mirrors public response models only; the IPC
  contract verifier checks high-risk mirrored structures and public error codes.
- `Task` (intent ownership) and `WorkspaceTask` (task-graph ownership) are
  distinct models and must not be merged.
- Top-level Tauri arguments use frontend camelCase mapped to Rust snake_case.
  Nested DTO field names follow their Rust `serde` representation.
- The response envelope is a discriminated contract:
  - success: `{ success: true, data: T }`
  - failure: `{ success: false, error: { code, message } }`
- Unit-returning commands serialize successful `data` as `null`.
- Additive optional fields are preferred for compatible evolution. Required
  field removal/rename or enum-value changes require an explicit contract phase.

## Error Contract

Every `KernelError::to_public()` code must exist in
`app/src/types/ipc-errors.ts`. `pnpm verify:ipc-contract` fails when the catalogs
drift. Infrastructure, integrity, permission, recommendation, and decision
engine failures remain distinct through `CommandError` and `IpcErrorBody`.

---

## Kernel lifecycle (no IPC)

- `initialize_workspace` / `initialize_workspace_in_memory` — Tauri setup
- `shutdown` — not exposed

---

## Rule

React must call IPC only via `app/src/lib/ipc.ts` (`invokeIpc`). Never import
`workspace-database` from the app crate. Run `pnpm verify:ipc-contract` whenever
commands, public errors, or mirrored domain contracts change.
