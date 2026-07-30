# Workspace Product Delivery Roadmap

| Field | Value |
|-------|-------|
| **Purpose** | Sequence the next product milestones that turn the approved concept references into a shippable desktop workspace product |
| **Owner** | Product / Engineering |
| **Status** | Planning only — **no implementation authorised by this document** |
| **Audience** | Product, engineering, commercial review |
| **References** | [`references/workspace-concept-01.png`](references/workspace-concept-01.png), [`references/workspace-concept-02.png`](references/workspace-concept-02.png) |
| **Visual north star** | [WORKSPACE-VISUAL-DIRECTION.md](WORKSPACE-VISUAL-DIRECTION.md) |
| **Related** | [WORKSPACE-PRODUCT-ALIGNMENT-AUDIT.md](WORKSPACE-PRODUCT-ALIGNMENT-AUDIT.md), [FLOW-FOCUS-MODE-DESIGN-CHARTER.md](FLOW-FOCUS-MODE-DESIGN-CHARTER.md), [DAF Architecture Audit](../03-Engineering/DAF-ARCHITECTURE-AUDIT.md), [08-Roadmap/ROADMAP.md](../08-Roadmap/ROADMAP.md) |

**Await human approval before starting Milestone D.**  
Do not treat this file as permission to code.

---

## 1. What Workspace is (read this first)

Workspace is a **desktop workspace operating environment** for Windows.

A person opens Workspace to:

1. See and manage the applications that belong to their work
2. Arrange those applications on the real desktop
3. Save and restore named desktop setups
4. Switch between productive density (Flow) and quieter density (Focus)
5. Optionally ask the Assistant for help — as a companion, never as the product

```text
Desktop Workspace experience
 ├── Applications          ← primary stage
 ├── Layouts               ← how the stage is organised
 ├── Desktop Arrangements  ← remember / restore real windows
 ├── Flow / Focus          ← density of the same working set
 ├── Assistant companion   ← supporting side rail
 └── Diagnostics / tools  ← engineering surfaces (not product)
```

The concept images are **direction**, not pixel specs. Match hierarchy and feel — do not copy brands, wallpaper, or nine layout products at once.

---

## 2. Foundation already in place (reuse this)

Engineering maturity is assumed. Product delivery builds **on** these systems — it does not replace them.

| Layer | What already exists | Use in product milestones |
|-------|---------------------|---------------------------|
| Product shell | Home / Workspaces / Applications / Layouts; Flow/Focus chrome density; Assistant companion rail | Extend stage and chrome — do not rebuild navigation |
| Application registry + launch | Domain + IPC + Permissions + launcher | Stage cards, launch, membership |
| Window observation | `windows-integration` capture + identity | Live stage facts, grouping inputs |
| Window control | `WindowController` (`set_bounds`, `focus`) via CommandPipeline → Permission Gateway | All real window moves |
| Desktop arrangements | Persist membership + bounds; capture / restore UI rail (DAF-1c–1e) | Editing, Flow/Focus geometry, groups |
| Companion canvas zones | Layout board (practice surface) | Keep secondary to OS arrangements; do not confuse names |
| Assistant | Existing read/compose panels in the companion rail | No new AI engines |
| Diagnostics | Operator console | Stay in tools group |

**Rule for every milestone:** reuse → extend → create only if unavoidable. No parallel “layout engine”, “mode engine”, or Assistant-owned window mover.

---

## 3. Sequence decision

### Suggested letters (from the brief)

D Stage → E Grouping → F Arrangement editing → G Flow/Focus behaviour → H Audio → I Polish

### Recommended ship order (this roadmap)

```text
D  Workspace Stage
F  Desktop Arrangements (real editing)
G  Flow / Focus Behaviour (real arrangement switching)
E  Desktop Grouping
H  Audio Mixer
I  Workspace Polish
```

### Why this order is better

