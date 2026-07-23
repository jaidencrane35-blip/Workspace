# System Overview

| Field | Value |
|-------|-------|
| **Purpose** | Provide a high-level conceptual map of the Workspace system and its major subsystems |
| **Owner** | Project Owner |
| **Dependencies** | [Architecture Principles](ARCHITECTURE-PRINCIPLES.md), [Product Vision](../01-Product/PRODUCT-VISION.md) |
| **Update Process** | Update when major subsystems are defined or boundaries change. Requires Decision Log entry for structural changes. |

---

## 1. System Context

Workspace operates as an adaptive layer above Windows, presenting a unified shell experience while delegating OS-level operations to Windows APIs.

```
┌──────────── User ────────────┐
│                              │
│  ┌────────────────────────┐  │
│  │   Workspace Shell      │  │
│  │  (Navigation, Panels,  │  │
│  │   Layouts, Modes)      │  │
│  └───────────┬────────────┘  │
│              │               │
│  ┌───────────▼────────────┐  │
│  │   Domain Services      │  │
│  │  Apps │ Windows │ Dev  │  │
│  │  Audio │ Automation   │  │
│  └───────────┬────────────┘  │
│              │               │
│  ┌───────────▼────────────┐  │
│  │   AI Subsystem         │  │
│  │  Observe → Learn →     │  │
│  │  Suggest               │  │
│  └───────────┬────────────┘  │
│              │               │
│  ┌───────────▼────────────┐  │
│  │   Plugin Runtime       │  │
│  └───────────┬────────────┘  │
│              │               │
│  ┌───────────▼────────────┐  │
│  │   Platform Kernel      │  │
│  │  Events │ State │      │  │
│  │  Permission Gateway │  │  │
│  │  Config │ Logging     │  │
│  └───────────┬────────────┘  │
│              │               │
│  ┌───────────▼────────────┐  │
│  │   Windows Integration  │  │
│  │   Layer                │  │
│  └───────────┬────────────┘  │
│                              │
└──────────────────────────────┘
         PC / Phone / Audio Devices
```

This diagram matches the layer model in [Architecture Principles](ARCHITECTURE-PRINCIPLES.md) §4.

---

## 2. Technology Stack (DEC-007)

| Layer | Technology |
|-------|------------|
| UI | React + TypeScript (Tauri webview) |
| Desktop runtime | Tauri |
| Core services | Rust |
| Data persistence | SQLite (+ JSON export) |
| Monorepo (JS/TS) | pnpm workspaces (DEC-012) |
| Monorepo (Rust) | Cargo workspace |

### Target Application Structure

```
Workspace Application
    Tauri Shell
        React + TypeScript Interface
        Rust Core Runtime
            SQLite Database
            AI Subsystem
            Plugin Runtime
            Windows Integration Services
```

---

## 3. Process Architecture (DEC-011)

```
Frontend Process (Tauri WebView — React UI)
        │
        │ IPC
        ▼
Workspace Core Process (Rust)
        │
        ├── Plugin Processes (isolated)
        └── AI Worker Processes (isolated)
```

Inter-process communication follows [Event and API Standards](EVENT-AND-API-STANDARDS.md).

---

## 4. Windows Integration (DEC-008)

Hybrid Companion + Overlay model. Workspace provides:

- Companion application (primary shell)
- Overlay interface (spatial workspace canvas)
- Window management integration
- Automation services
- AI assistance layer

See [Windows Integration Model](WINDOWS-INTEGRATION-MODEL.md).

---

## 5. Layout Model (DEC-009)

Spatial Workspace Canvas — not a fixed grid.

```
Workspace
    Zones
        Applications
        Widgets
        AI Suggestions
        Automation Blocks
```

Navigation and core controls remain consistent. Workspace content is user-customizable.

---

## 6. Major Subsystems

### 6.1 Shell

The user-facing workspace environment.

**Responsibilities:**
- Navigation and panel management
- Layout persistence and restoration
- Mode presets
- Visual presentation of domain state

**Does not:**
- Contain domain business logic
- Directly manipulate OS resources

### 6.2 Domain Services

Independent service modules for each product domain.

| Service | Responsibility |
|---------|---------------|
| Application Service | App discovery, launch, grouping, context |
| Window Service | Window tracking, layout integration, workspace-aware management |
| Device Service | Connected device discovery and integration |
| Audio Service | Audio routing, levels, context switching |
| Automation Service | User-approved workflow execution |

