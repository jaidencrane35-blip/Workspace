# Workspace State Authority Map

| Field | Value |
| --- | --- |
| **Purpose** | Document every authoritative state store and who may write/read it |
| **Authority** | Implementation in `packages/*`, `app/src` |
| **Date** | 2026-08-07 |

---

## Naming collision warning

| Name | Location | Meaning |
| --- | --- | --- |
| `WorkspaceState` | `packages/kernel/src/state/mod.rs` | Kernel **lifecycle** + version |
| `WorkspaceState` | `packages/domain/src/workspace_state.rs` | **Desktop observation projection** |

These are different types. Downstream docs must qualify “kernel lifecycle state” vs “domain desktop WorkspaceState”.

---

## Authoritative state catalogue

### 1. Kernel lifecycle state

| Field | Value |
| --- | --- |
| **Type** | `kernel::state::WorkspaceState` |
| **Owner** | `WorkspaceKernel` |
| **Writers** | Initialize / shutdown command path |
| **Readers** | Health, status IPC |
| **Lifetime** | Process |
| **Persistence** | None (ephemeral) |

### 2. SQLite database (`workspace.db`)

| Field | Value |
| --- | --- |
| **Owner** | `DatabaseServiceHandle` on kernel |
| **Writers** | Repositories via services after PermissionGateway allow |
| **Readers** | Same path (queries) |
| **Lifetime** | App data directory file |
| **Persistence** | Durable; migrations 001–046 |

Major durable domains (tables via migrations/repos):

| Domain | Repository / area | Writers (services) |
| --- | --- | --- |
| Settings | `SettingsRepository` | ConfigManager / UpdateSettings |
| Workspace / zone / app / widget | entity repos | Workspace/Zone/Application/WidgetService |
| Graph | `GraphRepository` | GraphService (+ entity creates) |
| Layout | `LayoutRepository` | LayoutService |
| Audit | `AuditRepository` | AuditService (+ EventBus subscriber) |
| Permission approvals | `PermissionApprovalRepository` | PermissionApprovalService / Gateway |
| AI memory | `AiMemoryRepository` | AiMemoryService |
| User preferences | `UserPreferenceRepository` | AiPersonalizationService |
| Workspace intent | `WorkspaceIntentRepository` | WorkspaceIntentService |
| Automation contracts/triggers | automation repos | AutomationContractService, TriggerEvaluatorService |
| Decision queue / engine | DE/DQ repos | DecisionEngineService, DecisionQueueService |
| Recommendation lifecycle | `RecommendationLifecycleRepository` | WorkspaceRecommendationEngineService |
| Task graph | `TaskGraphRepository` | TaskGraphService |
| Profiles | `WorkspaceProfileRepository` | WorkspaceProfileService |
| Observations | observation repos | WorkspaceObservationService |
| Saved contexts | `SavedContextRepository` | SavedContextService |
| Pilot measurement | `PilotMeasurementRepository` | PilotMeasurementService |
| Persistent session + recovery fence | `PersistentWorkspaceSessionRepository` | WorkspaceSessionStore |

### 3. Domain desktop WorkspaceState (derived)

| Field | Value |
| --- | --- |
| **Type** | `workspace_domain::WorkspaceState` |
| **Owner** | Derived by `WorkspaceStateEngine` |
| **Writers** | None directly — rebuilt from observations |
| **Readers** | `get_workspace_state`; environment/composition generators |
| **Lifetime** | Per request |
| **Persistence** | Underlying observations in SQLite |

### 4. WorkspaceRuntimeState (process-local live)

| Field | Value |
| --- | --- |
| **Type** | `WorkspaceRuntimeState` + `RuntimeHealth` |
| **Owner** | `WorkspaceRuntimeStateService` (`OnceLock<Mutex<…>>`) |
| **Writers** | CaptureCoordinator phases; RestoreExecutor phases; recovery/session hydration |
| **Readers** | `get_workspace_runtime_state` |
| **Lifetime** | Process |
| **Persistence** | Checkpoints via `PersistentWorkspaceSession` / fences — not a full mirror of all fields |

### 5. Persistent workspace session

| Field | Value |
| --- | --- |
| **Type** | `PersistentWorkspaceSession` |
| **Owner** | `WorkspaceSessionStore` |
| **Writers** | Hydration + success-path checkpoints |
| **Readers** | Startup recovery; runtime health |
| **Lifetime** | Cross-process (SQLite) |
| **Persistence** | Migrations 045–046 |

### 6. Observation snapshots

| Field | Value |
| --- | --- |
| **Owner** | `WorkspaceObservationService` / observation repos |
| **Writers** | CaptureCoordinator → capture path (explicit Manual under Product Proof) |
| **Readers** | Delta, state engine, save/resume, diagnostics |
| **Lifetime** | Durable |
| **Persistence** | SQLite observation tables |

### 7. Saved contexts (Moments)

| Field | Value |
| --- | --- |
| **Owner** | `SavedContextService` |
| **Writers** | `save_workspace_context` (after consent; refuses empty stub) |
| **Readers** | list/get/resolve/execute/delete IPC |
| **Lifetime** | Durable |
| **Persistence** | SQLite (+ restore identity fields) |

### 8. Configuration / settings

| Field | Value |
| --- | --- |
| **Owner** | `ConfigManager` / ConfigurationService |
| **Writers** | `update_settings` |
| **Readers** | Bootstrap, personalization flag |
| **Lifetime** | Durable |
| **Persistence** | SQLite `settings` keys |

