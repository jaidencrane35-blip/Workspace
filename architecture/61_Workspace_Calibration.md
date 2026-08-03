# 61 — Workspace Calibration

Status: Complete (Sprint 70)  
Date: 2026-08-03  
Branch: `v2-dev`  
Baseline: Sprint 69 commit `ebced20`  
Authority: `architecture/60_Workspace_Anticipation.md` · Version 2 docs `40`–`60`  
Implementation: `app/src/experience/workspaceCalibration.ts` · `anticipationPrediction.ts`

---

## 1. Objective

Continuously calibrate anticipation confidence against observed user behaviour so that prediction confidence reflects demonstrated accuracy rather than fixed heuristics alone.

Improves confidence calibration only.

Does not change Runtime behaviour. Does not execute actions. Does not alter navigation or persistence.  
Does not change prediction selection. No new resolver stage.

Evidence only — no recommendations.

---

## 2. Calibration schema

Type: `WorkspaceCalibration` (derived — not persisted)

| Field | Content |
|---|---|
| `calibrationId` | Content-addressed id (`calibrate-…`) |
| `predictionCount` | Scored prediction pairs |
| `confirmedPredictions` | Predictions matching next observed Moment |
| `missedPredictions` | Predictions not matching next observed Moment |
| `observedAccuracy` | confirmed / predictionCount |
| `confidenceCalibration` | Factor `0.5 + 0.5 * observedAccuracy` when active; else `1` |
| `reliabilityBand` | `insufficient` \| `low` \| `moderate` \| `high` |
| `evidenceLineage` | Evidence snapshot ids + tip |
| `replayLineage` | Replay session ids + bundle invocations |
| `anticipationLineageId` | Anticipation lineage reference |
| `architectureSnapshotIds` | Architecture snapshots from evolution |
| `reliabilityTrend` | Running accuracy after each scored pair |
| `active` | `true` only when validation passes |
| `validation` | Failure reasons + composition status |

No user content. No persistence. No AI-generated state.

---

## 3. Confidence calibration model

1. For consecutive evidence snapshots `(i, i+1)`, predict Moment from `i` via `predictAnticipationFromEvidence`.  
2. Observe Moment from `i+1` via hotspot / top hesitation.  
3. Score confirmed vs missed.  
4. When calibration active:  
   `calibratedConfidence = clamp01(rawConfidence * confidenceCalibration)`  
5. When inactive: anticipation keeps `rawConfidence`.

Anticipation fields unchanged by calibration:

- `likelyFocalRegion`  
- `likelyNextMoment`  
- `likelyContinuationTarget`  

Anticipation exposes: `rawConfidence`, `confidence` (calibrated), `calibrationId`, `reliabilityBand`.

Presentation pipeline (no new stage):

```
Runtime → Pack → Evolution → Presence → Anticipation → Presentation
```

Calibration is internal to anticipation confidence / readiness weighting only.

---

## 4. Validation

| Check | Failure |
|---|---|
| Evidence present | `no_evidence` |
| ≥ 2 scored pairs | `insufficient_pairs` |
| Replay lineage | `missing_replay` |
| Architecture snapshots | `missing_architecture_snapshot` |
| Anticipation lineage id | `missing_anticipation_lineage` |
| Composition / lineage | `composition_regressed` / `lineage_incomplete` |
| Accuracy collapse | `calibration_regressed` |

Invalid calibration remains inactive.  
Every active result references evidence, replay, anticipation lineage, and architecture snapshot.

Deterministic replay: `replayWorkspaceCalibration(store)`.

Confidence monotonicity: for fixed raw confidence, higher `observedAccuracy` ⇒ higher or equal calibrated confidence.

---

## 5. Replay / governance linkage

Reuses ExperienceEvidence, Replay history, WorkspaceAnticipation lineage, WorkspaceMemoryEvolution architecture snapshots, AdaptationComposition, governance primitives.

Authority:

```
40 → … → 60 → 61
```
