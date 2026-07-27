# Decision Engine Candidate Creation Request

| Field | Value |
|-------|-------|
| **Purpose** | DE-owned request that eligible intake seek conversion into DecisionCandidate |
| **Status** | Projected only — not persisted |
| **Authority** | Always `none` |
| **Owner** | Decision Engine |

---

## Boundary

```
DecisionEngineIntakePromotionBoundary
        ↓ derive_batch (recomputable projection)
DecisionEngineCandidateCreationRequest
  request_state =
    not_requested | requested | rejected | created
        ✗ requested ≠ DecisionCandidate creation
        ✗ ≠ scoring / planner / Gateway / goals / intents
        ✗ never mutates recommendation_lifecycle
```

## Rules

| State | When |
|-------|------|
| `not_requested` | Promotion boundary not allowed (e.g. not_ready) |
| `requested` | `promotion_allowed` + disposition retained + acceptance valid + seal aligned |
| `rejected` | Promotion blocked (withdrawn / invalidated / seal / acceptance / non-retained) |
| `created` | Reserved for a future DecisionCandidate creation event — never set here |

## Persistence

Not persisted. Fully derivable from the promotion boundary.

## Related

- [WORKSPACE-DECISION-ENGINE-INTAKE-PROMOTION-BOUNDARY.md](./WORKSPACE-DECISION-ENGINE-INTAKE-PROMOTION-BOUNDARY.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
