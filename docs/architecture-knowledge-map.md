# Workspace Architectural Knowledge Map

| Field | Value |
| --- | --- |
| **Purpose** | Definitive map of what Workspace **implements today**, reconstructed from repository evidence |
| **Authority order** | Executable behaviour > implementation source > documentation |
| **Branch observed** | `v2-dev` |
| **Date** | 2026-08-07 |
| **Companion docs** | `runtime-service-map.md`, `ipc-surface-map.md`, `state-authority-map.md`, `domain-contract-assessment.md`, `governance-map.md`, `kiro-comparison-readiness.md` |
| **Related audit** | `docs/repository-audit.md` (hygiene context; independently re-verified here) |

---

## What Workspace actually does

Workspace is a **Windows-targeted Tauri 2 desktop application** that:

1. Presents a companion shell (Home / Save / Continue / Check-in / Guide) over a React WebView.
2. Runs a Rust **Platform Kernel** that owns lifecycle, a command pipeline, a permission gateway, SQLite persistence, and Win32 desktop integration.
3. Lets a user **explicitly save** a named desktop moment (observe windows after consent), **preview a restore plan**, and **execute place/focus** effects on still-living windows in the same desktop session.
4. Records consented **pilot measurement** (self-report), not ambient sensing.
5. Exposes a much larger IPC surface for diagnostic / intelligence / automation / AI governance paths — implemented in the kernel and reachable from unmounted UI panels, but **not** the mounted Product Proof chrome.

It does **not** today: replace Windows; run a plugin runtime; run ambient observation at startup; relaunch closed apps on restore; ship multi-monitor placement as a passing gate; encrypt SQLite at application tier; or isolate AI/plugin workers in separate processes (DEC-011 aspirational).

---

## System context (implemented)

```
┌─────────────────────────────────────────────────────────────┐
│  React WebView (app/src)                                    │
│  WorkspaceShell → pilot destinations + experience/dev       │
└───────────────────────────┬─────────────────────────────────┘
                            │ Tauri IPC (invoke)
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  workspace-app (app/src-tauri)                              │
│  Thin #[tauri::command] wrappers + LocalUser actor context  │
└───────────────────────────┬─────────────────────────────────┘
                            │ CommandHandler
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  workspace-kernel                                           │
│  CommandPipeline → PermissionGateway → *Service functions   │
│  EventBus (sync) → AuditEventSubscriber                     │
│  ObservationScheduler (constructed; disabled at init)       │
└───────────────┬─────────────────────────────┬───────────────┘
                │                             │
                ▼                             ▼
┌───────────────────────────┐   ┌─────────────────────────────┐
│  workspace-database       │   │  workspace-windows-          │
│  SQLite + repositories    │   │  integration (Win32/stubs)  │
└───────────────────────────┘   └─────────────────────────────┘
                ▲
                │ types only
┌───────────────────────────┐
│  workspace-domain         │
│  Pure models (no I/O)     │
└───────────────────────────┘
```

---

## Subsystem catalogue

### Presentation Layer

| Aspect | Evidence |
| --- | --- |
| **Purpose** | Render companion Experience chrome and DEV presentation adaptation |
| **Responsibilities** | Shell layout, destination views, design tokens, motion, density; resolve presentation CSS vars |
| **Public interfaces** | React components under `app/src/components/`; `app/src/experience/index.ts` barrel |
| **Dependencies** | `invokeIpc` (`app/src/lib/ipc.ts`); `@tauri-apps/api`; `motion`; `lucide-react` |
| **Consumers** | End user (WebView); DEV evidence dashboard |
| **Lifecycle** | Mounted by `main.tsx` → `App.tsx`; shell stays mounted while destinations swap |
| **Authority** | Presentation only; must not own desktop truth |
| **Status** | **Implemented** — Product Proof chrome mounted; V2 adaptation pipeline in `experience/*` |
| **Future (docs)** | V2 handoff forbids new architectural layers without a brief (`architecture/V2_AGENT_HANDOFF.md`) |

### Workspace Shell

