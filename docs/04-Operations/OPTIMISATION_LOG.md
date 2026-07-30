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
| 1 | Visual hierarchy | PLATEAUED | Brand weight + product h3; further IA needs human (v2-25) |
| 2 | Layout consistency | PLATEAUED | Shell/banner/container pad tokens aligned (v2-25) |
| 3 | Spacing consistency | PLATEAUED | `--shell-*` / `--rail-*` applied (v2-25) |
| 4 | Typography consistency | PLATEAUED | Product sentence-case h3 vs diagnostic uppercase (v2-25) |
| 5 | Navigation clarity | PLATEAUED | Assistant aria-label; bottom-nav needs human (v2-23) |
| 6 | Accessibility | PLATEAUED | focus-visible + rail a11y complete for current chrome (v2-25) |
| 7 | Keyboard UX | PLATEAUED | Mode radiogroup + rail focus/Escape/return (v2-23) |
| 8 | Responsiveness | PLATEAUED | Merged 900px + stacked chrome (v2-25) |
| 9 | Animation polish | PLATEAUED | Rail enter + reduced-motion (v2-20) |
| 10 | Component consistency | PLATEAUED | Shared Focus chips + focus group (v2-25) |
| 11 | CSS simplification | PLATEAUED | Focus merge + media merge (v2-25) |
| 12 | Duplicate removal | PLATEAUED | partitionFocus + FocusSupportingAppChips (v2-25) |
| 13 | Dead code removal | PLATEAUED | Unused selectors removed incl. application-list-row (v2-24) |
| 14 | Documentation quality | PLATEAUED | Protocol v2 + living board (v2-25) |
| 15 | Naming clarity | PLATEAUED | Named banners/ids/tokens (v2-23) |
| 16 | Maintainability | PLATEAUED | Hooks + headers + shared chips (v2-25) |
| 17 | Human readability | PLATEAUED | Clear ownership comments on chrome modules (v2-25) |
| 18 | Developer experience | PLATEAUED | Arch map --write documented (v2-25) |
| 19 | Code organisation | PLATEAUED | Rail id + preference hooks (v2-25) |
| 20 | Test quality | PLATEAUED | Preference/banner/chips contracts covered (v2-25) |
| 21 | Performance | PLATEAUED | No measured shell bottleneck (v2-14) |
| 22 | Memory efficiency | PLATEAUED | No measured chrome leak path (v2-14) |
| 23 | Build cleanliness | PLATEAUED | Frontend verify/build green (v2-14) |
| 24 | IPC cleanliness | PLATEAUED | IPC contract verify green (v2-14) |
| 25 | Error messaging | PLATEAUED | String classify + preview banner (v2-23) |
| 26 | Commercial readiness | PLATEAUED | Chrome polish landed; OS geometry still boundary-blocked (v2-25) |

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

## Cycles

### Cycle: v8-5-focus-organisation-from-window-groups

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V8-5 |
| **Goal** | Focus Stage organisation consumes process_id window_groups |
| **Problem** | Focus still rebuilt primary-process membership with local PID filtering |
| **Analysis** | Pass authoritative groups into organiseStageForWorkMode; PID filter remains fallback only |
| **Changes** | organiseStageForWorkMode(windowGroups); Stage wires groups |
| **Files affected** | stageDesktopUi.ts, WorkspaceApplicationStage.tsx, stage-desktop-ui.test.ts, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary |
| **Deleted / reduced** | Invented Focus process bucketing when groups are present |
| **Next recommended cycle** | Two empty capability evaluation passes |

---

### Cycle: v8-4-environment-projects-workspace-state-groups

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V8-4 |
| **Goal** | Environment window groups consume WorkspaceState groups instead of rebuilding process buckets |
| **Problem** | Environment still ran a parallel process grouping path after V8-1 domain engine adoption |
| **Analysis** | Remap authoritative member ids → env_window ids; keep only matched_application as Environment-local (registry facts) |
| **Changes** | `project_groups_from_workspace_state`; deleted Environment process re-bucketing |
| **Files affected** | workspace_environment.rs, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary (+ environment grouping tests) |
| **Deleted / reduced** | Environment ProcessId re-group path |
| **Next recommended cycle** | Capability rescore; stop after two empty evals or platform/architecture gates |

---

