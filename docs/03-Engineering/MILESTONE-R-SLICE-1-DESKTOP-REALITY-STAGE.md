# Milestone R — Slice 1: Desktop Reality Stage

| Field | Value |
|-------|-------|
| **Status** | Implemented — **stop for human visual review** |
| **Date** | 2026-07-30 |
| **Branch** | `cursor/milestone-r-desktop-reality-stage-34a5` |
| **Charter** | [MILESTONE-R-DESKTOP-REALITY-STAGE-CHARTER.md](../01-Product/MILESTONE-R-DESKTOP-REALITY-STAGE-CHARTER.md) |
| **Interpretation** | [WORKSPACE-REFERENCE-INTERPRETATION.md](../01-Product/WORKSPACE-REFERENCE-INTERPRETATION.md) |

## Purpose

Deliver the smallest useful Desktop Reality Stage: opening Workspace shows an **observation-backed** representation of the user’s desktop (or an honest empty/runtime state) **without** requiring workspace creation or app registration.

## Ownership

Frontend product shell (`WorkspaceApplicationStage`, Stage helpers, Home framing).  
Reuses existing `get_workspace_state` IPC — no new engines.

## Hierarchy preserved

```text
Desktop reality (Stage hero)
        ↓
Workspace controls (arrangements rail, optional library)
        ↓
Assistant companion
```

## What changed

| Area | Change |
|------|--------|
| Stage | Always mounts after bootstrap (no create-workspace void) |
| Stage hero | Spatial map from `WorkspaceState.windows` via `layoutStageDesktopWindows` |
| Empty states | Honest: runtime unavailable / no windows observed / error — not invent apps |
| Registry | Demoted to “Library apps (optional)” under the reality map |
| Home / Applications copy | Reality-first; profiles optional |
| Tests | `stage-desktop-ui.test.ts`; layouts stage expectations updated |

## Non-goals (honoured)

- No grouping, audio, Flow/Focus OS apply, arrangement editor polish  
- No new AI / intelligence layers  
- No fake window thumbnails  
- No duplicate window models  

## Maintainability

- Pure layout/copy in `stageDesktopUi.ts` / `layoutsStageUi.ts`  
- Stage owns presentation + one observation fetch; no parallel store  
- Extend `WorkspaceApplicationStage` rather than a replacement app  
- Documented purpose/ownership/non-goals on touched components  

## Human review

See [MILESTONE-R-SLICE-1-VISUAL-REVIEW-CHECKLIST.md](MILESTONE-R-SLICE-1-VISUAL-REVIEW-CHECKLIST.md).

**Do not continue into later R slices or Milestone F until this slice is reviewed.**
