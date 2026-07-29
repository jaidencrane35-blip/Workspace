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
