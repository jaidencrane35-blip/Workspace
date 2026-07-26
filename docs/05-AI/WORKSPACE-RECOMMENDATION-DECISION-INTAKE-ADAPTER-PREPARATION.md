# Recommendation Decision Intake Adapter Preparation

| Field | Value |
|-------|-------|
| **Purpose** | Prepare a controlled RE → future-DE adapter path without invoking it |
| **Status** | Sprint 257 — preparation only |
| **Authority** | Always `none` |
| **Permission** | Always `none` |

---

## Boundary

```
PackageSeal (matching digest) + Confirmation (confirmed)
        ↓ try_prepare
RecommendationDecisionIntakeAdapterPreparation
  preparation_state = prepared | revoked
        ✗ prepare ≠ invoke adapter
        ✗ prepare ≠ DE object / intent / mapping
        ✗ prepare ≠ Gateway / ownership transfer
        ✗ revoke restores non-active preparation
        ↓ (future adapter invocation — separate increment; still DE-owned creation)
```

Preparation means the sealed intake is explicitly ready for a *future* adapter
to consider. It does **not** invoke that adapter or create Decision Engine objects.

---

## Requirements

1. Confirmation `confirmed` with `create_future_decision` or `request_action_review`
2. Package seal `sealed` + `package_matches_seal`
3. Intake digest matches seal

Seal mismatch → not active (`seal_aligned = false`); invoke remains denied.

---

## Guards

| Claim | Guard |
|-------|-------|
| Prepare ≠ invoke | `adapter_invoked = false`; `attempt_invoke_adapter()` fails |
| Prepare ≠ mapping | `mapping_performed = false`; `attempt_perform_mapping()` fails |
| Prepare ≠ DE ownership | `decision_engine_object_id = None`; create-DE fails |
| Prepare ≠ handoff | `handoff_performed = false`; `attempt_handoff()` fails |
| Prepare ≠ execution | `attempt_execute()` fails; `may_invoke_gateway() = false` |
| Ownership | `current_owner = recommendation_engine` |
| Reversible | `revoke()` → `revoked` without DE side effects |

---

## Related

- [WORKSPACE-RECOMMENDATION-DECISION-INTAKE-PACKAGE-SEAL.md](./WORKSPACE-RECOMMENDATION-DECISION-INTAKE-PACKAGE-SEAL.md)
- [WORKSPACE-RECOMMENDATION-DECISION-INTAKE-PROCEED-DENIAL.md](./WORKSPACE-RECOMMENDATION-DECISION-INTAKE-PROCEED-DENIAL.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
