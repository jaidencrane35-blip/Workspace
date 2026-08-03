# 52 — First Production Adaptation

Status: Complete (Sprint 61)  
Date: 2026-08-03  
Branch: `v2-dev`  
Baseline: Sprint 60 commit `ecd6f8a`  
Authority: `architecture/51_Adaptation_Operations.md` · Version 2 docs `40`–`51`  
Implementation: `app/src/experience/productionAdaptation.ts`

---

## 1. Objective

Exercise the complete governance system by shipping the first production presentation adaptation.

Not a new feature. First real activation of an evidence-backed adaptation via existing `activateAdaptation`. Evidence only — no subjective conclusions.

---

## 2. Selection

Procedure (`runFirstProductionAdaptation` → `selectProductionAdaptation`):

1. If catalog empty: run adaptation experiments to materialise governed candidates.
2. For each `passed` + (`candidate` | `rollout_candidate`) adaptation: extend longitudinal series to `LONGITUDINAL_MIN_OBSERVATIONS` via additional `ExperienceEvidence` generations (thresholds unchanged).
3. From `AdaptationCatalog`, keep entries that satisfy:
   - complete governance lineage
   - `isRolloutReady`
   - zero unresolved regressions
   - lifecycle `candidate` or `rollout_candidate`
4. Rank by `stabilityScore` desc, `adaptationId` asc; take exactly one.

If none qualify: outcome `blocked` with allowlisted `blockReasons` — no activation, thresholds not weakened.

### Selected adaptation (deterministic fixture run)

| Field | Value |
|---|---|
| `adaptationId` | `adapt-8290abed` |
| scopes | `environment`, `motion` |
| presentation | `environmentalWeight: 0.92`, `motionProfile: standard` |
| `expectedMetric` | `medianTimeToConfidenceMs` |
| `stabilityScore` | `1` |

---

## 3. Governance lineage

| Artefact | Id |
|---|---|
| Proposal | `prop-a7705a27` |
| Engineering record | `eng-2c86daba` |
| Architecture snapshot | `asnap-4f2f9156` |
| Replay sessions | `s-exp-baseline` |

Activation pathway (existing primitives only):

```
candidate → promoteToRolloutCandidate → rollout_candidate
         → activateAdaptation → active
```

Storage record: `ws.experience.adaptation.production.v1` (`ProductionActivationRecord`).

---

## 4. Activation evidence

| Field | Value |
|---|---|
| `outcome` | `activated` |
| `activatedAt` | `1700000000000` (test `now`) |
| `preEvidenceId` | `evd-5b4db69d` |
| `postEvidenceId` | `evd-e798b517` |
| `rollbackAvailable` | `true` |

---

## 5. Post-activation validation

| Check | Result |
|---|---|
| Expected metric | `medianTimeToConfidenceMs`: `380` → `320` (Δ `-60`) |
| Regression | `false` |
| Governance intact | `true` |
| Integrity valid | `true` |
| Rollback status | available; not exercised on success path |

Failure path (not taken on this run): `rollbackAdaptation` → outcome `rolled_back` with allowlisted `blockReasons`.

---

## 6. Authority chain

```
40 → 41 → 42 → 43 → 44 → 45 → 46 → 47 → 48 → 49 → 50 → 51 → 52
```

No additional lifecycle states. No additional governance layers. Runtime Core / navigation / persistence unchanged.
