# Recommendation Decision Intake Package Seal

| Field | Value |
|-------|-------|
| **Purpose** | Freeze intake package digest after proceed denial |
| **Status** | Sprint 252 — artifact freeze only |
| **Authority** | Always `none` |
| **Permission** | Always `none` |

---

## Boundary

```
RecommendationDecisionIntakeProceedDenial (proceed always denied)
        ↓ derive_from_proceed_denial
RecommendationDecisionIntakePackageSeal
  sealed = true
  intake_package_digest = digest(recommendation_decision_intake:v1:…)
        ✗ seal ≠ proceed / consume / adapter
        ✗ seal ≠ handoff / DE ownership
        ✗ seal_mismatch blocks progression
        ↓ RecommendationDecisionIntakeAdapterPreparation (prepare ≠ invoke)
        ↓ (future adapter invocation — separate increment)
```

Seal freezes the RE-owned intake snapshot. It does **not** authorize proceed,
adapter invocation, ownership transfer, or execution.

---

## States

| State | Meaning |
|-------|---------|
| `sealed` | Digest matches live package (or freshly sealed) |
| `seal_mismatch` | Live package drifted from sealed digest — cannot progress |

---

## Guards

| Claim | Guard |
|-------|-------|
| Seal ≠ mutate | `attempt_mutate_after_seal()` fails when `sealed` |
| Seal ≠ proceed | `proceed_authorized = false`; authorize-proceed fails |
| Seal ≠ adapter | `adapter_invokable = false`; invoke-adapter fails |
| Seal ≠ DE ownership | `decision_engine_object_id = None`; create-DE fails |
| Seal ≠ handoff | `handoff_performed = false`; `attempt_handoff()` fails |
| Seal ≠ execution | `attempt_execute()` fails; `may_invoke_gateway() = false` |
| Drift blocked | `assert_matches_intake()` fails on mismatch |

---

## Related

- [WORKSPACE-RECOMMENDATION-DECISION-INTAKE-PROCEED-DENIAL.md](./WORKSPACE-RECOMMENDATION-DECISION-INTAKE-PROCEED-DENIAL.md)
- [WORKSPACE-RECOMMENDATION-DECISION-INTAKE-COMPATIBILITY.md](./WORKSPACE-RECOMMENDATION-DECISION-INTAKE-COMPATIBILITY.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
