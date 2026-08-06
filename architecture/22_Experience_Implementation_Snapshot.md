# Experience Implementation Snapshot

**Captured:** 2026-08-02  
**Branch state:** experience layer through craftsmanship sprint (`e1a6d81` lineage)  
**Capture environment:** Vite `http://127.0.0.1:1420` (no Tauri IPC). Bootstrapped with `workspace === null`. IPC error toast visible on all screenshots.  
**Theme:** dark-only shell (`tokens.css` / `App.css`); no light-mode stylesheet or toggle exists. “Dark mode” screenshots are the only theme.  
**Screenshot directory:** `architecture/research/experience/screenshots/snapshot-2026-08-02/`  
**Capture script:** `architecture/research/experience/screenshots/snapshot-2026-08-02/capture.mjs`

This document is the authority for the next implementation sprint. It describes code and pixels as they exist, not intended design.

---

# 1. PROJECT STRUCTURE

## Entry / shell

| Role | Path |
|---|---|
| App root | `app/src/main.tsx` |
| App / view router | `app/src/App.tsx` |
| Experience stylesheet | `app/src/App.css` |
| WorkspaceShell | `app/src/components/WorkspaceShell.tsx` |
| WorkspaceCanvas | `app/src/components/WorkspaceCanvas.tsx` |
| WorkspaceComposition (density + ambient bridges) | `app/src/components/WorkspaceComposition.tsx` |
| AmbientLighting | `app/src/components/AmbientLighting.tsx` |
| CommandSurface | `app/src/components/CommandSurface.tsx` |
| Dock | Implemented inside `WorkspaceShell.tsx` as `DockItem` + `motion.nav.ws-dock` (no separate Dock file) |

## Engines

| Role | Path |
|---|---|
| Intent Engine (provider) | `app/src/components/IntentEngine.tsx` |
| Intent Engine (logic) | `app/src/lib/intent.ts` |
| Attention Engine (provider) | `app/src/components/AttentionEngine.tsx` |
| Attention Engine (logic) | `app/src/lib/attention.ts` |

## Surfaces / objects / primitives

| Role | Path |
|---|---|
| WorkspaceSurface | `app/src/components/WorkspaceSurface.tsx` |
| ElevatedCard (deprecated re-export) | `app/src/components/ElevatedCard.tsx` (+ alias in `WorkspaceSurface.tsx`) |
| WorkspaceObject | `app/src/components/WorkspaceObject.tsx` |
| MomentCard | `app/src/components/MomentCard.tsx` |
| EmptyStructure | `app/src/components/EmptyStructure.tsx` |
| IntentionObject | `app/src/components/objects/IntentionObject.tsx` |
| QuickActionObject | `app/src/components/objects/QuickActionObject.tsx` |
| ContinuePreviewBody / ContinuePreviewObject | `app/src/components/objects/ContinuePreviewObject.tsx` |
| CheckInSummaryObject | `app/src/components/objects/CheckInSummaryObject.tsx` |
| GuideStepObject | `app/src/components/objects/GuideStepObject.tsx` |
| RestoreLimitsNotice | `app/src/components/RestoreLimitsNotice.tsx` |

## Destinations (Product Proof chrome)

| Destination | View id (`PilotPrimaryView`) | Component path |
|---|---|---|
| Home | `home` | `app/src/components/HomeWorkspacePanel.tsx` |
| Save | `save` | `app/src/components/SaveContextPanel.tsx` |
| Continue | `resume` | `app/src/components/ResumeContextPanel.tsx` |
| Check-in | `pilot` | `app/src/components/PilotMeasurementPanel.tsx` |
| Guide | `help` | `app/src/components/PilotHelpPanel.tsx` |

Pilot chrome constants: `app/src/lib/pilotChrome.ts`

## Motion / density / object model

| Role | Path |
|---|---|
| Motion utilities | `app/src/lib/motion.ts` |
| Object state types | `app/src/lib/objectState.ts` |
| Density | `app/src/lib/density.ts` |
| Density hook | `app/src/hooks/useWorkspaceDensity.ts` |
| Icons | `app/src/lib/icons.ts` |
| Time formatting | `app/src/lib/time.ts` |
| Restore limits copy | `app/src/lib/restoreLimits.ts` |
| IPC | `app/src/lib/ipc.ts` |

## Design tokens / theme / typography / layout

| Role | Path |
|---|---|
| Token source (TS exports) | `app/src/design-system/tokens.ts` |
| Token CSS variables | `app/src/design-system/tokens.css` |
| CSS var bridge | `app/src/design-system/cssVars.ts` |
| Design-system barrel | `app/src/design-system/index.ts` |
| Theme (runtime) | Dark-only via `tokens.css` `:root` + `App.css` shell classes; no theme provider |
| Typography | `--exp-font` / `--exp-display` in `tokens.css`; type scale in `tokens.ts` `type` |
| Layout system | CSS in `App.css`: `.ws-compose*`, `.spatial-frame*`, `.ws-stage`, `.ws-spatial`, `.attention-orbit`, density breakpoints from `density.ts` |

## Present but not mounted by App (legacy / diagnostic)

