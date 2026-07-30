# Architectural Evidence Report — Workspace Repository Archaeology

| Field | Value |
|-------|-------|
| **Purpose** | Objective architectural inventory for Principal Architect evaluation |
| **Status** | Evidence only — no implementation, no roadmap, no programme selection |
| **Date** | 2026-07-30 |
| **Repository tip audited** | `cursor/product-foundation-v15-34a5` @ `b8b7466` (branch cut for this report: `cursor/architectural-evidence-programme-34a5`) |
| **Product** | Windows-targeted Tauri 2 desktop app (React 18 + Vite frontend; Rust kernel + SQLite) |
| **Method** | Repository archaeology: crates, IPC registrations, migrations, product UI, governance docs |

**Non-goals of this document:** feature implementation, refactoring, product-direction change, programme recommendation, roadmap decisions.

---

## 1. Executive Summary

Workspace is a single-product Windows desktop application. The repository contains:

1. A **mature observation → projection pipeline** that produces an in-memory desktop runtime model (`workspace_domain::WorkspaceState`) with windows, monitors, groups, observation delta, behaviour, runtime memory, semantics, decisions, and attention.
2. A **product shell** whose primary surface (Stage) consumes that model for Flow/Focus visualisation, focus, and arrangement working sets; Apps is a registry library; Assistant answers from WorkspaceState and optionally composes Programme IV surfaces.
3. A **governed control path** for window bounds/focus and arrangement restore via PermissionGateway → `windows-integration` WindowController (real Win32 on Windows; stubs elsewhere).
4. A **large durable SQLite + CommandHandler surface** (~79 migrations, ~396 CommandHandler methods, **207** Tauri-registered IPC commands) spanning resource CRUD, observation, arrangements, and extensive Programme II–IV cognitive / evidence / aggregator APIs — many of which are diagnostic UI only or kernel-only (not registered in Tauri).
5. **Documented product vision domains** (devices, audio, phone, deep automation) that have **no corresponding product implementation** in the audited tree beyond aspirational docs.

**Evidence-based characterisation:** Desktop observation and interpreted WorkspaceState are the strongest architectural foundations. Product UX has begun consuming them (V14–V15). Canvas layout/zones, OS-installed app discovery, layout locking, continuous OS sampling, device/audio domains, and durable “session restore” as a product concept remain absent, partial, or diagnostic-only. Multiple overlapping “state / attention / decision / semantic” stacks exist by design (desktop planes vs aggregators vs Programme III envelope vs Programme IV evidence).

---

## 2. Repository Inventory

### 2.1 Cargo workspace

| Crate | Path | Role |
|-------|------|------|
| `workspace-app` | `app/src-tauri` | Tauri 2 shell; IPC adapters; kernel bootstrap (`workspace.db`) |
| `workspace-domain` | `packages/domain` | Shared models; ~110 module dirs; no DB/UI |
| `workspace-kernel` | `packages/kernel` | Command pipeline, services, permissions, lifecycle |
| `workspace-database` | `packages/database` | SQLite connection, migrations, repositories |
| `workspace-windows-integration` | `packages/windows-integration` | Sole OS window/process API boundary (DEC-008) |

Dependency direction: app → kernel → {database, domain, windows-integration}; database → domain; windows-integration → domain.

### 2.2 Subsystem evidence sheets

#### A. Window / native desktop integration

| # | Item | Evidence |
|---|------|----------|
| 1 | **Purpose** | Capture desktop monitors/windows; launch processes; set bounds; focus windows |
| 2 | **Status** | Functional on Windows (`win32.rs`: `EnumWindows`, `EnumDisplayMonitors`, `GetMonitorInfoW`, `set_bounds`, `focus`). Stubbed on non-Windows (`stub.rs`, `platform_*` factories) |
| 3 | **Ownership** | `workspace-windows-integration` only talks to OS APIs |
| 4 | **Runtime dependencies** | Windows crate `windows` 0.58 (cfg windows); kernel services call `platform_*` factories |
| 5 | **IPC** | Indirect via observation capture, arrangement restore/focus, application launch — not raw Win32 IPC |
| 6 | **Data ownership** | Capture produces observed snapshots; control returns `WindowControlOutcome` (`simulated: true/false`) |
| 7 | **WorkspaceState** | Capture → observation persist → `WorkspaceStateEngine` projection |
| 8 | **Reuse** | High — single OS boundary for all control/observation |
| 9 | **Debt** | Linux/cloud cannot exercise real Win32; simulated paths used in tests |
| 10 | **Missing** | Continuous OS event streams, thumbnails, minimize APIs, audio/device APIs (absent) |

