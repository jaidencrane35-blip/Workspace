# Decision Engine Intake Candidate Lifecycle

| Field | Value |
|-------|-------|
| **Purpose** | DE-owned lifecycle management for intake candidates |
| **Status** | Persisted on `decision_engine_intake_candidate` |
| **Authority** | Always `none` |
| **Owner** | Decision Engine |

---

## Boundary

```
DecisionEngineIntakeCandidate
        ↓ lifecycle
DecisionEngineIntakeCandidateLifecycle
  lifecycle_state = active | withdrawn | invalidated
        ✗ ≠ DecisionCandidate lifecycle
        ✗ ≠ planner / execution lifecycle
        ✗ never mutates recommendation_lifecycle
        ✗ invalidated → active forbidden
```

## Rules

| Event | Effect |
|-------|--------|
| Eligible create | `active` |
| DE withdraw | `withdrawn` (DE state only) |
| Seal mismatch | `invalidated` |
| Acceptance revoked | `invalidated` |
| Recommendation superseded | `invalidated` |

## Related

- [WORKSPACE-DECISION-ENGINE-INTAKE-CANDIDATE.md](./WORKSPACE-DECISION-ENGINE-INTAKE-CANDIDATE.md)
- [WORKSPACE-DECISION-ENGINE-INTAKE-EVALUATION.md](./WORKSPACE-DECISION-ENGINE-INTAKE-EVALUATION.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