| Path | Notes |
|---|---|
| `app/src/components/CanvasShell.tsx` | Superseded by `WorkspaceShell` + `WorkspaceCanvas`; tests assert App does not import it |
| `app/src/components/OperatorConsole.tsx` | Diagnostic; excluded from pilot chrome |
| `app/src/components/AssistantPanel.tsx` | Not mounted |
| `app/src/components/WorkspaceIntelligencePanel.tsx` | Not mounted |
| `app/src/components/DisplayReasonList.tsx` | Used only by unused diagnostic panels |
| `app/src/components/RecommendationExplanationView.tsx` | Used only by unused diagnostic panels |
| `app/src/lib/layoutPersistence.ts` | Used by `CanvasShell` only |
| `app/src/lib/explanationResolver.ts` | Explanation catalog path; not in pilot destination tree |
| `app/src/lib/experienceTranslation.ts` | Boundary helper for diagnostic UI |

## Tests touching experience chrome

- `tests/pilot-chrome.test.ts`
- `tests/pilot-measurement.test.ts`
- `tests/ui-experience-boundary.test.ts`
- `scripts/verify-ui-experience-boundary.mjs`
- `scripts/ui-experience-boundary-lib.mjs`

---

# 2. COMPONENT TREE

Actual mount hierarchy from `main.tsx` → rendered shell (providers shown):

```
React.StrictMode
└── App                          (app/src/App.tsx)
    └── WorkspaceShell           (WorkspaceShell.tsx)
        └── WorkspaceCompositionProvider   (density from useWorkspaceDensity)
            └── AttentionEngineProvider    (via WorkspaceComposition)
                └── IntentEngineProvider   (view-driven)
                    └── ShellBody
                        ├── .ws-layer--bg
                        │   ├── .ws-atmosphere (6 glows + grain + vignette)
                        │   └── AmbientLighting
                        ├── .sr-only aria-live (intent · view label)
                        ├── header.ws-menubar
                        │   ├── .ws-brand (“Workspace”)
                        │   └── .ws-menubar__status
                        │       └── status toasts (error / ok from App)
                        ├── .ws-stage
                        │   └── motion.div.ws-spatial
                        │       └── WorkspaceCanvas
                        │           └── LayoutGroup#workspace-canvas
                        │               └── motion.div.ws-canvas-root
                        │                   ├── .ws-canvas-root__veil
                        │                   └── .ws-canvas-root__field
                        │                       └── AnimatePresence
                        │                           └── motion.div.ws-content  (key=`${intent}-${view}`)
                        │                               ├── {destination panel — one of:}
                        │                               │   ├── HomeWorkspacePanel      (view=home)
                        │                               │   ├── SaveContextPanel         (view=save)
                        │                               │   ├── ResumeContextPanel       (view=resume)
                        │                               │   ├── PilotMeasurementPanel    (view=pilot)
                        │                               │   └── PilotHelpPanel           (view=help)
                        │                               └── CommandSurface
                        └── motion.nav.ws-dock  (role=tablist)
                            ├── .ws-dock__breath
                            └── DockItem × 5
                                (home | save | resume | pilot | help)
                                └── layoutId="ws-dock-pill" when active
```

### Destination subtrees (when mounted)

**Home (`HomeWorkspacePanel`) — no workspace (screenshot state)**
```
section.spatial-frame.ws-canvas.place--empty
├── .place__identity
├── .ws-compose.ws-compose--empty.attention-field
│   ├── WorkspaceObject#home-create (slot=anchor)
│   └── .dash-grid.attention-orbit--ghost
│       └── EmptyStructure
│           └── MomentCard×5 (variant=placeholder)
└── .trust-strip
```

**Home — workspace + latest moment**
```
section.spatial-frame.ws-canvas
├── .place__identity
├── .ws-compose.attention-field
│   ├── .ws-compose__anchor → MomentCard (hero)
│   ├── aside.ws-compose__float
│   │   ├── IntentionObject
│   │   └── .ws-compose__utilities
│   │       ├── QuickActionObject#quick-save
│   │       └── QuickActionObject#quick-continue
│   └── .ws-compose__orbit.dash-grid.attention-orbit   [hidden when density=focus]
│       └── MomentCard×N (compact) | EmptyStructure
└── .trust-strip
```

**Save — write path (workspace present)**
```
section.spatial-frame.save-env.save-env--write.attention-field
└── WorkspaceObject#write-surface
    ├── name input.input-ghost
    ├── textarea.input-ghost.write-textarea
    ├── hint “You write this…”
    ├── button “Review what will be saved”
    └── AnimatePresence → .write-review (step=reviewing)
```

**Continue — browse (workspace + contexts)**
```
section.spatial-frame.continue-gallery.continue-dash
├── header.spatial-header
├── .trust-strip
└── .continue-cinema.ws-compose.attention-field
    ├── .continue-cinema__stage
    │   ├── MomentCard (hero, optional expandContent=ContinuePreviewBody)
    │   └── IntentionObject   [hidden while that moment’s preview open]
    └── .attention-orbit.continue-recede   [density≠focus]
        └── MomentCard×others
```

**Check-in — active (consent + snapshot loaded)**
```
section.spatial-frame.checkin-dash
├── header.spatial-header
├── .checkin-metrics → CheckInSummaryObject×3
└── .checkin-narrative
    ├── .checkin-spatial-trail (chapter chips)
    └── AnimatePresence → .checkin-chat (chapter 0–3 forms)
```

**Guide**
```
section.spatial-frame.guide-dash.guide-walk.attention-field
├── header.spatial-header
├── .guide-experience
│   ├── .guide-experience__stage
│   │   ├── GuideStepObject (active step)
│   │   └── .guide-demo--{write|expand|flow}
│   └── .guide-experience__rail → chip buttons×3
└── WorkspaceObject#guide-trust.quote-pane
```