### 9. Permission state

| Field | Value |
| --- | --- |
| **Owner** | PermissionApprovalService + Gateway |
| **Writers** | Gateway (pending); `decide_approval` (resolve grants) |
| **Readers** | Gateway grant merge; `get_permission_approvals` |
| **Lifetime** | Durable |
| **Persistence** | `permission_approval` tables |

### 10. Automation state

| Field | Value |
| --- | --- |
| **Owner** | AutomationContractService / TriggerEvaluatorService |
| **Writers** | Contract CRUD/approval lifecycle; trigger record/evaluate; proposal accept/reject |
| **Readers** | list/get/generate consumers |
| **Lifetime** | Durable definitions + events/proposals |
| **Persistence** | Automation migrations |
| **Note** | Does not own OS execution; prepare intent only |

### 11. AI memory & preferences

| Field | Value |
| --- | --- |
| **Owner** | AiMemoryService; AiPersonalizationService |
| **Writers** | Memory/preference IPC |
| **Readers** | Planning/personalization diagnose paths |
| **Lifetime** | Durable |
| **Persistence** | `ai_memory`, `user_preferences` |

### 12. AI orchestration / assistant (process-local)

| Field | Value |
| --- | --- |
| **Owner** | Kernel fields `orchestrated_plans`, `assistant_workflows` |
| **Writers** | Orchestration / assistant services |
| **Readers** | Matching get/advance/resume IPC |
| **Lifetime** | Process |
| **Persistence** | Not durable plan store (memory/prefs separate) |

### 13. Decision / recommendation overlays

| Field | Value |
| --- | --- |
| **Owner** | DecisionEngineService (DE artifacts); WorkspaceRecommendationEngineService (RE overlays); DecisionQueueService (queue overlays) |
| **Writers** | generate/select/dismiss/accept/reject families |
| **Readers** | Same IPC families |
| **Lifetime** | Durable overlays |
| **Persistence** | Distinct tables — RE vs DE ownership separation enforced in services (incl. debug_assert invariants) |

### 14. Task graph & workspace intent

| Field | Value |
| --- | --- |
| **Owner** | TaskGraphService; WorkspaceIntentService |
| **Writers** | create/update/relationship / project/task IPC |
| **Readers** | generate/validate/list |
| **Lifetime** | Durable |
| **Persistence** | SQLite |

### 15. Pilot measurement

| Field | Value |
| --- | --- |
| **Owner** | PilotMeasurementService |
| **Writers** | consent + record_* IPC |
| **Readers** | scope/snapshot |
| **Lifetime** | Durable (consented) |
| **Persistence** | SQLite |

### 16. Event bus (ephemeral notifications)

| Field | Value |
| --- | --- |
| **Owner** | `EventBus` |
| **Writers** | Commands publishing `DomainEvent` |
| **Readers** | `AuditEventSubscriber` (+ any test subscribers) |
| **Lifetime** | Process |
| **Persistence** | Indirect via audit records |

### 17. Caches

| Cache | Owner | Lifetime | Persistence |
| --- | --- | --- | --- |
| Desktop capture cache inside runtime state | WorkspaceRuntimeStateService | Process / pass id | Partial via session |
| Explanation catalog OnceLock | explanation_catalog | Process | Embedded JSON resource |
| Capture single-flight flag | CaptureCoordinator | Process | None |
| Ambient authorization flag | ObservationTriggerAdmissionPolicy | Process (closed) | None |

### 18. Frontend React state

| State | Owner | Writers | Readers | Lifetime | Persistence |
| --- | --- | --- | --- | --- | --- |
| `view`, `workspace`, toasts, busy | `App.tsx` | Bootstrap / navigation | Shell | Session | active_workspace_id → SQLite settings |
| CognitiveEngine context | `CognitiveEngine.tsx` | UI interactions | Shell/objects | Session | None |
| WorkspaceComposition context | `WorkspaceComposition.tsx` | Density/selection | Shell | Session | None |
| ActiveMoment | `ActiveMoment.tsx` | Stage expand | Stage | Session | None |

### 19. Frontend localStorage (presentation / DEV)

| Key prefix / key | Owner | Persistence |
| --- | --- | --- |
| `ws.experience.adaptation*.v1` | `app/src/experience/*` | localStorage |
| `ws.dev.experience.*.v1` | `app/src/dev/*` | localStorage |
| `workspace.active_id` | Legacy; migrated off | Removed after bootstrap |

These are **not** desktop/OS authority. Product Proof Moments live in SQLite via IPC.

### 20. Window / OS state

| Field | Value |
| --- | --- |
| **Owner** | Windows OS |
| **Writers (Workspace)** | `WindowMutator` place/focus; `ProcessLauncher` launch — only after approved commands |
| **Readers** | `DesktopCapturer` / enumerator |
| **Lifetime** | OS |
| **Persistence** | OS; Workspace stores observations/plans, not a window manager replacement |
| **Limits** | Closed apps not relaunched; z-order unsupported; SetForegroundWindow may refuse |

---

## Authority rules (implemented)

1. UI must not write SQLite except through IPC → kernel → pipeline → gateway.
2. Only `windows-integration` performs Win32.
3. Capture requires explicit trigger admission (Manual under Product Proof).
4. Restore mutates OS only via `execute_resume_plan` → `RestoreExecutor` after approval path.
5. Automation/AI prepare intents; non-human actors get ApprovalRequired at the gate.
6. Derived projections (`generate_*`, WorkspaceStateEngine) are not alternate write authorities for OS layout.