Each service exposes a public API. Internal state is private.

### 6.3 AI Subsystem

Observes system events and user patterns. Produces suggestions. Never acts without permission.

**Components (conceptual):**
- Event observer (subscribes to Platform Kernel event bus)
- Pattern store
- Suggestion engine
- Automation proposer

**Does not own:**
- Permission Gateway (owned by Platform Kernel)
- Direct domain service access (must route through Permission Gateway)

See [AI Principles](../05-AI/AI-PRINCIPLES.md) and [AI Operating Model](../05-AI/AI-OPERATING-MODEL.md).

### 6.4 Plugin Runtime

Executes third-party and first-party extensions in a sandboxed, permission-controlled environment. All plugin actions route through the Permission Gateway.

See [Plugin Architecture Vision](../06-Plugins/PLUGIN-ARCHITECTURE-VISION.md).

### 6.5 Platform Kernel

Shared infrastructure for all subsystems. Sits above the Windows Integration Layer and below all feature layers.

**Capabilities:**
- Event bus
- State management
- **Command pipeline** (mutations: policy → permission → execute → audit)
- **Resource services** (Sprint 12) — Workspace, Zone, Application, Widget
- **Passive workspace graph** (Sprint 12) — `GraphService` registers nodes/edges; services never touch graph tables directly
- **Permission Gateway** (see §6.7)
- Configuration store
- Logging, diagnostics, and audit

**Resource orchestration flow (implemented Sprint 12):**

```
IPC / CommandHandler
    → CommandPipeline (policy, permission, audit)
        → Resource Service (persistence + GraphService registration)
            → Repository (SQLite)
            → GraphService (passive nodes/edges)
```

Cross-resource operations (e.g. validating workspace existence before creating a zone) occur in **commands**, not services.

**Layout layer (Sprint 13):**

```
Graph (topology — what exists)
    ↓ referenced by ResourceRef only
Layout (presentation — where resources are positioned)
    ↓ future
Canvas (rendering — how it is displayed)
```

Layout never stores graph edges or mutates graph state. Layout nodes reference `ResourceRef`; commands verify graph node existence before persisting layout updates.

**Projection layer (Sprint 14):**

```
Authoritative domains (Workspace, Zone, Application, Widget, Layout, Graph)
    ↓
WorkspaceProjectionService (derived read model)
    ↓
WorkspaceSnapshot → future consumers (Canvas, AI, plugins, search)
```

Projections are read-only, non-authoritative, and not persisted. `ResourceRef` remains canonical identity throughout.

**Action intent layer (Sprint 15):**

```
ActionIntentRequest (what to do)
    ↓
IntentExecutionService (validate metadata + capability match)
    ↓
CommandPipeline → existing Command
```

Motivation intent (`IntentContext` — why) and action intent (`ActionIntentRequest` — what) coexist. Commands remain the mutation mechanism.

**Capability discovery layer (Sprint 16):**

```
ActorContext
    ↓
CapabilityResolver (policy + gate evaluation)
    ↓
CapabilityDiscovery (derived, non-authoritative)
```

Discovery exposes what an actor can do; it does not grant permission. Policy and CommandPipeline remain authoritative.

**Observation layer (Sprint 17):**

```
Domain Events (authoritative, ephemeral)
    ↓ AuditEventSubscriber → audit trail (durable history)
    ↓
ObservationService (derive on read)
    ↓
Observation (derived, disposable, neutral) → GetObservations (governed)
```

Observations answer "what is happening over time?" as a read-only interpretation of the recorded audit trail. Domain events remain authoritative; observations add no persistence, no new subscriber, and no interpretation beyond a static category/importance mapping. Reads are governed (`audit.read`) because they reveal activity history.

**Observation analytics layer (Sprint 18):**

```
Observation
    ↓
WorkspaceAnalyticsService (deterministic aggregation)
    ↓
WorkspaceMetrics (derived, read-only) → GetWorkspaceMetrics (governed)
```

Analytics is the "Learn" stage: it answers "what patterns exist in what happened?" by deterministically counting and grouping observations. No AI, scoring, prediction, ranking, or recommendations. It adds no persistence and no mutation path; it consumes `ObservationService` output and aggregates via a pure domain type. Reads are governed (`audit.read`).

