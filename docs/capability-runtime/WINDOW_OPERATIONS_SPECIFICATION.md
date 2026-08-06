# Window Operations Specification
## Permanent contract (P12)

Application Provider answers **what application**.  
Window Provider answers **where / how / what state**.

---

## Discovery (Level 1)

| Operation | Result |
| --- | --- |
| Enumerate windows | List of window descriptors |
| Active window | Foreground window or not_found |
| Find by title / process / executable hint | Matching windows |
| Bounds | Geometry + monitor index |
| Monitors | Attached display work areas |

---

## Focus (Level 2)

| Operation | Notes |
| --- | --- |
| Focus / activate / bring to front | Win32 `SetForegroundWindow` — may be refused by OS |

---

## State (Level 2)

| Operation | Notes |
| --- | --- |
| Minimize | `ShowWindow(SW_MINIMIZE)` |
| Restore | `ShowWindow(SW_RESTORE)` |
| Maximize | `ShowWindow(SW_MAXIMIZE)` |
| Hide | **Out of scope** (P12) |

---

## Placement (Level 2)

| Operation | Notes |
| --- | --- |
| Move | Absolute x/y or move to monitor work area |
| Resize | Width/height (min 100×80) |
| Center | Center on target/current monitor work area |
| Snap | Half-work-area left/right/top/bottom |
| Move between monitors | `move` + `monitorIndex` |

---

## Level 3 (prepared, not implemented)

Arrange workspace · restore multi-window layouts · monitor layout packs  
These must compose Window operations through the Capability Runtime — never provider-to-provider calls.

---

## Level 4 (future)

Suggest layouts · prepare workspace · workflow-aware positioning (Intelligence).  
