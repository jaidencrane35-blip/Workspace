# Workspace IPC Surface Map

| Field | Value |
| --- | --- |
| **Purpose** | Inventory every registered Tauri IPC command and how it is used |
| **Authority** | `app/src-tauri/src/lib.rs` `generate_handler!` (197 commands); wrappers in `app/src-tauri/src/commands/`; callers via `app/src/lib/ipc.ts` `invokeIpc` |
| **Date** | 2026-08-07 |
| **Constraint** | Document only — **do not reduce** the IPC surface |
| **Enforced tiers** | `docs/03-Engineering/ipc-tiers.json` → `app/src/generated/ipcTiers.ts` (`pnpm verify:ipc-tiers`) |

---

## Constitutional tiers (§4.8) — enforced

| Tier | Count (2026-08-07 sync) | Authority |
| --- | --- | --- |
| Product | **20** | Experience catalog |
| Developer | **112** | React `invokeIpc` − Product − Diagnostic |
| Diagnostic | **37** | `ipc-tiers.json` |
| Experimental | **28** | Remainder of `generate_handler!` |
| Quarantine (flag) | **16** | Unused-by-React parity set |

Legacy A/B/C labels below map roughly to Product / Developer+Diagnostic / Experimental+Quarantine.

---

## Caller tiers (legacy narrative)

| Tier | Surface | Approx. commands |
| --- | --- | --- |
| A — Experience catalog / Product Proof | `app/src/demo/experienceIpcCatalog.ts` + mounted pilot panels | **20** (+ Guide: none) |
| B — Unmounted diagnostic UI | `OperatorConsole.tsx`, `WorkspaceIntelligencePanel.tsx`, `AssistantPanel.tsx`, `layoutPersistence.ts` | Large subset of remaining registered commands |
| C — Registered for CommandHandler parity | Comment block in `lib.rs`; also `docs/03-Engineering/IPC-SURFACE.md` “quarantined” | **16** rarely/never called from React |

Demo adapter (`app/src/demo/demoIpc.ts`) implements the experience catalog for DEV/browser without Tauri.

All React calls must go through `invokeIpc` (enforced by convention + tests).

---

## Column legend

| Column | Meaning |
| --- | --- |
| **Mutates** | Intended to change durable or OS state (Y/N/partial) |
| **DB** | Touches SQLite via repositories |
| **Win32** | Uses windows-integration |
| **AI** | AI service family |
| **Auto** | Automation contracts/triggers |
| **Caller** | A / B / C / none observed |

Permission: every mutation and governed query passes `CommandPipeline` → `PermissionGateway` with a capability (exact capability per command lives in kernel `*Command` impls). LocalUser typically allowed by `StandardPermissionGate` when capability present.

---

## Domain: Bootstrap & settings

| Command | Purpose | Caller | Mutates | DB | Win32 | AI | Auto |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `get_workspace_status` | Kernel status | A, B | N | N | N | N | N |
| `get_workspace_health` | Health + registry | A, B | N | N | N | N | N |
| `get_settings` | Read settings | A, B | N | Y | N | N | N |
| `update_settings` | Write settings | A, B | Y | Y | N | N | N |

---

## Domain: Workspace entities & layout

| Command | Purpose | Caller | Mutates | DB | Win32 |
| --- | --- | --- | --- | --- | --- |
| `create_workspace` | Create workspace | A, B | Y | Y | N |
| `get_workspace` | Read workspace | A, B | N | Y | N |
| `create_zone` | Create zone | B | Y | Y | N |
| `delete_zone` | Delete zone | C | Y | Y | N |
| `get_zone` | Read zone | C | N | Y | N |
| `create_application` | Create app entity | B | Y | Y | N |
| `delete_application` | Delete app | C | Y | Y | N |
| `get_application` | Read app | C | N | Y | N |
| `launch_application` | Governed OS launch | B | Y (OS) | Y | Y |
| `create_widget` / `delete_widget` / `get_widget` | Widget CRUD | C | Y/N | Y | N |
| `create_layout` / `update_layout` / `get_layout` | Layout | B (`layoutPersistence`) | Y/N | Y | N |
| `delete_layout` / `reset_layout` / `get_layout_snapshot` | Layout extras | C | Y/N | Y | N |
| `get_workspace_snapshot` | Projection snapshot | C | N | Y | N |
| `get_workspace_context` | Composed context | B | N | Y | N |
| `get_workspace_metrics` | Analytics | C | N | Y | N |