| Aspect | Evidence |
| --- | --- |
| **Purpose** | Persistent spatial environment for pilot destinations |
| **Responsibilities** | Dock navigation (`PILOT_PRIMARY_VIEWS`), atmosphere, canvas, command surface, cognitive/composition providers |
| **Public interfaces** | `WorkspaceShell.tsx` |
| **Dependencies** | `CognitiveEngine`, `WorkspaceComposition`, `AmbientLighting`, `WorkspaceCanvas`, `ActiveMoment`, `CommandSurface`, `useResolvedPresentation` |
| **Consumers** | `App.tsx` children: Home / Save / Continue / Check-in / Guide |
| **Lifecycle** | Remains mounted for session; view id is React state |
| **Authority** | UI navigation authority only |
| **Status** | **Implemented**; supersedes unmounted `CanvasShell.tsx` |
| **Future** | Experience freeze docs vs V2 refoundation — concurrent programmes (doc drift) |

### Rust Kernel

| Aspect | Evidence |
| --- | --- |
| **Purpose** | Central runtime authority for mutations, queries, permissions, audit |
| **Responsibilities** | Lifecycle, DB handle, EventBus, permission traits, command dispatch, observation scheduler handle |
| **Public interfaces** | `WorkspaceKernel`, `CommandHandler`, `KERNEL_VERSION` (`packages/kernel/src/lib.rs`) |
| **Dependencies** | `workspace-domain`, `workspace-database`, `workspace-windows-integration` |
| **Consumers** | `workspace-app` Tauri setup |
| **Lifecycle** | `initialize(db_path)` / `initialize_in_memory()` → Ready; `begin_shutdown()` stops scheduler + shutdown command |
| **Authority** | Sole runtime owner of pipeline execution |
| **Status** | **Implemented** (~85 service modules; mostly on-demand static methods) |
| **Future** | Docs historically said “no AI”; code implements AI/automation services — README lag |

### Workspace State (domain desktop state)

| Aspect | Evidence |
| --- | --- |
| **Purpose** | Canonical projection of observed desktop into `WorkspaceState` |
| **Responsibilities** | Built by `WorkspaceStateEngine` from observation + delta (`packages/kernel/src/services/workspace_state_engine.rs`; domain `packages/domain/src/workspace_state.rs`) |
| **Public interfaces** | IPC `get_workspace_state` |
| **Dependencies** | Observation snapshots in SQLite |
| **Consumers** | Diagnostic UI; environment/composition generators |
| **Lifecycle** | Derived on read / after capture — not a long-lived writable store |
| **Authority** | Domain `WorkspaceState` ≠ kernel lifecycle `state::WorkspaceState` (naming collision; different types) |
| **Status** | **Implemented** |
| **Future** | Multi-monitor fidelity incomplete (release gate FAIL) |

### Command Pipeline

| Aspect | Evidence |
| --- | --- |
| **Purpose** | Uniform mutation/query dispatch with permission + audit |
| **Responsibilities** | Optional intent validation → `PermissionGateway` → execute → `AuditService::record_command` |
| **Public interfaces** | `CommandPipeline`, `MutationCommand`, `QueryCommand` (`packages/kernel/src/commands/pipeline.rs`) |
| **Dependencies** | PermissionGateway, AuditService, ActionIntentValidationService |
| **Consumers** | All `CommandHandler` paths |
| **Lifecycle** | Per-invocation |
| **Authority** | Enforces that state-changing work is gated |
| **Status** | **Implemented** |
| **Future** | Documented in DEC-016/017; no structural replacement planned in V2 handoff |

### Permission Gateway

| Aspect | Evidence |
| --- | --- |
| **Purpose** | Authority boundary: policy + gate + approval + decision audit |
| **Responsibilities** | Merge grants; evaluate `CapabilityBoundPolicy`; `StandardPermissionGate`; persist ApprovalRequired; audit `permission.*` |
| **Public interfaces** | `PermissionGateway::require` (`packages/kernel/src/security/gateway.rs`) |
| **Dependencies** | DB permission_approval repo; policy; gate |
| **Consumers** | CommandPipeline |
| **Lifecycle** | Per request |
| **Authority** | Constitution AI sequence structurally enforced for non-human actors (ApprovalRequired) |
| **Status** | **Implemented** (Sprint 40 path) |
| **Future** | Plugin actor type exists; no plugin runtime |

