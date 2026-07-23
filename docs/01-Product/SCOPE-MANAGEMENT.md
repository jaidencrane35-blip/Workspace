# Scope Management

| Field | Value |
|-------|-------|
| **Purpose** | Define how scope is proposed, evaluated, approved, and controlled throughout the project lifecycle |
| **Owner** | Product Owner (TBD) |
| **Dependencies** | [Product Vision](PRODUCT-VISION.md), [Project Constitution](../00-Constitution/PROJECT-CONSTITUTION.md), [Roadmap](../08-Roadmap/ROADMAP.md) |
| **Update Process** | Product Owner updates when scope processes change. Phase boundary changes require Decision Log entry. |

---

## 1. Scope Philosophy

Scope is a contract between product intent and engineering capacity. Workspace prioritises:

1. **Correctness over completeness** — Ship less, but ship what meets Definition of Done.
2. **Documented over assumed** — If it is not in scope documents, it is out of scope.
3. **Phased over monolithic** — Deliver value incrementally through roadmap phases.
4. **Controlled over reactive** — Scope changes follow a defined process, not ad-hoc additions.

---

## 2. Scope Hierarchy

```
Roadmap Phase Scope
    └── Sprint Scope
        └── Task / Story Scope
            └── PR Scope
```

Each level must fit within the level above. A PR must not introduce work outside its sprint scope without approval.

---

## 3. Scope Sources

| Source | Governs |
|--------|---------|
| [Project Constitution](../00-Constitution/PROJECT-CONSTITUTION.md) | Absolute boundaries |
| [Product Vision](PRODUCT-VISION.md) | Product direction |
| [Roadmap](../08-Roadmap/ROADMAP.md) | Phase-level deliverables |
| Sprint plan | Sprint-level deliverables |
| [Decision Log](../09-Decisions/DECISION-LOG.md) | Recorded scope decisions |
| [Open Questions](../09-Decisions/OPEN-QUESTIONS.md) | Unresolved scope items (blocked) |

---

## 4. Scope Change Process

### Minor Change

Affects sprint tasks but not sprint goal. Approved by Product Owner or delegate within 24 hours.

### Major Change

Affects sprint goal, phase deliverables, or product direction. Requires:

1. Written proposal with rationale and impact
2. Product Owner approval
3. Engineering feasibility assessment
4. Decision Log entry if precedent-setting
5. Roadmap update if phase boundaries shift

### Constitutional Change

Affects non-negotiable principles. See constitution amendment process.

---

## 5. Scope Creep Prevention

Contributors must ask before implementing:

- Features not in the current sprint plan
- Behaviour not described in product or UX documents
- Integrations not approved in architecture documents
- AI capabilities beyond the Observe → Learn → Suggest → Permission → Automate model

If in doubt, add to [Open Questions](../09-Decisions/OPEN-QUESTIONS.md) and stop.

---

## 6. Phase Gate Criteria

A roadmap phase cannot begin until the prior phase gate is met:

| Gate | Requirement |
|------|-------------|
| Phase 0 → Phase 1 | Documentation foundation complete; open questions triaged; tech stack decision recorded |
| Phase 1 → Phase 2 | Core shell architecture approved; CI/CD operational; first vertical slice defined |
| Phase 2 → Phase 3 | MVP feature set meets Definition of Done; security review complete |

Specific gate criteria are defined in the [Roadmap](../08-Roadmap/ROADMAP.md) and updated per phase.

---

## 7. De-Scoping Rules

When capacity is insufficient:

1. Product Owner prioritises must-have vs. nice-to-have
2. De-scoped items return to backlog, not deleted silently
3. Sprint retrospective documents what was de-scoped and why
4. Technical debt from de-scoping shortcuts must be logged

---

## 8. AI and Automation Scope

AI features are in scope only when:

- They follow [AI Principles](../05-AI/AI-PRINCIPLES.md)
- Permission flows are designed and reviewed
- Data handling complies with [Security](../07-Security/SECURITY-PRINCIPLES.md) principles
- Product Owner has approved the specific capability

Autonomous action without permission is **never** in scope.

---

## Related Documents

- [Product Vision](PRODUCT-VISION.md)
- [Roadmap](../08-Roadmap/ROADMAP.md)
- [Sprint Structure](../10-Sprints/SPRINT-STRUCTURE.md)
- [Definition of Done](../03-Engineering/DEFINITION-OF-DONE.md)
