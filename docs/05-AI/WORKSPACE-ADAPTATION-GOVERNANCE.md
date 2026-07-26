# Workspace Adaptation Governance

Sprint 140 — safe adaptation boundaries before any learning behaviour exists.

**No automatic learning.** Outcomes may inform proposals; humans must review; Gateway
still owns execution authority.

## Principle

### Allowed

```
RecommendationOutcome
    ↓
OutcomeAdaptationProposal (review required)
    ↓
Explicit review / approval for handoff
    ↓
[optional] Intent → Command Pipeline → Permission Gateway → Execution
    ↓
Future behaviour change (only via governed path)
```

### Forbidden

```
RecommendationOutcome
    ↓
silent scoring mutation   ← NEVER
```

Also forbidden:

- Auto-apply Adaptation without review
- Mutate historical `AttentionReason` / provenance from outcomes
- Retune Attention/Decision weights from feedback without an explicit, reviewed proposal + Gateway path
- Treat Adaptation accept as execution

---

## Audit of adaptation / learning concepts (Sprint 140)

| Concept | Location | Class |
|---------|----------|-------|
| `AdaptationProposal` / status overlays | Workspace Adaptation Engine | **A — Existing state tracking** (proposal lifecycle) |
| `AdaptationHandoff` | Adaptation accept | **A — Existing state tracking** (handoff only) |
| User preferences / personalization highlights | AI personalization → Decision context | **A — Existing state tracking** (explicit prefs, not outcome learning) |
| Working Style preference lines | Working Style model | **A / B — Observed vs preferred** (diagnostic presentation) |
| Recommendation / Decision confidence bands | Cognition display metadata | **B — Diagnostic feedback** |
| `RecommendationOutcome` quality flags | Sprint 139 | **B — Diagnostic feedback** / **C — Future adaptation candidate** |
| `OutcomeAdaptationProposal` | Sprint 140 | **C — Future adaptation candidate** |
| Experience translation traces | Experience debugging | **B — Diagnostic feedback** |
| AI evaluation / proposal outcome classes | AI evaluation | **B — Diagnostic feedback** |
| Automatic weight/score calibration from outcomes | — | **D — Unsafe mutation path** (must not exist) |
| Silent preference injection from rejection stats | — | **D — Unsafe mutation path** (must not exist) |

---

## OutcomeAdaptationProposal architecture

Domain type: `OutcomeAdaptationProposal` in `packages/domain/src/action_proposal/`.

Distinct from workspace `AdaptationProposal` (Pattern/OS aggregation). This type is the
**outcome → adaptation** governance bridge.

| Field | Meaning |
|-------|---------|
| `source_outcome_id` | `RecommendationOutcome` id |
| `source_recommendation_id` | Native recommendation id |
| `provenance` | Frozen reasoning snapshot |
| `affected_area` | What area would change (presentation, workflow hint, …) |
| `proposed_change` | Human-readable proposed change |
| `expected_effect` | Expected effect if later pursued |
| `confidence` | Copied outcome confidence metadata |
| `review_required` | Always `true` |
| `review_status` | Proposed → AwaitingReview → Reviewed → ApprovedForHandoff \| Rejected |
| `authority_effect` | Always `"none"` |

Rules:

- `may_auto_apply() == false`
- `may_mutate_cognition() == false`
- `may_silently_change_scoring() == false`
- `attempt_apply()` / `attempt_execute()` hard-fail
- Approve-for-handoff still does **not** execute — only unlocks a future Intent/Gateway path
- Sprint 141: self-approval forbidden; Applied/Evaluated are future-only — see
  [WORKSPACE-ADAPTATION-REVIEW.md](./WORKSPACE-ADAPTATION-REVIEW.md)

Existing `AdaptationProposal` remains the Pattern/OS improvement aggregator; both stay
non-executing and require human review before any handoff.

---

## Provenance chain

```
Evidence
    ↓
Reasoning
    ↓
Recommendation (+ Identity / Lifecycle)
    ↓
Experience (DisplayReason / traces)
    ↓
Decision (human)
    ↓
RecommendationOutcome
    ↓
OutcomeAdaptationProposal (explicit review)
    ↓
[optional] ActionProposal / Intent → Permission Gateway → Execution
```

Provenance remains immutable across outcome recording and adaptation proposal creation.

---

## Related docs

- [WORKSPACE-ADAPTATION-REVIEW.md](./WORKSPACE-ADAPTATION-REVIEW.md)
- [WORKSPACE-CONTROLLED-CHANGE.md](./WORKSPACE-CONTROLLED-CHANGE.md)
- [WORKSPACE-GOVERNANCE-LEDGER.md](./WORKSPACE-GOVERNANCE-LEDGER.md)
- [WORKSPACE-ADAPTATION-PROPOSAL.md](./WORKSPACE-ADAPTATION-PROPOSAL.md)
- [WORKSPACE-RECOMMENDATION-OUTCOME.md](./WORKSPACE-RECOMMENDATION-OUTCOME.md)
- [WORKSPACE-RECOMMENDATION-LIFECYCLE.md](./WORKSPACE-RECOMMENDATION-LIFECYCLE.md)
- [WORKSPACE-AUTOMATION-READINESS.md](./WORKSPACE-AUTOMATION-READINESS.md)
- [WORKSPACE-COGNITION-PIPELINE-CONTRACT.md](./WORKSPACE-COGNITION-PIPELINE-CONTRACT.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
