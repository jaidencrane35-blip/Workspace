# Open Questions

| Field | Value |
|-------|-------|
| **Purpose** | Track unresolved decisions and ambiguities that block or affect implementation |
| **Owner** | Project Lead |
| **Dependencies** | [Governance Model](../00-Constitution/GOVERNANCE.md), [Decision Log](DECISION-LOG.md) |
| **Update Process** | Add questions as identified. Triage weekly during active development. Resolve by moving to Decision Log. Never delete — mark resolved with link to decision. |

---

## 1. How to Use This Document

When a requirement is ambiguous:

1. **Stop** — do not guess
2. **Add** an entry here using the template below
3. **Wait** for the appropriate authority to resolve it
4. **Record** the resolution in the [Decision Log](DECISION-LOG.md)

### Entry Template

```
### OQ-NNN: Title
- **Category:** Architecture | Product | UX | AI | Security | Engineering | Legal
- **Priority:** Blocker | High | Medium | Low
- **Status:** Open | In Review | Resolved
- **Raised:** YYYY-MM-DD
- **Owner:** Role responsible for resolution
- **Question:** What needs to be decided?
- **Context:** Why this matters
- **Options:** Known alternatives (if any)
- **Impact:** What is blocked until this is resolved?
- **Resolution:** (filled when resolved — link to DEC-NNN)
```

---

## 2. Open Questions

### OQ-001: Technology Stack Selection

- **Category:** Architecture
- **Priority:** Blocker
- **Status:** Open
- **Raised:** 2026-07-23
- **Owner:** Architect (TBD)
- **Question:** What technology stack will Workspace use? (Language, UI framework, build system, packaging)
- **Context:** Blocks all Phase 1 implementation. Candidates may include Electron + TypeScript, Tauri + Rust, native C#/.NET, or others.
- **Options:** Electron, Tauri, native .NET, other — see [Stack Evaluation Criteria](../02-Architecture/STACK-EVALUATION-CRITERIA.md)
- **Impact:** Blocks Phase 1 gate. Coding standards language sections blocked. CI/CD design blocked.

### OQ-002: Inter-Process vs In-Process Architecture

- **Category:** Architecture
- **Priority:** High
- **Status:** Open
- **Raised:** 2026-07-23
- **Owner:** Architect (TBD)
- **Question:** Should domain services run in-process or as separate processes?
- **Context:** Affects performance, reliability, plugin isolation, and debugging complexity.
- **Options:** (A) All in-process; (B) Domain services as separate processes; (C) Hybrid
- **Impact:** Core architecture design. Plugin sandboxing approach depends on this.

### OQ-003: Data Persistence Format

- **Category:** Architecture
- **Priority:** High
- **Status:** Open
- **Raised:** 2026-07-23
- **Owner:** Architect (TBD)
- **Question:** What format and storage mechanism for layouts, preferences, and learned patterns?
- **Context:** Local-first decision (DEC-005) requires a storage strategy.
- **Options:** JSON files, SQLite, embedded database, Windows registry (partial)
- **Impact:** Layout persistence, AI pattern store, configuration management.

### OQ-004: AI Model Selection

- **Category:** AI
- **Priority:** High
- **Status:** Open
- **Raised:** 2026-07-23
- **Owner:** AI Lead (TBD)
- **Question:** What AI approach for pattern recognition and suggestion generation?
- **Context:** AI must observe, learn, and suggest. Model choice affects privacy, performance, and capability.
- **Options:** (A) Rule-based / statistical only; (B) Local LLM; (C) Hybrid (local + remote); (D) Remote API
- **Impact:** AI subsystem architecture, privacy model, offline capability.

### OQ-005: Cloud Sync Scope

- **Category:** Architecture
- **Priority:** Medium
- **Status:** Open
- **Raised:** 2026-07-23
- **Owner:** Product Owner (TBD)
- **Question:** Will Workspace offer cloud sync? If so, for what data and when?
- **Context:** Local-first is the preference (DEC-005) but cloud sync may be desired for layouts and settings across machines.
- **Options:** (A) No cloud sync; (B) Optional layout/settings sync; (C) Full sync; (D) Defer decision
- **Impact:** Data architecture, security model, infrastructure requirements.

