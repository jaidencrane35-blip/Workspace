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

## 2. Major Subsystems

### 2.1 Shell

The user-facing workspace environment.

**Responsibilities:**
- Navigation and panel management
- Layout persistence and restoration
- Mode presets
- Visual presentation of domain state

**Does not:**
- Contain domain business logic
- Directly manipulate OS resources

### 2.2 Domain Services

Independent service modules for each product domain.

| Service | Responsibility |
|---------|---------------|
| Application Service | App discovery, launch, grouping, context |
| Window Service | Window tracking, layout integration, workspace-aware management |
| Device Service | Connected device discovery and integration |
| Audio Service | Audio routing, levels, context switching |
| Automation Service | User-approved workflow execution |

Each service exposes a public API. Internal state is private.

### 2.3 AI Subsystem

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

### 2.4 Plugin Runtime

Executes third-party and first-party extensions in a sandboxed, permission-controlled environment. All plugin actions route through the Permission Gateway.

See [Plugin Architecture Vision](../06-Plugins/PLUGIN-ARCHITECTURE-VISION.md).

### 2.5 Platform Kernel

Shared infrastructure for all subsystems. Sits above the Windows Integration Layer and below all feature layers.

**Capabilities:**
- Event bus
- State management
- **Permission Gateway** (see §2.7)
- Configuration store
- Logging and diagnostics

### 2.6 Windows Integration Layer

Abstracts all Windows API interactions. Only this layer communicates directly with the OS.

### 2.7 Permission Gateway

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

## 3. Data Flow (Conceptual)

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

## 4. State Ownership

| State Type | Owner | Persistence |
|------------|-------|-------------|
| Layouts | Shell + User | Local (format TBD — OQ-003) |
| User preferences | Platform Kernel | Local |
| Workflow patterns | AI Subsystem | Local |
| Automation definitions | Automation Service | Local |
| Plugin configurations | Plugin Runtime | Local per plugin |
| Permission grants | Platform Kernel (Permission Gateway) | Local |
| Runtime/window state | Domain Services | Session |
| Audit log | Platform Kernel | Local |

Persistence formats and storage mechanisms are open decisions.

---

## 5. Integration Points (External)

| Integration | Purpose | Status |
|-------------|---------|--------|
| Windows Shell / APIs | OS operations | Required |
| Installed applications | Launch, monitor | Required |
| Audio devices / endpoints | Routing, levels | Required |
| Phone / mobile devices | Cross-device workflow | Planned |
| AI models (local or remote) | Suggestion generation | TBD |

Specific protocols and APIs are not yet selected. See [Windows Integration Model](WINDOWS-INTEGRATION-MODEL.md).

---

## 6. Non-Goals (Architectural)

- Replacing the Windows shell entirely
- Building a custom operating system kernel
- Centralised cloud dependency for core functionality
- Real-time multiplayer / collaboration (future consideration only)

---

## 7. Open Architectural Questions

See [Open Questions](../09-Decisions/OPEN-QUESTIONS.md) for unresolved items including:

- Technology stack selection (OQ-001)
- Windows integration model (OQ-014)
- Inter-process vs. in-process module architecture (OQ-002)
- Data persistence format (OQ-003)
- AI model deployment (OQ-004)
- Monorepo tooling (OQ-019)

---

## Related Documents

- [Architecture Principles](ARCHITECTURE-PRINCIPLES.md)
- [Event and API Standards](EVENT-AND-API-STANDARDS.md)
- [Windows Integration Model](WINDOWS-INTEGRATION-MODEL.md)
- [Repository Structure](REPOSITORY-STRUCTURE.md)
- [Plugin Architecture Vision](../06-Plugins/PLUGIN-ARCHITECTURE-VISION.md)
- [AI Operating Model](../05-AI/AI-OPERATING-MODEL.md)
