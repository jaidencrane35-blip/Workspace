# Programme IV Consolidation & Validation Report

**Status:** Consolidation milestone (post–Batch 16)  
**Branch:** `cursor/programme-iv-consolidation-34a5`  
**Date:** 2026-07-29  
**Audience:** Engineering leads, product, commercial due diligence

This is **not** Programme IV Batch 17. Feature expansion is frozen; work focuses on shipping, consolidation, validation, and human usability.

---

## 1. Merge summary

### Stack reality

Programme IV Batches 1–16 were developed as a **linear stacked history**. Tip `cursor/programme-iv-assistant-personalisation-34a5` @ `8e9bf65` already contains Batches 1–16 in architectural order (Semantic Query → … → Personalisation), plus Programme III underpinnings — **123 commits ahead of `origin/main`**.

Charter-only branches are ancestors of their implementation tips; merging each tip separately would re-apply the same commits. Correct merge strategy:

| Step | Action |
|---|---|
| 1 | Treat `assistant-personalisation-34a5` tip as the ordered Programme IV stack |
| 2 | Consolidation branch `cursor/programme-iv-consolidation-34a5` extends that tip with debt fixes + Assistant UI |
| 3 | Open PR consolidation → `main` (no history squash of Programme IV commits) |
| 4 | After merge, `main` becomes authoritative Programme IV implementation |

### Dependency order (preserved in history)

1. Batches 1–9 Evidence engines  
2. Batch 10 Observational Scaffold  
3. Batch 11 Surface → 12 Context → 13 Retrieval → 14 Explanation → 15 Interaction → 16 Personalisation  

### Gate note

Repository token permissions do not allow this agent to push directly to `main`. Consolidation is shipped as a PR for human merge approval. After each logical consolidation commit, `cargo check -p workspace-kernel`, `pnpm typecheck`, IPC, and architecture governance were run on the tip.

---

## 2. Engineering debt resolved

| Item | Fix | Commit |
|---|---|---|
| `case5_timeline_deterministic` | Filter generation telemetry **before** Activity Graph audit operational window | `d511006` |
| `case11_evaluation_does_not_contaminate_its_own_inputs` | Reopen Expired recommendation overlays on same fingerprint when source returns | `c4a5fe2` |
| Programme IV Assistant UI unreachable | Tauri IPC + `AssistantIntelligencePanel` composing Batches 11–16 | `d9c0738` |

Tests kept; assertions not weakened.

---

## 3. Remaining engineering debt

| Item | Severity | Notes |
|---|---|---|
| Evidence-family clone inflation (~1.4–1.7k LOC × engines) | High maintainability | Batch 10 scaffold helped; further mechanical consolidation deferred (prefer edit-in-place macros later — not a new engine) |
| Assistant package domains still large (~1.0–1.3k LOC each) | Medium | Clear ownership; shared helpers already reused; avoid clone-wave “mega module” |
| Linux `workspace-database` `READONLY_DBMOVED` tempdir tests | Platform | Windows CI OK; do not “fix” by breaking Windows semantics |
| Historical `resilience_validation` compile defect on older main | Platform | Verify after merge; consolidation tip compiles `workspace-kernel` |
| Legacy AssistantPanel goal/plan workflow still primary for execution | Product | Intentionally separated under “Governed workflow (legacy)” |

---

## 4. Maintainability score

| Dimension | Score (1–5) | Notes |
|---|---|---|
| Organisation | 4 | Clear `workspace_assistant_*` / `workspace_evidence_*` folders |
| Readability | 3.5 | Contracts clear; some modules still long |
| Discoverability | 4 | Programme IV indexes + vocabulary + batch map |
| Documentation | 4.5 | Architecture docs Active/implemented per batch |
| Consistency | 4 | Dual-channel + `load_snapshot` + projection helpers pattern |
| Extension safety | 4 | Ownership tables + governance guards |
| Commercial maintainability | 3.5 | Healthy layering; clone LOC still a cost centre |

**Overall maintainability: 4.0 / 5** (up from pre–Batch 10 audit caveats; residual clone inflation remains).

---

## 5. Commercial readiness score

