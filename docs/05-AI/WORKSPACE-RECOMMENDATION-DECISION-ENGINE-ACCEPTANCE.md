# Recommendation Decision Engine Acceptance

| Field | Value |
|-------|-------|
| **Purpose** | Typed non-executing DE acceptance boundary for an active handoff request |
| **Status** | Sprint 267 — acceptance speech-act only |
| **Authority** | Always `none` |
| **Permission** | Always `none` |

---

## Boundary

```
RecommendationDecisionHandoffRequest (active)
        ↓ derive_from_handoff_request
RecommendationDecisionEngineAcceptance
  acceptance_state = awaiting_acceptance | accepted | declined | revoked
  ownership_state = retained_by_recommendation
                  | accepted_for_future_decision_engine
                  | declined_by_decision_engine
  ownership_transferred = false (always)
  current_owner = recommendation_engine (always)
  decision_engine_object_id = None (always)
        ✗ accept ≠ ownership transfer
        ✗ accept ≠ DE object / intent / adapter invoke
        ✗ accept ≠ Gateway / execution
        ✗ revoke / inactive request blocks progression
        ↓ (future DE-owned object creation — separate; still not execution)
```

Acceptance means Decision Engine acknowledges the sealed handoff request for
*future* ownership. It does **not** transfer ownership or create Decision Engine
objects. Recommendation Engine remains `current_owner`.

---

## Requirements

1. Active handoff request (`handoff_requested` + preparation aligned)
2. Accept / decline only from `awaiting_acceptance`
3. Seal digest + compatibility identity preserved on the acceptance record

---

## Guards

| Claim | Guard |
|-------|-------|
| Accept ≠ transfer | `ownership_transferred = false`; `attempt_transfer_ownership()` fails |
| Accept ≠ DE object | `decision_engine_object_id = None`; create-DE fails |
| Accept ≠ adapter | `adapter_invoked = false`; invoke fails |
| Accept ≠ execution | `attempt_execute()` fails; `may_invoke_gateway() = false` |
| Ownership | `current_owner = recommendation_engine` |
| Future intent | `declared_future_owner = decision_engine` only when accepted |
| Reversible | `revoke()`; prep revoke cascades |

---

## Related

- [WORKSPACE-RECOMMENDATION-DECISION-HANDOFF-REQUEST.md](./WORKSPACE-RECOMMENDATION-DECISION-HANDOFF-REQUEST.md)
- [WORKSPACE-RECOMMENDATION-DECISION-BOUNDARY.md](./WORKSPACE-RECOMMENDATION-DECISION-BOUNDARY.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