#### B. Window discovery / observation

| # | Item | Evidence |
|---|------|----------|
| 1 | **Purpose** | Persist observation passes (monitors, windows, identities) |
| 2 | **Status** | Functional pipeline: `WorkspaceObservationService` + migrations `020`/`021`; scheduler/trigger services present |
| 3 | **Ownership** | Kernel observation services; domain contracts in `workspace_observation*` |
| 4 | **Dependencies** | Capturer; DB repositories; freshness/scheduler modules |
| 5 | **IPC** | `capture_workspace_observation`, get latest/by id, status, delta, freshness, scheduler status (Tauri-registered; product Stage prefers `get_workspace_state`) |
| 6 | **Data** | Durable observation tables; delta computed ephemerally |
| 7 | **WorkspaceState** | Primary input to `WorkspaceStateEngine` |
| 8 | **Reuse** | Foundation for all desktop understanding |
| 9 | **Debt** | Product UI largely stopped calling raw observation IPC; dual diagnostic vs product paths |
| 10 | **Missing** | Continuous sampling; OS hook-based change detection |

#### C. Window manipulation

| # | Item | Evidence |
|---|------|----------|
| 1 | **Purpose** | Move/resize (`set_bounds`) and focus via WindowController |
| 2 | **Status** | Functional path through PermissionGateway → controller; Stage uses `focus_desktop_window`; arrangements use restore |
| 3 | **Ownership** | Kernel `desktop_arrangement` service + commands; OS in windows-integration |
| 4 | **Dependencies** | Capability `desktop.restore` / `desktop.write` (IPC-SURFACE) |
| 5 | **IPC** | `focus_desktop_window`, `restore_desktop_arrangement` |
| 6 | **Data** | No durable “last focus” product store beyond observation |
| 7 | **WorkspaceState** | Focus does not mutate WorkspaceState; refresh re-projects after observation |
| 8 | **Reuse** | Shared controller for restore and Stage activate |
| 9 | **Debt** | Restore halt-on-failure; simulated on non-Windows |
| 10 | **Missing** | Snapping, minimize/maximize product APIs, batch geometry apply as Flow/Focus “OS apply” (docs place in Milestone G) |

#### D. Application discovery (installed)

| # | Item | Evidence |
|---|------|----------|
| 1 | **Purpose** | Discover installed applications on the OS |
| 2 | **Status** | **Not Present** as OS discovery — registry is manual `create_application` / `list_applications` |
| 3 | **Ownership** | Application repository + Commands |
| 4 | **Dependencies** | Workspace-scoped application rows |
| 5 | **IPC** | `list_applications`, `create_application`, `launch_application`, get/delete |
| 6 | **Data** | Durable `applications` table |
| 7 | **WorkspaceState** | `active_applications` derived from observed windows (running), not install discovery |
| 8 | **Reuse** | Library launch + Stage `matchLibraryApp` heuristic |
| 9 | **Debt** | Fuzzy name matching for Stage↔library |
| 10 | **Missing** | Start Menu / App Paths / packaged app enumeration |

#### E. Running application tracking

| # | Item | Evidence |
|---|------|----------|
| 1 | **Purpose** | Track processes/apps currently on the desktop |
| 2 | **Status** | Functional via `WorkspaceState.active_applications` + `windows` |
| 3 | **Ownership** | Projection on WorkspaceState; Stage owns product UX |
| 4 | **Dependencies** | Observation |
| 5 | **IPC** | Via `get_workspace_state` |
| 6 | **Data** | Ephemeral projection (rebuilt each get) |
| 7 | **WorkspaceState** | Direct fields |
| 8 | **Reuse** | Stage, Assistant local answers, Operator dump |
| 9 | **Debt** | Former Apps “Running” grid deleted (V15); orphan CSS `.app-object-*` remains |
| 10 | **Missing** | Durable running-app history beyond observation ring |

#### F. WorkspaceState desktop representation

| # | Item | Evidence |
|---|------|----------|
| 1 | **Purpose** | Canonical interpreted desktop runtime projection |
| 2 | **Status** | Mature projection modules V9–V13 planes; product consumption V14–V15 |
| 3 | **Ownership** | Domain `workspace_state` + `desktop_*`; kernel `WorkspaceStateEngine` |
| 4 | **Dependencies** | Observation history, delta, arrangements membership, identities |
| 5 | **IPC** | `get_workspace_state` (+ freshness ensure) |
| 6 | **Data** | **Ephemeral** — not a repository; rebuilt in memory |
| 7 | **WorkspaceState** | Self |
| 8 | **Reuse** | Stage, Apps (post-launch refresh), Assistant, Operator, Environment input |
| 9 | **Debt** | Three names: domain desktop `WorkspaceState`, kernel lifecycle `WorkspaceState`, Programme III `WorkspaceStateEnvelope` |
| 10 | **Missing** | Workspace-scoped desktop state (logged as architecture-gated); durable attention/behaviour across restarts beyond observation samples |

