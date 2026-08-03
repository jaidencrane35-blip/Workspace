# 45 — Experience Change Governance

Status: Complete (Sprint 54)  
Date: 2026-08-03  
Branch: `v2-dev`  
Baseline: Sprint 53 commit `9ca67f0`  
Authority: `architecture/44_Experience_Improvement_Model.md` · Version 2 docs `40`–`44`  
Implementation: `app/src/dev/experienceGovernance.ts` · DEV overlay proposal queue

---

## 1. Objective

Sit a deterministic governance layer between evidence/opportunities and implementation.

An opportunity alone never authorizes a change. Every future change requires a repeatable proposal lifecycle with measurable validation. This document is evidence-only — no UX recommendations.

---

## 2. Proposal schema

Type: `ExperienceChangeProposal` (`schemaVersion: 1`)  
Builder: `buildProposalFromOpportunities`  
Storage key: `ws.dev.experience.governance.v1` (separate from traces and evidence)

| Field | Meaning |
|---|---|
| `proposalId` | Stable `prop-<fnv1a(opportunityIds\|evidenceIds\|successMetric)>` |
| `opportunityIds` | Originating `ExperienceOpportunity` ids |
| `evidenceSnapshotIds` | Linked evidence snapshot ids |
| `workflows` | Allowlisted opportunity workflows |
| `affectedComponents` | Allowlisted surface tokens (`home`, `save`, `resume`, …) |
| `expectedImprovements` | Metric / direction / `targetDelta` / `baselineValue` |
| `confidence` | Mean opportunity confidence |
| `implementationScope` | `dev_tooling` \| `experience_surface` \| `instrumentation` \| `documentation` \| `none` |
| `state` | Lifecycle state |
| `validation` | Validation contract (see §3) |
| `validationStatus` | `incomplete` \| `complete` |

No natural-language fields. No user content. Builder returns `null` without evidence and replay linkage.

---

## 3. Validation contract

Type: `ProposalValidationContract`

| Field | Requirement |
|---|---|
| `evidenceBaselineId` | Non-empty allowlisted id |
| `replaySessionIds` | ≥1 session id (references Sprint 51 trace store; not duplicated) |
| `criteria` | Must include all of: `has_evidence_baseline`, `has_replay_references`, `has_success_metric`, `has_expected_improvements`, `opportunities_linked` |
| `successMetric` | Key of `ExperienceEvidenceMetrics` |
| `successDirection` | `lower_better` \| `higher_better` |
| `successThresholdDelta` | Positive measurable magnitude |

`evaluateValidationContract` → `complete` only when every requirement holds.  
Subjective acceptance is not represented.

A proposal cannot enter `review`, `accepted`, or `validated` unless `validationStatus === complete`.

---

## 4. Lifecycle

```
draft → review → accepted → implemented → validated → closed
              ↘ draft (reject)
```

| From | To | Reason token |
|---|---|---|
| (create) | draft | `created_from_opportunities` |
| draft | review | `submit_for_review` |
| review | accepted | `accept` |
| review | draft | `reject_to_draft` |
| accepted | implemented | `mark_implemented` |
| implemented | validated | `mark_validated` |
| validated | closed | `close` |

Properties:

- Manual transitions only (`transitionProposal`)
- No automatic promotion
- Closed is terminal
- Invalid edges return `invalid_transition` without writing history

---

## 5. History model

Type: `GovernanceHistoryEntry` — append-only, immutable after write.

| Field | Meaning |
|---|---|
| `entryId` | `hist-<proposalId>-<seq>` |
| `proposalId` | Proposal reference |
| `seq` | Per-proposal monotonic sequence |
| `t` | Epoch ms (caller-injectable) |
| `previousState` | Prior lifecycle or `null` on create |
| `newState` | Resulting lifecycle |
| `reason` | Allowlisted `GovernanceReason` |
| `evidenceReferenceId` | Evidence id recorded with the transition |

Guarantees:

- Prior entries are never mutated (`Object.freeze` on append; `assertHistoryImmutable` for audits)
- Proposal current state may update; history only grows
- Ring cap: 50 proposals, 2000 history entries (oldest history dropped only by cap)

---

## 6. Evidence linkage

| Link | Source |
|---|---|
| Opportunities | `opportunityIds` from Sprint 53 detector |
| Evidence | `evidenceSnapshotIds` + `validation.evidenceBaselineId` |
| Replay | `validation.replaySessionIds` → existing trace store / `replayStoredSession` |

Governance storage does not copy raw traces or user content.

---

## 7. Development workflow

1. Collect traces; analyze → evidence; detect opportunities (Sprints 51–53).
2. DEV overlay **Draft proposal** → persists draft + creation history.
3. Inspect proposal queue: state, validation status, linked evidence/opportunities/replays.
4. Manually advance: review → accepted → implemented → validated → closed (or reject review → draft).
5. Each transition appends immutable history with evidence reference.
6. Replay links invoke existing replay infrastructure (no second store).

Production Experience and Runtime Core are unchanged. Overlay + governance module lazy-load under the DEV validation gate.

---

## 8. Privacy guarantees

| Guarantee | Mechanism |
|---|---|
| Zero user-content persistence | Ids, enums, numbers only; no notes/titles/messages |
| Forbidden keys | Sprint 51 `FORBIDDEN_EVENT_KEYS` remain excluded |
| Local-only | `localStorage` / memory; no network APIs in `app/src/dev` |
| DEV-only visibility | Dynamic import behind `import.meta.env.DEV` |
| Reasons are tokens | Allowlisted `GovernanceReason` — not free text |

---

## 9. Engineering boundary

- Reuses `ExperienceOpportunity`, `ExperienceEvidence`, replay infrastructure
- Does not duplicate event or friction models
- Does not auto-implement or auto-accept changes
- Governance tooling stays behind the Experience Validation gate