| Choice | Reason |
|--------|--------|
| **F before E** | Named save/restore editing delivers the core “my desktop remembers” loop using **existing** DesktopArrangement + restore pathways. Grouping needs batch move/resize on top of that foundation and is higher risk. |
| **G before E** | Concept-01’s Flow ↔ Focus promise is arrangement density of the **same apps**, not new group math. G reuses arrangement variants + governed restore — earlier commercial proof of the north star. |
| **E after G** | “Move/resize/lock together” is the next power feature once single-window arrangements and mode switching are trustworthy. |
| **H late** | Audio is a supporting utility in the art — valuable, but not the centrepiece. Needs OS capability discovery; no audio domain yet. |
| **I last** | Commercial polish after behaviour is real; avoid polishing incomplete stage semantics. |

Letter labels stay stable for discussion; **execution order** follows the recommended ship order above.

Milestone letters A–C (Apps/switcher, Flow/Focus chrome, companion rail) and DAF-1a–1e are **complete foundations**, not reopened by this roadmap.

---

## 4. Dependency overview

```text
                    ┌─────────────────────┐
                    │  D Workspace Stage  │
                    └──────────┬──────────┘
                               │
                    ┌──────────▼──────────┐
                    │ F Arrangement edit  │
                    └──────────┬──────────┘
                               │
                    ┌──────────▼──────────┐
                    │ G Flow/Focus apply  │
                    └──────────┬──────────┘
                               │
                    ┌──────────▼──────────┐
                    │ E Desktop Grouping  │
                    └──────────┬──────────┘
                               │
              ┌────────────────┼────────────────┐
              ▼                                 ▼
     ┌────────────────┐              ┌──────────────────┐
     │ H Audio Mixer  │              │ I Polish (final) │
     └────────────────┘              └──────────────────┘
```

Assistant companion and Diagnostics remain available throughout; they are **not** milestone drivers.

---

## Milestone D — Workspace Stage

### Purpose

Make **real applications** the visual and interactive centre of Workspace (Layouts / Home), so the product reads as a desktop workspace — not a management form or AI console.

### Why this milestone exists

Concept art shows apps as the stage. Today the shell has registry cards and a stage presentation, but the experience still leans on companion canvas and rails rather than “these are my working apps.” D closes that perception gap with **visible product capability** first.

### User value

“I open Workspace and immediately see the apps that belong to this workspace. I can select, launch, and understand what is on stage.”

### Owner

Frontend product shell (primary) · Application domain / IPC (supporting) · Observation (read-only facts)

### Existing systems reused

- Application registry + `list_applications` / launch IPC  
- `WorkspaceApplicationStage`, `ApplicationsPanel`, `ApplicationList`, Focus partition helpers  
- Window observation summaries (where available)  
- Desktop Arrangement rail (unchanged behaviour)  
- Assistant companion rail (stays secondary)

### Files expected to change

- `app/src/components/WorkspaceApplicationStage.tsx`  
- `app/src/components/ApplicationsPanel.tsx` / `ApplicationList.tsx`  
- `app/src/App.tsx` / `App.css` (stage hierarchy only)  
- Possibly thin presentation helpers under `app/src/lib/`  
- Tests under `tests/` for stage copy/contracts  

**Unlikely to need new packages.** Prefer extending observation read models already exposed over inventing a stage service.

### Out of scope

- Moving OS windows on mode switch  
- Window grouping / lock / group resize  
- New AI features or engines  
- Bottom-nav IA rewrite (defer to I unless already approved)  
- Installed-app OS discovery (may be a thin D stretch only if reuse is trivial; otherwise later)

### Completion definition

- [ ] Layouts stage is the dominant product surface for the active workspace’s apps  
- [ ] Empty / loading / runtime-unavailable states remain honest  
- [ ] Launch still goes through existing permissioned paths  
- [ ] Assistant remains companion rail, not stage owner  
- [ ] Human visual review against concept hierarchy (apps first)

### Human review requirements

