# Project Constitution

| Field | Value |
|-------|-------|
| **Purpose** | Define the non-negotiable rules, values, and boundaries that govern all work on Workspace |
| **Owner** | Project Lead |
| **Dependencies** | None — this is the root governance document |
| **Update Process** | Requires explicit approval from Project Lead and Product Owner. Changes must be recorded in the [Decision Log](../09-Decisions/DECISION-LOG.md). All contributors must be notified of amendments. |

---

## 1. Mission

Workspace is an adaptive desktop workspace that brings together applications, windows, devices, audio, automation, and AI into one unified operating environment.

**Mission statement:** *One workspace that brings your PC, phone, audio, and apps together.*

---

## 2. Long-Term Goal

Users should eventually feel:

- *"Everything is finally organised."*
- *"My desktop works the way I want it to."*

The software adapts to the user. The user never adapts to the software.

---

## 3. Relationship to Windows

Workspace **must not** replace Windows.

Workspace **must** make Windows feel like the operating system people always wished it was.

All design, architecture, and implementation decisions must respect Windows as the underlying platform unless explicitly superseded by a recorded decision.

---

## 4. Non-Negotiable Principles

These principles cannot be overridden by convenience, speed, or feature pressure.

| # | Principle |
|---|-----------|
| 1 | **The user is always in control.** |
| 2 | **Everything should be customizable.** |
| 3 | **Modes are presets, not restrictions.** |
| 4 | **Navigation locations remain consistent.** |
| 5 | **Panels can be resized and moved.** |
| 6 | **Layouts can be saved and can evolve.** |
| 7 | **Workspace remembers workflows.** |
| 8 | **Workspace reduces repetitive actions.** |
| 9 | **Architecture takes priority over shortcuts.** |
| 10 | **Documentation before implementation.** |
| 11 | **Planning before coding.** |
| 12 | **Maintainability before speed.** |
| 13 | **Scalability before convenience.** |

---

## 5. AI Governance

The AI subsystem must follow this sequence without exception:

```
Observe → Learn → Suggest → Receive Permission → Automate
```

The AI **never** performs actions without explicit user permission.

Examples of correct behaviour:

- *"I noticed you always open Discord after Steam. Would you like me to automate that?"*
- *"I noticed Spotify is usually reduced to 20% when Discord voice chat starts. Would you like me to save this as an automation?"*

Any proposal to change this sequence requires a Decision Log entry and Product Owner approval.

---

## 6. Decision Authority

| Domain | Authority |
|--------|-----------|
| Product scope and behaviour | Product Owner |
| Architecture | Architect / Tech Lead |
| UX patterns and interaction design | UX Lead |
| AI behaviour and permissions | AI Lead + Product Owner |
| Security model | Security Lead |
| Engineering standards | Lead Software Engineer |

When a role is unfilled, the decision **must not** be made silently. It must be logged in [Open Questions](../09-Decisions/OPEN-QUESTIONS.md) and resolved before implementation proceeds.

---

## 7. What This Constitution Prohibits

- Silently changing product direction
- Implementing features without documented scope approval
- Bypassing user permission for AI or automation actions
- Shipping code without meeting [Definition of Done](../03-Engineering/DEFINITION-OF-DONE.md)
- Accumulating undocumented technical debt
- Guessing when requirements are ambiguous

---

## 8. Engineering Values

Optimise for, in order of equal priority unless a recorded decision states otherwise:

1. Professionalism
2. Maintainability
3. Scalability
4. Performance
5. Security
6. Documentation
7. Readability
8. Future contributors
9. AI collaboration

---

## 9. Scale Expectation

This project is expected to exceed 100,000 lines of production code over its lifetime. Every document, standard, and structural decision made during the foundation phase must remain valid at that scale.

---

## 10. Amendment Process

1. Propose amendment via Decision Request issue or pull request.
2. Document rationale and impact on dependent documents.
3. Obtain approval from Project Lead and affected domain owners.
4. Record in [Decision Log](../09-Decisions/DECISION-LOG.md).
5. Update all dependent documents in the same change set where possible.

---

## Related Documents

- [Product Vision](../01-Product/PRODUCT-VISION.md)
- [Engineering Principles](../03-Engineering/ENGINEERING-PRINCIPLES.md)
- [AI Principles](../05-AI/AI-PRINCIPLES.md)
- [Decision Log](../09-Decisions/DECISION-LOG.md)
