# Screenshot Provider — Architecture Proposal (P15 Prep)

| Field | Value |
| --- | --- |
| **Status** | Proposal only — **no implementation** |
| **Depends on** | P14 Owner acceptance |
| **Pipeline** | Conversation → Intent → Kernel Operator → Capability Runtime → Screenshot Provider → OS |

---

## Placement

```
Conversation
    ↓ Intent (screenshotStatus | screenshotCapture…)
execute_capability_intent
    ↓
Kernel Operator (plan / compose; Window composition for “this window”)
    ↓
Capability Runtime → Router → Registry
    ↓
ScreenshotProvider
    ↓
ScreenshotPort (windows-integration)
    ↓
WRAP xcap (WGC) → PNG/bytes in local app data
    ↓
Operator compose → desktop-language reply
```

Providers never call each other. Window title → HWND resolution is Operator → Window Provider, then Screenshot capture by handle/title as contract fields allow.

---

## Owned surfaces

| Layer | Proposed location |
| --- | --- |
| Port | `packages/windows-integration/src/screenshot.rs` — `ScreenshotPort` |
| Provider | `packages/kernel/src/capability_runtime/screenshot_provider.rs` |
| Commands | `ScreenshotStatus` / `CaptureScreenshot` |
| Domain id | `screenshots` (align with catalogue; alias `screenshot` in Operator parse) |
| Permissions | `screenshot.read`, `screenshot.capture` |
| Intent kinds | `screenshotStatus`, `screenshotCaptureMonitor`, `screenshotCaptureWindow` |

---

## Separation from DesktopCapturer

| Trait | Purpose |
| --- | --- |
| `DesktopCapturer` | Observation metadata (titles, bounds, monitors) — **existing** |
| `ScreenshotPort` | Bitmap stills — **new for P15** |

Do not merge. Observation must stay cheap and non-pixel; Screenshots are consent-heavy pixel capture.

---

## Storage model (draft)

- Write PNG under app-data `captures/` with opaque id  
- Return path + width/height + target label to Operator  
- Conversation reply: “Captured Cursor (1920×1080).” — no file-system lecture unless asked  
- Rollback: delete file on failed post-steps (future)

---

## Composition (Operator only)

| Utterance pattern | Plan |
| --- | --- |
| “Screenshot this window.” | Window `active` → Screenshot `capture_window` |
| “Screenshot Cursor.” | Window `find` → Screenshot `capture_window` |
| “Screenshot the right monitor.” | Window/monitors index → Screenshot `capture_monitor` |
| “Screenshot and notify me.” | Screenshot then Notifications `show` (future composition) |

---

## Failure truthfulness

| Condition | Reply intent |
| --- | --- |
| Capture API unavailable | “I can’t take screenshots on this PC right now.” |
| Window not found | “I couldn’t find that window — which one?” |
| Protected / blocked content | “Windows blocked that capture.” |
| Missing target | “What should I capture — a window or a monitor?” |

Never fabricate a successful capture.
