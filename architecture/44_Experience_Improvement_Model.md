# 44 — Experience Improvement Model

Status: Complete (Sprint 53)  
Date: 2026-08-03  
Branch: `v2-dev`  
Baseline: Sprint 52 commit `d0cee7c`  
Authority: `architecture/43_Experience_Evidence_Model.md` · Version 2 docs `40`–`43`  
Implementation: `app/src/dev/experienceImprovement.ts` · DEV overlay `ExperienceEvidenceDashboard.tsx`

---

## 1. Objective

Complete the feedback loop from interaction traces → evidence snapshots → deterministic improvement opportunities.

This document records schemas, thresholds, comparison, and replay linkage. Evidence only — no design guidance and no AI-generated recommendations.

---

## 2. Opportunity schema

Type: `ExperienceOpportunity` (`schemaVersion: 1`)  
Detector: `detectOpportunities(evidenceList)`

| Field | Meaning |
|---|---|
| `opportunityId` | Stable id `opp-<fnv1a(metric\|workflow\|threshold)>` |
| `metric` | Key of `ExperienceEvidenceMetrics` |
| `workflow` | `save` \| `continue` \| `navigation` \| `confidence` \| `recovery` \| `replay` \| `friction` |
| `evidenceCount` | `sessionCount` from primary (latest) evidence |
| `severity` | `low` \| `medium` \| `high` from threshold ratio |
| `confidence` | `min(1, sessionCount / 5)` |
| `reproducibilityScore` | `1 − replayDivergenceRate` (clamped 0–1) |
| `supportingEvidenceIds` | Evidence snapshot ids (sorted) |
| `replaySessionIds` | Trace session ids from evidence (sorted; not duplicated) |
| `observedValue` | Metric value on primary evidence |
| `threshold` | Rule threshold |
| `excess` | Distance beyond threshold in the worse direction |

Primary evidence = last element of the caller-supplied timeline.  
Opportunities are omitted when `replaySessionIds` is empty (cannot demonstrate via replay).

No natural-language fields. No user content.

---

## 3. Evidence thresholds

Rules: `OPPORTUNITY_RULES`

| Metric | Workflow | Direction | Threshold | Gate |
|---|---|---|---:|---|
| `medianTimeToConfidenceMs` | confidence | lower better | 3000 | — |
| `meanFrictionScore` | friction | lower better | 0.25 | — |
| `medianFrictionScore` | friction | lower better | 0.25 | — |
| `frictionP75` | friction | lower better | 0.4 | — |
| `hesitationHotspotCount` | navigation | lower better | 2 | — |
| `topHesitationMedianGapMs` | navigation | lower better | 2000 | hotspot count ≥ 1 |
| `navigationLoopCount` | navigation | lower better | 1 | — |
| `abandonedSave` | save | lower better | 1 | — |
| `abandonedContinue` | continue | lower better | 1 | — |
| `recoverySuccessRate` | recovery | higher better | 0.5 | interruption count ≥ 1 |
| `replayDivergenceRate` | replay | lower better | 0 | replay count ≥ 1 and rate > 0 |

Severity from ratio (lower-better: `value/threshold`; higher-better: `threshold/value`):

| Ratio | Severity |
|---|---|
| &lt; 1.25 | low |
| &lt; 2 | medium |
| ≥ 2 | high |

Identical evidence input → identical opportunity list (sorted by severity rank, then `opportunityId`).

---

## 4. Comparison algorithm (baseline evolution)

Function: `evolveBaselines(snapshots)`  
Timeline = caller array order (store append order).  
Signal = first snapshot vs last snapshot per `COMPARABLE_METRICS`.

Longitudinal significance floors (`LONGITUDINAL_SIGNIFICANCE`):

| Metric | Absolute | Relative |
|---|---:|---:|
| `medianTimeToConfidenceMs` | 250 | 0.10 |
| `meanFrictionScore` | 0.03 | 0.08 |
| `medianFrictionScore` | 0.03 | 0.08 |
| `frictionP75` | 0.04 | 0.08 |
| `hesitationHotspotCount` | 1 | 0 |
| `topHesitationMedianGapMs` | 200 | 0.10 |
| `navigationLoopCount` | 1 | 0 |
| `abandonedFlowTotal` | 1 | 0 |
| `recoverySuccessRate` | 0.10 | 0.15 |
| `replayDivergenceRate` | 0.05 | 0 |

Floor = `max(abs, |first| × rel)`.

Verdict:

1. If `|last − first| ≤ floor` → `stable` (insignificant variation ignored)
2. Else lower-better and last &lt; first → `improving`
3. Else higher-better and last &gt; first → `improving`
4. Else → `degrading`

Summary counts: `improving` / `stable` / `degrading`.  
`improvements` / `regressions` arrays list non-stable metrics only.

Pairwise baseline vs latest (`compareEvidence`) remains available from Sprint 52.

---

## 5. Replay linkage

| Guarantee | Mechanism |
|---|---|
| Every opportunity links sessions | `replaySessionIds` required; empty → no emit |
| No duplicated replay storage | Ids reference Sprint 51 trace store only |
| Deterministic replay | Existing `replayStoredSession` / `replaySession` |
| Overlay one-click | DEV dashboard buttons per session id and per opportunity |

Replay results are not persisted as a second copy of traces.

---

## 6. Development workflow

1. DEV app with validation gate on (Sprint 51 default).
2. Collect human interaction traces; **Analyze traces** → evidence snapshots.
3. Open **Evidence** overlay (DEV dynamic import; improvement module lazy-loads on open).
4. Read **Opportunities** (threshold crossings + severity / confidence / reproducibility).
5. Use **Replay** links to reproduce supporting sessions.
6. Read **Baseline evolution** across the evidence timeline (improving / stable / degrading).
7. Optional: set baseline and compare latest pairwise.

Production: no route, no Runtime Core changes, no Experience behaviour changes.

---

## 7. Privacy guarantees

| Guarantee | Mechanism |
|---|---|
| No user content in opportunities | Metrics, enums, ids, numbers only |
| Forbidden keys | Sprint 51 `FORBIDDEN_EVENT_KEYS` still apply to traces; opportunities do not introduce free text |
| Local-only | Reuses local evidence + trace stores; no network APIs in `app/src/dev` |
| DEV visibility | Overlay + improvement engine behind `import.meta.env.DEV` lazy load |
| Production off | Validation gate closed in PROD unless explicit force |

---

## 8. Engineering boundary

- Reuses instrumentation, replay, and `ExperienceEvidence`.
- Shared hash helper: `devHash.ts` (`fnv1a`).
- Does not invent parallel event models.
- Does not alter production Experience or Runtime Core.
