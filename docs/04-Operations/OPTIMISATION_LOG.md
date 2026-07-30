# Optimisation Log

| Field | Value |
|-------|-------|
| **Purpose** | Record material quality, performance, storage, indexing, and chrome-optimisation work so cycles remain human-inspectable |
| **Owner** | Engineering |
| **Status** | Active template |
| **Protocol** | [Optimisation Protocol v2 — Plateau Detection](OPTIMISATION_PROTOCOL_V2.md) |
| **Related** | [AI Engineering Governance §3 & §11](../00-Governance/AI_ENGINEERING_GOVERNANCE.md), [Performance Budgets](../02-Architecture/PERFORMANCE-BUDGETS.md) |

Use one cycle entry per meaningful optimisation. Do **not** log trivial micro-edits.

Under **Plateau v2**, a session may only declare **global plateau** after all 26 approved categories are PLATEAUED and two consecutive full-category evaluation passes find no measurable improvement. Premature “two weak cycles” stops are **superseded** (see protocol). Boundary stops (product direction, ownership, engines, PG/DA beyond approval) still end immediately.

Every cycle must preserve the understanding path:

```text
Human concept → Optimised representation → Decoder / inspector → Human understanding
```

When logging a cycle, name the **approved category** (1–26). When a category has no measurable improvement left, mark it **PLATEAUED**.

### Category board (Plateau v2 — living)

Update when a category is evaluated. Status: `open` | `PLATEAUED` | `boundary-blocked`.

| # | Category | Status | Notes |
|---|----------|--------|-------|
| 1 | Visual hierarchy | open | stronger chrome brand (v2-15) |
| 2 | Layout consistency | open | banner/container pad tokens (v2-16) |
| 3 | Spacing consistency | open | shell CSS tokens (v2-11) |
| 4 | Typography consistency | open | Product h3 (v2-07) |
| 5 | Navigation clarity | open | Assistant aria-label (v2-10) |
| 6 | Accessibility | open | focus-visible (v2-02) |
| 7 | Keyboard UX | open | rail focus (v2-03) |
| 8 | Responsiveness | open | chrome stack + merged 900px (v2-12) |
| 9 | Animation polish | open | rail enter + reduced-motion (v2-08) |
| 10 | Component consistency | open | |
| 11 | CSS simplification | open | focus merge (v2-06); 900px merge (v2-12) |
| 12 | Duplicate removal | open | FocusSupportingAppChips (v2-17) |
| 13 | Dead code removal | open | unused selectors (v2-04, v2-18) |
| 14 | Documentation quality | open | Protocol v2 + board |
| 15 | Naming clarity | open | DESKTOP_PREVIEW_BANNER (v2-05) |
| 16 | Maintainability | open | |
| 17 | Human readability | open | |
| 18 | Developer experience | open | arch map --write note (v2-13) |
| 19 | Code organisation | open | rail id in assistantRail.ts (v2-09) |
| 20 | Test quality | open | hook + banner tests (v2-01, v2-05) |
| 21 | Performance | PLATEAUED | No measured shell bottleneck; no speculative memo (v2-14) |
| 22 | Memory efficiency | PLATEAUED | No measured leak/allocation issue in chrome path (v2-14) |
| 23 | Build cleanliness | PLATEAUED | typecheck/build green; no actionable frontend warnings (v2-14) |
| 24 | IPC cleanliness | PLATEAUED | verify:ipc-contract green; no orphaned shell consumers (v2-14) |
| 25 | Error messaging | open | string classify (v2-05) |
| 26 | Commercial readiness | open | |

---

## Cycle template (copy below)

```markdown
### Cycle: <short-id>

| Field | Value |
|-------|-------|
| **Date** | YYYY-MM-DD |
| **Category** | #N — name |
| **Goal** | |
| **Problem** | |
| **Analysis** | |
| **Changes** | |
| **Files affected** | |
| **Validation** | |
| **Maintainability score** | /10 |
| **Reference alignment** | /10 |
| **Commercial readiness** | /10 |
| **Human readability** | /10 |
| **Accessibility** | /10 (when relevant) |
| **Performance** | /10 (when relevant) |
| **Developer experience** | /10 (when relevant) |
| **Test health** | /10 (when relevant) |
| **Category status** | open / PLATEAUED |
| **Human review required** | Yes / No |
| **Why this is safe** | |
| **Inspector / decoder path** | How a human inspects or decodes the representation |
```