### OQ-006: Phone Integration Protocol

- **Category:** Architecture
- **Priority:** Medium
- **Status:** Open
- **Raised:** 2026-07-23
- **Owner:** Architect (TBD)
- **Question:** How will Workspace integrate with phones and mobile devices?
- **Context:** Product vision includes phone integration. Protocol choice affects device service design.
- **Options:** (A) Companion app with custom protocol; (B) Existing protocols (KDE Connect, etc.); (C) Platform-specific (Phone Link); (D) Defer to Phase 3
- **Impact:** Device service architecture. Phase 2 scope.

### OQ-007: Plugin Runtime Technology

- **Category:** Architecture
- **Priority:** Medium
- **Status:** Open
- **Raised:** 2026-07-23
- **Owner:** Architect (TBD)
- **Question:** What runtime technology for plugin sandboxing?
- **Context:** Plugin architecture vision requires sandboxed execution. Technology choice affects security and capability.
- **Options:** WebView/isolated JS, WASM, native modules with restrictions, separate process
- **Impact:** Plugin SDK design. Phase 3 scope.

### OQ-008: Accessibility Target Level

- **Category:** UX
- **Priority:** Medium
- **Status:** Open
- **Raised:** 2026-07-23
- **Owner:** UX Lead (TBD)
- **Question:** What WCAG conformance level should Workspace target?
- **Context:** UX principles establish baseline accessibility expectations. Specific target needed before UI implementation.
- **Options:** WCAG 2.1 Level A, Level AA, Level AAA
- **Impact:** UI component design, testing requirements, release gate criteria.

### OQ-009: Distribution Model

- **Category:** Product
- **Priority:** Medium
- **Status:** Open
- **Raised:** 2026-07-23
- **Owner:** Product Owner (TBD)
- **Question:** How will Workspace be distributed to users?
- **Context:** Affects installer design, update mechanism, code signing, and Windows Store requirements.
- **Options:** (A) Direct download; (B) Windows Store; (C) Both; (D) Package manager (winget, etc.)
- **Impact:** Release pipeline, security requirements, licensing.

### OQ-010: Project License

- **Category:** Legal
- **Priority:** Medium
- **Status:** Open
- **Raised:** 2026-07-23
- **Owner:** Project Lead
- **Question:** What open-source or proprietary license will Workspace use?
- **Context:** Affects contribution model, third-party usage, and plugin ecosystem.
- **Options:** MIT, Apache 2.0, GPL, proprietary, other
- **Impact:** Contribution guidelines, dependency compatibility, commercial use.

### OQ-011: Final Product Name

- **Category:** Product
- **Priority:** Low
- **Status:** Open
- **Raised:** 2026-07-23
- **Owner:** Product Owner (TBD)
- **Question:** Is "Workspace" the final product name?
- **Context:** Currently used as temporary name (DEC-001). Branding affects repository, packages, and user-facing identity.
- **Options:** Keep "Workspace"; rebrand
- **Impact:** Naming across repository, packages, installer, documentation. Low urgency.

### OQ-012: Suggestion Frequency Limits

- **Category:** AI
- **Priority:** Medium
- **Status:** Open
- **Raised:** 2026-07-23
- **Owner:** AI Lead (TBD)
- **Question:** How often should AI present suggestions to the user?
- **Context:** Too frequent suggestions feel nagging; too rare misses value. Need design guidelines.
- **Options:** Fixed limit (N per hour/day); adaptive based on acceptance rate; user-configurable
- **Impact:** AI suggestion engine design, UX quality.

### OQ-013: Layout System Design

- **Category:** UX
- **Priority:** High
- **Status:** Open
- **Raised:** 2026-07-23
- **Owner:** UX Lead (TBD)
- **Question:** What layout system will the shell use?
- **Context:** UX principles require resizable, movable, persistent panels. Layout system choice affects shell architecture.
- **Options:** (A) Free-form floating panels; (B) Grid/tile system; (C) Zones/areas; (D) Hybrid
- **Impact:** Shell architecture, Phase 1 prototype design.

