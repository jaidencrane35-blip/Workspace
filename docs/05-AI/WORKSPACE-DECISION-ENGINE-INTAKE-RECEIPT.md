# Decision Engine Intake Receipt

| Field | Value |
|-------|-------|
| **Purpose** | DE-owned observational receipt of an accepted RE sealed package |
| **Status** | Observational foundation only |
| **Authority** | Always `none` |
| **Owner** | Decision Engine (type + projection); RE remains package owner |

---

## Boundary

```
RE: DecisionEngineAcceptance (accepted) + PackageSeal
        ↓ try_observe (read-only; no RE mutation)
DE: DecisionEngineIntakeReceipt
  receipt_state = observed | seal_mismatch
  ownership_transferred = false
  current_owner = recommendation_engine
  decision_engine_object_id = None
  handoff_command = None
        ✗ observe ≠ DecisionCandidate
        ✗ observe ≠ goal / intent / planner
        ✗ observe ≠ adapter / Gateway / ownership transfer
        ↓ (future DE-owned object creation — separate)
```

---

## Why not reuse existing DE types

| Existing type | Why not |
|---------------|---------|
| `DecisionCandidate` | Scored planning opportunity with `submit_assistant_goal` |
| `DecisionContext` | Synthesis count snapshot, not external intake |
| `DecisionEngineHandoff` | Planner handoff after select |
| `DecisionEngineOverlay` | Lifecycle for `engine_decision:*` only |
| `DecisionItem` (Queue) | Human inbox of blocked/approval/intent sources |

---

## Related

- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
- [WORKSPACE-RECOMMENDATION-DECISION-ENGINE-ACCEPTANCE.md](./WORKSPACE-RECOMMENDATION-DECISION-ENGINE-ACCEPTANCE.md)
