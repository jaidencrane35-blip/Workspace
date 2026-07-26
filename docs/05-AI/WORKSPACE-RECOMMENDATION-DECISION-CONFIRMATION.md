# Recommendation Decision Confirmation Contract

| Field | Value |
|-------|-------|
| **Purpose** | Separate recommendation acceptance from future Decision Engine creation |
| **Status** | Sprints 227–231 — confirmation state only |
| **Authority** | Always `none` |

---

## Boundary

```
User saw recommendation (Presented)
        ↓
User accepted recommendation (agreement only)
        ↓
RecommendationDecisionConfirmation
  required | not_required | confirmed | declined
        ✗ does not create Decision / Intent / Gateway grant
        ↓ (when confirmed)
RecommendationDecisionIntakeRequest (typed package only)
        ↓ RecommendationDecisionIntakeInspection (safe inspect ≠ handoff)
        ↓ (future DE adapter)
Decision Engine owns decision creation
```

See [WORKSPACE-RECOMMENDATION-DECISION-INTAKE.md](./WORKSPACE-RECOMMENDATION-DECISION-INTAKE.md)
and [WORKSPACE-RECOMMENDATION-DECISION-INTAKE-INSPECTION.md](./WORKSPACE-RECOMMENDATION-DECISION-INTAKE-INSPECTION.md).

| Step | Meaning |
|------|---------|
| Saw | Lifecycle `presented` |
| Accepted | `recommendation_agreement` — not a decision created |
| Wants decision created | Confirmation `confirmed` + `create_future_decision` |
| Wants action reviewed | Confirmation `confirmed` + `request_action_review` |
| Authorised execution | **Never** from confirmation — Permission Gateway only |

---

## States

| State | Meaning |
|-------|---------|
| `not_required` | No future DE intake path open |
| `required` | Boundary is `awaiting_decision_engine_intake` — user must confirm or decline |
| `confirmed` | User wants future DE consideration — still no DE object |
| `declined` | User declined future DE consideration |

Accept never sets `confirmed`.

---

## Confirmation intents

| Intent | Meaning |
|--------|---------|
| `agreement_only` | Default after accept / decline |
| `create_future_decision` | User wants a future Decision Engine object (not created yet) |
| `request_action_review` | User wants future action review consideration (not execution) |

---

## Ownership

| Owner | Domain |
|-------|--------|
| Recommendation Engine | Suggestion lifecycle |
| User | Confirmation choice |
| Decision Engine | Future decision creation |
| Permission Gateway | Execution |

---

## Guards

| Claim | Guard |
|-------|-------|
| Accept ≠ confirmation | Accept derives `required`/`not_required` + `agreement_only` |
| Confirmation ≠ intent | `creates_intent = false`; `attempt_create_intent()` fails |
| Confirmation ≠ execution | `grants_execution_authority = false`; `attempt_execute()` fails |
| Confirmed ≠ handoff | `handoff_performed = false`; `attempt_handoff()` fails |

---

## Related

- [WORKSPACE-RECOMMENDATION-DECISION-BOUNDARY.md](./WORKSPACE-RECOMMENDATION-DECISION-BOUNDARY.md)
- [WORKSPACE-RECOMMENDATION-DECISION-CONTEXT.md](./WORKSPACE-RECOMMENDATION-DECISION-CONTEXT.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