---

# 3. DESIGN TOKEN INVENTORY

Source of exported TS tokens: `app/src/design-system/tokens.ts`.  
CSS mirrors + extras: `app/src/design-system/tokens.css` and consumption in `app/src/App.css`.

## Spacing (`space`)

| Token | Value |
|---|---|
| `space.1` | `4px` |
| `space.2` | `8px` |
| `space.3` | `12px` |
| `space.4` | `16px` |
| `space.5` | `24px` |
| `space.6` | `32px` |
| `space.7` | `48px` |
| `space.8` | `64px` |

CSS: `--ws-space-*` → aliased `--space-*` (1–7 aliased; 8 exists as `--ws-space-8` only).

## Typography (`type`)

| Token | Value |
|---|---|
| `type.display` | `clamp(2.2rem, 4.4vw, 3.25rem)` |
| `type.title` | `clamp(1.5rem, 2.5vw, 2.05rem)` |
| `type.section` | `1.2rem` |
| `type.body` | `0.98rem` |
| `type.label` | `0.68rem` |
| `type.meta` | `0.82rem` |

Font stacks (CSS only, not in `tokens.ts`):  
`--exp-font`: `"Segoe UI Variable Text", "Segoe UI", Candara, "Gill Sans", sans-serif`  
`--exp-display`: `"Segoe UI Variable Display", "Segoe UI", Candara, sans-serif`

## Elevation (`elevation`)

| Token | Value |
|---|---|
| `elevation.surface` | `1` |
| `elevation.floating` | `2` |
| `elevation.overlay` | `3` |

Mapped in `WorkspaceObject` / `WorkspaceSurface` levels `surface|floating|overlay`.

## Motion duration (`duration`)

| Token | Value |
|---|---|
| `duration.instant` | `0.01` |
| `duration.fast` | `0.14` |
| `duration.base` | `0.28` |
| `duration.slow` | `0.4` |
| `duration.ambient` | `72` |

CSS durations: `--motion-fast: 140ms`, `--motion-base: 280ms`, `--motion-slow: 400ms`, `--ease: cubic-bezier(0.22, 0.82, 0.2, 1)`.

## Spring presets (`spring`)

| Token | Spec |
|---|---|
| `spring.snappy` | stiffness 480, damping 32, mass 0.45 |
| `spring.soft` | stiffness 300, damping 34, mass 0.85 |
| `spring.lush` | stiffness 240, damping 30, mass 1 |
| `spring.dock` | stiffness 420, damping 28, mass 0.4 |
| `spring.layout` | stiffness 340, damping 36 |

## Animation presets (`motionPrimitive` in `lib/motion.ts`)

| Name | Behaviour |
|---|---|
| `reveal` | opacity/y/blur enter-exit; `spring.soft` |
| `elevate` | scale/y lift; `spring.lush` |
| `settle` | scale settle; `spring.soft` |
| `dissolve` | opacity + blur down; `spring.soft` |
| `focus` | scale up + clear blur; `spring.lush` |
| `restore` | height auto expand; `spring.lush` |
| `orbit` | soft secondary presence; `spring.soft` |
| `compress` | scale/opacity down; `spring.soft` |
| `expand` | scale/height expand; `spring.lush` |

Also: `contentTransition`, `layoutTransition`.

## Lighting (`lighting`)

| Token | Value |
|---|---|
| `lighting.ambient` | `rgba(95, 208, 216, 0.12)` |
| `lighting.warm` | `rgba(232, 184, 122, 0.1)` |
| `lighting.focus` | `rgba(142, 236, 240, 0.18)` |
| `lighting.hover` | `rgba(255, 255, 255, 0.06)` |
| `lighting.dim` | `rgba(0, 0, 0, 0.45)` |

## Opacity (`opacity`)

| Token | Value |
|---|---|
| `opacity.mute` | `0.55` |
| `opacity.soft` | `0.72` |
| `opacity.glass` | `0.62` |
| `opacity.strong` | `0.88` |
| `opacity.full` | `1` |

## Blur (`blur`)

| Token | Value |
|---|---|
| `blur.surface` | `18px` |
| `blur.floating` | `26px` |
| `blur.overlay` | `34px` |
| `blur.dock` | `30px` |

## Radius (`radius`)

| Token | Value |
|---|---|
| `radius.sm` | `10px` |
| `radius.md` | `18px` |
| `radius.lg` | `26px` |
| `radius.xl` | `32px` |
| `radius.pill` | `999px` |

## Shadow (CSS-only; not exported from `tokens.ts`)

| Token | Value |
|---|---|
| `--shadow-1` | `0 10px 36px rgba(0,0,0,0.38)` |
| `--shadow-2` | `0 22px 64px rgba(0,0,0,0.48)` |
| `--shadow-3` | `0 32px 90px rgba(0,0,0,0.55)` |
| `--shadow-4` | `0 40px 120px rgba(0,0,0,0.6)` |
| `--shadow-glow` | `0 0 56px var(--ws-light-ambient)` |
| `--edge-light` | inset top highlight + hairline |

## Interaction (`interaction`)

| Token | Value |
|---|---|
| `interaction.hoverLift` | `-3` |
| `interaction.pressScale` | `0.97` |
| `interaction.magneticMax` | `6` |
| `interaction.magneticFactor` | `0.22` |

## Color (`color`)

