# Engineering Milestone Report
## P4 Native Windows Shell Lifecycle (Desktop Operator Foundation)

| Field | Value |
| --- | --- |
| **Execution program** | P4 Native Windows Shell Lifecycle |
| **Date** | 2026-08-07 |
| **Prior** | P3 Windows Shell Completion (`0a50f19`) |
| **Repository version** | `0.1.0` |
| **Commit** | `23d9d01` |
| **Handoff** | `AWAITING_PROJECT_OWNER_DESKTOP_OPERATOR_FOUNDATION_REVIEW` |
| **Detail** | `docs/execution-program-native-windows-shell-lifecycle.md` |

---

## Executive Summary

This program hardened Workspace as a **native Windows shell lifecycle** — not AI, not automation, not memory. Floating operator is a first-class companion; Settings completes the operator menu; Zero-Trap remains law; development tooling stays out of the product runtime.

---

## Shell architecture summary

| Window | Modes |
| --- | --- |
| `operator` | 0 — floating W (always available, movable, recoverable) |
| `main` | 1 Compact · 2 Expanded · 3 Specialized (Settings / tools / health) |

Durable: mode, positions, hidden flag, specialized target.

---

## State transition diagram

See mermaid in `docs/execution-program-native-windows-shell-lifecycle.md`.

Core recoveries: **0↔1↔2↔3**, **Hide→taskbar**, **Exit→process end**, **main X→Mode 0**.

---

## Zero-Trap verification checklist

See execution program §3. Automated: `pnpm verify:shell-zero-trap` + `tests/shell-zero-trap.test.ts`.

---

## Windows lifecycle summary

Close→collapse; Hide→taskbar; focus/z-order polished; Settings on operator menu; left-click→Compact; double-click→Expand; quiet logging / no console in release.

---

## Repository health summary

| Signal | Status |
| --- | --- |
| Typecheck / build / tests | pass (after validation) |
| `verify:shell-zero-trap` | Settings + Mode 0→3 covered |
| `cargo check -p workspace-app` | pass |
| Current program | `native-windows-shell-lifecycle-p4` |

---

## Product Owner review checklist

- [ ] Floating → Compact (left click)
- [ ] Compact → Expanded
- [ ] Expanded → Compact
- [ ] Compact → Floating
- [ ] Floating → Expanded (double click)
- [ ] Floating → Settings → Compact
- [ ] Recovery from every state without Task Manager
- [ ] No dead ends / no duplicate windows
- [ ] No terminal leakage in product UX

---

## Next program (after approval only)

Documentation authority convergence (Phase A / G3) — or owner redirect.

---

## Stop

Launched fresh for review. **Do not begin another execution program until Product Owner approval.**
