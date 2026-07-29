# DAF-1d Completion Report

| Field | Value |
|-------|-------|
| **Batch** | DAF-1d — Desktop Arrangement Capture & Governed Restore |
| **Date** | 2026-07-29 |
| **Branch** | `cursor/daf-1d-arrangement-restore-34a5` |
| **Status** | Complete |

---

## 1. Goal

Connect observation + persistence + WindowController so users can **explicitly** save arrangements and **request** restoration through PermissionGateway. User remains the authority.

---

## 2. Architecture decisions

| Decision | Choice |
|----------|--------|
| Pathway | Observe → DesktopArrangement → Restore Request → CommandPipeline → PermissionGateway → WindowController → OS |
| Geometry | Optional `x,y,width,height` on entries (migration `080`) captured from observation; no invented layout |
| Matching | Deterministic: `stable_window_id` then `hwnd`; use **current** hwnd for apply |
| Missing windows | Diagnostics + restore gaps; never launch replacements |
| Mid-apply failure | Halt further applies; record failed/skipped; no automatic repair |
| Capabilities | `desktop.write` (capture), `desktop.restore` (apply); local user standard includes both; AI empty by default |
| Assistant | No restore execution path (`attempt_assistant_restore` sealed); AI requires approval even with caps |
| Canvas Layout | Untouched |

---

## 3. Files changed (summary)

### Docs
- `docs/03-Engineering/DAF-1D-DESKTOP-ARRANGEMENT-RESTORE.md`
- `docs/03-Engineering/DAF-1D-ALIGNMENT-CHECK.md`
- `docs/03-Engineering/DAF-1D-COMPLETION-REPORT.md`
- Indexes: `docs/README.md`, `DAF-ARCHITECTURE-AUDIT.md`, `ENGINEERING-GOVERNANCE.md`, `IPC-SURFACE.md`, `DAF-1C-DESKTOP-ARRANGEMENT.md`

### Domain / DB
- `packages/domain/src/desktop_arrangement/mod.rs` — bounds, plan/match/capture helpers, restore DTOs
- `packages/domain/src/capability/mod.rs` — `desktop.write` / `desktop.restore`
- `packages/domain/src/intent/action.rs` — action intents for capture/restore/get/list
- `packages/database/migrations/080_desktop_arrangement_bounds.sql`
- `packages/database/src/repositories/desktop_arrangement.rs`

### Kernel / IPC
- `packages/kernel/src/services/desktop_arrangement.rs`
- `packages/kernel/src/commands/desktop_arrangement.rs` (+ tests)
- `packages/kernel/src/commands/handler.rs`
- `packages/kernel/src/error.rs` + `app/src/types/ipc-errors.ts`
- `app/src-tauri/src/commands/desktop_arrangement.rs` + registration
- `scripts/architecture-governance-lib.mjs` + generated map

---

## 4. Ownership verification

| Concern | Owner | OK |
|---------|-------|----|
| Capture / restore orchestration | Kernel `DesktopArrangementService` | Yes |
| Permission | CommandPipeline + Gateway (`desktop.write` / `desktop.restore`) | Yes |
| OS mutation | WindowController only | Yes |
| Membership + bounds persistence | Domain + database repo | Yes |
| Assistant → move windows | Forbidden | Yes |
| Canvas Layout | Untouched | Yes |

---

## 5. Validation results

| Check | Result |
|-------|--------|
| `cargo test -p workspace-domain --lib desktop_arrangement` | **9 passed** |
| `cargo test -p workspace-database --lib desktop_arrangement` | **2 passed** |
| `cargo test -p workspace-kernel --lib desktop_arrangement` | **10 passed** |
| `pnpm verify:ipc-contract` | Pass (204 commands) |
| `pnpm verify:architecture-governance` | Pass |
| Human visual review | Not required |
| Videos / screenshots | None |

---

## 6. Maintainability impact

- Extends existing observation, repository, WindowController, and launch-style command patterns
- No parallel control layer; no new abstraction framework
- Clear capability split: read / write / restore

---

## 7. Remaining technical debt

- UI Save/Restore chrome not in this batch (IPC ready)
- No automatic reverse of already-applied bounds on mid-restore failure (by design; diagnostics only)
- Full workspace manager / auto-layout / modes remain future DAF batches
