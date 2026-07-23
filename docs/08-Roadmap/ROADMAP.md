# Roadmap

| Field | Value |
|-------|-------|
| **Purpose** | Define the phased delivery plan for Workspace from foundation through mature product |
| **Owner** | Project Owner |
| **Dependencies** | [Product Vision](../01-Product/PRODUCT-VISION.md), [Scope Management](../01-Product/SCOPE-MANAGEMENT.md), [Project Constitution](../00-Constitution/PROJECT-CONSTITUTION.md) |
| **Update Process** | Project Owner updates at phase boundaries. Phase scope changes require Decision Log entry. |

---

## 1. Roadmap Overview

```
Phase 0        Phase 0.5       Phase 1           Phase 2            Phase 3+
Foundation     Decisions       Core Platform     Feature Expansion   Maturity
(docs)         (recorded)      (Tauri shell)     (domains + AI)      (plugins)
   │               │                │                  │                  │
   ▼               ▼                ▼                  ▼                  ▼
 DONE            DONE         Foundation complete   Gateway (S40)      Not started
                            Sprints 01–39 done      Phase 2 entry
```

---

## 2. Phase 0 — Foundation

**Goal:** Establish the professional engineering and product foundation.

**Status:** **Complete**

### Deliverables

- [x] Project constitution and governance
- [x] Product vision and scope management
- [x] Architecture principles and system overview
- [x] Engineering standards (coding, repo, docs, DoD, debt)
- [x] UX, AI, plugin, and security principles
- [x] Decision tracking and open questions
- [x] Roadmap and sprint structure
- [x] Repository initialisation and GitHub connection
- [x] Phase 0 improvement pass

---

## 3. Phase 0.5 — Decision Recording

**Goal:** Resolve Phase 1 architecture blockers and record approved decisions.

**Status:** **Complete**

### Deliverables

- [x] Technology stack decided — DEC-007 (Tauri + React + TypeScript + Rust + SQLite)
- [x] Windows integration model decided — DEC-008 (Hybrid Companion + Overlay)
- [x] Layout system decided — DEC-009 (Spatial Workspace Canvas)
- [x] Data persistence decided — DEC-010 (SQLite + JSON export)
- [x] Process architecture decided — DEC-011 (Multi-process)
- [x] Monorepo tooling decided — DEC-012 (pnpm workspaces)
- [x] Project license decided — DEC-006 (MIT)
- [x] AI confidence framework decided — DEC-013 (L0–L4)
- [x] AI memory retention decided — DEC-014 (User-controlled adaptive)
- [x] Architecture documents updated to reflect decisions
- [x] Open questions resolved and tracked

### Gate Criteria (Phase 0.5 → Phase 1)

| Criterion | Status |
|-----------|--------|
| All Phase 1 blocker decisions recorded in Decision Log | **Done** |
| Architecture documents reflect approved stack and models | **Done** |
| MIT License added to repository | **Done** |
| Remaining open questions do not block Phase 1 scaffolding | **Done** |

**Phase 1 is ready to begin after final verification.**

---

## 4. Phase 1 — Core Platform

**Goal:** Scaffold the Tauri application and deliver a running shell with spatial layout persistence.

**Status:** **Complete (foundation)** — Sprints 01–39 delivered. Permission Gateway is Phase 2 entry (Sprint 40).

### Technology Direction (DEC-007)

| Layer | Technology |
|-------|------------|
| Frontend | React + TypeScript |
| Desktop runtime | Tauri |
| Core runtime | Rust services |
| Data layer | SQLite |
| Monorepo | pnpm workspaces + Cargo workspace |

### Expected Deliverables