### IPC Layer

| Aspect | Evidence |
| --- | --- |
| **Purpose** | Cross-process (WebView ↔ Rust) command surface |
| **Responsibilities** | `#[tauri::command]` wrappers; `IpcResponse` envelope; LocalUser actor context |
| **Public interfaces** | 197 commands in `app/src-tauri/src/lib.rs` `generate_handler!`; frontend `invokeIpc` |
| **Dependencies** | `Arc<Mutex<WorkspaceKernel>>` managed state |
| **Consumers** | Mounted pilot panels; unmounted OperatorConsole / Intelligence / Assistant; demo adapter |
| **Lifecycle** | Process lifetime of Tauri app |
| **Authority** | Transport only; kernel owns semantics |
| **Status** | **Implemented**; experience catalog freezes 21 commands for Product Proof |
| **Future** | See `docs/ipc-surface-map.md` — document only, no reduction in this work |

### Persistence Layer / SQLite / Repositories / Migrations

| Aspect | Evidence |
| --- | --- |
| **Purpose** | Local-first durable storage (DEC-010) |
| **Responsibilities** | Open DB, run migrations, repository CRUD, settings keys |
| **Public interfaces** | `Database`, `DatabaseService`, `MigrationRunner::bundled()`, ~24 repositories |
| **Dependencies** | `rusqlite` bundled; `workspace-domain` types |
| **Consumers** | Kernel services via `DatabaseServiceHandle` |
| **Lifecycle** | Path `{app_data_dir}/workspace.db` at Tauri startup |
| **Authority** | Durable truth for entities, audit, observations, sessions, approvals |
| **Status** | **Implemented** — 45 SQL migrations (`001`–`046`, no `003`); Tier-0 `NoOpEncryptionProvider` |
| **Future** | Encryption tiers 1/2 documented placeholders (DEC-015) |

### Windows Integration

| Aspect | Evidence |
| --- | --- |
| **Purpose** | Sole OS API boundary (DEC-008) |
| **Responsibilities** | Enumerate/capture desktop; launch processes; place/focus windows |
| **Public interfaces** | Traits `DesktopCapturer`, `WindowEnumerator`, `ProcessLauncher`, `WindowMutator`; `platform_*()` factories |
| **Dependencies** | `windows` crate on Windows; stubs elsewhere |
| **Consumers** | CaptureCoordinator, RestoreExecutor, ApplicationLaunchService |
| **Lifecycle** | Created per operation via platform factories |
| **Authority** | Only crate allowed to talk Win32 |
| **Status** | **Implemented** on Windows; stubs off-Windows; product-proof tests write `architecture/evidence/` |
| **Future** | Multi-monitor topology harness FAIL for release gate |

### Automation

| Aspect | Evidence |
| --- | --- |
| **Purpose** | Governed automation **definitions** and trigger→intent proposals — not silent execution |
| **Responsibilities** | Contracts, approvals, pause/resume/revoke; trigger evaluation → Intent Proposals |
| **Public interfaces** | IPC under `create_automation_contract` … `reject_automation_intent_proposal` |
| **Dependencies** | SQLite automation tables; PermissionGateway for any later execution path |
| **Consumers** | Unmounted intelligence/operator UI; kernel services |
| **Lifecycle** | Durable contracts; scheduled trigger kind exists but does not auto-fire |
| **Authority** | Definition ≠ execution; prepare intent only |
| **Status** | **Implemented** as governance surface; **not** a background automator |
| **Future** | Docs vision for richer automation; Batch 7 notes scheduled never auto-fires |

### AI

| Aspect | Evidence |
| --- | --- |
| **Purpose** | Observe→Learn→Suggest style assistance without autonomous action |
| **Responsibilities** | Memory, model providers (deterministic stubs), planning, evaluation, orchestration, assistant workflows, personalization ranking |
| **Public interfaces** | IPC memory/model/personalization/orchestrated/assistant families |
| **Dependencies** | Pipeline + gateway; in-memory `OrchestratedPlanStore` / `AssistantWorkflowStore` on kernel |
| **Consumers** | Unmounted `AssistantPanel` / OperatorConsole |
| **Lifecycle** | Plans/workflows process-local unless persisted via specific repos (memory/preferences) |
| **Authority** | Non-human actors → ApprovalRequired at gate |
| **Status** | **Implemented** with stub model providers; not LLM-backed by default |
| **Future** | DEC-011 AI worker processes not implemented |

