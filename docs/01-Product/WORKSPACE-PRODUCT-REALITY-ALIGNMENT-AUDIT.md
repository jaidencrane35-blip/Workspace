# Workspace Product Reality Alignment Audit

| Field | Value |
|-------|-------|
| **Status** | Audit only — **no implementation authorised** |
| **Date** | 2026-07-30 |
| **Trigger** | Human visual review of Milestone D |
| **Branch** | `cursor/product-reality-alignment-audit-34a5` |
| **Concept references** | [`references/workspace-concept-01.png`](references/workspace-concept-01.png), [`references/workspace-concept-02.png`](references/workspace-concept-02.png) |
| **Interpretation** | [WORKSPACE-REFERENCE-INTERPRETATION.md](WORKSPACE-REFERENCE-INTERPRETATION.md) |
| **Supersedes (product model)** | Parts of [WORKSPACE-PRODUCT-ALIGNMENT-AUDIT.md](WORKSPACE-PRODUCT-ALIGNMENT-AUDIT.md) and [WORKSPACE-PRODUCT-DELIVERY-ROADMAP.md](WORKSPACE-PRODUCT-DELIVERY-ROADMAP.md) that frame Workspace as a user-built management dashboard |
| **Related** | [WORKSPACE-VISUAL-DIRECTION.md](WORKSPACE-VISUAL-DIRECTION.md), [PRODUCT-VISION.md](PRODUCT-VISION.md) |
| **Audience** | Product, engineering, commercial review |

**Do not implement from this document until human approval.**  
**Do not begin Milestone F.**  
**Do not create new AI or new engines.**  
**Do not delete existing infrastructure.**

---

## 0. Corrected product definition (from human review)

> **Workspace is a spatial representation of the user's active desktop environment.**  
> The user does **not** build a workspace from empty UI components.  
> Workspace **observes, represents, organises, and controls** existing applications.

### What that means

| Correct | Incorrect |
|---------|-----------|
| Open Workspace → see your **real** running desktop reflected | Open Workspace → empty stage until you “create a workspace” |
| Apps already on the PC are the subject | User must register apps and design an environment first |
| Spatial stage mirrors / controls window reality | Tabbed management panels are the product |
| Named arrangements **remember** a live desktop state | Arrangements are optional extras after setup |
| Flow / Focus transform how the **same** live set is presented | Flow / Focus are chrome-density toggles on a blank board |
| Assistant comments on / helps with the desktop | Assistant (or Home briefings) define the product |

### One-line identity test

If removing the nav leaves a screen that could be “any SaaS workspace admin tool,” the product is wrong.  
If removing the nav still shows **your apps in space**, the product is right.

---

## 1. Executive verdict

Milestone D succeeded at **demoting AI** and **promoting Stage in the shell**.  
Human review still rejects the product because the Stage remains a **setup-gated management surface**, not a **desktop reality layer**.

```text
Concept art:     LIVE DESKTOP  →  represent / organise / control
Current UI:      EMPTY STATE   →  create workspace → register apps → maybe arrange later
```

The mismatch is **product model**, not polish density.

| Prior framing (A–D era) | Corrected framing (this audit) |
|-------------------------|--------------------------------|
| “Application-centric management product” | “Spatial desktop representation + control layer” |
| User organises apps *into* workspaces | Workspace organises apps *already present* |
| Stage shows registry tiles for a named environment | Stage shows observed / controllable desktop reality |
| Empty until create + register | Populated from observation; create/register are power tools |

**Milestone D is not accepted** as the target product identity. Keep useful shell gains; do not treat D as permission to start F.

---

## 2. UI mismatch analysis

### 2.1 Surfaces that incorrectly present a setup / dashboard tool

