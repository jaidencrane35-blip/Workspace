# Recommendation Decision Intake Request

| Field | Value |
|-------|-------|
| **Purpose** | Typed package for *future* Decision Engine intake after confirmation |
| **Status** | Sprints 232–236 — assemble only; inspection in Sprint 237 |
| **Authority** | Always `none` |

---

## Boundary

```
Confirmation (confirmed + create_future_decision | request_action_review)
        ↓ try_assemble (gates)
RecommendationDecisionIntakeRequest (intake_state = requested)
        ✗ no DecisionCandidate
        ✗ no intent / goal / Gateway
        ✗ handoff_performed = false
        ↓ RecommendationDecisionIntakeInspection::verify (read-only)
        ✗ inspect ≠ handoff / DE ownership
        ↓ (future DE adapter — Decision Engine owns object creation)
```

Intake is **not** DE ownership transfer. It is the structured evidence package a future
adapter may consume. See
[WORKSPACE-RECOMMENDATION-DECISION-INTAKE-INSPECTION.md](./WORKSPACE-RECOMMENDATION-DECISION-INTAKE-INSPECTION.md).

---

## Assembly gates (all required)

1. `confirmation_state == confirmed`
2. `confirmation_intent` is `create_future_decision` or `request_action_review`
3. `RecommendationDecisionContext.complete`
4. `RecommendationDecisionReadiness.ready_for_future_handoff`

Accept / required / declined → no intake.

---

## Ownership

| Concern | Owner |
|---------|-------|
| Suggestion + confirmation + intake assembly | Recommendation Engine |
| Decision object / goal / intent creation | Decision Engine (future) |
| Execution | Permission Gateway |

---

## Guards

| Claim | Guard |
|-------|-------|
| Intake ≠ DE object | `decision_engine_object_id = None`; `attempt_create_decision_engine_object()` fails |
| Intake ≠ handoff performed | `handoff_performed = false`; `attempt_handoff()` fails |
| Intake ≠ execution | `attempt_execute()` fails; `may_invoke_gateway() = false` |
| Confirmation semantics | Accept never emits intake |

---

## Related

- [WORKSPACE-RECOMMENDATION-DECISION-INTAKE-INSPECTION.md](./WORKSPACE-RECOMMENDATION-DECISION-INTAKE-INSPECTION.md)
- [WORKSPACE-RECOMMENDATION-DECISION-CONFIRMATION.md](./WORKSPACE-RECOMMENDATION-DECISION-CONFIRMATION.md)
- [WORKSPACE-RECOMMENDATION-DECISION-BOUNDARY.md](./WORKSPACE-RECOMMENDATION-DECISION-BOUNDARY.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
