# 40 — Experience Refoundation

Status: Active (Version 2 authority)  
Version: 2.0.0-sprint40  
Date: 2026-08-03  
Branch: `v2-dev`  
Supersedes for Experience presentation: Version 1 frozen chrome (`00`–`33` remain immutable history).  
Concept authority: Concept boards (Aug 1 2026) — sole visual target for this audit.  
Not authority: Version 1 screenshots, prior implementation passes, Participant opinion as product law.

---

## 1. Product philosophy

Version 1 proved the product can recover interrupted work with honest limits.  
Version 2 must feel like the place those concept boards promised — a living desktop companion, not a Product Proof utility.

- **Version 1 = Product Proof.** Trust, capture, restore, recovery: certified baseline.
- **Version 2 = Product.** Perception, place, motion, and material must surpass the concept boards — not converge on them as a ceiling.
- Any implementation may be replaced. No frozen Experience chrome. Assumptions from V1 are challenged, not inherited.

---

## 2. Design principles

1. **Place before page** — Destinations are rooms in one place, not document routes.
2. **One composition** — The first viewport is a single spatial stage, never a stacked dashboard of equal modules.
3. **Memory is visible** — Owned moments occupy space with varied weight; emptiness shows structure, never fake activity.
4. **Intention first** — Handoff and return lead; process/window metadata stays behind Inspect.
5. **Quiet chrome** — Dock, menubar, and commands float; they never compete with the stage.
6. **Honest atmosphere** — Depth, glass, and light serve attention — not spectacle, gauges, or AI theatre.
7. **Density without reading** — Icons, tiles, relative time, short handoff. Kill instructional paragraphs in the hero band.
8. **Surpass, don’t clone** — Concept boards are the floor for emotional and compositional ambition.

---

## 3. Experience goals

| Goal | Signal |
|---|---|
| Arrive and feel owned | Workspace name / place identity dominates first glance |
| Know what to do next | One primary Moment or write surface owns attention |
| Feel memory | Secondary moments recede in space, not as equal CRUD cards |
| Trust the calm | No wizard chips, compliance wallpaper, or engineering IDs in hero |
| Move without thinking | Icon-led dock; destination change is stage change, not page load |

---

## 4. Composition system

- **Stage** — Full-bleed atmosphere; content floats in a spatial frame.
- **Anchor** — One primary object (hero Moment, write surface, continue cinema, check-in story, guide step).
- **Satellites** — Uneven modular tiles with depth recession (scale, opacity, blur). Never a uniform 3-column admin grid.
- **Float** — Intention / quick action as secondary orbit, not a sidebar form.
- **Chrome** — Menubar brand mark + icon dock. Labels are accessible, not always painted.

Density breakpoints remain (`focus` / `balanced` / `flow`) but must change *composition*, not only padding.

---

## 5. Motion philosophy

- Motion reveals attention order: settle → elevate → dissolve.
- Springs are lush for place transitions; snappy only for chrome press.
- No perpetual breath on dock icons. Atmosphere drift may exist at near-threshold perception.
- Reduced-motion collapses to opacity/position fades without layout theatre.

---

## 6. Material language

- One lighting model: edge-light glass, shared blur recipes across surfaces.
- Hero may be larger, not brighter-glowing than neighbours.
- Accent cyan is action and focus — never neon bloom stacks.
- Background is atmospheric gradient + restrained glow fields, not flat `#000` panels.

---

## 7. Interaction language

- Primary action is always one obvious control on the anchor.
- Secondary actions are ghost / recessed.
- Hover lifts the object in z; it does not spawn toolbars.
- Inspect is opt-in, never a permanent hero affordance.

---

## 8. Navigation philosophy

- Five destinations: Home · Save · Continue · Check-in · Guide.
- Dock is icon-led; active state is a soft highlight, not a labelled tab strip.
- CommandSurface may offer a single contextual command — never duplicate the dock.

---

## 9. Visual anti-patterns (never allowed again)

1. Page-imperative titles that erase place identity (“Continue your work” as Home’s hero title).
2. Equal-weight card dashboards / admin grids.
3. Wizard chip trails in the hero band.
4. Compliance / restore-limits prose as permanent wallpaper on every destination.
5. Always-visible dock text labels competing with the stage.
6. Engineering IDs, scopes, PIDs as primary content.
7. Fake activity, AI chat rails, live desktop thumbnails, CPU gauges.
8. Multi-layer neon glow / spectacle lighting.
9. Nested empty structures (double ghost orbits).
10. Screenshots that could be mistaken for an internal business dashboard.

---

## 10. Acceptance criteria (Version 2 Experience)