| Token | Value |
|---|---|
| `color.ink` | `#05070c` |
| `color.bg` | `#070b12` |
| `color.text` | `#f2f6fb` |
| `color.muted` | `#a3b6c9` |
| `color.accent` | `#5fd0d8` |
| `color.accentStrong` | `#8eecf0` |
| `color.accentWarm` | `#e8b87a` |
| `color.ok` | `#86efac` |
| `color.danger` | `#f87171` |

## Icon (`icon`) / spatial (`spatial`)

| Token | Value |
|---|---|
| `icon.sm/md/lg` | `14` / `16` / `20` |
| `icon.stroke` | `1.75` |
| `spatial.max` | `76rem` |
| `spatial.gutter` | `clamp(1.25rem, 3vw, 2rem)` |
| `spatial.dockH` | `5rem` |

## CSS-only extras used by experience

`--ws-control-h`, `--ws-focus-ring`, `--ws-caret`, `--color-border*`, `--color-surface*`, `--color-bg-elevated`, `--exp-*` aliases, `--intent-space`, `--intent-depth` (set inline by shell from intent profile).

---

# 4. CURRENT HOME IMPLEMENTATION

**File:** `app/src/components/HomeWorkspacePanel.tsx`  
**Styles:** `.spatial-frame`, `.place__identity`, `.ws-compose*`, `.attention-orbit`, `.trust-strip` in `App.css`

### Actual composition (no workspace) — what screenshots show

1. **Identity block** (`.place__identity`, centered): kicker `Workspace`, title `This is your Workspace`, pulse `Where your work lives.`
2. **Dominant object:** `WorkspaceObject` `home-create`, kind `quick-action`, slot `anchor`, state `expanded`, class `empty-invite` — “Begin / Create your Workspace / Create a workspace”.
3. **Supporting objects:** `EmptyStructure` inside an outer `.dash-grid.attention-orbit.attention-orbit--ghost` — **nested**: `EmptyStructure` itself renders another `.dash-grid.attention-orbit` with **five** `MomentCard` placeholders (`Latest moment` + four `Recent` ghosts). Opacity ~0.55 + `blur(0.4px)` via `.attention-orbit--ghost`.
4. **Layout containers:** `.ws-compose.ws-compose--empty` forces single-column grid areas `anchor / float / orbit`. In practice only anchor + nested orbit content render.
5. **Trust strip:** `RESTORE_LIMITS_SUMMARY` muted under compose.
6. **Shell overlays:** IPC error toast in menubar; `CommandSurface` may render landing commands under content; dock active on Home.

### Actual composition (workspace + ≥1 saved context)

1. **Identity:** workspace name as `h1.place__title`; pulse from relative time / count.
2. **Dominant:** `MomentCard` hero in `.ws-compose__anchor` (`variant="hero"`, `state="expanded"`).
3. **Supporting float column** (`.ws-compose__float` flex column, gap `--space-4`):
   - `IntentionObject` with handoff text
   - `.ws-compose__utilities` flex-wrap: Quick save + Continue
4. **Orbit:** `.ws-compose__orbit.dash-grid.attention-orbit` flex-wrap centered; compact `MomentCard`s with staggered `margin-top` via `.orbit-item--N`. **Not rendered when `density === "focus"`** (`width < 900`).
5. **Grid:** default `.ws-compose` = `1.55fr / 0.7fr` columns, areas `anchor float` / `orbit orbit`; gap `clamp(1rem, 2.4vw, 1.75rem)`.

### Responsive

| Viewport width | Density | Home behaviour |
|---|---|---|
| `< 900` | `focus` | Single column; orbit omitted |
| `900–1279` | `balanced` | Two-column compose + orbit |
| `≥ 1280` | `flow` | Same compose grid; content still capped by `--spatial-max: 76rem` so ultrawide leaves large side gutters |

---

# 5. CURRENT SAVE IMPLEMENTATION

**File:** `app/src/components/SaveContextPanel.tsx`

### Branch A — no workspace (screenshot state)

- Centered `.spatial-frame.spatial-frame--center.save-env`
- Single `WorkspaceSurface` `level=floating` `tone=hero` `padding=xl` `focus-card`
- Copy: kicker Save · title `Start your workspace` · `Then leave yourself a note.` · primary `Create a workspace`
- Write surface **not** mounted
- `CommandSurface` still shows Capture intent commands (`Back to Workspace` navigates; `Review what will be saved` is a no-op — see UX debt)

### Branch B — scope error

- Same centered card pattern: `Saving is unavailable` + error text

### Branch C — write (workspace present, step `naming`/`reviewing`)

- Section: `spatial-frame--center save-env save-env--write attention-field` with `data-writing`
- **Dominant:** `WorkspaceObject` `write-surface`, kind `intention`, slot `anchor`, state `expanded`, level `overlay`, `lit`, classes `focus-card write-card write-focus`
- Width: `min(48rem, 100%)` via `.save-env--write .write-card`
- Fields: ghost name input; textarea rows `11` (focus) / `9` (else); min-height `12rem`; font-size `1.22rem`
- Required trust string: `You write this. Workspace will not invent or rewrite it.`
- Review emerges **inside** the same object via `AnimatePresence` + `motion.div.write-review` (height auto), not a separate route
- Writing mode: `setWriting(true)` dims dock via intent `dockEmphasis: 0.34` and canvas brightness filter for `capture`

### Branch D — saved success

- Centered success `WorkspaceSurface` with inspect `<details>`, `RestoreLimitsNotice`, `Save another moment`

---

# 6. CURRENT CONTINUE IMPLEMENTATION

