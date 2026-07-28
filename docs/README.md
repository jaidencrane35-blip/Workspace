# Workspace Documentation

> The single source of truth for all project knowledge, governance, and decisions.

---

## Quick Start

| If you are... | Start here |
|---------------|------------|
| A new contributor | [Project Constitution](00-Constitution/PROJECT-CONSTITUTION.md) → [Engineering Principles](03-Engineering/ENGINEERING-PRINCIPLES.md) |
| Making a product decision | [Product Vision](01-Product/PRODUCT-VISION.md) → [MVP Definition](01-Product/MVP-DEFINITION.md) |
| Designing architecture | [Architecture Principles](02-Architecture/ARCHITECTURE-PRINCIPLES.md) → [System Overview](02-Architecture/SYSTEM-OVERVIEW.md) |
| Selecting technology | [Stack Evaluation Criteria](02-Architecture/STACK-EVALUATION-CRITERIA.md) |
| Writing code (future) | [Coding Standards](03-Engineering/CODING-STANDARDS.md) → [Definition of Done](03-Engineering/DEFINITION-OF-DONE.md) |
| Working on AI features | [AI Principles](05-AI/AI-PRINCIPLES.md) → [AI Operating Model](05-AI/AI-OPERATING-MODEL.md) |
| Reviewing security | [Threat Model](07-Security/THREAT-MODEL.md) → [Security Principles](07-Security/SECURITY-PRINCIPLES.md) |
| Building plugins (future) | [Plugin Architecture Vision](06-Plugins/PLUGIN-ARCHITECTURE-VISION.md) |
| Unsure about something | [Open Questions](09-Decisions/OPEN-QUESTIONS.md) — identify, don't guess |

---

## Documentation Index

### 00 — Constitution

Foundational rules and governance that cannot be overridden.

| Document | Description |
|----------|-------------|
| [Project Constitution](00-Constitution/PROJECT-CONSTITUTION.md) | Non-negotiable project rules and values |
| [Governance Model](00-Constitution/GOVERNANCE.md) | Founder-led decision authority and AI advisory roles |

### 01 — Product

What Workspace is, who it serves, and how scope is managed.

| Document | Description |
|----------|-------------|
| [Product Vision](01-Product/PRODUCT-VISION.md) | Product goals, domains, and target experience |
| [Scope Management](01-Product/SCOPE-MANAGEMENT.md) | How scope is proposed, approved, and controlled |
| [MVP Definition](01-Product/MVP-DEFINITION.md) | First useful product slice and acceptance criteria |

### 02 — Architecture

System design principles and conceptual structure.

| Document | Description |
|----------|-------------|
| [Architecture Principles](02-Architecture/ARCHITECTURE-PRINCIPLES.md) | Architectural values and constraints |
| [System Overview](02-Architecture/SYSTEM-OVERVIEW.md) | High-level subsystem map, Permission Gateway ownership |
| [Repository Structure](02-Architecture/REPOSITORY-STRUCTURE.md) | Recommended repository layout |
| [Stack Evaluation Criteria](02-Architecture/STACK-EVALUATION-CRITERIA.md) | Framework for technology stack decision (OQ-001) |
| [Windows Integration Model](02-Architecture/WINDOWS-INTEGRATION-MODEL.md) | Approaches for Windows coexistence (OQ-014) |
| [Event and API Standards](02-Architecture/EVENT-AND-API-STANDARDS.md) | Event naming, API contracts, versioning rules |
| [Performance Budgets](02-Architecture/PERFORMANCE-BUDGETS.md) | Startup, memory, and responsiveness targets |

### 03 — Engineering

How the team builds, reviews, and maintains the codebase.