---

## Domain: Suggestions & governed execution

| Command | Purpose | Caller | Mutates | DB | Win32 | AI | Auto |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `get_suggestions` | List suggestions | B | N | Y | N | N | N |
| `accept_suggestion` / `reject_suggestion` | Lifecycle | B | Y (audit/lifecycle) | Y | N | N | N |
| `get_suggestion_lifecycle` | Lifecycle projection | B | N | Y | N | N | N |
| `create_suggestion_intent_request` | Bridge to intent | B | partial | Y | N | N | N |
| `execute_intent_request` | Governed execute | B | Y | Y | maybe | N | N |
| `get_execution_outcomes` / `get_execution_states` | Outcomes | B | N | Y | N | N | N |
| `get_execution_state` | Singular state | C | N | Y | N | N | N |
| `request_execution_cancellation` | Cancel | B | Y | Y | N | N | N |

---

## Domain: Observation

| Command | Purpose | Caller | Mutates | DB | Win32 |
| --- | --- | --- | --- | --- | --- |
| `capture_workspace_observation` | Explicit capture | B | Y | Y | Y |
| `get_latest_workspace_observation` | Latest snapshot | B | N | Y | N |
| `get_workspace_observation_by_id` | By id | B | N | Y | N |
| `get_workspace_observation_status` | Freshness metadata | B | N | Y | N |
| `get_observation_scheduler_status` | Scheduler health | B | N | N | N |
| `get_latest_observation_delta` | Delta facts | B | N | Y | N |
| `ensure_observation_freshness` | Manual ensure capture | B | maybe | Y | Y |
| `get_observations` | Audit-derived observations | C | N | Y | N |
| `get_workspace_state` | Domain WorkspaceState | B | N | Y | N |
| `get_workspace_runtime_state` | Live runtime bundle | B (operator) | N | N* | N |

\*Runtime state is process-local; may reflect persisted session hydration.

---

## Domain: Saved context / resume (Product Proof core)

| Command | Purpose | Caller | Mutates | DB | Win32 |
| --- | --- | --- | --- | --- | --- |
| `get_saved_context_capture_scope` | Consent scope | A | N | N | N |
| `save_workspace_context` | Save Moment (capture+persist) | A | Y | Y | Y |
| `list_saved_contexts` | List moments | A | N | Y | N |
| `get_saved_context` | Inspect | A | N | Y | N |
| `delete_saved_context` | Delete | A | Y | Y | N |
| `resolve_resume_plan` | Plan + compatibility | A | N | Y | N* |
| `execute_resume_plan` | Place/focus execute | A | Y (OS) | Y | Y |

\*Plan resolution uses matching rules; live rematch at execute uses Win32.

---

## Domain: Pilot measurement

| Command | Purpose | Caller | Mutates | DB |
| --- | --- | --- | --- | --- |
| `get_pilot_measurement_scope` | Scope disclosure | A | N | Y |
| `get_pilot_measurement_snapshot` | Snapshot | A | N | Y |
| `grant_pilot_consent` / `withdraw_pilot_consent` | Consent | A | Y | Y |
| `record_pilot_baseline` | Baseline self-report | A | Y | Y |
| `record_pilot_leave_resume` | Leave/resume event | A | Y | Y |
| `record_pilot_interview` | Interview responses | A | Y | Y |

