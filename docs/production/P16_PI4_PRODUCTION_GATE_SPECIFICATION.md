# P16.PI4 — Canonical Production Gate Specification

| Field | Value |
| --- | --- |
| **Kind** | Schema / authority strengthening (not gate implementation) |
| **Status** | Complete — normalize only |
| **Implemented gates in PI4?** | **None** |
| **Next ReadyNow (unchanged)** | `A1-artifact-checksums` |
| **Commit** | `5c38a9c` (`v2-dev`) |

---

## Decision

| Question | Result |
| --- | --- |
| Does A1 already satisfy the full Spec? | **No** (was partial prose) |
| Action | Establish Spec + normalize all units into identical schema |
| Implement A1? | **No** — await Owner review |
| Order redesigned? | **No** |
| Constitution / runtime / P17? | **No** |

---

## Artifacts

| Artifact | Role |
| --- | --- |
| `PRODUCTION_GATE_SPECIFICATION.md` | Binding schema + Operational Acceptance |
| `production-gates-dependency.json` v2 | Machine catalog (all fields) |
| `PRODUCTION_GATE_CATALOG.md` | Generated identical human sections |
| `sync-production-gate-catalog.mjs` | Generator |
| `verify-production-gate-specification.mjs` | Enforces schema |

---

## Operational Acceptance

Added as mandatory “A user can…” checklist per unit — distinct from Product Proof and from CI verification.

---

## Stop

Do not implement A1 or any other gate until Product Owner reviews PI4.  
No further meta-frameworks after this Spec.
