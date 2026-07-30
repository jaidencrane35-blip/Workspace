# Workspace Interaction Model Audit

| Field | Value |
|-------|-------|
| **Status** | Audit only — **no implementation authorised** |
| **Date** | 2026-07-30 |
| **Branch** | `cursor/workspace-interaction-model-audit-34a5` |
| **Contract** | [WORKSPACE-DESKTOP-INTERACTION-MODEL.md](WORKSPACE-DESKTOP-INTERACTION-MODEL.md) (**approved**) |
| **Code baseline** | Current product shell including Milestone R Slice 1 (observation-backed Stage) |
| **Audience** | Product, engineering, human reviewers |

**Do not implement from this document until a slice is explicitly approved.**  
**Do not treat this audit as permission to rewrite the shell.**  
**Reuse observation, WindowController, DesktopArrangement, Permission Gateway, and existing IPC.**

---

## 0. How to read this audit

The approved [Desktop Interaction Model](WORKSPACE-DESKTOP-INTERACTION-MODEL.md) is the product contract. This audit measures the **current UI** against that contract.

**Important baseline note:** Milestone R Slice 1 already landed Stage observation wiring and removed the create-workspace void from Stage landing. This audit scores that reality. Remaining work closes the gap between “Stage can show desktop” and “the whole product *feels* like a desktop environment.”

**Alignment score:** 0 = opposite of contract · 10 = fully meets contract for that surface’s role.

---

## 1. Surface audits

### 1.1 Stage

| Field | Assessment |
|-------|------------|
| **Paths** | `WorkspaceApplicationStage.tsx`, `stageDesktopUi.ts`, `layoutsStageUi.ts`, `App.tsx` (`view === "layouts"`) |
| **Current purpose** | Default landing. Fetches `get_workspace_state`, shows a spatial map of observed windows when available, otherwise honest empty/runtime copy. Optional library section below. |
| **Intended purpose** | Primary **workspace surface**: desktop reality → applications → spatial organisation. First five seconds should feel like “my desktop.” |
| **Alignment score** | **7 / 10** |

**What is correct**

- Default landing is Stage, not Home or Assistant.  
- No create-workspace / register gate to *see* Stage.  
- Eyebrow/title: “Desktop reality” / “Your desktop.”  
- Spatial map from real window bounds when observation returns windows.  
- Honest empty states (runtime unavailable / no windows) — does not invent apps.  
- Library demoted under the map.  
- Flow/Focus change Stage presentation of the observed set.

**What is incorrect**

- When observation is empty or unavailable, the hero is still **text-heavy** (paragraphs + toolbar), not a calm empty *spatial* surface.  
- Contract wants “layout before text”; empty Stage still reads as explanation.  
- Secondary library and arrangement rail can still pull attention when the map is empty.  
- Companion canvas (when a profile exists) remains a large secondary board that can confuse “practice” with desktop.

**Why it diverged**

- Product evolved from management panels; R Slice 1 fixed the data source and gate, not the empty-state visual language or density of surrounding chrome.  
- Browser preview has no windows, so reviewers mostly see the text path — which over-indexes on reading.

**Smallest change toward contract**

- Redesign Stage empty/runtime as a **quiet spatial frame** (empty desktop plane + one short line), move secondary library/arrangements visually quieter when no windows.  
- No new engines — presentation only on existing load states.

**Human-review impact**

- High. First glance decides whether Workspace feels like a desktop or a help page.

**Maintainability impact**

- Low risk if limited to Stage empty layout + CSS; keep `stageDesktopUi` as the single empty-copy owner.

---

### 1.2 Home

| Field | Assessment |
|-------|------------|
| **Paths** | `WorkspaceHome.tsx` |
| **Current purpose** | Orientation hub: explain product, deep-link to Stage/Applications/Workspaces, optional profile create. |
| **Intended purpose** | Not the product centre. If present, must not teach setup-first or dashboard mental models. Stage remains the environment. |
| **Alignment score** | **5 / 10** |

**What is correct**

- Copy shifted toward “Your desktop, represented.”  
- Primary CTA: “Open desktop stage.”  
- Create profile is ghost/optional, not the only path.  
- Not the default landing.

**What is incorrect**