### Cycle: v8-3-identity-continuity-on-workspace-state

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V8-3 |
| **Goal** | Surface live identity-registry continuity on WorkspaceState windows |
| **Problem** | first_seen/last_seen/confidence existed in the registry but never reached the authoritative runtime model or Assistant |
| **Analysis** | Enrich windows from `ObservationWindowIdentity` by stable id without treating registry rows as historical snapshot state |
| **Changes** | `first_seen_at` / `last_seen_at` / `identity_confidence` on `WorkspaceStateWindow`; engine `list_by_ids`; Assistant continuity answers |
| **Files affected** | domain workspace_state, state engine, domain.ts, assistantCompanion, stage tests, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary (+ domain state tests) |
| **Deleted / reduced** | none (pure enrichment of existing identity facts) |
| **Next recommended cycle** | Environment consume WorkspaceState.window_groups, or two empty eval passes if only architecture-gated work remains |

---

### Cycle: v8-2-atomic-latest-delta-on-workspace-state

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V8-2 |
| **Goal** | Make observation delta atomic with WorkspaceState; stop Assistant dual-read race |
| **Problem** | Assistant refreshed WorkspaceState and `get_latest_observation_delta` separately — windows and change facts could disagree across passes |
| **Analysis** | Delta already computed when projecting state; embedding it removes an IPC round-trip and restores one runtime truth |
| **Changes** | `WorkspaceState.latest_delta`; Assistant reads `state.latest_delta`; working-on answers prefer process groups |
| **Files affected** | domain workspace_state, domain.ts, AssistantIntelligencePanel, assistantCompanion, kernel state engine tests, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary (+ domain/kernel state tests) |
| **Deleted / reduced** | Assistant `get_latest_observation_delta` parallel fetch |
| **Next recommended cycle** | Identity continuity facts on windows, or Environment consume WorkspaceState.window_groups |

---

### Cycle: v8-1-authoritative-groups-and-arrangement-membership

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V8-1 |
| **Goal** | Consume V7 `window_groups` as sole relationship source; fold arrangement membership into WorkspaceState grouping |
| **Problem** | Stage invented process/monitor relationships; Assistant reinvented process buckets; arrangement facts never reached the grouping engine; duplicate observation refresh paths |
| **Analysis** | Highest capability gain: one observation client + authoritative groups + arrangement membership on WorkspaceState without new engines |
| **Changes** | `workspaceStateClient`; Stage/Apps/Assistant refresh via client; Stage relationships from `window_groups`; Environment uses domain `group_desktop_members`; engine loads arrangement membership facts into `window_groups`; Assistant local answers/enrichment read groups |
| **Files affected** | domain `workspace_state`, database arrangement repo, kernel state engine/environment, Stage/Apps/Assistant TS, tests, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary (+ domain/database/kernel grouping tests) |
| **Deleted / reduced** | Local Stage relationship invention; Environment HashMap bucketing; per-panel ensure+get duplication; Assistant ad-hoc process counting for “belongs together” |
| **Next recommended cycle** | Deterministic desktop delta→history identity on WorkspaceState, or geometry/thumbnail ownership hooks if platform APIs remain blocked |

---

### Cycle: v6-eval-pass-2-capability-plateau

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V6-eval-2 |
| **Analysis** | Second pass after V6-1/V6-2. No safe measurable capability left without architecture approval. |
| **Changes** | none |
| **Stop** | **CAPABILITY PLATEAU** — two consecutive empty evaluation passes |

---

### Cycle: v6-eval-pass-1-capability-rescore

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V6-eval-1 |
| **Analysis** | Objects/Relationships/Interaction/Organisation/Assistant loops closed within existing IPC. Remaining candidates are architecture-gated or non-measurable polish. |
| **Changes** | none |

---

### Cycle: v6-2-selection-working-set-capture

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V6-2 |
| **Capability groups** | Organisation |
| **Changes** | `capture_desktop_arrangement` optional `member_hwnds`; Stage “Save selection” creates arrangement working set from selected objects |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary (+ domain/kernel arrangement tests) |

---

