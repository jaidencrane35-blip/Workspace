# Sprint Structure

| Field | Value |
|-------|-------|
| **Purpose** | Define how sprints are planned, executed, reviewed, and closed for Workspace |
| **Owner** | Lead Software Engineer |
| **Dependencies** | [Scope Management](../01-Product/SCOPE-MANAGEMENT.md), [Definition of Done](../03-Engineering/DEFINITION-OF-DONE.md), [Roadmap](../08-Roadmap/ROADMAP.md) |
| **Update Process** | Update when sprint duration, ceremony, or tooling changes. |

---

## 1. Sprint Philosophy

Sprints provide a regular cadence for delivering incremental value, reviewing progress, and adapting plans. Workspace sprints prioritise quality and documentation over velocity.

---

## 2. Sprint Parameters

| Parameter | Value | Notes |
|-----------|-------|-------|
| **Duration** | 2 weeks | Adjustable by team agreement |
| **Planning** | Day 1 (Monday) | First day of sprint |
| **Review** | Last day (Friday, week 2) | Demo and stakeholder review |
| **Retrospective** | After review | Same day or following Monday |
| **Capacity** | TBD per team size | Include 15–30% debt budget |

---

## 3. Sprint Ceremonies

### 3.1 Sprint Planning

**When:** First day of sprint
**Duration:** 1–2 hours
**Participants:** All active contributors

**Agenda:**
1. Review sprint goal (aligned with roadmap phase)
2. Review backlog items prioritised by Product Owner
3. Break items into tasks with clear acceptance criteria
4. Estimate capacity and commit to sprint scope
5. Identify dependencies and risks
6. Assign owners

**Output:** Sprint plan document in `docs/10-Sprints/sprints/`

### 3.2 Daily Check-In

**When:** Daily (async acceptable for small/remote teams)
**Duration:** 15 minutes max

**Format:**
- What did I complete?
- What am I working on next?
- Any blockers?

Blockers that are decision-related go to [Open Questions](../09-Decisions/OPEN-QUESTIONS.md).

### 3.3 Sprint Review

**When:** Last day of sprint
**Duration:** 30–60 minutes
**Participants:** All contributors + Product Owner

**Agenda:**
1. Demo completed work against acceptance criteria
2. Product Owner accepts or rejects items
3. Review scope changes that occurred during sprint
4. Update roadmap if phase progress changed

### 3.4 Sprint Retrospective

**When:** After sprint review
**Duration:** 30–45 minutes
**Participants:** All contributors

**Agenda:**
1. What went well?
2. What did not go well?
3. What should we change?
4. Review open technical debt items
5. Assign action items for next sprint

---

## 4. Sprint Planning Document Template

Each sprint has a plan document stored at:

```
docs/10-Sprints/sprints/YYYY-MM-DD-sprint-NN.md
```

### Template

```markdown
# Sprint NN — YYYY-MM-DD to YYYY-MM-DD

## Sprint Goal
[One sentence describing the sprint's primary objective]

## Roadmap Phase
[Which roadmap phase this sprint contributes to]

## Committed Items

| ID | Item | Owner | Acceptance Criteria | Status |
|----|------|-------|---------------------|--------|
| 1  | ...  | ...   | ...                 | ...    |

## Stretch Items
[Items to pick up if committed items are complete]

## Dependencies
[External dependencies or decisions needed]

## Risks
[Known risks for this sprint]

## Capacity Notes
[Team availability, holidays, etc.]

## Sprint Review Notes
[Filled in during review]

## Retrospective Notes
[Filled in during retro]
```

---

## 5. Backlog Management

| Backlog Level | Location | Owner |
|---------------|----------|-------|
| Product backlog | GitHub Issues (milestones per phase) | Product Owner |
| Sprint backlog | Sprint plan document | Sprint team |
| Task level | GitHub Issues or project board | Task owner |

### Prioritisation

Product Owner prioritises the product backlog. Priority levels:

1. **Must have** — Sprint goal depends on it
2. **Should have** — Important but sprint succeeds without it
3. **Could have** — Stretch items
4. **Won't have** — Explicitly out of scope for this sprint

---

## 6. Sprint Rules

1. **No scope creep** — New work mid-sprint requires Product Owner approval and equivalent de-scoping
2. **Definition of Done is mandatory** — Incomplete items roll to next sprint, not marked done
3. **Document as you go** — Documentation updates are part of each item, not a separate task
4. **Blockers escalate fast** — Blockers unresolved after 24 hours escalate to Project Lead
5. **Decisions get logged** — Sprint-level decisions that set precedent go to Decision Log
6. **Debt is visible** — New technical debt is logged during the sprint, not after

---

## 7. Phase 0 Sprint Note

During the foundation phase (Phase 0), sprints focus exclusively on documentation, governance, and decision-making. No production code sprints until Phase 1 gate criteria are met.

---

## 8. Tooling (Future)

When the team grows, consider:

- GitHub Projects for sprint boards
- Milestone tracking per roadmap phase
- Automated sprint report generation
- Velocity tracking (informational, not a target)

Tooling selection is a team decision when needed.

---

## Related Documents

- [Scope Management](../01-Product/SCOPE-MANAGEMENT.md)
- [Definition of Done](../03-Engineering/DEFINITION-OF-DONE.md)
- [Roadmap](../08-Roadmap/ROADMAP.md)
- [Technical Debt Policy](../03-Engineering/TECHNICAL-DEBT-POLICY.md)