- Still a **text-and-card dashboard**: lede paragraphs, “Go to” cards, “Not available yet” feature list.  
- Competes with Stage as a second “explanation of Workspace.”  
- Profile section and roadmap list reintroduce configuration/roadmap-admin feel.

**Why it diverged**

- Home was built as onboarding for a management product; R Slice 1 updated slogans without changing composition.

**Smallest change toward contract**

- Collapse Home to a thin orientation: Stage CTA + one short line; remove or bury “Not available yet” and multi-card “Go to” grid.  
- Or demote Home further in nav prominence (keep reachable, not parallel hero).

**Human-review impact**

- Medium. Reviewers who open Home judge the whole product as a dashboard.

**Maintainability impact**

- Low — mostly delete/simplify JSX and copy; no IPC changes.

---

### 1.3 Applications

| Field | Assessment |
|-------|------------|
| **Paths** | `ApplicationsPanel.tsx`, `applicationsUi.ts`, `ActiveApplicationsView.tsx` |
| **Current purpose** | Optional library (register/launch) + “Running on the desktop” process list. |
| **Intended purpose** | Applications are the **subject of the product**, primarily visible on Stage. This tab may host library/detail, not become the hero catalogue. Observed apps must not be trapped behind setup. |
| **Alignment score** | **6 / 10** |

**What is correct**

- Lede says Stage shows observed desktop first; this page is optional library.  
- “Running on the desktop” works without a named profile.  
- Register form is not forced open.

**What is incorrect**

- Without a profile, library empty state still dominates the top of the page (management framing).  
- Observed section is a **flat list**, not spatial — acceptable for this tab, but the page still feels like an app manager.  
- “Save to workspace” / library-first structure can re-teach registration as the job.

**Why it diverged**

- Built as registry + launch surface before Stage became the reality layer; observation was bolted on below.

**Smallest change toward contract**

- Put **Running on the desktop** above library when no profile; shrink empty-library block to one line + link to Stage.  
- Rename remaining “workspace assets” language to library consistently (partially done).

**Human-review impact**

- Medium. Confirms whether apps are managed or lived-with.

**Maintainability impact**

- Low — reorder sections + copy; reuse `ActiveApplicationsView` / `get_workspace_state`.

---

### 1.4 Workspaces

| Field | Assessment |
|-------|------------|
| **Paths** | `WorkspaceSwitcher.tsx`, `workspaceSwitcherUi.ts` |
| **Current purpose** | Create / list / activate named environments (profiles). |
| **Intended purpose** | Optional **labels over reality** — not admission to the product. Controls layer, not desktop reality. |
| **Alignment score** | **3 / 10** |

**What is correct**

- Does not block Stage landing anymore.  
- Useful for arrangements persistence that still keys off workspace id.

**What is incorrect**

- Still framed as **“Saved work environments”** with create form always prominent.  
- Empty copy: “Create a named workspace to organise applications and desktop arrangements” — classic setup-wizard language.  
- Visually a management admin screen; easy to mistake for the product’s centre if users navigate here early.

**Why it diverged**

- Named workspace entity was the original product centrepiece; demotion in IA was incomplete after the contract change.

**Smallest change toward contract**

- Reframe copy to “Named profiles (optional)” and demote create form below the list; empty state: “Profiles are optional — open Stage to see your desktop.”  
- Keep create available; stop making it the story.

**Human-review impact**

- High for anyone who clicks Workspaces during review — currently reintroduces setup mental model.

**Maintainability impact**

- Low — copy + layout order only; persistence model unchanged.

---

### 1.5 Layouts

| Field | Assessment |
|-------|------------|
| **Paths** | Same as Stage (`layouts` view id; tab label “Stage”) |
| **Current purpose** | Alias of Stage in chrome; historical name remains in some diagnostics/advanced copy. |
| **Intended purpose** | One primary environment surface — naming should not imply a separate “layouts admin” product. |
| **Alignment score** | **8 / 10** |

**What is correct**

- Single surface; Stage label in primary nav.  
- Spatial organisation lives here.

**What is incorrect**

- Internal id `layouts` and leftover “Layouts” strings in Assistant advanced copy / diagnostics can confuse engineers and power users.  
- Companion canvas under Stage still looks like a second “layout product.”

**Why it diverged**

