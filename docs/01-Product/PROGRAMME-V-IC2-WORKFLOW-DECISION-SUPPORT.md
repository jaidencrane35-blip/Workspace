# Programme V — Implementation Contract 2  
# Workflow Decision Support

| Field | Value |
|-------|-------|
| **Authority** | Principal Architect |
| **Implementation agent** | Cursor |
| **Programme** | [Programme V — Operator Workflows](PROGRAMME-V-OPERATOR-WORKFLOWS.md) |
| **Contract** | Implementation Contract 2 |
| **Status** | Complete |
| **Date** | 2026-07-30 |
| **Approved to commence** | 2026-07-30 |
| **Depends on** | [IC1 — Operator Workflow Composition](PROGRAMME-V-IC1-OPERATOR-WORKFLOW-COMPOSITION.md) (approved) |

---

## Objective (satisfied)

Help operators understand the next appropriate action within the composed workflow by surfacing **state-derived recommendations** that explain **what** and **why**, without transferring decision-making or execution authority.

---

## Existing authorities consumed

| Authority | Role | Change? |
|-----------|------|---------|
| WorkspaceState | Observed window counts / readiness | Unchanged |
| Programme I IC4/IC6 projections | Change diff, currency, pre-Restore | Consumed |
| Programme V IC1 workflow projection | Workflow surface context | Consumed |
| Capture / Update / Restore / Preview | Referenced capabilities only | Unchanged ownership |

---

## Projection performed

`projectWorkflowRecommendations` emits independent recommendations in **fixed declaration order** (not ranked priority):

| Id | When (facts) |
|----|----------------|
| `preview_unavailable` | No Arrangement selected |
| `capture_recommended` | Profile + observed windows + no Arrangement, or new windows vs selection |
| `update_recommended` | Selected Arrangement differs from desktop (change diff) |
| `preview_recommended` | Arrangement selected; Preview not active |
| `restore_available` | Pre-Restore availability projects ready |

Each includes: action · because · owner · workflow step · Explain Ownership line.

### Recommendation stability

Same fact inputs ⇒ identical recommendation set and wording. No time-based or random variation. Suppressed while an operation is in flight (avoids flicker; not “thinking”).

### Operator authority

Recommendations **explain**. Existing Save / Update / Preview / Restore controls **execute** only when the operator acts. Copy states recommendations do not execute.

---

## Explicit non-goals (honoured)

No recommendation engine, workflow AI, scoring, urgency, weighting, automation, execution, persistence, history, or learning.

---

## Ownership unchanged

Capture owns capture · Restore owns restore · Preview owns preview · Arrangement comparison facts remain Programme I projections · Recommendations own nothing.

---

## Validation

- `pnpm typecheck` · `pnpm test` · `pnpm build`
- Working tree clean
- Documentation complete

---

## Files touched

- `app/src/lib/workflowDecisionSupportUi.ts`
- `app/src/components/WorkspaceApplicationStage.tsx`
- `app/src/App.css`
- `tests/workflow-decision-support-ui.test.ts`
- This document; Programme V charter link

---

## Stop condition

IC2 complete. **Await Principal Architect review** before Programme V IC3.
