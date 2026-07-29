# Workspace Documentation

> The single source of truth for all project knowledge, governance, and decisions.

---

## Quick Start

| A new contributor | [Project Constitution](00-Constitution/PROJECT-CONSTITUTION.md) → [Engineering Governance](03-Engineering/ENGINEERING-GOVERNANCE.md) → [Visual Direction](01-Product/WORKSPACE-VISUAL-DIRECTION.md) |
| Making a product decision | [Product Vision](01-Product/PRODUCT-VISION.md) → [Visual Direction](01-Product/WORKSPACE-VISUAL-DIRECTION.md) → [Realignment Audit](01-Product/PRODUCT-VISION-REALIGNMENT-AUDIT.md) |
| Starting a major batch | [Batch Alignment Check](03-Engineering/BATCH-ALIGNMENT-CHECK.md) → [Engineering Governance](03-Engineering/ENGINEERING-GOVERNANCE.md) |
| Human visual review | [Human Review Policy](03-Engineering/HUMAN-REVIEW-POLICY.md) → [Visual Review Checklist](03-Engineering/VISUAL-REVIEW-CHECKLIST.md) |
| Designing architecture | [Architecture Principles](02-Architecture/ARCHITECTURE-PRINCIPLES.md) → [System Overview](02-Architecture/SYSTEM-OVERVIEW.md) |
| Desktop arrangement (DAF) | [DAF Architecture Audit](03-Engineering/DAF-ARCHITECTURE-AUDIT.md) → [DAF-1a](03-Engineering/DAF-1A-WINDOW-CONTROLLER.md) → [DAF-1b](03-Engineering/DAF-1B-WINDOW-OBSERVATION.md) |
| Selecting technology | [Stack Evaluation Criteria](02-Architecture/STACK-EVALUATION-CRITERIA.md) |
| Writing code | [Coding Standards](03-Engineering/CODING-STANDARDS.md) → [Definition of Done](03-Engineering/DEFINITION-OF-DONE.md) |
| Working on AI features | Confirm need against [Engineering Governance](03-Engineering/ENGINEERING-GOVERNANCE.md) (AI expansion frozen unless product-required) → [AI Principles](05-AI/AI-PRINCIPLES.md) |
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
| [Workspace Visual Direction](01-Product/WORKSPACE-VISUAL-DIRECTION.md) | Concept-art north star; literal vs non-literal interpretation |
| [Product Vision Realignment Audit](01-Product/PRODUCT-VISION-REALIGNMENT-AUDIT.md) | Gap analysis vs concept art; DAF milestone; freeze AI expansion |
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
| [Engineering Governance](03-Engineering/ENGINEERING-GOVERNANCE.md) | Product alignment, freeze rules, batch docs, drift prevention |
| [Human Review Policy](03-Engineering/HUMAN-REVIEW-POLICY.md) | When human visual review is required; batching; media restraint |
| [Visual Review Checklist](03-Engineering/VISUAL-REVIEW-CHECKLIST.md) | Checkpoint template for batched UI audits |
| [Batch Alignment Check](03-Engineering/BATCH-ALIGNMENT-CHECK.md) | Pre-implementation checklist template |
| [DAF Architecture Audit](03-Engineering/DAF-ARCHITECTURE-AUDIT.md) | DAF-0 archaeology + proposed desktop arrangement architecture |
| [DAF-0 Completion Report](03-Engineering/DAF-0-COMPLETION-REPORT.md) | Governance reset batch report |
| [DAF-1a WindowController](03-Engineering/DAF-1A-WINDOW-CONTROLLER.md) | OS window mutation boundary |
| [DAF-1a Completion Report](03-Engineering/DAF-1A-COMPLETION-REPORT.md) | WindowController foundation batch report |
| [DAF-1b Window Observation](03-Engineering/DAF-1B-WINDOW-OBSERVATION.md) | Observation & identity foundation |
| [DAF-1b Completion Report](03-Engineering/DAF-1B-COMPLETION-REPORT.md) | Observation foundation batch report |
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
| [Programme IV Maintainability Audit](03-Engineering/PROGRAMME-IV-MAINTAINABILITY-AUDIT.md) | Pre–Batch 10 human maintainability & integrity gate |
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
| [Programme IV — Interaction Runtime](05-AI/PROGRAMME-IV-INTERACTION-RUNTIME.md) | Multi-batch coherent runtime programme charter |
| [Unified Workspace State Architecture](05-AI/UNIFIED-WORKSPACE-STATE-ARCHITECTURE.md) | Programme III Batch 1 composition envelope |
| [Policy & Governance Architecture](05-AI/POLICY-GOVERNANCE-ARCHITECTURE.md) | Programme III Batch 2 policy reasoning / evaluation evidence |
| [Historical Workspace Reconstruction Architecture](05-AI/HISTORICAL-WORKSPACE-RECONSTRUCTION-ARCHITECTURE.md) | Programme III Batch 3 temporal reconstruction / change explanation |
| [Temporal Intelligence Architecture](05-AI/TEMPORAL-INTELLIGENCE-ARCHITECTURE.md) | Programme III Batch 4 temporal analysis / historical understanding extensions |
| [Workspace Explanation Layer Architecture](05-AI/WORKSPACE-EXPLANATION-LAYER-ARCHITECTURE.md) | Programme III Batch 5 cross-surface evidence-backed explanation |
| [Contextual Workspace Understanding Architecture](05-AI/CONTEXTUAL-WORKSPACE-UNDERSTANDING-ARCHITECTURE.md) | Programme III Batch 6 — situational understanding (Active / accepted) |
| [Knowledge Synthesis Architecture](05-AI/KNOWLEDGE-SYNTHESIS-ARCHITECTURE.md) | Programme III Batch 7 — structured knowledge artefacts (Active / accepted) |
| [Knowledge Integration Architecture](05-AI/KNOWLEDGE-INTEGRATION-ARCHITECTURE.md) | Programme III Batch 8 — retrieval / integration over evidence layers (Active / accepted) |
| [Insight Coordination Architecture](05-AI/INSIGHT-COORDINATION-ARCHITECTURE.md) | Programme III Batch 9 — cross-layer insight coordination (Active / accepted) |
| [Cross-Workspace Intelligence Architecture](05-AI/CROSS-WORKSPACE-INTELLIGENCE-ARCHITECTURE.md) | Programme III Batch 10 — cross-workspace aggregate observations (Active / accepted) |
| [Workspace Decision Support Architecture](05-AI/WORKSPACE-DECISION-SUPPORT-ARCHITECTURE.md) | Programme III Batch 11 — decision-ready evidence packages without deciding (Active / accepted) |
| [Workspace Evidence Reliability Architecture](05-AI/WORKSPACE-EVIDENCE-RELIABILITY-ARCHITECTURE.md) | Programme IV Batch 9 — reliability observations without truth or trust authority (Active / accepted) |
| [Workspace Evidence Observational Scaffold Architecture](05-AI/WORKSPACE-EVIDENCE-OBSERVATIONAL-SCAFFOLD-ARCHITECTURE.md) | Programme IV Batch 10 — shared helpers without a new evidence engine |
| [Conversational / Assistant Surface Architecture](05-AI/CONVERSATIONAL-ASSISTANT-SURFACE-ARCHITECTURE.md) | Programme IV Batch 11 — conversational presentation over recorded evidence (Active / implemented) |
| [Assistant Context Intelligence Architecture](05-AI/ASSISTANT-CONTEXT-INTELLIGENCE-ARCHITECTURE.md) | Programme IV Batch 12 — context selection / continuity packaging (Active / implemented) |
| [Assistant Retrieval Intelligence Architecture](05-AI/ASSISTANT-RETRIEVAL-INTELLIGENCE-ARCHITECTURE.md) | Programme IV Batch 13 — retrieval packaging / evidence presentation (implemented) |
| [Assistant Explanation Intelligence Architecture](05-AI/ASSISTANT-EXPLANATION-INTELLIGENCE-ARCHITECTURE.md) | Programme IV Batch 14 — explanation packaging / citation clarity (Active / implemented) |
| [Assistant Interaction Intelligence Architecture](05-AI/ASSISTANT-INTERACTION-INTELLIGENCE-ARCHITECTURE.md) | Programme IV Batch 15 — conversation flow packaging / response routing (Active / implemented) |
| [Assistant Personalisation Boundary Architecture](05-AI/ASSISTANT-PERSONALISATION-BOUNDARY-ARCHITECTURE.md) | Programme IV Batch 16 — explicit presentation preference packaging (charter only) |
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