### Plugin System

| Aspect | Evidence |
| --- | --- |
| **Purpose** | Future extension platform |
| **Responsibilities** | None in runtime |
| **Public interfaces** | Actor type `Plugin` in permission gate; `plugins/README.md` placeholder |
| **Dependencies** | — |
| **Consumers** | — |
| **Lifecycle** | — |
| **Authority** | Gate would ApprovalRequired Plugin actors if they existed |
| **Status** | **Not implemented** (DEC-011 out of scope) |
| **Future** | `docs/06-Plugins/PLUGIN-ARCHITECTURE-VISION.md` |

### Event Bus

| Aspect | Evidence |
| --- | --- |
| **Purpose** | In-process domain event fan-out |
| **Responsibilities** | Sync publish/subscribe; audit subscriber records domain events |
| **Public interfaces** | `EventBus::subscribe` / `publish`; `DomainEvent` variants |
| **Dependencies** | `Arc<Mutex<Vec<handlers>>>` |
| **Consumers** | `AuditEventSubscriber`; lifecycle/resource commands |
| **Lifecycle** | Kernel lifetime |
| **Authority** | Notification only; not a command bus |
| **Status** | **Implemented** (synchronous; no tokio) |
| **Future** | No async bus planned in current tip |

### Scheduling / Background Workers

| Aspect | Evidence |
| --- | --- |
| **Purpose** | Optional ambient observation ticks |
| **Responsibilities** | `ObservationScheduler` OS thread when enabled |
| **Public interfaces** | `start_observation_scheduler`, `get_observation_scheduler_status` |
| **Dependencies** | DB; ObservationScheduledTrigger → TriggerAuthority |
| **Consumers** | Would be ambient observation; Product Proof disables |
| **Lifecycle** | Started with `ObservationScheduleConfig::disabled()` on `initialize`; stopped on shutdown |
| **Authority** | Ambient capture gate closed (`AMBIENT_CAPTURE_AUTHORIZED = false`) |
| **Status** | **Code present; runtime dormant** |
| **Future** | Retained startup trigger unwired by design |

### Configuration

| Aspect | Evidence |
| --- | --- |
| **Purpose** | App settings |
| **Responsibilities** | theme, first_run, settings_version, active_workspace_id, personalization_enabled |
| **Public interfaces** | IPC `get_settings` / `update_settings`; `ConfigManager` |
| **Dependencies** | SQLite `settings` table |
| **Consumers** | App bootstrap; personalization |
| **Lifecycle** | Durable |
| **Authority** | Kernel ConfigurationService / ConfigManager |
| **Status** | **Implemented** |
| **Future** | — |

### Logging / Diagnostics

| Aspect | Evidence |
| --- | --- |
| **Purpose** | Operational logs + DEV evidence |
| **Responsibilities** | `log` + `env_logger` (info default); DEV `app/src/dev/*` localStorage governance/evidence |
| **Public interfaces** | log macros; ExperienceEvidenceDashboard (DEV) |
| **Dependencies** | — |
| **Consumers** | Developers; evidence JSON writers in Windows/kernel tests |
| **Lifecycle** | Process / localStorage |
| **Authority** | Non-authoritative for product behaviour |
| **Status** | **Implemented**; no `tracing` crate |
| **Future** | — |

### Testing

| Aspect | Evidence |
| --- | --- |
| **Purpose** | Contract and behavioural proof |
| **Responsibilities** | In-crate Rust tests; Vitest experience/governance; node verifiers (catalog, UI boundary, CSP) |
| **Public interfaces** | `pnpm test`, `cargo test -p …` |
| **Dependencies** | Vitest 2; Cargo |
| **Consumers** | CI (`.github/workflows/ci-pr.yml`) |
| **Lifecycle** | CI + local |
| **Authority** | Release evidence under `architecture/evidence/` |
| **Status** | **Implemented** (dense kernel coverage) |
| **Future** | — |

