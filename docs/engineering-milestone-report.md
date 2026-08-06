# Engineering Milestone Report
## P5 Native Desktop Operator Refoundation

| Field | Value |
| --- | --- |
| **Execution program** | P5 Native Desktop Operator Refoundation |
| **Date** | 2026-08-07 |
| **Prior** | P4 Native Windows Shell Lifecycle (`23d9d01`) |
| **Repository version** | `0.1.0` |
| **Commit** | _(filled after commit)_ |
| **Handoff** | `AWAITING_PROJECT_OWNER_OPERATOR_REFOUNDATION_REVIEW` |
| **Detail** | `docs/execution-program-native-desktop-operator-refoundation.md` |

---

## Executive Summary

Owner review rejected polishing the four-mode shell. P5 **replaces** that model with two forms only: **Desktop Operator** and **Conversation Window**. Expanded/Settings/Hide are gone as shell states. Tools are conversation satellites. Collapse/Close return to the operator; Exit exits.

---

## Shell architecture

| Form | Window |
| --- | --- |
| A — Desktop Operator | `operator` |
| B — Conversation | `main` (resizable, restorable) |

---

## Validation (after run)

| Signal | Status |
| --- | --- |
| Typecheck / test / build | _(validation)_ |
| `verify:shell-zero-trap` | two-form graph |
| Fresh launch for review | required |

---

## Product Owner checklist

- [ ] Operator idle on launch
- [ ] Click ↔ Conversation deterministic
- [ ] Close → Operator; Exit → quit
- [ ] Drag works; no Expand/Settings
- [ ] Clean runtime / no terminal leakage

---

## Stop

**Do not begin another execution program until Product Owner approval.**