---

## Domain: Permissions & AI diagnostics

| Command | Purpose | Caller | Mutates | DB | AI |
| --- | --- | --- | --- | --- | --- |
| `get_permission_approvals` | List approvals | B | N | Y | N |
| `decide_approval` | Allow/deny grant | B | Y | Y | N |
| `request_ai_application_launch` | AI launch request → gateway | B | maybe | Y | Y |
| `diagnose_ai_workspace_plan` | Plan diagnose | B | N | Y | Y |
| `diagnose_ai_plan_preview` | Preview | B | N | Y | Y |
| `diagnose_ai_plan_evaluation` | Evaluate | B | N | Y | Y |
| `get_ai_evaluation_history` | History | B | N | Y | Y |
| `create_orchestrated_ai_plan` … `cancel_orchestrated_ai_plan` | Orchestration (5 cmds) | B | Y (store) | Y | Y |
| `submit_assistant_goal` … `record_assistant_explanation_viewed` | Assistant (9 cmds) | B | Y (store) | Y | Y |

### Memory / model / personalization

| Commands | Caller | Mutates | DB | AI |
| --- | --- | --- | --- | --- |
| `create_memory_entry`, `list_memory_entries`, `get_memory_context`, `delete_memory_entry`, `clear_memory_entries` | B | Y/N | Y | Y |
| `list_model_providers`, `get_model_provider_metadata`, `test_model_provider_request`, `diagnose_model_proposal_generation` | B | N | N | Y |
| `create_user_preference`, `update_user_preference`, `get_preference_profile`, `delete_user_preference`, `set_personalization_enabled`, `diagnose_ai_plan_with_personalization`, `compare_personalized_vs_neutral_plan` | B | Y/N | Y | Y |

---

## Domain: Workspace intelligence & intent

| Commands | Purpose | Caller | Mutates | DB |
| --- | --- | --- | --- | --- |
| `create_project`, `list_projects`, `get_project` | Projects | B | Y/N | Y |
| `create_task`, `list_tasks`, `get_task`, `update_task_status` | Tasks | B | Y/N | Y |
| `set_active_work`, `get_workflow_context` | Active work | B | Y/N | Y |
| `generate_workspace_intelligence`, `compare_workspace_intelligence_states` | Intelligence RM | B | N | Y |

---

## Domain: Automation

| Commands | Purpose | Caller | Mutates | DB | Auto |
| --- | --- | --- | --- | --- | --- |
| `create_automation_contract` … `prepare_automation_contract_intent` (10) | Contract lifecycle | B | Y/N | Y | Y |
| `record_trigger_event`, `evaluate_triggers`, `record_and_evaluate_triggers`, `list_trigger_events`, `list_automation_intent_proposals`, `accept_automation_intent_proposal`, `reject_automation_intent_proposal` (7) | Trigger evaluation | B | Y/N | Y | Y |

Scheduled trigger **kind** does not auto-fire (domain/docs).

---

## Domain: Decision queue & decision engine

| Commands | Purpose | Caller | Mutates | DB |
| --- | --- | --- | --- | --- |
| `generate_decision_queue`, `mark_decision_item_viewed`, `defer_decision_item`, `dismiss_decision_item`, `accept_decision_item`, `reject_decision_item` | Queue overlays | B | Y/N | Y |
| `generate_decision_engine`, `select_decision_candidate`, `dismiss_decision_candidate`, `postpone_decision_candidate` | DE candidates | B | Y/N | Y |

---

## Domain: Cognition generate / compare / validate

Read-model generators (typically non-mutating except where named accept/reject/select):

