# Recommendation Decision Context Contract

| Field | Value |
|-------|-------|
| **Purpose** | Define the structured intake snapshot a future Decision Engine would require |
| **Status** | Sprints 217–221 — context assembly only |
| **Authority** | Always `none` — never creates intents, DE objects, or Gateway grants |

---

## Boundary

```
Recommendation candidate
        ↓ human accept (decision record only)
RecommendationOutcome
        ↓ assemble (read-only)
RecommendationDecisionContext
        ↓ assess
RecommendationDecisionReadiness
        ✗ attempt_handoff() hard-fails
        ↓ (future explicit intake + user confirmation)
Decision Engine owns goals / intents
```

`RecommendationDecisionContext` is **assessment/input only**. It is not a Decision Engine
candidate, not an intent, and not a handoff payload.

---

## Context fields

| Field | Meaning |
|-------|---------|
| `recommendation_id` / `kind` / `title` / `family` | Recommendation identity (`family = recommendation_engine`) |
| `explanation_ref` + `explanation_keys` | Explanation surface references (not CoT) |
| `evidence_refs` / `experience_trace_match_keys` / `continuity_fingerprint` | Provenance references |
| `lifecycle_state` / `lifecycle_resolution` | Lifecycle completion |
| `outcome_id` / `outcome_history_refs` | Outcome history references |
| `user_decision` / `result_kind` | Human decision record |
| `related_*` | Required decision context anchors |
| `missing` / `complete` | Completeness vs readiness prerequisites |
| `decision_engine_object_id` | Always `None` until future DE intake wiring |
| `handoff_performed` | Always `false` |

---

## Validation

| Guard | Behavior |
|-------|----------|
| `attempt_execute()` | Hard-fail |
| `attempt_handoff()` | Hard-fail (`CannotBecomeHandoff`) even when `complete` |
| `may_create_intent()` | Always `false` |
| `may_become_decision_engine_object()` | Always `false` |
| `may_invoke_gateway()` | Always `false` |
| `may_mutate_provenance()` | Always `false` |

Readiness is derived from context (`assess_from_context`). Incomplete context blocks
`handoff_deferred`. Completeness never upgrades into a handoff.

---

## Operator distinction

| Signal | Meaning |
|--------|---------|
| Context `complete` + readiness `handoff_deferred` | Ready for *future* DE consideration |
| Lifecycle `accepted` + outcome | Historically accepted recommendation decision |
| `decision_engine_object_id` present | Would mean a DE object exists — **never set today** |
| `handoff_performed` | Would mean handoff occurred — **always false today** |

UI must not imply a Decision Engine transition has occurred.

---

## Ownership

| Concern | Owner |
|---------|-------|
| Candidates + lifecycle + outcomes + context assembly | Recommendation Engine |
| Goals / intents / planner handoff | Decision Engine |
| Execution | Permission Gateway |
| Change review | Governance |
| Evidence / traces | Experience |

---

## Related

- [WORKSPACE-RECOMMENDATION-DECISION-READINESS.md](./WORKSPACE-RECOMMENDATION-DECISION-READINESS.md)
- [WORKSPACE-RECOMMENDATION-OUTCOME.md](./WORKSPACE-RECOMMENDATION-OUTCOME.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
