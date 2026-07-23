# Architecture Principles

| Field | Value |
|-------|-------|
| **Purpose** | Define the architectural values and constraints that guide all system design decisions |
| **Owner** | Architect (TBD) |
| **Dependencies** | [Project Constitution](../00-Constitution/PROJECT-CONSTITUTION.md), [Product Vision](../01-Product/PRODUCT-VISION.md) |
| **Update Process** | Architect proposes changes. Material changes require Decision Log entry and engineering review. |

---

## 1. Architectural Mission

Design a system that can grow beyond 100,000 lines of code while remaining understandable, testable, and modifiable. Workspace architecture must support six integrated domains (applications, windows, devices, audio, automation, AI) without creating a monolith that is impossible to evolve.

---

## 2. Core Principles

### 2.1 Layer Over Replace

Workspace is a layer on Windows, not a replacement. Architecture must:

- Integrate with Windows APIs and conventions
- Avoid reimplementing OS-level capabilities without clear justification
- Degrade gracefully when Windows features are unavailable

### 2.2 Modularity Over Monolith

- Separate concerns into bounded modules with explicit interfaces
- Domains (apps, windows, devices, audio, automation, AI) must be independently developable
- Shared kernel provides cross-cutting services only — not domain logic

### 2.3 Explicit Boundaries

- Every module declares its public interface
- Internal implementation details are not shared across module boundaries
- Cross-module communication uses defined contracts, not direct coupling

### 2.4 Local-First Preference

User data, layouts, workflows, and preferences should default to local storage. Cloud sync — if adopted — must be opt-in and documented. Final decision pending; see [Open Questions](../09-Decisions/OPEN-QUESTIONS.md).

### 2.5 Event-Driven Coordination

Domains coordinate through events and observable state, not direct calls where possible. This supports:

- Plugin extensibility
- AI observation without invasive instrumentation
- Automation triggers based on system events
- Loose coupling between domains

### 2.6 Permission-Gated Actions

Any action that changes user state (automation, layout changes, audio routing, app launching) must pass through a permission or approval layer. Architecture must make unauthorised action paths structurally difficult.

### 2.7 Testability by Design

- Business logic must be testable without UI
- External dependencies (OS APIs, devices, audio hardware) must be abstracted behind interfaces
- Architecture must support unit, integration, and end-to-end test layers

### 2.8 Performance as Architecture

- Lazy loading for non-critical subsystems
- Avoid blocking the UI thread for I/O or computation
- Define performance budgets per subsystem before implementation

### 2.9 Security by Design

- Least privilege for all components and plugins
- Secrets never in source code
- See [Security Principles](../07-Security/SECURITY-PRINCIPLES.md)

### 2.10 Document Before Implement

No production architecture (module structure, API contracts, data models) is implemented without a corresponding architecture document or Decision Log entry.

---

## 3. Architectural Constraints

| Constraint | Rationale |
|------------|-----------|
| Windows-first | Primary platform; others are future consideration |
| Desktop-native | Not a web app pretending to be desktop — technology choice TBD |
| Extensible via plugins | Third-party and first-party features share extension model |
| AI is a subsystem, not the product | AI supports the workspace; it does not define it |
| No silent side effects | All state-changing operations are auditable |

---

## 4. System Layers (Conceptual)

These layers describe intent, not implementation. Technology mapping is a future decision.

```
┌──────────────────────────────────────────────┐
│  Presentation (Shell, Panels, Navigation)    │
├──────────────────────────────────────────────┤
│  Domain Services                             │
│  (Apps, Windows, Devices, Audio, Automation) │
├──────────────────────────────────────────────┤
│  AI Subsystem (Observe, Learn, Suggest)      │
├──────────────────────────────────────────────┤
│  Plugin Runtime                              │
├──────────────────────────────────────────────┤
│  Platform Kernel                             │
│  (Events, State, Permission Gateway, Config) │
├──────────────────────────────────────────────┤
│  Windows Integration Layer                   │
└──────────────────────────────────────────────┘
```

Higher layers depend on lower layers. The Platform Kernel provides shared infrastructure (event bus, state management, Permission Gateway, configuration, logging) to all layers above it. The Permission Gateway is owned by the Platform Kernel — not the AI Subsystem. See [System Overview](SYSTEM-OVERVIEW.md) and [Event and API Standards](EVENT-AND-API-STANDARDS.md).

Layer responsibilities and boundaries will be detailed in future architecture documents as decisions are made.

---

## 5. Anti-Patterns

The following are prohibited unless explicitly approved via Decision Log:

| Anti-Pattern | Why |
|--------------|-----|
| God module | Unmaintainable at scale |
| Shared mutable global state | Race conditions, untestable |
| Direct OS calls from UI layer | Untestable, platform-locked |
| Feature flags without documentation | Hidden behaviour |
| Circular module dependencies | Prevents independent development |
| AI bypassing permission layer | Violates constitution |
| Plugin full system access | Security risk |

---

## 6. Architecture Decision Process

1. Identify need (feature, scale, integration)
2. Write proposal with options and trade-offs
3. Review against these principles
4. Record decision in [Decision Log](../09-Decisions/DECISION-LOG.md)
5. Create or update detailed architecture document
6. Implement only after document approval

---

## Related Documents

- [System Overview](SYSTEM-OVERVIEW.md)
- [Event and API Standards](EVENT-AND-API-STANDARDS.md)
- [Repository Structure](REPOSITORY-STRUCTURE.md)
- [Plugin Architecture Vision](../06-Plugins/PLUGIN-ARCHITECTURE-VISION.md)
- [AI Principles](../05-AI/AI-PRINCIPLES.md)
- [Security Principles](../07-Security/SECURITY-PRINCIPLES.md)
