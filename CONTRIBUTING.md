# Contributing to Workspace

Thank you for contributing to Workspace. This repository is in its foundation phase. All contributors — human and AI — must follow the governance documents before writing production code.

---

## Before You Start

1. Read the [Project Constitution](docs/00-Constitution/PROJECT-CONSTITUTION.md).
2. Read [Engineering Principles](docs/03-Engineering/ENGINEERING-PRINCIPLES.md) and [Coding Standards](docs/03-Engineering/CODING-STANDARDS.md).
3. Check [Open Questions](docs/09-Decisions/OPEN-QUESTIONS.md) for unresolved decisions in your area.
4. Review the [Decision Log](docs/09-Decisions/DECISION-LOG.md) for prior architectural choices.

---

## Workflow

### Documentation Changes

- Follow [Documentation Standards](docs/03-Engineering/DOCUMENTATION-STANDARDS.md).
- Update the document metadata (Purpose, Owner, Dependencies, Update Process).
- Link related documents bidirectionally where appropriate.
- If a change affects product direction, architecture, UX, or AI behaviour, flag it in [Open Questions](docs/09-Decisions/OPEN-QUESTIONS.md) or add a Decision Log entry.

### Code Changes

- Keep implementation within approved architecture and sprint scope.
- Every change must satisfy the [Definition of Done](docs/03-Engineering/DEFINITION-OF-DONE.md).
- Follow [Repository Standards](docs/03-Engineering/REPOSITORY-STANDARDS.md) for branch naming, commits, and PRs.
- Record significant technical decisions in the [Decision Log](docs/09-Decisions/DECISION-LOG.md).

---

## Pull Requests

Use the PR template. Every PR must:

- Reference related issues or sprint items.
- Describe **what** changed and **why**.
- Confirm Definition of Done criteria are met.
- Note any new technical debt (see [Technical Debt Policy](docs/03-Engineering/TECHNICAL-DEBT-POLICY.md)).
- Flag decisions that require product or architecture review.

---

## Commits

- Write clear, imperative commit messages.
- Group related changes logically.
- Never force-push to shared branches.
- Never rewrite published history.

---

## Roles and Escalation

| Decision Type | Escalate To |
|---------------|-------------|
| Product scope or behaviour | Product Owner |
| Architecture | Architect / Tech Lead |
| UX patterns | UX Lead |
| AI behaviour and permissions | AI Lead + Product Owner |
| Security | Security Lead |

When a role is unfilled, document the question in [Open Questions](docs/09-Decisions/OPEN-QUESTIONS.md) and proceed only with explicit approval.

---

## Questions

If something is ambiguous, **identify it — do not guess.** Add it to [Open Questions](docs/09-Decisions/OPEN-QUESTIONS.md).
