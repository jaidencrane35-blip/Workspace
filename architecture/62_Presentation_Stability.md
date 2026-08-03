# 62 — Presentation Stability

Status: Complete (Sprint 71)  
Date: 2026-08-03  
Branch: `v2-dev`  
Baseline: Sprint 70 commit `b032be5`  
Authority: `architecture/61_Workspace_Calibration.md` · Version 2 docs `40`–`61`  
Implementation: `app/src/experience/workspacePresentationStability.ts`

---

## 1. Objective

Improve the perceptual stability of Workspace so the environment feels calm, continuous, and trustworthy during prolonged use.

Refines behaviour. Does not add features. Does not alter Runtime behaviour, navigation, or persistence.

No AI autonomy. No new resolver stage. No new governance systems.

Evidence only — no recommendations.

---

## 2. Stability schema

Type: `WorkspacePresentationStability` (derived — not persisted)

| Field | Content |
|---|---|
| `stabilityId` | Content-addressed id (`stability-…`) |
| `presentationStabilityScore` | 0–1 aggregate calm/continuity |
| `environmentalStability` | Inverse friction variance |
| `motionContinuity` | Inverse replay divergence + motion variance |
| `focalStability` | Inverse hesitation-gap variance |
| `transitionConsistency` | Inverse abandon-transition variance |
| `presentationVariance` | Mean population variance of proxies |
| `varianceTrend` | Running variance by evidence prefix |
| `evidenceLineage` / `replayLineage` | Evidence + replay refs |
| `calibrationLineageId` | Calibration reference |
| `anticipationLineageId` | Anticipation reference |
| `architectureSnapshotIds` | Architecture snapshots |
| `motionDamping` / `transitionCadence` / `emphasisSmoothing` / `environmentalInterpolation` / `focalSettling` / `atmosphericContinuity` | Damping factors when active |
| `active` | `true` only when validation passes |
| `validation` | Failure reasons + composition status |

No user content. No persistence. No AI state.

---

## 3. Variance model

From consecutive ExperienceEvidence snapshots, proxy series:

| Proxy | Source metric |
|---|---|
| environmental | `meanFrictionScore` |
| focal | `topHesitationMedianGapMs` (normalized) |
| motion | `replayDivergenceRate` |
| transition | `abandonedFlowTotal` (normalized) |

`presentationVariance` = mean population variance of the four proxies.  
Stability scores = `1 - √variance` (clamped), combined into `presentationStabilityScore`.

When active, `applyStabilityToPresentation` lerps emphasis / environment / spacing / grouping toward identity and may damp expressive motion — never changes prediction selection or adaptation activation.

---

## 4. Resolver integration

Pipeline (no new stage):

```
Runtime → Pack → Evolution → Presence → Anticipation → Presentation
```

Stability is applied internally at the end of `resolvePresentationWithAnticipation` via `resolvePresentationWithStability`.

Invalid stability ⇒ presentation unchanged (inactive).

---

## 5. Validation

| Check | Failure |
|---|---|
| ≥ 2 evidence samples | `insufficient_samples` |
| Replay lineage | `missing_replay` |
| Architecture snapshots | `missing_architecture_snapshot` |
| Calibration lineage | `missing_calibration_lineage` |
| Composition / lineage | `composition_regressed` / `lineage_incomplete` |
| Variance collapse gate | `stability_regressed` |

Every active result references evidence, replay, calibration lineage, and architecture snapshot.

Deterministic replay: `replayPresentationStability(store)`.

---

## 6. Governance linkage

Reuses WorkspaceCalibration, WorkspaceAnticipation lineage, WorkspacePresence, WorkspaceMemoryEvolution, ExperienceEvidence, Replay history, AdaptationComposition, governance primitives.

Authority:

```
40 → … → 61 → 62
```
