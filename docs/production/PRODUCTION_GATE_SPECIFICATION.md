# Canonical Production Gate Specification

| Field | Value |
| --- | --- |
| **Kind** | Binding schema for every production gate unit |
| **Status** | Authoritative under Sustainable Engineering Operations |
| **Program** | P16.PI4 |
| **Sequencing authority** | `PRODUCTION_DEPENDENCY_AUTHORITY.md` (order unchanged by this Spec) |
| **Machine catalog** | `production-gates-dependency.json` |
| **Human catalog** | `PRODUCTION_GATE_CATALOG.md` (generated) |
| **Does not amend** | Constitutional Specification v2 · EES v1 · Product Proof Rule |

This document is to production gates what the Constitutional Specification is to architecture:  
**a fixed shape every unit must satisfy** — not a roadmap, not optional prose.

---

## 1. Mandatory schema

Every production gate unit SHALL document all of the following fields. Missing any field ⇒ unit is **not implementable** under this Specification.

| Field | Meaning |
| --- | --- |
| **id** | Stable unit id (e.g. `A1-artifact-checksums`) |
| **gate** | Family A–F |
| **purpose** | Why this gate exists |
| **userValue** | What user problem disappears |
| **engineeringValue** | What engineering capability appears |
| **constitutionalJustification** | Why allowed under Spec (owners/laws — no new architecture) |
| **productionClassification** | ReadyNow · BlockedExternal · BlockedByGate · OptionalPolish · ReleaseOnly · Complete |
| **prerequisites** | Gate unit ids that must be Complete |
| **dependents** | Units that wait on this unit |
| **blockingConditions** | What prevents implementation now |
| **entryCriteria** | Conditions required before work starts |
| **exitCriteria** | Observable completion evidence |
| **verification** | How completion is proven (commands / artifacts) |
| **productionReadinessDelta** | Exact readiness change (Eng / Production / Release) |
| **regressionRisks** | What could break |
| **rollbackStrategy** | How to undo or disable if defective |
| **successMetrics** | Measurable success signals |
| **operationalAcceptance** | Human-trust checklist (“A user can…”) — see §2 |

---

## 2. Operational Acceptance (mandatory)

Verification passed ≠ a human would trust the feature.

Every unit SHALL include an **Operational Acceptance** list of concrete human statements.  
These are **not** Product Proof and **not** automated tests. They define *production-quality feel*.

Form: `A <role> can …` where role is typically `user`, and may be `release engineer` or `developer` for ReleaseOnly / engineering-facing units.

Example shape:

```
operationalAcceptance:
  - "A user can install without confusion"
  - "A user can recover after failure"
  - "A release engineer can regenerate checksums in one command"
```

A unit may be **Engineering Complete** while Operational Acceptance remains unchecked by the Product Owner.  
**Production Ready** for that unit requires Engineering Complete **and** Owner-affirmed Operational Acceptance (or explicit Owner waiver recorded in the unit’s execution report).

---

## 3. Readiness levels (unchanged)

| Level | Meaning |
| --- | --- |
| Engineering Complete | Code + verifiers pass |
| Production Ready | Operational Acceptance satisfied for the unit’s scope |
| Release Ready | Safe to distribute to end users at scale |

Green CI ⇒ Engineering Complete only.

---

## 4. Rules

1. **No implementation** of a unit until its catalog entry satisfies this schema (verifier green).  
2. **Do not change canonical order** in this program — order lives in Dependency Authority.  
3. **One unit per execution program** after Owner review.  
4. **No new meta-frameworks** beyond this Spec + Dependency Authority + catalog.  
5. Product Proof remains the product gate for capability feel; Operational Acceptance is for production-surface trust.

---

## 5. PI4 decision

| Question | Result |
| --- | --- |
| Does next Ready Now (`A1-artifact-checksums`) already satisfy this Spec? | **No** — was partial prose only |
| Action | **Normalize catalog only** |
| Implement A1 in PI4? | **No** |
| P17 / Spec / runtime architecture? | **No** |

Await Product Owner review before implementing any further gate unit.