**File:** `app/src/components/ResumeContextPanel.tsx`  
**Preview body:** `ContinuePreviewBody` from `objects/ContinuePreviewObject.tsx` (wrapper `ContinuePreviewObject` unused)

### Branch A — no workspace (screenshot state)

- Centered `WorkspaceSurface` focus-card: Continue · `Open your workspace` · `Go to Home`
- Cinema / MomentCard tree **not** mounted

### Branch B — empty contexts

- Header `What were you doing?` + trust strip + `EmptyStructure`

### Branch C — contexts present

- Header + `RESTORE_LIMITS_SUMMARY`
- `.continue-cinema` column flex; stage is **CSS grid** `1.45fr / 0.7fr` (collapses to 1fr in focus density)
- Stage: featured `MomentCard` hero + `IntentionObject` (intention hidden when that moment’s preview is open)
- Preview: `expandContent={<ContinuePreviewBody …/>}` **inside** the MomentCard (not a sibling overlay panel)
- Others: `.attention-orbit.continue-recede` compact cards; when preview/selection active, parent gets `.is-dimmed` → orbit `filter: saturate(0.7) brightness(0.72); opacity: 0.55`
- Separate inspect/delete surfaces remain in the same file (additional steps beyond browse/preview)

---

# 7. CURRENT CHECK-IN IMPLEMENTATION

**File:** `app/src/components/PilotMeasurementPanel.tsx`

### Branch A — loading (screenshot state: IPC fail / no scope+snapshot yet)

- Centered card: Check-in · `One moment...` · `Loading...`
- Metrics / chapters **not** mounted
- `CommandSurface` still shows Reflect commands (e.g. `Continue a moment`)

### Branch B — consent

- `.checkin-chat` with soft bubble question, open `<details>` scope lists, consent CTA `I consent to local pilot measurement`
- Required copy includes not-saved / local pilot framing

### Branch C — active

- Centered header: `How’s the return feeling?` + summary including **“are not saved”**
- **Metrics row:** `.checkin-metrics` CSS grid `repeat(3, 1fr)` → 1 column in focus density; three `CheckInSummaryObject` (Baseline hero, Leave→resume, Median)
- **Narrative:** `.checkin-spatial-trail` horizontal chapter chips (Baseline / Return / Reflect / Week four); `AnimatePresence mode="wait"` swaps `.checkin-chat` forms with x-slide `spring.soft`
- Forms use `WorkspaceSurface` bubbles (`soft` prompt / `solid` answer) + `input-wide` controls
- IPC measurement behaviour unchanged; presentation-only redesign

---

# 8. CURRENT GUIDE IMPLEMENTATION

**File:** `app/src/components/PilotHelpPanel.tsx`

### Actual rendered layout (matches screenshots; does not need workspace)

1. **Header:** Guide · `How this pilot works`
2. **`.guide-experience` grid:** `1.4fr / 0.55fr` (1fr stacked in focus density)
3. **Stage:** `GuideStepObject` for `STEPS[active]` (Save / Continue / Check-in) + decorative `.guide-demo--{write|expand|flow}` with pulse spans (aria-hidden)
4. **Rail:** vertical chips; classes `is-active` / `is-done`
5. **Trust object:** `WorkspaceObject` `guide-trust` `.quote-pane` with ShieldCheck, approval copy including “You approve every restore plan”, meta = `RESTORE_LIMITS_SUMMARY`
6. Attention: scene `guide`; primary = active step id; ambient `card`
7. `CommandSurface`: Learn commands e.g. `Try Save` / `Try Continue`

Obsolete CSS still present for prior guide story UI: `.guide-story__*` in `App.css` (no JSX references).

---

# 9. SCREENSHOTS

Directory: `architecture/research/experience/screenshots/snapshot-2026-08-02/`

### Capture matrix

| View | Desktop 1440×900 | Ultrawide 2560×1080 | Narrow 1100×800 | Dark mode desktop |
|---|---|---|---|---|
| Home | `home-desktop-dark.png` | `home-ultrawide-dark.png` | `home-narrow-dark.png` | `home-desktop-darkmode.png` |
| Save | `save-desktop-dark.png` | `save-ultrawide-dark.png` | `save-narrow-dark.png` | `save-desktop-darkmode.png` |
| Continue | `resume-desktop-dark.png` | `resume-ultrawide-dark.png` | `resume-narrow-dark.png` | `resume-desktop-darkmode.png` |
| Check-in | `pilot-desktop-dark.png` | `pilot-ultrawide-dark.png` | `pilot-narrow-dark.png` | `pilot-desktop-darkmode.png` |
| Guide | `help-desktop-dark.png` | `help-ultrawide-dark.png` | `help-narrow-dark.png` | `help-desktop-darkmode.png` |

### Capture conditions (must not be mistaken for Tauri-populated UI)

- Ran against Vite only; `invoke` undefined → red toast on every frame
- `workspace === null` after failed bootstrap → Home empty create; Save create gate; Continue open-workspace gate; Check-in loading gate
- Guide is the only destination that shows its full designed content without IPC
- Ultrawide does **not** widen the compose field beyond `76rem`; content remains a centered column
- Narrow `1100px` is still `balanced` (≥900); true `focus` density requires `<900` (not in this matrix)
- Dark-mode files are visually identical to dark desktop (no alternate theme)

### Re-capture

```bash
# terminal 1
cd app && pnpm exec vite --host 127.0.0.1 --port 1420
# terminal 2 (requires playwright + Edge channel)
node architecture/research/experience/screenshots/snapshot-2026-08-02/capture.mjs
```

