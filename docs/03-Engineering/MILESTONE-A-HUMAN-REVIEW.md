# Milestone A — Human Review & Alignment Checkpoint

| Field | Value |
|-------|-------|
| **Status** | Checkpoint review — analysis only |
| **Date** | 2026-07-29 |
| **Branch** | `cursor/milestone-a-workspace-apps-switcher-34a5` |
| **Inspection mode** | Browser Vite (`http://localhost:1420`) — Tauri desktop unavailable in this environment |
| **Screenshots** | `/opt/cursor/artifacts/screenshots/milestone-a-review-*.png` (+ earlier `milestone-a-*.png`) |
| **Vision gap detail** | [MILESTONE-A-VISION-GAP-REPORT.md](MILESTONE-A-VISION-GAP-REPORT.md) |
| **Commit** | **Not created** — awaiting human approval of findings / next batch |

**Core principle checked:** Workspace is the product. AI is a capability inside Workspace.

---

## Summary

Milestone A **moved product chrome in the right direction**: primary navigation is Home → Workspaces → Applications → Layouts; Assistant is labelled and styled as secondary; Apps and switcher foundations reuse existing services without new AI engines.

Usability in live browser review is still **foundation-grade (~5–6/10 overall)**: empty states dominate, switcher/apps feel form-like rather than desktop-tool-like, and Assistant remains a **full peer tab** rather than a sidecar. Work + Diagnostic still add nav noise.

**Verdict:** Alignment trajectory is good. Do not expand AI. Prefer product UX / modes next — not intelligence layers.

---

## Visual Review

Inspection: live click-through of Home, Workspaces, Applications, Layouts, Assistant (plus Work / Diagnostic for hierarchy notes). Expected red IPC banner in browser Vite.

### Workspace Home — score **6/10**

| Question | Finding |
|----------|---------|
| Where am I? | Brand + Home eyebrow present; “Your workspace” is slightly ambiguous |
| What workspace? | Empty state clear (“No active workspace”); populated path shows name + zone count (not exercised without Tauri) |
| How to switch? | Card to Workspaces is visible |
| Where are apps? | Card to Applications is visible |

**Why 6:** Orientation and primary actions exist, but Home still feels sparse/admin; cards sit under empty-state messaging; no apps-as-stage presence.

### Workspace Switcher — score **6/10**

| Check | Finding |
|-------|---------|
| Discoverability | Reachable from nav + Home |
| Clarity | “Saved work environments” helps first-time understanding |
| Speed of switching | Cannot exercise without Tauri; UI is a list/form, not a visual switcher |
| Hierarchy | Create / switch / arrangements rail are understandable |

**First-time without docs?** Partially — concept is stated; desktop-runtime empty copy is honest but heavy.  
**Feels like desktop workspace tool?** Partially form/settings; missing tiles/previews of environments.

### Applications Panel — score **4/10** (empty browser state)

| Check | Finding |
|-------|---------|
| Visibility | Blocked by “select a workspace first” in this session |
| Organisation | Registry + active observation design is sound when populated |
| Launch flow | Governed launch + executable-path gate — clear when data exists |
| Relation to layouts | Weak — little copy tying apps to arrangements/Layouts |

**Feel:** Closer to a **settings/registry panel** than a workspace manager stage in the empty state. Structure is correct for a foundation; product feel incomplete.

### Assistant Placement — score **4/10** toward sidecar goal

| Check | Finding |
|-------|---------|
| Size | Full main content area when selected |
| Prominence | Still a top-level tab peer to Home/Apps/Layouts |
| Nav placement | After Layouts with dashed/secondary class — helpful but subtle |
| Visual priority | Copy correctly demotes Assistant; structure does not |

**Conclusion:** Intent is right; chrome is not yet a supporting side panel. Stale Assistant copy still says “Open the Workspace tab” in places (should be Home/Workspaces).

### Chrome noise

Work + Diagnostic remain top-level. Diagnostic copy correctly points users to Home/Apps/Layouts; Work still competes for attention. Seven tabs exceed concept chrome calmness.

---

## Product Alignment

See also [MILESTONE-A-VISION-GAP-REPORT.md](MILESTONE-A-VISION-GAP-REPORT.md).

| Vision | Post–Milestone A |
|--------|------------------|
| Apps stage | Companion canvas + registry/launch — not OS app tiles as stage |
| Movable layouts / modes | Arrangements save/restore exist; **modes missing** |
| Grouped apps | Missing |
| Saved arrangements | Present (DAF-1c–1e + rail) |
| Workspace switching | Foundation present |
| Audio | Missing (future) |
| Assistant side panel | Tab demotion only |

**Gap priority:** modes + apps-as-stage feel + true Assistant sidecar.

---

## Maintainability Score

### Human engineer maintainability — **7/10** (Milestone A surfaces)

Definition: Could a senior engineer understand, modify, debug, and extend **these Milestone A files** without AI?

| Area | Assessment |
|------|------------|
| File organisation | Good — `WorkspaceHome`, `WorkspaceSwitcher`, `ApplicationsPanel`, helpers under `app/src/lib/*Ui.ts` |
| Names | Descriptive; match responsibilities |
| Concept separation | UI presentation vs kernel permissions/control preserved |
| Headers | Purpose / Owner / Inputs / Outputs / Dependencies / Non-responsibilities present |
| Broader repo | Still hard: giant `domain.ts`, OperatorConsole, intelligence panels, large `handler.rs` (not introduced by A) |

