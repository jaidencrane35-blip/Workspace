# Governance Model

| Field | Value |
|-------|-------|
| **Purpose** | Define how decisions are made, documented, and enforced across the Workspace project |
| **Owner** | Project Lead |
| **Dependencies** | [Project Constitution](PROJECT-CONSTITUTION.md) |
| **Update Process** | Update when roles are filled, decision processes change, or new governance domains emerge. Record material changes in the Decision Log. |

---

## 1. Governance Layers

Workspace governance operates at four layers. Lower layers cannot override higher layers.

```
┌─────────────────────────────────────┐
│  Constitution (non-negotiable)      │
├─────────────────────────────────────┤
│  Principles (Product, Arch, Eng, UX, │
│              AI, Security)            │
├─────────────────────────────────────┤
│  Decisions (recorded choices)       │
├─────────────────────────────────────┤
│  Implementation (code, configs)     │
└─────────────────────────────────────┘
```

---

## 2. Roles

| Role | Responsibility | Status |
|------|----------------|--------|
| Product Owner | Scope, priorities, user value | TBD |
| Project Lead | Overall delivery, constitution enforcement | Active (foundation phase) |
| Architect | System design, technical direction | TBD |
| Lead Software Engineer | Engineering standards, code quality, delivery process | Active (foundation phase) |
| UX Lead | Interaction design, usability standards | TBD |
| AI Lead | AI behaviour, learning models, permission flows | TBD |
| Security Lead | Threat model, security reviews | TBD |

Until roles are assigned, decisions in that domain are **blocked** or logged as [Open Questions](../09-Decisions/OPEN-QUESTIONS.md).

---

## 3. Decision Types

### Type A — Constitutional

Affects non-negotiable principles. Requires Project Lead + Product Owner approval and Decision Log entry.

### Type B — Strategic

Affects architecture, product direction, AI behaviour, or security model. Requires domain owner approval and Decision Log entry.

### Type C — Tactical

Affects implementation within approved architecture. Lead Software Engineer or delegated reviewer can approve. Log if it sets precedent.

### Type D — Operational

Sprint-level choices within approved scope. Documented in sprint notes. No Decision Log entry unless precedent-setting.

---

## 4. Decision Workflow

1. **Identify** — Recognise that a decision is needed. If ambiguous, add to Open Questions.
2. **Document** — Write options, trade-offs, and recommendation.
3. **Review** — Route to appropriate authority per decision type.
4. **Record** — Accepted decisions go to Decision Log with date, owner, and status.
5. **Implement** — Update dependent documents before or alongside code.
6. **Validate** — Confirm implementation matches the recorded decision.

---

## 5. Escalation

Escalate when:

- A decision spans multiple domains (e.g., AI + UX + Security)
- Contributors disagree on interpretation of principles
- A shortcut would violate constitution or principles
- Scope exceeds current roadmap phase

Escalation path: Contributor → Domain Lead → Project Lead → Product Owner

---

## 6. AI Contributor Governance

AI tools (including Cursor agents) are contributors. They must:

- Read constitution and relevant principles before acting
- Never silently change product direction
- Flag ambiguous requirements in Open Questions
- Follow Documentation Standards for all written output
- Not write production code during foundation phase unless explicitly authorised by sprint scope

---

## 7. Review Cadence

| Activity | Frequency |
|----------|-----------|
| Open Questions triage | Weekly (when active development begins) |
| Decision Log review | Monthly |
| Constitution and principles review | Quarterly |
| Risk Register review | Monthly |
| Roadmap review | Per phase gate |

---

## Related Documents

- [Project Constitution](PROJECT-CONSTITUTION.md)
- [Decision Log](../09-Decisions/DECISION-LOG.md)
- [Open Questions](../09-Decisions/OPEN-QUESTIONS.md)
- [Scope Management](../01-Product/SCOPE-MANAGEMENT.md)
