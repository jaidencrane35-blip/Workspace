# Open Questions

| Field | Value |
|-------|-------|
| **Purpose** | Track unresolved decisions and ambiguities that block or affect implementation |
| **Owner** | Project Owner |
| **Dependencies** | [Governance Model](../00-Constitution/GOVERNANCE.md), [Decision Log](DECISION-LOG.md) |
| **Update Process** | Add questions as identified. Triage weekly during active development. Resolve by moving to Decision Log. Never delete — mark resolved with link to decision. |

---

## 1. How to Use This Document

When a requirement is ambiguous:

1. **Stop** — do not guess
2. **Add** an entry here using the template below
3. **Wait** for the Project Owner to resolve it
4. **Record** the resolution in the [Decision Log](DECISION-LOG.md)

---

## 2. Open Questions

### OQ-004: AI Model Selection

- **Category:** AI
- **Priority:** High
- **Status:** Open
- **Raised:** 2026-07-23
- **Owner:** Project Owner
- **Question:** What AI approach for pattern recognition and suggestion generation?
- **Context:** AI must observe, learn, and suggest. Model choice affects privacy, performance, and capability.
- **Options:** (A) Rule-based / statistical only; (B) Local LLM; (C) Hybrid (local + remote); (D) Remote API
- **Impact:** AI subsystem architecture (Phase 2), privacy model, offline capability.

### OQ-005: Cloud Sync Scope

- **Category:** Architecture
- **Priority:** Medium
- **Status:** Open
- **Raised:** 2026-07-23
- **Owner:** Project Owner
- **Question:** Will Workspace offer cloud sync? If so, for what data and when?
- **Context:** Local-first is confirmed (DEC-005, DEC-010) but cloud sync may be desired for layouts and settings across machines.
- **Options:** (A) No cloud sync; (B) Optional layout/settings sync; (C) Full sync; (D) Defer decision
- **Impact:** Data architecture, security model, infrastructure requirements.

### OQ-006: Phone Integration Protocol

- **Category:** Architecture
- **Priority:** Medium
- **Status:** Open
- **Raised:** 2026-07-23
- **Owner:** Project Owner
- **Question:** How will Workspace integrate with phones and mobile devices?
- **Context:** Product vision includes phone integration. Protocol choice affects device service design.
- **Options:** (A) Companion app with custom protocol; (B) Existing protocols (KDE Connect, etc.); (C) Platform-specific (Phone Link); (D) Defer to Phase 3
- **Impact:** Device service architecture. Phase 2+ scope.

### OQ-007: Plugin Runtime Technology

- **Category:** Architecture
- **Priority:** Medium
- **Status:** Open
- **Raised:** 2026-07-23
- **Owner:** Project Owner
- **Question:** What runtime technology for plugin sandboxing?
- **Context:** Multi-process architecture decided (DEC-011). Plugin processes isolated but runtime technology within process undecided.
- **Options:** WebView/isolated JS, WASM, native modules with restrictions, separate Rust processes
- **Impact:** Plugin SDK design. Phase 3 scope.

### OQ-008: Accessibility Target Level

- **Category:** UX
- **Priority:** Medium
- **Status:** Open
- **Raised:** 2026-07-23
- **Owner:** Project Owner
- **Question:** What WCAG conformance level should Workspace target?
- **Context:** UX principles establish baseline accessibility expectations. Specific target needed before UI implementation.
- **Options:** WCAG 2.1 Level A, Level AA, Level AAA
- **Impact:** UI component design, testing requirements, release gate criteria.

### OQ-009: Distribution Model

- **Category:** Product
- **Priority:** Medium
- **Status:** Open
- **Raised:** 2026-07-23
- **Owner:** Project Owner
- **Question:** How will Workspace be distributed to users?
- **Context:** Affects installer design, update mechanism, code signing, and Windows Store requirements. Tauri supports multiple distribution paths.
- **Options:** (A) Direct download; (B) Windows Store; (C) Both; (D) Package manager (winget, etc.)
- **Impact:** Release pipeline, security requirements.

### OQ-011: Final Product Name

- **Category:** Product
- **Priority:** Low
- **Status:** Open
- **Raised:** 2026-07-23
- **Owner:** Project Owner
- **Question:** Is "Workspace" the final product name?
- **Context:** Currently used as temporary name (DEC-001). Branding affects repository, packages, and user-facing identity.
- **Options:** Keep "Workspace"; rebrand
- **Impact:** Naming across repository, packages, installer, documentation. Low urgency.

### OQ-012: Suggestion Frequency Limits

- **Category:** AI
- **Priority:** Medium
- **Status:** Open
- **Raised:** 2026-07-23
- **Owner:** Project Owner
- **Question:** How often should AI present suggestions to the user?
- **Context:** Confidence framework approved (DEC-013) but frequency limits not defined.
- **Options:** Fixed limit (N per hour/day); adaptive based on acceptance rate; user-configurable
- **Impact:** AI suggestion engine design (Phase 2), UX quality.

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

---

## 3. Resolved Questions

| ID | Question | Resolution | Date |
|----|----------|------------|------|
| OQ-001 | Technology stack selection | [DEC-007](DECISION-LOG.md) — Tauri + React + TypeScript + Rust + SQLite | 2026-07-23 |
| OQ-002 | Inter-process vs in-process architecture | [DEC-011](DECISION-LOG.md) — Multi-process | 2026-07-23 |
| OQ-003 | Data persistence format | [DEC-010](DECISION-LOG.md) — SQLite + JSON export | 2026-07-23 |
| OQ-010 | Project license | [DEC-006](DECISION-LOG.md) — MIT License | 2026-07-23 |
| OQ-013 | Layout system design | [DEC-009](DECISION-LOG.md) — Spatial Workspace Canvas | 2026-07-23 |
| OQ-014 | Windows integration model | [DEC-008](DECISION-LOG.md) — Hybrid Companion + Overlay | 2026-07-23 |
| OQ-015 | AI confidence thresholds | [DEC-013](DECISION-LOG.md) — L0–L4 framework | 2026-07-23 |
| OQ-016 | AI learned-data retention | [DEC-014](DECISION-LOG.md) — User-controlled adaptive memory | 2026-07-23 |
| OQ-019 | Monorepo tooling | [DEC-012](DECISION-LOG.md) — pnpm workspaces | 2026-07-23 |
| OQ-017 | Encryption at rest requirements | [DEC-015](DECISION-LOG.md) — Tiered encryption strategy | 2026-07-23 |

---

## Related Documents

- [Decision Log](DECISION-LOG.md)
- [Risk Register](../07-Security/RISK-REGISTER.md)
- [Roadmap](../08-Roadmap/ROADMAP.md)
