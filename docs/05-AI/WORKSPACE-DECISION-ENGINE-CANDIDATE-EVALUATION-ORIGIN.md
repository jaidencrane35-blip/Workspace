# Decision Candidate Evaluation Origin Contract

| Field | Value |
|-------|-------|
| **Purpose** | Define how DecisionCandidate evaluation differs by origin |
| **Status** | Projected for readiness; `evaluated` persisted in `decision_candidate_evaluation_origin` |
| **Authority** | Always `none` |
| **Owner** | Decision Engine |

---

## Boundary

```
DecisionCandidate + LifecycleIntegration
        ↓ derive / try_evaluate
DecisionCandidateEvaluationOriginContract
  evaluation_state =
    unevaluated | eligible_for_evaluation | blocked | evaluated
        ✗ evaluated ≠ DecisionScore / ranking
        ✗ no planner / Gateway / goals / intents / execution
        ✗ never mutates recommendation_lifecycle
```

## Origins

| Origin | Evaluation rules |
|--------|------------------|
| `native` | Existing DE path; no intake provenance required |
| `recommendation_intake` | Provenance must be valid; recommendation source visible; package identity traceable |

## Rules

| State | When |
|-------|------|
| `eligible_for_evaluation` | Lifecycle valid, provenance valid, origin supported |
| `blocked` | Provenance invalid, lifecycle invalid, or unsupported/missing origin |
| `evaluated` | Contract acknowledgment only — no scoring model yet |
| `unevaluated` | Not eligible and not blocked |

## Persistence

Only `evaluated` acknowledgments are persisted (non-derivable event).
Eligible / blocked / unevaluated remain projected.

## Related

- [WORKSPACE-DECISION-ENGINE-CANDIDATE-LIFECYCLE-INTEGRATION.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-LIFECYCLE-INTEGRATION.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