**Planes on domain `WorkspaceState`:** `metadata`, `focused_window`, `active_applications`, `windows`, `monitors`, `window_groups`, `latest_delta`, `behaviour`, `runtime_memory`, `semantics`, `decisions`, `attention`, `authority_effect` (`packages/domain/src/workspace_state.rs`; mirrored in `app/src/types/domain.ts`).

#### G. Workspace models / switching

| # | Item | Evidence |
|---|------|----------|
| 1 | **Purpose** | Named workspace profiles (product “Profiles”) |
| 2 | **Status** | Functional CRUD + active id in settings; Stage works without profile |
| 3 | **Ownership** | Workspace repository; `WorkspaceSwitcher` UI |
| 4 | **Dependencies** | Settings `active_workspace_id` |
| 5 | **IPC** | `list_workspaces`, `create_workspace`, `get_workspace`, settings update |
| 6 | **Data** | Durable workspaces table |
| 7 | **WorkspaceState** | Desktop observation is **global** (not workspace-scoped) per optimisation log / engine design |
| 8 | **Reuse** | Scopes applications, arrangements, zones |
| 9 | **Debt** | “Workspace” name collision with desktop WorkspaceState |
| 10 | **Missing** | Switching that remaps OS desktop into a different saved geometry set as primary product metaphor (arrangements are separate) |

#### H. Layout models / canvas

| # | Item | Evidence |
|---|------|----------|
| 1 | **Purpose** | Canvas layout graph / zones (historical shell) |
| 2 | **Status** | **Partial** — DB + kernel `LayoutService` / zone CRUD exist; product Stage does **not** render canvas zones; `layoutPersistence.ts` unused by App |
| 3 | **Ownership** | Layout/zone repositories; Operator seeds zones |
| 4 | **Dependencies** | Workspace id |
| 5 | **IPC** | layout/zone commands registered; several “registered not invoked by React” per IPC-SURFACE |
| 6 | **Data** | Durable layouts/zones |
| 7 | **WorkspaceState** | Not the desktop layout plane |
| 8 | **Reuse** | Low for current Stage product |
| 9 | **Debt** | Stale Operator copy (“zones on Layouts tab”); orphan frontend layout types |
| 10 | **Missing** | Product canvas as primary experience (explicitly superseded by Desktop Reality Stage charter) |

#### I. Desktop arrangement

| # | Item | Evidence |
|---|------|----------|
| 1 | **Purpose** | Named durable HWND/relationship captures; restore via WindowController |
| 2 | **Status** | Functional capture/list/restore/focus; Stage working-set overlay + Profiles/Stage rail |
| 3 | **Ownership** | Domain `desktop_arrangement`; kernel service; migrations `079`–`080` |
| 4 | **Dependencies** | Observation for capture; Gateway for restore |
| 5 | **IPC** | `capture_desktop_arrangement`, `restore_desktop_arrangement`, `list/get`, `focus_desktop_window` |
| 6 | **Data** | Durable arrangements + bounds |
| 7 | **WorkspaceState** | Arrangement membership can strengthen `window_groups`; separate from canvas Layout |
| 8 | **Reuse** | Primary “remember/restore desktop organisation” mechanism |
| 9 | **Debt** | Dual UI (Stage panel + working set); not a full layout editor (Milestone F in docs) |
| 10 | **Missing** | Interactive arrangement editing, lock |

#### J. Persistence / SQLite / settings

| # | Item | Evidence |
|---|------|----------|
| 1 | **Purpose** | Durable store for resources, observation, Programme stacks, settings |
| 2 | **Status** | Mature migration set (**79** SQL files, `001`–`080` with skip); repositories for major tables |
| 3 | **Ownership** | `workspace-database` |
| 4 | **Dependencies** | App data dir `workspace.db` |
| 5 | **IPC** | None directly from React (architecture rule) |
| 6 | **Data** | See migration families: core, observation, arrangements, Programme II–IV |
| 7 | **WorkspaceState** | Not persisted as a row; built from observation |
| 8 | **Reuse** | Entire product durability |
| 9 | **Debt** | Linux tempdir test `READONLY_DBMOVED` (AGENTS.md); large schema surface vs product UX |
| 10 | **Missing** | N/A for store itself; product gaps are about what is *not* stored (e.g. desktop WorkspaceState) |

