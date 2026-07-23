# Decision Log

| Field | Value |
|-------|-------|
| **Purpose** | Record all significant project decisions with context, options, rationale, and outcomes |
| **Owner** | Project Lead |
| **Dependencies** | [Governance Model](../00-Constitution/GOVERNANCE.md) |
| **Update Process** | Add entry when a Type A, B, or precedent-setting Type C decision is made. Never delete entries — supersede with new entries. |

---

## 1. How to Use This Log

Each decision entry follows this format:

```
### DEC-NNN: Title
- **Date:** YYYY-MM-DD
- **Status:** Proposed | Accepted | Superseded | Rejected
- **Decision Type:** A (Constitutional) | B (Strategic) | C (Tactical)
- **Owner:** Role
- **Context:** Why this decision was needed
- **Options Considered:** Alternatives evaluated
- **Decision:** What was chosen
- **Rationale:** Why this option was selected
- **Consequences:** Expected impact
- **Related:** Links to issues, PRs, documents
```

---

## 2. Decisions

### DEC-001: Project Name — "Workspace" (Temporary)

- **Date:** 2026-07-23
- **Status:** Accepted (temporary)
- **Decision Type:** B (Strategic)
- **Owner:** Project Lead
- **Context:** Project needs a working name for repository and documentation.
- **Options Considered:** Various names; "Workspace" selected as placeholder.
- **Decision:** Use "Workspace" as the temporary project name.
- **Rationale:** Descriptive of the product concept. Final branding decision deferred.
- **Consequences:** Repository, documentation, and packages use "Workspace" naming. May require rename later.
- **Related:** OQ-011 (final product name)

---

### DEC-002: Documentation-First Foundation

- **Date:** 2026-07-23
- **Status:** Accepted
- **Decision Type:** B (Strategic)
- **Owner:** Lead Software Engineer
- **Context:** Project begins with no implementation. Need to establish governance before code.
- **Options Considered:** (A) Start coding immediately; (B) Document foundation first; (C) Parallel docs and code.
- **Decision:** Complete documentation foundation (Phase 0) before any production code.
- **Rationale:** Aligns with constitution ("Documentation before implementation"). Prevents direction drift. Enables AI contributors to work effectively.
- **Consequences:** Phase 0 delivers docs only. Implementation begins in Phase 1 after stack decision.
- **Related:** [Roadmap](../08-Roadmap/ROADMAP.md) Phase 0

---

### DEC-003: Monorepo Structure

- **Date:** 2026-07-23
- **Status:** Accepted
- **Decision Type:** B (Strategic)
- **Owner:** Lead Software Engineer
- **Context:** Need to decide repository organisation before scaffolding.
- **Options Considered:** (A) Monorepo with packages; (B) Multi-repo; (C) Single flat repo.
- **Decision:** Monorepo with `app/`, `packages/`, `plugins/` structure.
- **Rationale:** Supports modular architecture, shared tooling, atomic cross-module changes, and simpler CI. Appropriate for expected scale (100k+ LOC).
- **Consequences:** Requires monorepo tooling when stack is selected. All packages in one repository.
- **Related:** [Repository Structure](../02-Architecture/REPOSITORY-STRUCTURE.md)

---

### DEC-004: AI Permission Sequence

- **Date:** 2026-07-23
- **Status:** Accepted
- **Decision Type:** A (Constitutional)
- **Owner:** Project Lead
- **Context:** AI behaviour must be governed from the start to maintain user trust.
- **Options Considered:** (A) Mandatory Observe→Learn→Suggest→Permission→Automate; (B) Opt-out automation; (C) Tiered permission levels.
- **Decision:** Mandatory five-stage sequence. No stage may be skipped. No autonomous action.
- **Rationale:** Core product philosophy. User trust depends on predictable, permission-gated AI.
- **Consequences:** Architecture must include permission gateway. AI features cannot ship without this flow.
- **Related:** [AI Principles](../05-AI/AI-PRINCIPLES.md), [Project Constitution](../00-Constitution/PROJECT-CONSTITUTION.md)