### Cycle: v6-1-runtime-objects-relationships-interaction

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V6-1 |
| **Capability groups** | Objects · Relationships · Interaction · Organisation · Assistant |
| **Changes** | Stage tiles as runtime objects (pid/stable/visible/monitor); Flow process+monitor relationships; select≠activate + multi-select + keyboard; arrangement working-set overlay; Assistant local answers from state/delta/arrangements |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary |
| **Deleted** | name-only process sibling grouping (replaced by pid) |

---

### Cycle: v5-eval-pass-2-interaction-plateau

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V5-eval-2 |
| **Analysis** | Second full pass after V5-1…V5-3. No safe measurable interaction/organisation change remains without inventing min/max APIs, Assistant-owned control, streaming IPC, OS geometry apply, or grouping engines. |
| **Changes** | none |
| **Stop** | **INTERACTION PLATEAU** — two consecutive empty evaluation passes |

---

### Cycle: v5-eval-pass-1-interaction-rescore

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V5-eval-1 |
| **Analysis** | After V5-3 honesty + dead-helper cleanup, candidates are architecture-gated (minimize, geometry apply, streaming, Assistant restore) or cosmetic. |
| **Changes** | none |
| **Category status** | open only behind architecture approval |

---

### Cycle: v5-3-restore-honesty-dead-helpers

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V5-3 |
| **Goal** | Honest simulated restore messaging; delete unused presentation helpers |
| **Changes** | restore summary/banner disclose simulated; drop `stageDesktopEmptyCopy` / `activeApplicationLabel` / `applicationsLayoutsRelationCopy` / Focus partition |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary |
| **Stop** | Two empty evaluation passes pending after this commit |

---

### Cycle: v5-2-restore-refresh-apps-focus-cleanup

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V5-2 |
| **Goal** | Close Organisation→Reality loop; Apps objects focus; delete dead chrome |
| **Changes** | Stage refreshes after restore/launch (`observationEpoch`); Focus prefers selection; Apps click→`focus_desktop_window`; arrangements Stage-only; dead CSS/helpers removed |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary |
| **Human review required** | No for this slice |

---

### Cycle: v5-1-desktop-interaction-focus-ipc

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V5-1 |
| **Goal** | Stop presentation-only work; implement Desktop Interaction Layer behaviour |
| **Changes** | `focus_desktop_window` IPC (domain → kernel → Tauri); Stage click/Focus/Restore via WindowController; observation freshness before Stage read; Focus organises primary process on map + process dock; Assistant chat history thread + typing + desktop-observed ask enrichment; arrangements remounted under Stage |
| **Architecture decisions remaining** | Minimize/maximize OS APIs; Assistant-owned restore/focus (sealed); true streaming IPC; Flow/Focus OS geometry apply |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary (+ kernel desktop arrangement tests) |
| **Human review required** | No for this slice; Yes for minimize APIs / geometry apply |

---

### Cycle: v4-eval-pass-2-presentation-plateau

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V4-eval-2 |
| **Analysis** | Re-scored all V4 categories after V4-1/V4-2. No safe presentation-only change raises the lowest scores without inventing windows, OS geometry apply, grouping engines, audio, or live thumbnails. |
| **Category board (1–10)** | Desktop visibility 9 · Desktop prominence 9 · App prominence 8 · Spatial 8 · Relationships 7 · Nav 8 · Assistant 9 · Config friction 9 · Object-first 7 · Density 8 · Calmness 8 · Commercial 7 |
| **Lowest remaining** | Object-first / Relationships / Commercial — blocked on Interaction (click→OS focus), true grouping, richer capture |
| **Changes** | none |
| **Stop** | **PRESENTATION PLATEAU** — two consecutive full evaluation passes with no measurable safe improvement |
| **Human review required** | **Yes** — next product move needs architecture/workflow approval |

---

### Cycle: v4-eval-pass-1-category-rescore

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V4-eval-1 |
| **Analysis** | After V4-1/V4-2, candidate presentation edits (further text deletion, Profiles demotion, more CSS) do not measurably change the five-second “understands my computer” answer beyond current Stage-first state. |
| **Candidates considered** | Drop Profiles from primary nav; invent thumbnail placeholders; Stage click→OS focus; remove empty-plane message |
| **Rejected because** | Workflow change / fake reality / OS permission boundary / honesty required for empty observation |
| **Changes** | none |
| **Category status** | open for Interaction/Relationships systems; presentation PLATEAU pending pass 2 |

---

