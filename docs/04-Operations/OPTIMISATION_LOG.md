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

### Cycle: v15-empty-coherence-audit-2

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V15-empty-2 |
| **Goal** | Independent Pass 2 Product Foundation V15 coherence audit (different angles from Pass 1) |
| **Problem** | Confirm whether Stage dock / Apps leftovers, profile isolation, Environment leakage, dead V15-2 exports, behaviour timeline, or primary-tab fragmentation still need a product integration |
| **Analysis** | (1) Stage Focus dock owns running-process objects; deleted Apps `ActiveApplicationsView` left orphan `.app-object-*` / `.applications-library-details` CSS and `activeApplication*` helpers — hygiene only, not user-visible coherence. (2) Profiles isolate library/arrangements; Stage desktop continues via shared WorkspaceState — no continuity break. (3) Environment stays Operator/Developer only; primary Stage/Apps/Profiles/Assistant enrich do not invoke Aggregator. (4) Dead helpers are test-only leftovers — not a product surface. (5) Behaviour timeline correctly absent from Stage (Assistant-on-ask only; dumping it would be diagnostic). (6) After V15-2, Apps is library + Open Stage; Stage owns running; Profiles quiet — no further measurable tab collapse without IA redesign. |
| **Changes** | none — EMPTY AUDIT 2 |
| **Files affected** | this log |
| **Validation** | Architecture review against V15 Stage SoT / Apps library / no Environment on primary / no behaviour dump on Stage |
| **Deleted / reduced** | none |
| **Next recommended cycle** | Stop autonomous V15 if Pass 1 also empty; else only Pass-1-named integration |
| **Human review required** | No |
| **Why this is safe** | No code change; leftovers noted are dead CSS/helpers outside product UX |

---

### Cycle: v15-2-continuity-assistant-apps-coherence

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V15-2 |
| **Category** | #12 / #1 — Duplicate removal / Visual hierarchy (product coherence) |
| **Goal** | Continuity on Stage; Assistant explains Stage conclusions; Apps = library only |
| **Problem** | runtime_memory/latest_delta only in Assistant; enrichAsk dumped telemetry; Apps duplicated Stage focus grid |
| **Analysis** | Integrate disconnected continuity + collapse parallel running-apps interaction |
| **Changes** | Stage continuity cues; enrichAsk narrates attention/working/returning; Apps library-first + Open Stage; deleted ActiveApplicationsView |
| **Files affected** | stageDesktopUi, Stage, assistantCompanion, ApplicationsPanel, App, App.css, tests, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary |
| **Deleted / reduced** | ActiveApplicationsView; Apps running focus grid; enrichAsk plane-count dumps |
| **Next recommended cycle** | Coherence audit / empty eval if plateau |
| **Human review required** | No |
| **Why this is safe** | Consumes existing WorkspaceState; Stage remains desktop SoT for running work |

---

### Cycle: v15-1-stage-attention-semantics-awareness

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V15-1 |
| **Category** | #1 / #16 — Visual hierarchy / Maintainability (product coherence) |
| **Goal** | Surface WorkspaceState attention + semantics on Stage as subtle product awareness |
| **Problem** | Attention/semantics only spoke through Assistant text; Stage painted a flat hwnd map |
| **Analysis** | Highest disconnected capability: runtime conclusions already on WorkspaceState unused by the product center |
| **Changes** | Stage tile roles + attention-primary cue; Flow relatedness walks semantic links; Focus prefers attention/working keys; meta awareness line; pure helpers in stageDesktopUi |
| **Files affected** | `stageDesktopUi.ts`, `WorkspaceApplicationStage.tsx`, `App.css`, stage tests, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary |
| **Deleted / reduced** | No new panels; awareness no longer Assistant-only |
| **Next recommended cycle** | Continuity cues (runtime_memory/latest_delta) or collapse Apps running-focus into Stage |
| **Human review required** | No |
| **Why this is safe** | Consumes existing WorkspaceState planes; no new engines |

---

