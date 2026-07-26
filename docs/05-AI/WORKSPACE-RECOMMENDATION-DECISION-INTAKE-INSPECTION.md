# Recommendation Decision Intake Inspection

| Field | Value |
|-------|-------|
| **Purpose** | Prove a future consumer can safely inspect intake without handoff / DE ownership |
| **Status** | Sprint 237 — verification contract only |
| **Authority** | Always `none` |

---

## Boundary

```
RecommendationDecisionIntakeRequest (assembled, non-authoritative)
        ↓ verify(intake, live context, confirmation, readiness)
RecommendationDecisionIntakeInspection
  safe_to_inspect = true | false
        ✗ inspection ≠ handoff
        ✗ inspection ≠ DE object / intent
        ✗ inspection ≠ Gateway / execution
        ↓ RecommendationDecisionIntakeCompatibility (version pin)
        ✗ compatible ≠ transfer / handoff
        ↓ (future DE adapter — separate increment)
```

`safe_to_inspect` means the package integrity checks pass for a consumer to *read* it.
It does **not** authorize handoff, Decision Engine object creation, or execution.

---

## Checks

| Check | Meaning |
|-------|---------|
| Non-authoritative | Intake asserts no handoff / DE object / wrong state |
| Fingerprint | Continuity fingerprint matches live context |
| Confirmation bound | Live confirmation is `confirmed` and matches intake intent/time |
| Context compatible | Context complete + readiness + identity fields align |
| Provenance intact | Evidence/explanation/related refs match live context |
| Ownership intact | RE owns intake package; DE does not own it yet |

---

## Guards

| Claim | Guard |
|-------|-------|
| Inspect ≠ handoff | `handoff_performed = false`; `attempt_handoff()` fails |
| Inspect ≠ DE ownership | `may_create_decision_engine_object() = false` |
| Inspect ≠ execution | `attempt_execute()` fails; `may_invoke_gateway() = false` |
| Confirmation still required | Unconfirmed confirmation → binding failed; `try_assemble` still gated |

---

## Related

- [WORKSPACE-RECOMMENDATION-DECISION-INTAKE-COMPATIBILITY.md](./WORKSPACE-RECOMMENDATION-DECISION-INTAKE-COMPATIBILITY.md)
- [WORKSPACE-RECOMMENDATION-DECISION-INTAKE.md](./WORKSPACE-RECOMMENDATION-DECISION-INTAKE.md)
- [WORKSPACE-RECOMMENDATION-DECISION-CONFIRMATION.md](./WORKSPACE-RECOMMENDATION-DECISION-CONFIRMATION.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
