# Production Gates Maturity Model

| Field | Value |
| --- | --- |
| **Kind** | Production delivery framing (not constitutional law) |
| **Replaces** | Ad-hoc “Slice N” sequencing as the primary mental model |
| **Authority for readiness facts** | `production-readiness.json` + gate execution reports |

A production change is valuable only if it **measurably** improves production readiness.

---

## Four questions (every gate)

### Before implementation

1. Why is this the highest production risk?  
2. Why now?  
3. What user problem disappears?  
4. How will success be measured?

### During implementation

- Minimum required only  
- No speculative infrastructure  
- No “while we’re here…”

### After implementation

1. Did production readiness actually improve?  
2. Can it be independently verified?  
3. Can it ship?  
4. Did complexity increase more than value?

### Before moving on

- Stop? Or is another gate objectively justified?

---

## Gates

| Gate | Focus | Status |
| --- | --- | --- |
| **A — Distribution** | Installer, signing, upgrade, repair, rollback, verification | **Partial** — NSIS foundation (PI1 S1); signing open |
| **B — Operations** | Updater, diagnostics, support bundle, crash reporting, recovery | Open |
| **C — Reliability** | Single instance, startup, shutdown, recovery, config/DB validation | **In progress** — single-instance closed in PI2 |
| **D — Security** | IPC reduction, permission audit, least privilege, release hardening | Open |
| **E — Experience** | Tray, animations, a11y, notifications polish, performance | Open |
| **F — Release** | CI, signing automation, artifact verification, release notes, rollback docs | Open |

---

## Rule

Do not implement the next listed feature.  
**Close the highest-risk remaining gate** (or the minimum completable unit that removes that risk), verify, stop, await Owner review.
