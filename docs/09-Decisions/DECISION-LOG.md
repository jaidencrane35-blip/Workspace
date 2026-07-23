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

### DEC-016: Resource Addressing Model

- **Date:** 2026-07-23
- **Status:** Accepted
- **Decision Type:** B (Strategic)
- **Owner:** Lead Software Engineer
- **Context:** Sprint 06 introduced per-entity typed IDs (`WorkspaceId`, `ZoneId`, `ApplicationId`, `WidgetId`). Sprint 09 introduced a closed `PermissionSubject` enum (`System`, `Settings`, `Workspace`). The next phase (Resource Services + Workspace Graph) requires a uniform way to *address* any resource for permissions, audit, graph nodes, and IPC. Without a canonical address, every new resource kind forces cross-cutting edits (permission subject, event enum, audit metadata) and the graph has no uniform node identity. The audit (post–Sprint 09) flagged this as the highest feature-refactor risk if deferred.

- **Resource identity strategy:** Introduce a canonical resource address composed of a **`ResourceKind`** (an extensible classification — `Workspace`, `Zone`, `Application`, `Widget`, and future kinds) and a **`ResourceId`** (a generic, globally-unique identifier). Together they form a **`ResourceRef { kind, id }`**. Resource IDs remain UUIDv4 and are globally unique across kinds, so `kind` is descriptive/routing metadata rather than a uniqueness requirement.

- **Node identity vs typed IDs:** Both are retained, at different layers.
  - **Typed IDs** (`WorkspaceId`, `ZoneId`, …) remain the compile-time-safe identifiers *inside* each bounded entity and its repository, preserving the type safety established in Sprint 06.
  - **`ResourceRef`** is the *cross-cutting* address used wherever code must treat resources uniformly: permission subjects, audit records, Workspace Graph nodes/edges, and resource-facing IPC. Each typed entity provides a lossless mapping to and from its `ResourceRef`.
  - `PermissionSubject` is redefined to be `ResourceKind`-driven for resource operations, while retaining a dedicated `System` subject for non-resource operations (startup, shutdown, settings-as-system). This removes the closed 3-variant enum bottleneck.

- **Relationship to the future Workspace Graph:** The Workspace Graph is defined *on top of* this addressing model. **Nodes** are resources addressed by `ResourceRef`. **Edges** are typed relationships referencing a source `ResourceRef` and a target `ResourceRef` plus a relationship type. The addressing model is therefore a prerequisite for the graph, not part of it — the graph adds edges and traversal, but node identity is already defined here.

- **IPC implications:** `ResourceRef` serializes over IPC as a structured object `{ "kind": "...", "id": "..." }` (forward-compatible; avoids string-parsing ambiguity), with a canonical `kind:id` string form reserved for audit records and logs. Existing typed IDs currently serialize transparently as bare strings; resource-facing IPC introduced from this point uses `ResourceRef` from the start. Pre-existing settings/workspace endpoints may continue returning bare IDs until a coordinated, versioned contract change — no forced rewrite of shipped endpoints.

- **Migration implications:** Additive only. Existing tables are unchanged; their UUID primary keys already satisfy global uniqueness. The future graph introduces a `graph_edges` table keyed by `(source_kind, source_id, relationship, target_kind, target_id)`. No data rewrite is required to adopt the addressing model; only new resource/graph tables reference `ResourceRef` columns.

- **Options Considered:**
  - **(A) Keep per-type typed IDs only; extend `PermissionSubject` per resource kind.** Rejected — closed enum forces cross-cutting edits per kind and gives the graph no uniform node identity.
  - **(B) Replace typed IDs with a single opaque global `ResourceId` string everywhere.** Rejected — discards the compile-time safety established in DEC-era Sprint 06.
  - **(C) Hybrid: typed IDs for entity internals + `ResourceRef { kind, id }` for cross-cutting concerns.** **Chosen** — preserves type safety while giving permissions, audit, graph, and IPC one uniform address.
  - **(D) URN-style single string (`workspace:uuid`) as the universal identifier.** Adopted only as the canonical *serialized/audit* form within (C), not as the primary in-memory type.

- **Decision:** Adopt option **(C)**. Define `ResourceKind`, `ResourceId`, and `ResourceRef` as the canonical resource address. Redefine `PermissionSubject` to be `ResourceKind`-driven for resource operations while retaining a `System` subject. Address Workspace Graph nodes and edges by `ResourceRef`. Serialize `ResourceRef` as `{ kind, id }` over IPC and as `kind:id` in audit/logs.

- **Rationale:** Fixes the highest-risk foundational gap before feature code depends on it. Uniform addressing makes permissions, audit, and the graph extensible without cross-cutting edits, while typed IDs keep entity code type-safe. The change is additive to storage and to shipped IPC contracts.

- **Consequences:** A resource-addressing type set is introduced in the domain layer and threaded through permission subjects, audit attribution, and (later) graph nodes/edges. New resource kinds are added by extending `ResourceKind` and providing a typed-ID↔`ResourceRef` mapping — no changes to unrelated commands. Existing typed IDs and shipped endpoints remain valid. Implementation is performed in a later Composer step, not by this record.

- **Related:** DEC-003, DEC-009 (Spatial Workspace Canvas / graph), DEC-010, Sprint 06 (typed IDs), Sprint 09 (permission subject), DEC-017