Settings: theme, first_run, `active_workspace_id` via `get_settings` / `update_settings`.

#### K. Session restoration

| # | Item | Evidence |
|---|------|----------|
| 1 | **Purpose** | Restore prior desktop/session |
| 2 | **Status** | **Partial / ambiguous:** (a) arrangement restore = functional desktop geometry restore; (b) `WorkspaceSessionState` aggregator = semantic projection for diagnostics/intelligence, not OS session manager |
| 3 | **Ownership** | Arrangements vs `WorkspaceSessionService` |
| 4 | **Dependencies** | Distinct |
| 5 | **IPC** | Arrangement restore; `generate_workspace_session` family (product-labeled aggregators in IPC-SURFACE; used from diagnostic panels) |
| 6 | **Data** | Arrangements durable; session aggregator ephemeral/snapshot patterns |
| 7 | **WorkspaceState** | Behaviour has observation “sessions”; not Windows user-session restore |
| 8 | **Reuse** | Arrangement restore is the concrete control mechanism |
| 9 | **Debt** | Naming overlap “session” |
| 10 | **Missing** | Full OS login-session / reboot desktop reconstitution product |

#### L. Monitor awareness

| # | Item | Evidence |
|---|------|----------|
| 1 | **Purpose** | Multi-monitor geometry in observation and Stage plane |
| 2 | **Status** | Functional capture + `WorkspaceState.monitors`; Stage layouts tiles against monitor union; groups by `monitor_index` |
| 3 | **Ownership** | Capture + WorkspaceState |
| 4 | **Dependencies** | Win32 monitor enum |
| 5 | **IPC** | Embedded in `get_workspace_state` |
| 6 | **Data** | Per observation pass |
| 7 | **WorkspaceState** | `monitors` field |
| 8 | **Reuse** | Stage spatial map |
| 9 | **Debt** | Window limit truncation (`WORKSPACE_STATE_WINDOW_LIMIT = 50`) |
| 10 | **Missing** | Per-monitor product organisation UI beyond map |

#### M. IPC / Tauri

| # | Item | Evidence |
|---|------|----------|
| 1 | **Purpose** | Frontend ↔ kernel command surface |
| 2 | **Status** | **207** Tauri handlers; **396** CommandHandler methods; **194** handler methods not in Tauri; React via `app/src/lib/ipc.ts` |
| 3 | **Ownership** | `app/src-tauri/src/commands/*` thin adapters |
| 4 | **Dependencies** | Managed `Arc<Mutex<WorkspaceKernel>>` |
| 5 | **IPC** | Documented in `docs/03-Engineering/IPC-SURFACE.md` |
| 6 | **Data** | DTOs in domain / frontend `domain.ts` |
| 7 | **WorkspaceState** | `get_workspace_state` labeled Diagnostic in IPC-SURFACE but used by product Stage |
| 8 | **Reuse** | Shared error taxonomy / capability paths |
| 9 | **Debt** | IPC label “Diagnostic” vs product use; large registered-but-unused-by-React set |
| 10 | **Missing** | Streaming IPC for live desktop (none) |

#### N. Assistant integration with workspace data

| # | Item | Evidence |
|---|------|----------|
| 1 | **Purpose** | Companion ask/answer over desktop understanding |
| 2 | **Status** | Functional: local answers from WorkspaceState; else `compose_workspace_assistant_turn` with enrich; Programme IV package panels optional |
| 3 | **Ownership** | Frontend `assistantCompanion.ts` + Programme IV assistant services |
| 4 | **Dependencies** | Shared WorkspaceState client; compose IPC |
| 5 | **IPC** | compose + package/get/explain assistant_* |
| 6 | **Data** | Chat history in sessionStorage; Programme IV durable packages in DB |
| 7 | **WorkspaceState** | Direct consumption for local Q&A and enrich |
| 8 | **Reuse** | Explains attention/decisions/semantics without re-deriving |
| 9 | **Debt** | Programme IV evidence Batches 1–9 mostly not Tauri-registered; parallel to desktop attention |
| 10 | **Missing** | Durable companion history; streaming |

#### O. Environment aggregator