### OQ-014: Workspace Windows Integration Model

- **Category:** Architecture
- **Priority:** Blocker
- **Status:** Open
- **Raised:** 2026-07-23
- **Owner:** Project Owner
- **Question:** How does Workspace coexist with and integrate into Windows?
- **Context:** Constitution requires Workspace to layer over Windows, not replace it. Integration model affects shell design, MVP scope, and stack evaluation.
- **Options:** (A) Overlay; (B) Companion application; (C) Deeper system integration; (D) Hybrid — see [Windows Integration Model](../02-Architecture/WINDOWS-INTEGRATION-MODEL.md)
- **Impact:** Shell architecture, Phase 1 prototype, stack evaluation (OQ-001).

### OQ-015: AI Confidence Thresholds

- **Category:** AI
- **Priority:** High
- **Status:** Open
- **Raised:** 2026-07-23
- **Owner:** Project Owner
- **Question:** What numeric confidence thresholds govern when AI suggests automations?
- **Context:** [Confidence Policy](../05-AI/CONFIDENCE-POLICY.md) defines levels L0–L4 but specific values are proposed, not approved.
- **Options:** Approve proposed thresholds in Confidence Policy; modify; or define alternatives
- **Impact:** AI suggestion engine design (Phase 2).

### OQ-016: AI Learned-Data Retention Policy

- **Category:** AI
- **Priority:** High
- **Status:** Open
- **Raised:** 2026-07-23
- **Owner:** Project Owner
- **Question:** How long does Workspace retain learned patterns, suggestion history, and automation logs?
- **Context:** [Memory Policy](../05-AI/MEMORY-POLICY.md) defines principles but proposed retention tiers are unapproved.
- **Options:** Indefinite until user deletes; time-based expiry; tiered retention per data type
- **Impact:** Storage design, privacy, performance (pattern store size).

### OQ-017: Encryption at Rest Requirements

- **Category:** Security
- **Priority:** High
- **Status:** Open
- **Raised:** 2026-07-23
- **Owner:** Project Owner
- **Question:** Must user data (layouts, patterns, preferences) be encrypted at rest on disk?
- **Context:** Security Principles and Threat Model (TD-01) reference encryption at rest as TBD.
- **Options:** (A) No encryption (OS file permissions only); (B) Encrypt sensitive data (patterns, automations); (C) Encrypt all user data; (D) Optional user-controlled encryption
- **Impact:** Storage implementation, security posture, performance.

### OQ-018: Cross-Device Learning Scope

- **Category:** AI
- **Priority:** Medium
- **Status:** Open
- **Raised:** 2026-07-23
- **Owner:** Project Owner
- **Question:** May Workspace share learned patterns across a user's devices?
- **Context:** Memory Policy prohibits cross-device learning until resolved. Product vision includes phone integration.
- **Options:** (A) No cross-device learning; (B) Opt-in sync of patterns; (C) Full cross-device learning with cloud; (D) Defer until device integration (Phase 3)
- **Impact:** Data architecture, privacy model, cloud requirements.

### OQ-019: Monorepo Tooling Selection

- **Category:** Engineering
- **Priority:** High
- **Status:** Open
- **Raised:** 2026-07-23
- **Owner:** Lead Software Engineer
- **Question:** What monorepo tooling will manage the Workspace repository?
- **Context:** DEC-003 commits to monorepo structure. Tooling depends on stack (OQ-001).
- **Options:** pnpm workspaces, npm workspaces, Nx, Turborepo, Lerna, Cargo workspace (if Tauri), .NET solution (if native)
- **Impact:** CI/CD design, developer workflow, build performance.

---

## 3. Resolved Questions

_None yet. Resolved questions move here with a link to their Decision Log entry._

---

## Related Documents

- [Decision Log](DECISION-LOG.md)
- [Risk Register](../07-Security/RISK-REGISTER.md)
- [Roadmap](../08-Roadmap/ROADMAP.md)
