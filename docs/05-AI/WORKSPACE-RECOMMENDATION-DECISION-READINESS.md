# Recommendation → Decision Engine Readiness Contract

| Field | Value |
|-------|-------|
| **Purpose** | Define handoff *prerequisites* between Recommendation Engine and Decision Engine |
| **Status** | Sprints 212–216 — readiness assessment only |
| **Authority** | Always `none` — never creates intents, commands, or Gateway grants |

---

## Boundary

```
Recommendation Engine (suggest + lifecycle + outcome)
        ↓ human accept
RecommendationOutcome (immutable)
        ↓ assemble (read-only)
RecommendationDecisionContext
        ↓ assess_from_context
RecommendationDecisionReadiness
        ✗ attempt_handoff() hard-fails
        ↓ (future, explicit sprint + user confirmation)
Decision Engine owns intents / goals / planner handoff
        ↓
Command Pipeline → Permission Gateway → Execution
```

Recommendation acceptance records a human decision and outcome only.
It does **not** call Decision Engine, create `WorkGoal` / intents, or invoke Permission Gateway.
See [WORKSPACE-RECOMMENDATION-DECISION-CONTEXT.md](./WORKSPACE-RECOMMENDATION-DECISION-CONTEXT.md).

---

## Prerequisites (all required for `handoff_deferred`)

| Id | Meaning |
|----|---------|
| `has_provenance` | Evidence, attention reasons, or explanation refs present |
| `has_lifecycle_completion` | Lifecycle is `accepted` |
| `has_outcome_history` | Immutable `RecommendationOutcome` recorded |
| `has_explanation` | Structured why-suggested / reason available |
| `has_required_decision_context` | Related task, attention, decision, or purpose context |

### Readiness states

| State | Meaning |
|-------|---------|
| `incomplete` | Not yet accepted / no outcome |
| `blocked` | Accepted + outcome, but one or more prerequisites missing |
| `handoff_deferred` | All prerequisites satisfied — future DE handoff *eligible*, not performed |

`ready_for_future_handoff: true` only in `handoff_deferred`. Still informational:
`may_create_decision_commands()` and `may_invoke_gateway()` are always `false`.

---

## Handoff request contract (non-executing)

Sprint 262 adds `RecommendationDecisionHandoffRequest` — a typed *request*
artifact after adapter preparation. It is **not** `DecisionEngineHandoff` /
`AdaptationHandoff` (no `next_command`, no planner submit). See
[WORKSPACE-RECOMMENDATION-DECISION-HANDOFF-REQUEST.md](./WORKSPACE-RECOMMENDATION-DECISION-HANDOFF-REQUEST.md).

Still true:

- `handoff_performed` remains `false`
- Decision Engine does **not** accept or own the package yet
- No Gateway / Command Pipeline / intent creation from this path

---

## Ownership

| Concern | Owner |
|---------|-------|
| Suggestion + lifecycle + outcomes | Recommendation Engine |
| Evidence translation / traces | Experience (evidence-only) |
| Intent / goal / planner handoff | Decision Engine (future from RE path) |
| Execution authority | Permission Gateway only |
| Scoring / learning from outcomes | **Forbidden** |

---

## Related

- [WORKSPACE-RECOMMENDATION-DECISION-CONTEXT.md](./WORKSPACE-RECOMMENDATION-DECISION-CONTEXT.md)
- [WORKSPACE-RECOMMENDATION-OUTCOME.md](./WORKSPACE-RECOMMENDATION-OUTCOME.md)
- [WORKSPACE-RECOMMENDATION-LIFECYCLE.md](./WORKSPACE-RECOMMENDATION-LIFECYCLE.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
