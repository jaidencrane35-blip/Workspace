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

## Law: Providers own operations

A provider is not a bag of unrelated features. It owns a coherent operation set  
(e.g. Application: Launch, Enumerate, Focus, Close, Minimize, Restore, Find).

---

## Registered providers

| Domain | Provider | Adoption | Operations | Status |
| --- | --- | --- | --- | --- |
| clipboard | `ClipboardProvider` | WRAP `arboard` | read, write | P10 reference |
| application | `ApplicationProvider` | ADAPT Win32 ports | launch, enumerate, find, focus, close, minimize, restore | P11 |
| window | `WindowProvider` | ADAPT Win32 ports | enumerate, find, active, bounds, monitors, focus, minimize, restore, maximize, move, resize, center, snap | P12 |
| notifications | `NotificationProvider` | WRAP WinRT toast | status, show, dismiss | **P13** |

Placeholders: Browser, Screenshot, File, Terminal, Memory, Voice, Automation.

**Independence:** Providers never call each other (Kernel Operator composes).

---

## Registration law

- Duplicate domain registration fails.
- Unregistered domain invoke fails closed.
- Providers are Workspace-owned adapters; commodity crates stay behind ports.  