Batched visual review: Home + Layouts + Applications with Flow and Focus chrome. Screenshots only if they help review — no videos required.

### Architecture risks

- Confusing companion **canvas zones** with **OS windows** — naming and copy must stay explicit  
- Temptation to fake live HWND thumbnails — forbidden unless observation provides real facts  

### Validation requirements

`pnpm typecheck`, `pnpm test`, architecture / IPC / UI-boundary verifies, `pnpm build`. Windows smoke: stage + launch with Tauri runtime.

### Commercial value

First impression: Workspace looks like a desktop product. Unlocks demos without inventing new backend systems.

---

## Milestone F — Desktop Arrangements (real editing)

### Purpose

Turn the existing arrangement rail into a **product-grade editor** for named desktop setups: clearer membership, bounds honesty, capture/restore confidence, and editable metadata — without a new arrangement engine.

### Why this milestone exists

Save/restore is the economic core (“come back tomorrow; windows are where I left them”). DAF-1c–1e built the pathway; F makes it **daily-driver usable**.

### User value

“I save this desktop setup by name, adjust which windows belong, restore it later, and understand what succeeded or failed.”

### Owner

Desktop Arrangement domain + UI · Kernel restore commands · WindowController (unchanged ownership)

### Existing systems reused

- DesktopArrangement persistence and migrations  
- Capture / restore via CommandPipeline → Permission Gateway → WindowController  
- `DesktopArrangementPanel` + diagnostics views  
- Observation identity for membership matching  

### Files expected to change

- `app/src/components/DesktopArrangement*.tsx`  
- `packages/domain` arrangement types (extend, don’t fork)  
- `packages/kernel` arrangement commands (extend)  
- `packages/database` only if additive migration required  
- Arrangement UI helpers / tests  

### Out of scope

- Flow/Focus auto-geometry (Milestone G)  
- Group move/resize (Milestone E)  
- AI-suggested arrangements  
- Replacing Permission Gateway or WindowController  

### Completion definition

- [ ] User can create, rename, select, capture, restore with clear feedback  
- [ ] Membership and bounds are inspectable by a human without AI help  
- [ ] Restore gaps/diagnostics remain evidence-only and readable  
- [ ] Independently shippable on Windows with human verification of restore  

### Human review requirements

Windows desktop verification of capture/restore on at least two apps. Visual review of rail under Flow/Focus chrome.

### Architecture risks

- Expanding arrangement schema into a second geometry store — **forbidden**  
- Blurring canvas Layout persistence with DesktopArrangement — keep separate  

### Validation requirements

Standard frontend verifies + Rust tests for touched crates (`workspace-domain`, arrangement-related kernel/database as applicable). Windows restore smoke required for “done.”

### Commercial value

Credible “Workspace remembers my desktop” story — primary purchase reason.

---

## Milestone G — Flow / Focus Behaviour

### Purpose

Connect the existing Flow ↔ Focus **chrome** to **real desktop density** by applying user-selected arrangement variants (or equivalent saved setups) through the governed restore path — apps stay open.

### Why this milestone exists

Concept-01 is incomplete while mode switch only changes UI density. G delivers the promised behaviour using arrangements already owned by DesktopArrangement — **not** a new mode engine.

### User value

“I switch to Focus and my desktop quiets to a primary layout; I switch to Flow and the multitasking setup returns — without quitting my apps.”

### Owner

Product shell (mode UX) · Desktop Arrangement restore · WindowController  

### Existing systems reused

- `workMode` preference + `WorkModeSwitch`  
- DesktopArrangement capture/restore  
- Flow/Focus design charter rules (user-driven; no autonomous switch)  
- Assistant rail (unchanged placement)

### Files expected to change

- `app/src/lib/workMode.ts` / shell wiring (link mode → arrangement choice)  
- Arrangement UI (associate Flow/Focus variants with a workspace)  
- Kernel restore invocation paths already used by the rail  
- Docs: update charter status from chrome-only to behaviour shipped  