- Rename was chrome-level; companion canvas and old strings lag.

**Smallest change toward contract**

- Sweep user-visible “Layouts” → “Stage” where it means the environment; keep canvas clearly labelled practice-only and visually smaller.

**Human-review impact**

- Low–medium (naming clarity).

**Maintainability impact**

- Low — string sweep; avoid renaming IPC without need.

---

### 1.6 Assistant companion

| Field | Assessment |
|-------|------------|
| **Paths** | `AssistantCompanionRail.tsx`, `AssistantIntelligencePanel.tsx`, `assistantRail.ts` |
| **Current purpose** | Persistent right rail; ask/explain; advanced workflow collapsed. |
| **Intended purpose** | Optional companion. May answer/explain/navigate/suggest. Must never replace workspace, invent reality, or require interaction before work. |
| **Alignment score** | **6 / 10** |

**What is correct**

- Side rail, not landing tab.  
- Copy: “Supporting help… stay primary.”  
- Hideable; Stage remains without it.  
- Advanced workflow secondary.

**What is incorrect**

- Default **open** (`DEFAULT_ASSISTANT_RAIL_OPEN = true`) — competes visually with empty Stage in first five seconds.  
- Empty state still: “No workspace selected… create or select a workspace” — setup language and invents a gate for help.  
- On a text-heavy empty Stage, the rail’s vertical mass can feel like a co-primary column.

**Why it diverged**

- Milestone C made the rail persistent and useful; default-open optimised for discoverability over “optional start.”

**Smallest change toward contract**

- Default rail **closed** for first-run / or auto-collapse when Stage has no observed windows; rewrite empty intel to desktop-context language (“Ask about your desktop when you’re ready”) without create-workspace CTA.  
- No new AI capability.

**Human-review impact**

- High. Reviewers often read “AI product” from an always-open companion beside an empty stage.

**Maintainability impact**

- Low — preference default + copy; keep rail component.

---

### 1.7 Navigation

| Field | Assessment |
|-------|------------|
| **Paths** | `App.tsx` chrome tabs |
| **Current purpose** | Primary: Stage → Applications → Workspaces → Home. Tools: Assistant, Diagnostics, Developer. Flow/Focus in chrome. |
| **Intended purpose** | Calm access to core areas without equalising admin tabs with the environment surface. |
| **Alignment score** | **7 / 10** |

**What is correct**

- Stage first; default `layouts`.  
- Tools separated from product tabs.  
- Mode control always reachable.

**What is incorrect**

- Four primary tabs still look like a **peer dashboard** (Home/Workspaces compete with Stage for identity).  
- Duplicate Flow/Focus controls (chrome + Stage header) add chrome noise.  
- Brand subtitle “Desktop workspace environment” is fine; overall bar still reads “app with many sections.”

**Why it diverged**

- Incremental tab growth from management IA; Stage-first order fixed sequence but not visual weight.

**Smallest change toward contract**

- Visually emphasise Stage as the environment; demote Home/Workspaces styling (or move Home behind a quieter entry).  
- Keep one Flow/Focus control locus (chrome *or* Stage, not both equally loud).

**Human-review impact**

- Medium–high (first chrome impression).

**Maintainability impact**

- Low–medium CSS/IA; avoid router rewrite.

---

### 1.8 Flow mode

| Field | Assessment |
|-------|------------|
| **Paths** | `workMode.ts`, `WorkModeSwitch.tsx`, Stage Flow branch |
| **Current purpose** | Chrome-density preference; Stage shows full spatial map of observed windows; canvas available with profile. |
| **Intended purpose** | Productive density of the **same** working set — many apps visible, environment busy in a useful way. |
| **Alignment score** | **6 / 10** |

**What is correct**

- User-controlled; apps not closed.  
- Honest: does not move OS windows yet.  
- Spatial map in Flow when windows exist — aligns with density + space.

**What is incorrect**

- Without windows, Flow is not a dense desktop — it’s empty text.  
- Companion canvas can steal “layout” meaning from real desktop organisation.  
- Mode does not yet change real desktop geometry (expected later; must not be faked).

**Why it diverged**

- Mode chrome shipped before reality stage; geometry apply is a later milestone by design.

**Smallest change toward contract**

