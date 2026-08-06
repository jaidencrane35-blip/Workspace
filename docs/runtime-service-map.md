# Workspace Runtime Service Map

| Field | Value |
| --- | --- |
| **Purpose** | Inventory every runtime service / service-like module in `workspace-kernel` |
| **Authority** | `packages/kernel/src/services/*`, `packages/kernel/src/lib.rs` |
| **Date** | 2026-08-07 |
| **Note** | Most “services” are zero-sized namespaces with associated functions, invoked on demand from `CommandHandler`. They are **not** DI-injected instances. |

---

## Kernel-owned runtime objects

These are the only long-lived objects on `WorkspaceKernel` (`packages/kernel/src/lib.rs`):

| Object | Type | Startup | Shutdown | Thread ownership | External resources | State ownership |
| --- | --- | --- | --- | --- | --- | --- |
| Lifecycle state | `state::WorkspaceState` | `bootstrap_shell` → Starting; init → Ready | `begin_shutdown` → ShuttingDown | Caller thread (Tauri) | — | Kernel process-local |
| Database handle | `DatabaseServiceHandle` (`Arc<Mutex<Database>>`) | Open path or in-memory during initialize | Dropped with kernel | Mutex; locked per call | SQLite file `{app_data_dir}/workspace.db` | Durable via SQLite |
| Service registry | `ServiceRegistry` | Registers `database`, `configuration`, `workspace` names | Cleared with kernel | — | — | Health names only (not DI) |
| Event bus | `EventBus` | New on bootstrap; `AuditEventSubscriber` registered in `apply_runtime` | Dropped | Sync handlers on publisher thread | — | Subscriber list |
| Permission gate | `Arc<dyn PermissionGate>` → `StandardPermissionGate` | Bootstrap | — | — | — | Stateless |
| Permission policy | `Arc<dyn PermissionPolicy>` → `CapabilityBoundPolicy` | Bootstrap | — | — | — | Stateless |
| Orchestrated plans | `Arc<Mutex<OrchestratedPlanStore>>` | Empty on bootstrap | Dropped | Mutex | — | Process-local diagnostic |
| Assistant workflows | `Arc<Mutex<AssistantWorkflowStore>>` | Empty on bootstrap | Dropped | Mutex | — | Process-local diagnostic |
| Observation scheduler | `ObservationScheduler` | `start(ObservationScheduleConfig::disabled())` on disk init — **no thread** | `stop()` joins if running | Would own OS thread `"observation-scheduler"` when enabled | DB via shared handle | Config + join handle |

**Message flow (all commands):**

```
Tauri command → lock Arc<Mutex<WorkspaceKernel>>
  → CommandHandler → CommandPipeline
    → PermissionGateway → *Service::method(&db | &kernel pieces)
      → optional Win32 / EventBus.publish
```

**No tokio runtime** in `packages/kernel`.

---

## Process-static / singleton owners (not on WorkspaceKernel fields)

| Owner | File | Purpose | Thread | Persistence |
| --- | --- | --- | --- | --- |
| `WorkspaceRuntimeStateService` | `workspace_runtime_state.rs` | Live `WorkspaceRuntimeState` (observation + execution phases) | `OnceLock<Mutex<…>>` | Process-local; checkpoints via session store |
| `CaptureCoordinator` single-flight | `capture_coordinator.rs` | One capture at a time | `AtomicBool` | — |
| Observation ambient gate | `observation_trigger_admission.rs` | Blocks ambient sources | `AtomicBool` + rate mutex | Process lifetime closed |
| Explanation catalog | `explanation_catalog.rs` | Loaded JSON catalog | `OnceLock` | Embedded / file resource |
| Adaptation / interaction overlays | various | In-memory overlays | `OnceLock<Mutex<HashMap>>` | Process-local |

---

## Service inventory (on-demand modules)

Prefix path: `packages/kernel/src/services/`.

### Foundation

| Service | Purpose | Startup | Dependencies | External | State ownership |
| --- | --- | --- | --- | --- | --- |
| `DatabaseServiceHandle` | Shared DB | Kernel init | rusqlite | SQLite file | Durable |
| `ConfigurationService` | Settings boundary | On call | SettingsRepository | SQLite settings | Durable |
| `ServiceRegistry` | Health name tracking | Init registers 3 | — | — | Process-local names |
| `AuditService` | Persist audit events | On call | AuditRepository | SQLite | Durable append-only |
| `GraphService` | Graph node/edge registration | On call | GraphRepository | SQLite | Durable |

### Workspace entities