---

## Cycles

### Cycle: opt-v2-18-dead-assistant-tool-note

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Category** | 13 — Dead code removal |
| **Problem** | `.assistant-tool-note` CSS had no TSX consumers after companion-rail migration |
| **Reason** | Delete dead selector |
| **Files changed** | `App.css`, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build |
| **Category status** | open |
| **Why this is safe** | Dead CSS only |

---

### Cycle: opt-v2-17-focus-supporting-chips

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Category** | 12 — Duplicate removal |
| **Problem** | Apps and Layouts duplicated Focus supporting chip markup |
| **Reason** | Shared `FocusSupportingAppChips` keeps Focus density presentation one place |
| **Files changed** | `FocusSupportingAppChips.tsx`, `ApplicationList.tsx`, `WorkspaceApplicationStage.tsx`, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build |
| **Maintainability score** | 9.2/10 |
| **Category status** | open |
| **Why this is safe** | Presentational extract; same callbacks/behaviour |

---

### Cycle: opt-v2-16-layout-pad-tokens

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Category** | 2 — Layout consistency |
| **Problem** | Banner/container horizontal padding drifted from chrome shell pad |
| **Reason** | Use `--shell-pad-x` for banners and containers |
| **Files changed** | `App.css`, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build |
| **Category status** | open |
| **Why this is safe** | CSS token alignment only |

---

### Cycle: opt-v2-15-chrome-brand-hierarchy

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Category** | 1 — Visual hierarchy |
| **Problem** | Chrome “Workspace” brand under-weighted vs tabs |
| **Reason** | Slightly stronger brand type weight/size (concept: brand is hero signal in chrome) |
| **Files changed** | `App.css`, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build |
| **Reference alignment** | 9.2/10 |
| **Category status** | open |
| **Why this is safe** | Typography weight only; no IA change |

---

### Cycle: opt-v2-14-plateau-perf-memory-build-ipc

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Category** | 21–24 — Performance, Memory, Build, IPC |
| **Problem** | Categories not yet formally evaluated under Plateau v2 |
| **Analysis** | Perf/memory: no measured chrome regression to fix without speculative optimisation (anti-slop). Build: `pnpm typecheck`/`build` clean. IPC: `verify:ipc-contract` 206/159 green; shell uses existing invoke helpers only. |
| **Changes** | Mark categories 21–24 **PLATEAUED** (analysis only) |
| **Files changed** | this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build (reaffirmed) |
| **Category status** | PLATEAUED (21–24) |
| **Why this is safe** | Documentation of evaluation; no speculative code |

---

### Cycle: opt-v2-13-dx-architecture-map-note

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Category** | 18 — Developer experience |
| **Problem** | Agents often forgot `--write` when architecture inventory drifted |
| **Reason** | Document refresh command in AGENTS.md known-good checks |
| **Files changed** | `AGENTS.md`, this log |
| **Validation** | docs + existing verify scripts |
| **Developer experience** | 8.5/10 |
| **Category status** | open |
| **Why this is safe** | AGENTS documentation only |

---

### Cycle: opt-v2-12-responsive-media-merge

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Category** | 8 — Responsiveness (+ 11 CSS simplification) |
| **Problem** | Three separate `max-width: 900px` blocks; chrome cramped on narrow widths |
| **Reason** | Single responsive block; stack chrome brand above nav on narrow viewports |
| **Files changed** | `App.css`, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build |
| **Reference alignment** | 9.1/10 |
| **Commercial readiness** | 6.5/10 |
| **Category status** | open |
| **Why this is safe** | CSS layout only |

---

### Cycle: opt-v2-11-shell-spacing-tokens

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Category** | 3 — Spacing consistency |
| **Problem** | Chrome/rail padding used magic rem literals |
| **Reason** | Named `--shell-*` / `--rail-*` tokens for consistent chrome spacing |
| **Files changed** | `App.css`, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build |
| **Maintainability score** | 9.1/10 |
| **Category status** | open |
| **Why this is safe** | CSS variables only |

