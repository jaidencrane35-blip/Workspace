# Decision Engine Intake Eligibility

| Field | Value |
|-------|-------|
| **Purpose** | Observational gate: may this assessment ever become a future DecisionCandidate? |
| **Status** | Projected only — not persisted |
| **Authority** | Always `none` |
| **Owner** | Decision Engine |

---

## Boundary

```
DecisionEngineIntakeAssessment
        ↓ derive_batch (recomputable projection)
DecisionEngineIntakeEligibility
  eligibility_state =
    not_eligible | blocked | duplicate | superseded | stale | eligible
  is_eligible = informational only
        ↓ eligible only
DecisionEngineIntakeCandidate (DE-owned acknowledgement; still ≠ DecisionCandidate)
        ✗ eligible ≠ DecisionCandidate creation
        ✗ eligible ≠ score / ranking
        ✗ eligible ≠ goal / intent / planner / Gateway
        ✗ eligible ≠ ownership transfer
        ✗ never mutates Recommendation Engine overlays
```

## Persistence

Not persisted. Fully derivable from receipt + assessment
(acceptance active, seal alignment, duplicate / superseded / stale flags).

## Related

- [WORKSPACE-DECISION-ENGINE-INTAKE-ASSESSMENT.md](./WORKSPACE-DECISION-ENGINE-INTAKE-ASSESSMENT.md)
- [WORKSPACE-DECISION-ENGINE-INTAKE-RECEIPT.md](./WORKSPACE-DECISION-ENGINE-INTAKE-RECEIPT.md)
- [WORKSPACE-DECISION-ENGINE-INTAKE-CANDIDATE.md](./WORKSPACE-DECISION-ENGINE-INTAKE-CANDIDATE.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
