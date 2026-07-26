# Recommendation Decision Handoff Request

| Field | Value |
|-------|-------|
| **Purpose** | Typed non-executing RE → future-DE handoff *request* artifact |
| **Status** | Sprint 262 — request only |
| **Authority** | Always `none` |
| **Permission** | Always `none` |

---

## Boundary

```
AdapterPreparation (active) + Confirmation + PackageSeal + Compatibility
        ↓ try_request
RecommendationDecisionHandoffRequest
  request_state = requested | revoked
  handoff_requested = true | false
  handoff_performed = false (always)
  decision_engine_object_id = None (always)
        ✗ request ≠ preparation
        ✗ request ≠ performed handoff
        ✗ request ≠ DE object / intent
        ✗ request ≠ adapter invoke / Gateway / ownership transfer
        ✗ revoke blocks progression
        ↓ (future DE acceptance — separate increment; still not execution)
```

A handoff request means Recommendation Engine asks a *future* Decision Engine to
consider the sealed, prepared intake. It does **not** perform handoff or create
Decision Engine objects. Ownership remains Recommendation Engine.

---

## Requirements

1. Active adapter preparation (`prepared` + seal aligned)
2. Confirmation `confirmed` with future-decision intent
3. Package seal `sealed` + matching digest
4. Compatibility identity pin (`compatible`)

Revoked preparation → request cannot be issued; rebound request clears
`handoff_requested`.

---

## Guards

| Claim | Guard |
|-------|-------|
| Prep ≠ handoff | Preparation has no `handoff_requested`; request is a separate artifact |
| Request ≠ performed | `handoff_performed = false`; `attempt_perform_handoff()` fails |
| Request ≠ DE object | `decision_engine_object_id = None`; create-DE fails |
| Request ≠ execution | `attempt_execute()` fails; `may_invoke_gateway() = false` |
| Ownership | `current_owner = recommendation_engine` |
| Provenance | Seal digest + confirmation + compatibility refs immutable |
| Reversible | `revoke()` → `revoked`; prep revoke cascades |

---

## Related

- [WORKSPACE-RECOMMENDATION-DECISION-INTAKE-ADAPTER-PREPARATION.md](./WORKSPACE-RECOMMENDATION-DECISION-INTAKE-ADAPTER-PREPARATION.md)
- [WORKSPACE-RECOMMENDATION-DECISION-INTAKE-PACKAGE-SEAL.md](./WORKSPACE-RECOMMENDATION-DECISION-INTAKE-PACKAGE-SEAL.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