| Dimension | Score (1–5) | Notes |
|---|---|---|
| Ownership / authority safety | 4.5 | No SoT elevation in assistant stack |
| Test / governance gates | 4.5 | case5/case11 green; gov baseline 88/39 |
| User-facing surface | 3.5 | First Assistant Intelligence UI present; full data needs Tauri |
| Ops / CI on main | 3 | Pending merge + Windows CI confirmation |
| Human onboarding | 4 | Stack narrative is teachable |

**Overall commercial readiness: 3.9 / 5** — ready to merge and harden on `main`; not yet “ship to end users” without Tauri desktop validation on Windows.

---

## 6. UI implementation summary

- **IPC:** `workspace_assistant_intelligence` Tauri commands for Compose/Package/Get/Explain across Batches 11–16  
- **UI:** `AssistantIntelligencePanel` mounted first on Assistant tab; legacy goal/plan under “Governed workflow (legacy)”  
- **Behaviour:** Projection-only display; uses existing `assistant*Projection.ts` helpers; no approve/execute affordances  
- **Frontend-only caveat:** Vite browser preview cannot invoke Tauri; layout/navigation still reviewable  

---

## 7. Human usability findings

Reviewed with frontend-only Vite preview at `http://localhost:1420` (Tauri IPC unavailable in browser — expected red invoke banner). Assistant tab inspected for navigation, layout, hierarchy, and empty-state clarity.

### What works
- Dark theme and tab chrome are consistent with the rest of the app
- Assistant Intelligence sits above legacy governed workflow with a clear visual delimiter
- Six-layer stack labeling (Batches 11–16) is discoverable once packages exist
- Empty / idle states avoid fake data; presentation-only framing is readable
- Primary compose vs secondary refresh button hierarchy is clear

### Issues found (severity)

| Severity | Finding | Status |
|---|---|---|
| High | Weak next-step when no workspace selected | **Polished** — empty-state hint routes users to **Canvas** tab (initial draft said “Workspaces”; corrected after visual review) |
| High | Dense / overlapping purpose between Programme IV panel and legacy | **Polished** — delimiter + de-emphasized legacy heading + clearer copy |
| Med | Human-ask textarea lacked example prompt | **Polished** — placeholder added |
| Med | Lede text was dense | **Polished** — shortened to stack path + non-authority clause |
| Med | Legacy mutation controls compete visually with intelligence panel | Deferred — product may later hide legacy behind an explicit toggle |
| Low | Diagnostic tab naming / undismissable invoke error banner in browser | Expected for frontend-only; full data needs Tauri/Windows |
| Low | Action buttons wrap on narrow widths | Acceptable; flex-wrap already applied |

### Manual review checklist (Phase 7)

| Criterion | Result |
|---|---|
| Navigation | Pass — Assistant tab reachable; stack order clear |
| Layout / spacing | Pass after polish — delimiter + empty-state spacing |
| Responsiveness | Pass for primary actions (wrap); layers remain single-column |
| Clarity / discoverability | Pass with residual product debt on legacy vs intelligence |
| Interaction flow | Compose gated on ask text; refresh always available |
| Typography / hierarchy | Pass — intelligence hero primary; legacy secondary |
| Obvious bugs | Browser IPC banner only (environment); no layout breakage |

**Usability verdict:** Suitable for engineer/product review of Programme IV composition. Not yet end-user shippable without Windows Tauri E2E and real workspace data.
---

## 8. Recommended next architectural milestone (after consolidation)

Only after this consolidation merges and Windows CI is green:

1. **Evidence-family mechanical consolidation** (edit-in-place macros / shared envelope) — reduce clone LOC without new engines  
2. **Windows desktop E2E** of Assistant Intelligence with real IPC + seeded workspace  
3. **Do not start Batch 17** until product confirms a new boundary is needed  

---

## Explicit confirmation

> Programme IV consolidation freezes feature expansion, lands the stacked Batches 1–16 tip toward `main`,
> resolves case5/case11 without weakening tests, exposes Batches 11–16 in the Assistant UI,
> and prioritises human maintainability over further architecture growth.
