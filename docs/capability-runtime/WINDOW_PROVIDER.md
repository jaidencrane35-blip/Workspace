# Window Provider — P12

| Field | Value |
| --- | --- |
| **Name** | WindowProvider |
| **Domain** | `window` |
| **Purpose** | Discover, focus, place, and inspect native windows under Workspace governance |
| **Adoption** | **ADAPT** existing Win32 ports (`WindowEnumerator`, `WindowMutator`, `DesktopCapturer`) |
| **Rejected identity** | FancyZones / AutoHotkey as product |
| **Independence** | Never calls ApplicationProvider — Runtime routes only |
| **Product Proof** | P12.5 — Conversation integration (`product-proof/WINDOW_PROVIDER_PRODUCT_PROOF.md`) |

---

## Capability levels

| Level | Scope | P12 |
| --- | --- | --- |
| **1 Read** | Enumerate, active, find, bounds, monitors | **Implemented** |
| **2 Operate** | Focus, minimize, restore, maximize, move, resize, center, snap | **Implemented** |
| **3 Compose** | Arrange workspace / multi-window layouts | Prepared in docs — not executed |
| **4 Intelligent** | Suggest layouts / workflow positioning | Future |

Hide is **not** implemented (risk of lost windows).

---

## Operations

| Operation | Capability | Examples |
| --- | --- | --- |
| `enumerate` | `window.read` | “List windows” |
| `active` | `window.read` | “Active window” |
| `find` | `window.read` | (internal / query) |
| `bounds` | `window.read` | “Where is Notepad” |
| `monitors` | `window.read` | “List monitors” |
| `focus` | `window.focus` | “Focus window Chrome” |
| `minimize` / `restore` / `maximize` | `window.state` | “Maximize Notepad” |
| `move` / `resize` / `center` / `snap` | `window.place` | “Snap Chrome left” / “Move X to monitor 1” |

---

## Arguments

`query` · `hwnd` · `pid` · `path` · `x` · `y` · `width` · `height` · `monitorIndex` · `snap` (`left\|right\|top\|bottom`)

---

## Ownership

| Layer | Location |
| --- | --- |
| Provider | `packages/kernel/src/capability_runtime/window_provider.rs` |
| Command | `ExecuteWindowOperation` |
| IPC | `execute_window_operation` |
| Intent | `winEnumerate`, `winSnap`, `winMaximize`, … |

---

## Future (Level 3+)

- Arrange / restore multi-window layouts via composed Runtime plans  
- Z-order deep control when practical  
- Moments layout restore quality (with Window ops)  
