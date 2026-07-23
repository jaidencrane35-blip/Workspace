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
