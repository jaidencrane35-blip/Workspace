# Screenshot Provider Research (P15)

| Field | Value |
| --- | --- |
| **Program** | P15 Screenshot Provider |
| **Status** | **Implemented — WRAP decision locked** |
| **Primary recommendation** | **WRAP** `xcap` (Apache-2.0) behind `ScreenshotPort` |
| **Windows-native alternative** | **STUDY → WRAP** `windows-capture` if WGC/DXGI tuning is required |
| **Clipboard image** | **WRAP** `arboard` image API inside ScreenshotPort (not ClipboardProvider) |

---

## Problem

Workspace needs governed, consented capture of:

- a monitor / display
- an application window
- PNG save + optional image clipboard copy

Conversation must sound like desktop operation (“Take a screenshot of Cursor.”), never expose Provider / DXGI / WGC terminology, and never invent success.

Existing `DesktopCapturer` in `packages/windows-integration` captures **observation metadata** (window/monitor topology). It is **not** a bitmap Screenshot Provider. P15 does not overload that trait.

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
| **`xcap`** | Apache-2.0 | Cross-platform; Windows via WGC | Active (`0.9.x`) | **WRAP (primary)** |
| **`windows-capture`** | MIT | WGC + DXGI Desktop Duplication | Active | **STUDY → WRAP** if `xcap` gaps appear |
| **`screenshots`** (legacy / forks) | Varies | Older helpers | Mixed / superseded | **REJECT** as primary |
| Raw `windows` crate bindings only | MIT/Apache | Full ADAPT | Highest ownership cost | **ADAPT fallback** if WRAP crates regress |
| **`arboard`** (image) | Apache-2.0 / MIT | Image clipboard | Already used for text clipboard | **WRAP** inside ScreenshotPort |

---

## Decision matrix (constitutional classes)

| Option | Class | Rationale |
| --- | --- | --- |
| `xcap` behind `ScreenshotPort` | **WRAP** | Commodity bitmap capture; Workspace owns consent, audit, paths, Conversation truth |
| `windows-capture` | **STUDY** → possible **WRAP** | Prefer if single-frame DXGI path or picker UX is needed later |
| Own WGC/DXGI via `windows` crate | **ADAPT** (fallback) | Only if WRAP crates cannot meet Levels 1–2 |
| Continuous / ambient capture | **REJECT** | Violates Product Gravity; Screenshots are explicit user intent |
| Cloud upload of captures | **REJECT** | Local-first; images stay in app data |
| OCR / annotate in P15 | **REJECT** (later programs) | Out of Screenshot L1–2 ownership |
| Call ClipboardProvider for images | **REJECT** | Providers never call providers; image clipboard stays in port |

**Headline decision:**  
**WRAP `xcap`** for monitor + window stills. Image clipboard via **arboard inside ScreenshotPort**. DXGI/`windows-capture` remains STUDY — not dual-shipped.

---

## Security · privacy · performance

| Concern | Requirement |
| --- | --- |
| **Consent** | Capture only on explicit Conversation / Operator intent |
| **Permissions** | `screenshot.read`, `screenshot.capture` |
| **Audit** | Target kind, dimensions, path id — not full pixel payload |
| **Storage** | `%APPDATA%\com.workspace.app\captures\` |
| **Protected content** | Truthful failure when WGC blocks DRM / secure desktop |
| **Performance** | Single-frame stills for L1–2; no recording loop |
| **UI leakage** | Never mention DXGI, WGC, `xcap`, Provider, Runtime in Conversation |

---

## Levels delivered (P15)

| Level | Operations | Effect |
| --- | --- | --- |
| 1 | `status` | Capture availability / monitor count |
| 2 | `capture_desktop` / `capture_monitor` / `capture_window` | Still → PNG |
| 2 | `save_png` / `copy_clipboard` | Explicit save / image clipboard |
| Operator | `capture_and_copy` | Capture then copy |

---

## Explicit non-goals (P15)

- Video recording / streaming  
- Always-on / scheduled capture  
- OCR / annotation / AI vision  
- Image search  
- Overloading `DesktopCapturer` observation trait  
- Provider-to-provider calls  
