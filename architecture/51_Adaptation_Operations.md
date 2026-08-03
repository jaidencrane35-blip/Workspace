# 51 — Adaptation Operations

Status: Complete (Sprint 60)  
Date: 2026-08-03  
Branch: `v2-dev`  
Baseline: Sprint 59 commit `be64b02`  
Authority: `architecture/50_Longitudinal_Adaptation_Validation.md` · Version 2 docs `40`–`50`  
Implementation: `app/src/experience/adaptationOperations.ts`

---

## 1. Objective

Deterministic operational tooling for managing large numbers of governed adaptations.

Engineering operations only. No production behaviour changes. Evidence only — no design recommendations.

---

## 2. Catalog schema

Type: `AdaptationCatalog` / `AdaptationCatalogEntry` (`schemaVersion: 1`)  
Derived view — not persisted. Built by `buildAdaptationCatalog(store)`.

| Field | Source |
|---|---|
| `adaptationId` | `WorkspaceAdaptation.adaptationId` |
| `proposalId` | `WorkspaceAdaptation.proposalId` |
| `engineeringChangeId` | `WorkspaceAdaptation.engineeringChangeId` |
| `lifecycleState` | `WorkspaceAdaptation.rolloutState` |
| `rolloutReady` | `isRolloutReady(adaptation, StabilityReport)` |
| `currentDisposition` | `AdaptationStabilityReport.rolloutDisposition` or `none` |
| `stabilityScore` | `LongitudinalAdaptationRecord.stabilityScore` |
| `evidenceCount` | `LongitudinalAdaptationRecord.observationCount` |
| `latestEvidenceSnapshotId` | Longitudinal latest → adaptation evidence → store tip |
| `latestArchitectureSnapshotId` | Adaptation architecture id if present, else latest valid snapshot |

`lifecycleDistribution` counts adaptations per `AdaptationRolloutState`.

Identifiers are referenced, not duplicated as nested governance payloads.

---

## 3. Batch validation

Function: `runBatchValidation(store, { adaptationIds? })`  
Report: `BatchValidationReport`

| Bucket | Meaning |
|---|---|
| `validated` | Result transitioned to / remained `passed` with state change applied |
| `failed` | Result `failed` — cause `rollback_triggered` or `improvement_unmet` |
| `unchanged` | Same validation result and rollout state — cause `identical_result` |
| `skipped` | Not evaluated — cause `missing_baseline`, `missing_after_evidence`, or `rolled_back` |

Algorithm:

1. Resolve baseline / after evidence for each selected adaptation (deterministic).
2. Compute all `validateAdaptationEvidence` outcomes in memory.
3. Commit all pending adaptation writes in **one** `saveAdaptationBundle` call.

No partial mutations: adaptations are not written until the full batch computation finishes.

---

## 4. Operational health

Type: `AdaptationOperationalHealth` (`schemaVersion: 1`)  
Derived by `deriveOperationalHealth(store)`. Counts only.

| Metric | Definition |
|---|---|
| `adaptationCount` | `listAdaptations` length |
| `rolloutBacklog` | `rollout_candidate` count + `candidate` with `isRolloutReady` |
| `validationBacklog` | `validationResult === pending` |
| `staleEvidence` | Longitudinal `latestEvidenceId` ≠ store tip evidence id |
| `expiredLongitudinalSamples` | Timeline ids missing from evidence store, or passed adaptation with `0 < observationCount < 3` |
| `orphanRolloutCandidates` | `rollout_candidate` missing lineage refs or architecture integrity invalid |
| `inactiveValidatedAdaptations` | `validationResult === passed` ∧ `rolloutState === inactive` |
| `lifecycleDistribution` | Same counters as catalog |

---

## 5. Lifecycle reporting

Catalog and health both expose `lifecycleDistribution` for:

`inactive` · `candidate` · `rollout_candidate` · `active` · `rolled_back`

Overlay surfaces catalog rows, health backlog summary, and last batch report. Lazy-loaded via `adaptationOperations` import. No production bundle impact.

---

## 6. Engineering workflow

```
buildAdaptationCatalog → inspect lifecycle / readiness / disposition
runBatchValidation → apply single-snapshot validation across set
deriveOperationalHealth → count backlogs and stale/expired/orphan conditions
```

Reuse only:

- `WorkspaceAdaptation`
- `LongitudinalAdaptationRecord`
- `AdaptationStabilityReport`
- `EngineeringChangeRecord` (via adaptation ids)
- `ArchitectureSnapshot`

Authority chain:

```
40 → 41 → 42 → 43 → 44 → 45 → 46 → 47 → 48 → 49 → 50 → 51
```

No additional governance layers. No AI autonomy. No Runtime Core / navigation / persistence changes.
