# Provider Registry
## Capability Runtime Foundation (P10)

| Field | Value |
| --- | --- |
| **Rust** | `packages/kernel/src/capability_runtime/registry.rs` |
| **Bootstrap** | `capability_runtime::runtime()` registers reference providers |

---

## Rule

**Every provider owns exactly one capability domain.**  
Future providers extend the registry. They never redesign the runtime.

---

## Provider contract (required fields)

| Field | Description |
| --- | --- |
| Name | Stable provider name (e.g. `ClipboardProvider`) |
| Purpose | One-sentence domain purpose |
| Capability set | Permission keys (`clipboard.read`, …) |
| Permissions | Gateway behaviour |
| Dependencies | Ports / WRAP crates |
| Conversation examples | Intent phrases |
| Arguments | Invoke inputs |
| Return values | Invoke outputs |
| Failure modes | Locked clipboard, missing provider, … |
| Audit behaviour | What is logged / redacted |
| Rust ownership | kernel provider + windows-integration port |
| TypeScript ownership | Intent kinds + thin IPC only |
| IPC ownership | Tauri commands → CommandHandler |
| Tests | Port unit + pipeline + intent bridge |
| Future extensions | Documented non-goals for this program |

---

## Registered providers (P10)

| Domain | Provider | Adoption | Status |
| --- | --- | --- | --- |
| clipboard | `ClipboardProvider` | WRAP `arboard` | **Reference implementation** |

Placeholders for later programs (not registered yet): Application, Window, Notifications, Browser, Screenshot, Terminal, Memory, Voice, Automation.

---

## Registration law

- Duplicate domain registration fails.
- Unregistered domain invoke fails closed.
- Providers are Workspace-owned adapters; commodity crates stay behind ports.  
