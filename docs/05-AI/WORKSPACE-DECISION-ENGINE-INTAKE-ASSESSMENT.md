# Decision Engine Intake Assessment

| Field | Value |
|-------|-------|
| **Purpose** | Observational evaluation of an intake receipt for *future* candidate consideration |
| **Status** | Projected only — not persisted |
| **Authority** | Always `none` |
| **Owner** | Decision Engine |

---

## Boundary

```
DecisionEngineIntakeReceipt
        ↓ assess_batch (recomputable projection)
DecisionEngineIntakeAssessment
  assessment_state =
    blocked | superseded | duplicate | stale | valid | eligible_for_future_candidate
  eligible_for_future_candidate = informational only
        ✗ assess ≠ DecisionCandidate creation
        ✗ assess ≠ goal / intent / planner / Gateway
        ✗ assess ≠ ownership transfer
        ✗ never mutates Recommendation Engine overlays
```

## Persistence

Not persisted. Fully derivable from receipt + RE overlay facts
(acceptance active, prep/handoff current, lifecycle superseded, peer digests).

## Related

- [WORKSPACE-DECISION-ENGINE-INTAKE-RECEIPT.md](./WORKSPACE-DECISION-ENGINE-INTAKE-RECEIPT.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
