# ADR-0004 - Complexity Budget

Status: Accepted

---

# Context

Software naturally accumulates complexity.

Complexity reduces maintainability and increases contextual drift.

---

# Decision

Every new subsystem must justify its existence.

Before adding a new subsystem ask:

Can an existing capability be extended?

If yes, prefer extension.

---

# Consequences

Smaller codebase.

Better maintainability.

Lower cognitive load.
