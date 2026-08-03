# 46 — Engineering Governance

Status: Complete (Sprint 55)  
Date: 2026-08-03  
Branch: `v2-dev`  
Baseline: Sprint 54 commit `5f7800b`  
Authority: `architecture/45_Experience_Change_Governance.md` · Version 2 docs `40`–`45`  
Implementation: `app/src/dev/engineeringGovernance.ts` · DEV overlay engineering records

---

## 1. Objective

Establish deterministic engineering governance that records, validates, and certifies every implementation change before it becomes part of the Workspace architecture.

This governs engineering itself — not user behaviour, production Experience, or Runtime Core. All advancement is developer-authorized. Evidence only — no recommendations.

---

## 2. EngineeringChangeRecord schema

Type: `EngineeringChangeRecord` (`schemaVersion: 1`)  
Builder: `buildEngineeringRecordFromProposals`  
Storage: `ws.dev.experience.engineering.v1`

| Field | Meaning |
|---|---|
| `changeId` | Stable `eng-<fnv1a(proposalIds\|commits\|architectureDocuments)>` |
| `proposalIds` | Originating `ExperienceChangeProposal` ids (reused, not duplicated) |
| `commits` | Hex commit SHAs (`^[a-f0-9]{7,40}$`) |
| `architectureDocuments` | Authority docs from the Version 2 set (`40`–`46`) |
| `affectedModules` | Allowlisted module path tokens |
| `affectedTests` | Allowlisted test path tokens |
| `validationEvidence` | Linked proposals / evidence / opportunities / replays + completeness flag |
| `releaseImpact` | `none` \| `dev_tooling` \| `documentation` \| `tests` \| `architecture` \| `validation_tooling` |
| `state` | Lifecycle status |
| `consistencyStatus` | `incomplete` \| `complete` |

`EngineeringValidationEvidence` reuses existing identifiers:

- `proposalIds`, `evidenceSnapshotIds`, `opportunityIds`, `replaySessionIds`
- `proposalValidationComplete`
- `architectureAuthorityIds`

No user-content fields. No parallel opportunity/evidence models.

---

## 3. Lifecycle

```
draft → implemented → validated → architecturally_accepted → released
```

| From | To | Reason token |
|---|---|---|
| (create) | draft | `created_from_proposals` |
| draft | implemented | `mark_implemented` |
| implemented | validated | `mark_validated` |
| validated | architecturally_accepted | `architecturally_accept` |
| architecturally_accepted | released | `release` |

Properties:

- Manual transitions only (`transitionEngineeringRecord`)
- No automatic advancement
- `released` is terminal
- Invalid edges return `invalid_transition` without writing history

---

## 4. Authority chain

```
Architecture authority docs (40–46)
  → ExperienceChangeProposal (Sprint 54)
    → ExperienceOpportunity (Sprint 53)
      → ExperienceEvidence (Sprint 52)
        → Replay / interaction traces (Sprint 51)
```

Authority document allowlist (`ARCHITECTURE_AUTHORITY_DOCS`):

- `40_Experience_Refoundation.md`
- `41_Perceptual_Convergence.md`
- `42_Experience_Validation.md`
- `43_Experience_Evidence_Model.md`
- `44_Experience_Improvement_Model.md`
- `45_Experience_Change_Governance.md`
- `46_Engineering_Governance.md`

Every record must reference ≥1 authority document. History entries record `authorityReference` on each transition.

---

## 5. Validation rules (architecture consistency)

Function: `verifyEngineeringConsistency(record, context)`

Context supplies existing proposals and evidence snapshot ids.

Reject (`consistencyStatus = incomplete`) when any hold:

| Error | Condition |
|---|---|
| `missing_proposal` | No / unknown proposal ids |
| `missing_evidence` | No evidence ids or ids absent from evidence store |
| `missing_validation` | `proposalValidationComplete` is false |
| `proposal_validation_incomplete` | Linked proposal fails Sprint 54 validation contract |
| `missing_architecture_authority` | No / unknown authority docs |
| `missing_commit` | No valid commit SHAs |
| `missing_module` / `missing_test` | Empty affected module or test lists |
| `orphan_opportunity` | Linked proposal has no opportunity ids |
| `orphan_replay` | Linked proposal has no replay session ids |

`persistEngineeringRecord` refuses to store incomplete records.  
Transitions also require `complete` consistency.

---

## 6. Release traceability

Function: `buildReleaseTraceability(record, context)`

Required chain for every released change:

```
Commit
  → Proposal
    → Opportunity
      → Evidence
        → Replay
          → Interaction traces
```

| Gap token | Meaning |
|---|---|
| `missing_commit` | No commits on record |
| `missing_proposal` | No / unknown proposals |
| `missing_opportunity` | No opportunity ids in chain |
| `missing_evidence` | No evidence snapshot ids |
| `missing_replay` | No replay session ids |
| `incomplete_proposal_validation` | Proposal validation not complete |

`interactionSessionIds` = replay session ids (Sprint 51 trace store; not duplicated).

Transition to `released` requires `trace.complete === true`.  
`assertNoOrphanReleasedRecords` certifies no released orphan work.

---

## 7. History model

Type: `EngineeringHistoryEntry` — append-only, immutable after write.

| Field | Meaning |
|---|---|
| `entryId` | `ehist-<changeId>-<seq>` |
| `changeId` | Record reference |
| `seq` | Per-record monotonic sequence |
| `t` | Epoch ms (caller-injectable) |
| `previousState` / `newState` | Lifecycle edge |
| `reason` | Allowlisted `EngineeringReason` |
| `authorityReference` | Architecture authority doc |

Prior entries are never mutated (`Object.freeze` on append; `assertEngineeringHistoryImmutable` for audits).

---

## 8. Governance workflow

1. Complete Experience governance: opportunities → proposal → validation (Sprints 51–54).
2. Advance proposal to `accepted` (or later).
3. DEV overlay **Draft engineering** (lazy-loads `engineeringGovernance`) → consistency-gated persist.
4. Inspect engineering records: architecture linkage, validation completeness, commit lineage, release readiness.
5. Manually advance: implemented → validated → architecturally_accepted → released.
6. Released records must show full traceability; incomplete chains are rejected.

Production Experience and Runtime Core remain unchanged. Tooling stays behind the Experience Validation DEV gate.

---

## 9. Privacy / boundary

| Guarantee | Mechanism |
|---|---|
| Zero user-content persistence | Ids, enums, path tokens, numbers only |
| Local-only | Separate localStorage key; no network APIs in `app/src/dev` |
| DEV-only visibility | Dynamic import under `import.meta.env.DEV` |
| No Runtime Core / production Experience changes | Out of scope for this module |
| Identifier reuse | Proposal / evidence / opportunity / replay ids not re-minted |