A destination ships only when:

1. First viewport reads as **one composition** against concept board structure.
2. Place identity (brand / workspace name) outranks process copy.
3. Primary action is unambiguous within 1 second of glance.
4. Secondary content is spatially recessed, not equally bordered.
5. Materials share one lighting model.
6. Motion respects reduced-motion and does not perpetual-animate chrome.
7. No anti-pattern from §9 appears in evidence screenshots.
8. Parity matrix score for that destination improves on the changed dimensions, with evidence.

---

## 11. Concept-board parity audit (Sprint 40)

### Authority used

Concept boards (Aug 1 2026) as described and constrained by:

- Documented concept targets in recovery principles (living place, varied weight, density without reading, icon/tile memory).
- Named concept modes in prior parity language: **Minimal Immersive** (dock), **Modular Tiles** (Home field), **Focus** (single primary surface), **Continuity** (place-first Continue).
- Concept refusals: AI rails, gauges, fabricated activity, live thumbnails.

Implementation evidence reviewed (current build screenshots, not as targets):

- `architecture/research/experience/screenshots/convergence-pass-07/`
- `architecture/research/experience/screenshots/populated-2026-08-02/`

### Scoring

Each cell: **0–10** concept parity (10 = meets or surpasses concept board on that dimension).  
Severity of gaps: **S0** cosmetic · **S1** local · **S2** structural perception · **S3** first-impression / product identity.

### Quantified parity matrix (pre-change baseline)

| Dimension | Home | Save | Continue | Check-in | Guide | Concept target |
|---|---:|---:|---:|---:|---:|---|
| Visual hierarchy | 5 | 7 | 5 | 5 | 6 | 9 |
| Emotional tone | 5 | 7 | 5 | 6 | 6 | 9 |
| Spatial composition | 4 | 7 | 4 | 5 | 6 | 9 |
| Interaction model | 6 | 7 | 6 | 5 | 5 | 9 |
| Motion language | 5 | 6 | 5 | 5 | 5 | 8 |
| Material system | 6 | 7 | 6 | 6 | 6 | 9 |
| Typography rhythm | 5 | 8 | 5 | 6 | 7 | 9 |
| Information density | 5 | 8 | 4 | 5 | 6 | 9 |
| Attention flow | 5 | 8 | 5 | 5 | 6 | 9 |
| First-impression quality | 4 | 7 | 5 | 5 | 6 | 9 |
| **Destination mean** | **5.0** | **7.2** | **5.0** | **5.3** | **5.9** | **8.9** |

Overall baseline mean: **5.68 / 10**.

### Structural findings (only)

#### F-S40-01 — Home is still a page, not a place
- **Severity:** S3  
- **Confidence:** 0.92  
- **Dimension hits:** visual hierarchy, spatial composition, typography rhythm, first-impression, attention flow  
- **Concept:** Place identity (workspace / Atelier-class name) is the hero signal; work tiles inhabit the place.  
- **Current:** Populated Home leads with page imperative `Continue your work` — a route title, not a place.  
- **Affected files:** `app/src/components/HomeWorkspacePanel.tsx`, `app/src/App.css`  
- **Screenshots:** `convergence-pass-07/home-desktop.png`, `populated-2026-08-02/home-desktop.png`  
- **Proposed replacement:** Place-first identity — workspace name as display title, living pulse (owned count + relative time), hero Moment as stage centrepiece inside one `.home-place` composition.

#### F-S40-02 — Home satellites lack cinematic depth recession
- **Severity:** S2  
- **Confidence:** 0.84  
- **Concept:** Modular tiles with shallow depth-of-field; secondary memory feels behind the hero.  
- **Current:** Uneven column spans exist, but opacity/blur/scale recession is weak — field still reads as a card row under a page title.  
- **Affected files:** `app/src/App.css`, `HomeWorkspacePanel.tsx`  
- **Screenshots:** `convergence-pass-07/home-desktop.png`  
- **Proposed replacement:** Stronger satellite recession tied to attention weight; field belongs to the place stage, not a second page section.

#### F-S40-03 — Continue still stacks proof machinery in the hero band
- **Severity:** S2  
- **Confidence:** 0.88  
- **Concept:** Continuity is place-first cinema; restore honesty is secondary.  
- **Current:** Dense restore stats + limits prose compete with intention inside the primary card.  
- **Affected files:** `ResumeContextPanel.tsx`, `ContinuePreviewObject.tsx`, `RestoreLimitsNotice.tsx`, `App.css`  
- **Screenshots:** `populated-2026-08-02/continue-desktop.png`  
- **Proposed replacement:** Cinema stage with handoff + Approve; limits as progressive disclosure.

