# Visual Review Checklist — DAF-1e

| Field | Value |
|-------|-------|
| **Checkpoint name** | DAF-1e Desktop Arrangement UI foundation |
| **Date** | 2026-07-29 |
| **Branch / PR** | `cursor/daf-1e-arrangement-ui-34a5` |
| **Related batch** | DAF-1e |
| **Human review required?** | **Yes** ([HUMAN-REVIEW-POLICY.md](HUMAN-REVIEW-POLICY.md) — new major UI surface) |

---

## 1. What changed

- Primary tab renamed **Workspace** (was Canvas).
- Workspace stage layout: canvas + **Desktop Arrangements** rail.
- Save / list / select / restore UI consuming DAF-1d IPC.
- Restore diagnostics presentation for gaps and apply outcomes.

## 2. Why it changed

Make existing capture/restore capability usable without expanding ownership into AI or layout engines.

## 3. What needs verification

- Arrangements rail is discoverable beside the workspace stage.
- Copy leads with “remember your setup”, not AI organisation.
- Empty states are clear (no workspace / no arrangements / runtime unavailable).
- Restore is an explicit button; no auto-restore cues.
- Assistant remains a separate supporting tab.

## 4. Expected behaviour

- With Tauri runtime: list/capture/restore invoke IPC.
- In browser Vite: controls disabled or hint that desktop runtime is required; UI still renders.

## 5. Known limitations

- Full HWND restore verification requires Windows + Tauri.
- Browser Vite cannot exercise successful restore against OS windows.
- Assistant-as-true-sidecar chrome deferred to DAF-1f.

## 6. How to inspect

| Method | Used? | Notes |
|--------|-------|-------|
| Live app (browser Vite / Tauri) | Yes | `cd app && pnpm exec vite` → http://localhost:1420 |
| Screenshot(s) | Yes | See artifacts under `/opt/cursor/artifacts/screenshots/` |
| Screen recording | No | Not requested |

**App URL / launch command:** `cd app && pnpm exec vite` → http://localhost:1420 (Workspace tab)

## 7. Reviewer outcome

| Criterion | Pass / Fail / N/A | Notes |
|-----------|-------------------|-------|
| Usability | ☐ | Pending human |
| Visual quality | ☐ | Pending human |
| Product feel | ☐ | Pending human |
| Matches visual direction (non-literal) | ☐ | Pending human |
| Navigation / interaction | ☐ | Pending human |
| Desktop arrangement (if applicable) | ☐ | Pending human |

**Decision:** ☐ Accept  ☐ Changes requested  ☐ Defer

### Agent pre-check notes

- Screenshots captured of Workspace stage + arrangements rail (empty / runtime-hint states):
  - `/opt/cursor/artifacts/screenshots/daf-1e-workspace-stage-arrangements-rail.png`
  - `/opt/cursor/artifacts/screenshots/daf-1e-arrangements-panel-detail.png`
  - `/opt/cursor/artifacts/screenshots/daf-1e-assistant-tab-separation.png`
- Rail copy and placement match charter; Assistant remains a separate tab.
- Browser Vite shows expected runtime-unavailable banner; no fabricated success.
- No automatic review videos generated.