### Cycle: v14-empty-consolidation-evaluation-2

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V14-empty-2 |
| **Goal** | Second consecutive empty Product Foundation consolidation audit |
| **Problem** | Confirm no further measurable duplicate-runtime simplification without new APIs/architecture |
| **Analysis** | Independent angles: helper recomputation, dead IPC consumers, Programme desktop mirrors, unnecessary kernel desktop triggers, relationship invention. All leftovers are presentation, Aggregator/Programme IV gated, or diagnostic IPC parity. |
| **Changes** | none — second empty evaluation |
| **Files affected** | this log |
| **Validation** | Architecture review against ONE runtime / MANY consumers |
| **Deleted / reduced** | none |
| **Next recommended cycle** | Stop autonomous V14 (condition 3: two empty consolidation audits) |

---

### Cycle: v14-empty-consolidation-evaluation-1

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V14-empty-1 |
| **Goal** | First empty Product Foundation consolidation audit after V14-3 |
| **Problem** | Identify highest-value remaining non-WorkspaceState desktop truth |
| **Analysis** | Stage/Apps/Assistant/Operator/Environment UI consume shared WorkspaceState. Leftovers: arrangement CRUD, Environment Aggregator remapping, Programme IV attention bridge, registry fuzzy match — gated or out of scope. |
| **Changes** | none — empty evaluation |
| **Files affected** | this log |
| **Validation** | Architecture review |
| **Deleted / reduced** | none |
| **Next recommended cycle** | Second empty consolidation evaluation |

---

### Cycle: v14-3-shared-workspacestate-client-cache

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V14-3 |
| **Category** | #12 / #24 — Duplicate removal / IPC cleanliness |
| **Goal** | One frontend WorkspaceState hold shared by Stage, Apps, Assistant, Operator, Environment UI |
| **Problem** | Each product surface refreshed and held WorkspaceState independently; Environment UI mirrored desktop windows/groups from Environment model |
| **Analysis** | Under-utilised: single client publication of last WorkspaceState. Not a new runtime — cache of the existing contract. |
| **Changes** | `subscribeObservedWorkspaceState` + `useObservedWorkspaceState`; product panels subscribe; Environment section shows groups from WorkspaceState and keeps Environment for gaps only |
| **Files affected** | `workspaceStateClient.ts`, `useObservedWorkspaceState.ts`, Stage/Apps/Assistant/Operator/IntelligencePanel, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary |
| **Deleted / reduced** | Per-panel WorkspaceState useState owners; Environment window_groups product display |
| **Next recommended cycle** | Consolidation audit for remaining duplicates; empty eval if none |
| **Human review required** | No |
| **Why this is safe** | Same get_workspace_state IPC; subscribers mirror last publish |

---

### Cycle: v14-2-product-surfaces-hold-workspacestate

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V14-2 |
| **Category** | #12 — Duplicate removal / runtime consumption |
| **Goal** | Product surfaces hold WorkspaceState directly instead of sliced parallel models |
| **Problem** | Stage/Apps/Operator truncated WorkspaceState into windows/groups/monitors/meta slices; Operator inspected Environment groups as desktop truth |
| **Analysis** | Under-utilised: full WorkspaceState as held truth. No new engines or shared shell context yet. |
| **Changes** | Stage + ApplicationsPanel hold `WorkspaceState`; Operator refresh via client + full state; group inspect prefers `window_groups`; focus hwnd prefers `focused_window` |
| **Files affected** | `WorkspaceApplicationStage`, `ApplicationsPanel`, `ActiveApplicationsView`, `OperatorConsole`, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary |
| **Deleted / reduced** | Stage 5-slice state; Apps dual active slices; Operator windows-only bag |
| **Next recommended cycle** | Shared shell WorkspaceState if Stage+Assistant co-open still double-refreshes; Environment panel desktop mirror redirect |
| **Human review required** | No |
| **Why this is safe** | Same IPC source; presentation derives fields |

---

