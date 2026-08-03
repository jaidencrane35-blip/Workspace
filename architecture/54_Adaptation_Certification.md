# 54 — Adaptation Certification

Status: Complete (Sprint 63)  
Date: 2026-08-03  
Branch: `v2-dev`  
Baseline: Sprint 62 commit `a9ade19`  
Authority: `architecture/53_Adaptation_Composition.md` · Version 2 docs `40`–`53`  
Implementation: `app/src/experience/adaptationCertification.ts`

---

## 1. Objective

Deterministic certification that future adaptation sets preserve previously validated behaviour.

Verification sprint — not a feature sprint. Evidence only — no recommendations.

---

## 2. Certification schema

Type: `AdaptationCertification` (`schemaVersion: 1`)  
Storage: `ws.experience.adaptation.certification.v1` (append-only)

| Field | Meaning |
|---|---|
| `certificationId` | Stable `acert-<fnv1a(hash\|now\|previousId)>` |
| `adaptationSetHash` | Hash of sorted adaptation ids + evidence id + architecture id + stability |
| `certifiedAt` | Certification epoch (`now` option) |
| `adaptationIds` | Composed active set (composition order) |
| `evidenceSnapshotId` | Tip `ExperienceEvidence` at certification |
| `engineeringChangeIds` | From composition lineage |
| `architectureSnapshotId` | Valid architecture snapshot id |
| `compositionValidationResult` | From `validateComposition` |
| `certifiedMetrics` | Frozen comparable metric values from evidence (not a parallel metric system) |
| `regressionStatus` | `clear` on stored success records |
| `integrityValid` / `governanceValid` | Gate inputs |
| `composedStabilityScore` | From composition validation |
| `previousCertificationId` | Prior history tip or null |
| `evidenceLineage` | evidence id, fingerprint, source session ids |

Records are immutable: existing entries are never updated in place.

---

## 3. Certification gate

Function: `certifyAdaptationSet(store, { now? })` → `CertificationGateResult`

Succeeds only when all hold:

| Gate | Condition |
|---|---|
| Composition validation | `validateComposition.validationResult === passed` |
| Governance | `governanceIntact` |
| Integrity | `validateArchitectureIntegrity` valid |
| Evidence complete | Tip evidence snapshot present + architecture snapshot resolvable |
| Regression-free | vs previous certification, `compareCertifications.regressionFree` |

On failure: return sorted `failureReasons`; **write nothing**.  
On success: append one frozen certification to history.

Failure reasons (allowlisted):

`governance_invalid` · `integrity_invalid` · `composition_validation_failed` · `evidence_incomplete` · `regression_detected` · `stability_regressed`

---

## 4. Regression comparison

Function: `compareCertifications(previous, current, prevEvidence?, currEvidence?)`

| Cause | Detection |
|---|---|
| `metric_regression` | `compareEvidence` verdict `regressed` on comparable metrics (or frozen metric deltas when evidence missing) |
| `evidence_regression` | Previous evidence id present, current evidence missing |
| `governance_regression` | Previous `governanceValid` ∧ ¬ current |
| `integrity_regression` | Previous `integrityValid` ∧ ¬ current |
| `stability_regression` | `current.composedStabilityScore < previous.composedStabilityScore` |

Each regression lists: cause, affected adaptation ids, affected metrics, evidence ids, allowlisted `field`.

No heuristic scoring.

---

## 5. Immutable history

- Append-only list capped at `MAX_CERTIFICATIONS` (40)
- Duplicate `certificationId` rejected without mutation
- `listCertifications` / `getLatestCertification` / `compareLatestCertifications` are read paths

---

## 6. Authority linkage

Reuses:

- `AdaptationComposition` / `validateComposition`
- `ExperienceEvidence` / `compareEvidence`
- `EngineeringChangeRecord` ids via composition lineage
- `ArchitectureSnapshot`
- `AdaptationStabilityReport` (via composed stability score)

Authority chain:

```
40 → … → 53 → 54
```

No new adaptation primitives. No new lifecycle states. Production behaviour unchanged (verification tooling only).