| Document | Description |
|----------|-------------|
| [Engineering Principles](03-Engineering/ENGINEERING-PRINCIPLES.md) | Core engineering values and quality gates |
| [Coding Standards](03-Engineering/CODING-STANDARDS.md) | Naming, formatting, and code conventions |
| [Repository Standards](03-Engineering/REPOSITORY-STANDARDS.md) | Git workflow, branches, commits, PRs |
| [Documentation Standards](03-Engineering/DOCUMENTATION-STANDARDS.md) | How to write and maintain documentation |
| [Definition of Done](03-Engineering/DEFINITION-OF-DONE.md) | Completion criteria for all deliverables |
| [Technical Debt Policy](03-Engineering/TECHNICAL-DEBT-POLICY.md) | How debt is tracked and resolved |
| [CI/CD Plan](03-Engineering/CI-CD-PLAN.md) | Future pipeline stages and branch protection |
| [Testing Strategy](03-Engineering/TESTING-STRATEGY.md) | Unit, integration, E2E, and AI behaviour testing |
| [Dependency Policy](03-Engineering/DEPENDENCY-POLICY.md) | Dependency approval, vulnerabilities, lock files |
| [IPC Surface Inventory](03-Engineering/IPC-SURFACE.md) | Used vs quarantined Tauri commands |
| [Kernel Error Taxonomy](03-Engineering/KERNEL-ERROR-TAXONOMY.md) | Failure classes and boundary rules for `KernelError` |
| [Governed Execution Audit Durability](03-Engineering/GOVERNED-AUDIT-DURABILITY.md) | Fail-closed vs best-effort audit persistence for governed commands |
| [Lifecycle Governance](03-Engineering/LIFECYCLE-GOVERNANCE.md) | Explicit transition guards and terminal-state rules |
| [Persistence Boundary Governance](03-Engineering/PERSISTENCE-BOUNDARY-GOVERNANCE.md) | Repository-level lifecycle and immutability enforcement |
| [Projection Integrity](03-Engineering/PROJECTION-INTEGRITY.md) | Dual-channel projections and immutable evidence consumption |
| [Architecture Governance](03-Engineering/ARCHITECTURE-GOVERNANCE.md) | Authority map, forbidden edges, capability audit, failure modes |
| [Operational Recovery](03-Engineering/OPERATIONAL-RECOVERY.md) | Restart/recovery contracts without new lifecycle authorities |
| [Foundation Hardening Report](03-Engineering/FOUNDATION-HARDENING-REPORT.md) | Phase 1 foundation audit results |
| [Audit Reports Index](03-Engineering/AUDIT-REPORTS-INDEX.md) | Index of point-in-time architecture/engineering audits |

### 04 — UX

Interaction design principles and constraints.

| Document | Description |
|----------|-------------|
| [UX Principles](04-UX/UX-PRINCIPLES.md) | User experience values and interaction rules |

### 05 — AI

AI behaviour, permissions, and boundaries.

