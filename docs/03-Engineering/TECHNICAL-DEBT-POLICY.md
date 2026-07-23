# Technical Debt Policy

| Field | Value |
|-------|-------|
| **Purpose** | Define how technical debt is identified, recorded, prioritised, and resolved |
| **Owner** | Lead Software Engineer |
| **Dependencies** | [Engineering Principles](ENGINEERING-PRINCIPLES.md), [Definition of Done](DEFINITION-OF-DONE.md) |
| **Update Process** | Review quarterly. Update when debt categories or resolution processes change. |

---

## 1. Philosophy

Technical debt is a deliberate or accidental trade-off that speeds delivery today at the cost of increased effort tomorrow. Workspace does not pretend debt does not exist — it tracks, prioritises, and pays it down systematically.

Debt is acceptable when:

- It is **conscious** (deliberate decision, not accident)
- It is **documented** (recorded with context and impact)
- It is **bounded** (has a resolution plan and timeline)
- It is **approved** (Lead Software Engineer or Architect sign-off)

---

## 2. Debt Categories

| Category | Description | Example |
|----------|-------------|---------|
| **Deliberate** | Conscious shortcut with known trade-off | Ship MVP without full test coverage; add tests in next sprint |
| **Accidental** | Discovered during development or review | Coupling between modules that should be independent |
| **Bit rot** | Code that was fine but degraded as surroundings changed | Deprecated API still in use after Windows update |
| **Missing docs** | Implementation without adequate documentation | Module with no README or API docs |
| **Missing tests** | Behaviour without automated test coverage | Critical path tested manually only |
| **Architectural** | Structure that blocks future development | Monolith module that should be split |

---

## 3. Debt Register

All known technical debt is tracked in GitHub Issues with the `tech-debt` label.

### Issue Template Fields

```markdown
## Debt Description
[What is the debt?]

## Category
[Deliberate / Accidental / Bit rot / Missing docs / Missing tests / Architectural]

## Impact
[What happens if we don't fix this?]

## Introduced In
[PR, sprint, or date]

## Resolution Plan
[How and when will this be fixed?]

## Priority
[Critical / High / Medium / Low]
```

---

## 4. Priority Levels

| Priority | Criteria | Resolution Target |
|----------|----------|-------------------|
| **Critical** | Blocks development, causes bugs, or creates security risk | Current or next sprint |
| **High** | Significantly slows development or increases bug risk | Within 2 sprints |
| **Medium** | Causes inconvenience but has workarounds | Within 1 phase |
| **Low** | Cosmetic or minor maintainability concern | Backlog; address when touching related code |

---

## 5. When Debt Is Incurred

### During Development

1. Developer identifies debt during implementation
2. Creates `tech-debt` issue with full template
3. References issue in PR description
4. Lead Software Engineer approves or rejects the trade-off

### During Code Review

1. Reviewer identifies debt
2. Author creates `tech-debt` issue or fixes inline
3. PR cannot merge with unapproved Critical or High debt

### During Retrospective

1. Team reviews open debt items
2. Prioritises items for upcoming sprints
3. Assigns owners and target sprints

---

## 6. Debt Budget

Each sprint allocates capacity for debt resolution:

| Phase | Debt Budget |
|-------|-------------|
| Early development (Phase 1–2) | Up to 30% of sprint capacity |
| Active feature development | At least 15% of sprint capacity |
| Maintenance / mature product | At least 20% of sprint capacity |

Debt budget is not optional. Sprints that skip debt work require Lead Software Engineer approval.

---

## 7. Prohibited Debt

The following are never acceptable, even as deliberate debt:

- Skipping permission checks for AI or automation
- Committing secrets or credentials
- Disabling security controls
- Removing error handling to ship faster
- Bypassing code review
- Ignoring failing tests

---

## 8. Debt Review Cadence

| Activity | Frequency |
|----------|-----------|
| New debt triage | Each PR review |
| Open debt review | Sprint retrospective |
| Debt register audit | Monthly |
| Architectural debt review | Quarterly |

---

## Related Documents

- [Engineering Principles](ENGINEERING-PRINCIPLES.md)
- [Definition of Done](DEFINITION-OF-DONE.md)
- [Risk Register](../07-Security/RISK-REGISTER.md)
