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
9. ~~DE acceptance (accept ≠ ownership transfer / DE object)~~ — see [WORKSPACE-RECOMMENDATION-DECISION-ENGINE-ACCEPTANCE.md](./WORKSPACE-RECOMMENDATION-DECISION-ENGINE-ACCEPTANCE.md)
10. ~~DE intake receipt (observe ≠ DecisionCandidate)~~ — see [WORKSPACE-DECISION-ENGINE-INTAKE-RECEIPT.md](./WORKSPACE-DECISION-ENGINE-INTAKE-RECEIPT.md)
11. ~~DE intake assessment (eligible ≠ create candidate)~~ — see [WORKSPACE-DECISION-ENGINE-INTAKE-ASSESSMENT.md](./WORKSPACE-DECISION-ENGINE-INTAKE-ASSESSMENT.md)
12. ~~DE intake eligibility (eligible ≠ create candidate)~~ — see [WORKSPACE-DECISION-ENGINE-INTAKE-ELIGIBILITY.md](./WORKSPACE-DECISION-ENGINE-INTAKE-ELIGIBILITY.md)
13. ~~DE intake candidate (acknowledgement ≠ DecisionCandidate)~~ — see [WORKSPACE-DECISION-ENGINE-INTAKE-CANDIDATE.md](./WORKSPACE-DECISION-ENGINE-INTAKE-CANDIDATE.md)
14. ~~DE intake candidate lifecycle (active/withdrawn/invalidated ≠ DecisionCandidate lifecycle)~~ — see [WORKSPACE-DECISION-ENGINE-INTAKE-CANDIDATE-LIFECYCLE.md](./WORKSPACE-DECISION-ENGINE-INTAKE-CANDIDATE-LIFECYCLE.md)
15. ~~DE intake evaluation (examine ≠ DecisionCandidate / planning authority)~~ — see [WORKSPACE-DECISION-ENGINE-INTAKE-EVALUATION.md](./WORKSPACE-DECISION-ENGINE-INTAKE-EVALUATION.md)
16. ~~DE intake disposition (retain/dismiss/defer ≠ DecisionCandidate / planning)~~ — see [WORKSPACE-DECISION-ENGINE-INTAKE-DISPOSITION.md](./WORKSPACE-DECISION-ENGINE-INTAKE-DISPOSITION.md)
17. ~~DE intake promotion boundary (promotion_allowed ≠ DecisionCandidate creation)~~ — see [WORKSPACE-DECISION-ENGINE-INTAKE-PROMOTION-BOUNDARY.md](./WORKSPACE-DECISION-ENGINE-INTAKE-PROMOTION-BOUNDARY.md)
18. ~~DE candidate creation request (requested ≠ DecisionCandidate creation)~~ — see [WORKSPACE-DECISION-ENGINE-CANDIDATE-CREATION-REQUEST.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-CREATION-REQUEST.md)
19. ~~DE candidate creation (creates DecisionCandidate without scoring / planner / Gateway)~~ — see [WORKSPACE-DECISION-ENGINE-CANDIDATE-CREATION.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-CREATION.md)
20. ~~DE candidate lifecycle integration (origin-aware lifecycle; provenance immutable; no scoring/planner)~~ — see [WORKSPACE-DECISION-ENGINE-CANDIDATE-LIFECYCLE-INTEGRATION.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-LIFECYCLE-INTEGRATION.md)
21. ~~DE candidate evaluation origin contract (origin rules; evaluated ≠ scoring)~~ — see [WORKSPACE-DECISION-ENGINE-CANDIDATE-EVALUATION-ORIGIN.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-EVALUATION-ORIGIN.md)
22. ~~DE candidate evaluation resolution (accepted/rejected for scoring path; ≠ DecisionScore)~~ — see [WORKSPACE-DECISION-ENGINE-CANDIDATE-EVALUATION-RESOLUTION.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-EVALUATION-RESOLUTION.md)
23. ~~DE DecisionScore (scoring result only; ≠ ranking / planner / Gateway)~~ — see [WORKSPACE-DECISION-ENGINE-CANDIDATE-SCORE.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-SCORE.md)
24. ~~DE candidate ranking (comparative ordering only; ≠ selection / planner / Gateway)~~ — see [WORKSPACE-DECISION-ENGINE-CANDIDATE-RANKING.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-RANKING.md)
25. Optional **selection / adapter / planner handoff** after ranking — still separate; not execution

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