- Strengthen Flow empty state as an open desktop plane; keep honesty about OS apply.  
- Do not implement geometry apply in a “small polish” slice — that is a separate behaviour milestone.

**Human-review impact**

- Medium (with windows: good; without: weak).

**Maintainability impact**

- Low for presentation; high if scope creeps into OS apply — **out of scope** for early slices.

---

### 1.9 Focus mode

| Field | Assessment |
|-------|------------|
| **Paths** | Same mode stack; Stage Focus branch |
| **Current purpose** | Emphasises focused observed window + supporting chips; hides companion canvas. |
| **Intended purpose** | Quieter density of the same set; one primary emphasis; others remain available. |
| **Alignment score** | **6 / 10** |

**What is correct**

- Emphasises focused observed window when present.  
- Supporting windows remain listed.  
- Does not quit apps; canvas hidden to reduce noise.  
- Honest about no OS move yet.

**What is incorrect**

- Focus empty state still text-led.  
- Library “Focus primary” section can confuse observed focus with registry focus.  
- Visual calm is incomplete while Assistant rail stays fully open by default.

**Why it diverged**

- Focus was first implemented for registry tiles; R Slice 1 remapped observation but kept dual (desktop + library) Focus UIs.

**Smallest change toward contract**

- In Focus, hide or further collapse library secondary; let observed focus be the only hero.  
- Pair with Assistant default-closed for calm.

**Human-review impact**

- Medium–high for “does Focus feel immersive?”

**Maintainability impact**

- Low — conditional render / CSS density.

---

### 1.10 Related control surface: Desktop Arrangements (on Stage / Workspaces)

Not a primary nav tab, but contract-critical (**workspace controls** layer).

| Field | Assessment |
|-------|------------|
| **Paths** | `DesktopArrangementPanel.tsx`, `desktopArrangementUi.ts` |
| **Current purpose** | Save/restore real window layouts; rail beside Stage and Workspaces. |
| **Intended purpose** | Remember/restore the live desktop — controls serving reality. |
| **Alignment score** | **4 / 10** |

**What is correct**

- Real capture/restore path via existing systems.  
- Present near Stage (good placement for controls).

**What is incorrect**

- Empty gate: **“Create a workspace first.”** — configuration-first.  
- Title framing: “Your workspace can remember your setup” — management tone.  
- Can dominate the right column next to an empty Stage (with Assistant also open → triple column of non-desktop).

**Smallest change toward contract**

- Allow viewing “remember this desktop” framing without create CTA as hero; keep persist action honest if profile id required (“Name a profile to save” as secondary).  
- Soften visual weight when Stage has no windows.

---

## 2. Cross-surface findings

| Pattern | Where it shows up | Contract violation |
|---------|-------------------|--------------------|
| **Dashboard behaviour** | Home “Go to” cards + feature list; equal-weight primary tabs | Tool-about-tool instead of desktop-as-subject |
| **Configuration-first workflows** | Workspaces create form; Arrangements “Create a workspace first”; Assistant “select a workspace” | Setup before work |
| **Excessive explanatory text** | Stage empty/runtime; Home lede; multiple muted notes on Stage | Layout-before-text fails; feels like documentation |
| **Hidden spatial hierarchy** | Empty Stage has no plane; Applications list not spatial (OK for tab); canvas can fake “space” | Space should mean desktop, not zone board |
| **Applications not always the hero** | Empty Stage (text); Applications tab library-first without profile; Home cards | Apps must occupy attention when anything is shown |
| **Assistant visually competing** | Default-open rail beside empty Stage + arrangements rail | Companion must not co-own first glance |
| **Unnecessary management UI** | Always-visible create workspace; roadmap “Not available yet”; ghost example cards (reduced but culture remains) | Management ≠ product |
| **Duplicate controls** | Flow/Focus in chrome and Stage | Noise without new value |

**Architecture note:** Divergence is mostly **presentation, defaults, and copy**. Observation, arrangements, and window control systems are reusable and largely correct. Do not invent new engines to fix these findings.

---

## 3. Prioritisation

Ranked by **product value** (visual contract alignment) × **low architectural risk** × **reuse**.