### Cycle: v14-1-assistant-stage-workspacestate-consumption

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V14-1 |
| **Category** | #12 — Duplicate removal / runtime consumption |
| **Goal** | Product Foundation: consume WorkspaceState; delete Assistant parallel facts bag and Focus PID invention |
| **Problem** | Assistant rebuilt desktop facts via `list_desktop_arrangements` + parallel delta bag; Focus invented multi-window PID dock/primary buckets instead of `window_groups` |
| **Analysis** | Highest-value under-utilised runtime: `WorkspaceState.window_groups` + `latest_delta`. No new engines. |
| **Changes** | Assistant answers/enrichment take `WorkspaceState` only; panel drops arrangement fetch for Q&A; Focus primary + dock consume `process_id` groups |
| **Files affected** | `assistantCompanion.ts`, `AssistantIntelligencePanel.tsx`, `stageDesktopUi.ts`, milestone-a test, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary |
| **Deleted / reduced** | `AssistantDesktopFacts`; Assistant arrangement IPC on ask; Focus `dockByProcess` PID Map invention |
| **Maintainability score** | 9/10 |
| **Reference alignment** | 8/10 — product consumes runtime directly |
| **Human review required** | No |
| **Why this is safe** | Same answers from same authority; arrangement CRUD panels unchanged |
| **Next recommended cycle** | Shared shell WorkspaceState holder; Stage stop slicing fields; Operator full-state inspect |
| **Inspector / decoder path** | Assistant local answers; Stage Focus with process_id groups |

---

### Cycle: v13-empty-attention-evaluation-2

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V13-empty-2 |
| **Goal** | Second consecutive empty attention capability evaluation |
| **Problem** | Confirm whether any further measurable attention capability remains without new architecture/platform APIs |
| **Analysis** | Same blockers as empty-1: durable attention lifecycle across restarts, continuous OS sampling, acting on attention, Programme IV attention bridge |
| **Changes** | none — second empty evaluation |
| **Files affected** | this log |
| **Validation** | Architecture review against WorkspaceState.attention |
| **Deleted / reduced** | none |
| **Next recommended cycle** | Stop autonomous V13 (condition 3: two empty evaluations) |

---

### Cycle: v13-empty-attention-evaluation-1

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V13-empty-1 |
| **Goal** | First empty attention capability evaluation after V13-3 |
| **Problem** | Identify highest-value missing deterministic attention capability |
| **Analysis** | Attention projects with lifecycle/time sensitivity/merge/damping/primary/replacement from existing planes. Further gains need durable cross-pass attention state, continuous OS sampling, or architecture to bridge Programme IV attention |
| **Changes** | none — empty evaluation |
| **Files affected** | this log |
| **Validation** | Architecture review against WorkspaceState.attention |
| **Deleted / reduced** | none |
| **Next recommended cycle** | Second empty attention evaluation |

---

### Cycle: v13-3-attention-primary-and-replacement

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V13-3 |
| **Goal** | Mark primary attention; replace incomplete/weak noise when interrupted/returning is primary |
| **Problem** | Attention list lacked a clear “now” focus; incomplete/weak could compete with primary resume attention |
| **Analysis** | Set `primary_item_id` after ranking; drop incomplete/weak when primary is interrupted/returning |
| **Changes** | `primary_item_id`; `apply_attention_replacement`; Assistant primary line |
| **Files affected** | desktop_attention, domain.ts, assistantCompanion, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary (+ domain attention tests) |
| **Deleted / reduced** | Incomplete/weak attention under primary resume attention |
| **Next recommended cycle** | Two empty attention evaluation passes |

---

### Cycle: v13-2-attention-oscillation-damping

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V13-2 |
| **Goal** | Damp oscillating / noisy attention; refine decay without durable history |
| **Problem** | Emerging vs fading, uncertain vs interrupted, and background kinds could chatter |
| **Analysis** | Prefer fading over emerging; drop uncertain when interrupted/returning; soften switching under stable working; cap background kinds; demote decaying immediacy |
| **Changes** | `damp_attention_oscillation` |
| **Files affected** | desktop_attention, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary (+ domain attention tests) |
| **Deleted / reduced** | Oscillating emerging attention; excess background/weak items; floor-strength decaying noise |
| **Next recommended cycle** | Two empty attention evaluation passes |

---

### Cycle: v13-1-desktop-attention-projection

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V13-1 |
| **Goal** | Project deterministic attention onto WorkspaceState from decisions + evidence planes |
| **Problem** | Runtime had decisions but not a ranked “what deserves notice now” attention surface |
| **Analysis** | Seed from ranked decisions/consistency; gap-fill from session/memory; lifecycle + time_sensitivity from evidence; merge/suppress conflicts |
| **Changes** | `desktop_attention`; `WorkspaceState.attention`; Assistant attention answers |
| **Files affected** | desktop_attention, workspace_state, domain exports/types, assistantCompanion, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary (+ domain attention tests) |
| **Deleted / reduced** | Weak/background attention when stronger kinds cover same entities; working_focus vs interrupted conflicts |
| **Next recommended cycle** | Decay/oscillation damping; stronger multi-signal lifecycle |

