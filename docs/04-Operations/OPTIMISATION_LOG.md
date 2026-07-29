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
