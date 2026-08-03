# 50 — Longitudinal Adaptation Validation

Status: Complete (Sprint 59)  
Date: 2026-08-03  
Branch: `v2-dev`  
Baseline: Sprint 58 commit `d86eace`  
Authority: `architecture/49_Adaptation_Experiments.md` · Version 2 docs `40`–`49`  
Implementation: `app/src/experience/longitudinalAdaptation.ts`

---

## 1. Objective

Validate adaptation behaviour across multiple evidence generations.

An adaptation is not production-ready from a single before/after snapshot. It must demonstrate stable improvement over time. Evidence only — no recommendations.

---

## 2. Stability model

Type: `AdaptationStabilityReport` (`schemaVersion: 1`)  
Stored alongside adaptations in `ws.experience.adaptation.v1` (no separate evidence store).

Metrics are read from existing `ExperienceEvidence` snapshots. Values are not duplicated.

| Field | Definition |
|---|---|
| `adaptationId` | Target adaptation |
| `sampleCount` | Post-baseline evidence snapshots analysed |
| `stabilityScore` | `improvementConsistency × (1 − metricVariance) × (1 − regressionFrequency)` clamped to `[0,1]` |
| `confidenceTrend` | `rising` \| `stable` \| `falling` from confidence evolution halves (Δ > 0.05) |
| `regressionEvents` | Per-sample rollback criterion hits or expected-metric `regressed` verdicts |
| `rolloutDisposition` | `hold` \| `rollout_candidate` \| `reject` |
| `metricVariance` | Population variance of post-baseline metric values, normalised by `max(|baseline|, ε)²` |
| `improvementConsistency` | Fraction of samples meeting expected improvement delta vs baseline |
| `regressionFrequency` | `regressionEvents.length / sampleCount` |
| `confidenceEvolution` | Per-snapshot `min(1, sessionCount / 4)` |
| `evidenceTimeline` | Ordered evidence ids (baseline first) |

`LongitudinalAdaptationRecord` indexes the same timeline:

| Field | Meaning |
|---|---|
| `baselineEvidenceId` | First snapshot |
| `intermediateEvidenceIds` | Snapshots between baseline and latest |
| `latestEvidenceId` | Last snapshot |
| `observationCount` | Timeline length including baseline |
| `stabilityScore` | From latest report |
| `regressionCount` | `regressionEvents.length` |

---

## 3. Evidence requirements

| Requirement | Threshold |
|---|---|
| Minimum observations (incl. baseline) | `LONGITUDINAL_MIN_OBSERVATIONS = 3` |
| Minimum post-baseline samples | 2 |
| Evidence source | Existing `ExperienceEvidence` via `listEvidenceSnapshots` / append |
| Metric source | Adaptation `expectedMetric` / `expectedDirection` / `expectedImprovementDelta` |

Append path: `appendLongitudinalObservation(store, adaptation, evidence)`.  
Re-analyse path: `runLongitudinalValidation(store, adaptation)`.

---

## 4. Rollout thresholds

| Gate | Condition |
|---|---|
| Governance complete | `validationResult === passed` ∧ replay refs ∧ engineering ∧ proposal ∧ architecture snapshot |
| Stability threshold | `stabilityScore ≥ 0.7` (`LONGITUDINAL_STABILITY_THRESHOLD`) |
| Unresolved regressions | `regressionEvents.length ≤ 0` (`LONGITUDINAL_MAX_UNRESOLVED_REGRESSIONS`) |
| Evidence minimum | `evidenceTimeline.length ≥ 3` |

When all gates pass: `rolloutDisposition = rollout_candidate`.  
Otherwise: `hold` (insufficient evidence/stability) or `reject` (governance incomplete or regressions).

Promotion: `promoteToRolloutCandidate` sets `rolloutState = rollout_candidate` only when disposition is `rollout_candidate`. Never auto-activates.

---

## 5. Regression policy

| Event | Detection |
|---|---|
| Criterion hit | Same thresholds as `rollbackTriggered` on adaptation rollback criteria |
| Metric regression | `compareEvidence` verdict `regressed` on `expectedMetric` |
| Unresolved | Any regression event in the current timeline (no separate resolution ledger) |
| Demotion | If state is `rollout_candidate` and new observation introduces regressions → `candidate` |

Failed single-snapshot validation still maps active → `rolled_back`, otherwise → `inactive` (unchanged Sprint 57 behaviour).

---

## 6. Lifecycle extension

```
inactive
  → (single-snapshot validation passed) → candidate
  → (longitudinal gates passed, manual promote) → rollout_candidate
  → (manual activate) → active
  → (rollback / failed while active) → rolled_back
```

One new state only: `rollout_candidate`.  
`active` remains a manual developer action from `inactive` | `candidate` | `rollout_candidate`.

Authority chain:

```
40 → 41 → 42 → 43 → 44 → 45 → 46 → 47 → 48 → 49 → 50
```

No additional governance layers. No new adaptation primitives.
