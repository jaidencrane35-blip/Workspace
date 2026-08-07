# Screenshot Provider
## Product Implementation Program P15 — Levels 1–2

| Field | Value |
| --- | --- |
| **Status** | **PERMANENTLY CLOSED — ACCEPTED — REPOSITORY TRUTH — DO NOT REOPEN** (bugfixes only) |
| **Domain** | `screenshots` |
| **Adoption** | WRAP `xcap` (+ arboard image clipboard) behind `ScreenshotPort` |
| **Research** | `research/SCREENSHOT_RESEARCH.md` |
| **Conversation IPC** | `execute_capability_intent` |
| **Permissions** | `screenshot.read`, `screenshot.capture` |
| **Independence** | Satisfies Capability Independence Rule |

---

## Operations

| Level | Operation | Effect |
| --- | --- | --- |
| 1 | `status` | Capture availability, monitor count, window/clipboard capability |
| 2 | `capture_desktop` | Capture primary / desktop still → PNG |
| 2 | `capture_window` | Capture named or active window → PNG |
| 2 | `capture_monitor` | Capture 1-based monitor index → PNG |
| 2 | `save_png` | Capture desktop and save PNG (explicit save phrasing) |
| 2 | `copy_clipboard` | Copy last / path PNG image to clipboard |
| 2 | `capture_and_copy` | Operator composition: capture then copy |

---

## Ownership

| Layer | Location |
| --- | --- |
| Port | `packages/windows-integration/src/screenshot.rs` |
| Provider | `packages/kernel/src/capability_runtime/screenshot_provider.rs` |
| Commands | `ScreenshotStatus` / `ExecuteScreenshotOperation` |
| Intent | `screenshotStatus` / `screenshotDesktop` / `screenshotWindow` / `screenshotMonitor` / `screenshotSave` / `screenshotCopy` / `screenshotCaptureAndCopy` |

---

## Storage

PNG files under `%APPDATA%\com.workspace.app\captures\`.

Image clipboard uses arboard **inside** `ScreenshotPort` (not ClipboardProvider) so providers never call providers.

---

## Out of scope

OCR · annotation · editing · AI image understanding · image search · recording · streaming · ambient capture