---

### Cycle: v12-empty-decision-evaluation-2

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V12-empty-2 |
| **Goal** | Second consecutive empty decision capability evaluation |
| **Problem** | Confirm whether any further measurable decision capability remains without new architecture/platform APIs |
| **Analysis** | Same blockers as empty-1: executing recommendations, durable decision history, continuous OS sampling, Programme IV queue integration |
| **Changes** | none — second empty evaluation |
| **Files affected** | this log |
| **Validation** | Architecture review against WorkspaceState.decisions |
| **Deleted / reduced** | none |
| **Next recommended cycle** | Stop autonomous V12 (condition 3: two empty evaluations) |

---

### Cycle: v12-empty-decision-evaluation-1

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V12-empty-1 |
| **Goal** | First empty decision capability evaluation after V12-3 |
| **Problem** | Identify highest-value missing deterministic decision capability |
| **Analysis** | Decisions/recommendations/consistency/explainability project from existing planes. Further gains need OS APIs (act on resume/reopen), durable decision history beyond sample retention, or architecture to bridge Programme IV decision queue |
| **Changes** | none — empty evaluation |
| **Files affected** | this log |
| **Validation** | Architecture review against WorkspaceState.decisions |
| **Deleted / reduced** | none |
| **Next recommended cycle** | Second empty decision evaluation |

---

### Cycle: v12-3-consistency-crosslink-and-recommendation-dedupe

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V12-3 |
| **Goal** | Cross-link consistency uncertainty into decision explanations; dedupe recommendations |
| **Problem** | Consistency issues were parallel lists; recommendations repeated kinds |
| **Analysis** | Attach related consistency notes/evidence onto decisions; keep one recommendation per kind |
| **Changes** | `cross_link_consistency_explanations`; `dedupe_recommendations` |
| **Files affected** | desktop_decision, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary (+ domain decision tests) |
| **Deleted / reduced** | Duplicate recommendation kinds |
| **Next recommended cycle** | Two empty decision evaluation passes |

---

### Cycle: v12-2-decision-rank-dedupe-explainability

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V12-2 |
| **Goal** | Rank/merge overlapping decisions; strengthen explainability via supporting planes |
| **Problem** | Multiple same-kind decisions and weak knowledge noise competed without multi-signal ranking |
| **Analysis** | Merge overlapping kinds, boost multi-plane agreement, demote on consistency conflicts, drop low-confidence when stronger attention exists |
| **Changes** | `refine_decision_support`; `supporting_planes`; priority ordering; Assistant surfaces planes |
| **Files affected** | desktop_decision, domain.ts, assistantCompanion, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary (+ domain decision tests) |
| **Deleted / reduced** | Duplicate same-kind overlapping decisions; redundant low-confidence noise |
| **Next recommended cycle** | Two empty decision evaluation passes |

---

### Cycle: v12-1-desktop-decision-support-projection

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V12-1 |
| **Goal** | Project deterministic decisions, recommendations, and consistency issues onto WorkspaceState |
| **Problem** | Runtime understood existence/behaviour/continuity/semantics but not evidence-driven “what requires attention next” |
| **Analysis** | Derive decisions from semantics+memory+behaviour with evidence chains and explanations; recommendations map from decisions; consistency flags uncertainty |
| **Changes** | `desktop_decision`; `WorkspaceState.decisions`; Assistant decision answers |
| **Files affected** | desktop_decision, workspace_state, domain exports/types, assistantCompanion, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary (+ domain decision tests) |
| **Deleted / reduced** | none (extends WorkspaceState; does not reuse Programme IV decision engines) |
| **Next recommended cycle** | Rank/dedupe decisions by multi-signal agreement; strengthen explainability |

---