For populated Moment/Save/Check-in frames: capture inside Tauri (`pnpm dev` on Windows) with a real workspace and saved contexts.

---

# 10. DEAD CODE

## Unused components (not imported by App / pilot tree)

| Item | Path | Evidence |
|---|---|---|
| `CanvasShell` | `app/src/components/CanvasShell.tsx` | Tests forbid App import; superseded |
| `OperatorConsole` | `app/src/components/OperatorConsole.tsx` | Diagnostic only |
| `AssistantPanel` | `app/src/components/AssistantPanel.tsx` | Not mounted |
| `WorkspaceIntelligencePanel` | `app/src/components/WorkspaceIntelligencePanel.tsx` | Not mounted |
| `ContinuePreviewObject` component | `ContinuePreviewObject.tsx` export | Only `ContinuePreviewBody` imported |
| `ElevatedCard.tsx` barrel file | `app/src/components/ElevatedCard.tsx` | Duplicate of re-export already on `WorkspaceSurface` |

## Duplicate / superseded abstractions

| Issue | Detail |
|---|---|
| `elevated-card*` + `ws-surface*` dual classnames | `WorkspaceSurface` emits both; CSS maintains parallel rules |
| `.ws-canvas__*` vs `.ws-compose__*` | Older canvas grid classes remain in `App.css`; Home/Continue use `.ws-compose__*` |
| `.guide-story__*` CSS | Superseded by `.guide-experience__*`; no JSX |
| `dash-grid` + `attention-orbit` together | Orbit forces `display:flex !important` and nullifies grid columns — `dash-grid` class is misleading/dead on those nodes |
| Nested `EmptyStructure` inside Home empty orbit | Home wraps `EmptyStructure` in another orbit container → double structure |

## Obsolete / orphan libs for pilot chrome

- `app/src/lib/layoutPersistence.ts` (CanvasShell only)
- Explanation UI chain: `DisplayReasonList`, `RecommendationExplanationView`, `explanationResolver` (not in destination tree)

## Duplicate styling patterns

- Button systems: `.exp-btn` and `.ws-command__action.exp-btn` overlapping hover/focus rules in `App.css`
- Toast/banner: `.error.banner.ws-toast` vs intent command chrome both competing at edges
- Placeholders: `MomentCard` placeholder variant vs `EmptyStructure` both inventing “waiting” fields

---

# 11. UX DEBT

Ranked by impact on perceived product quality against concept boards / Participant #1 bar. References are concrete.

1. **IPC failure toast dominates every Vite/browser frame** — `App.tsx` bootstrap catch → menubar status; ruins first impression; blocks reading destinations.
2. **Home empty nests `EmptyStructure` inside another ghost orbit** — `HomeWorkspacePanel.tsx` lines wrapping `EmptyStructure` in `.dash-grid.attention-orbit--ghost` while `EmptyStructure` already owns that structure → stacked/tall ghost field, looks broken on ultrawide.
3. **`CommandSurface` actions that lack `view` are inert** — `CommandSurface.tsx` only handles `create` + `view`; Capture’s “Review what will be saved”, Restore’s “Approve and restore”, Reflect’s “Continue reflection” click with no effect.
4. **CommandSurface contradicts empty gates** — on Continue without workspace, Restore commands still offer “Save a moment” / “Approve and restore” while center card says open workspace (`ResumeContextPanel` + `intent.ts` restore profile).
5. **Check-in loading is a dead-end card** — `PilotMeasurementPanel` “One moment…/Loading…” with no retry; Reflect commands still clickable and misleading.
6. **Orbit is not spatial** — `.attention-orbit` is flex-wrap + fake stagger margins (`.orbit-item--N`), not free placement; reads as a wrapped chip row, not an orbit.
7. **Ultrawide wastes canvas** — `--spatial-max: 76rem` + centered `.ws-spatial` leaves huge empty side planes; density `flow` does not change composition.
8. **Save write experience unreachable without workspace + IPC** — screenshots only show create gate; writing-mode dock dim / review emergence unverified in capture set.
9. **Continue cinema unverified in capture set** — same; preview-in-Moment and dimming path not visible without data.
10. **Dual vocabulary Home vs Landing intent** — live region / CommandSurface say “What matters now” / CAPTURE while dock says Home/Save (`INTENT_LABELS` vs `PILOT_VIEW_LABELS`).
11. **Guide demo pulses are non-interactive decoration** — `.guide-demo` does not navigate or demonstrate real Save/Continue; “Try it” copy overpromises (`PilotHelpPanel.tsx`).
12. **Quick actions vs CommandSurface duplication on Home** — `QuickActionObject` Save/Continue plus landing `CommandSurface` commands compete for the same jobs.
13. **Focus density threshold untested in screenshot matrix** — narrow capture at 1100px still balanced; true focus layout (`<900`) unknown in evidence pack.
14. **Ghost placeholder copy leaks restore jargon** — `EmptyStructure` placeholders mention “Same Windows session restore” / “Still-open windows only” on Home empty — instructional, not atmospheric.
15. **Menubar status vs brand imbalance** — error pill top-right overpowers brand mark; no calm empty status.
16. **Dock magnetic + pill + breath stack** — visually busy relative to quiet concept boards; `DockItem` velocity springs + `.ws-dock__breath` perpetual motion.
17. **Check-in metrics always above narrative** — three orbs persist while answering; divides attention from chapter chat (`PilotMeasurementPanel` + `.checkin-metrics`).
18. **Write review expands inside already-tall card** — `write-review` height animation can push past viewport; no dedicated review stage.
19. **Trust strips repeated** — Home, Continue, Guide each show restore-limits prose; feels like compliance wallpaper rather than one trusted moment.
20. **MomentCard action chrome density** — hero cards expose continue/inspect/select affordances that compete with IntentionObject + QuickActions.
21. **Placeholder MomentCards use full card chrome** — borders/shadows on ghosts make empty Home feel like a dashboard of disabled tiles.
22. **No light theme / no theme toggle** — “dark mode” matrix is fake differentiation; product only has one look.
23. **Guide rail vs dock conceptual overlap** — rail steps Save/Continue/Check-in duplicate dock destinations without syncing dock selection.
24. **Atmosphere 6-glow stack** — `.ws-atmosphere__glow--a…f` can muddy midtones behind glass cards (visible in Guide screenshot orbs).
25. **Reduced-motion capture path used for matrix** — `capture.mjs` sets `reducedMotion: "reduce"` for viewport passes; motion quality of lush springs not evidenced in PNGs.

