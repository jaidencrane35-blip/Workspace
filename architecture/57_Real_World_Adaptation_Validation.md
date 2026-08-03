# 57 — Real-World Adaptation Validation

Status: Complete (Sprint 66)  
Date: 2026-08-03  
Branch: `v2-dev`  
Baseline: Sprint 65 commit `27d05c1`  
Authority: `architecture/56_Governance_Consolidation.md` · Version 2 docs `40`–`56`  
Implementation: `app/src/experience/adaptationValidation.ts`

---

## 1. Objective

Exercise the complete adaptation platform using locally accumulated ExperienceEvidence.

Primary input: longitudinal evidence already present in development stores.  
No Runtime Core changes. No persistence schema changes. No new governance / evidence / adaptation abstractions.

Evidence only — no recommendations.

---

## 2. Evidence inventory model

Function: `buildEvidenceInventory(store)`

| Field | Derivation |
|---|---|
| `evidenceId` | `ExperienceEvidence.evidenceId` |
| `sequenceIndex` | Append order in evidence store |
| `timestampMs` | Certification `certifiedAt` for this snapshot, else production `activatedAt` when pre/post matches; else `null` |
| `interactionCount` | `metrics.sessionCount` (session-derived proxy; no raw events) |
| `replayCount` | `metrics.replayCount` |
| `bundleReplayInvocations` | Evidence bundle replay counter |
| `activeAdaptationIds` | Currently `active` adaptations referencing the snapshot via lineage / longitudinal timeline |
| `certificationId` | Latest certification freezing this `evidenceSnapshotId` |
| `fingerprint` / `tag` | Opaque evidence identifiers only |

No user content is read. Inventory is ephemeral — not persisted.

---

## 3. Performance report schema

Function: `buildAdaptationPerformanceReport(store)` → `AdaptationPerformanceReport`

### Per active adaptation / certified pack (`AdaptationPerformanceEntry`)

| Field | Source |
|---|---|
| `activationCount` | Current `active` state and/or production activation record |
| `observationCount` | Longitudinal record |
| `stabilityScore` | Stability report / longitudinal / pack composed score |
| `stabilityTrend` | `confidenceTrend` (`rising` \| `stable` \| `falling` \| `unknown`) |
| `improvementPersistence` | `improvementConsistency` |
| `regressionFrequency` | Stability report or regressionCount/observationCount |
| `confidenceEvolution` | Stability report series |
| `evidenceGrowth` | Distinct evidence ids on timeline / pack summary |
| `rolloutSuccess` | Active / rollout_candidate / certified+active pack |
| `certificationIds` | Covering certifications |

Derived only from existing stores. No duplicate persistence.

---

## 4. Pack comparison methodology

Function: `buildPackEffectivenessReport(store)` → `PackEffectivenessReport`

Cohorts:

1. **single_adaptations** — performance rows for each currently active adaptation  
2. **certified_packs** — performance rows for each certified pack

Measured only:

| Measure | Definition |
|---|---|
| Stability | Mean `stabilityScore` |
| Regression rate | Mean `regressionFrequency` |
| Evidence growth | Sum of `evidenceGrowth` |
| Rollout success | Count / rate of `rolloutSuccess` |

`delta` = pack cohort − single cohort (nullable when a side lacks samples).  
No subjective quality metrics.

---

## 5. Longitudinal validation

| Report | Function | Content |
|---|---|---|
| Longitudinal trend | `buildLongitudinalTrendReport` | Per-snapshot sessionCount, friction, TTC, replayCount in append order |
| Certification longevity | `buildCertificationLongevityReport` | `certifiedAt` gaps along certification chain |

Trend points reuse evidence metrics already shown in the DEV evidence timeline / evolution panels.

---

## 6. Known evidence limitations

| Limitation | Fact |
|---|---|
| No wall clock on evidence | Snapshots omit timestamps; inventory uses cert/production clocks or `null` |
| Interaction proxy | `sessionCount` stands in for interaction volume; raw events are not stored in evidence |
| Production activation history | Single production record — not a multi-activation ledger |
| Synthetic vs human | Local stores may still contain development/synthetic sessions; harvest does not classify provenance |
| Overlapping cohorts | Active adaptations may also belong to packs; comparison is pathway-level, not disjoint sets |

---

## 7. Development tooling

DEV overlay (`ExperienceEvidenceDashboard`) lazy-loads `adaptationValidation` and surfaces:

- evidence inventory  
- adaptation performance  
- pack effectiveness  
- longitudinal trend series  
- certification longevity  

No new mount host. Lazy-load only when the overlay opens.

---

## 8. Authority linkage

```
40 → … → 56 → 57
```

Governance platform unchanged. Reports are read-only derivations.