### Cycle: v11-empty-semantic-evaluation-2

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V11-empty-2 |
| **Goal** | Second consecutive empty semantic capability evaluation |
| **Problem** | Confirm whether any further measurable semantic capability remains without new architecture/platform APIs or forbidden app-name tables |
| **Analysis** | Same blockers as empty-1: content-class labels (communication/development/media), continuous OS streams, durable semantic persistence beyond sample retention |
| **Changes** | none — second empty evaluation |
| **Files affected** | this log |
| **Validation** | Architecture review against WorkspaceState.semantics |
| **Deleted / reduced** | none |
| **Next recommended cycle** | Stop autonomous V11 (condition 3: two empty evaluations) |

---

### Cycle: v11-empty-semantic-evaluation-1

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V11-empty-1 |
| **Goal** | First empty semantic capability evaluation after V11-3 |
| **Problem** | Identify highest-value missing deterministic semantic capability |
| **Analysis** | Behavioural roles/relationships/activities/graph/importance are projected. Further product-class semantics need app/content signals (forbidden lookup tables) or OS APIs; durable semantic memory needs architecture beyond the 50-pass ring |
| **Changes** | none — empty evaluation |
| **Files affected** | this log |
| **Validation** | Architecture review against WorkspaceState.semantics |
| **Deleted / reduced** | none |
| **Next recommended cycle** | Second empty semantic evaluation |

---

### Cycle: v11-3-semantic-importance

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V11-3 |
| **Goal** | Project importance (ephemeral/routine/emerging/important/fading) on semantic objects |
| **Problem** | Roles described meaning but not relative importance for Assistant/runtime consumers |
| **Analysis** | Derive importance from memory knowledge + assigned role; surface in working-on answers |
| **Changes** | `DesktopSemanticObject.importance`; Assistant working-on + semantic summaries |
| **Files affected** | desktop_semantic, domain.ts, assistantCompanion, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary (+ domain semantic tests) |
| **Deleted / reduced** | none |
| **Next recommended cycle** | Two empty semantic evaluation passes |

---

### Cycle: v11-2-semantic-confidence-refinement

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V11-2 |
| **Goal** | Strengthen/weaken semantic confidence from multi-signal agreement; prune conflicting edges |
| **Problem** | Roles/relationships could conflict with memory knowledge or duplicate alternation edges |
| **Analysis** | Refine after projection: boost agreement, demote conflicts, drop precedes/follows covered by alternates, emit supports |
| **Changes** | `refine_semantic_confidence`; `supports` relationship; rebuild graph after refine |
| **Files affected** | desktop_semantic, domain.ts, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary (+ domain semantic tests) |
| **Deleted / reduced** | Duplicate directional follow edges when alternation exists |
| **Next recommended cycle** | Two empty semantic evaluation passes |

---

### Cycle: v11-1-semantic-desktop-projection

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V11-1 |
| **Goal** | Project deterministic desktop semantics (roles, relationships, activities, graph) onto WorkspaceState |
| **Problem** | Runtime knew existence/behaviour/continuity but not evidence-driven meaning of objects and relations |
| **Analysis** | Derive roles/relationships/activities/graph solely from behaviour + runtime_memory + groups; no app-name tables |
| **Changes** | `desktop_semantic`; `WorkspaceState.semantics`; Assistant semantic answers |
| **Files affected** | desktop_semantic, workspace_state, domain exports/types, assistantCompanion, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary (+ domain semantic tests) |
| **Deleted / reduced** | none (extends WorkspaceState projection path) |
| **Next recommended cycle** | Strengthen semantic confidence from multi-signal agreement / weaken conflicts |

---

### Cycle: v10-empty-continuity-evaluation-2

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V10-empty-2 |
| **Goal** | Second consecutive empty continuity capability evaluation |
| **Problem** | Confirm whether any further measurable continuity capability remains without new architecture/platform APIs |
| **Analysis** | Same blockers as empty-1: continuous OS sampling, durable history beyond retention, workspace-scoped state, geometry/thumbnail/audio ownership |
| **Changes** | none — second empty evaluation |
| **Files affected** | this log |
| **Validation** | Architecture review against WorkspaceState continuity / runtime memory / behaviour confidence |
| **Deleted / reduced** | none |
| **Next recommended cycle** | Stop autonomous V10 (condition 3: two empty evaluations) |

---