---

### Cycle: opt-v2-10-assistant-nav-label

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Category** | 5 — Navigation clarity |
| **Problem** | Assistant chrome control lacked aria-label describing toggle intent |
| **Reason** | Visible label stays “Assistant”; accessible name uses show/hide companion phrasing |
| **Files changed** | `App.tsx`, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build |
| **Accessibility** | 8.6/10 |
| **Category status** | open |
| **Why this is safe** | aria-label only |

---

### Cycle: opt-v2-09-rail-id-ownership

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Category** | 19 — Code organisation |
| **Problem** | DOM id constant lived on the React component module instead of preference helpers |
| **Reason** | Keep rail identity next to `assistantRail` preference API |
| **Files changed** | `assistantRail.ts`, `AssistantCompanionRail.tsx`, `App.tsx`, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build |
| **Maintainability score** | 9.1/10 |
| **Category status** | open |
| **Why this is safe** | Import move only |

---

### Cycle: opt-v2-08-companion-rail-motion

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Category** | 9 — Animation polish |
| **Problem** | Companion rail appeared without presence; no reduced-motion consideration |
| **Reason** | Short enter motion; disabled when `prefers-reduced-motion` |
| **Files changed** | `App.css`, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build |
| **Reference alignment** | 9.0/10 |
| **Accessibility** | 8.5/10 |
| **Category status** | open |
| **Why this is safe** | Presentational motion only |

---

### Cycle: opt-v2-07-product-heading-typography

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Category** | 4 — Typography consistency |
| **Problem** | Global `section h3` forced uppercase diagnostic labels onto product Home/Apps/Layouts titles |
| **Reason** | Sentence-case product headings; keep uppercase for engineering containers |
| **Files changed** | `App.css`, this log (+ category board under docs cycle) |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build |
| **Reference alignment** | 9.1/10 |
| **Human readability** | 9.0/10 |
| **Category status** | open |
| **Why this is safe** | Typography CSS scope only |

---

### Cycle: opt-v2-06-css-focus-rule-merge

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Category** | 11 — CSS simplification |
| **Problem** | Repeated `:focus-visible` blocks duplicated the same outline tokens |
| **Reason** | One selector group; easier to keep keyboard focus styling consistent |
| **Files changed** | `App.css`, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build |
| **Maintainability score** | 9.0/10 |
| **Category status** | open |
| **Why this is safe** | CSS consolidation only |

---

### Cycle: opt-v2-05-error-banner-strings

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Category** | 25 — Error messaging |
| **Problem** | `onError` wrapped strings in `Error`, duplicating preview copy and skipping string classification |
| **Reason** | Named `DESKTOP_PREVIEW_BANNER`; classify strings directly |
| **Files changed** | `productShellUi.ts`, `App.tsx`, tests, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build |
| **Maintainability score** | 9.0/10 |
| **Commercial readiness** | 6.5/10 |
| **Human readability** | 9.0/10 |
| **Category status** | open |
| **Why this is safe** | Banner classification only; no IPC ownership change |

---

### Cycle: opt-v2-04-dead-css-selectors

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Category** | 13 — Dead code removal |
| **Problem** | Unused `.assistant-intel-container`, `.tab.secondary`, `.tab.quiet` remained after chrome evolution |
| **Reason** | Remove selectors with no TSX consumers |
| **Files changed** | `App.css`, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build |
| **Maintainability score** | 9.1/10 |
| **Category status** | open |
| **Why this is safe** | Dead CSS only |

---

### Cycle: opt-v2-03-keyboard-rail-focus

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Category** | 7 — Keyboard UX |
| **Problem** | Opening companion rail did not move keyboard focus; Escape left focus nowhere useful |
| **Reason** | Focus rail on open; return focus to Assistant chrome control on hide |
| **Files changed** | `App.tsx`, `AssistantCompanionRail.tsx`, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build |
| **Maintainability score** | 9.0/10 |
| **Accessibility** | 8.5/10 |
| **Human readability** | 8.9/10 |
| **Category status** | open |
| **Why this is safe** | Focus management only; no ownership/IPC/AI change |

