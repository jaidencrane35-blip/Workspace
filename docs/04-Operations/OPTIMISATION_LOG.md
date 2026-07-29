# Optimisation Log

| Field | Value |
|-------|-------|
| **Purpose** | Record material performance, storage, indexing, and compression work so optimisations remain human-inspectable |
| **Owner** | Engineering |
| **Status** | Active template |
| **Related** | [AI Engineering Governance §3 & §11](../00-Governance/AI_ENGINEERING_GOVERNANCE.md), [Performance Budgets](../02-Architecture/PERFORMANCE-BUDGETS.md) |

Use one cycle entry per meaningful optimisation. Do **not** log trivial micro-edits.

Every cycle must preserve the understanding path:

```text
Human concept → Optimised representation → Decoder / inspector → Human understanding
```

---

## Cycle template (copy below)

```markdown
### Cycle: <short-id>

| Field | Value |
|-------|-------|
| **Date** | YYYY-MM-DD |
| **Goal** | |
| **Problem** | |
| **Analysis** | |
| **Changes** | |
| **Files affected** | |
| **Validation** | |
| **Maintainability score** | /10 |
| **Product alignment score** | /10 |
| **Human review required** | Yes / No |
| **Inspector / decoder path** | How a human inspects or decodes the representation |
```

---

## Cycles

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