**Overall product shell (including legacy intelligence tabs):** closer to **5–6/10** until Work/Diagnostic/Assistant surfaces shrink or move.

### Abstraction quality

**Good:** Thin IPC queries (`list_workspaces`, `list_applications`) over existing services; no wrapper pyramids; no new engines.

**Watch:** `App.tsx` still owns bootstrap + view routing + workspace activation (acceptable for now; avoid growing into a god component).

### Magic numbers / hidden assumptions

| Finding | Problem | Impact | Suggested fix |
|---------|---------|--------|---------------|
| `limit: 200` in `loadZones` (`App.tsx`) | Unexplained zone fetch bound | Silent truncation risk if many zones | Named constant + comment (e.g. context page size) |
| Arrangements `limit: 50` (existing panel) | Same pattern | Incomplete list without notice | Shared named constant / “showing N of …” later |
| Browser IPC treated as hard **Error** banner | Assumes missing Tauri = failure | Preview feels “broken” | Softer runtime-unavailable status for Vite |
| Home `Create workspace` uses fixed name `"Canvas Workspace"` | Hidden naming assumption | Duplicate generic names | Prefer named create on Workspaces only, or prompt |
| Assistant copy references “Workspace tab” | Stale after chrome rename | Confusion | Update strings to Home/Workspaces |

### Documentation quality

Milestone A components answer “what / why / who owns” via file headers. Completion + visual checklist exist. Engineers still need DAF docs for arrangement/restore ownership — appropriate.

---

## AI Engineering Boundary Review

| Constraint | Status |
|------------|--------|
| No new AI engines | **Confirmed** — Milestone A added list queries + product UI only |
| No autonomous workflows / reasoning / recommendation expansion | **Confirmed** |
| No hidden user modelling | **Confirmed** |
| Assistant continues via Surface → Context → Retrieval → Explanation → Interaction → Personalisation | **Unchanged** — no parallel stack |
| Assistant not workflow controller / source of truth | **Preserved** — launch/restore still PermissionGateway + existing owners |

Frozen surface remains frozen. Do not start Batch 17 from this checkpoint.

---

## Technical Debt (from this review)

Recommendations only — no deletions from this checkpoint.

1. **Assistant still peer-tab** — High — Milestone C sidecar chrome  
2. **Work + Diagnostic nav noise** — Medium — group/hide under Diagnostic or settings  
3. **Apps feel like settings** — Medium — product presentation pass when data available (icons, relationship to arrangements)  
4. **Dual layout vocabulary** — Medium — keep Canvas vs Desktop Arrangement naming explicit in UI  
5. **Runtime banner severity** — Low/Medium — presentation fix for browser preview  
6. **Stale Assistant tab strings** — Low — copy fix  

---

## Optimisation Review (analysis only)

### Critical (fix now)
- None identified for Milestone A correctness.

### Important (schedule)
- Bootstrap always fetches `get_workspace_status` + `get_workspace_health` and discards results in `App` — unnecessary startup IPC when Tauri is present.
- Switching Workspaces ↔ Layouts remounts `DesktopArrangementPanel` (duplicate fetch). Consider lifting list state or accepting remount until polish.

### Optional (ignore for now)
- Bundle size of intelligence panels still large — defer until Assistant/Work chrome shrinks.
- No evidence of render thrash unique to Milestone A helpers (pure label functions).

**Do not rewrite for speculative performance.** Log any future optimisation in `docs/04-Operations/OPTIMISATION_LOG.md`.

---

## Recommended Next Steps

1. **Human Accept / Changes** on Milestone A visual checklist (this checkpoint).  
2. Choose next batch (await approval):
   - **Option 1 (recommended for vision):** **Milestone B — Work modes** (productive ↔ focus) on top of arrangements.  
   - **Option 2 (recommended if UX first):** **Milestone A.1 — Product UX hardening** (banner, nav demotion, copy, empty-state education) then B.  
   - **Option 3:** **Milestone C — Assistant sidecar** after core stage exists to host the rail.  
3. **Do not** expand Assistant engines or create Batch 17.

---

## Human Review Required?

**Yes** — meaningful visual/product checkpoint.

| Ask human to verify | Why |
|---------------------|-----|
| Home / Workspaces / Applications / Layouts hierarchy | Product direction |
| Whether Assistant demotion is “enough” for now | Sidecar still deferred |
| Preference: A.1 polish vs Milestone B modes next | Roadmap gate |

**Not required for:** docs-only edits, test-only changes, or approving freeze of AI expansion (already governed).

### How to inspect

```bash
cd app && pnpm exec vite
# → http://localhost:1420
```

Prefer Tauri on Windows for data actions. Screenshots: `/opt/cursor/artifacts/screenshots/milestone-a-review-*.png`.

**Decision:** ☐ Accept Milestone A  ☐ Changes requested  ☐ Defer  · Next batch: ☐ A.1  ☐ B modes  ☐ C sidecar