| # | Item | Evidence |
|---|------|----------|
| 1 | **Purpose** | Read model: live desktop vs registered work / gaps |
| 2 | **Status** | Functional service consuming WorkspaceState; product primary UX does not depend on it for desktop truth (V14–V15); Operator/Developer still invoke `generate_workspace_environment` |
| 3 | **Ownership** | Aggregator (`platform_coherence`) |
| 4 | **Dependencies** | WorkspaceStateEngine + apps/workflow inputs |
| 5 | **IPC** | `generate_workspace_environment` |
| 6 | **Data** | Ephemeral aggregator output; remaps groups + matched_application |
| 7 | **WorkspaceState** | Primary input |
| 8 | **Reuse** | Gaps/diagnostics |
| 9 | **Debt** | Doc tension: IPC-SURFACE “Product” vs optimisation log “Operator/Developer only” |
| 10 | **Missing** | N/A as product SoT (explicitly not) |

#### P. Programme III–IV / cognitive stacks

| # | Item | Evidence |
|---|------|----------|
| 1 | **Purpose** | Cognitive graphs, envelopes, evidence packages, assistant intelligence |
| 2 | **Status** | Large durable + service implementation; Assistant packages exposed; evidence mostly CommandHandler/test |
| 3 | **Ownership** | Separate from desktop `WorkspaceState` planes |
| 4 | **Dependencies** | Composition envelope, not WorkspaceStateEngine (evidence services load composition snapshots) |
| 5 | **IPC** | Assistant yes; many evidence/cognitive generate APIs kernel-only |
| 6 | **Data** | Migrations `044`–`078` families |
| 7 | **WorkspaceState** | Parallel stack; desktop attention “Not Programme IV attention” |
| 8 | **Reuse** | Assistant compose path |
| 9 | **Debt** | Conceptual overlap with desktop semantics/decisions/attention; mega diagnostic UIs |
| 10 | **Missing** | Bridging to desktop planes (explicitly gated in optimisation log) |

### 2.3 Frontend product surfaces (classification)

| Surface | Path | Class |
|---------|------|-------|
| Stage (view id `layouts`) | `WorkspaceApplicationStage.tsx` | Product |
| Flow/Focus chrome | `WorkModeSwitch` / `workMode.ts` | Product (UI density + Stage organisation; not OS geometry apply) |
| Apps | `ApplicationsPanel.tsx` | Product library |
| Profiles | `WorkspaceSwitcher.tsx` | Product |
| Arrangements rail | `DesktopArrangementPanel.tsx` | Product control |
| Assistant rail | `AssistantCompanionRail` / `AssistantIntelligencePanel` | Product companion (+ nested diagnostic packages) |
| OperatorConsole | Tools → Diagnostics | Diagnostic |
| WorkspaceIntelligencePanel | Tools → Developer | Diagnostic |
| Shared state client | `workspaceStateClient.ts`, `useObservedWorkspaceState.ts` | Infrastructure |

### 2.4 Compile / environment facts (this tree)

- `cargo check -p workspace-kernel`: **succeeds** (182 warnings) on audited tip — AGENTS.md historical compile-break claim does not hold on this tip.
- Full `tauri dev` / real Win32 not the supported path on Linux cloud VMs (`AGENTS.md`).
- Known-good on Linux: `pnpm typecheck`, `pnpm test`, `pnpm build`, domain and windows-integration tests (per AGENTS.md).

---

## 3. Product Capability Matrix

Classification key: **Not Present** | **Prototype** | **Partial** | **Functional** | **Mature**

