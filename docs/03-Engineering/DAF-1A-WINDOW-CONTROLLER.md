# DAF-1a — WindowController Foundation

| Field | Value |
|-------|-------|
| **Purpose** | OS window mutation boundary for Desktop Arrangement Foundation |
| **Batch** | DAF-1a |
| **Owner** | `workspace-windows-integration` |
| **Status** | Implemented (foundation) |
| **Related** | [DAF-ARCHITECTURE-AUDIT.md](DAF-ARCHITECTURE-AUDIT.md), [HUMAN-REVIEW-POLICY.md](HUMAN-REVIEW-POLICY.md) |

---

## 1. Why this exists

DAF must arrange **real** desktop windows. Observation (capture) is not enough. A dedicated control surface is required so kernel/UI never call Win32 directly.

---

## 2. Problem it solves

Provides a single, testable API to:

* Set window bounds (move + resize)
* Focus a window

…with a stub for non-Windows CI and a Win32 implementation on Windows.

---

## 3. Responsibilities

* Parse HWND identity strings used by capture (`0x…`)
* Validate bounds (finite, positive size)
* Perform OS mutations **only** inside this crate
* Report whether an operation was simulated (stub) or real (Win32)

---

## 4. Non-responsibilities

* Permission / capability checks (kernel Permission Gateway)
* Persistence of arrangements (later DAF domain/SQLite)
* IPC / UI
* Assistant or recommendation logic
* Audio, grouping, work-mode transforms
* Launching processes (existing `ProcessLauncher`)
* Capturing desktop state (existing `DesktopCapturer`)

---

## 5. Integration points

```text
Future: Kernel DesktopArrangementService
    → WindowController (this batch)
        → Win32 SetWindowPos / SetForegroundWindow
        → Stub records ops (non-Windows / tests)

Existing: DesktopCapturer / WindowEnumerator (read-only)
```

Factory: `platform_window_controller()` mirrors other platform factories in `lib.rs`.

---

## 6. Ownership

| Layer | Role |
|-------|------|
| `windows-integration` | **Owns** control API |
| `kernel` | Will call controller after Gateway (DAF-1c+) — not in this batch |
| Assistant | Must never call controller |

---

## 7. Human review

**Not required** for DAF-1a (see [HUMAN-REVIEW-POLICY.md](HUMAN-REVIEW-POLICY.md)).
