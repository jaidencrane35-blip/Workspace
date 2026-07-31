# Architectural Decision Records — Index

Status: Active
Authority: Index only (not a decision source of truth)

Accepted ADRs live in `architecture/decisions/`.
This document lists them for navigation. It does not restate ADR rationale, alternatives, or consequences.

New ADRs use `architecture/decisions/ADR_TEMPLATE.md`.

---

## Accepted ADRs

| ID | Title | Summary | File |
|----|-------|---------|------|
| ADR-0001 | Local First | Workspace is local-first; core function must work offline when feasible, with cloud optional. | [decisions/ADR-0001 Local First.md](decisions/ADR-0001%20Local%20First.md) |
| ADR-0002 | Integrate Before Reinvent | Custom software only where Workspace creates unique value; research common capabilities before building. | [decisions/ADR-0002 Integrate Before Reinvent.md](decisions/ADR-0002%20Integrate%20Before%20Reinvent.md) |
| ADR-0003 | Permission First | Request permission before meaningful actions; permissions are explicit, revocable, and explainable. | [decisions/ADR-0003 Permission First.md](decisions/ADR-0003%20Permission%20First.md) |
| ADR-0004 | Complexity Budget | Every new subsystem must justify its existence; prefer extending an existing capability. | [decisions/ADR-0004 Complexity Budget.md](decisions/ADR-0004%20Complexity%20Budget.md) |
| ADR-0005 | Single Responsibility | Each subsystem has one primary responsibility; resolve overlap by simplification. | [decisions/ADR-0005 Single Responsibility.md](decisions/ADR-0005%20Single%20Responsibility.md) |
| ADR-0006 | Explainability | Workspace must explain what it is doing, why, which permissions apply, and which capability is responsible. | [decisions/ADR-0006 Explainability.md](decisions/ADR-0006%20Explainability.md) |
| ADR-0007 | Engineering Documentation Outside Runtime | Engineering artifacts (blueprint, research, ledger, protocols, architecture) do not ship with runtime logic. | [decisions/ADR-0007 Engineering Documentation Outside Runtime.md](decisions/ADR-0007%20Engineering%20Documentation%20Outside%20Runtime.md) |
| ADR-0008 | AI Sessions Are Replaceable | No AI session is authoritative; authority resides in Blueprint, Current State, Ledger, Research Catalogue, and ADRs. | [decisions/ADR-0008 AI Sessions Are Replaceable.md](decisions/ADR-0008%20AI%20Sessions%20Are%20Replaceable.md) |

---

## Template

| Item | File |
|------|------|
| ADR template | [decisions/ADR_TEMPLATE.md](decisions/ADR_TEMPLATE.md) |
