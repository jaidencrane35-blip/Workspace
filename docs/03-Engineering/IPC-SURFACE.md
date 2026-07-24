# IPC Surface Inventory

| Field | Value |
|-------|-------|
| **Purpose** | Document which Tauri IPC commands the React shell uses vs which remain registered for CommandHandler parity |
| **Owner** | Lead Software Engineer |
| **Dependencies** | `app/src-tauri/src/lib.rs`, `app/src` |
| **Update Process** | Update when React invokes change or IPC is added/removed |

---

## Product-used (Canvas + Diagnostic)

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
| `get_desktop_windows` | Diagnostic |
| `create_application` / `launch_application` | Diagnostic (governed launch) |
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

## Registered, unused by React (quarantined)

Kept for kernel CommandHandler parity and future UI. Not removed during foundation hardening to avoid breaking external/test callers of the Tauri surface.

- `delete_zone`, `get_zone`
- `delete_application`, `get_application`
- `create_widget`, `delete_widget`, `get_widget`
- `delete_layout`, `reset_layout`, `get_layout_snapshot`
- `get_workspace_snapshot`
- `get_actor_capabilities`, `get_audit_history`, `get_observations`, `get_workspace_metrics`
- `get_execution_state` (singular; UI uses list)

---

## Kernel lifecycle (no IPC)

- `initialize_workspace` / `initialize_workspace_in_memory` — Tauri setup
- `shutdown` — not exposed

---

## Rule

React must call IPC only via `app/src/lib/ipc.ts` (`invokeIpc`). Never import `workspace-database` from the app crate.
