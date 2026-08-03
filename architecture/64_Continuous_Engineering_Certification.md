# 64 — Continuous Engineering Certification

Status: Complete (Sprint 73)  
Date: 2026-08-03  
Branch: `v2-dev`  
Baseline: Sprint 72 commit `5864c2c`  
Authority: `architecture/63_Architectural_Simplification.md` · Version 2 docs `40`–`63`  
Implementation: `app/src/dev/engineeringCertification.ts`

---

## 1. Objective

Transform Version 2 into a continuously self-verifying engineering system.

Every certification run deterministically proves that Workspace remains internally correct against a canonical invariant registry.

Engineering confidence only. No Runtime Core, Experience, presentation, navigation, persistence, or behavioural changes.

Evidence only — no recommendations.

---

## 2. Invariant schema

`EngineeringInvariant` (schemaVersion: 1):

| Field | Role |
|---|---|
| `invariantId` | Stable identifier |
| `authorityDocument` | Owning architecture authority doc |
| `scope` | `resolver` · `lineage` · `replay` · `storage` · `api` · `schema` · `authority` |
| `validationMethod` | Named method that reuses existing validators |
| `expectedResult` | Always `pass` |
| `owningSubsystem` | `authority` · `governance` · `evidence` · `adaptation` · `presentation` · `certification` · `complexity` · `integrity` |

Invariants are derived metadata only. Registry: `ENGINEERING_INVARIANTS` / `listEngineeringInvariants()`.

Examples covered:

- Resolver stage ordering (`inv-resolver-stage-count`)
- Governance / authority lineage completeness
- Replay determinism
- Certification list immutability shape
- Storage schema key compatibility
- Public API export compatibility
- Evidence schema key stability
- Architectural complexity zero-duplication

---

## 3. Certification runner

`runEngineeringCertification(store, options)` executes every invariant deterministically.

Produces `EngineeringCertificationReport`:

| Field | Contents |
|---|---|
| `passed` / `failed` / `skipped` | Sorted invariant ids |
| `results` | Per-invariant outcome, cause, duration, subsystem, authority |
| `durationMs` | Aggregate execution duration |
| `authorityReferences` | Distinct authority documents touched |
| `subsystems` | Distinct owning subsystems |
| `complete` | `true` iff `failed.length === 0` |

Failures report deterministic causes (`InvariantFailureCause`). Skips are allowed when store context is absent. No partial certification: `complete` requires zero failures.

History is an in-memory ring (max 20). No new persistence keys.

---

## 4. Comparison model

`compareEngineeringCertifications(previous, current)` compares certified outcomes only:

| Field | Meaning |
|---|---|
| `newlyFailed` | Failures present in current, absent in previous |
| `newlyResolved` | Failures present in previous, absent in current |
| `invariantsAdded` | Invariant ids present only in current |
| `invariantsRemoved` | Invariant ids present only in previous |

Implementation details are not compared.

---

## 5. Subsystem ownership

| Subsystem | Representative invariants |
|---|---|
| `authority` | Chain linkage, tip document |
| `governance` | Storage key constants |
| `evidence` | Evidence storage key |
| `adaptation` | Composition validation |
| `presentation` | Resolver stages, public API, replay |
| `certification` | Certification list shape |
| `complexity` | Zero duplication after simplification |
| `integrity` | ArchitectureGraph / store integrity |

`deriveSubsystemHealth(report)` aggregates passed / failed / skipped per subsystem.

---

## 6. Authority linkage

```
40 → … → 63 → 64
```

Reuse:

- Governance primitives (`contentAddressedId`, `storeArchitectureIntegrityValid`)
- `ArchitectureGraph` / `AUTHORITY_CHAIN`
- `AdaptationCertification` list shape checks
- Existing composition / complexity / barrel validators

No duplicated validators. No duplicated persistence. No public API changes.

---

## 7. Development tooling

DEV overlay (lazy-loaded `engineeringCertification`):

- Engineering certification report
- Invariant registry
- Certification history
- Baseline comparison
- Subsystem health

Derived only. Lazy-load only.
