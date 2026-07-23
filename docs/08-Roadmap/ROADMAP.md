# Roadmap

| Field | Value |
|-------|-------|
| **Purpose** | Define the phased delivery plan for Workspace from foundation through mature product |
| **Owner** | Project Owner |
| **Dependencies** | [Product Vision](../01-Product/PRODUCT-VISION.md), [Scope Management](../01-Product/SCOPE-MANAGEMENT.md), [Project Constitution](../00-Constitution/PROJECT-CONSTITUTION.md) |
| **Update Process** | Project Owner updates at phase boundaries. Phase scope changes require Decision Log entry. |

---

## 1. Roadmap Overview

```
Phase 0          Phase 1           Phase 2            Phase 3+
Foundation       Core Platform     Feature Expansion   Maturity
(docs only)      (MVP shell)       (domains + AI)      (plugins, polish)
   │                 │                  │                  │
   ▼                 ▼                  ▼                  ▼
 NOW            Stack decided       First domains       Plugin SDK
                Shell running       AI observing        Public release
                CI/CD active        Automations         Ecosystem
```

---

## 2. Phase 0 — Foundation

**Goal:** Establish the professional engineering and product foundation.

**Status:** Documentation foundation complete. Improvement pass complete. Phase 1 remains **blocked** until required decisions are resolved.

### Deliverables

- [x] Project constitution and governance
- [x] Product vision and scope management
- [x] Architecture principles and system overview
- [x] Engineering standards (coding, repo, docs, DoD, debt)
- [x] UX, AI, plugin, and security principles
- [x] Decision tracking and open questions
- [x] Roadmap and sprint structure
- [x] Repository initialisation and GitHub connection
- [x] Phase 0 improvement pass (AI governance, threat model, CI/CD plan, testing strategy, architecture foundation, MVP definition)
- [ ] Critical open questions resolved (see gate criteria below)
- [ ] Project Owner decisions recorded for Phase 1 blockers

### Gate Criteria (Phase 0 → Phase 1)

| Criterion | Status |
|-----------|--------|
| All foundation documents complete | **Done** |
| Improvement pass documents complete | **Done** |
| Threat model (initial) complete | **Done** |
| CI/CD plan documented | **Done** |
| Technology stack decision recorded | **Blocked** — OQ-001 |
| Windows integration model decided | **Blocked** — OQ-014 |
| Layout system decided | **Blocked** — OQ-013 |
| Data persistence format decided | **Blocked** — OQ-003 |
| Project license decided | **Blocked** — OQ-010 |
| Open questions triaged with owners and priorities | **Done** (19 questions tracked) |
| Architecture layer diagrams consistent | **Done** |

**Phase 1 must not begin until OQ-001, OQ-014, OQ-013, OQ-003, and OQ-010 are resolved and recorded in the Decision Log.**

---

## 3. Phase 1 — Core Platform

**Goal:** Select technology stack, scaffold the application, and deliver a running shell.

**Status:** Not started

### Expected Deliverables

- Technology stack decision (Decision Log)
- Application scaffolding (app entry point, build system)
- Platform kernel (event bus, state management, configuration)
- Shell prototype (navigation, basic panels, layout persistence)
- Windows Integration Layer (basic window enumeration)
- CI/CD pipeline (lint, build, test)
- Developer setup documentation

### Gate Criteria (Phase 1 → Phase 2)

| Criterion | Detail |
|-----------|--------|
| Shell renders and accepts user interaction | Basic panel system working |
| Layouts persist between sessions | Save/restore verified |
| CI/CD operational | All PRs pass automated checks |
| Architecture validated | Core module boundaries implemented as designed |
| First vertical slice defined | One domain (likely Apps) scoped for Phase 2 |

### Out of Scope for Phase 1

- AI features
- Plugin system
- Device integration
- Audio management
- Automation engine
- Production UI design

---

## 4. Phase 2 — Feature Expansion

**Goal:** Implement core domain services and introduce AI observation and suggestion.

**Status:** Not started

### Expected Deliverables

- Application service (discovery, launch, grouping)
- Window service (tracking, layout integration)
- Audio service (basic routing and volume)
- AI observer and pattern store
- AI suggestion UI (permission-gated)
- Basic automation service (user-approved workflows)
- Device service (initial device discovery)
- Integration tests for cross-domain flows

### Gate Criteria (Phase 2 → Phase 3)

| Criterion | Detail |
|-----------|--------|
| Core domains operational | Apps, windows, audio functional |
| AI observes and suggests | Permission model working end-to-end |
| At least one automation type works | User can approve and run an automation |
| Security review complete | Threat model validated against implementation |
| MVP feature set meets DoD | All Phase 2 deliverables pass Definition of Done |

---

## 5. Phase 3 — Maturity

**Goal:** Plugin platform, advanced features, polish, and public release preparation.

**Status:** Not started

### Expected Deliverables

- Plugin SDK and runtime
- Plugin registry (or GitHub-based distribution)
- Advanced automation (cross-domain, conditional)
- Phone/device deep integration
- Performance optimisation pass
- Accessibility audit and fixes
- Public release preparation (installer, updates, documentation)

### Gate Criteria (Phase 3 → Release)

| Criterion | Detail |
|-----------|--------|
| Plugin SDK documented and tested | Third-party developer can build a plugin |
| Performance budgets met | No perceptible lag in normal use |
| Security audit passed | No Critical or High vulnerabilities |
| Accessibility target met | WCAG level achieved (TBD) |
| User documentation complete | Setup, usage, and troubleshooting guides |

---

## 6. Future Considerations (Post-Release)

Not in current roadmap scope. Recorded for planning continuity:

- Cloud sync (opt-in)
- Multi-monitor advanced layouts
- Collaboration features
- macOS / Linux ports
- Mobile companion app
- Marketplace for plugins and themes
- Enterprise features

---

## 7. Roadmap Change Process

1. Product Owner proposes change
2. Impact assessment on current phase and resources
3. Decision Log entry if phase boundaries shift
4. Update this document
5. Communicate to all contributors

---

## Related Documents

- [Scope Management](../01-Product/SCOPE-MANAGEMENT.md)
- [Sprint Structure](../10-Sprints/SPRINT-STRUCTURE.md)
- [Decision Log](../09-Decisions/DECISION-LOG.md)
- [Open Questions](../09-Decisions/OPEN-QUESTIONS.md)
- [Risk Register](../07-Security/RISK-REGISTER.md)