---

### DEC-017: Read Governance Policy

- **Date:** 2026-07-23
- **Status:** Accepted
- **Decision Type:** B (Strategic)
- **Owner:** Project Owner
- **Context:** The Sprint 09 command pipeline governs mutations (policy + gate + audit) but `execute_query` bypasses policy, gate, and audit entirely. Read capabilities (`workspace.read`, `settings.read`, `audit.read`) exist but are never consulted. Before Resource Services introduce read-heavy and non-human-initiated reads, the project must decide *which reads require governance*, or later governing them would force query-command, IPC, and frontend refactors (post–Sprint 09 audit finding C2). This must align with the constitution's AI-advisory model (DEC-004) and confidence framework (DEC-013).

- **Which reads require governance:** Governance is **actor-driven with a resource-sensitivity override**:
  - **Ungoverned** (allowed without policy/gate): reads by the **local human user** of **non-sensitive** local resources (e.g. listing own workspaces, zones, applications, widgets). Consistent with human authority — the local human already holds full authority over local data.
  - **Governed** (policy + gate consulted): (1) **all reads by non-human actors** (AIAssistant, Automation, Plugin, RemoteSession), and (2) **all reads of sensitive resource kinds** regardless of actor — currently the audit log itself, settings containing secrets, and any future credential/permission/policy resources.

- **Human vs AI vs Plugin vs Automation reads:**
  | Actor | Non-sensitive read | Sensitive-kind read |
  |-------|--------------------|---------------------|
  | LocalUser (human) | Ungoverned, unaudited | Governed + audited |
  | AIAssistant | Governed + audited | Governed + audited |
  | Plugin | Governed + audited | Governed + audited |
  | Automation | Governed + audited | Governed + audited |
  | RemoteSession | Governed + audited | Governed + audited |
  AI reads are always governed because observation beyond passive L1 must remain permission-gated (DEC-004, DEC-013). Plugin and automation reads are governed under least-privilege and delegated-authority principles.

- **Audit requirements:** Governed reads produce a full audit entry (actor, intent, `*.read` capability, target `ResourceRef`, decision) with **no payloads or resource content** (consistent with Sprint 07). Human ungoverned reads of non-sensitive data are **not audited by default** to prevent audit flooding. Reads of sensitive kinds are **always audited regardless of actor**.

- **Performance implications:** Auditing every read would flood the audit table and amplify writes on the single serialized database connection. Actor-driven governance keeps the human hot path (browsing one's own workspace/graph) free of gate checks and audit writes, while lower-frequency non-human reads bear the governance cost. Batched/asynchronous read-audit writes are noted as a *future* optimisation if governed-read volume grows; not decided here.

- **Future extensibility:** The query contract is extended so each `QueryCommand` declares (1) a required `*.read` capability and (2) a **governance class** (`Ungoverned` | `Governed`). At execution the pipeline combines the declared class, the actor type, and the target resource kind's sensitivity to decide whether to invoke policy, gate, and audit. Sensitive kinds opt in via a central classification. Declaring these now means query commands are shaped correctly from the outset, so later *enforcement* is additive rather than a refactor.

- **Options Considered:**
  - **(A) Leave all reads ungoverned.** Rejected — violates the AI-advisory constitution and leaves sensitive reads (audit log, secrets) unprotected.
  - **(B) Govern and audit every read for every actor.** Rejected — audit flooding and write amplification on the single connection; burdens the human hot path with no trust benefit.
  - **(C) Actor-driven governance with a sensitivity override.** **Chosen** — enforces the constitution for non-human actors and sensitive data while keeping the local human experience fast and quiet.
  - **(D) Purely resource-sensitivity-driven (ignore actor).** Rejected — would under-govern AI/plugin reads of ordinary resources, contradicting DEC-004.

- **Decision:** Adopt option **(C)**. Local-human reads of non-sensitive local resources are ungoverned and unaudited; all non-human-actor reads and all sensitive-resource-kind reads are governed (policy + gate) and audited. Extend the `QueryCommand` contract to declare a read capability and a governance class now, with enforcement remaining allow-all (per Sprint 09) until a dedicated enforcement sprint.

- **Rationale:** Resolves the second high-risk foundational gap before read-heavy features exist. Keeps query commands correctly shaped so future enforcement is additive, upholds the AI-advisory constitution, protects sensitive reads, and avoids audit/performance blowup on the hot path.

- **Consequences:** `QueryCommand` gains capability + governance-class declarations; the query pipeline gains an (initially allow-all) governance step mirroring mutations; a resource-kind sensitivity classification is introduced. Human non-sensitive reads remain unaudited by design. Implementation is performed in a later Composer step, not by this record.

- **Related:** DEC-004 (AI permission sequence), DEC-013 (confidence model), Sprint 07 (audit), Sprint 09 (policy/gate/capability), DEC-016

---

## 3. Pending Decisions

Decisions that are needed but not yet made are tracked in [Open Questions](OPEN-QUESTIONS.md), not here. When resolved, they move from Open Questions to this log.

---

## Related Documents

- [Open Questions](OPEN-QUESTIONS.md)
- [Governance Model](../00-Constitution/GOVERNANCE.md)
- [Roadmap](../08-Roadmap/ROADMAP.md)