**Workspace context layer (Sprint 19):**

```
Projection (state) + Observations (activity) + Metrics (learn) + Capabilities (authority)
    ↓
WorkspaceContextService (deterministic composition)
    ↓
WorkspaceContext (derived, read-only) → GetWorkspaceContext (governed)
```

The context boundary assembles the existing derived read layers into a single read-only structure — the "Context" stage before "Suggest". It is a deterministic composition only: no AI, suggestions, memory, embeddings, or persistence. It orchestrates existing services, preserves `ResourceRef` identity, and reads are governed (`audit.read`). Sprint 26 additively includes `execution_context: ExecutionContextSummary` so trusted consumers can see recent execution outcome history without a separate surface.

**Suggestion layer (Sprint 20):**

```
WorkspaceContext (Sprint 19)
    ↓
derive_suggestions(context)  ← pure, threshold-based rules (no AI)
    ↓
SuggestionService::list → GetSuggestions (governed) → get_suggestions (IPC)
```

The suggestion layer is the "Suggest" stage: it derives deterministic **proposals** on demand from `WorkspaceContext` using simple threshold rules (e.g. `resource_creation_count >= 3`). A `Suggestion` is a proposal object, **never an action** — it does not execute, mutate state, grant permissions, or bypass governance. "Confidence" is deterministic metadata (a supporting count + rule basis), never a probabilistic or learned score. No AI, LLM, ML ranking, behaviour prediction, automation, or suggestion persistence. Suggestions start `Pending`. Reads are governed (`audit.read`).

**Suggestion approval layer (Sprint 21):**

```
Suggestion (Pending)
    ↓
AcceptSuggestion / RejectSuggestion  ← MutationCommand (audit.write)
    ↓
Intent → Capability → PermissionGate → Audit (decision metadata)
    ↓
GetSuggestions suppresses decided ids (audit-derived; no suggestion store)
```

The approval layer is the "Receive Permission" decision stage: explicit Accept/Reject through the existing command pipeline. Decisions are durable in the audit trail and do **not** execute Automate-stage workspace mutations. Automate remains a future stage (proposed actions bound to intents). Approval-type UX and the real Permission Gateway remain deferred.

The preserved future flow is Context → Suggestion → User Approval → Intent → Capability → Permission → CommandPipeline → Execution.

**Suggestion lifecycle layer (Sprint 22):**

```
Audit Trail (Accept/Reject decisions + metadata)
    ↓
classify_suggestion_lifecycle_event (pure)
    ↓
SuggestionLifecycleService::list_recent → GetSuggestionLifecycle (governed) → get_suggestion_lifecycle (IPC)
```

The lifecycle layer adds read-only visibility into suggestion governance history. It derives `SuggestionLifecycleRecord` entries from audit events without a suggestion store, inference, or automation. Reads are governed (`audit.read`).

**Suggestion intent bridge (Sprint 23):**

```
Accepted Suggestion (audit)
    ↓
map_suggestion_type_to_intent (static)
    ↓
SuggestionIntentService → IntentExecutionService::validate_request
    ↓
CreateSuggestionIntentRequest (governed, audit.write)
    ↓
SuggestionIntentRequest
    ↓
[Future] governed command execution
```

The bridge creates a validated intent request from an accepted suggestion. It does not dispatch commands, automate actions, or bypass the permission pipeline.

**Governed execution boundary (Sprint 24):**

```
SuggestionIntentRequest (audit bridge)
    ↓
ExecuteIntentRequest (governed, audit.write)
    ↓
GovernedIntentExecutionService::prepare
    ↓
CommandPipeline::execute_*_with_action
    ↓
Existing mapped command → Audit
```

Execution remains explicit and governed. This is not automation — each execution request is a separate mutation through the full authority pipeline. Future automation systems remain a separate stage.

**Execution outcome layer (Sprint 25):**

```
ExecuteIntentRequest audit (success / failure)
    ↓
classify_execution_outcome_event (pure)
    ↓
ExecutionOutcomeService::list_recent → GetExecutionOutcomes (governed, audit.read)
```

Outcomes answer whether governed execution completed, failed, or was cancelled. Derived only — no outcome store, no AI feedback loops, no automatic retries. Audit remains authoritative.

**Execution outcome context layer (Sprint 26):**

