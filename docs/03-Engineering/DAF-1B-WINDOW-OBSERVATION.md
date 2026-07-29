# DAF-1b — Window Observation & Identity Foundation

| Field | Value |
|-------|-------|
| **Purpose** | Let Workspace see and identify real desktop windows before arranging them |
| **Batch** | DAF-1b |
| **Owner** | `workspace-windows-integration` (capture) + `workspace-domain` / `workspace-kernel` observation (durable facts) |
| **Status** | Implemented (foundation completion on existing stack) |
| **Related** | [DAF-1A-WINDOW-CONTROLLER.md](DAF-1A-WINDOW-CONTROLLER.md), [DAF-ARCHITECTURE-AUDIT.md](DAF-ARCHITECTURE-AUDIT.md) |

---

## 1. Why this exists

> Workspace must first be able to see and identify desktop applications reliably before it can manage them.

DAF-1a added **control**. DAF-1b completes **observation & identity** so later arrangement batches have stable, factual window representations.

This is **not** AI, Assistant, automation, or layout UI.

---

## 2. Problem it solves

- Capture incomplete process metadata (`process_name` was dropped)
- Need explicit **availability** diagnostics when a referenced window is gone
- Need platform limitations to be **visible** (Win32 vs stub)
- Need a clear contract: observation ≠ control ≠ intelligence

---

## 3. Responsibilities (this batch owns)

* Window observation contracts (capture DTO + domain `ObservedWindow`)
* Window identity representation (`hwnd`, `stable_window_id`, identity registry)
* Window metadata capture (title, bounds, process id/name, visibility, focus, minimize)
* Current-state observation snapshots
* Diagnostics for unavailable / missing windows
* Platform observation abstraction (`DesktopCapturer` + stub/Win32)

---

## 4. Non-responsibilities

* Window movement / resize / focus **execution** (DAF-1a controller; callers later)
* Grouping, desktop arrangements, layout modes
* AI suggestions, Assistant actions, recommendations, automation
* Permissions / Permission Gateway policy
* Persistence of **user arrangements** (later DAF)
* UI chrome
* Matching windows to registered `ApplicationId` (Environment heuristic — not observation)

---

## 5. Architecture

```text
Operating System
        │
        ▼
windows-integration (DesktopCapturer)
        │  CapturedDesktopWindow (+ process_name)
        ▼
kernel WorkspaceObservationService
        │  ObservedWindow + ObservationWindowIdentity
        ▼
Future Desktop Arrangement Domain
        │
        ▼
Workspace UI
```

**Do not** bypass `windows-integration`.  
**Do not** create a second OS integration crate.  
**Do not** attach observation to Assistant.

### Edit-in-place (no duplicate window model)

| Existing type | DAF-1b change |
|---------------|---------------|
| `CapturedDesktopWindow` | Add `process_name` |
| Win32 / stub capture | Populate `process_name` |
| `map_window` | Copy `process_name` (was hardcoded `None`) |
| `ObservedWindow` / identities | Already owned observation — extend with availability helpers |
| `WindowController` | Untouched; observation must not call it |

---

## 6. Data rules

Observed data is:

* **factual**
* **current-state** (per capture/pass)
* **non-authoritative** (`authority_effect: none`)

Do **not** infer user intent, importance, preferred layout, productivity, or recommended grouping.

---

## 7. Platform limitations

| Platform | Behaviour |
|----------|-----------|
| Windows | Real Win32 enumeration; process image name best-effort |
| Linux / CI | Deterministic stub capture; `real_os_observation = false` |
| Stub | Must not pretend to be Win32 |

`CaptureMetadata` exposes `real_os_observation` and `process_names_available`.

---

## 8. Availability diagnostics

`ObservedWindowAvailability` answers whether a hwnd / stable identity is present in a snapshot:

* `Available` — window row present
* `IdentityKnownWindowMissing` — identity exists but hwnd not in pass windows
* `Unavailable` — neither hwnd nor identity found

Pass-level `WorkspaceObservationStatus` / freshness remains for “observation system unavailable”.

---

## 9. Future extension points

* Arrangement domain references `stable_window_id` / `hwnd` / process facts
* Kernel commands that **read** observation then call `WindowController` (gated)
* Optional richer process image path (still factual)

---

## 10. Human review

**Not required** — backend/platform foundation ([HUMAN-REVIEW-POLICY.md](HUMAN-REVIEW-POLICY.md)).