| Surface | What it communicates today | Why it mismatches the corrected model |
|---------|----------------------------|----------------------------------------|
| **Stage empty state** | “No active workspace. Create one under Workspaces…” → **Go to Workspaces** | Primary surface refuses to represent the desktop until the user invents a named container |
| **Home** | “Your applications, organised” + **Create workspace** as primary empty CTA | Orientation page for a management app; teaches “build first, work later” |
| **Workspaces panel** | “Saved work environments” / “Create a named workspace to organise…” | Makes **named environments** the product centrepiece instead of the live desktop |
| **Applications (no workspace)** | “Choose a workspace first” | Registry gated behind setup; observed runners are secondary |
| **Applications (empty registry)** | “Add the apps you use here” | Manual design of membership before representation |
| **Arrangements (no workspace)** | “Create a workspace first” | Memory/restore of real windows blocked by admin entity creation |
| **Companion canvas** | Zones / “spatial canvas” / board practice under Stage | Easy to misread as the workspace; still a design board, not OS reality |
| **Product copy cluster** | “organise applications,” “belong to each environment,” register → stage | Language of CRM/admin tools, not of a desktop control layer |
| **Preview banner** | “create, switch, launch, and restore” | Verb set emphasises admin verbs before observe/represent |

### 2.2 What the concept references show (and the UI does not)

From `workspace-concept-01.png` / visual direction:

| Concept signal | Current Stage reality |
|----------------|----------------------|
| Multiple **live** application windows as the stage | Empty void or registry monogram cards |
| Flow ↔ Focus changes **layout of the same apps** | Chrome density only; honest “OS windows unchanged” |
| Audio mixer / system facts as utilities around apps | Absent or demoted; not framing the stage |
| Assistant as right rail commenting on an already-full stage | Rail correct as companion; content still asks for workspace selection |
| Bottom product nav around workspace use | Top tab shell still reads as multi-panel admin |

### 2.3 Partial alignments (keep the intent, change the centre)

| Element | Partial win | Remaining miss |
|---------|-------------|----------------|
| Stage-first default (Milestone D) | Correct **slot** for the product | Slot is empty / setup-gated |
| Nav order Stage → Apps → Workspaces → Home | Stage leads | Leading view still sends users to Workspaces |
| Assistant companion rail | Correct hierarchy | Does not sit beside a live desktop map |
| Honest “canvas ≠ OS” / “Focus ≠ move windows” copy | Truthful | Honesty without live representation still feels unfinished |
| Applications “Running on the desktop” section | Observation exists in UI | Secondary list, not spatial stage |
| Desktop Arrangement rail | Real capture/restore path | Buried under management framing + workspace gate |

---

## 3. Systems that already support the correct direction

**Do not replace these.** Product work should **surface and wire** them.

| Capability | Existing system | Product role under corrected model |
|------------|-----------------|-------------------------------------|
| **Observe** | `windows-integration` capture / identity; `WorkspaceObservationService`; `get_workspace_state` | Feed the Stage with real windows/monitors |
| **Control** | `WindowController` (`set_bounds`, `focus`); CommandPipeline → Permission Gateway | Execute organise/control from Stage / modes / arrangements |
| **Remember** | `DesktopArrangement` persist + capture/restore IPC (DAF-1c–1e) | Save/restore **observed** desktop states — not invent fake boards |
| **Launch** | Application registry + `launch_application` + permissions | Supplement observation (start something missing), not define the stage alone |
| **Organise (entity)** | Workspace records, switcher, settings active id | **Demote** to labels/profiles over reality — optional, not a gate |
| **Companion practice** | Canvas zones / layout board | Keep as optional practice surface; never primary reality |
| **Modes chrome** | Flow / Focus presentation (Milestone B) | Keep chrome; Milestone G still applies **real** geometries |
| **Assistant** | Companion rail + read/compose projections | Keep secondary; bind suggestions to observed desktop later — **no new engines** |
| **Governance** | Permission Gateway, audit, IPC contracts | Non-negotiable for all control actions |

### What is *not* missing as infrastructure

- A new “desktop engine”
- A new AI layer
- Deletion of Programme / projection code
- Replacing WindowController or DesktopArrangement

What *is* missing is a **product surface that treats observation as the default Stage content**.