| Capability | Class | Evidence (brief) |
|------------|-------|------------------|
| Discover installed applications | **Not Present** | Manual registry only; no OS install enumerator |
| Discover running applications | **Functional** | `WorkspaceState.active_applications` + windows from observation |
| Observe desktop windows | **Functional** | Capture + persist + project; stubbed non-Windows |
| Represent windows | **Mature** | Rich `WorkspaceStateWindow` + Stage tiles |
| Represent monitors | **Functional** | Capture + Stage plane |
| Represent layouts (canvas) | **Partial** | DB/kernel exist; not product Stage |
| Save layouts (canvas) | **Partial** | Kernel/DB; unused by product shell |
| Restore layouts (canvas) | **Partial** | Same |
| Save desktop arrangements | **Functional** | `capture_desktop_arrangement` + Stage working sets |
| Restore desktop arrangements | **Functional** | Gateway → WindowController; simulated off-Windows |
| Switch workspaces (profiles) | **Functional** | List/create/activate + settings active id |
| Group applications/windows | **Functional** (read-only) | `window_groups` criteria; semantic relationships; no user-authored group control product |
| Lock layouts | **Not Present** | No lock model/API found |
| Multi-monitor awareness | **Functional** | Enum + projection + Stage map |
| Workspace persistence | **Functional** | Workspaces, apps, arrangements, settings durable |
| Desktop arrangement editing | **Partial** | Capture/restore/list; not interactive editor |
| Session restoration (OS/reboot) | **Partial** | Arrangement restore only; session aggregators ≠ OS restore |
| Focus window from product | **Functional** | Stage activate / IPC focus |
| Apply Flow/Focus as OS geometry | **Not Present** / gated | Docs: Milestone G; current Focus is Stage organisation only |
| Continuity / memory awareness | **Functional** (projection + Stage cues) | `runtime_memory` + V15 Stage cues |
| Behaviour timeline | **Mature** (projection) / **Partial** (product) | On WorkspaceState; Assistant explain; not Stage timeline |
| Semantics / roles | **Functional** | Projection + Stage role cues |
| Decision support (desktop) | **Functional** (projection) | On WorkspaceState; Assistant; Stage via attention only |
| Attention | **Functional** | Projection + Stage primary cue |
| Assistant desktop Q&A | **Functional** | Local + compose |
| Environment gaps analysis | **Functional** (diagnostic) | Aggregator + Operator/Developer |
| Audio context | **Not Present** | Vision only |
| Phone / devices | **Not Present** | Vision only |
| Plugins | **Not Present** (product) | Vision doc only |
| Automation execution | **Partial** | Contracts/triggers/proposals + boundary “prepare” without execute; many attempt_execute guards |
| Permission-gated OS control | **Functional** | Gateway path for restore/focus/launch |

Maturity is not inflated: “Mature” reserved for planes with deep module tests + stable product consumption or durable contracts with broad internal use.

---

## 4. Architectural Ownership Map

### 4.1 Authoritative desktop runtime chain

```text
windows-integration (capture / control)
  → WorkspaceObservationService (durable facts)
  → ObservationDeltaService (ephemeral change)
  → WorkspaceStateEngine
  → workspace_domain::WorkspaceState  ← product SoT for desktop understanding
  → Stage / Assistant / (Environment aggregator input)
```

### 4.2 Control chain

```text
UI intent → IPC → CommandPipeline → PermissionGateway → WindowController / ProcessLauncher
```

### 4.3 Parallel ownership (must not be conflated)

| Concept | Owner | Not owner |
|---------|-------|-----------|
| Desktop interpreted state | `WorkspaceStateEngine` / domain `WorkspaceState` | Environment, Programme III envelope |
| Kernel lifecycle state | `workspace_kernel::state::WorkspaceState` | Desktop projection |
| Programme III composition | `WorkspaceStateEnvelope` / CompositionService | Desktop engine |
| Desktop attention | `desktop_attention` on WorkspaceState | `WorkspaceAttentionService` aggregator / Programme IV |
| Desktop decisions | `desktop_decision` | DecisionQueue / DecisionEngine / DecisionSupport |
| Desktop semantics | `desktop_semantic` | `WorkspaceSemanticQuery` (P-IV) |
| Canvas Layout | Layout repository | DesktopArrangement |
| DesktopArrangement | Arrangement repository + DAF docs | Canvas Layout |
| OS APIs | `windows-integration` only | domain, React, Assistant |

`platform_coherence::PLATFORM_CONCEPT_OWNERS` encodes DurableStore vs Aggregator vs Derived for many concepts (`packages/domain/src/platform_coherence/mod.rs`).

### 4.4 Components evidenced as should remain untouched (boundary evidence)

- `windows-integration` as sole OS API crate
- PermissionGateway / CommandPipeline for mutations
- Domain purity (no Win32 in domain)
- React → IPC only (no direct database)
- Desktop `WorkspaceState` as ephemeral projection (not silently made SoT via Environment)

---

## 5. Reuse Opportunities

Facts about **existing** assets that are already built and either consumed or under-consumed:

| Asset | Present | Product consumption |
|-------|---------|---------------------|
| WorkspaceState planes (behaviour → attention) | Yes | Stage uses groups/attention/semantics/memory/delta; behaviour/decisions mainly Assistant |
| Shared `workspaceStateClient` | Yes | Stage, Apps, Assistant, Operator, Intelligence |
| WindowController focus/bounds | Yes | Stage + arrangement restore |
| DesktopArrangement CRUD | Yes | Stage panel + working sets |
| Observation persistence | Yes | Feeds engine; raw IPC mostly diagnostic |
| Application registry + launch | Yes | Apps library + Stage secondary library |
| Environment aggregator | Yes | Diagnostic gaps; not product SoT |
| Programme IV assistant compose | Yes | Companion fallback path |
| Programme IV evidence 1–9 | Yes (kernel/DB) | Largely not Tauri-exposed |
| Canvas layout/zone stack | Yes (DB/kernel) | Not product Stage |
| OperatorConsole / IntelligencePanel | Yes | Diagnostics only |