---

### Cycle: opt-v2-02-focus-visible-buttons

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Category** | 6 — Accessibility |
| **Problem** | Global `button:focus` outlined every mouse click; inconsistent with newer `:focus-visible` rules |
| **Reason** | Keyboard-visible focus without mouse outline noise |
| **Files changed** | `App.css`, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build |
| **Maintainability score** | 9.0/10 |
| **Accessibility** | 8.4/10 (+0.3 focus-visible consistency) |
| **Reference alignment** | 9.0/10 |
| **Category status** | open |
| **Why this is safe** | CSS focus ring behaviour only |

---

### Cycle: opt-v2-01-test-preference-hooks

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Category** | 20 — Test quality |
| **Problem** | `useWorkMode` / `useAssistantRail` were unreferenced by tests after C5 extraction |
| **Reason** | Measurable test health: assert chrome preference hooks remain importable contracts |
| **Files changed** | `tests/milestone-a-workspace-apps.test.ts`, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build |
| **Maintainability score** | 9.0/10 |
| **Reference alignment** | 9.0/10 |
| **Commercial readiness** | 6.4/10 |
| **Human readability** | 8.9/10 |
| **Test health** | 8.6/10 (+0.2 hook contract coverage) |
| **Category status** | open (more UI tests possible later) |
| **Why this is safe** | Tests only; no product behaviour change |

---

### Cycle: gov-plateau-v2-protocol

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Category** | 14 — Documentation quality (governance) |
| **Problem** | Premature plateau rule stopped sessions before approved engineering categories were exhausted |
| **Reason** | Adopt Plateau Detection v2: category exhaustion + anti-slop + unchanged product boundary stops |
| **Files changed** | `OPTIMISATION_PROTOCOL_V2.md`, `AGENTS.md`, `AI_ENGINEERING_GOVERNANCE.md`, `OPTIMISATION_LOG.md`, `docs/README.md`, `04-Operations/README.md` |
| **Validation** | docs-only; no application behaviour change; governance scripts as applicable |
| **Maintainability score** | 9.0/10 |
| **Reference alignment** | n/a (process) |
| **Commercial readiness** | +process clarity |
| **Human readability** | 9.0/10 |
| **Category status** | open (docs improved; product UI categories not re-evaluated here) |
| **Human review required** | No |
| **Why this is safe** | Governance/process documentation only — no product code |

---

### Cycle: opt-c10-plateau-detection-2

| Field | Value |
|-------|-------|
| **Date** | 2026-07-29 |
| **Problem** | Second consecutive review of remaining candidates |
| **Reason** | **PLATEAU DETECTION 2** — no new meaningful safe improvement without human product-direction choice (e.g. bottom nav vs top chrome) or blocked scope (OS geometry apply on mode switch) |
| **Files changed** | this log only |
| **Validation** | prior cycle green; no code change |
| **Maintainability score** | 9.0/10 |
| **Reference alignment** | 9.0/10 |
| **Commercial readiness** | 6.4/10 |
| **Human readability** | 8.9/10 |
| **Why this is safe** | Documentation-only stop record |
| **Stop** | Two consecutive plateau detections → end controlled optimisation session |
| **Superseded by** | [Optimisation Protocol v2](OPTIMISATION_PROTOCOL_V2.md) — premature without category exhaustion; session may resume under V2 |

---

### Cycle: opt-c9-plateau-detection-1

| Field | Value |
|-------|-------|
| **Date** | 2026-07-29 |
| **Problem** | After C1–C8, remaining candidates are cosmetic or require direction |
| **Reason** | **PLATEAU DETECTION 1** — scored candidates: (a) bottom nav IA swap — equally valid vs current top chrome, needs human; (b) OS geometry on Flow/Focus — not approved; (c) further CSS micro-polish — not meaningful |
| **Files changed** | this log only |
| **Validation** | reaffirm: typecheck/test/arch/ipc/ui/build from tip `79ce102` |
| **Maintainability score** | 9.0/10 |
| **Reference alignment** | 9.0/10 |
| **Commercial readiness** | 6.4/10 |
| **Human readability** | 8.9/10 |
| **Why this is safe** | Analysis only; no speculative edits |

