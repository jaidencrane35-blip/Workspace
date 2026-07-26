# Workspace Recommendation Outcome & Feedback

Sprint 139 — complete the recommendation loop with outcome governance.

**Does not add learning automation.** Outcomes may *inform* future Adaptation proposals;
they must not silently rescore Attention/Decision or execute actions.

## Principle

```
Evidence
    ↓
Reasoning
    ↓
Recommendation (+ Identity / Provenance / Lifecycle)
    ↓
Experience (DisplayReason / optional trace)
    ↓
Human decision
    ↓
RecommendationOutcome
    ↓
[optional] Future Adaptation evidence (explicit, human-reviewed)
    ↓
[optional] ActionProposal → Permission Gateway → ExecutionOutcome
```

Questions this contract answers:

> What happened after this recommendation was presented?  
> Was rejection a failure? (No.)

---

## RecommendationOutcome model

Domain type: `RecommendationOutcome` in `packages/domain/src/action_proposal/`.

| Field | Meaning |
|-------|---------|
| `identity` | `RecommendationIdentity` reference |
| `provenance` | Frozen reasoning snapshot (immutable copy) |
| `lifecycle_resolution` | Accepted / Rejected / Expired / Superseded |
| `user_decision` | Human decision classification |
| `result_kind` | Resulting outcome kind (see below) |
| `recorded_at` | Timestamp |
| `quality` | Confidence-at-outcome / optional useful flag / notes |
| `experience_trace_match_keys` | Experience translation refs for the debug chain |
| `authority_effect` | Always `"none"` |

`RecommendationResultKind`:

| Kind | System failure? |
|------|-----------------|
| `accepted_follow_through` | No |
| `rejected_by_user` | **No** |
| `expired_without_action` | **No** |
| `superseded` | **No** |
| `downstream_execution_linked` | No (architecture link only) |

`RecommendationOutcome::attempt_execute()` hard-fails.

---

## Feedback boundary

| May | Must not |
|-----|----------|
| Inform future Adaptation proposals as **evidence** | Mutate historical `reasoning_origins` / provenance |
| Be reviewed by humans / Operator | Silently change Attention or Decision scoring |
| Link later to ExecutionOutcome via Gateway path | Trigger execution or grant capabilities |
| Attach Experience trace match keys | Treat rejection/expiry as system failure |

Adaptation remains proposal-only: accept → handoff → Pipeline → Gateway. Outcomes never
auto-apply workspace changes.

---

## Audit of existing “outcome” concepts (Sprint 139)

| Concept | Location | Class |
|---------|----------|-------|
| `DecisionOutcome` (open/selected/dismissed/…) | Decision Engine | **A — lifecycle state** |
| `RecommendationLifecycleState` / `RecommendationResolutionType` | Sprint 138 | **A — lifecycle state** |
| `ExecutionOutcome` | Execution services / audit | **B — true outcome** (post-Gateway) |
| `RecommendationOutcome` | Sprint 139 | **B — true outcome** (recommendation loop) |
| `AiProposalAuthorityOutcome` / AI evaluation classes | AI planning/evaluation | **C — diagnostic / authority class** |
| `ObservationTriggerOutcome` | Observation admission | **C — diagnostic telemetry** |
| Activity / Continuity “recent outcome” facets | Aggregators | **C — presentation telemetry** |
| Evolution `DecisionOutcome` event kind | Evolution model | **C — diagnostic telemetry** |
| PurposeOutcome attention signal | Attention | **C — signal**, not recommendation outcome |
| Adaptation accept status | Adaptation | **A — lifecycle**; handoff only |
| Durable feedback store for recommendation quality | — | **D — missing contract** (future) |
| Automatic score update from outcomes | — | **D — forbidden**, not missing |

---

## Experience traces ↔ Outcome chain

```
Evidence
    ↓
Reasoning (AttentionReason / DecisionReason)
    ↓
Recommendation (+ RecommendationIdentity)
    ↓
Experience (resolve_*_traced → match_key / DisplayReason)
    ↓
Human decision (lifecycle → Accepted / Rejected / …)
    ↓
RecommendationOutcome (provenance + experience_trace_match_keys + result_kind)
```

Debugging: start from `recommendation_outcome:{native_id}` → identity → provenance →
experience keys → lifecycle resolution → optional downstream ExecutionOutcome.

---

## Related docs

- [WORKSPACE-RECOMMENDATION-LIFECYCLE.md](./WORKSPACE-RECOMMENDATION-LIFECYCLE.md)
- [WORKSPACE-RECOMMENDATION-PROVENANCE.md](./WORKSPACE-RECOMMENDATION-PROVENANCE.md)
- [WORKSPACE-AUTOMATION-READINESS.md](./WORKSPACE-AUTOMATION-READINESS.md)
- [WORKSPACE-ADAPTATION-PROPOSAL.md](./WORKSPACE-ADAPTATION-PROPOSAL.md)
- [WORKSPACE-EXPERIENCE-DEBUGGING.md](./WORKSPACE-EXPERIENCE-DEBUGGING.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