### Cycle: v4-2-running-apps-as-objects

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V4-2 |
| **Approved category** | 1 Visual hierarchy (+ Application prominence) |
| **Goal** | Apps tab presents running processes as objects; Stage plane larger |
| **Changes** | `ActiveApplicationsView` object grid; quieter Apps panel; larger Stage map; drop unused workMode on Apps |
| **Drift scores** | Desktop-first 9 · App prominence 8 · Object-first 7 · Dashboard-first 2 · AI-first 1 · Reference 7 · Maintainability 8 |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary |
| **Next** | Full evaluation pass — remaining gains need OS geometry / grouping / audio / live thumbnails |

---

### Cycle: v4-1-plane-first-objects-relationships

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V4-1 |
| **Approved category** | 1 Visual hierarchy (+ 5 Navigation, 12 Duplicate removal) |
| **Goal** | First five seconds = “this understands my computer”; raise Relationships + Object-first |
| **Scores before** | Relationships 4 · Object-first 5 · Commercial 6 · Nav 7 |
| **Changes** | Delete Home; Stage plane-only (Remember on Profiles); process relation accents; selectable related tiles; Tools disclosure; quieter Assistant Ask header; dead Home CSS |
| **Drift scores after (est.)** | Desktop-first 9 · Dashboard-first 2 · AI-first 1 · Spatial 8 · Config friction 9 · Desktop realism 8 · Reference 7 · Commercial 7 · Maintainability 8 |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary |
| **Human review required** | No for presentation; Yes before OS geometry / grouping engines / audio |
| **Inspector path** | Open app → Stage fills view, no Home, no Remember under Stage; click tile → related process windows highlight |

---

### Cycle: v3-2-library-simplify-dead-css

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V3-2 |
| **Approved category** | 12 Duplicate removal (+ 13 Dead code) |
| **Goal** | One library grid; remove FocusSupporting chips parallel UI + orphan CSS |
| **Problem** | Optional library still had Focus primary/chip layout; AssistantPanel/canvas CSS remained |
| **Changes** | `ApplicationList` single grid; delete `FocusSupportingAppChips`; strip dead assistant/canvas/home-chip CSS |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary |
| **Human review required** | No — continue until presentation plateau |
| **Next** | Presentation plateau unless OS geometry / grouping / audio / durable chat approved |

---

### Cycle: v3-1-desktop-first-spatial-control

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V3-1 |
| **Approved category** | 1 Visual hierarchy (+ 12 Duplicate removal, 13 Dead code) |
| **Goal** | Product Contract V3: Stage owns first glance; Focus stays spatial; Control/Assistant demoted |
| **Problem** | Arrangements co-primary rail, companion canvas second desktop, Focus list UI, Ask Assistant strip, evidence/advanced in rail, Home/Profiles form-first |
| **Analysis** | Audit vs references: objects + space before forms; delete competing surfaces |
| **Changes** | Spatial Focus CSS density; arrangements collapsed strip; remove Stage canvas; strip rail to chat; quiet Home/Profiles/nav; delete `CanvasShell` + `AssistantPanel`; dead CSS/helpers |
| **Files affected** | Stage, App, arrangements, Home, Profiles, Apps, Assistant rail/panel, workMode, layoutsStageUi, App.css, tests, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary |
| **Maintainability score** | 8/10 — fewer parallel surfaces; Stage file smaller |
| **Reference alignment** | Closer: desktop plane primary; companion optional; control secondary |
| **Human review required** | No for presentation; **Yes** before OS geometry apply / grouping / audio / durable chat |
| **Inspector / decoder path** | Open app → Stage fills view; Focus dims non-focused tiles; Remember layout collapsed; Assistant only via chrome |

---

### Cycle: dil-1-assistant-companion-chat

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | DIL-1 |
| **Approved category** | 1 Visual hierarchy (+ 16 Maintainability) |
| **Goal** | Usable Assistant companion: input, send, answer body, recent — AI stays secondary |
| **Problem** | Rail exposed six evidence layers; utterance.body never shown; ask felt diagnostic |
| **Analysis** | Reuse `compose_workspace_assistant_turn` only on send; session recent; quiet Desktop profile ensure |
| **Changes** | Companion chat UI; evidence packages collapsed; `assistantCompanion.ts`; rail header quieted |
| **Files affected** | `AssistantIntelligencePanel.tsx`, `AssistantCompanionRail.tsx`, `assistantCompanion.ts`, `App.tsx`, `App.css`, tests, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary |
| **Maintainability score** | 8/10 — one compose path; helpers pure |
| **Human review required** | No — continue DIL slices |
| **Inspector / decoder path** | Open Ask Assistant → type → Send → answer body in thread; Recent expands |

