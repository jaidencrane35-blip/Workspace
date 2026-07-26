# Recommendation Decision Intake Proceed Denial

| Field | Value |
|-------|-------|
| **Purpose** | Type that intake compatibility is not proceed / consume permission |
| **Status** | Sprint 247 — denial / ownership clarity only |
| **Authority** | Always `none` |
| **Permission** | Always `none` |

---

## Boundary

```
RecommendationDecisionIntakeCompatibility (compatible = true | false)
        ↓ derive_from_compatibility
RecommendationDecisionIntakeProceedDenial
  eligibility_state = identity_pin_only | incompatible_blocked
  proceed_authorized = false (always)
        ✗ compatible ≠ proceed / consume / adapter
        ✗ declared_consumer role = future_reader_pin_only
        ✗ current_owner remains recommendation_engine
        ✗ no DE object / intent / Gateway / handoff
        ↓ (future DE adapter — separate increment)
```

`compatible` means a future consumer may **pin** package identity.
Proceed denial makes explicit that this pin is **not** permission to proceed.

---

## States

| State | Meaning |
|-------|---------|
| `identity_pin_only` | Compatible pin recorded; proceed/consume/adapter still denied |
| `incompatible_blocked` | Identity mismatch / invalid inspection; progression denied |

---

## Guards

| Claim | Guard |
|-------|-------|
| Compatible ≠ proceed | `proceed_authorized = false`; `attempt_authorize_proceed()` fails |
| Compatible ≠ consume | `consume_authorized = false`; `attempt_consume()` fails |
| Compatible ≠ adapter | `adapter_invokable = false`; `attempt_invoke_adapter()` fails |
| Compatible ≠ DE ownership | `decision_engine_object_id = None`; create-DE fails |
| Compatible ≠ handoff | `handoff_performed = false`; `attempt_handoff()` fails |
| Compatible ≠ execution | `attempt_execute()` fails; `may_invoke_gateway() = false` |
| Ownership | `current_owner = recommendation_engine` |

---

## Related

- [WORKSPACE-RECOMMENDATION-DECISION-INTAKE-COMPATIBILITY.md](./WORKSPACE-RECOMMENDATION-DECISION-INTAKE-COMPATIBILITY.md)
- [WORKSPACE-RECOMMENDATION-DECISION-INTAKE-INSPECTION.md](./WORKSPACE-RECOMMENDATION-DECISION-INTAKE-INSPECTION.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
