# 49 — Adaptation Experiments

Status: Complete (Sprint 58)  
Date: 2026-08-03  
Branch: `v2-dev`  
Baseline: Sprint 57 commit `92fb9ff`  
Authority: `architecture/48_Adaptive_Workspace.md` · Version 2 docs `40`–`48`  
Implementation: `app/src/experience/adaptationExperiments.ts`

---

## 1. Objective

Exercise the complete governance pipeline by introducing the first production presentation adaptations.

These are not new features. They are evidence-backed presentation refinements using `WorkspaceAdaptation`. Evidence only — no subjective conclusions.

---

## 2. Selection

At Sprint 58 selection time, the repository store contained **no** pre-existing approved `ExperienceChangeProposal` records (proposals are runtime governance artefacts).

Selection procedure (`runAdaptationExperiments`):

1. Call `selectEligibleProposals` on the governance store.
2. If none: materialise lineage from fixture traces through  
   Interaction → Evidence → Opportunity → Proposal (accepted) → Engineering Record (architecturally_accepted) → Architecture Snapshot (integrity.valid).
3. Rank `ADAPTATION_EXPERIMENT_SPECS` by expected improvement (desc), regression risk (asc); take ≤3.

Selection note recorded in experiment summary storage (`ws.experience.adaptation.experiments.v1`).

Observed selection note (deterministic fixture run):

> No approved ExperienceChangeProposals were present in the store at selection time. Lineage was materialised through the full governance pipeline from fixture interaction traces (baseline → opportunities → proposal → engineering → architecture snapshot), then experiments were selected against that approved proposal.

---

## 3. Experiments

| experimentKey | scopes | presentation | expectedMetric | expectedImprovementDelta | direction | regressionRisk |
|---|---|---|---|---:|---|---:|
| `environment_quiet` | environment, motion | environmentalWeight 0.92; motion standard | `medianTimeToConfidenceMs` | 200 | lower_better | 2 |
| `spacing_tighten` | spacing, grouping | spacingScale 0.94; groupingTightness 0.62 | `meanFrictionScore` | 0.02 | lower_better | 1 |
| `density_balanced` | density, emphasis | density balanced; emphasisScale 1.03 | `meanFrictionScore` | 0.015 | lower_better | 2 |

Targets are shell/canvas/home only. No workflow, navigation, Runtime Core, persistence, or domain model changes.

Each resulting `WorkspaceAdaptation` is individually toggleable (`toggleAdaptationExperiment`).

---

## 4. Evidence lineage (shared)

Fixture sessions:

- Baseline: `s-exp-baseline` (high-friction navigate/abandon pattern)
- After: `s-exp-after` (save-success path)

Shared lineage fields from deterministic fixture run:

| Field | Value |
|---|---|
| `proposalId` | `prop-a7705a27` |
| `engineeringChangeId` | `eng-2c86daba` |
| `evidenceBaselineId` | `evd-1cd7c6e9` |
| `evidenceAfterId` | `evd-2596bf65` |
| `architectureSnapshotId` | `asnap-27676593` |
| `replaySessionIds` | `["s-exp-baseline"]` |

---

## 5. Validation outcomes

Validation uses `validateAdaptationEvidence(adaptation, baseline, after)`.

Deterministic fixture run results:

| experimentKey | adaptationId | expectedMetric | baseline | observed | Δ | confidence | validationOutcome | experimentStatus | rolloutDisposition | regressionCheck |
|---|---|---|---:|---:|---:|---:|---|---|---|---|
| `environment_quiet` | `adapt-b10000d7` | `medianTimeToConfidenceMs` | 9000 | 400 | -8600 | 0.5 | passed | validated | activate | clear |
| `spacing_tighten` | `adapt-b4ce3c9d` | `meanFrictionScore` | 0.3927 | 0.0059 | -0.3868 | 0.5 | passed | validated | activate | clear |
| `density_balanced` | `adapt-fea0c2b3` | `meanFrictionScore` | 0.3927 | 0.0059 | -0.3868 | 0.5 | passed | validated | activate | clear |

Failed experiments remain `inactive` adaptations (`experimentStatus: rejected`).  
Successful experiments transition only to `validated` (`rolloutState: candidate` on the adaptation).  
Activation remains a **manual** developer action (overlay toggle). No self-promotion to `active`.

---

## 6. Activation recommendation

| Condition | rolloutDisposition |
|---|---|
| validationOutcome = passed ∧ regressionCheck = clear | `activate` |
| validationOutcome = failed ∨ regressionCheck = triggered | `reject` |
| validationOutcome = skipped | `reject` |

Rollback readiness: replay references present and adaptation not missing (`experimentRollbackReady`).

---

## 7. Authority chain

```
40 → 41 → 42 → 43 → 44 → 45 → 46 → 47 → 48 → 49
```

No additional governance layers were introduced. Experiments reuse `WorkspaceAdaptation`, `ExperienceEvidence`, `ExperienceChangeProposal`, `EngineeringChangeRecord`, and `ArchitectureSnapshot` identifiers without duplication.

---

## 8. Privacy

Experiment storage keys avoid forbidden event content keys (`summary`, free-text content fields). Values are opaque ids and numeric metrics only. No network telemetry in `app/src/experience`.