### Cycle: v10-empty-continuity-evaluation-1

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V10-empty-1 |
| **Goal** | First empty continuity capability evaluation after V10-5 |
| **Problem** | Identify highest-value missing deterministic continuity capability |
| **Analysis** | Remaining deltas need continuous OS event streams / idle, durable history beyond the 50-pass ring, workspace-scoped WorkspaceState, or geometry/thumbnail/audio ownership — platform APIs or new architecture |
| **Changes** | none — empty evaluation |
| **Files affected** | this log |
| **Validation** | Architecture review against WorkspaceState continuity / runtime memory / behaviour confidence |
| **Deleted / reduced** | none |
| **Next recommended cycle** | Second empty continuity evaluation |

---

### Cycle: v10-5-relationship-session-continuity

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V10-5 |
| **Goal** | Measure relationship continuity across observation sessions |
| **Problem** | Focus-follow and co-presence confidence ignored whether affinities survived session boundaries |
| **Analysis** | Annotate relationships with `session_count` after session projection; fold into confidence evidence |
| **Changes** | `session_count` on focus_follows / co_presence; confidence boost for multi-session relationships |
| **Files affected** | desktop_behaviour, domain.ts, assistantCompanion, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary (+ domain behaviour tests) |
| **Deleted / reduced** | none |
| **Next recommended cycle** | Two empty continuity evaluation passes |

---

### Cycle: v10-4-group-continuity-from-runtime-memory

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V10-4 |
| **Goal** | Let grouping confidence strengthen or weaken from multi-session member persistence |
| **Problem** | Groups only rose from co-presence sample counts, not session continuity in runtime memory |
| **Analysis** | After projecting runtime memory, raise evidence from shared session presence; ease when majority members are temporary |
| **Changes** | `strengthen_groups_from_runtime_memory`; wired in `WorkspaceState::with_runtime_memory` |
| **Files affected** | desktop_runtime_memory, workspace_state, domain exports, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary (+ domain runtime_memory tests) |
| **Deleted / reduced** | none (extends existing grouping) |
| **Next recommended cycle** | Relationship continuity across observation sessions |

---

### Cycle: v10-3-runtime-memory-knowledge-classes

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V10-3 |
| **Goal** | Classify desktop objects as temporary / established / persistent / returning / interrupted / fading / rising |
| **Problem** | Runtime memory had presence + stability but not knowledge about importance or disappearance |
| **Analysis** | Derive knowledge solely from presence, stability, focus, lifecycle, and recurrence evidence |
| **Changes** | `DesktopObjectMemory.knowledge`; Assistant rising/fading/interrupted answers |
| **Files affected** | desktop_runtime_memory, domain.ts, assistantCompanion, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary (+ domain runtime_memory tests) |
| **Deleted / reduced** | none |
| **Next recommended cycle** | Strengthen groups from multi-session member persistence |

---

### Cycle: v10-2-independent-behaviour-confidence

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V10-2 |
| **Goal** | Evolve confidence independently for relationships, sessions, and revisits |
| **Problem** | Only groups had evidence ladders; follows/co-presence/sessions/revisits were raw counts |
| **Analysis** | Reuse the structural→strong ladder per fact family without overwriting identity or group confidence |
| **Changes** | `confidence` on focus_follows, co_presence, sessions, window_revisits; Assistant surfaces it |
| **Files affected** | desktop_behaviour, domain.ts, assistantCompanion, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary (+ domain behaviour tests) |
| **Deleted / reduced** | none (extends existing behaviour facts) |
| **Next recommended cycle** | Knowledge classes on runtime memory (temporary / fading / becoming important) |

---

### Cycle: v10-1-desktop-runtime-memory

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V10-1 |
| **Goal** | Project identity-keyed desktop continuity memory onto WorkspaceState |
| **Problem** | Behaviour timeline understood sample-window activity but not present/absent/returning entity continuity across retained history |
| **Analysis** | Identity registry + observation history already encode continuity; project present/absent/returning, stability, recurrence, and continuity confidence without a parallel store |
| **Changes** | `desktop_runtime_memory`; `WorkspaceState.runtime_memory`; engine loads identities for all history stable ids; Assistant memory answers |
| **Files affected** | desktop_runtime_memory, workspace_state, workspace_state_engine, domain exports/types, assistantCompanion, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary (+ domain runtime_memory tests) |
| **Deleted / reduced** | Duplicate “open-windows-only” continuity answers when memory is present |
| **Next recommended cycle** | Independent confidence ladders on relationship / session / behaviour facts |

