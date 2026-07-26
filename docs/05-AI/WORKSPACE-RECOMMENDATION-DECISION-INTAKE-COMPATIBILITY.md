# Recommendation Decision Intake Compatibility

| Field | Value |
|-------|-------|
| **Purpose** | Pin versioned intake package identity for a future consumer |
| **Status** | Sprint 242 — identity / field-floor only |
| **Authority** | Always `none` |

---

## Boundary

```
RecommendationDecisionIntakeInspection (safe_to_inspect)
        ↓ derive_from_inspection
RecommendationDecisionIntakeCompatibility
  contract_version = recommendation_decision_intake:v1
  compatible = true | false
        ✗ compatible ≠ transfer / migrate / handoff
        ✗ declared_consumer ≠ current owner
        ✗ no DE object / intent / Gateway
        ↓ (future DE adapter — separate increment)
```

`compatible` means a future consumer may **pin** this package identity.
It does **not** authorize ownership transfer, migration, handoff, or execution.

---

## Identity

| Field | Value |
|-------|-------|
| `contract_family` | `recommendation_decision_intake` |
| `contract_version` | `recommendation_decision_intake:v1` |
| `schema_version` | `1` |
| `producer` | `recommendation_engine` |
| `declared_consumer` | `decision_engine` (reader pin only) |

---

## Gates (all required for `compatible`)

1. Inspection `safe_to_inspect` / `valid`
2. Expected contract + schema match producer pin
3. Required field floor satisfied
4. Intake remains non-authoritative (no DE object, handoff false)

Invalid / stale inspection → `compatible = false` (cannot progress).

---

## Guards

| Claim | Guard |
|-------|-------|
| Compatible ≠ transfer | `transfer_authorized = false`; `attempt_transfer()` fails |
| Compatible ≠ migrate | `may_migrate = false`; `attempt_migrate()` fails |
| Compatible ≠ handoff | `handoff_performed = false`; `attempt_handoff()` fails |
| Compatible ≠ DE ownership | `decision_engine_object_id = None`; create-DE fails |
| Compatible ≠ execution | `attempt_execute()` fails; `may_invoke_gateway() = false` |

---

## Related

- [WORKSPACE-RECOMMENDATION-DECISION-INTAKE-INSPECTION.md](./WORKSPACE-RECOMMENDATION-DECISION-INTAKE-INSPECTION.md)
- [WORKSPACE-RECOMMENDATION-DECISION-INTAKE.md](./WORKSPACE-RECOMMENDATION-DECISION-INTAKE.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