---

### Cycle: opt-c8-applications-rail-width

| Field | Value |
|-------|-------|
| **Date** | 2026-07-29 |
| **Problem** | Applications panel kept a narrow max-width beside the companion rail, wasting stage space |
| **Reason** | Desktop-first: product stage should use remaining width when companion is present |
| **Files changed** | `App.css`, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build |
| **Maintainability score** | 9.0/10 |
| **Reference alignment** | 9.0/10 (+0.1 stage uses available width) |
| **Commercial readiness** | 6.4/10 |
| **Human readability** | 8.9/10 |
| **Why this is safe** | CSS layout only |

---

### Cycle: opt-c7-dead-assistant-page-css

| Field | Value |
|-------|-------|
| **Date** | 2026-07-29 |
| **Problem** | Full-page Assistant stage CSS remained after Milestone C rail replaced it |
| **Reason** | Dead CSS confuses future engineers about current chrome model |
| **Files changed** | `App.css`, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build |
| **Maintainability score** | 9.0/10 |
| **Reference alignment** | 8.9/10 |
| **Commercial readiness** | 6.4/10 |
| **Human readability** | 8.9/10 |
| **Why this is safe** | CSS cleanup only; behaviour already on companion rail |

---

### Cycle: opt-c6-chrome-noise-arrangement-hierarchy

| Field | Value |
|-------|-------|
| **Date** | 2026-07-29 |
| **Problem** | Rail toggles spammed status banners; arrangements hero still competed when stacked under stage |
| **Reason** | Quieter desktop-first chrome; arrangements read as secondary under the stage |
| **Files changed** | `App.tsx`, `App.css`, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build |
| **Maintainability score** | 8.9/10 |
| **Reference alignment** | 8.9/10 (+0.1 quieter hierarchy) |
| **Commercial readiness** | 6.4/10 |
| **Human readability** | 8.8/10 |
| **Why this is safe** | Status/copy density + CSS hierarchy only |

---

### Cycle: opt-c5-preference-hooks-milestone-c-docs

| Field | Value |
|-------|-------|
| **Date** | 2026-07-29 |
| **Problem** | App.tsx owned two preference state machines inline; Milestone C lacked an engineering report |
| **Reason** | Maintainability + human-readable milestone record without ownership change |
| **Files changed** | `useWorkMode.ts`, `useAssistantRail.ts`, `App.tsx`, `MILESTONE-C-COMPANION-RAIL-REPORT.md`, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build |
| **Maintainability score** | 8.9/10 (+0.2 hooks) |
| **Reference alignment** | 8.8/10 |
| **Commercial readiness** | 6.3/10 |
| **Human readability** | 8.8/10 |
| **Why this is safe** | Presentation preference extraction + docs only |

---

### Cycle: opt-c4-companion-rail-a11y

| Field | Value |
|-------|-------|
| **Date** | 2026-07-29 |
| **Problem** | Companion rail toggle lacked aria-controls; no Escape hide path |
| **Reason** | Commercial a11y for the newly persistent companion chrome |
| **Files changed** | `AssistantCompanionRail.tsx`, `App.tsx`, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build |
| **Maintainability score** | 8.7/10 |
| **Reference alignment** | 8.8/10 (unchanged layout) |
| **Commercial readiness** | 6.3/10 (+0.2 a11y) |
| **Human readability** | 8.7/10 |
| **Why this is safe** | Chrome a11y only; no ownership/IPC/AI/DA change |

---

### Cycle: opt-c3-layout-hierarchy-home-honesty

| Field | Value |
|-------|-------|
| **Date** | 2026-07-29 |
| **Problem** | With companion rail open, Layouts/Workspaces showed three competing columns; Home still claimed sidecar was missing |
| **Reason** | Concept: one right companion; arrangements stay under stage; product copy must match shipped chrome |
| **Files changed** | `App.css`, `WorkspaceHome.tsx`, `AssistantIntelligencePanel.tsx`, `OperatorConsole.tsx`, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build |
| **Maintainability score** | 8.7/10 |
| **Reference alignment** | 8.8/10 (+0.2 hierarchy) |
| **Commercial readiness** | 6.1/10 |
| **Human readability** | 8.7/10 |
| **Why this is safe** | CSS hierarchy + copy only; no DA/AI/ownership behaviour change |