---

## 4. What should remain

Keep and reuse:

1. **Shell hierarchy wins** — Stage-first nav; Assistant as companion; Diagnostics/Developer as tools.  
2. **DAF window stack** — observation, WindowController, arrangements, gateway.  
3. **Desktop Arrangement capture/restore pathway** — core “remember my desktop.”  
4. **Flow / Focus chrome** — mode switch affordance (behaviour still Milestone G).  
5. **Application registry + launch** — as enrichment and recovery, not as Stage sole source.  
6. **Honest empty/runtime messaging** — keep honesty; change *what* is empty (no windows observed vs no named workspace).  
7. **Companion canvas** — retained, clearly optional, never confused with OS tiling.

---

## 5. What should be demoted

| Demote | From | To |
|--------|------|----|
| **Create workspace** CTAs on Stage/Home | Primary product path | Secondary / settings-like (“name this desktop profile”) |
| **Named Workspaces panel** | Primary identity | Profile / switcher utility over live desktop |
| **Manual registry-first Stage** | Hero content | Fallback when observation is empty; “pin / favourites” later |
| **Home as product briefing** | Default teaching surface | Optional orientation; not required to see the desktop |
| **Companion canvas** | Near-peer to Stage | Explicitly tertiary practice board |
| **Admin verb cluster** (create, register, choose workspace first) | Default copy | Power-user / recovery copy |
| **Milestone D “apps as registry tiles” identity** | Accepted stage model | Interim shell only until Reality Stage |

**Do not delete** workspace entities, registry, or canvas — demote in **IA, default paths, and copy**.

---

## 6. Revised milestone recommendation

### 6.1 Status of prior plan

| Milestone | Prior plan | Status after this audit |
|-----------|------------|-------------------------|
| **D — Workspace Stage** | Ship apps-first shell | **Not accepted** as product identity; retain shell hierarchy only |
| **F — Arrangement editing** | Next after D | **Do not start** until Reality Stage approved |
| **G — Flow/Focus behaviour** | After F | Still correct *capability*, but depends on live stage + arrangements |
| **E / H / I** | Later | Unchanged in spirit; reorder after Reality Stage |

### 6.2 Insert next milestone: **R — Desktop Reality Stage**

**Purpose:** Make the primary Stage a **spatial representation of the user’s active desktop**, fed by existing observation, with organise/control actions routed through existing WindowController + Permission Gateway.

**User value:**  
“I open Workspace and see my real applications. I can organise and control them. I did not have to design an empty workspace first.”

**In scope (product surface only — reuse systems):**

- Stage default content = **observed windows** (and monitors), not “create workspace” void  
- Clear spatial/layout presentation of the active set (even if early version is a structured map rather than live thumbnails)  
- Workspace entity **optional** for naming/switching profiles — not a gate for seeing reality  
- Registry / launch as supporting actions  
- Arrangements rail remains, framed as **remember this desktop**  
- Assistant stays companion  
- No new engines; no new AI; no OS grouping yet (E); no audio yet (H)

**Out of scope:**

- Pixel-perfect concept art wallpaper / brand clones  
- Nine layout products  
- Autonomous layout loading without permission  
- Milestone F full arrangement editor polish (can follow once Stage shows reality)  
- Flow/Focus OS apply (still G) once there is a real set to transform

### 6.3 Revised recommended ship order

```text
R  Desktop Reality Stage     ← NEXT (pending approval)
F  Desktop Arrangements      ← remember/edit live desktop states
G  Flow / Focus Behaviour    ← apply real geometries to the same set
E  Desktop Grouping
H  Audio Mixer
I  Workspace Polish
```

```text
                    ┌──────────────────────────┐
                    │ R Desktop Reality Stage  │
                    └────────────┬─────────────┘
                                 │
                    ┌────────────▼─────────────┐
                    │ F Arrangement editing    │
                    └────────────┬─────────────┘
                                 │
                    ┌────────────▼─────────────┐
                    │ G Flow/Focus apply       │
                    └────────────┬─────────────┘
                                 │
                    ┌────────────▼─────────────┐
                    │ E → H → I                │
                    └──────────────────────────┘
```