| Service | Purpose | Dependencies | External | State |
| --- | --- | --- | --- | --- |
| `WorkspaceService` | Workspace CRUD + graph | DB | SQLite | Durable |
| `ZoneService` | Zone CRUD + graph | DB | SQLite | Durable |
| `ApplicationService` | Application CRUD + graph | DB | SQLite | Durable |
| `ApplicationLaunchService` | Governed process launch | Win32 launcher | OS process | Side effect |
| `WidgetService` | Widget CRUD + graph | DB | SQLite | Durable |
| `LayoutService` | Spatial layout | LayoutRepository | SQLite | Durable |

### Observation / capture / restore

| Service | Purpose | Startup | Dependencies | External | State |
| --- | --- | --- | --- | --- | --- |
| `CaptureCoordinator` | Sole capture entry; single-flight | On call | Observation stack | Win32 capturer | Snapshot durable |
| `WorkspaceObservationService` | Capture/reconcile/persist | On call | Observation repos + capturer | Win32 + SQLite | Durable snapshots |
| `ObservationService` | Audit → neutral observations | On call | Audit | SQLite | Derived |
| `ObservationDeltaService` | Diff consecutive snapshots | On call | Observation repos | SQLite | Derived |
| `ObservationRefreshPolicyService` | Freshness need | On call | Observation status | — | Derived |
| `ObservationTriggerAuthority` | Sole trigger evaluation | On call | Admission → Capture | — | — |
| `ObservationTriggerAdmissionPolicy` | Admit/rate-limit; ambient closed | Static gate | — | — | Process gate |
| `ObservationScheduledTrigger` | Schedule tick → authority | Scheduler thread if enabled | TriggerAuthority | — | — |
| `ObservationScheduler` | Background ticks | **Disabled at init** | DB | OS thread when on | Config |
| `ObservationStartupTrigger` | Startup trigger | **Not called** | TriggerAuthority | — | Dormant code |
| `ObservationEventGateway` | Future events → triggers | On call | TriggerAuthority | — | — |
| `WorkspaceStateEngine` | Observation+delta → domain WorkspaceState | On call | Observation | SQLite | Derived |
| `DesktopActionService` | Plan resolution + declared effects | On call | Domain match rules | — | Plan objects |
| `RestoreExecutor` | Approved plan → place/focus | On call | WindowMutator | Win32 | Runtime phases + history |
| `SavedContextService` | Explicit Save Moment | On call | CaptureCoordinator + repo | Win32 + SQLite | Durable contexts |
| `WorkspaceSessionStore` | Load/save persistent session | Init hydrate / checkpoints | PersistentWorkspaceSessionRepository | SQLite | Durable session |
| `WorkspaceRuntimeStateService` | Live runtime bundle owner | Process static | Restore/capture publishers | — | Process-local |

### Permissions / execution intelligence

| Service | Purpose | Dependencies | State |
| --- | --- | --- | --- |
| `PermissionApprovalService` | Approvals + grants | PermissionApprovalRepository | Durable |
| `CapabilityResolver` (`discovery`) | Derived capabilities | Policy | Derived |
| `ActionCatalogService` | List actions | Domain catalog | Static |
| `SuggestionService` | Deterministic suggestions | Context/metrics | Derived |
| `SuggestionLifecycleService` | Lifecycle over audit | Audit | Derived |
| `SuggestionIntentService` | Suggestion → intent bridge | Domain mapping | Transient |
| `GovernedIntentExecutionService` | Prep execution request | Pipeline | Transient |
| `ExecutionGuardService` | Idempotency for execute | Outcomes | Derived |
| `ExecutionOutcomeService` | Outcomes from audits | Audit | Derived |
| `ExecutionCancellationService` | Cancellation decision | — | Transient |
| `ExecutionContextService` | Execution summary | Audit | Derived |
| `ExecutionReconciliationService` | Reconcile by request id | Outcomes | Derived |
| `WorkspaceContextService` | Compose context read model | Many projections | Derived |
| `WorkspaceAnalyticsService` | Metrics (“Learn”) | Observations | Derived |
| `WorkspaceProjectionService` | Snapshot projection | Entities/layout | Derived |

### AI (governed; stub models)

| Service | Purpose | Dependencies | State |
| --- | --- | --- | --- |
| `AiMemoryService` | Planning memory entries | AiMemoryRepository | Durable |
| `ModelProviderService` | Provider registry / stubs | In-memory providers | Process |
| `AiPlanningService` | Goals → proposals | Context | Transient |
| `AiEvaluationService` | Plan quality metrics | — | Derived / audit |
| `AiPersonalizationService` | Preference ranking | UserPreferenceRepository | Durable prefs |
| `AiParticipationService` | Build AiActionRequest | — | Transient |
| `AiOrchestrationService` | Multi-step plans via gateway | OrchestratedPlanStore | Process-local |
| `AiAssistantService` | Assistant workflow | AssistantWorkflowStore | Process-local |