---

### DEC-005: Local-First Data Preference

- **Date:** 2026-07-23
- **Status:** Accepted
- **Decision Type:** B (Strategic)
- **Owner:** Project Owner
- **Context:** Need guiding principle for where user data lives.
- **Options Considered:** (A) Local-first; (B) Cloud-first; (C) Hybrid default.
- **Decision:** Local-first as the guiding preference. Cloud sync is opt-in if adopted.
- **Rationale:** Privacy, performance, offline capability, and user trust. Aligns with security principles.
- **Consequences:** Architecture defaults to local storage. Any cloud feature requires explicit opt-in and Decision Log entry. Reinforced by DEC-010 (SQLite local database).
- **Related:** [Architecture Principles](../02-Architecture/ARCHITECTURE-PRINCIPLES.md), OQ-005, DEC-010

---

### DEC-006: Project License — MIT

- **Date:** 2026-07-23
- **Status:** Accepted
- **Decision Type:** B (Strategic)
- **Owner:** Project Owner
- **Context:** Project license undecided (OQ-010). Required before dependency selection and contribution model.
- **Options Considered:** MIT, Apache 2.0, GPL, proprietary.
- **Decision:** Use the MIT License.
- **Rationale:** Maximum flexibility for commercial development, future plugin ecosystem, community contribution, and low legal complexity.
- **Consequences:** Project remains open and permissive. Future proprietary components can be separated if required. `LICENSE` file added to repository root.
- **Related:** OQ-010 (resolved), [Dependency Policy](../03-Engineering/DEPENDENCY-POLICY.md)

---

### DEC-007: Technology Stack — Tauri

- **Date:** 2026-07-23
- **Status:** Accepted
- **Decision Type:** B (Strategic)
- **Owner:** Project Owner
- **Context:** Technology stack blocked Phase 1 (OQ-001). Evaluated against [Stack Evaluation Criteria](../02-Architecture/STACK-EVALUATION-CRITERIA.md).
- **Options Considered:** Electron + TypeScript, Tauri + Rust + React, native .NET, other.
- **Decision:** Tauri-based architecture:
  - **Frontend:** React + TypeScript
  - **Desktop runtime:** Tauri
  - **Core runtime:** Rust services
  - **Data layer:** SQLite
  - **Supporting layers:** Plugin Runtime, AI Subsystem, Windows Integration Layer
- **Rationale:** Lower resource usage than Electron, stronger system integration potential, suitable for long-running desktop software, supports secure native capabilities, aligns with local-first architecture.
- **Consequences:** Rust knowledge required. More initial complexity than Electron. Better long-term foundation. See [System Overview](../02-Architecture/SYSTEM-OVERVIEW.md) for target architecture.
- **Related:** OQ-001 (resolved), DEC-010, DEC-011, [Repository Structure](../02-Architecture/REPOSITORY-STRUCTURE.md)

---

### DEC-008: Windows Integration Model — Hybrid Companion + Overlay

- **Date:** 2026-07-23
- **Status:** Accepted
- **Decision Type:** B (Strategic)
- **Owner:** Project Owner
- **Context:** Windows coexistence model blocked Phase 1 (OQ-014). Constitution prohibits replacing Windows.
- **Options Considered:** Overlay only, companion application only, deeper system integration, hybrid.
- **Decision:** Hybrid Companion + Overlay model. Workspace operates as an intelligent desktop environment layer — not a Windows replacement. v1 focus: workspace management, application launching, layout management, user-approved automation. Future: deeper Windows integration and advanced system awareness.
- **Rationale:** Strong capability without excessive risk. Remains compatible with Windows. Deeper integration can be added gradually.
- **Consequences:** Requires both companion app shell and overlay interface components. See [Windows Integration Model](../02-Architecture/WINDOWS-INTEGRATION-MODEL.md).
- **Related:** OQ-014 (resolved), DEC-007, [MVP Definition](../01-Product/MVP-DEFINITION.md)

