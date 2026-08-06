# Open Source Adoption Matrix — Capability Runtime

| Field | Value |
| --- | --- |
| **Program** | P9 |
| **Classes** | ADOPT · ADAPT · WRAP · STUDY · REJECT |
| **ADOPT meaning** | Take a crate/component as dependency behind a Workspace interface (not product identity) |
| **Rule** | Prefer WRAP/ADAPT; full ADOPT of platforms = 0 |

Kiro / Kiro Crew appear where relevant as **references**, never as the sole answer.

---

## Matrix by domain

| Domain | Strongest candidates | Class | License notes | Windows / Rust / Tauri | Rationale |
| --- | --- | --- | --- | --- | --- |
| **Application Control** | `windows` crate (Win32); ShellExecuteW; existing `workspace-windows-integration` | **ADAPT** (own) + **WRAP** Win32 | MIT/Apache (`windows`) | Excellent / native / IPC | Workspace already owns Win32 authority (DEC-008). Do not adopt AutoHotkey/PowerToys as identity. |
| **Window Management** | `windows` Win32_UI_WindowsAndMessaging; existing observation/placement code | **ADAPT** | MIT/Apache | Excellent | Keep single OS authority path. FancyZones-class UX = STUDY ideas only. |
| **Clipboard** | `arboard` (Rust); Tauri clipboard plugin | **WRAP** `arboard` | Apache-2.0/MIT | Good | Commodity I/O behind ClipboardPort. Permission on read of sensitive content. |
| **File Operations** | `std::fs` + `trash` / `opener`; `notify` for watch | **WRAP** trash/notify; **ADAPT** scoped FS service | MIT/Apache | Good | Never ambient file watch as product. Scope paths via permission. |
| **Desktop Observation** | Existing Workspace observation pipeline; UI Automation via `windows` / `uiautomation` crates | **ADAPT** existing; **STUDY** uiautomation helpers | Varies | Good | Law XII: off by default; consented Moments. Reject continuous spy frameworks. |
| **Screenshots** | `xcap` / `screenshots` Rust crates; Windows Graphics Capture | **WRAP** capture crate | MIT/Apache typical | Good | Capture behind ScreenshotPort; user-visible consent for region/display. |
| **OCR** | Windows.Media.Ocr; `tesseract` / `leptess` | **STUDY** → **WRAP** | Apache (Tesseract) | Medium | Prefer OS OCR when quality OK; Tesseract as fallback WRAP. |
| **Voice Input** | `whisper.cpp` / Candle Whisper; Windows Speech SDK | **STUDY** → **WRAP** | MIT (whisper.cpp) | Medium | Speech → IntentEnvelope only. No voice-first product shell. |
| **Automation** | `enigo` / `rdev` for input; Workspace CommandPipeline | **WRAP** input libs; **REJECT** AutoHotkey/Pullover suites as product | MIT | Good | Synthetic input is Critical trust. Gateway + audit mandatory. Kiro `computer_use` → STUDY then likely REJECT for PP. |
| **Memory** | Existing SQLite Moments / memory tables; FTS5 | **ADAPT** existing; **ADAPT** Kiro Crew *principle* (inspectable local memory) | — | Excellent | No ADOPT of Crew Python memory stack. |
| **Search** | SQLite FTS5; optional `tantivy` later | **ADAPT** FTS5 first; **STUDY** tantivy | MIT | Good | Local-only; no cloud search identity. |
| **Notifications** | `tauri-plugin-notification`; WinRT toasts | **WRAP** | MIT | Excellent | Quiet; never spam. |
| **Terminal** | `portable-pty` + ConPTY; `shared_child` | **WRAP** PTY; **REJECT** embedding full IDE terminals as UI | MIT/Apache | Good | Governed commands only; no free-form agent shell as default. |
| **Browser** | `webbrowser` crate (open URL); CDP via `chromiumoxide` later | **WRAP** open; **STUDY** CDP | MIT/Apache | Good | Opening URLs first; deep browser control later under permission. |
| **Workflow** | Workspace CommandPipeline + plans | **ADAPT** own; **REJECT** n8n/Zapier-as-product | — | — | Multi-domain plans stay Workspace-owned. |
| **Agent gateway (cross-cutting)** | Kiro Crew | **REJECT** dependency; **ADAPT** principles (tool gate outside model, inspectable memory) | Apache-2.0 (Crew) | Python-heavy | See `docs/kiro-adoption-matrix.md`. Gateway ideas → AgentToolGate design, not Crew embed. |
| **MCP tools** | MCP servers ecosystem | **STUDY** → optional **WRAP** behind tool interface | Varies | Varies | Only if Expand needs; each tool still hits Gateway. |

---

## Summary counts

| Class | Domains / items |
| --- | --- |
| ADOPT (platform-as-product) | **0** |
| WRAP (crate behind interface) | Clipboard, Files helpers, Screenshots, Notifications, PTY, URL open, input libs |
| ADAPT (Workspace-owned + principles) | App control, Window mgmt, Observation, Memory, Search, Workflow, Kiro principles |
| STUDY | OCR engines, Voice, CDP browser, MCP, tantivy |
| REJECT | AutoHotkey/PowerToys identity, n8n-as-product, Kiro Crew as dependency, ambient spy stacks |

---

## Strongest implementation per domain (headline)

| Domain | Strongest path |
| --- | --- |
| App / Window | Own Win32 via `workspace-windows-integration` + `windows` crate |
| Clipboard | WRAP `arboard` |
| Files | std + WRAP `trash`/`notify` with scopes |
| Observation | Extend existing pipeline |
| Screenshots | WRAP `xcap` (or equivalent) |
| OCR | STUDY Windows OCR → WRAP |
| Voice | STUDY Whisper-class → WRAP utterance only |
| Automation | WRAP `enigo`/`rdev` under Critical policy |
| Memory / Search | ADAPT SQLite/FTS already in tree |
| Notifications | WRAP Tauri notification plugin |
| Terminal | WRAP ConPTY/`portable-pty` |
| Browser | WRAP URL open; STUDY CDP |
| Workflow | Own planner over contracts |
