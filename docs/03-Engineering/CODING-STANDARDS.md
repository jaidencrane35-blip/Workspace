# Coding Standards

| Field | Value |
|-------|-------|
| **Purpose** | Define language-agnostic and language-specific coding conventions for Workspace |
| **Owner** | Lead Software Engineer |
| **Dependencies** | [Engineering Principles](ENGINEERING-PRINCIPLES.md), [Architecture Principles](../02-Architecture/ARCHITECTURE-PRINCIPLES.md) |
| **Update Process** | Update when technology stack is selected and as conventions evolve. Language-specific sections added after stack decision. |

---

## 1. Status

**Technology stack not yet selected.** This document defines universal standards applicable regardless of language. Language-specific conventions will be added as an appendix when the stack is decided (see [Open Questions](../09-Decisions/OPEN-QUESTIONS.md)).

---

## 2. Universal Standards

### 2.1 Naming

| Element | Convention | Example |
|---------|------------|---------|
| Modules / packages | `kebab-case` or language convention | `domain-audio`, `domain_audio` |
| Classes / types | `PascalCase` | `WindowService`, `LayoutState` |
| Functions / methods | `camelCase` or language convention | `getActiveLayout()` |
| Constants | `SCREAMING_SNAKE_CASE` or language convention | `MAX_PANEL_COUNT` |
| Files | Match primary export; `kebab-case` for files | `window-service.ts` |
| Interfaces | `PascalCase`; no `I` prefix | `LayoutProvider` not `ILayoutProvider` |
| Enums | `PascalCase` type, `PascalCase` or `SCREAMING_SNAKE_CASE` members | `PermissionLevel.ReadOnly` |

Names must be descriptive. Avoid abbreviations except widely understood ones (`id`, `url`, `api`).

### 2.2 File Organisation

- One primary export per file (exceptions for tightly coupled types)
- Group by feature/module, not by type (prefer `domain-audio/routing/` over `services/`)
- Colocate tests with source or in parallel `tests/` directory per package convention
- No file exceeds 400 lines without documented justification

### 2.3 Functions and Methods

- Single responsibility — one function, one job
- Maximum ~40 lines; extract if longer
- Maximum 4 parameters; use an options object beyond that
- No side effects in functions named as queries (e.g., `get*` must not mutate state)
- Early returns over deep nesting

### 2.4 Error Handling

- Never swallow errors silently
- Use typed errors where the language supports it
- Error messages must be actionable ("Failed to load layout: file not found at {path}")
- Log errors with context (module, operation, relevant IDs)
- User-facing errors must not expose internal details

### 2.5 Comments

- Code should be self-explanatory
- Comments explain **why**, not **what**
- Required comments:
  - Non-obvious business logic
  - Workarounds (with issue/ticket reference)
  - Public API documentation
- Prohibited comments:
  - Restating the code
  - Commented-out code (delete it; git has history)
  - TODO without issue reference

### 2.6 Dependencies

- Explicit imports; no wildcard imports in production code
- Dependencies declared in package manifest, not assumed
- No circular dependencies between packages
- Third-party dependencies require justification and license check

### 2.7 State Management

- Minimise mutable state
- State ownership must be clear (one module owns each piece of state)
- No shared mutable global state
- State changes emit events for observers (AI, automation, UI)

### 2.8 Security in Code

- No secrets in source code
- No string concatenation for queries or commands
- Validate all external input at system boundaries
- Permission checks at the point of action, not just at entry

### 2.9 Performance

- No premature optimisation
- Profile before optimising
- Avoid blocking operations on UI/render paths
- Lazy-load heavy modules

### 2.10 Formatting

- Automated formatter enforced via CI (specific tool TBD with stack selection)
- No formatting debates in code review — formatter is authority
- Line length: 100 characters (soft limit; break for readability)

---

## 3. Prohibited Patterns

| Pattern | Reason |
|---------|--------|
| Magic numbers/strings | Use named constants |
| Copy-paste code blocks | Extract shared logic |
| God classes / modules | Split by responsibility |
| Catch-all error handlers | Handle specific errors |
| `@ts-ignore` / equivalent without comment | Fix the type error or document why |
| Direct OS calls outside integration layer | Breaks testability and portability |
| Hardcoded paths | Use configuration |
| Feature logic in UI components | Delegate to domain services |

---

## 4. Language-Specific Standards

_To be added when technology stack is decided._

Placeholder sections:

- [ ] TypeScript / JavaScript conventions
- [ ] Rust conventions (if applicable)
- [ ] C# / .NET conventions (if applicable)
- [ ] Python conventions (if applicable for tooling)

---

## 5. Code Review Checklist

Reviewers verify:

- [ ] Follows naming and formatting conventions
- [ ] No prohibited patterns
- [ ] Error handling is explicit and logged
- [ ] No secrets or hardcoded credentials
- [ ] Tests cover new behaviour
- [ ] Documentation updated if public API changed
- [ ] No scope creep beyond PR description
- [ ] Permission checks present for state-changing operations

---

## Related Documents

- [Engineering Principles](ENGINEERING-PRINCIPLES.md)
- [Repository Standards](REPOSITORY-STANDARDS.md)
- [Definition of Done](DEFINITION-OF-DONE.md)