---

### DEC-009: Layout System — Spatial Workspace Canvas

- **Date:** 2026-07-23
- **Status:** Accepted
- **Decision Type:** B (Strategic)
- **Owner:** Project Owner
- **Context:** Layout system blocked shell prototype design (OQ-013).
- **Options Considered:** Free-form floating panels, grid/tile system, zones/areas, hybrid spatial model.
- **Decision:** Spatial Workspace Canvas model. Workspace contains Zones containing Applications, Widgets, AI Suggestions, and Automation Blocks. Draggable elements, customizable workspace, saved layouts, multiple workspaces, user-defined arrangements, persistent state. UX constraint: consistent navigation/button placement — customization applies to workspace content, not core navigation.
- **Rationale:** Matches intended premium workspace/operator environment. Supports future AI-assisted organization.
- **Consequences:** Requires flexible layout data model in SQLite (DEC-010). Schema design needed in Phase 1.
- **Related:** OQ-013 (resolved), DEC-010, [UX Principles](../04-UX/UX-PRINCIPLES.md)

---

### DEC-010: Data Persistence — SQLite + JSON Export

- **Date:** 2026-07-23
- **Status:** Accepted
- **Decision Type:** B (Strategic)
- **Owner:** Project Owner
- **Context:** Persistence format blocked layout save/restore (OQ-003). Local-first preference (DEC-005).
- **Options Considered:** JSON files only, SQLite only, SQLite + JSON export, Windows registry (partial).
- **Decision:** SQLite as primary local database. JSON for export, backup, migration, debugging, and portability. SQLite stores: layouts, application configurations, permissions, AI patterns, automation rules, system preferences.
- **Rationale:** Structured growth while maintaining local-first principles. Queryable, transactional, suitable for relational data at scale.
- **Consequences:** Requires schema versioning and migrations. Rust SQLite integration in core runtime.
- **Related:** OQ-003 (resolved), DEC-005, DEC-007, DEC-009

---

### DEC-011: Process Architecture — Multi-Process

- **Date:** 2026-07-23
- **Status:** Accepted
- **Decision Type:** B (Strategic)
- **Owner:** Project Owner
- **Context:** Process model affects plugin isolation and crash containment (OQ-002).
- **Options Considered:** All in-process, domain services as separate processes, hybrid multi-process.
- **Decision:** Multi-process architecture:
  - Frontend Process (Tauri webview — React UI)
  - Workspace Core Process (Rust runtime)
  - Plugin Processes (isolated)
  - AI Worker Processes (isolated)
- **Rationale:** Plugin isolation, crash containment, security boundaries, scalability.
- **Consequences:** Requires inter-process communication contracts. See [Event and API Standards](../02-Architecture/EVENT-AND-API-STANDARDS.md).
- **Related:** OQ-002 (resolved), DEC-007, [Threat Model](../07-Security/THREAT-MODEL.md)

---

### DEC-012: Monorepo Tooling — pnpm Workspaces

- **Date:** 2026-07-23
- **Status:** Accepted
- **Decision Type:** C (Tactical)
- **Owner:** Lead Software Engineer
- **Context:** Monorepo structure decided (DEC-003) but tooling undecided (OQ-019). Stack now Tauri + TypeScript (DEC-007).
- **Options Considered:** pnpm workspaces, npm workspaces, Nx, Turborepo, Lerna, Cargo workspace only.
- **Decision:** pnpm workspaces for TypeScript/JavaScript packages. Rust crates managed via Cargo workspace within the same repository.
- **Rationale:** Good TypeScript support, simple workspace management, suitable for modular architecture.
- **Consequences:** `pnpm-workspace.yaml` at repository root. Lock file committed per [Dependency Policy](../03-Engineering/DEPENDENCY-POLICY.md).
- **Related:** OQ-019 (resolved), DEC-003, [Repository Structure](../02-Architecture/REPOSITORY-STRUCTURE.md)

