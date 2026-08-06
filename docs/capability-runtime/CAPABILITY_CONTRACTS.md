# Capability Contracts — Conversation Entry (P9)

Conversation-entry contracts for Track B domains.  
Cross-capability message envelopes remain governed by `architecture/10_Capability_Contracts.md`.

**Common fields (all domains)**

| Field | Rule |
| --- | --- |
| IPC boundary | Tauri command → Kernel CommandPipeline → domain port |
| Rust ownership | `workspace-kernel` + `workspace-windows-integration` (effects) |
| TypeScript ownership | Intent bridge / thin invoke helpers — no effect logic |
| UI ownership | Conversation reply + optional satellite; **no new primary UI** |
| Audit | Every consequential effect emits audit event |
| Permission | Gateway validate-for-use at point of effect |

---

## app_control — Application Control

| | |
| --- | --- |
| **Purpose** | Launch, focus, or quit applications by user intent |
| **Conversation examples** | “Open Cursor.” / “Switch to Chrome.” / “Close Spotify.” / “List apps.” |
| **Permissions** | `application.launch`, `application.focus`, `application.close`, `application.minimize`, `application.restore`, `application.read` |
| **Arguments** | `query` \| `path` \| `hwnd?` |
| **Expected results** | `{ status, target, items? }` — see `APPLICATION_PROVIDER.md` |
| **Failure modes** | Not installed; not found; focus refused; access denied |
| **Rollback** | Close is graceful WM_CLOSE only (no force-kill in P11) |
| **Audit** | operation + status + target — no keystroke payload |
| **P11 status** | **Application Provider shipped** — operations model |
| **Adoption** | ADAPT Win32 ports (`ProcessLauncher`, `WindowEnumerator`, `WindowMutator`) |
| **Rust** | `ApplicationProvider` + `ExecuteApplicationOperation` |
| **TypeScript** | Intent kinds `appOpen` / `appLaunch` / `appFocus` / … → CapabilityIntent |
| **Conversation IPC** | `execute_capability_intent` (Kernel Operator) |
| **Diagnostic IPC** | `execute_application_operation` (not Conversation) |
| **Future** | Ambiguity satellite; ApprovalRequired for risky close; Intelligence ranking |

---

## window_mgmt — Window Management

| | |
| --- | --- |
| **Purpose** | Discover, place, and organize HWND-level windows |
| **Examples** | “List windows.” / “Snap Chrome left.” / “Move Notepad to monitor 1.” / “Maximize Cursor.” |
| **Permissions** | `window.read`, `window.focus`, `window.place`, `window.state` |
| **Arguments** | `query` \| `hwnd` \| `pid` · `x/y/width/height` · `monitorIndex` · `snap` |
| **Results** | `{ status, target, items?, monitors? }` — see `WINDOW_PROVIDER.md` |
| **Failures** | not_found; focus refused; invalid size; no monitor |
| **Rollback** | Prior bounds snapshot — future |
| **Audit** | operation + status + target + geometry summary |
| **P12 status** | **Window Provider engineering complete** (Levels 1–2) |
| **P12.5 status** | **Window Conversation Product Proof shipped** — P12 series permanently closed |
| **Product Proof Rule** | `docs/capability-runtime/PRODUCT_PROOF_RULE.md` (permanent) |
| **Adoption** | ADAPT Win32 ports; FancyZones = STUDY only |
| **Rust** | `WindowProvider` + `ExecuteWindowOperation` |
| **TypeScript** | `winEnumerate` / `winSnap` / … → CapabilityIntent |
| **Conversation IPC** | `execute_capability_intent` (Kernel Operator) |
| **Diagnostic IPC** | `execute_window_operation` (not Conversation) |
| **Future** | Level 3 compose layouts; Level 4 Intelligence |

---

## clipboard — Clipboard

| | |
| --- | --- |
| **Purpose** | Read/write clipboard under permission |
| **Examples** | “What’s on my clipboard?” / “Copy to clipboard: …” |
| **Permissions** | `clipboard.read` (sensitive), `clipboard.write` |
| **Arguments** | `format?` · `text?` |
| **Results** | Payload summary + truncated preview |
| **Failures** | Locked clipboard; unsupported format; permission denied |
| **Rollback** | N/A for read; write may restore previous if buffered |
| **Audit** | format + length — not full secrets by default |
| **Future** | Image clipboard; history (explicit opt-in) |
| **P10 status** | **Reference provider shipped** — see `CLIPBOARD_PROVIDER.md` |
| **Adoption** | WRAP `arboard` behind `ClipboardPort` |
| **Rust** | `ClipboardProvider` + `ReadClipboard` / `WriteClipboard` |
| **TypeScript** | Intent kinds `clipboardRead` / `clipboardWrite` → CapabilityIntent |
| **Conversation IPC** | `execute_capability_intent` (Kernel Operator) |
| **Diagnostic IPC** | `read_clipboard`, `write_clipboard` (not Conversation) |
| **Pipeline** | Intent → Kernel Operator → Runtime → Router → Registry → ClipboardProvider → reply |

---

## file_ops — File Operations

| | |
| --- | --- |
| **Purpose** | Scoped file read/write/move/trash |
| **Examples** | “Trash the downloads zip named report.” / “Open this folder.” |
| **Permissions** | `fs.read`, `fs.write`, `fs.trash` + path scope |
| **Arguments** | `path` · `op` · `destination?` |
| **Results** | `{ op, path, ok }` |
| **Failures** | Outside scope; in use; not found |
| **Rollback** | Trash preferred over delete; restore from trash when possible |
| **Audit** | op + path |
| **Future** | Batch ops with plan satellite |

---

## desktop_obs — Desktop Observation