| Family | Commands (count) | Caller |
| --- | --- | --- |
| Activity | `generate_workspace_activity_graph`, `get_workspace_activity_timeline` (2) | B |
| Continuity / Attention | `generate_workspace_continuity`, `generate_workspace_attention` (2) | B |
| Environment / Composition / Purpose / Evolution | 4× `generate_workspace_*` | B |
| Recommendation engine | `generate_workspace_recommendation_engine`, `present_recommendation`, `accept_recommendation`, `reject_recommendation`, `confirm_recommendation_decision`, `decline_recommendation_decision`, `revoke_recommendation_adapter_preparation` (7) | B |
| Operating / Pattern / Adaptation | `generate_workspace_operating_state`, `generate_workspace_pattern`, `generate_workspace_adaptation`, `review_adaptation_proposal`, `accept_adaptation_proposal`, `reject_adaptation_proposal` (6) | B |
| Readiness / Runtime / Session | `generate_workspace_readiness`, `generate_workspace_runtime_overview`, `generate_workspace_session`, `compare_workspace_sessions` (4) | B |
| Experience / Work context | `generate_workspace_experience`, `compare_workspace_experiences`, `generate_workspace_work_context`, `compare_workspace_work_contexts`, `validate_workspace_work_context` (5) | B |
| Navigation / Milestones / Working style / Transitions / Interactions | generate/compare/validate (+ `select_workspace_interaction`) | B |
| Profiles | `create_workspace_profile`, `update_workspace_profile`, `list_workspace_profiles`, `get_workspace_profile`, `generate_workspace_profile_state`, `compare_workspace_profile`, `compare_workspace_profile_states`, `validate_workspace_profile_state` (8) | B |

---

## Domain: Task graph

| Commands | Caller | Mutates | DB |
| --- | --- | --- | --- |
| `generate_task_graph`, `create_workspace_task`, `update_workspace_task_status`, `add_task_relationship`, `validate_task_graph`, `get_task_graph_planning_inputs` | B | Y/N | Y |

---

## Domain: Discovery / audit (parity)

| Command | Purpose | Caller | Mutates | DB |
| --- | --- | --- | --- | --- |
| `get_actor_capabilities` | Capability discovery | C (doc); may appear in B | N | N |
| `get_action_catalog` | Action list | B (OperatorConsole) | N | N |
| `get_audit_history` | Audit read | C | N | Y |

---

## Experience catalog (Product tier) — complete list

See `PRODUCT_IPC_COMMANDS` in `app/src/generated/ipcTiers.ts` (20 commands). Guide: no IPC.

---

## Quarantined / parity (Tier C) — documented unused-by-React set

From `docs/03-Engineering/IPC-SURFACE.md` + `lib.rs` comment block:

`delete_zone`, `get_zone`, `delete_application`, `get_application`, `create_widget`, `delete_widget`, `get_widget`, `delete_layout`, `reset_layout`, `get_layout_snapshot`, `get_workspace_snapshot`, `get_actor_capabilities`, `get_audit_history`, `get_observations`, `get_workspace_metrics`, `get_execution_state`

`lib.rs` also groups `get_action_catalog` in the parity comment; React **does** call it from OperatorConsole — doc/code comment inconsistency.

---

## Duplicate / consolidation opportunities (document only)

| Observation | Evidence |
| --- | --- |
| Singular vs plural execution state | `get_execution_state` (C) vs `get_execution_states` (B) |
| Many `generate_*` / `compare_*` / `validate_*` triples | Parallel cognition surfaces with similar shapes |
| Recommendation engine vs Decision engine vs Decision queue | Three human-decision overlays with distinct ownership (see prior RE/DE audit) |
| `generate_workspace_session` vs `WorkspaceSessionStore` | Different concepts; naming overlap |
| Experience catalog vs full IPC | Intentional freeze vs diagnostic breadth |

No consolidations are performed in this document set.

---

## Counts

| Metric | Value |
| --- | --- |
| Registered in `generate_handler!` | **197** |
| Experience catalog (Product) | **20** |
| Doc-quarantined unused-by-React | **16** |
| Kernel lifecycle IPC | None (`initialize`/`shutdown` not exposed) |