---

### DEC-013: AI Confidence Framework — L0–L4 Model

- **Date:** 2026-07-23
- **Status:** Accepted
- **Decision Type:** B (Strategic)
- **Owner:** Project Owner
- **Context:** Confidence thresholds needed for AI suggestion engine (OQ-015).
- **Options Considered:** Rule-based thresholds only, L0–L4 model (approved), binary suggest/don't-suggest.
- **Decision:** Adopt L0–L4 confidence model:
  - **L0:** No confidence — no suggestion
  - **L1:** Observation only
  - **L2:** Suggestion allowed
  - **L3:** Permission request required
  - **L4:** Previously approved automation
  Rules: AI cannot silently escalate confidence. Automation requires explicit user approval regardless of level.
- **Rationale:** Graduated trust model protects users while enabling useful suggestions at appropriate confidence.
- **Consequences:** [Confidence Policy](../05-AI/CONFIDENCE-POLICY.md) updated to reflect approved levels. Implementation in Phase 2.
- **Related:** OQ-015 (resolved), DEC-004, [AI Operating Model](../05-AI/AI-OPERATING-MODEL.md)

---

### DEC-014: AI Memory Retention — User-Controlled Adaptive Memory

- **Date:** 2026-07-23
- **Status:** Accepted
- **Decision Type:** B (Strategic)
- **Owner:** Project Owner
- **Context:** Retention policy needed for AI pattern store (OQ-016).
- **Options Considered:** Indefinite retention, fixed expiry, user-controlled adaptive memory (approved).
- **Decision:** User-controlled adaptive memory:
  - Temporary observations expire
  - Learned patterns require sufficient confidence before persistence
  - Stored memories are reviewable
  - User controls deletion
  - No hidden learning
- **Rationale:** Maintains trust and transparency. Aligns with constitution user-control principle.
- **Consequences:** [Memory Policy](../05-AI/MEMORY-POLICY.md) updated. SQLite schema must support memory inspection and deletion.
- **Related:** OQ-016 (resolved), DEC-010, [Memory Policy](../05-AI/MEMORY-POLICY.md)

---

### DEC-015: Tiered Encryption Strategy

- **Date:** 2026-07-23
- **Status:** Accepted
- **Decision Type:** B (Strategic)
- **Owner:** Project Owner
- **Context:** Encryption at rest requirements needed before persistence implementation (OQ-017). Threat Model TD-01 references local data exposure.
- **Options Considered:** (A) No encryption (OS file permissions only); (B) Encrypt sensitive data only; (C) Encrypt all user data; (D) Tiered approach (approved).
- **Decision:** Adopt tiered encryption strategy:
  - **Tier 0:** Normal local storage with OS-level file protection (default for Sprint 01)
  - **Tier 1:** Sensitive data encryption abstraction (patterns, automations — future)
  - **Tier 2:** Full database encryption (future)
  Sprint 01 implements `EncryptionProvider` trait and `NoOpEncryptionProvider` placeholder only. No application-level encryption in Sprint 01.
- **Rationale:** Balances security posture with implementation complexity. Allows incremental hardening without blocking Phase 1 scaffold. Sensitive data can be protected before full-database encryption is justified.
- **Consequences:** `packages/database` includes encryption module boundary. Future tiers require Decision Log updates if scope changes. Threat Model TD-01 mitigation path defined.
- **Related:** OQ-017 (resolved), DEC-010, [Threat Model](../07-Security/THREAT-MODEL.md), [Security Principles](../07-Security/SECURITY-PRINCIPLES.md)

---

## 3. Pending Decisions

Decisions that are needed but not yet made are tracked in [Open Questions](OPEN-QUESTIONS.md), not here. When resolved, they move from Open Questions to this log.

---

## Related Documents

- [Open Questions](OPEN-QUESTIONS.md)
- [Governance Model](../00-Constitution/GOVERNANCE.md)
- [Roadmap](../08-Roadmap/ROADMAP.md)