---

# 12. IMPLEMENTATION DEBT

Separate from visual/UX.

1. **View id ≠ label map** — `resume`/`pilot`/`help` vs Continue/Check-in/Guide scattered through App, shell, tests, CSS `data-destination`.
2. **Intent profile commands are half-wired** — `lib/intent.ts` defines actions `save|continue|checkin|…` but `CommandSurface` ignores them; no callbacks into panels.
3. **Attention + Intent both mutate visual weight** — `WorkspaceObject` multiplies attention weight by intent heuristics; hard to reason which system “wins”.
4. **Composition provider duplicates AttentionEngine fields** — `WorkspaceComposition` re-exposes scene/primary/secondary already on AttentionEngine.
5. **Writing mode set in three places** — shell focus capture, Save panel enter/leave, composition `setWritingMode`, intent `setWriting`.
6. **CSS dual material system** — `ws-surface*` and `elevated-card*` must stay in sync manually.
7. **Orphan CSS blocks** — `.guide-story__*`, `.ws-canvas__stage/anchor/float` largely unused by current JSX.
8. **Giant `App.css`** (~2300+ lines) owns layout, materials, destinations, a11y — no co-location.
9. **Legacy diagnostic components still in tree** — inflate bundle risk and confuse agents (even if tree-shaken).
10. **`ContinuePreviewObject` dead export** beside live `ContinuePreviewBody`.
11. **Home empty orbit double-wrap** — structural bug, not just visual.
12. **Screenshot/test gap** — no automated visual regression; pilot tests are string/import boundary checks only.
13. **Density hook is viewport-only** — comment says manual override reserved; intent `spacingScale` CSS var underused by destination layouts.
14. **Tauri-required for truthful QA** — experience acceptance currently unverifiable on Linux/Vite path.
15. **Error handling surfaces raw exception strings** — `Cannot read properties of undefined (reading 'invoke')` shown to humans.

---

# 13. NEXT SPRINT

**Do not implement in this document.**  
**Sprint name:** Experience Reality Alignment  
**Goal:** Remove dead structure, wire CommandSurface to real panel actions, fix Home empty composition, and produce Tauri-backed screenshots of populated Save/Continue/Check-in — without new capabilities or Product Proof chrome changes.

## Constraints (preserve)

- Keep `PILOT_PRIMARY_VIEWS` / dock labels Home · Save · Continue · Check-in · Guide
- Keep required pilot strings: `This is your Workspace`, `Create a workspace`, `You write this`, `are not saved`, `You approve every restore plan`, `dash-grid`, `EmptyStructure`, `MomentCard`, `Quick save`
- No new IPC capabilities, no AI, no ambient observation

## Work items (deterministic)

### A. Fix Home empty nesting

| | |
|---|---|
| **File** | `app/src/components/HomeWorkspacePanel.tsx` |
| **Change** | In `!workspace` branch, replace outer `<div className="dash-grid attention-orbit attention-orbit--ghost"><EmptyStructure /></div>` with bare `<EmptyStructure />` (or pass props). Remove the redundant wrapper. |
| **Also check** | Workspace-empty invite branch (no latest): same pattern at orbit — keep one orbit owner only. |
| **Why** | Eliminates double ghost stack seen in `home-*-dark.png`. |

### B. Soften EmptyStructure placeholders

| | |
|---|---|
| **File** | `app/src/components/EmptyStructure.tsx` |
| **Change** | Reduce placeholder count from 5 → 2 (Latest + Recent) **or** keep 5 but strip restore-jargon hints to atmospheric fragments. Prefer 2 for Home empty. |
| **CSS** | `app/src/App.css` `.attention-orbit--ghost` / `.moment-card` placeholder — lower opacity/border so ghosts are atmosphere, not disabled cards. |
| **Why** | Home empty currently reads as a dashboard of waiting tiles. |

### C. Wire CommandSurface actions