---

## 6. Technical Debt Summary

| Debt | Evidence |
|------|----------|
| Naming collision: three “WorkspaceState” concepts | Domain desktop, kernel lifecycle, P-III envelope |
| Dual attention / decision / semantic stacks | Desktop planes vs aggregators vs P-IV |
| IPC-SURFACE labels `get_workspace_state` Diagnostic while Stage is product | Doc vs usage |
| Environment labeled Product in IPC-SURFACE; product coherence log says Operator/Developer only | Doc tension |
| Orphan frontend: `layoutPersistence.ts`, most `*Projection.ts` UI helpers (test-only), `.app-object-*` CSS, unused `activeApplication*` helpers | No App imports / no className |
| Mega diagnostic components (~4k / ~3.4k LOC) | OperatorConsole, WorkspaceIntelligencePanel |
| 194 CommandHandler methods without Tauri registration | Kernel/test surface larger than app IPC |
| 61 `*_attempt_execute` always-error guards | Architecture fences, not product features |
| Window projection truncation (50) | `WORKSPACE_STATE_WINDOW_LIMIT` |
| Non-Windows stub/simulated control | Cloud/Linux cannot validate real restore |
| Historical AGENTS.md compile-break note stale on this tip | `cargo check -p workspace-kernel` ok |
| Vision domains (audio/phone/devices) unimplemented | PRODUCT-VISION.md vs code absence |

---

## 7. Product Alignment Summary

### 7.1 Vision sources (docs)

- Constitution / PRODUCT-VISION: adaptive desktop layer unifying apps, windows, devices, audio, automation, AI.
- Binding interaction model (`WORKSPACE-DESKTOP-INTERACTION-MODEL.md`): desktop environment representation and control for Windows — observe, represent, organise, control; Assistant secondary.
- Reality alignment (2026-07-30): spatial representation of the user’s active desktop; user does not build from empty canvas.

### 7.2 What already supports the vision (evidence)

- Live desktop representation on Stage from WorkspaceState
- Multi-monitor spatial map
- Flow/Focus as organisation modes on Stage
- Focus window and arrangement restore control paths
- Interpreted understanding planes (groups, continuity, semantics, decisions, attention)
- Assistant that narrates runtime conclusions
- Profile-scoped library and arrangements
- Permission-gated OS control boundary

### 7.3 What does not (evidence)

- Devices / phone / audio product domains
- OS-installed application discovery
- Layout lock
- Flow/Focus applying OS geometry (doc Milestone G)
- Interactive arrangement editor (doc Milestone F)
- Plugins
- Deep automation execution as user-facing finished product
- Canvas-first empty workspace (explicitly rejected by later charters; residual zone/layout code remains)

### 7.4 Under-utilised completed foundations

| Foundation | Under-utilisation evidence |
|------------|----------------------------|
| `behaviour` timeline | On WorkspaceState; Stage does not show timeline (by V15 choice); Assistant-only |
| `decisions` plane | Surfaced via attention/Assistant; no Stage inbox (intentional) |
| Observation raw IPC | Superseded for product by `get_workspace_state` |
| Canvas layout/zone stack | Durable but not Stage |
| Programme IV evidence packages | Implemented; mostly not product IPC |
| Environment matched_application grouping | Environment-local; not on global WorkspaceState (gated) |

### 7.5 Limiting factors for product capability (facts, not prescriptions)

Evidence shows product completeness is currently bounded by:

1. **Platform:** real Win32 only on Windows; no continuous OS sampling APIs in-tree.
2. **Control surface breadth:** focus + arrangement restore exist; no lock, snap, minimize product APIs, no Flow/Focus OS apply.
3. **Discovery:** running apps observed; installed apps not discovered.
4. **Scope model:** desktop observation global vs profile-scoped resources.
5. **Surface sprawl:** large durable Programme/aggregator surface vs narrow product shell.

---

## 8. Architectural Constraints

Documented and/or enforced constraints relevant to any future evaluation:

