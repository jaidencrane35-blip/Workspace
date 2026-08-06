# Clipboard Provider — Reference Implementation (P10)

| Field | Value |
| --- | --- |
| **Name** | ClipboardProvider |
| **Domain** | `clipboard` |
| **Purpose** | Read/write clipboard text under Workspace permission and audit |
| **Adoption** | **WRAP** `arboard` behind `ClipboardPort` |
| **Why this reference** | P9 Phase 1 #1 — highest value, limited blast radius, clear Conversation entry |

---

## Capability set

| Capability | Command | Governance |
| --- | --- | --- |
| `clipboard.read` | `ReadClipboard` / IPC `read_clipboard` | Governed read |
| `clipboard.write` | `WriteClipboard` / IPC `write_clipboard` | Mutation + audit metadata |

Local user standard set includes both (P10).

---

## Dependencies

- `workspace-windows-integration::ClipboardPort`
- Production: `ArboardClipboard` (WRAP)
- Kernel tests: `MemoryClipboard`

---

## Conversation examples

- “What’s on my clipboard?”
- “Copy to clipboard: report-path.txt”

---

## Arguments / returns

| Op | Args | Returns |
| --- | --- | --- |
| read | — | `{ format, bytes, preview, text }` |
| write | `text` | `{ format, bytes, preview, message }` |

Failure modes: clipboard locked / OS error; write without text; >1MB write; permission denied.

---

## Audit behaviour

- Write: format + byte length + short preview (not full payload as secret store).
- Read: governed command audit (capability + command name).

---

## Ownership

| Layer | Location |
| --- | --- |
| Rust provider | `packages/kernel/src/capability_runtime/clipboard_provider.rs` |
| Port | `packages/windows-integration/src/clipboard.rs` |
| Commands | `packages/kernel/src/commands/clipboard.rs` |
| IPC | `app/src-tauri/src/commands/clipboard.rs` |
| Intent | `intentBridge.ts` kinds `clipboardRead` / `clipboardWrite` |
| Conversation | `OperatorRoot` invoke + reply only |

---

## Tests

- Memory clipboard roundtrip (windows-integration)
- Runtime router write/read (kernel)
- Pipeline permission denial without capability
- Intent bridge routing

---

## Future extensions (not P10)

- Image / rich formats  
- Clipboard history (explicit opt-in)  
- Restore-previous-on-write buffer  
