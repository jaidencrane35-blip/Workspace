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
- **Status:** Accepted (preference, not final)
- **Decision Type:** B (Strategic)
- **Owner:** Architect (TBD)
- **Context:** Need guiding principle for where user data lives.
- **Options Considered:** (A) Local-first; (B) Cloud-first; (C) Hybrid default.
- **Decision:** Local-first as the guiding preference. Cloud sync is opt-in if adopted.
- **Rationale:** Privacy, performance, offline capability, and user trust. Aligns with security principles.
- **Consequences:** Architecture defaults to local storage. Any cloud feature requires explicit opt-in and Decision Log entry.
- **Related:** [Architecture Principles](../02-Architecture/ARCHITECTURE-PRINCIPLES.md), OQ-005

---

## 3. Pending Decisions

Decisions that are needed but not yet made are tracked in [Open Questions](OPEN-QUESTIONS.md), not here. When resolved, they move from Open Questions to this log.

---

## Related Documents

- [Open Questions](OPEN-QUESTIONS.md)
- [Governance Model](../00-Constitution/GOVERNANCE.md)
- [Roadmap](../08-Roadmap/ROADMAP.md)
