# Engineering Principles

| Field | Value |
|-------|-------|
| **Purpose** | Define how the engineering team builds, reviews, and maintains Workspace |
| **Owner** | Lead Software Engineer |
| **Dependencies** | [Project Constitution](../00-Constitution/PROJECT-CONSTITUTION.md), [Architecture Principles](../02-Architecture/ARCHITECTURE-PRINCIPLES.md) |
| **Update Process** | Lead Software Engineer proposes changes. Material changes require team review and Decision Log entry. |

---

## 1. Engineering Mission

Build a professional, maintainable, scalable codebase that future engineers and AI contributors can understand and extend without archaeology.

---

## 2. Core Principles

### 2.1 Documentation Before Implementation

No production code without prior documentation of intent. At minimum:

- What problem is being solved
- Which module owns the change
- How it interacts with existing boundaries
- What tests will verify it

During Phase 0, this means architecture and product docs. During implementation, this extends to module READMEs and API contracts.

### 2.2 Planning Before Coding

Every sprint item has a clear scope, owner, and acceptance criteria before work begins. No "start coding and figure it out."

### 2.3 Maintainability Before Speed

Readable, well-structured code beats fast delivery of brittle code. Deadlines do not override Definition of Done.

Score major new systems with the **Human Maintainability Standard (0–10)** in [AI Engineering Governance](../00-Governance/AI_ENGINEERING_GOVERNANCE.md). Target ≥ 8.

### 2.4 Scalability Before Convenience

Choose patterns that work at 100,000 lines, not patterns that are easy today and painful tomorrow.

### 2.5 Explicit Over Implicit

- Explicit types over dynamic typing where the language supports it
- Explicit error handling over silent failures
- Explicit dependencies over hidden coupling
- Explicit permissions over assumed trust

### 2.6 Small, Reviewable Changes

- Pull requests should be focused and reviewable in under 30 minutes
- Large changes are split into logical, sequential PRs
- Each PR has one clear purpose

### 2.7 Test What Matters

- Test behaviour, not implementation details
- Critical paths require automated tests
- See [Definition of Done](DEFINITION-OF-DONE.md)

### 2.8 No Silent Behaviour Changes

- Breaking changes are documented and communicated
- Feature changes require updated documentation
- Deprecations follow a documented timeline

### 2.9 Identify, Don't Guess

When requirements are ambiguous:

1. Stop
2. Document the ambiguity in [Open Questions](../09-Decisions/OPEN-QUESTIONS.md)
3. Wait for resolution
4. Never invent product behaviour

### 2.10 AI as Contributor

AI tools are subject to the same standards as human contributors:

- Must read governance documents
- Must not bypass review
- Must not silently change direction
- Must produce reviewable, documented output

---

## 3. Quality Gates

| Gate | When | Criteria |
|------|------|----------|
| Design review | Before implementation of new modules | Architecture doc or Decision Log entry approved |
| Code review | Every PR | At least one reviewer; no self-merge |
| CI pass | Every PR | Lint, type-check, tests pass |
| DoD check | Every PR | [Definition of Done](DEFINITION-OF-DONE.md) met |
| Security review | Security-sensitive changes | Security Lead or checklist review |

---

## 4. Technical Decision Hierarchy

When making engineering choices, prioritise in this order:

1. Constitution and principles compliance
2. Architecture alignment
3. Security requirements
4. Maintainability and readability
5. Performance requirements
6. Developer convenience

If convenience conflicts with maintainability, maintainability wins unless a Decision Log entry documents the exception.

---

## 5. Debt and Refactoring

- Technical debt is tracked, not hidden. See [Technical Debt Policy](TECHNICAL-DEBT-POLICY.md).
- Refactoring is planned work, not "when we have time"
- Boy Scout Rule: leave code better than you found it, within PR scope

---

## 6. Release and Versioning

Release strategy will be defined before first public release. Expectations:

- Semantic versioning for public APIs
- Changelog maintained
- Breaking changes in major versions with migration guides

---

## Related Documents

- [Coding Standards](CODING-STANDARDS.md)
- [Repository Standards](REPOSITORY-STANDARDS.md)
- [Documentation Standards](DOCUMENTATION-STANDARDS.md)
- [Definition of Done](DEFINITION-OF-DONE.md)
- [Technical Debt Policy](TECHNICAL-DEBT-POLICY.md)