- [Programme IV — Interaction Runtime](./05-AI/PROGRAMME-IV-INTERACTION-RUNTIME.md)
- [Workspace Semantic Query Architecture](./05-AI/WORKSPACE-SEMANTIC-QUERY-ARCHITECTURE.md)
- [Workspace Evidence Navigation Architecture](./05-AI/WORKSPACE-EVIDENCE-NAVIGATION-ARCHITECTURE.md)
- [Workspace Evidence Trace Architecture](./05-AI/WORKSPACE-EVIDENCE-TRACE-ARCHITECTURE.md)
- [Workspace Evidence Coverage Architecture](./05-AI/WORKSPACE-EVIDENCE-COVERAGE-ARCHITECTURE.md)
- [Workspace Evidence Consistency Architecture](./05-AI/WORKSPACE-EVIDENCE-CONSISTENCY-ARCHITECTURE.md)
- [Workspace Evidence Dependency Architecture](./05-AI/WORKSPACE-EVIDENCE-DEPENDENCY-ARCHITECTURE.md)
- [Workspace Evidence Freshness Architecture](./05-AI/WORKSPACE-EVIDENCE-FRESHNESS-ARCHITECTURE.md)
- [Workspace Evidence Completeness Architecture](./05-AI/WORKSPACE-EVIDENCE-COMPLETENESS-ARCHITECTURE.md)