- [x] pnpm workspace and Cargo workspace configuration — Sprint 01
- [x] Tauri application scaffolding (`app/`) — Sprint 01
- [x] Platform kernel placeholder (`packages/kernel`) — Sprint 01 boundary only
- [x] SQLite foundation (`packages/database`) — Sprint 01
- [x] Workspace Core state + configuration foundation — Sprint 02
- [x] Settings persistence (SQLite) — Sprint 02
- [x] IPC conventions (React → Tauri → Kernel → Database) — Sprint 02
- [x] Runtime lifecycle, health reporting, and IPC envelope — Sprint 03
- [x] Internal event bus and command layer — Sprint 04
- [x] Workspace domain entities and repository pattern — Sprint 05
- [x] Command pipeline, permission seam, typed IDs, transactions — Sprint 06
- [x] Observability and audit foundation — Sprint 07
- [x] Actor and execution identity foundation — Sprint 08
- [x] Intent, capability, and policy foundation — Sprint 09
- [x] Governance hardening: resource addressing (DEC-016), read governance (DEC-017), FK enforcement — Sprint 10
- [x] Resource services foundation: GraphService, Zone/Application/Widget services, graph registration, resource IPC — Sprint 12
- [x] Workspace layout foundation: LayoutService, spatial persistence, layout IPC — Sprint 13
- [x] Workspace state projection foundation: WorkspaceProjectionService, GetWorkspaceSnapshot, projection IPC — Sprint 14
- [x] Intent execution foundation: ActionIntentRegistry, IntentExecutionService, command mapping — Sprint 15
- [x] Capability discovery foundation: CapabilityResolver, GetActorCapabilities, derived governance-aware discovery — Sprint 16
- [x] Workspace observation foundation: ObservationService, GetObservations, derived audit-based activity stream — Sprint 17
- [x] Observation analytics foundation: WorkspaceAnalyticsService, GetWorkspaceMetrics, deterministic learning layer — Sprint 18
- [x] Workspace context foundation: WorkspaceContextService, GetWorkspaceContext, deterministic context boundary — Sprint 19
- [x] Suggestion foundation: SuggestionService, GetSuggestions, deterministic proposal layer — Sprint 20
- [x] Suggestion approval foundation: AcceptSuggestion / RejectSuggestion decision commands (audit-backed, no automate) — Sprint 21
- [x] Suggestion lifecycle foundation: SuggestionLifecycleService, GetSuggestionLifecycle, audit-derived lifecycle projection — Sprint 22
- [x] Suggestion intent bridge foundation: SuggestionIntentService, CreateSuggestionIntentRequest, approval-gated intent request — Sprint 23
- [x] Governed suggestion execution boundary: ExecuteIntentRequest, GovernedIntentExecutionService, pipeline-dispatched execution — Sprint 24
- [x] Execution outcome foundation: ExecutionOutcomeService, GetExecutionOutcomes, audit-derived outcome projection — Sprint 25
- [x] Execution Context Foundation — Sprint 26: ExecutionContextService, WorkspaceContext.execution_context enrichment
- [x] Execution Idempotency Foundation — Sprint 27: ExecutionGuardService, audit-derived duplicate dispatch protection
- [x] Execution Cancellation Foundation — Sprint 28: ExecutionCancellationService, RequestExecutionCancellation (request boundary only)
- [x] Execution State Reconciliation Foundation — Sprint 29: ExecutionReconciliationService, GetExecutionState
- [x] Execution States Projection Foundation — Sprint 30: reconcile_execution_states, GetExecutionStates
- [x] Execution State Context Enrichment — Sprint 31: WorkspaceContext.execution_states via ExecutionReconciliationService
- [x] Execution IPC + Operator Console — Sprint 32: wire S23–S30 IPC; front-to-back suggestion→execute path in app shell
- [x] Spatial Workspace Canvas shell prototype — Sprint 33: CanvasShell, zone drag, pan/zoom (DEC-009)
- [x] SQLite schema v1 (layouts, preferences) — verified existing Sprint 02/13 schema (no redesign)
- [x] Layout save/restore — Sprint 34: ensureLayout + update_layout on canvas interaction
- [x] Windows Integration Layer (basic window enumeration) — Sprint 35–36: windows-integration crate + GetDesktopWindows IPC
- [x] Phase 1 harden: CI cargo test + ROADMAP/DEC-011 honesty — Sprint 37
- [x] Phase 1 harden: canvas resize + canvas-first create + Operator diagnostic label — Sprint 38
- [x] Foundation hardening: seal service mutators, ActionIntentValidationService rename, IPC inventory — Sprint 39
- [x] CI/CD pipeline (lint, build, test) — Sprint 01 PR workflow; Sprint 37 adds `cargo test --workspace`
- [x] Developer setup documentation — Sprint 01
- [x] Platform kernel (event bus, state management, Permission Gateway) — seam Sprint 06; gateway Sprint 40

**Phase 1 note:** Application launch is Phase 2 entry (ROADMAP), not a missed Phase 1 checkbox, even though MVP §2.2 lists it under the overall MVP.
### Gate Criteria (Phase 1 → Phase 2)

