# Engineering Milestone Report
## P11 Application Provider

| Field | Value |
| --- | --- |
| **Execution program** | P11 Application Provider |
| **Date** | 2026-08-07 |
| **Prior** | P10 Foundation complete (`23492a8`); Owner directed Application next |
| **Commit** | `330a26b` |
| **Handoff** | `AWAITING_PROJECT_OWNER_APPLICATION_PROVIDER_REVIEW` |
| **Index** | `docs/capability-runtime/00_INDEX.md` |

---

## Mission

Implement Application Provider on the frozen Capability Runtime using an **operations** model (Launch, Enumerate, Focus, Close, Minimize, Restore, Find). Maximize Desktop Operator usefulness without redesigning presentation.

---

## Architectural refinement (Owner)

Providers own operations, not individual features. Documented in `PROVIDER_REGISTRY.md` and `APPLICATION_PROVIDER.md`.

---

## Deliverables

| Artifact | Path |
| --- | --- |
| Application Provider | `packages/kernel/src/capability_runtime/application_provider.rs` |
| Operations (Win32) | `minimize_window` / `restore_window` / `close_window` on `WindowMutator` |
| Command + IPC | `ExecuteApplicationOperation` / `execute_application_operation` |
| Intent kinds | `appOpen`, `appLaunch`, `appFocus`, `appClose`, … |
| Provider doc | `docs/capability-runtime/APPLICATION_PROVIDER.md` |
| Updated roadmap | `docs/capability-runtime/FIVE_PROGRAM_ROADMAP.md` |

---

## Adoption

**ADAPT** Workspace-owned Application Control + existing Win32 ports. No AutoHotkey/PowerToys identity.

---

## Explicit non-goals

- No UI / Operator / Conversation redesign  
- No Window / Browser / Notification providers  
- No force-kill (graceful WM_CLOSE only)  
- P10 Clipboard left intact  

---

## Stop

Wait for Product Owner review before P12.  