#### F-S40-04 — Check-in dual-module story/form
- **Severity:** S2  
- **Confidence:** 0.86  
- **Concept:** Focus — one primary surface; metrics recessed.  
- **Current:** Metrics orbs + story/form modules split attention; trail chrome historically reappears as wizard energy.  
- **Affected files:** `PilotMeasurementPanel.tsx`, `CheckInSummaryObject.tsx`, `App.css`  
- **Screenshots:** `populated-2026-08-02/checkin-desktop.png`, `convergence-pass-07/checkin-desktop.png`  
- **Proposed replacement:** Single narrative surface; metrics as soft periphery.

#### F-S40-05 — Guide duplicates destination invitation
- **Severity:** S2  
- **Confidence:** 0.80  
- **Concept:** Focus teaching surface; dock is navigation.  
- **Current:** CommandSurface “Try Save / Try Continue” duplicates dock while step already invites try.  
- **Affected files:** `PilotHelpPanel.tsx`, `CommandSurface.tsx`, `lib/intent.ts`  
- **Screenshots:** `populated-2026-08-02/guide-desktop.png`  
- **Proposed replacement:** One in-step CTA; silence CommandSurface on Guide.

#### F-S40-06 — Empty Home nests ghost structure
- **Severity:** S2  
- **Confidence:** 0.90  
- **Concept:** Honest emptiness — one ghost structure that shows the shape of memory.  
- **Current:** Create invite + `EmptyStructure` can stack as double structure (especially with orbit wrap on invite path).  
- **Affected files:** `HomeWorkspacePanel.tsx`, `EmptyStructure.tsx`  
- **Screenshots:** `snapshot-2026-08-02/home-desktop-dark.png`  
- **Proposed replacement:** Single empty composition path.

#### F-S40-07 — Trust-strip wallpaper
- **Severity:** S2  
- **Confidence:** 0.85  
- **Concept:** Trust is earned once, not printed on every wall.  
- **Current:** Restore-limits summary repeats across Home / Continue / Guide.  
- **Affected files:** panels using `RESTORE_LIMITS_SUMMARY`, `RestoreLimitsNotice.tsx`  
- **Screenshots:** Home/Continue/Guide populated set  
- **Proposed replacement:** One trusted moment (Continue approve or Guide step), not chrome wallpaper.

### Save note

Save scores highest (leave-a-note Focus). Remaining gaps are motion polish and writing-mode atmosphere — not structural S3. Not selected for Sprint 40’s first change.

---

## 12. Sprint 40 first change (selected)

**Change:** F-S40-01 (+ supporting depth from F-S40-02) — **Home place-first hub**.

**Why first:** Highest severity (S3) on the arrival destination; closes the core concept gap (page → place); measurable on hierarchy, composition, typography, attention, first impression.

**Out of scope this commit:** Continue cinema, Check-in focus merge, Guide command silence, trust-strip global removal.

**Validation:** Screenshots under `architecture/research/experience/screenshots/sprint-40/`; parity delta recorded in §13 after implementation.

---

## 13. Parity delta (after F-S40-01)

Evidence: `architecture/research/experience/screenshots/sprint-40/home-place-after.png`  
Validation: `architecture/research/experience/screenshots/sprint-40/place-first-validation.json`  
Before reference: `architecture/research/experience/screenshots/convergence-pass-07/home-desktop.png`

| Dimension | Home before | Home after | Δ |
|---|---:|---:|---:|
| Visual hierarchy | 5 | 7.5 | +2.5 |
| Emotional tone | 5 | 6.5 | +1.5 |
| Spatial composition | 4 | 6.5 | +2.5 |
| Interaction model | 6 | 6.5 | +0.5 |
| Motion language | 5 | 5.5 | +0.5 |
| Material system | 6 | 6.5 | +0.5 |
| Typography rhythm | 5 | 7.5 | +2.5 |
| Information density | 5 | 6.5 | +1.5 |
| Attention flow | 5 | 7.0 | +2.0 |
| First-impression quality | 4 | 7.5 | +3.5 |
| **Home mean (10 dims)** | **5.0** | **6.75** | **+1.75** |

Overall product mean: **5.68 → 6.03** (+0.35), driven by Home only.

Measured acceptance for this change:

- [x] Workspace name (`Atelier`) is the Home display title.
- [x] Page imperative `Continue your work` removed from Home identity.
- [x] Living pulse uses owned moment count + relative time.
- [x] `.home-place` wraps identity + stage as one composition.
- [x] Satellite field recession increased (opacity / saturate / scale).
