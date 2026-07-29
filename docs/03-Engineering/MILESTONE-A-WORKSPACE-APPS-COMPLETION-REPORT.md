# Milestone A — Workspace Apps & Switcher Foundation — Completion Report

| Field | Value |
|-------|-------|
| **Status** | Complete (implementation batch) |
| **Date** | 2026-07-29 |
| **Branch** | `cursor/milestone-a-workspace-apps-switcher-34a5` |
| **Charter** | Product Alignment Audit — Milestone A |
| **Human visual review** | Required — see [MILESTONE-A-VISUAL-REVIEW-CHECKLIST.md](MILESTONE-A-VISUAL-REVIEW-CHECKLIST.md) |

**Not Batch 17. No new AI engines. No Assistant architecture expansion.**

---

## Architecture

Reused existing owners; UI owns presentation and navigation only.

```
User → Home / Workspaces / Applications / Layouts (chrome)
     → WorkspaceSwitcher → list_workspaces / create_workspace / settings active id
     → ApplicationsPanel → list_applications / create_application / launch_application
                         → get_workspace_state (observed active apps)
     → DesktopArrangementPanel (Workspaces + Layouts rails)
         → list / capture / restore_desktop_arrangement
         → PermissionGateway + WindowController (unchanged)
```

| Concern | Owner (unchanged) |
|---------|-------------------|
| Window control | `packages/windows-integration` WindowController |
| Permissions / launch | PermissionGateway + CommandPipeline |
| Arrangement restore gaps | Desktop arrangement domain + RestoreDiagnosticsView |
| Assistant reasoning | Frozen; secondary tab only |

---

## Changes

### Product chrome
- Primary tabs: **Home**, **Workspaces**, **Applications**, **Layouts**
- Secondary: **Assistant** (dashed/secondary styling)
- Quiet: **Work**, **Diagnostic** (retained, de-emphasised)

### IPC (thin queries over existing services)
- `list_workspaces` → `WorkspaceService::list`
- `list_applications` → `ApplicationService::list_by_workspace`

### UI surfaces
- `WorkspaceHome` — current workspace + navigation to primary surfaces
- `WorkspaceSwitcher` — list / create-by-name / activate
- `ApplicationsPanel` + `ApplicationList` + `ActiveApplicationsView` — registry, identity, governed launch, observed actives
- Workspaces stage pairs switcher with existing **DesktopArrangementPanel** (arrangements + unavailable window diagnostics)

---

## Ownership

Every new component documents Purpose / Owner / Inputs / Outputs / Dependencies / Non-responsibilities in a file header.

UI may own: presentation, interaction, navigation, state display.  
UI must not own: window control, permissions, execution policy, Assistant reasoning.

---

## Files

### Created
- `app/src/components/WorkspaceHome.tsx`
- `app/src/components/WorkspaceSwitcher.tsx`
- `app/src/components/ApplicationsPanel.tsx`
- `app/src/components/ApplicationList.tsx`
- `app/src/components/ActiveApplicationsView.tsx`
- `app/src/lib/workspaceSwitcherUi.ts`
- `app/src/lib/applicationsUi.ts`
- `tests/milestone-a-workspace-apps.test.ts`
- `docs/03-Engineering/MILESTONE-A-WORKSPACE-APPS-COMPLETION-REPORT.md`
- `docs/03-Engineering/MILESTONE-A-VISUAL-REVIEW-CHECKLIST.md`

### Modified
- `packages/kernel/src/commands/get_workspace.rs` — `ListWorkspaces`
- `packages/kernel/src/commands/application.rs` — `ListApplications`
- `packages/kernel/src/commands/mod.rs`, `handler.rs`
- `app/src-tauri/src/commands/workspace.rs`, `resources.rs`, `lib.rs`
- `app/src/App.tsx`, `app/src/App.css`
- `docs/README.md` (index)

---

## Validation

| Check | Result |
|-------|--------|
| `pnpm typecheck` | Pass |
| `pnpm test` | Pass (109) |
| `pnpm verify:architecture-governance` | Pass (map refreshed: +2 query commands) |
| `pnpm verify:ipc-contract` | Pass (206 commands) |
| `pnpm verify:ui-experience-boundary` | Pass |
| `cargo test -p workspace-kernel get_workspace::` | Pass |
| `cargo test -p workspace-kernel commands::application::list_tests` | Pass |

---

## Known limitations

- OS app discovery still missing (manual registry only).
- Active applications require prior observation / WorkspaceState; empty until capture.
- Full launch/restore proof remains Windows + Tauri (browser Vite disables data actions).
- Assistant is secondary tab, not yet a persistent sidecar (Milestone C).
- Work modes (Flow ↔ Focus) not in this milestone.

---

## Human review status

**Pending** visual review for Home / Workspaces / Applications / Layouts chrome and Apps interaction.

---

## Maintainability assessment

| Question | Answer |
|----------|--------|
| Where does workspace switching happen? | `WorkspaceSwitcher.tsx` → `App.activateWorkspace` + `list_workspaces` |
| Where does application display happen? | `ApplicationsPanel.tsx` / `ApplicationList.tsx` / `ActiveApplicationsView.tsx` |
| Where is IPC called? | Component `invokeIpc` calls; shared `app/src/lib/ipc.ts` |
| Where are permissions enforced? | Kernel CommandPipeline / PermissionGateway (unchanged) |

---

## Product vision movement

Closer: users can open Workspace → Home → manage apps and switch work environments, with arrangements still available beside the switcher. Assistant is visually demoted. Remaining gaps: modes, chrome sidecar, discovery.