### Developer Tooling

| Aspect | Evidence |
| --- | --- |
| **Purpose** | Scripts + placeholders |
| **Responsibilities** | Explanation catalog sync/verify; CSP/boundary verify; `tools/` empty placeholder |
| **Public interfaces** | `scripts/*.mjs` |
| **Status** | **Partial** — scripts real; `tools/` placeholder |

### Security

| Aspect | Evidence |
| --- | --- |
| **Purpose** | Least privilege at command boundary; WebView CSP |
| **Responsibilities** | PermissionGateway; CSP pinned in `tauri.conf.json` + verifier; capabilities `core:default` |
| **Status** | **Implemented** for IPC/CSP; encryption Tier 0 only |
| **Future** | See governance map |

---

## Documentation vs implementation discrepancies

| Topic | Documentation says | Implementation shows |
| --- | --- | --- |
| Phase status | `docs/README.md`: Phase 1 “Ready to begin” | V1 baseline + V2 Sprint 74 tip on `v2-dev` |
| Experience freeze | `experience-freeze.md` / `23_*` freeze chrome | `40_*` V2 refoundation lifts presentation freeze for V2 track |
| Kernel README | Early “no AI/automation” exclusions | Full AI/automation/decision service modules |
| Process architecture | DEC-011 multi-process plugins/AI workers | Single Tauri process |
| IPC-SURFACE consumers | Canvas + Diagnostic primary | Mounted product = pilot chrome; CanvasShell unmounted |
| Encryption | Tiers described | `NoOpEncryptionProvider` only |
| Ambient observation | Research/future ambient | Explicitly disabled at Product Proof init |

---

## Dependency graphs (summary)

### Rust crates

```
workspace-domain
       ↑
       ├── workspace-database
       │         ↑
       │         └── workspace-kernel ──→ workspace-app
       │                    ↑
       └── workspace-windows-integration ─┘
```

### TypeScript packages (pnpm)

```
workspace (root scripts)
  ├── @workspace/app
  └── @workspace/tests  (imports app/src for assertions)
```

### Persistence / state / IPC

See dedicated maps: `state-authority-map.md`, `ipc-surface-map.md`, `runtime-service-map.md`.

---

## Architecture quality (explain only)

### Strengths

- Clear OS boundary (`windows-integration` only).
- Mutations structurally pass PermissionGateway.
- Product Proof path is behaviourally evidenced.
- Domain crate purity (no I/O).
- Dense kernel contract tests.

### Weaknesses / incompleteness

- Dual documentation authority tracks.
- Very large IPC vs small mounted chrome.
- Hand-duplicated TS domain types.
- Many cognition `generate_*` projections overlap conceptually.
- Plugin/AI process isolation not built.
- Application-level encryption absent.
- Unmounted diagnostic UIs retain large call surface.

### Duplicate / dead / incomplete abstractions

| Kind | Examples |
| --- | --- |
| Duplicate naming | Kernel `WorkspaceState` (lifecycle) vs domain `WorkspaceState` (desktop) |
| Dead / dormant | `ObservationStartupTrigger` unwired; scheduler disabled; ambient gate closed |
| Incomplete | Plugin runtime; encryption tiers; multi-monitor gate |
| Overbuilt vs mounted UI | ~197 IPC vs 21 experience catalog commands |

### Extension points (existing)

- Permission actor types (Plugin, RemoteSession) ready at gate
- Observation scheduler enablement (policy still blocks ambient)
- Migration runner for schema growth
- Experience presentation pipeline stages (locked at 5 for V2)
- ModelProvider registry (stub providers)

---

## Index of generated knowledge docs

| Document | Focus |
| --- | --- |
| `docs/architecture-knowledge-map.md` | This file — subsystem map |
| `docs/runtime-service-map.md` | Every kernel runtime service |
| `docs/ipc-surface-map.md` | Every IPC command by domain |
| `docs/state-authority-map.md` | Authoritative state owners |
| `docs/domain-contract-assessment.md` | Rust ↔ TS contracts |
| `docs/governance-map.md` | Permissions, audit, safety |
| `docs/kiro-comparison-readiness.md` | Preparation for future comparison |