**Why R before F:** Editing and restoring arrangements is meaningless as the *centre* of the product if the Stage still teaches users to invent environments. Representation must precede arrangement craftsmanship.

**Why F before G (unchanged logic):** Mode switch should apply **named real geometries** already capturable via arrangements.

---

## 7. Migration path (current UI → reference-image direction)

Phased **product** migration. Each phase reuses infrastructure; none invent parallel engines.

### Phase R0 — Framing (can ship with R)

1. Rewrite Stage/Home empty and lede copy: **observe first**, create/register secondary.  
2. Stop routing the default empty Stage exclusively to **Go to Workspaces**.  
3. Relabel Workspaces as profiles / saved environments **over** the desktop.  
4. Keep Assistant companion; bind empty intel to “no windows observed” when applicable.

### Phase R1 — Reality on Stage (Milestone R core)

1. Drive Stage primary content from **observation / workspace state** IPC already present.  
2. Present active applications as the Stage hero (map, tiles tied to HWND identity, or honest placeholders — **no fake live pixels** if capture thumbnails are unavailable).  
3. Expose organise/control entry points that call **existing** permissioned window APIs.  
4. Keep registry list as “Library / pinned,” not the only Stage.  
5. Windows human verification required for observation/control claims.

### Phase R2 — Memory (Milestone F, reframed)

1. Arrangement editing UX against **captured real windows**.  
2. Save/restore remains gateway-governed.  
3. UI title/copy: “Remember this desktop,” not “configure arrangement after setup.”

### Phase R3 — Transform (Milestone G)

1. Flow / Focus apply arrangement geometries to the **same** observed set.  
2. Apps stay open; chrome density + OS layout move together.

### Phase R4 — Power + polish (E, H, I)

1. Grouping, audio, commercial finish — only after the Stage is recognisably a desktop layer.

### Explicit non-migrations

| Do not | Why |
|--------|-----|
| Delete workspace tables / switcher | Still useful as profiles |
| Delete companion canvas | Optional practice; demote only |
| Delete Assistant projections | Companion infrastructure; freeze expansion |
| Build a second window stack | WindowController + observation already exist |
| Resume Milestone F from D acceptance | D identity rejected |

---

## 8. Documentation / vision follow-ups (approval-gated)

After this audit is approved, update (separate doc commits; still no product code until R is authorised):

| Document | Change |
|----------|--------|
| [PRODUCT-VISION.md](PRODUCT-VISION.md) | Primary surface = spatial desktop representation/control, not “management dashboard” |
| [WORKSPACE-VISUAL-DIRECTION.md](WORKSPACE-VISUAL-DIRECTION.md) | Clarify “management” means control of real apps, not admin setup |
| [WORKSPACE-PRODUCT-DELIVERY-ROADMAP.md](WORKSPACE-PRODUCT-DELIVERY-ROADMAP.md) | Insert Milestone R; mark D not accepted; F blocked on R |
| [FLOW-FOCUS-MODE-DESIGN-CHARTER.md](FLOW-FOCUS-MODE-DESIGN-CHARTER.md) | Modes transform a live represented set |
| Milestone D completion / review docs | Record: hierarchy kept; product identity rejected pending R |

---

## 9. Completion checklist for this audit

- [x] Corrected product definition recorded  
- [x] UI surfaces identified as setup/dashboard mismatches  
- [x] Existing systems mapped to observe / represent / organise / control  
- [x] Remain vs demote lists  
- [x] Next milestone **R — Desktop Reality Stage** recommended  
- [x] Migration path to concept references  
- [ ] Human approval of audit + authorisation of Milestone R  
- [ ] Implementation (explicitly **not** started)

---

## 10. Stop

Documentation complete.  
**Await human review of this audit.**  
Do **not** implement Milestone R, F, or any later milestone from this file until approved.
