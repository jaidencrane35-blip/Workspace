# Decision Engine Intake Candidate

| Field | Value |
|-------|-------|
| **Purpose** | DE-owned acknowledgement that an eligible RE sealed package is accepted for future evaluation |
| **Status** | Persisted DE aggregate (`decision_engine_intake_candidate`) |
| **Authority** | Always `none` |
| **Owner** | Decision Engine |

---

## Boundary

```
DecisionEngineIntakeEligibility
        ↓ eligible only
DecisionEngineIntakeCandidate
  state =
    observed | ready_for_future_evaluation | blocked | withdrawn
  lifecycle =
    active | withdrawn | invalidated
  namespace = engine_decision_intake:*
        ✗ IntakeCandidate ≠ DecisionCandidate
        ✗ ≠ goal / intent / planner / Gateway / score / rank
        ✗ ≠ ownership transfer
        ✗ never mutates Recommendation Engine overlays
```

## Creation rules

Allowed only when receipt observed, assessment valid, eligibility eligible,
acceptance active, and package seal matches. New candidates start lifecycle
`active`.

Blocked for declined/revoked acceptance, seal mismatch, superseded,
duplicate digest, or stale package.

## Persistence

DE-owned table `decision_engine_intake_candidate`. Thin identity + lifecycle
state only — never stores Recommendation Engine payloads. Unique per
`(workspace_id, package_seal_digest)` so duplicate intakes cannot materialize.

Lifecycle details:
[WORKSPACE-DECISION-ENGINE-INTAKE-CANDIDATE-LIFECYCLE.md](./WORKSPACE-DECISION-ENGINE-INTAKE-CANDIDATE-LIFECYCLE.md)

## Related

- [WORKSPACE-DECISION-ENGINE-INTAKE-ELIGIBILITY.md](./WORKSPACE-DECISION-ENGINE-INTAKE-ELIGIBILITY.md)
- [WORKSPACE-DECISION-ENGINE-INTAKE-CANDIDATE-LIFECYCLE.md](./WORKSPACE-DECISION-ENGINE-INTAKE-CANDIDATE-LIFECYCLE.md)
- [WORKSPACE-DECISION-ENGINE-INTAKE-EVALUATION.md](./WORKSPACE-DECISION-ENGINE-INTAKE-EVALUATION.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
