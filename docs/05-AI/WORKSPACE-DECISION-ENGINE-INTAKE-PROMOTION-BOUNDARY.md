# Decision Engine Intake Promotion Boundary

| Field | Value |
|-------|-------|
| **Purpose** | DE-owned readiness gate for *future* DecisionCandidate promotion |
| **Status** | Projected only — not persisted |
| **Authority** | Always `none` |
| **Owner** | Decision Engine |

---

## Boundary

```
DecisionEngineIntakeDisposition
        ↓ derive_batch (recomputable projection)
DecisionEngineIntakePromotionBoundary
  boundary_state =
    not_ready | promotion_allowed | promotion_blocked | promoted
        ✗ promotion_allowed ≠ DecisionCandidate creation
        ✗ ≠ scoring / ranking / planner / Gateway
        ✗ never mutates recommendation_lifecycle
```

## Rules

| State | When |
|-------|------|
| `not_ready` | Evaluation incomplete or disposition not retained (and not hard-blocked) |
| `promotion_allowed` | Active + evaluation completed + disposition retained + acceptance valid + seal aligned |
| `promotion_blocked` | Withdrawn / invalidated / revoked acceptance / seal mismatch / dismissed or deferred disposition |
| `promoted` | Future promotion event marker only — not set by this boundary alone |

## Persistence

Not persisted. Fully derivable from intake candidate lifecycle, evaluation,
disposition, and seal/acceptance facts. `promoted` remains reserved for a
future promotion event sprint.

## Related

- [WORKSPACE-DECISION-ENGINE-INTAKE-DISPOSITION.md](./WORKSPACE-DECISION-ENGINE-INTAKE-DISPOSITION.md)
- [WORKSPACE-DECISION-ENGINE-CANDIDATE-CREATION-REQUEST.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-CREATION-REQUEST.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