### Out of scope

- AI-chosen layouts or time-based auto switch  
- Quitting apps on mode change  
- Nine concept-02 presets  
- New “ModeEngine” package  

### Completion definition

- [ ] User-initiated Flow/Focus can apply distinct saved arrangement geometries via existing restore  
- [ ] Failure paths are honest (partial restore, missing windows)  
- [ ] Chrome density and OS density stay conceptually aligned in copy  
- [ ] Charter non-goals still hold  

### Human review requirements

Windows verification of Flow→Focus→Flow with apps remaining running. Visual + behavioural checklist update.

### Architecture risks

- Implementing mode logic inside Assistant — **forbidden**  
- Silent window moves without Permission Gateway — **forbidden**  

### Validation requirements

Frontend verifies + Windows behavioural smoke. Prefer extending arrangement tests over new frameworks.

### Commercial value

Differentiating demo: one control transforms the working desktop — the concept image becomes believable.

---

## Milestone E — Desktop Grouping

### Purpose

Let users treat related windows as a **group**: move, resize, and optionally lock them together, using observation + WindowController batch applies — not a new window OS.

### Why this milestone exists

Concept and audits call for “applications controlled together.” Environment window groups today are informational only. E turns that into user-authored control **after** arrangements and mode switching are solid.

### User value

“I group my research windows, drag once, and they stay together; I can lock the group so it doesn’t drift.”

### Owner

Desktop / observation domain extension · WindowController batch operations · Product UI for groups  

### Existing systems reused

- Observation (`EnvironmentWindowGroup` as **input facts**, not the product model)  
- WindowController per-window bounds/focus  
- DesktopArrangement membership (groups should compose with arrangements, not replace them)  
- Permission Gateway for every mutation  

### Files expected to change

- `packages/domain` (group model owned beside arrangements — extend carefully)  
- `packages/windows-integration` / kernel commands for batched bounds  
- `packages/database` additive migrations if persisting user groups  
- `app/src` group presentation on stage / arrangements  

### Out of scope

- AI auto-grouping  
- Replacing DesktopArrangement  
- Global OS hooks outside WindowController  

### Completion definition

- [ ] User can create/edit a group of observed windows  
- [ ] Move/resize applies through governed batch path with per-window diagnostics  
- [ ] Lock prevents accidental group edits (product lock ≠ canvas-only lock)  
- [ ] Works with an existing arrangement restore story  

### Human review requirements

Windows multi-window verification. Explicit review that lock semantics match user expectation.

### Architecture risks

- Inventing a second geometry system parallel to DesktopArrangement  
- Treating observational `EnvironmentWindowGroup` as writable product state without a clear ownership boundary  

### Validation requirements

Domain/kernel tests for batch apply + UI verifies. Windows smoke with ≥2 windows.

### Commercial value

Power-user control that competitors under-deliver; deepens “desktop operating workspace” positioning.

---

## Milestone H — Audio Mixer

### Purpose

Add per-application (and later per-group) audio level controls where the OS allows — a supporting utility from the concept art, not a new product.

### Why this milestone exists

Concept references show per-app utilities. Audio is high visible polish **after** stage, arrangements, modes, and groups exist to hang controls on.

### User value

“I quiet one app’s audio without hunting through the system mixer.”

### Owner

New thin audio boundary (only if required) · Product UI utilities strip · OS integration under existing permission patterns  

### Existing systems reused

- Application identity / observation for targeting sessions  
- Product shell utilities area (do not put mixer in Assistant)  
- Permission / capability patterns used by other OS touches  

### Files expected to change

- Likely new `packages/` audio boundary **only after** proving Win32 session APIs fit ownership rules  
- `app/src` mixer UI  
- IPC contracts + verifies  

### Out of scope

- Assistant-driven mute automation  
- Cross-device exotic audio graphs  
- Blocking Milestones D–G on audio  

