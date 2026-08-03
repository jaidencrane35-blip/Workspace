# 53 — Adaptation Composition

Status: Complete (Sprint 62)  
Date: 2026-08-03  
Branch: `v2-dev`  
Baseline: Sprint 61 commit `39b30dc`  
Authority: `architecture/52_First_Production_Adaptation.md` · Version 2 docs `40`–`52`  
Implementation: `app/src/experience/adaptationComposition.ts`

---

## 1. Objective

Enable multiple independently-governed adaptations to coexist deterministically.

The resolver composes adaptations predictably without ordering ambiguity. Production behaviour may change only through already-governed adaptations. Evidence only — no recommendations.

---

## 2. Composition algorithm

Function: `composeAdaptations(adaptations)` → `CompositionResult`

1. Select composable set: `rolloutState === active` ∧ `validationResult === passed`.
2. Sort by `adaptationId` ascending — **not** registration / insertion order.
3. Assign priority ranks `0 … n−1` in that order (higher rank applied later).
4. Merge presentations via existing `resolvePresentationConfiguration` rules.
5. Analyse pairwise conflicts → `AdaptationConflictReport`.
6. Collect composition lineage identifiers (no duplicated payloads).

Empty active set → identity presentation; compatible; empty lineage.

---

## 3. Priority rules

| Rule | Definition |
|---|---|
| Key | `adaptationId` lexicographic ascending |
| Rank 0 | Applied first (lowest priority) |
| Rank n−1 | Applied last (highest priority) |
| Replace fields | Later rank wins (`priority_replace`) |
| Multiply fields | Product of scales, then clamp (`merge_multiply` / `clamp_range`) |

Input array order does not affect output.

---

## 4. Merge rules

Inherited from `resolvePresentationConfiguration` / `mergePresentation`:

| Field | Merge |
|---|---|
| `spacingScale` / `emphasisScale` / `environmentalWeight` | Multiply, clamp `[0.5, 1.5]` |
| `groupingTightness` | Replace when defined |
| `density` / `motionProfile` | Replace when defined |
| `cssVars` | Keywise replace |
| `appliedAdaptationIds` | Composition order list |

---

## 5. Conflict model

Type: `AdaptationConflictReport`

| Cause | When | Strategy |
|---|---|---|
| `overlapping_targets` | Shared `targetComponents` | `union_targets` |
| `contradictory_density` | Distinct defined densities | `priority_replace` |
| `contradictory_motion` | Distinct motion profiles | `priority_replace` |
| `contradictory_environment` | Distinct environmental weights | `merge_multiply` (+ `clamp_range` if product outside clamp before apply) |
| `contradictory_spacing` / `contradictory_emphasis` | Distinct scales | `merge_multiply` |
| `contradictory_grouping` | Distinct grouping | `priority_replace` |
| `contradictory_css_var` | Same CSS var, different values | `priority_replace` |

Every conflict records: cause, affected adaptation id pair (sorted), field, resolution strategy, winner id (when replace).

`compatible === true` iff `conflicts.length === 0`.  
Conflicts are never silent — overrides appear in the report.

---

## 6. Validation

Function: `validateComposition(store)` → `CompositionValidationReport`

| Check | Definition |
|---|---|
| Composed presentation | From `composeAdaptations` |
| Composed stability | Mean of per-adaptation longitudinal `stabilityScore` (0 if none) |
| Regressions | Adaptations with non-empty `regressionEvents` |
| Governance lineage | Proposal, engineering, architecture, replay refs present for each active |
| Traceability | `CompositionLineage` lists adaptation / proposal / engineering / evidence / architecture / replay ids |

`validationResult === passed` when governance intact and zero regression adaptations.  
Reported conflicts do not alone fail validation.

---

## 7. Governance linkage

Composition does not store adaptation state. It derives from:

- `WorkspaceAdaptation` (active set)
- Existing resolver
- `AdaptationStabilityReport` (optional scores)
- `EngineeringChangeRecord` / proposal / `ArchitectureSnapshot` ids via lineage

Authority chain:

```
40 → … → 52 → 53
```

No new adaptation primitives. No new lifecycle states. Runtime Core / navigation / persistence unchanged.
