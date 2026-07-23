# Definition of Done

| Field | Value |
|-------|-------|
| **Purpose** | Define the criteria that every deliverable must meet before it is considered complete |
| **Owner** | Lead Software Engineer |
| **Dependencies** | [Engineering Principles](ENGINEERING-PRINCIPLES.md), [Coding Standards](CODING-STANDARDS.md) |
| **Update Process** | Update when quality gates change. Changes require team agreement. |

---

## 1. Purpose

Definition of Done (DoD) is the quality contract for all Workspace deliverables. No item is complete until every applicable criterion is met. "Almost done" is not done.

---

## 2. Documentation Deliverables

Applies to all changes in `docs/`.

- [ ] Document includes required metadata (Purpose, Owner, Dependencies, Update Process)
- [ ] Content is accurate and aligned with constitution and principles
- [ ] Cross-references are valid and bidirectional where appropriate
- [ ] Writing follows [Documentation Standards](DOCUMENTATION-STANDARDS.md)
- [ ] `docs/README.md` index updated if documents were added or removed
- [ ] PR reviewed and approved
- [ ] No unresolved ambiguities introduced without logging in [Open Questions](../09-Decisions/OPEN-QUESTIONS.md)

---

## 3. Code Deliverables (Future)

Applies when implementation begins.

### 3.1 All Code Changes

- [ ] Implements only what is in approved scope
- [ ] Follows [Coding Standards](CODING-STANDARDS.md)
- [ ] Follows [Architecture Principles](../02-Architecture/ARCHITECTURE-PRINCIPLES.md)
- [ ] No secrets, credentials, or hardcoded paths
- [ ] Error handling is explicit — no silent failures
- [ ] Code reviewed and approved by at least one reviewer
- [ ] CI passes (lint, type-check, build)
- [ ] Commit messages follow [Repository Standards](REPOSITORY-STANDARDS.md)

### 3.2 New Features

All of the above, plus:

- [ ] Unit tests cover core behaviour and edge cases
- [ ] Integration tests for cross-module interactions (where applicable)
- [ ] Public API documented (module README or inline docs)
- [ ] UX aligns with [UX Principles](../04-UX/UX-PRINCIPLES.md)
- [ ] Permission checks for state-changing operations
- [ ] No new technical debt without [Technical Debt Policy](TECHNICAL-DEBT-POLICY.md) entry

### 3.3 Bug Fixes

All of section 3.1, plus:

- [ ] Regression test added (unless impractical — document why)
- [ ] Root cause documented in PR or issue

### 3.4 Refactoring

All of section 3.1, plus:

- [ ] No behaviour change (unless explicitly scoped)
- [ ] Existing tests still pass
- [ ] Improvement rationale documented in PR

---

## 4. AI Features (Additional Criteria)

Applies to any deliverable involving AI behaviour.

- [ ] Follows Observe → Learn → Suggest → Permission → Automate sequence
- [ ] No autonomous action without explicit user approval
- [ ] Suggestion UI is clear and dismissible
- [ ] Data handling complies with [Security Principles](../07-Security/SECURITY-PRINCIPLES.md)
- [ ] AI Lead or designated reviewer approved

---

## 5. Plugin API Changes (Additional Criteria)

- [ ] Backward compatibility considered and documented
- [ ] Plugin SDK documentation updated
- [ ] Sandbox and permission boundaries verified
- [ ] Example plugin updated if API surface changed

---

## 6. Security-Sensitive Changes (Additional Criteria)

- [ ] Threat model reviewed
- [ ] No new attack surface without mitigation
- [ ] Security Lead or security checklist reviewer approved

---

## 7. Sprint Item Completion

A sprint item is done when:

- [ ] All applicable DoD criteria above are met
- [ ] Product Owner accepted (for user-facing items)
- [ ] Merged to target branch
- [ ] Sprint board updated

---

## 8. Exemptions

Exemptions to DoD criteria require:

1. Written justification in PR or issue
2. Approval from Lead Software Engineer
3. Entry in [Technical Debt Policy](TECHNICAL-DEBT-POLICY.md) if debt is incurred
4. Plan to resolve exemption in a future sprint

Exemptions are not permanent.

---

## Related Documents

- [Engineering Principles](ENGINEERING-PRINCIPLES.md)
- [Coding Standards](CODING-STANDARDS.md)
- [Technical Debt Policy](TECHNICAL-DEBT-POLICY.md)
- [Sprint Structure](../10-Sprints/SPRINT-STRUCTURE.md)