---

### Cycle: opt-c2-companion-rail-density

| Field | Value |
|-------|-------|
| **Date** | 2026-07-29 |
| **Problem** | Persistent rail duplicated companion hero and flooded the stage with six evidence layers |
| **Reason** | Companion must stay secondary; ask-first, packages on demand |
| **Files changed** | `AssistantIntelligencePanel.tsx`, `AssistantCompanionRail.tsx`, `App.css`, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build |
| **Maintainability score** | 8.7/10 |
| **Reference alignment** | 8.6/10 (+0.2 quieter companion) |
| **Commercial readiness** | 6.0/10 |
| **Human readability** | 8.6/10 |
| **Why this is safe** | Presentation-only `presentation="rail"`; no IPC/AI/ownership change |

---

### Cycle: opt-c1-persistent-assistant-rail

| Field | Value |
|-------|-------|
| **Date** | 2026-07-29 |
| **Problem** | Assistant was a peer full-page tool tab, not a concept-aligned companion rail |
| **Reason** | Approved Milestone C: persistent right rail on primary product views |
| **Files changed** | `App.tsx`, `App.css`, `AssistantCompanionRail.tsx`, `assistantRail.ts`, tests, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build |
| **Maintainability score** | 8.6/10 |
| **Reference alignment** | 8.4/10 (+0.7 companion placement) |
| **Commercial readiness** | 5.8/10 |
| **Human readability** | 8.5/10 |
| **Why this is safe** | Chrome placement only; reuses existing assistant panels/IPC; no AI/ownership/PG/DA behaviour change |

---

### Cycle: opt-o3-focus-partition-helper

| Field | Value |
|-------|-------|
| **Date** | 2026-07-29 |
| **Problem** | Apps and Layouts each inlined Focus primary/supporting split — drift risk after O2 |
| **Reason** | One named rule (`partitionFocusApplications`) keeps Focus density consistent and testable |
| **Files changed** | `workMode.ts`, `ApplicationList.tsx`, `WorkspaceApplicationStage.tsx`, `tests/milestone-a-workspace-apps.test.ts`, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary |
| **Maintainability score** | 8.7/10 (+0.2 shared Focus partition) |
| **Reference alignment** | 7.7/10 (unchanged presentation) |
| **Commercial readiness** | 5.4/10 |
| **Human readability** | 8.4/10 |
| **Why this is safe** | Pure presentation helper; no OS/AI/IPC/ownership change |

---

### Cycle: opt-o2-applications-work-mode-density

| Field | Value |
|-------|-------|
| **Date** | 2026-07-29 |
| **Problem** | Applications tab ignored Flow/Focus, so density felt inconsistent with Layouts |
| **Reason** | Align product hierarchy presentation across primary surfaces without new features |
| **Files changed** | `ApplicationList.tsx`, `ApplicationsPanel.tsx`, `App.tsx`, `App.css`, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary |
| **Maintainability score** | 8.5/10 |
| **Reference alignment** | 7.7/10 (+0.2 density consistency) |
| **Commercial readiness** | 5.4/10 |
| **Human readability** | 8.3/10 |
| **Why this is safe** | Reuses existing workMode preference; presentation only; no OS/AI/IPC ownership change |

---

### Cycle: opt-o1-work-mode-keyboard-a11y

| Field | Value |
|-------|-------|
| **Date** | 2026-07-29 |
| **Problem** | Flow/Focus switch was mouse-oriented; weak keyboard/focus-visible affordances |
| **Reason** | Commercial usability + a11y without product-scope change |
| **Files changed** | `WorkModeSwitch.tsx`, `App.css`, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary |
| **Maintainability score** | 8.5/10 |
| **Reference alignment** | 7.5/10 (unchanged product shape) |
| **Commercial readiness** | 5.2/10 (+0.2 from keyboard parity) |
| **Human readability** | 8.2/10 |
| **Why this is safe** | Presentation control only; no OS/IPC/AI/ownership change |