| | |
| --- | --- |
| **Purpose** | Consented snapshot of windows/processes for Moments / awareness |
| **Examples** | “Save this moment.” / “What apps are open?” (if allowed) |
| **Permissions** | `observe.snapshot` — **off by default** |
| **Arguments** | `scope` (monitors/apps) · `purpose` |
| **Results** | Structured snapshot id |
| **Failures** | Permission denied; capture timeout |
| **Rollback** | Delete snapshot on user request |
| **Audit** | scope + purpose + snapshot id |
| **Future** | Delta observation (still consented) |

---

## screenshots — Screenshots

| | |
| --- | --- |
| **Purpose** | Capture display/window image for user intent |
| **Examples** | “Screenshot this window.” / “Capture the right monitor.” |
| **Permissions** | `capture.display`, `capture.window` |
| **Arguments** | `target` · `region?` |
| **Results** | Image handle / path in app data |
| **Failures** | HDR/capture API unavailable; permission |
| **Rollback** | Delete capture file |
| **Audit** | target type + dimensions |
| **Future** | Annotate → OCR pipeline |

---

## ocr — OCR

| | |
| --- | --- |
| **Purpose** | Extract text from image/region |
| **Examples** | “Read the text in that screenshot.” |
| **Permissions** | `ocr.image` (implies capture access) |
| **Arguments** | `image_ref` · `lang?` |
| **Results** | Text + confidence |
| **Failures** | Low confidence; engine missing |
| **Rollback** | N/A |
| **Audit** | image_ref + char_count |
| **Future** | Structured table extract |

---

## voice_input — Voice Input

| | |
| --- | --- |
| **Purpose** | Speech → utterance into Conversation (IntentEnvelope) |
| **Examples** | Push-to-talk: “Save this moment as Northwind.” |
| **Permissions** | `mic.capture` session-scoped |
| **Arguments** | `audio_stream` · `locale?` |
| **Results** | `{ utterance, confidence }` → same intent bridge |
| **Failures** | Mic denied; low confidence |
| **Rollback** | Discard utterance |
| **Audit** | session id — not raw audio by default |
| **Future** | Hotword (explicit opt-in only) |

---

## automation — Automation

| | |
| --- | --- |
| **Purpose** | Synthetic input / sequenced UI actions |
| **Examples** | “Click Send in Outlook.” (high friction) |
| **Permissions** | `input.synthesize` — **Critical**, always ApprovalRequired |
| **Arguments** | `steps[]` · `timeout` |
| **Results** | Per-step status |
| **Failures** | Target missing; focus lost |
| **Rollback** | Stop remaining steps; no silent retry |
| **Audit** | Full step plan + outcomes |
| **Future** | Record/replay with edit satellite |

---

## memory — Memory

| | |
| --- | --- |
| **Purpose** | Explicit remember / recall / forget |
| **Examples** | “Remember my design folder.” / “What did I ask you to remember?” |
| **Permissions** | `memory.write`, `memory.read`, `memory.delete` |
| **Arguments** | `key` · `value` · `scope` |
| **Results** | Stored item / list |
| **Failures** | Not found; quota |
| **Rollback** | Soft-delete + undo window |
| **Audit** | key + op |
| **Future** | Intelligence ranking (Layer 4) |

---

## search — Search

| | |
| --- | --- |
| **Purpose** | Local search over Moments / memory / scoped files |
| **Examples** | “Find the Northwind moment.” |
| **Permissions** | `search.local` |
| **Arguments** | `query` · `corpus` |
| **Results** | Ranked hits |
| **Failures** | Empty index |
| **Rollback** | N/A |
| **Audit** | query hash + hit count |
| **Future** | Semantic index (Layer 4) |

---

## notifications — Notifications

| | |
| --- | --- |
| **Purpose** | Sparse OS attention signals |
| **Examples** | System: “Restore finished.” |
| **Permissions** | `notify.show` (user preference) |
| **Arguments** | `title` · `body` · `action?` |
| **Results** | `{ shown: bool }` |
| **Failures** | OS focus assist |
| **Rollback** | N/A |
| **Audit** | template id |
| **Future** | Actionable toasts → conversation |

---

## terminal — Terminal

| | |
| --- | --- |
| **Purpose** | Governed command execution |
| **Examples** | “Run the project’s typecheck.” |
| **Permissions** | `terminal.exec` — ApprovalRequired + allowlist |
| **Arguments** | `command` · `cwd` · `timeout` |
| **Results** | exit code + truncated output |
| **Failures** | Denied command; timeout |
| **Rollback** | Kill process tree on cancel |
| **Audit** | command template + cwd + exit |
| **Future** | Named recipes only (no free agent shell) |

---

## browser — Browser

| | |
| --- | --- |
| **Purpose** | Open URLs; later assisted browser control |
| **Examples** | “Open the PR link.” |
| **Permissions** | `browser.open`; future `browser.cdp` Critical |
| **Arguments** | `url` |
| **Results** | `{ opened: bool }` |
| **Failures** | Invalid URL; no handler |
| **Rollback** | N/A |
| **Audit** | host only (not full URL if sensitive) |
| **Future** | CDP WRAP under STUDY |

---

## workflow — Workflow

| | |
| --- | --- |
| **Purpose** | Multi-step plans composing other domains |
| **Examples** | “Save moment, then open yesterday’s layout.” |
| **Permissions** | Union of step permissions; plan ApprovalRequired |
| **Arguments** | `steps[]` referencing domain contracts |
| **Results** | Plan satellite + per-step results |
| **Failures** | Step failure → stop or compensated path |
| **Rollback** | Compensating steps where defined |
| **Audit** | plan id + step ids |
| **Future** | Intelligence-suggested plans (still approved) |