1. Only `windows-integration` may call OS window/process APIs.
2. Mutations go CommandPipeline → PermissionGateway (no React→DB; no Assistant→WindowController bypass).
3. Domain must not import Win32.
4. Desktop `WorkspaceState` is a projection, not a durable SoT table; Observation is SoT for raw desktop facts.
5. Environment / aggregators must not gain authority to move windows or become desktop SoT.
6. Programme III envelope is explicitly **not** source of truth.
7. Desktop attention ≠ Programme IV / aggregator attention.
8. Canvas `Layout` must not hold HWNDs; DesktopArrangement is the HWND relationship model (DAF docs).
9. Constitution: Workspace must not replace Windows.
10. Product interaction hierarchy: Desktop reality → Applications → Spatial organisation → Controls → Assistant.

---

## 9. Recommended Evidence (facts only)

The following facts are suitable as inputs to Architecture Evaluation. They are not programme selections.

1. **Fact:** Domain `WorkspaceState` currently carries windows, monitors, groups, delta, behaviour, runtime_memory, semantics, decisions, attention — produced by `WorkspaceStateEngine` from observation history.
2. **Fact:** Product Stage consumes those planes for map, Focus/Flow organisation, awareness cues; Apps is library-only; Assistant local-answers from WorkspaceState.
3. **Fact:** OS control available in-product: `focus_desktop_window`, `restore_desktop_arrangement`, `launch_application` (capability-gated).
4. **Fact:** OS control absent in-product: layout lock, minimize APIs, installed-app discovery, Flow/Focus geometry apply, audio/device.
5. **Fact:** DesktopArrangement is the durable remember/restore mechanism; canvas Layout is a separate residual stack.
6. **Fact:** Tauri exposes 207 commands; CommandHandler defines 396; Programme IV evidence engines largely lack Tauri registration.
7. **Fact:** Three distinct WorkspaceState-named concepts coexist (desktop projection, kernel lifecycle, P-III envelope).
8. **Fact:** Dual attention/decision/semantic stacks coexist (desktop planes vs aggregators/P-IV) with explicit “not the other” comments.
9. **Fact:** Persistence is extensive (79 migrations) relative to the narrow product shell surfaces.
10. **Fact:** V9–V13 built projection capability; V14–V15 pivoted to consumption/coherence and stopped after two empty audits each (OPTIMISATION_LOG).
11. **Fact:** `cargo check -p workspace-kernel` succeeds on audited tip despite older AGENTS.md compile-failure note.
12. **Fact:** Product vision documents still list devices/audio/phone; repository contains no product implementations of those domains.

---

## Appendix A — Subsystem checklist coverage

| Requested scope item | Covered in § |
|----------------------|--------------|
| Window integration | 2.2 A–C |
| Native desktop integration | 2.2 A |
| Window discovery | 2.2 B |
| Window manipulation | 2.2 C |
| Application discovery | 2.2 D |
| Running application tracking | 2.2 E |
| WorkspaceState desktop representation | 2.2 F |
| Workspace models / switching | 2.2 G |
| Layout models | 2.2 H |
| Canvas models | 2.2 H |
| Persistence / SQLite | 2.2 J |
| Session restoration | 2.2 K |
| Monitor awareness | 2.2 L |
| IPC / Tauri | 2.2 M |
| Settings | 2.2 J |
| UI relating to workspaces | 2.3, 2.2 G |
| Assistant + workspace data | 2.2 N |

## Appendix B — Primary evidence paths

- `packages/domain/src/workspace_state.rs`, `desktop_*`
- `packages/kernel/src/services/workspace_state_engine.rs`, `workspace_observation.rs`, `desktop_arrangement.rs`
- `packages/windows-integration/src/{win32,stub,controller,capture}.rs`
- `packages/database/migrations/`
- `app/src-tauri/src/lib.rs` (handler registration)
- `app/src/components/WorkspaceApplicationStage.tsx`, `ApplicationsPanel.tsx`, `AssistantIntelligencePanel.tsx`
- `app/src/lib/workspaceStateClient.ts`, `stageDesktopUi.ts`, `assistantCompanion.ts`
- `docs/03-Engineering/IPC-SURFACE.md`, `ARCHITECTURE-GOVERNANCE.md`, `DAF-*`
- `docs/01-Product/PRODUCT-VISION.md`, `WORKSPACE-DESKTOP-INTERACTION-MODEL.md`
- `docs/04-Operations/OPTIMISATION_LOG.md` (V9–V15)
- `docs/05-AI/WORKSPACE-ENVIRONMENT-MODEL.md`, `UNIFIED-WORKSPACE-STATE-ARCHITECTURE.md`
- `AGENTS.md`

---

*End of evidence report. No implementation contract. No programme recommendation.*
