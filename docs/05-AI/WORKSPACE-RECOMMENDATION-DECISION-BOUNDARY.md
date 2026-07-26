# Recommendation Decision Boundary Contract

| Field | Value |
|-------|-------|
| **Purpose** | Define the exact separation between recommendation acceptance and future Decision Engine intake |
| **Status** | Sprints 222–226 — boundary classification only |
| **Authority** | Always `none` |

---

## Path (current)

```
Recommendation Candidate
        ↓
Lifecycle + Explanation + Outcome
        ↓
RecommendationDecisionContext
        ↓
RecommendationDecisionReadiness
        ↓
RecommendationDecisionBoundary
        ✗ no DE object / intent / Gateway
        ↓ (future explicit adapter + user confirmation)
Decision Engine intake (not implemented)
```

Remaining before a future handoff could exist (not this sprint):

1. Explicit **user confirmation** to request Decision Engine intake (beyond accept-as-agreement)
2. Optional **adapter** that maps `RecommendationDecisionContext` → DE intake without merging domains
3. **Handoff payload** owned/emitted only after that confirmation (`DecisionEngineHandoff`-shaped)

---

## Transition states

| State | Meaning |
|-------|---------|
| `recommendation_only` | RE-owned; accept (if any) is agreement only, or context incomplete |
| `context_ready` | Context complete but not marked ready for future intake |
| `awaiting_decision_engine_intake` | Context complete + readiness deferred — still **not** handed off |
| `handoff_not_performed` | Permanent `handoff_state` until a future intake sprint |

---

## User intent kinds

| Kind | Meaning |
|------|---------|
| `recommendation_agreement` | Accept = agree with a suggestion (decision record) |
| `recommendation_rejection` | Reject / dismiss |
| `recommendation_terminal` | Expired / superseded |
| `none` | No terminal human decision yet |

Accept is **not**: requesting an action, creating an intent, or authorizing execution.

---

## Ownership metadata

| Owner field | Domain |
|-------------|--------|
| `recommendation_owner` | Recommendation Engine — candidates / lifecycle / history |
| `decision_owner` | Decision Engine — goals / intents |
| `execution_owner` | Permission Gateway |
| `governance_owner` | Governance — change review |
| `experience_owner` | Experience — evidence / traces |

---

## Rejection guards

| Claim | Guard |
|-------|-------|
| Accepted recommendation ≠ intent | `creates_intent = false`; `attempt_create_intent()` fails |
| Context complete ≠ handoff | `handoff_state = handoff_not_performed`; `attempt_handoff()` fails |
| Readiness true ≠ execution authority | `grants_execution_authority = false`; `attempt_authorize_execution()` fails |

---

## Operator clarity

| Phrase | Allowed meaning |
|--------|-----------------|
| Ready for future decision consideration | `awaiting_decision_engine_intake` |
| Decision created | **Never** implied — no DE object id |
| Action approved | **Never** — accept is recommendation agreement |
| Execution authorised | **Never** — Gateway ownership only |

---

## Related

- [WORKSPACE-RECOMMENDATION-DECISION-CONTEXT.md](./WORKSPACE-RECOMMENDATION-DECISION-CONTEXT.md)
- [WORKSPACE-RECOMMENDATION-DECISION-READINESS.md](./WORKSPACE-RECOMMENDATION-DECISION-READINESS.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
