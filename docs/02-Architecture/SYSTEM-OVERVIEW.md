# System Overview

| Field | Value |
|-------|-------|
| **Purpose** | Provide a high-level conceptual map of the Workspace system and its major subsystems |
| **Owner** | Architect (TBD) |
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
│  │  Observe → Suggest     │  │
│  └───────────┬────────────┘  │
│              │               │
│  ┌───────────▼────────────┐  │
│  │   Plugin Runtime       │  │
│  └───────────┬────────────┘  │
│              │               │
│  ┌───────────▼────────────┐  │
│  │   Windows Platform     │  │
│  └────────────────────────┘  │
│                              │
└──────────────────────────────┘
         PC / Phone / Audio Devices
```

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
- Event observer
- Pattern store
- Suggestion engine
- Permission gateway
- Automation proposer

See [AI Principles](../05-AI/AI-PRINCIPLES.md).

### 2.4 Platform Kernel

Shared infrastructure for all subsystems.

**Expected capabilities:**
- Event bus
- State management
- Permission and audit layer
- Configuration store
- Logging and diagnostics

### 2.5 Plugin Runtime

Executes third-party and first-party extensions in a sandboxed, permission-controlled environment.

See [Plugin Architecture Vision](../06-Plugins/PLUGIN-ARCHITECTURE-VISION.md).

### 2.6 Windows Integration Layer

Abstracts all Windows API interactions. Only this layer communicates directly with the OS.

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
AI Observer → Pattern Store → Suggestion Engine → User Prompt
                                                      ↓
                                              User Approval
                                                      ↓
                                           Automation Service
```

### Plugin Flow

```
Plugin → Plugin Runtime → Permission Check → Domain Service API
                                              (never direct OS access)
```

---

## 4. State Ownership

| State Type | Owner | Persistence |
|------------|-------|-------------|
| Layouts | Shell + User | Local (format TBD) |
| User preferences | Platform Kernel | Local |
| Workflow patterns | AI Subsystem | Local |
| Automation definitions | Automation Service | Local |
| Plugin configurations | Plugin Runtime | Local per plugin |
| Runtime/window state | Domain Services | Session |

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

Specific protocols and APIs are not yet selected.

---

## 6. Non-Goals (Architectural)

- Replacing the Windows shell entirely
- Building a custom operating system kernel
- Centralised cloud dependency for core functionality
- Real-time multiplayer / collaboration (future consideration only)

---

## 7. Open Architectural Questions

See [Open Questions](../09-Decisions/OPEN-QUESTIONS.md) for unresolved items including:

- Technology stack selection
- Inter-process vs. in-process module architecture
- Data persistence format
- AI model deployment (local vs. hybrid)
- Phone integration protocol

---

## Related Documents

- [Architecture Principles](ARCHITECTURE-PRINCIPLES.md)
- [Repository Structure](REPOSITORY-STRUCTURE.md)
- [Plugin Architecture Vision](../06-Plugins/PLUGIN-ARCHITECTURE-VISION.md)
- [AI Principles](../05-AI/AI-PRINCIPLES.md)
