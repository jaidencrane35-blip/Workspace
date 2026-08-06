# Engineering Milestone Report
## P12 Window Provider

| Field | Value |
| --- | --- |
| **Execution program** | P12 Window Provider |
| **Date** | 2026-08-07 |
| **Prior** | P11 Application Provider (`330a26b`) |
| **Commit** | `8cefb88` |
| **Handoff** | `AWAITING_PROJECT_OWNER_WINDOW_PROVIDER_REVIEW` |
| **Index** | `docs/capability-runtime/00_INDEX.md` |

---

## Mission

Give Workspace governed native window control (where / how / state) via Window Provider Levels 1–2 on the frozen Capability Runtime — without redesigning UI or calling Application Provider.

---

## Decision

**ADAPT** existing Win32 ports. FancyZones-class UX = **STUDY** only. Hide = out of scope.

---

## Deliverables

| Artifact | Path |
| --- | --- |
| Window Provider | `packages/kernel/src/capability_runtime/window_provider.rs` |
| Operations Spec | `docs/capability-runtime/WINDOW_OPERATIONS_SPECIFICATION.md` |
| Provider doc | `docs/capability-runtime/WINDOW_PROVIDER.md` |
| IPC | `execute_window_operation` |
| Intent | `winEnumerate`, `winSnap`, `winMaximize`, … |

---

## User-visible

Conversation can list windows/monitors, read active/bounds, maximize, snap, center, and move windows between monitors.

---

## Explicit non-goals

P13+ providers · UI redesign · Level 3/4 layout intelligence · Hide windows

---

## Stop

Wait for Product Owner review before P13.  
