# Decision Engine Candidate Creation

| Field | Value |
|-------|-------|
| **Purpose** | DE-owned boundary for controlled creation of a native `DecisionCandidate` from an approved creation request |
| **Status** | Persisted when `created` — `decision_engine_candidate_creation` |
| **Authority** | Always `none` |
| **Owner** | Decision Engine |

---

## Boundary

```
DecisionEngineCandidateCreationRequest (requested)
        ↓ try_create (explicit)
DecisionEngineCandidateCreation
  creation_state =
    blocked | eligible_for_creation | created
        ↓ created
DecisionCandidate  (engine_decision:intake:*)
  provenance:
    intake_candidate_id
    creation_request_id
    package_seal_digest / recommendation_reference
        ✗ no scoring / DecisionScore behaviour
        ✗ no planner / Gateway / goals / intents / execution
        ✗ never mutates recommendation_lifecycle
```

## Rules

| State | When |
|-------|------|
| `blocked` | Request rejected, intake invalid/withdrawn/invalidated, seal mismatch, acceptance revoked, provenance broken |
| `eligible_for_creation` | Request `requested`, intake active, promotion allowed, seal/acceptance valid, provenance intact |
| `created` | Explicit create event produced a DE-owned `DecisionCandidate` |

## Namespace

| Before | After |
|--------|-------|
| `engine_decision_intake:*` | `engine_decision:intake:*` |

## Persistence

Only `created` records are persisted (DE-owned table). Blocked / eligible projections are recomputed.

## Related

- [WORKSPACE-DECISION-ENGINE-CANDIDATE-CREATION-REQUEST.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-CREATION-REQUEST.md)
- [WORKSPACE-DECISION-ENGINE-CANDIDATE-LIFECYCLE-INTEGRATION.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-LIFECYCLE-INTEGRATION.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