```
ExecutionOutcomeService (Sprint 25)
    ↓
ExecutionContextService → ExecutionContextSummary (counts + recent commands)
    ↓
WorkspaceContext.execution_context (additive enrichment)
```

Execution context enrichment answers "what happened after previous executions?" as a deterministic, read-only summary inside `WorkspaceContext`. No persistence, no new command/capability, no IPC, no AI memory or ranking. Audit remains the durable history; outcomes remain historical facts.

### 6.6 Windows Integration Layer

Abstracts all Windows API interactions. Only this layer communicates directly with the OS.

### 6.7 Permission Gateway

**Owner: Platform Kernel**

The Permission Gateway is the single enforcement point for all state-changing operations in Workspace.

| Responsibility | Detail |
|----------------|--------|
| Validate permission requests | From AI, plugins, shell, and automation service |
| Present approval UI | User-facing permission prompts |
| Issue permission tokens | Scoped, time-limited authorisation |
| Enforce policy | Block unapproved actions |
| Audit logging | Record all permission requests and outcomes |

**Request flow:**
```
Requester (AI / Plugin / Shell / Automation)
    → Permission Gateway (Platform Kernel)
    → User Prompt (if required)
    → Domain Service (if approved)
```

No subsystem bypasses the Permission Gateway. AI Subsystem submits requests; it does not enforce permissions itself.

See [Event and API Standards](EVENT-AND-API-STANDARDS.md) §7 and [AI Operating Model](../05-AI/AI-OPERATING-MODEL.md) §9.

---

## 7. Data Flow (Conceptual)

### User Action Flow

```
User Input → Shell → Domain Service → Platform Kernel → Windows Integration
                ↓
           Event Bus → AI Observer (passive)
```

### AI Suggestion Flow

```
AI Observer → Pattern Store → Suggestion Engine → Permission Gateway
                                                        ↓
                                                  User Prompt
                                                        ↓
                                                  User Approval
                                                        ↓
                                              Automation Service
```

### Plugin Flow

```
Plugin → Plugin Runtime → Permission Gateway → Domain Service API
                              (never direct OS access)
```

---

## 8. State Ownership

| State Type | Owner | Persistence |
|------------|-------|-------------|
| Layouts | Shell + User | SQLite (DEC-010) |
| User preferences | Platform Kernel | SQLite |
| Workflow patterns | AI Subsystem | SQLite |
| Automation definitions | Automation Service | SQLite |
| Plugin configurations | Plugin Runtime | SQLite per plugin |
| Permission grants | Platform Kernel (Permission Gateway) | SQLite |
| Runtime/window state | Domain Services | Session |
| Audit log | Platform Kernel | SQLite |

JSON export supported for backup, migration, and debugging (DEC-010).

---

## 9. Integration Points (External)

| Integration | Purpose | Status |
|-------------|---------|--------|
| Windows Shell / APIs | OS operations | Required |
| Installed applications | Launch, monitor | Required |
| Audio devices / endpoints | Routing, levels | Required |
| Phone / mobile devices | Cross-device workflow | Planned |
| AI models (local or remote) | Suggestion generation | TBD |

Specific protocols and APIs are not yet selected. See [Windows Integration Model](WINDOWS-INTEGRATION-MODEL.md).

---

## 10. Non-Goals (Architectural)

- Replacing the Windows shell entirely
- Building a custom operating system kernel
- Centralised cloud dependency for core functionality
- Real-time multiplayer / collaboration (future consideration only)

---

## 11. Open Architectural Questions

See [Open Questions](../09-Decisions/OPEN-QUESTIONS.md) for remaining items including:

- AI model selection (OQ-004)
- Plugin runtime technology (OQ-007)
- Encryption at rest ([DEC-015](DECISION-LOG.md))
- Cloud sync scope (OQ-005)

---

## Related Documents

- [Architecture Principles](ARCHITECTURE-PRINCIPLES.md)
- [Event and API Standards](EVENT-AND-API-STANDARDS.md)
- [Windows Integration Model](WINDOWS-INTEGRATION-MODEL.md)
- [Repository Structure](REPOSITORY-STRUCTURE.md)
- [Plugin Architecture Vision](../06-Plugins/PLUGIN-ARCHITECTURE-VISION.md)
- [AI Operating Model](../05-AI/AI-OPERATING-MODEL.md)