---

### Cycle: dil-2-anti-dashboard-apps-objects

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | DIL-2 |
| **Approved category** | 5 Navigation clarity (+ 12 Duplicate removal) |
| **Goal** | Remove Home dashboard chrome; Applications observed-first; Stage app-object labels; drop duplicate Flow/Focus |
| **Problem** | Home “Go to” cards + roadmap; Apps library-first; Stage duplicated mode switch; window titles over apps |
| **Analysis** | Hierarchy: Desktop → Running apps; profiles/library optional; shell owns Flow/Focus |
| **Changes** | Slim Home; Apps Running now + library `<details>`; Stage process-primary tiles; remove stage WorkModeSwitch + unused zoneCount; dead home CSS |
| **Files affected** | `WorkspaceHome.tsx`, `ApplicationsPanel.tsx`, `WorkspaceApplicationStage.tsx`, `stageDesktopUi.ts`, `applicationsUi.ts`, `App.tsx`, `App.css`, tests, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary |
| **Maintainability score** | 8/10 — fewer props/controls; clearer Stage identity |
| **Human review required** | **Yes** — next steps need architecture/workflow: OS Flow/Focus geometry apply, window grouping, audio mixer domain, or durable assistant history IPC |
| **Inspector / decoder path** | Home → Open Stage only; Applications → Running now first; Stage tiles show process names |

---

### Cycle: im-2-desktop-reality-first

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | IM-2 |
| **Approved category** | 5 Navigation clarity (+ 1 Visual hierarchy) |
| **Goal** | Observe → Represent before Ask/Configure; less setup before value |
| **Problem** | Assistant default-open + create-workspace empty copy competed with Stage |
| **Analysis** | Interaction Model IM-2; preference default + copy; no engines |
| **Changes** | `DEFAULT_ASSISTANT_RAIL_OPEN = false`; desktop-first Assistant/arrangements/profiles copy; Profiles nav label |
| **Files affected** | `assistantRail.ts`, Assistant panels, `desktopArrangementUi`, `WorkspaceSwitcher`, `App.tsx`, tests, IM-2 reports, this log |
| **Validation** | typecheck / test (125) / build / architecture / ipc / ui-boundary |
| **Maintainability score** | 8/10 — defaults + pure copy helpers |
| **Human review required** | **Yes** — stop before IM-3 |
| **Next recommended cycle** | Await IM-2 approval → IM-3 arrangements remember-control (if still needed after copy) |
| **Inspector / decoder path** | Fresh load / clear rail preference → Stage without Assistant column |

---

### Cycle: im-1-stage-empty-spatial-calm

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | IM-1 |
| **Approved category** | 1 Visual hierarchy (+ 2 Layout consistency) |
| **Goal** | Stage empty state reads as a calm desktop plane (layout before text) |
| **Problem** | Observation-backed Stage still used paragraph empty states → dashboard/help feel |
| **Analysis** | Interaction Model Audit IM-1; presentation-only; reuse existing load states |
| **Changes** | Empty desktop plane UI; one-line `stageDesktopPlaneMessage`; compact hero; library `<details>`; quiet arrangements via `:has(.stage-plane-calm)` |
| **Files affected** | `WorkspaceApplicationStage.tsx`, `stageDesktopUi.ts`, `layoutsStageUi.ts`, `App.css`, tests, IM-1 reports, this log |
| **Validation** | typecheck / test (125) / build / architecture / ipc / ui-boundary |
| **Maintainability score** | 8/10 — extended Stage; single copy owner; no new engines |
| **Reference / contract alignment** | Improved first-glance spatial calm; Assistant default still open (IM-2) |
| **Human review required** | **Yes** — stop before IM-2 |
| **Remaining risks** | Preview has no windows so plane message always shows; live Windows map path unchanged |
| **Next recommended cycle** | Await IM-1 approval → IM-2 Assistant optional start |
| **Inspector / decoder path** | Open Stage → large empty plane + one line; expand Library details |