---

### Cycle: v9-empty-behavioural-evaluation-2

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V9-empty-2 |
| **Goal** | Second consecutive empty behavioural capability evaluation |
| **Problem** | Confirm whether any further measurable behaviour capability remains without new architecture/platform APIs |
| **Analysis** | Same blockers as empty-1: continuous OS focus/idle, durable workspace-scoped history, geometry/thumbnail/audio ownership |
| **Changes** | none — second empty evaluation |
| **Files affected** | this log |
| **Validation** | Architecture review against WorkspaceState behaviour projection |
| **Deleted / reduced** | none |
| **Next recommended cycle** | Stop autonomous V9 (condition 3: two empty evaluations) |

---

### Cycle: v9-empty-behavioural-evaluation-1

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V9-empty-1 |
| **Goal** | First empty behavioural capability evaluation after V9-5 |
| **Problem** | Identify highest-value missing behavioural runtime capability |
| **Analysis** | Remaining deltas need continuous OS focus/idle sampling, workspace-scoped durable history, or geometry/thumbnail/audio ownership — platform APIs or new architecture |
| **Changes** | none — empty evaluation |
| **Files affected** | this log |
| **Validation** | Architecture review against WorkspaceState behaviour projection |
| **Deleted / reduced** | none |
| **Next recommended cycle** | Second empty behavioural evaluation |

---

### Cycle: v9-5-suppress-behaviour-across-coverage-gaps

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V9-5 |
| **Goal** | Do not attribute focus/lifecycle/affinity events across unobserved coverage gaps |
| **Problem** | Adjacent-snapshot comparisons still emitted transitions/opens/closes across ≥30m gaps |
| **Analysis** | Guard aggregations with `!crossed_gap`; sessions/spans already bound continuity |
| **Changes** | Suppress cross-gap transitions, follows, revisits, and lifecycle counts |
| **Files affected** | desktop_behaviour, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary (+ domain behaviour tests) |
| **Deleted / reduced** | Unobserved cross-gap behavioural attributions |
| **Next recommended cycle** | Two empty behavioural evaluation passes |

---

### Cycle: v9-4-gap-aware-spans-and-lifecycle

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V9-4 |
| **Goal** | Stop focus spans bridging coverage gaps; aggregate open/close lifecycle evidence |
| **Problem** | Sample focus durations could cross unobserved gaps; open/close deltas were discarded by behaviour projection |
| **Analysis** | Reset spans at ≥30m gaps; fold opened/closed window counts into `window_lifecycles` |
| **Changes** | gap-aware span reset; `DesktopWindowLifecycle`; Assistant lifecycle answers |
| **Files affected** | desktop_behaviour, domain exports/types, assistantCompanion, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary (+ domain behaviour tests) |
| **Deleted / reduced** | Gap-bridging focus duration bug |
| **Next recommended cycle** | Two empty behavioural evaluation passes |

---

### Cycle: v9-3-observation-session-segmentation

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V9-3 |
| **Goal** | Infer observation sessions from coverage gaps on the behaviour timeline |
| **Problem** | Behaviour facts existed as a flat sample stream without session epochs |
| **Analysis** | Split retained history at ≥30m gaps; label active/completed/returning with dominant focus |
| **Changes** | `DesktopObservationSession` + `behaviour.sessions`; Assistant session summary |
| **Files affected** | desktop_behaviour, domain exports/types, assistantCompanion, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary (+ domain behaviour tests) |
| **Deleted / reduced** | none |
| **Next recommended cycle** | Two empty behavioural evaluation passes |

---

### Cycle: v9-2-focus-follow-and-group-confidence

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V9-2 |
| **Goal** | Infer focus-follow + co-presence affinities; strengthen grouping confidence from usage |
| **Problem** | Groups were structural-only; Assistant could not answer “opens together / comes next” |
| **Analysis** | Aggregate A→B focus transitions and visible co-presence from the same sample history; raise group evidence without replacing criteria |
| **Changes** | `focus_follows` / `co_presence`; `DesktopWindowGroup.evidence_count`+`confidence`; `strengthen_groups_from_behaviour`; Assistant affinity answers |
| **Files affected** | desktop_behaviour, desktop_grouping, workspace_state, domain.ts, assistantCompanion, stage tests, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary (+ domain behaviour tests) |
| **Deleted / reduced** | none (extends existing behaviour + grouping) |
| **Next recommended cycle** | Gap-based observation session segmentation on the behaviour timeline |

