# Screenshot Provider Research (P15 Preparation)

| Field | Value |
| --- | --- |
| **Program** | P15 Screenshot Provider — **preparation only** |
| **Status** | Research complete — **no implementation** |
| **Blocked on** | P14 Product Owner Product Proof acceptance |
| **Primary recommendation** | **WRAP** `xcap` (Apache-2.0) behind `ScreenshotPort` |
| **Windows-native alternative** | **STUDY → WRAP** `windows-capture` if WGC/DXGI tuning is required |

---

## Problem

Workspace needs governed, consented capture of:

- a monitor / display
- an application window
- (later) a region

Conversation must sound like desktop operation (“Take a screenshot of Cursor.”), never expose Provider / DXGI / WGC terminology, and never invent success.

Existing `DesktopCapturer` in `packages/windows-integration` captures **observation metadata** (window/monitor topology). It is **not** a bitmap Screenshot Provider. P15 must not overload that trait.

---

## Windows capture APIs

| API | Role | Pros | Cons | Verdict |
| --- | --- | --- | --- | --- |
| **Windows Graphics Capture (WGC)** | Modern WinRT capture for monitors/windows | Secure, supports window capture, active maintenance path | Requires Win10 1803+; some protected content blocked | **ADOPT as preferred OS path** (via WRAP crate) |
| **DXGI Desktop Duplication** | GPU desktop frames | High performance; good for full-screen / streaming | Monitor-oriented; weaker as primary window API; session/GPU quirks | **STUDY** fallback / advanced path |
| **GDI `BitBlt` / PrintWindow** | Legacy bitmap copy | Simple | Incomplete for modern DWM/UWP; quality/reliability issues | **REJECT** as primary |
| **WinRT `GraphicsCaptureItem` direct** | Same family as WGC | Full control | Heavy ownership cost vs WRAP | **ADAPT only if WRAP fails** |
| **Magnification API** | Accessibility overlay | Niche | Wrong product fit | **REJECT** |

---

## Rust ecosystem candidates

| Crate | License | Capture model | Maintenance (observed 2026) | Verdict |
| --- | --- | --- | --- | --- |
| **`xcap`** | Apache-2.0 | Cross-platform; Windows via WGC (`wgc` feature) | Active (`0.9.x`, May 2026) | **WRAP (primary)** |
| **`windows-capture`** | MIT | WGC + DXGI Desktop Duplication; strong Windows focus | Active (`2.0.0`, Apr 2026) | **STUDY → WRAP** if `xcap` gaps appear |
| **`screenshots`** (legacy name / forks) | Varies | Older screenshot helpers | Mixed / superseded by `xcap` lineage | **REJECT** as primary |
| Raw `windows` crate bindings only | MIT/Apache | Full ADAPT | Highest ownership cost | **ADAPT fallback** if WRAP crates regress |

---

## Decision matrix (constitutional classes)

| Option | Class | Rationale |
| --- | --- | --- |
| `xcap` behind `ScreenshotPort` | **WRAP** | Commodity bitmap capture; Workspace owns consent, audit, paths, Conversation truth |
| `windows-capture` | **STUDY** → possible **WRAP** | Prefer if single-frame DXGI path or picker UX is needed later |
| Own WGC/DXGI via `windows` crate | **ADAPT** (fallback) | Only if WRAP crates cannot meet Levels 1–2 |
| Continuous / ambient capture | **REJECT** | Violates Product Gravity + Law XII; Screenshots are explicit user intent |
| Cloud upload of captures | **REJECT** | Local-first; images stay in app data |
| OCR / annotate in P15 | **STUDY** (later programs) | Out of Screenshot L1–2 ownership |

**Headline decision for P15 implementation (when unblocked):**  
**WRAP `xcap`** for monitor + window stills. Keep DXGI/`windows-capture` as documented STUDY alternative — not dual-shipped at start.

---

## Security · privacy · performance

| Concern | Requirement |
| --- | --- |
| **Consent** | Capture only on explicit Conversation / Operator intent; no ambient screenshots |
| **Permissions** | `screenshot.read` (capability/status), `screenshot.capture` (mutation) |
| **Audit** | Target kind (monitor/window), dimensions, path id — not full pixel payload in audit logs |
| **Storage** | Local app-data directory; optional TTL cleanup later |
| **Protected content** | Truthful failure when WGC blocks DRM / secure desktop |
| **Performance** | Single-frame stills for L1–2; no recording loop in P15 |
| **UI leakage** | Never mention DXGI, WGC, `xcap`, Provider, Runtime in Conversation |

---

## Levels proposed for P15 (implementation later)

| Level | Operations | Effect |
| --- | --- | --- |
| 1 | `status` | Capture availability / supported targets summary |
| 2 | `capture_monitor` | Still of primary or indexed monitor |
| 2 | `capture_window` | Still of named / active window (compose with Window for resolution) |
| Later | `capture_region`, recording, OCR handoff | Not P15 |

---

## Explicit non-goals (P15)

- Video recording  
- Always-on / scheduled capture  
- OCR (separate domain)  
- Sharing / clipboard auto-copy of image (may compose later via Clipboard)  
- Overloading `DesktopCapturer` observation trait  

---

## References (repository)

- `OPEN_SOURCE_ADOPTION_MATRIX.md` — Screenshots → WRAP `xcap`  
- `CAPABILITY_CONTRACTS.md` — `screenshots` stub  
- `ADOPTION_RISK_ASSESSMENT.md` — ambient capture Critical  
- Window observation: `packages/windows-integration/src/capture.rs` (metadata only)