| Document | Description |
|----------|-------------|
| [AI Principles](05-AI/AI-PRINCIPLES.md) | AI sequence, permission model, and safety rules |
| [AI Operating Model](05-AI/AI-OPERATING-MODEL.md) | Responsibilities, limits, approval boundaries, escalation |
| [Programme II — Cognitive Workspace](05-AI/PROGRAMME-II-COGNITIVE-WORKSPACE.md) | Multi-batch cognitive programme charter and principles |
| [Workspace Cognitive Model](05-AI/WORKSPACE-COGNITIVE-MODEL.md) | Batch 1 durable semantic layer (objectives, focus, uncertainty) |
| [Planning Architecture](05-AI/PLANNING-ARCHITECTURE.md) | Batch 2 non-executing Cognitive Planning Engine |
| [Reasoning Memory Architecture](05-AI/REASONING-MEMORY-ARCHITECTURE.md) | Batch 3 reasoning evidence / reflection layer |
| [Cognitive Graph Architecture](05-AI/COGNITIVE-GRAPH-ARCHITECTURE.md) | Batch 4 cross-domain reference-only topology |
| [Cognitive Orchestration Architecture](05-AI/COGNITIVE-ORCHESTRATION-ARCHITECTURE.md) | Batch 5 non-executing coordination / refresh ordering |
| [Learning & Adaptation Architecture](05-AI/LEARNING-ADAPTATION-ARCHITECTURE.md) | Batch 6 observational learning / suggestion-only adaptation |
| [Cognitive Agent Cast Architecture](05-AI/COGNITIVE-AGENT-CAST-ARCHITECTURE.md) | Batch 7 cognitive roles / perspectives / critiques / syntheses |
| [Cognitive Autonomy Architecture](05-AI/COGNITIVE-AUTONOMY-ARCHITECTURE.md) | Batch 8 governed suggestion / opportunity layer (Programme II final) |
| [Programme III — Coherent Workspace Runtime](05-AI/PROGRAMME-III-COHERENT-WORKSPACE-RUNTIME.md) | Multi-batch coherent runtime programme charter |
| [Unified Workspace State Architecture](05-AI/UNIFIED-WORKSPACE-STATE-ARCHITECTURE.md) | Programme III Batch 1 composition envelope |
| [Policy & Governance Architecture](05-AI/POLICY-GOVERNANCE-ARCHITECTURE.md) | Programme III Batch 2 policy reasoning / evaluation evidence |
| [Historical Workspace Reconstruction Architecture](05-AI/HISTORICAL-WORKSPACE-RECONSTRUCTION-ARCHITECTURE.md) | Programme III Batch 3 temporal reconstruction / change explanation |
| [Temporal Intelligence Architecture](05-AI/TEMPORAL-INTELLIGENCE-ARCHITECTURE.md) | Programme III Batch 4 temporal analysis / historical understanding extensions |
| [Workspace Explanation Layer Architecture](05-AI/WORKSPACE-EXPLANATION-LAYER-ARCHITECTURE.md) | Programme III Batch 5 cross-surface evidence-backed explanation |
| [Contextual Workspace Understanding Architecture](05-AI/CONTEXTUAL-WORKSPACE-UNDERSTANDING-ARCHITECTURE.md) | Programme III Batch 6 — situational understanding (Active) |
| [Memory Policy](05-AI/MEMORY-POLICY.md) | What may be stored, retention, user control, deletion |
| [Confidence Policy](05-AI/CONFIDENCE-POLICY.md) | Confidence levels, suggestion thresholds, uncertainty |
| [AI Architecture Index](05-AI/README.md) | Navigation hub for Recommendation/Decision/Governance contracts |

### 06 — Plugins

Extension platform vision and permission model.

| Document | Description |
|----------|-------------|
| [Plugin Architecture Vision](06-Plugins/PLUGIN-ARCHITECTURE-VISION.md) | Plugin capabilities, sandboxing, and lifecycle |

### 07 — Security

Security principles and risk tracking.

| Document | Description |
|----------|-------------|
| [Security Principles](07-Security/SECURITY-PRINCIPLES.md) | Security values and review requirements |
| [Threat Model](07-Security/THREAT-MODEL.md) | Initial STRIDE analysis by subsystem |
| [Risk Register](07-Security/RISK-REGISTER.md) | Identified risks and mitigation strategies |
| [Permission Architecture](07-Security/PERMISSION-ARCHITECTURE.md) | Permission Gateway boundaries and approval flow |

### 08 — Roadmap

Phased delivery plan and gate criteria.

| Document | Description |
|----------|-------------|
| [Roadmap](08-Roadmap/ROADMAP.md) | Phase 0 through release plan |

### 09 — Decisions

Decision tracking and unresolved questions.

| Document | Description |
|----------|-------------|
| [Decision Log](09-Decisions/DECISION-LOG.md) | Record of all significant decisions |
| [Open Questions](09-Decisions/OPEN-QUESTIONS.md) | Active unresolved decisions requiring review |

### 10 — Sprints

Sprint planning and execution structure.

| Document | Description |
|----------|-------------|
| [Sprint Structure](10-Sprints/SPRINT-STRUCTURE.md) | Sprint ceremonies, templates, and rules |

---

## Document Conventions

Every document in this hierarchy includes:

- **Purpose** — Why the document exists
- **Owner** — Who is responsible for its accuracy
- **Dependencies** — What other documents it relies on
- **Update Process** — How and when it should be updated

See [Documentation Standards](03-Engineering/DOCUMENTATION-STANDARDS.md) for full writing conventions.

---

## Current Phase

**Phase 0 — Foundation:** Complete  
**Phase 0.5 — Decision Recording:** Complete  
**Phase 1 — Core Platform:** Complete  
**Current execution:** Ongoing multi-sprint implementation through Sprint 102

See [Roadmap](08-Roadmap/ROADMAP.md) and [Decision Log](09-Decisions/DECISION-LOG.md) for current decisions and phase status.
