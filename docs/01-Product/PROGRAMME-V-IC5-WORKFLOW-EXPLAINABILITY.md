# Programme V — Implementation Contract 5  
# Workflow Explainability

| Field | Value |
|-------|-------|
| **Authority** | Principal Architect |
| **Implementation agent** | Cursor |
| **Programme** | [Programme V — Operator Workflows](PROGRAMME-V-OPERATOR-WORKFLOWS.md) |
| **Contract** | Implementation Contract 5 |
| **Status** | Complete — implemented; awaiting Principal Architect review |
| **Date** | 2026-07-30 |
| **Approved to commence** | 2026-07-30 |
| **Depends on** | [IC1](PROGRAMME-V-IC1-OPERATOR-WORKFLOW-COMPOSITION.md)–[IC4](PROGRAMME-V-IC4-WORKFLOW-PREDICTABILITY.md) (approved) |

---

## Objective (satisfied)

Enable an operator to understand an **entire workflow** from one unified explanation by composing Programme V IC1–IC4 projections — without a narrative engine or new ownership.

Answers: *Can Workspace explain my current workflow as one coherent story?*

---

## Existing authorities consumed

| Authority | Role | Change? |
|-----------|------|---------|
| IC1 `projectOperatorWorkflow` | What / Why / Owner / fallback Next action | **Consumed** |
| IC2 `projectWorkflowRecommendations` | Recommended next action | **Consumed** |
| IC3 `projectWorkflowRecoverability` | Recoverability (when present) | **Consumed** |
| IC4 `projectWorkflowPredictability` | Expected outcome (when present) | **Consumed** |
| WorkspaceState / Capture / Restore | Unchanged | **Unchanged** |

IC5 **composes outputs**. It does not call Programme I comparison helpers or re-derive IC1–IC4 facts.

---

## Projection / composition performed

`composeWorkflowExplainability` emits a fixed section sequence:

| Section | Source | Selection rule |
|---------|--------|----------------|
| What | IC1 current step `label · detail` | Always |
| Why | IC1 `workflow.why` | Always |
| Owner | IC1 current step `owner` | Always |
| Next action | IC2 first recommendation, else IC1 `nextAction` | Declaration order |
| Recoverability | IC3 first condition | If any (declaration order) |
| Expected outcome | IC4 first outcome | If any (declaration order) |

### Narrative fidelity

Section text is built from source fields only. Information gaps from IC4 are preserved (`Unavailable information · …`). No smoothing of uncertainty.

### Explanation traceability

Every section carries:

- `sourceProjection` — `ic1_workflow` | `ic2_recommendation` | `ic3_recoverability` | `ic4_predictability`  
- `sourceId` — originating step / recommendation / condition / outcome id  

Exposed as `data-source-projection` / `data-source-id` on Desktop chrome for engineering attribution (not operator-facing implementation detail copy).

### Stability

Identical IC1–IC4 outputs ⇒ identical sections, wording, ownership, and ordering. No scoring.

---

## Explicit non-goals (honoured)

No narrative engine, summary cache, explanation persistence, AI-generated summaries, natural-language inference, workflow memory, or additional ownership.

IC1–IC4 detail surfaces remain available alongside the unified summary.

---

## Ownership unchanged

| Boundary | Affirmation |
|----------|-------------|
| IC1–IC4 projections | Remain authorities for their dimensions |
| Explainability (IC5) | Composition only — owns no facts and no execution |
| WorkspaceState / Restore | Unchanged |

**Architectural test:** existing projections? **Yes** · compose? **Yes** · deterministic? **Yes** · ownership unchanged? **Yes**

---

## Validation

- `pnpm typecheck` · `pnpm test` · `pnpm build`
- Working tree clean
- Documentation complete

---

## Files touched

- `app/src/lib/workflowExplainabilityUi.ts`
- `app/src/lib/operatorWorkflowUi.ts` (exposes structured `why` for composition)
- `app/src/components/WorkspaceApplicationStage.tsx`
- `app/src/App.css`
- `tests/workflow-explainability-ui.test.ts`
- This document; Programme V charter link

---

## Success criteria

IC5 succeeds when an operator can understand the complete state of a workflow from one unified explanation while every statement remains attributable to existing Workspace state and Programme I/V projections.

---

## Stop condition

IC5 implementation complete for Principal Architect review.

**Do not commence further Programme V work until the Principal Architect approves IC5 and authorises the next step (including any programme conclusion).**
