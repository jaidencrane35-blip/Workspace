# 48 — Adaptive Workspace

Status: Complete (Sprint 57)  
Date: 2026-08-03  
Branch: `v2-dev`  
Baseline: Sprint 56 commit `4e707f3`  
Authority: `architecture/47_Architectural_Integrity.md` · Version 2 docs `40`–`47`  
Implementation: `app/src/experience/workspaceAdaptation.ts` · shell apply in `WorkspaceShell.tsx`

---

## 1. Objective

Enable Workspace to adapt presentation from observed evidence while remaining deterministic and fully governed.

This is the first Version 2 sprint that may alter production Experience since Sprint 50. Every adaptation must originate from approved governance lineage. Evidence only — no design recommendations.

---

## 2. Adaptation schema

Type: `WorkspaceAdaptation` (`schemaVersion: 1`)  
Storage: `ws.experience.adaptation.v1`

| Field | Meaning |
|---|---|
| `adaptationId` | Stable `adapt-<fnv1a(lineage\|scopes\|metric)>` |
| `engineeringChangeId` | `EngineeringChangeRecord` id |
| `proposalId` | `ExperienceChangeProposal` id |
| `evidenceSnapshotId` | `ExperienceEvidence` id |
| `architectureSnapshotId` | `ArchitectureSnapshot` id |
| `targetComponents` | Allowlisted surfaces (`shell`, `home`, `save`, …) |
| `scopes` | `spacing` \| `emphasis` \| `grouping` \| `motion` \| `density` \| `environment` |
| `expectedMetric` / `expectedImprovementDelta` / `expectedDirection` | Measurable success |
| `rolloutState` | `inactive` \| `candidate` \| `active` \| `rolled_back` |
| `validation` | Validation contract + result |
| `presentation` | Presentation configuration only |

No adaptation may be persisted without governance lineage (`verifyAdaptationLineage`).

---

## 3. Resolver

Function: `resolvePresentationConfiguration(adaptations)`

Inputs: stored adaptations (+ current shell intent spacing/depth at apply time).  
Outputs: `ResolvedPresentation` — presentation configuration only.

Eligible adaptations: `rolloutState === active` **and** `validationResult === passed`.  
Merge order: sorted `adaptationId` (deterministic; later ids overwrite scalar fields via multiply/replace rules).

Influences:

| Scope | Mechanism |
|---|---|
| spacing | `--intent-space` multiplier |
| emphasis | `--adapt-emphasis` |
| grouping | `--adapt-group` / `--spatial-gutter` |
| motion | `data-adapt-motion` + motion duration vars |
| density | `useWorkspaceDensity(override)` |
| environment | `--intent-depth` multiplier |

Must not alter: navigation, feature availability, Runtime Core, persistence schemas, domain data model.

Apply point: `WorkspaceShell` / `ShellBody` (`presentationToShellStyle`).

---

## 4. Governance linkage

Required lineage (all must match):

```
ArchitectureSnapshot (integrity.valid)
  ← WorkspaceAdaptation
    ← ExperienceEvidence
    ← ExperienceChangeProposal (accepted+)
    ← EngineeringChangeRecord (architecturally_accepted | released)
```

Proposal id must appear on the engineering record.  
Replay session ids are taken from the proposal validation contract (not duplicated).

Builder: `buildAdaptationFromLineage`.  
Persister rejects incomplete lineage. Create always starts `inactive` with non-passed validation (no self-promotion).

---

## 5. Validation

Contract fields:

- `baselineEvidenceId`
- `expectedImprovementDelta` / `expectedDirection` / `expectedMetric`
- `rollbackCriteria` (`friction_regression`, `abandon_increase`, `ttc_regression`, `replay_divergence`)
- `replaySessionIds`
- `validationResult` (`pending` \| `passed` \| `failed`)

`validateAdaptationEvidence(adaptation, baseline, after)`:

1. Evaluate rollback criteria against before/after metrics.
2. Require measurable improvement on `expectedMetric` by `expectedImprovementDelta`.
3. On failure → `failed`; if was active → `rolled_back`, else `inactive`.
4. On success → `passed` and `candidate` (not auto-active).

`activateAdaptation` requires `passed` and is manual only.

---

## 6. Rollback model

`rollbackAdaptation` sets `rolled_back`.  
Record retained (no data loss). Resolver excludes rolled-back / inactive / failed adaptations immediately.  
Shell returns to identity presentation when no active adaptations remain.

---

## 7. Authority chain

```
40 → 41 → 42 → 43 → 44 → 45 → 46 → 47 → 48
```

This document is the authority for governed presentation adaptation.  
Upstream: integrity snapshots (47), engineering records (46), proposals (45), evidence (42–43).

---

## 8. Development workflow

1. Complete governance pipeline through engineering acceptance + architecture snapshot.
2. DEV overlay **Draft adaptation** (lazy-loads adaptation module).
3. **Validate vs latest** evidence → passed/failed.
4. Manual **Activate** only when passed.
5. Observe shell presentation; **Rollback** returns to inactive presentation without deleting the record.

Production applies only active+passed adaptations from local storage. Default is identity (Sprint 50 presentation behaviour unchanged until an adaptation is activated).
