# Decision Engine Intake Disposition

| Field | Value |
|-------|-------|
| **Purpose** | DE-owned lifecycle decision for an evaluated intake candidate |
| **Status** | Persisted (`decision_engine_intake_disposition`) |
| **Authority** | Always `none` |
| **Owner** | Decision Engine |

---

## Boundary

```
DecisionEngineIntakeEvaluation
        ↓ try_dispose (active + evaluated only)
DecisionEngineIntakeDisposition
  disposition_state = retained | dismissed | deferred
        ✗ dispose ≠ DecisionCandidate
        ✗ ≠ goal / intent / planner / Gateway / adapter / execution
        ✗ never mutates recommendation_lifecycle
```

## Rules

| Condition | May dispose? |
|-----------|--------------|
| Active + evaluated | yes |
| Unevaluated | no |
| Withdrawn | no |
| Invalidated | no |

| State | Meaning |
|-------|---------|
| `retained` | Keep available for possible future processing |
| `dismissed` | Decline further consideration |
| `deferred` | Postpone consideration |

No disposition state creates execution or planning authority.

## Related

- [WORKSPACE-DECISION-ENGINE-INTAKE-EVALUATION.md](./WORKSPACE-DECISION-ENGINE-INTAKE-EVALUATION.md)
- [WORKSPACE-DECISION-ENGINE-INTAKE-CANDIDATE.md](./WORKSPACE-DECISION-ENGINE-INTAKE-CANDIDATE.md)
- [WORKSPACE-DECISION-ENGINE-INTAKE-PROMOTION-BOUNDARY.md](./WORKSPACE-DECISION-ENGINE-INTAKE-PROMOTION-BOUNDARY.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