---

### Cycle: milestone-b-flow-focus-chrome

| Field | Value |
|-------|-------|
| **Date** | 2026-07-29 |
| **Cycle number** | Milestone B |
| **Goal** | User-facing Flow ↔ Focus chrome density without OS window apply |
| **Problem** | Reference density modes missing; geometry apply not approved yet |
| **Analysis** | Approved scope: presentation only; reuse Layouts stage + registry |
| **Changes** | `workMode` module, `WorkModeSwitch`, stage Flow/Focus layouts, Focus hides canvas |
| **Files affected** | See FLOW-FOCUS-IMPLEMENTATION-REPORT.md |
| **Validation** | typecheck / build / test / architecture / ipc / ui-boundary |
| **Maintainability score** | 8.5/10 |
| **Reference alignment score** | 7.5/10 |
| **Human review required** | Yes — visual |
| **Remaining risks** | Users may expect OS windows to move; mitigated by explicit copy |
| **Next recommended cycle** | After Accept: OS arrangement apply on mode switch **or** persistent Assistant rail (C) |
| **Inspector / decoder path** | Layouts → Flow/Focus control; `workMode.ts` |

---

### Cycle: cycle-2-assistant-copy-clarity

| Field | Value |
|-------|-------|
| **Date** | 2026-07-29 |
| **Cycle number** | 2 |
| **Goal** | Reduce Assistant engineering jargon so the companion feels user-facing, not Programme IV documentation |
| **Problem** | Assistant rail still led with batch numbers and intelligence-stack language |
| **Analysis** | Small UX / cognitive-noise fix under autonomy rules; no AI behaviour change |
| **Changes** | Softened `AssistantIntelligencePanel` + `AssistantPanel` user copy; layers labeled 1–6 instead of Batch 11–16 in UI |
| **Files affected** | `AssistantIntelligencePanel.tsx`, `AssistantPanel.tsx`, this log |
| **Validation** | typecheck / test (run at commit) |
| **Maintainability score** | 8/10 — copy-only; ownership unchanged |
| **Reference alignment score** | ~6.7/10 — companion feels calmer; still not persistent sidecar |
| **Human review required** | No |
| **Remaining risks** | Deep panel internals still engineering-dense when expanded |
| **Next recommended cycle** | **Stop for human review before Milestone B (Flow ↔ Focus)** — workflow/visual direction change |
| **Inspector / decoder path** | Open Assistant tool tab → companion rail headers |

---

### Cycle: cycle-1-layouts-app-stage

| Field | Value |
|-------|-------|
| **Date** | 2026-07-29 |
| **Cycle number** | 1 |
| **Goal** | Make Layouts feel like a workspace stage by presenting registered apps as spatial assets |
| **Problem** | Layouts showed only canvas zones; apps stayed in an admin registry — weak reference alignment |
| **Analysis** | Highest autonomous gap without implementing Flow/Focus (workflow change → human review). Reuse `list_applications` + arrangements rail. |
| **Changes** | Added `WorkspaceApplicationStage` + `layoutsStageUi`; Layouts hosts stage above canvas; refresh apps on Home/Layouts view |
| **Files affected** | `WorkspaceApplicationStage.tsx`, `layoutsStageUi.ts`, `App.tsx`, `App.css`, tests, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary (run at commit) |
| **Maintainability score** | 8/10 — thin presentation component, pure helpers, clear non-responsibilities |
| **Reference alignment score** | 6.5/10 (from ~6.0) — stage presence improved; still not live OS tiles or modes |
| **Human review required** | No for this cycle; **Yes before Milestone B (Flow ↔ Focus)** |
| **Remaining risks** | Users may confuse registry tiles with live windows — mitigated by explicit copy |
| **Next recommended cycle** | Soften Assistant rail engineering jargon **or** human-approved Milestone B modes |
| **Inspector / decoder path** | Open Layouts tab → stage strip + arrangements rail; helpers under `layoutsStageUi.ts` |

---

_Newest first above. Template remains at top of file._