---

### Cycle: opt-v2-26-full-pass-2-global-plateau

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Category** | Full-category evaluation pass 2 |
| **Analysis** | Re-checked all 26 PLATEAUED categories. No new measurable safe improvement without inventing product direction (bottom nav) or crossing boundaries (OS geometry on mode switch). |
| **Changes** | none |
| **Files changed** | this log |
| **Validation** | reaffirm tip green |
| **Stop** | **GLOBAL PLATEAU** under Protocol v2 — all categories PLATEAUED + two consecutive full passes with no measurable improvement |
| **Why this is safe** | Stop record only |

---

### Cycle: opt-v2-25-full-pass-1-all-categories

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Category** | Full-category evaluation pass 1 |
| **Analysis** | Every approved category evaluated; remaining open items closed as PLATEAUED after confirming no measurable safe chrome/quality work remains without boundary stops. |
| **Changes** | Category board → all PLATEAUED |
| **Files changed** | this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build |
| **Category status** | all PLATEAUED |
| **Why this is safe** | Evaluation pass |

---

### Cycle: opt-v2-24-dead-application-list-row

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Category** | 13 — Dead code removal |
| **Problem** | `.application-list-row` CSS had no TSX consumers after card-grid migration |
| **Reason** | Remove dead selector + media rule |
| **Files changed** | `App.css`, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build |
| **Category status** | PLATEAUED (with v2-25 board close) |
| **Why this is safe** | Dead CSS only |

---

### Cycle: opt-v2-23-plateau-nav-keyboard-naming-errors

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Category** | 5, 7, 15, 25 |
| **Analysis** | Navigation: Assistant toggle labelled; bottom-nav IA is a human product decision (boundary). Keyboard: Flow/Focus radiogroup + rail focus/Escape/return complete for current chrome. Naming: preference keys, rail id, preview banner named. Errors: classifyBanner handles strings + runtime preview copy. |
| **Changes** | Mark 5, 7, 15, 25 **PLATEAUED** |
| **Files changed** | this log |
| **Category status** | PLATEAUED |
| **Why this is safe** | Evaluation only; no speculative chrome |

---

### Cycle: opt-v2-22-arrangement-pad-and-chip-test

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Category** | 3 — Spacing consistency (+ 20 test quality) |
| **Problem** | Arrangement rail padding still used literal rems; Focus chips module untested as a contract |
| **Reason** | Use `--rail-pad-*` tokens; assert FocusSupportingAppChips export |
| **Files changed** | `App.css`, tests, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build |
| **Category status** | open |
| **Why this is safe** | Token alignment + import test only |

---

### Cycle: opt-v2-21-maintainability-commercial-pad

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Category** | 16 — Maintainability (+ 26 commercial readiness) |
| **Problem** | Shell helper header omitted banner constant; reopen bar padding used a one-off rem |
| **Reason** | Document `DESKTOP_PREVIEW_BANNER` in file purpose; align reopen bar to `--shell-pad-x` |
| **Files changed** | `productShellUi.ts`, `App.css`, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build |
| **Maintainability score** | 9.2/10 |
| **Commercial readiness** | 6.5/10 |
| **Category status** | open |
| **Why this is safe** | Header + padding alignment only |

---

### Cycle: opt-v2-20-plateau-animation

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Category** | 9 — Animation polish |
| **Analysis** | Companion rail enter + `prefers-reduced-motion` already shipped (v2-08). Additional chrome animation would be decorative without hierarchy gain (anti-slop). |
| **Changes** | Mark category 9 **PLATEAUED** |
| **Files changed** | this log |
| **Category status** | PLATEAUED |
| **Why this is safe** | Evaluation only |

---

### Cycle: opt-v2-19-ghost-focus-visible

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Category** | 6 — Accessibility (+ 10 component consistency) |
| **Problem** | Ghost / rail control buttons relied only on generic `button:focus-visible` without explicit companion selectors for review clarity |
| **Reason** | Name companion controls in the shared focus-visible group |
| **Files changed** | `App.css`, this log |
| **Validation** | typecheck / test / architecture / ipc / ui-boundary / build |
| **Accessibility** | 8.7/10 |
| **Category status** | open |
| **Why this is safe** | Focus ring selectors only |

---

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