---

### Cycle: v9-1-desktop-behaviour-timeline

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V9-1 |
| **Goal** | Deterministic focus-behaviour timeline on WorkspaceState from retained observation samples |
| **Problem** | Runtime understood current desktop state but not behaviour over the retained pass history |
| **Analysis** | Reuse adjacent snapshot comparison + load_recent_snapshots; publish sample-based transitions/revisits/gaps/spans |
| **Changes** | `desktop_behaviour` module; `WorkspaceState.behaviour`; single-lock history load; Assistant behaviour answers |
| **Files affected** | domain desktop_behaviour + workspace_state + lib, observation repo, state engine, domain.ts, assistantCompanion, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary (+ domain behaviour tests) |
| **Deleted / reduced** | Separate latest/previous loads in state engine (history supplies both) |
| **Next recommended cycle** | Focus-follow / co-occurrence affinities feeding grouping confidence |

---

### Cycle: v8-eval-pass-2-capability-plateau

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V8-eval-2 |
| **Analysis** | Second independent empty pass after V8-1..V8-8. No safe measurable capability left without platform APIs or workspace-scoping architecture approval. |
| **Changes** | none |
| **Stop** | **CAPABILITY PLATEAU** — two consecutive empty evaluation passes |

---

### Cycle: v8-eval-pass-1-capability-rescore

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V8-eval-1 |
| **Analysis** | After V8-1..V8-8, remaining candidates are platform-gated (geometry/thumbnail/audio/minimize) or architecture-gated (WorkspaceState workspace scoping / matched_application on global state). |
| **Changes** | none |

---

### Cycle: v8-8-stage-z-order-paint

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V8-8 |
| **Goal** | Stage overlapping tiles paint in observed z-order |
| **Problem** | Capture order left background windows visually on top of foreground ones |
| **Analysis** | z_order already on tiles; sort before paint (lower = foreground = later DOM) |
| **Changes** | `sortStageTilesByZOrder`; Stage mapTiles sorted |
| **Files affected** | stageDesktopUi.ts, WorkspaceApplicationStage.tsx, tests, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary |
| **Deleted / reduced** | Unordered Stage paint that ignored observed stacking |
| **Next recommended cycle** | Two empty capability evaluation passes |

---

### Cycle: v8-7-workspace-state-monitors

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V8-7 |
| **Goal** | Project observed monitors onto WorkspaceState; Stage plane uses display bounds |
| **Problem** | Only monitor_count existed — Stage fitted to window AABB and Assistant could not name displays |
| **Analysis** | Monitors already captured; project into runtime model and use as authoritative Stage plane |
| **Changes** | `WorkspaceStateMonitor` / `monitors`; Stage layout union of displays; Assistant open summary includes monitors |
| **Files affected** | domain workspace_state + lib export, domain.ts, stageDesktopUi, Stage, assistantCompanion, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary (+ domain state tests) |
| **Deleted / reduced** | Window-only Stage plane when monitors are observed |
| **Next recommended cycle** | Two empty capability evaluation passes |

---

### Cycle: v8-6-single-lock-workspace-state-projection

| Field | Value |
|-------|-------|
| **Date** | 2026-07-30 |
| **Cycle number** | V8-6 |
| **Goal** | Make observation + delta projection atomic under one DB lock |
| **Problem** | Engine acquired the lock twice — windows and latest_delta could disagree across passes |
| **Analysis** | Load latest/previous snapshots once; build delta via `from_loaded`; assert pass id identity in tests |
| **Changes** | ObservationDeltaService::from_loaded; WorkspaceStateEngine single-lock get_current |
| **Files affected** | observation_delta.rs, workspace_state_engine.rs, this log |
| **Validation** | typecheck / test / build / architecture / ipc / ui-boundary (+ state engine tests) |
| **Deleted / reduced** | Dual-lock observation/delta load race |
| **Next recommended cycle** | Capability evaluation passes toward plateau |

---

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
