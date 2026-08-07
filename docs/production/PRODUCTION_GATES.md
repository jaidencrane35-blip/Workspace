# Production Gates Maturity Model

| Field | Value |
| --- | --- |
| **Kind** | Production delivery framing (not constitutional law) |
| **Gate schema** | **`PRODUCTION_GATE_SPECIFICATION.md`** (mandatory identical shape) |
| **Execution / order** | **`PRODUCTION_DEPENDENCY_AUTHORITY.md`** |
| **Catalog** | `PRODUCTION_GATE_CATALOG.md` ← `production-gates-dependency.json` |
| **Readiness facts** | `production-readiness.json` |

A production change is valuable only if it **measurably** improves production readiness.

**Never confuse:** Engineering Complete ≠ Production Ready ≠ Release Ready  
**Never confuse:** Verifier green ≠ Operational Acceptance (human trust)

---

## Four questions (every gate unit)

### Before

1. Why is this the highest production risk among **ReadyNow** units?  
2. Why now?  
3. What user problem disappears?  
4. How will success be measured?

### During

Minimum required · No speculative infrastructure · No “while we’re here…”

### After

Readiness improved? · Independently verified? · Operational Acceptance reviewed? · Complexity ≤ value?

### Before moving on

Stop? Or is the next unit in the **canonical order** objectively justified and ReadyNow?

---

## Gates (summary)

| Gate | Focus | Status |
| --- | --- | --- |
| **A — Distribution** | Installer, signing, checksums, rollback | A0 done; A1 next ReadyNow; A2 external |
| **B — Operations** | Diagnostics, updater, crash | B1 done; B2 blocked by A2 |
| **C — Reliability** | Single instance, validation, shutdown | C0 done |
| **D — Security** | IPC, permissions | Open |
| **E — Experience** | Tray, polish | Open |
| **F — Release** | CI, signed release | Open |

---

## Rule

1. Unit must satisfy **Gate Specification** schema.  
2. Select next unit from **Dependency Authority** order.  
3. Do not jump blocked units (especially updater before signing).  
4. No further meta-frameworks — execute gates.