### Completion definition

- [ ] Per-app volume/mute for supported sessions  
- [ ] Clear unsupported/empty states when OS APIs cannot bind  
- [ ] Optional per-group aggregate only if E shipped  

### Human review requirements

Windows audio device verification. Confirm mixer never appears as primary nav.

### Architecture risks

- Large new subsystem with weak ownership — keep boundary small and explicit  
- Creating audio features that bypass Permission Gateway norms  

### Validation requirements

Contract tests + Windows manual audio check. Skip CI claims the VM cannot prove.

### Commercial value

Concept fidelity and daily convenience; secondary to arrangement/mode story.

---

## Milestone I — Workspace Polish

### Purpose

Final commercial UX pass: information hierarchy, empty states, responsiveness, naming consistency, Windows installer/readiness cues, and optional chrome refinements that do **not** change product philosophy.

### Why this milestone exists

After behaviour is real, polish converts a capable prototype into something a customer trusts. This is not another architecture programme.

### User value

“Workspace feels finished: clear, fast enough, honest when something fails, and pleasant to live in.”

### Owner

Product shell · Docs / release checklist · Light touches across existing panels  

### Existing systems reused

- Entire product shell and DAF stack  
- Human review checklists already in `docs/03-Engineering/`  
- Optimisation Protocol only if a measured polish defect appears — **not** a new optimisation campaign  

### Files expected to change

- `app/src` CSS/copy/chrome consistency  
- Possibly nav labelling (still Workspace-first; bottom nav only if Product explicitly approves)  
- Release/README Windows runbooks  

### Out of scope

- New domains or engines  
- Reopening frozen AI programmes  
- Scenic wallpaper / phone mirroring / nine presets  

### Completion definition

- [ ] Visual review against concept hierarchy passes for Home / Apps / Layouts / Arrangements / Flow-Focus / Assistant rail  
- [ ] Critical Windows journeys documented and smoke-tested  
- [ ] No known P0 UX honesty bugs (fake capabilities, silent failures)  

### Human review requirements

Full product walkthrough on Windows. Commercial stakeholder sign-off.

### Architecture risks

- Polish PRs that sneak in behaviour changes — keep scope cosmetic/UX unless separately approved  

### Validation requirements

Full known-good Linux verifies + Windows release smoke checklist.

### Commercial value

Ship confidence, supportability, and brand trust.

---

## 5. Cross-cutting rules for all milestones

| Rule | Meaning |
|------|---------|
| Independently shippable | Each milestone can release without the next |
| Reuse first | Prefer DesktopArrangement + WindowController + shell |
| Assistant stays companion | No milestone makes AI the centre |
| Permissioned desktop control | All window/audio mutations stay governed |
| Honest empty states | Never fake live windows or successful restore |
| Human-readable ownership | A new engineer can find Purpose / Owner / Non-responsibilities |
| No clone architecture | One geometry story, one control boundary |

---

## 6. What a new engineer should take away

1. **Workspace** = desktop workspace product; Assistant = side companion.  
2. Foundations (DAF + Milestones A–C) already provide control, persistence, chrome, and companion rail.  
3. **Next code** starts only after approval of **Milestone D (Workspace Stage)**.  
4. Sequence **D → F → G → E → H → I** maximises reuse and early visible value.  
5. Grouping and audio come after the arrangement/mode loop is real.  
6. Architecture already supports this path; milestones extend it rather than inventing parallel systems.

---

## 7. Approval gate

| Decision | Ask |
|----------|-----|
| Accept this roadmap? | ☐ Yes ☐ Changes requested |
| Accept recommended order D→F→G→E→H→I? | ☐ Yes ☐ Prefer brief’s D→E→F→G→H→I |
| Authorise Milestone D implementation? | ☐ Approved ☐ Not yet |

**Agent instruction:** Stop after publishing this roadmap. Do not implement Milestone D until explicitly approved.
