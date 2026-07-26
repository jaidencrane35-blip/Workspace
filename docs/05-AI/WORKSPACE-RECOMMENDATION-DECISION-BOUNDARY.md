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
        ↓
RecommendationDecisionConfirmation
        ↓
RecommendationDecisionIntakeRequest
        ✗ no DE object / intent / Gateway
        ↓ (future explicit DE adapter after confirmed intake)
Decision Engine object creation (not implemented)
```

Remaining before a future handoff could exist:

1. ~~Explicit user confirmation~~ — see [WORKSPACE-RECOMMENDATION-DECISION-CONFIRMATION.md](./WORKSPACE-RECOMMENDATION-DECISION-CONFIRMATION.md)
2. ~~Typed intake payload~~ — see [WORKSPACE-RECOMMENDATION-DECISION-INTAKE.md](./WORKSPACE-RECOMMENDATION-DECISION-INTAKE.md)
3. ~~Intake inspection (safe inspect ≠ handoff)~~ — see [WORKSPACE-RECOMMENDATION-DECISION-INTAKE-INSPECTION.md](./WORKSPACE-RECOMMENDATION-DECISION-INTAKE-INSPECTION.md)
4. ~~Intake compatibility (version pin ≠ transfer)~~ — see [WORKSPACE-RECOMMENDATION-DECISION-INTAKE-COMPATIBILITY.md](./WORKSPACE-RECOMMENDATION-DECISION-INTAKE-COMPATIBILITY.md)
5. ~~Proceed denial (compatible ≠ permission)~~ — see [WORKSPACE-RECOMMENDATION-DECISION-INTAKE-PROCEED-DENIAL.md](./WORKSPACE-RECOMMENDATION-DECISION-INTAKE-PROCEED-DENIAL.md)
6. ~~Intake package seal (frozen digest ≠ handoff)~~ — see [WORKSPACE-RECOMMENDATION-DECISION-INTAKE-PACKAGE-SEAL.md](./WORKSPACE-RECOMMENDATION-DECISION-INTAKE-PACKAGE-SEAL.md)
7. ~~Adapter preparation (prepare ≠ invoke)~~ — see [WORKSPACE-RECOMMENDATION-DECISION-INTAKE-ADAPTER-PREPARATION.md](./WORKSPACE-RECOMMENDATION-DECISION-INTAKE-ADAPTER-PREPARATION.md)
8. ~~Handoff request (request ≠ performed / DE object)~~ — see [WORKSPACE-RECOMMENDATION-DECISION-HANDOFF-REQUEST.md](./WORKSPACE-RECOMMENDATION-DECISION-HANDOFF-REQUEST.md)
9. Optional **DE acceptance / adapter invocation** that maps sealed intake → DE object creation without merging domains

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
