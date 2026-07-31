# ADR-0008 - AI Sessions Are Replaceable

Status: Accepted

---

# Context

Individual AI conversations experience contextual drift and have finite context windows.

---

# Decision

No AI session is considered authoritative.

Authority resides in project artifacts:

- Blueprint
- Current State
- Engineering Ledger
- Research Catalogue
- ADRs

New AI sessions shall be able to continue development using only these artifacts and the latest implementation output.

---

# Consequences

Reduced dependency on conversation history.

Improved continuity.

Long-term engineering stability.
