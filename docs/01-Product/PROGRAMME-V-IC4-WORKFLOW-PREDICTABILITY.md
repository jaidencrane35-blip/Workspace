# Programme V — Implementation Contract 4  
# Workflow Predictability

| Field | Value |
|-------|-------|
| **Authority** | Principal Architect |
| **Implementation agent** | Cursor |
| **Programme** | [Programme V — Operator Workflows](PROGRAMME-V-OPERATOR-WORKFLOWS.md) |
| **Contract** | Implementation Contract 4 |
| **Status** | Complete — **Approved** by Principal Architect |
| **Date** | 2026-07-30 |
| **Approved to commence** | 2026-07-30 |
| **Review** | 2026-07-30 — predictability without planning ownership; proceed to IC5 planning |
| **Depends on** | [IC1](PROGRAMME-V-IC1-OPERATOR-WORKFLOW-COMPOSITION.md) (approved); [IC2](PROGRAMME-V-IC2-WORKFLOW-DECISION-SUPPORT.md) (approved); [IC3](PROGRAMME-V-IC3-WORKFLOW-RECOVERABILITY.md) (approved) |

---

## Objective (satisfied)

Help operators understand **what will happen before they act** by projecting deterministic consequences already implied by current Workspace state:

1. Expected affected entities  
2. Expected unchanged entities  
3. Expected skipped / unavailable items  
4. Information gaps when required facts are missing  
5. Explicit what / why / owner for each outcome  

Predictability explains existing comparison and pre-Restore plans — it does **not** predict the future, simulate, or plan.

---

## Existing authorities consumed

| Authority | Role in IC4 | Change? |
|-----------|-------------|---------|
| WorkspaceState | Live desktop readiness / selection counts | **Unchanged** |
| Arrangement / Capture / Update / Restore IPC | Actions whose outcomes are explained | **Unchanged** |
| Programme I IC4/IC6 | Change diff counts, pre-Restore readiness, Arrangement meta | **Consumed** |
| Programme V IC1–IC3 | Workflow chrome context | **Consumed** |

IC4 **interprets** existing comparison authority. It does **not** duplicate comparison, restore planning, or arrangement evaluation.

---

## Existing runtime state consumed

| State | Use |
|-------|-----|
| Load / observation readiness | When outcomes cannot be projected |
| Selection count | Capture outcome fidelity |
| Change-diff counts (added / removed / bounds / z-order / unchanged) | Update + Restore move/unchanged counts |
| Arrangement meta (missing / bounds complete) | Unavailable / without-bounds / Preview bound entry counts |
| Pre-Restore availability | Full vs limited Restore outcome |
| Preview active flag | Preview outcome when inactive |

**No simulation engine. No planner. No prediction cache.**

---

## Projection performed

`projectWorkflowPredictability` emits independent outcomes in **fixed declaration order**:

| Id | When (facts) |
|----|----------------|
| `outcome_unavailable` | Observation / runtime not ready |
| `restore_outcome` | Arrangement selected; Restore ready — count-based move / unchanged / skip |
| `restore_outcome_limited` | Arrangement selected; Restore not ready — information gaps only |
| `update_outcome` | Selected; desktop differs — add / remove / bounds / z-order vs unchanged |
| `update_noop` | Selected; desktop matches — no effective changes |
| `capture_outcome` | Profile + selected windows — save selection; desktop positions unchanged |
| `preview_outcome` | Arrangement selected; Preview inactive — overlays; no window moves |

Each includes: what · effects · unchanged · unavailable · information gaps · because · owner · workflow step · Explain Ownership line.

### Prediction fidelity

Predictions never promise more precision than underlying data supports.

- Acceptable: `Restore will move 12 windows`  
- Not acceptable: `Restore will perfectly recreate your desktop`  

When required facts are unavailable, uncertainty is projected deterministically (`informationGaps`).

### Stability

Identical inputs ⇒ identical wording, counts, classifications, ownership, and ordering. Suppressed while an operation is in flight.

---

## Explicit non-goals (honoured)

No simulation engine, planner, speculative execution, forecasting, prediction cache, execution scheduling, optimisation engine, or behavioural inference.

---

## Ownership unchanged

| Boundary | Affirmation |
|----------|-------------|
| WorkspaceState | Sole runtime desktop truth |
| Restore | Sole product OS positioning path |
| Comparison / pre-Restore | Remain Programme I projections |
| Predictability (IC4) | Projection only — never executes predicted outcomes |

**Architectural test:**

1. Existing WorkspaceState? **Yes**  
2. Compose existing comparison / plans? **Yes**  
3. Deterministic? **Yes**  
4. Ownership unchanged? **Yes**  

---

## Validation

- `pnpm typecheck` · `pnpm test` · `pnpm build`
- Working tree clean
- Documentation complete

---

## Files touched

- `app/src/lib/workflowPredictabilityUi.ts`
- `app/src/components/WorkspaceApplicationStage.tsx`
- `app/src/App.css`
- `tests/workflow-predictability-ui.test.ts`
- This document; Programme V charter link

---

## Success criteria

IC4 succeeds when an operator can answer, before initiating an action:

- What will happen?  
- What will remain unchanged?  
- Why?  
- Which subsystem determines this?  
- What information is unavailable?  

…without Workspace introducing planning, simulation, or speculative behaviour.

---

## Stop condition

IC4 complete and **approved** by the Principal Architect (2026-07-30).

Review confirmed: predictability as interpretation of Programme I comparison / pre-Restore facts; prediction fidelity; stable wording; no simulation, planner, forecasting, prediction cache, or speculative execution.

**Next:** [IC5 — Workflow Explainability](PROGRAMME-V-IC5-WORKFLOW-EXPLAINABILITY.md) (complete — approved).