| Rank | Work theme | Visual gain | Arch risk | Reuse |
|------|------------|-------------|-----------|-------|
| 1 | Stage empty/runtime as spatial plane + quieter secondary chrome | Very high | Low | Existing load states |
| 2 | Assistant default-closed + desktop-context empty copy | High | Low | `assistantRail` prefs |
| 3 | Arrangements gate/copy demotion (“remember desktop”) | High | Low | Existing panel/IPC |
| 4 | Workspaces → optional profiles framing | High | Low | Switcher only |
| 5 | Home simplification (anti-dashboard) | Medium–high | Low | Home only |
| 6 | Applications: observed-first ordering | Medium | Low | Existing list/IPC |
| 7 | Nav weight / single mode control locus | Medium | Low | CSS/chrome |
| 8 | Focus: observed-only hero (hide library noise) | Medium | Low | Stage conditionals |
| — | Flow/Focus **OS geometry apply** | High (later) | Higher | Arrangements + WindowController — **not** an early polish slice |
| — | Grouping / audio | Later roadmap | Higher | Out of band |

---

## 4. Recommended implementation order (small slices)

Each slice independently reviewable. No large rewrites. No new engines. No duplicate systems.

| Slice | Name | Goal | Primary files | Done when (human) |
|-------|------|------|---------------|-------------------|
| **IM-1** | Stage empty spatial calm | Empty/runtime Stage looks like a quiet desktop plane; minimal text; library/arrangements quieter | `WorkspaceApplicationStage`, `stageDesktopUi`, `App.css` | **Implemented** — [completion report](../03-Engineering/IM-1-STAGE-EMPTY-SPATIAL-CALM-COMPLETION-REPORT.md); await visual review |
| **IM-2** | Desktop Reality First | Default rail closed; remove create-workspace empty CTA; demote profile/setup copy | `assistantRail.ts`, Assistant/arrangements/profiles copy | **Implemented** — [completion report](../03-Engineering/IM-2-DESKTOP-REALITY-FIRST-COMPLETION-REPORT.md); await visual review |
| **IM-3** | Arrangements as remember-control | Replace “Create a workspace first” hero; profile needed only to **save** | `desktopArrangementUi`, panel layout | Controls serve desktop; no setup story |
| **IM-4** | Profiles, not Workspaces-as-product | Workspaces tab = optional profiles; create demoted | `WorkspaceSwitcher`, `workspaceSwitcherUi` | Tab no longer teaches setup wizard |
| **IM-5** | Home anti-dashboard | Strip cards/roadmap; Stage CTA only | `WorkspaceHome` | Home cannot be mistaken for the product |
| **IM-6** | Applications observed-first | Running list above library; library empty one-liner | `ApplicationsPanel` | Tab supports Stage; doesn’t replace it |
| **IM-7** | Chrome quieting | Nav weight + single Flow/Focus locus | `App.tsx`, `App.css`, Stage header | Peer-tab dashboard feeling reduced |

**Explicitly later (not in this sequence):** OS geometry apply for modes; grouping; audio; arrangement editor craftsmanship; new observation engines.

**Relationship to Milestone R:**  
R Slice 1 already delivered observation-backed Stage. Treat **IM-1…IM-7** as contract-alignment slices on top of that baseline (presentation/IA). Do not reopen architecture. Do not start a parallel “R rewrite.”

---

## 5. Scoreboard summary

| Surface | Score | One-line gap |
|---------|------:|--------------|
| Stage | 7 | Reality wired; empty state still text-led |
| Home | 5 | Still a dashboard of explanations |
| Applications | 6 | Library management still too loud |
| Workspaces | 3 | Setup-wizard identity |
| Layouts (= Stage) | 8 | Naming leftovers / canvas confusion |
| Assistant | 6 | Default-open + workspace-gated empty copy |
| Navigation | 7 | Peer tabs flatten hierarchy |
| Flow | 6 | Good with windows; weak empty plane |
| Focus | 6 | Dual heroes (observed + library) |
| Arrangements (control) | 4 | Create-workspace gate |

**Overall product feel vs contract today:** roughly **6 / 10** — direction correct after R Slice 1; first glance in preview/empty states still drifts toward dashboard + companion column.

---

## 6. Stop

Audit complete.  

**Do not implement Milestone R or IM slices from this file until a specific slice is approved.**  
**Do not modify application code in response to this document without an explicit implementation brief.**