### Automation / decisions / tasks

| Service | Purpose | Dependencies | State |
| --- | --- | --- | --- |
| `AutomationContractService` | Contract defs/approvals | AutomationContractRepository | Durable |
| `TriggerEvaluatorService` | Triggers → Intent Proposals | AutomationTriggerRepository | Durable events/proposals |
| `DecisionQueueService` | Aggregate human decisions | DecisionQueueRepository | Overlay durable |
| `DecisionEngineService` | Ranked candidates; never executes | DecisionEngineRepository | Durable DE artifacts |
| `WorkspaceRecommendationEngineService` | Typed recommendations | RecommendationLifecycleRepository | Overlay durable |
| `TaskGraphService` | Work graph | TaskGraphRepository | Durable |
| `WorkspaceIntentService` | Projects/tasks intent | WorkspaceIntentRepository | Durable |
| `PilotMeasurementService` | Consented pilot records | PilotMeasurementRepository | Durable |

### Cognition / presentation projections (generate_* family)

These assemble **read models** / proposals; they do not place windows or bypass the gateway.

| Service | Purpose |
| --- | --- |
| `WorkspaceIntelligenceService` | Awareness aggregator |
| `WorkspaceActivityGraphService` | History read model |
| `WorkspaceContinuityService` | Resume narrative |
| `WorkspaceAttentionService` | Prioritization |
| `WorkspaceEnvironmentService` | Environment from WorkspaceState |
| `WorkspaceCompositionService` | Logical working environment |
| `WorkspacePurposeService` | Why work exists |
| `WorkspaceEvolutionService` | How work changed |
| `WorkspaceOperatingStateService` | Current-situation snapshot |
| `WorkspacePatternService` | Recurring structures |
| `WorkspaceAdaptationService` | Improvement proposals → Intent handoff |
| `WorkspaceReadinessService` | Preparedness |
| `WorkspaceRuntimeService` | Runtime overview assembly |
| `WorkspaceSessionService` | Session snapshot (intelligence path; distinct from SessionStore) |
| `WorkspaceExperienceService` | Work-surface presentation state |
| `WorkspaceWorkContextService` | Semantic classification |
| `WorkspaceNavigationService` | Navigation paths |
| `WorkspaceMilestoneService` | Coordination projection |
| `WorkspaceWorkingStyleService` | Style patterns |
| `WorkspaceTransitionService` | Transitions between work states |
| `WorkspaceInteractionService` | Interaction opportunities |
| `WorkspaceProfileService` | Preferred setups |
| `ExplanationResolver` / catalog | DisplayReason mapping |
| `ResilienceValidation` | Boundary invariant checks |
| `workspace_scope` helpers | Attribution helpers |

---

## Startup sequence (production disk init)

Evidence: `WorkspaceKernel::initialize` + Tauri setup in `app/src-tauri/src/lib.rs`.

1. Tauri resolves `app_data_dir` / `workspace.db`.
2. `WorkspaceKernel::initialize(path)`:
   - `bootstrap_shell` (placeholder DB, empty registry, EventBus, StandardPermissionGate, CapabilityBoundPolicy, disabled scheduler).
   - `CommandHandler::initialize_workspace` (open DB, migrations, register foundation services, lifecycle Ready, publish started events).
   - `start_observation_scheduler(disabled)` — **no ambient thread**.
3. Kernel wrapped in `Arc<Mutex<_>>` for IPC.
4. **No** `ObservationStartupTrigger` invocation.

Shutdown: `begin_shutdown` → scheduler.stop → shutdown command → lifecycle ShuttingDown + audit.

---

## Background workers summary

| Worker | Present in code | Active in Product Proof init |
| --- | --- | --- |
| ObservationScheduler thread | Yes | **No** (disabled config) |
| Ambient capture | Admission rejects | **No** |
| Automation scheduled triggers | Kind exists | **No** auto-fire |
| AI worker process | DEC-011 docs | **No** |
| Plugin process | DEC-011 docs | **No** |
| Frontend instrumentation debounce | `setTimeout` | DEV persist only |

---

## Consumers of services

| Consumer | Path |
| --- | --- |
| Tauri IPC wrappers | `app/src-tauri/src/commands/*` → `CommandHandler` |
| Kernel command modules | `packages/kernel/src/commands/*` |
| In-crate tests | `*_tests.rs` using `initialize_in_memory` + stubs |
| Evidence harnesses | Windows-only product proof / E2E tests |
