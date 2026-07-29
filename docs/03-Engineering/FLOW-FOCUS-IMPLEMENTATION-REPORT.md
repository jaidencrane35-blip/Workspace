# Milestone B — Flow / Focus Chrome Density — Implementation Report

| Field | Value |
|-------|-------|
| **Status** | Complete (chrome-density only) |
| **Date** | 2026-07-29 |
| **Branch** | `cursor/milestone-b-flow-focus-chrome-34a5` |
| **Charter** | [FLOW-FOCUS-MODE-DESIGN-CHARTER.md](../01-Product/FLOW-FOCUS-MODE-DESIGN-CHARTER.md) |
| **Scope** | Chrome-density + workspace experience — **no OS arrangement apply** |

**Not:** OS window move/resize on mode switch. **Not:** AI / automation / new engines.

---

## Problem solved

Users needed a real Flow ↔ Focus distinction toward the reference images without waiting for Win32 geometry apply. This batch adds **user-controlled presentation density** over the existing Layouts stage, application registry, and arrangements rail.

---

## Files changed

| File | Role |
|------|------|
| `app/src/lib/workMode.ts` | Mode model, storage key, labels, named constants |
| `app/src/components/WorkModeSwitch.tsx` | Segmented Flow/Focus control |
| `app/src/components/WorkspaceApplicationStage.tsx` | Dense vs primary/supporting stage presentation |
| `app/src/components/WorkspaceHome.tsx` | Mode awareness + updated “not available yet” |
| `app/src/App.tsx` | Mode state, chrome switch, Focus hides companion canvas |
| `app/src/App.css` | Density styles |
| `tests/milestone-a-workspace-apps.test.ts` | workMode helper tests |
| This report + visual checklist | Documentation |

---

## Architecture impact

| Layer | Impact |
|-------|--------|
| Domain / kernel / IPC | **None** |
| DesktopArrangement / WindowController | **Untouched** — still available in rail; not applied on mode switch |
| Application registry | Read-only presentation reuse |
| UI shell | Presentation preference (`localStorage`) + CSS `data-work-mode` |

No new layout engine. No Assistant behaviour change.

---

## Ownership boundaries

| Concern | Owner |
|---------|-------|
| Work mode preference | `workMode.ts` + App shell |
| Stage density presentation | `WorkspaceApplicationStage` |
| OS geometry | Still DesktopArrangement restore path (future batch) |
| Permissions | Unchanged PermissionGateway |

---

## Behaviour

### Flow
- Dense application tile grid on Layouts
- Companion canvas visible
- Arrangements rail full
- Overview-oriented copy

### Focus
- One primary application emphasised; others as supporting chips (selectable to promote)
- Companion canvas suppressed (honest note + return to Flow)
- Arrangements rail retained (slightly quieter chrome)
- Clear copy: apps not quit; OS windows not moved

---

## Validation

| Check | Result |
|-------|--------|
| `pnpm typecheck` | Pass |
| `pnpm build` | Pass |
| `pnpm test` | Pass |
| `verify:architecture-governance` | Pass |
| `verify:ipc-contract` | Pass |
| `verify:ui-experience-boundary` | Pass |

---

## Scores

| Score | Value |
|-------|------:|
| **Maintainability** | **8.5/10** — small pure module, named constants, ownership headers, no OS side effects |
| **Reference alignment** | **7.5/10** — density switch + stage emphasis; still no live OS tiles or persistent Assistant rail |

---

## Human visual review

Required. Launch:

```bash
cd app && pnpm exec vite
# http://localhost:1420 → Layouts → toggle Flow / Focus
```

Checklist: [FLOW-FOCUS-VISUAL-REVIEW-CHECKLIST.md](FLOW-FOCUS-VISUAL-REVIEW-CHECKLIST.md)

---

## Explicit non-goals preserved

- No OS arrangement apply on mode switch  
- No AI recommendations / automation  
- No new intelligence or layout engines  