| Criterion | Detail |
|-----------|--------|
| Shell renders Spatial Workspace Canvas | Zones, draggable elements working |
| Layouts persist between sessions | SQLite save/restore verified |
| CI/CD operational | All PRs pass automated checks |
| Architecture validated | Multi-process boundaries implemented |
| MVP Phase 1 scope complete | Per [MVP Definition](../01-Product/MVP-DEFINITION.md) v0.1 shell items |

### Out of Scope for Phase 1

- AI features (Phase 2)
- Plugin processes (Phase 3)
- Device integration
- Audio management
- Automation engine
- Production visual design

---

## 5. Phase 2 — Feature Expansion

**Goal:** Application launching, AI observation/suggestion, and user-approved automation.

**Status:** **In progress** — governed AI assistant (Sprints 58–59). Intelligence batches 1–7 complete; see [Intelligence Roadmap](../05-AI/INTELLIGENCE-ROADMAP.md).

### Expected Deliverables

- [x] Permission Gateway (replace AllowAll on execution path; decision audit) — Sprint 40
- [x] Governed application launch through Permission Gateway — Sprints 41–42
- [x] Permission approval foundation (pending → allow once / deny → grant) — Sprints 43–44
- [x] Permission architecture seal (no bypass, docs, boundary tests) — Sprint 45; see [Permission Architecture](../07-Security/PERMISSION-ARCHITECTURE.md)
- [x] AI actor foundation (propose → pipeline → gateway; zero default authority) — Sprints 46–47; see [AI Actor Foundation](../05-AI/AI-ACTOR-FOUNDATION.md)
- [x] AI planning foundation (goals → proposals → governed requests) — Sprints 48–49; see [AI Planning Foundation](../05-AI/AI-PLANNING-FOUNDATION.md)
- [x] AI context foundation (read-only awareness → better proposals) — Sprints 50–51; see [AI Context Foundation](../05-AI/AI-CONTEXT-FOUNDATION.md)
- [x] AI action catalog (existence ≠ authorization) — Sprints 52–53; see [AI Action Catalog](../05-AI/AI-ACTION-CATALOG.md)
- [x] AI planning evaluation (quality/outcome measurement only) — Sprints 54–55; see [AI Evaluation Foundation](../05-AI/AI-EVALUATION-FOUNDATION.md)
- [x] Governed AI orchestration (multi-step plans, per-step gateway) — Sprints 56–57; see [AI Orchestration Foundation](../05-AI/AI-ORCHESTRATION-FOUNDATION.md)
- [x] Governed AI assistant (goal → preview → confirm → gateway) — Sprints 58–59; see [AI Assistant Foundation](../05-AI/AI-ASSISTANT-FOUNDATION.md)
- Application service (discovery, launch, grouping) — launch authority done; discovery/grouping later
- Window service (tracking, layout integration)
- AI observer and pattern store (L0–L4 confidence model)
- AI suggestion UI (permission-gated)
- Basic automation service (user-approved app launch sequences)
- Integration tests for cross-domain flows

### Gate Criteria (Phase 2 → Phase 3)

| Criterion | Detail |
|-----------|--------|
| MVP v0.1 complete | All [MVP Definition](../01-Product/MVP-DEFINITION.md) acceptance criteria met |
| AI observes and suggests | Permission model working end-to-end |
| At least one automation type works | User can approve and run an automation |
| Security review complete | Threat model validated against implementation |

---

## 6. Phase 3 — Maturity

**Goal:** Plugin platform, advanced features, polish, and public release preparation.

**Status:** Not started

### Expected Deliverables

- Plugin SDK and isolated plugin processes
- Plugin registry (or GitHub-based distribution)
- Advanced automation (cross-domain, conditional)
- Phone/device deep integration
- Performance optimisation pass
- Accessibility audit and fixes
- Public release preparation (installer, updates, documentation)

---

## 7. Future Considerations (Post-Release)

- Cloud sync (opt-in) — OQ-005
- Multi-monitor advanced layouts
- Deeper Windows integration (DEC-008 future expansion)
- macOS / Linux ports
- Mobile companion app
- Plugin marketplace

---

## Related Documents

- [Decision Log](../09-Decisions/DECISION-LOG.md)
- [MVP Definition](../01-Product/MVP-DEFINITION.md)
- [Scope Management](../01-Product/SCOPE-MANAGEMENT.md)
- [Sprint Structure](../10-Sprints/SPRINT-STRUCTURE.md)
- [Open Questions](../09-Decisions/OPEN-QUESTIONS.md)
