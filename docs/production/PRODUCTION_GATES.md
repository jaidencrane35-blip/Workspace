# Production Gates Maturity Model

| Field | Value |
| --- | --- |
| **Kind** | Production delivery framing (not constitutional law) |
| **Execution authority** | **`PRODUCTION_DEPENDENCY_AUTHORITY.md`** (canonical — Track A sequencing) |
| **Machine index** | `production-gates-dependency.json` |
| **Readiness facts** | `production-readiness.json` |

A production change is valuable only if it **measurably** improves production readiness.

**Never confuse:** Engineering Complete ≠ Production Ready ≠ Release Ready  
(See Dependency Authority §1.)

---

## Four questions (every gate unit)

### Before

1. Why is this the highest production risk among **Ready Now** units?  
2. Why now?  
3. What user problem disappears?  
4. How will success be measured?

### During

Minimum required · No speculative infrastructure · No “while we’re here…”

### After

Readiness improved? · Independently verified? · Can it ship at its level? · Complexity ≤ value?

### Before moving on

Stop? Or is the next unit in the **canonical order** objectively justified and Ready Now?

---

## Gates (summary)

| Gate | Focus | Status |
| --- | --- | --- |
| **A — Distribution** | Installer, signing, upgrade, repair, rollback, verification | Partial (A0 done; A2 external) |
| **B — Operations** | Updater, diagnostics, support bundle, crash, recovery | B1 next per authority |
| **C — Reliability** | Single instance, startup, shutdown, recovery, validation | C0 done |
| **D — Security** | IPC, permissions, least privilege | Open |
| **E — Experience** | Tray, animations, a11y, performance | Open |
| **F — Release** | CI, signing automation, artifacts, notes | Open |

---

## Rule

Select the next unit from **`PRODUCTION_DEPENDENCY_AUTHORITY.md` canonical execution order**.  
Do not invent order. Do not jump blocked units (especially updater before signing).