| | |
|---|---|
| **File** | `app/src/components/CommandSurface.tsx` |
| **Change** | Extend props with optional handlers: `onReviewSave?: () => void`, `onApproveRestore?: () => void`, `onAdvanceCheckin?: () => void`. In `onClick`, switch on `command.action` and call handlers; if handler missing, **do not render** that command. |
| **File** | `app/src/components/WorkspaceShell.tsx` |
| **Change** | Pass handlers through from App **or** remove inert commands from profiles until handlers exist. |
| **File** | `app/src/lib/intent.ts` |
| **Change** | For empty/no-workspace conditions, composition profiles must not advertise Approve/Review. Prefer filtering in `IntentEngine.tsx` when `empty` / `!workspace` signals exist — today `empty` affects landing profile only; extend similarly for restore/capture/reflect. |
| **File** | `app/src/App.tsx` / panels |
| **Change** | Only after handlers exist: Save panel exposes review trigger; Resume exposes approve; Check-in exposes chapter advance. If wiring is too coupled for one sprint, **delete inert commands from profiles** instead of leaving dead buttons. |
| **Why** | Dead glowing CTAs are high-severity trust damage. |

### D. Hide CommandSurface on hard gates

| | |
|---|---|
| **Files** | `CommandSurface.tsx`, `IntentEngine.tsx` |
| **Change** | When destination is loading gate / no-workspace gate, return `null` from CommandSurface **or** set `commands: []` for those states. |
| **Why** | Matches Continue/Save/Check-in screenshot gates. |

### E. Remove or quarantine dead components

| | |
|---|---|
| **Remove or move** | `CanvasShell.tsx`, `ElevatedCard.tsx` (keep alias on `WorkspaceSurface` only), unused `ContinuePreviewObject` export (keep `ContinuePreviewBody`). |
| **Quarantine** | `OperatorConsole`, `AssistantPanel`, `WorkspaceIntelligencePanel` — if deletion risks large domain-type fallout, move under `app/src/diagnostics/` and update boundary script allowlist. |
| **Tests** | Update `tests/pilot-chrome.test.ts` paths if moved. |
| **Why** | Stops next sprint from editing obsolete shells. |

### F. Delete obsolete CSS

| | |
|---|---|
| **File** | `app/src/App.css` |
| **Remove blocks** | `.guide-story__*` entire section; unused `.ws-canvas__stage`, `.ws-canvas__anchor`, `.ws-canvas__float`, `.ws-canvas__orbit` if grep shows zero JSX class usage after audit. |
| **Do not remove** | `.elevated-card*` until `WorkspaceSurface` stops emitting those classnames (separate cleanup). |
| **Why** | Prevents styling the wrong abstraction. |

### G. Dual classname cleanup (optional same sprint if time)

| | |
|---|---|
| **File** | `app/src/components/WorkspaceSurface.tsx` |
| **Change** | Stop emitting `elevated-card*` classes; keep `ws-surface*`. |
| **File** | `app/src/App.css` |
| **Change** | Migrate selectors from `.elevated-card__body` → `.ws-surface__body` (and related) in one pass; delete duplicate elevated rules. |
| **Why** | Single material system. |

### H. Orbit honesty

| | |
|---|---|
| **File** | `app/src/App.css` |
| **Change** | Rename conceptually: either implement true absolute/grid orbit positions for `.orbit-item--0…4` within a defined stage height, **or** rename classes/docs to `moment-row` and remove “orbit” claims from comments in `HomeWorkspacePanel` / `ResumeContextPanel`. |
| **Preferred this sprint** | Real positions: container `position: relative; min-height: 12rem`; items absolute percentages — still density-aware; hide in focus. |
| **Why** | Attention Engine sprint promised spatial orbit; flex-wrap is the current reality. |

### I. Ultrawide compose

| | |
|---|---|
| **File** | `app/src/design-system/tokens.ts` + `tokens.css` |
| **Change** | Raise `spatial.max` for `flow` only via CSS `[data-density="flow"] { --spatial-max: 92rem; }` in `App.css` (do not break focus). |
| **File** | `app/src/App.css` `.ws-compose` |
| **Change** | Under `[data-density="flow"]`, widen float column slightly (`0.85fr`) and increase orbit gap. |
| **Why** | Ultrawide screenshots show a phone-column of cards in a cinema screen. |

### J. Error toast hygiene

| | |
|---|---|
| **File** | `app/src/App.tsx` |
| **Change** | Map known IPC-missing errors to a calm single line (“Workspace needs the desktop app”) and suppress stack-like `Cannot read properties…` strings. |
| **File** | `app/src/App.css` |
| **Change** | Demote `.ws-toast` visual weight (smaller, no competing with brand). |
| **Why** | First pixel of every current screenshot is a failure. |

### K. Evidence pack (end of sprint)

| | |
|---|---|
| **Action** | Re-run capture **inside Tauri on Windows** with a workspace that has ≥2 saved contexts; include Save write + review, Continue preview expanded, Check-in consent + chapter 0. |
| **Update** | Replace or add files under `architecture/research/experience/screenshots/snapshot-2026-08-02/tauri/`. |
| **Update this doc §9** | Note which frames are Vite-empty vs Tauri-populated. |
| **Why** | Next sprint after this one must not assume populated cinema from empty gates. |

## Explicit non-goals for next sprint

- No new destinations / capabilities / contracts
- No light theme
- No AI guide demos that call models
- No deleting Product Proof measurement IPC
- No claiming Experience complete without Participant #1 re-review

## Definition of done

1. `pnpm typecheck` + `pnpm test` green  
2. Home empty screenshot shows single ghost structure, not nested stacks  
3. No CommandSurface button is a no-op  
4. Dead `CanvasShell` / unused preview wrapper addressed  
5. Tauri populated screenshots exist for Save write, Continue cinema, Check-in active  
6. This snapshot file updated only in §9/§10/§11 checkboxes if desired — or superseded by `23_…`

---

*End of snapshot.*
