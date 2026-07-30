# Programme V — Implementation Contract 3  
# Workflow Recoverability

| Field | Value |
|-------|-------|
| **Authority** | Principal Architect |
| **Implementation agent** | Cursor |
| **Programme** | [Programme V — Operator Workflows](PROGRAMME-V-OPERATOR-WORKFLOWS.md) |
| **Contract** | Implementation Contract 3 |
| **Status** | Complete — **Approved** by Principal Architect |
| **Date** | 2026-07-30 |
| **Approved to commence** | 2026-07-30 |
| **Review** | 2026-07-30 — recoverability without recovery ownership; proceed to IC4 planning |
| **Depends on** | [IC1](PROGRAMME-V-IC1-OPERATOR-WORKFLOW-COMPOSITION.md) (approved); [IC2](PROGRAMME-V-IC2-WORKFLOW-DECISION-SUPPORT.md) (approved) |

---

## Objective (satisfied)

Help operators understand how to recover from interruptions or incomplete workflows by projecting:

1. **Why** a workflow cannot currently continue  
2. **Missing prerequisites**  
3. **Recoverable versus non-recoverable** classification (descriptive only)  
4. **Next available operator action** using existing Workspace verbs only  
5. **Ownership** of the justifying facts  

Every explanation is a **deterministic projection** of existing runtime state. Workspace explains reality; it does not create or manage recovery.

---

## Existing authorities consumed

| Authority | Role in IC3 | Change? |
|-----------|-------------|---------|
| WorkspaceState | Live desktop / observation readiness | **Unchanged** |
| Arrangement / Capture / Restore IPC | Existing recovery actions operators may choose | **Unchanged** |
| Programme I IC5/IC6 | Currency, pre-Restore, activity | **Consumed** |
| Programme V IC1 workflow projection | Current step / blocked steps | **Consumed** |
| Programme V IC2 recommendations | Related guidance (compose, do not replace) | **Consumed** |

---

## Existing runtime state consumed

| State | Use |
|-------|-----|
| Observation load / runtime availability | Why Desktop step is blocked |
| Profile presence | Prerequisite for Arrangement save |
| Arrangement selection / emptiness / stale id | Why Preview / Restore cannot proceed; Arrangement deleted |
| Pre-Restore availability + gaps | Can / cannot Restore; missing windows |
| Currency change diff | Update unavailable when desktop matches |
| In-flight op (session) | Suppress flicker while ops run |

**No recovery engine. No resumable workflow state. No diagnostic persistence.**

---

## Projection performed

`projectWorkflowRecoverability` emits independent conditions in **fixed declaration order** (not ranked severity):

| Id | Classification | When (facts) | Next step |
|----|----------------|--------------|-----------|
| `runtime_unavailable` | Non-recoverable | Load state runtime unavailable | No action required |
| `observation_failed` | Non-recoverable | Load state error | No action required |
| `no_desktop_windows` | Non-recoverable | Ready; zero observed windows | No action required |
| `no_profile` | Non-recoverable | No Profile selected | No action required |
| `arrangement_deleted` | Non-recoverable | Selection id with Arrangement gone | No action required |
| `restore_data_unavailable` | Non-recoverable | Selected; Restore not ready | No action required |
| `no_arrangement_capture` | Recoverable | Profile; no saved Arrangement; windows present | Capture Desktop |
| `restore_unavailable_no_arrangement` | Recoverable | Arrangements exist; none selected | Select an Arrangement |
| `preview_unavailable_no_arrangement` | Recoverable | No Arrangement selected | Select an Arrangement / Capture Desktop |
| `update_unavailable_matches` | Recoverable | Selected; desktop matches Arrangement | No action required |
| `restore_partial_gaps` | Recoverable | Restore ready; missing tracked windows | Restore |

Each includes: what · classification · because · missing prerequisite · owner · next step · workflow step · Explain Ownership line.

### Recoverable vs non-recoverable

These are **descriptive classifications**, not runtime stores:

- **Recoverable** — the operator can continue by performing an existing action  
- **Non-recoverable** — Workspace cannot proceed because required facts do not exist  

### Next steps (existing verbs only)

Capture · Update · Preview · Restore · Select Arrangement · No action required  

No new actions are introduced. Capabilities continue to own their own execution.

### Explanation consistency

For identical underlying facts, Workspace always produces the same:

- recoverability classification  
- explanation wording  
- next step  
- ownership attribution  

Suppressed while an operation is in flight (avoids flicker; not “thinking”).

### Graceful interruption (carry forward from IC1)

Interruptions remain **re-projection**, not resume:

- Preview closed  
- Arrangement deselected  
- Desktop changes  
- Restore cancelled / completed  

IC3 explains the resulting condition; it does not checkpoint or roll back a workflow.

---

## Explicit non-goals (honoured)

IC3 does **not** introduce:

- Recovery engine  
- Resumable / transactional workflow state  
- Workflow checkpoints  
- Automatic retry  
- Diagnostic persistence or recovery history  
- Recovery automation / background healing  
- Scoring of recoverability  

If recovery requires operator action, Workspace **explains** that action — it does not perform it.

---

## Ownership unchanged

| Boundary | Affirmation |
|----------|-------------|
| WorkspaceState | Sole runtime desktop truth |
| Restore | Sole product OS positioning path |
| Workflow (IC1) | Remains a projection |
| Recommendations (IC2) | Remain explanatory |
| Recoverability (IC3) | Projection only — never executes recovery |

Capture owns Capture · Preview owns Preview · Restore owns Restore · Arrangement owns Arrangement.

**Architectural test:**

1. Existing WorkspaceState? **Yes**  
2. Compose existing capabilities? **Yes**  
3. Deterministic? **Yes**  
4. Ownership unchanged? **Yes**  

---

## Validation

- `pnpm typecheck` · `pnpm test` · `pnpm build`
- Working tree clean
- Documentation complete

---

## Files touched

- `app/src/lib/workflowRecoverabilityUi.ts`
- `app/src/components/WorkspaceApplicationStage.tsx`
- `app/src/App.css`
- `tests/workflow-recoverability-ui.test.ts`
- This document; Programme V charter link

---

## Success criteria

IC3 succeeds when an operator can answer, without external documentation:

- Why can’t this workflow continue?  
- Is this recoverable?  
- What is missing?  
- What should I do next?  
- Which subsystem owns this condition?  

…while every answer remains a deterministic projection of existing Workspace state.

---

## Stop condition

IC3 complete and **approved** by the Principal Architect (2026-07-30).

Review confirmed: recoverability as pure projection; recoverable vs non-recoverable classifications; existing verbs only; explanation consistency; no recovery engine, resumable state, checkpoints, auto-retry, or diagnostic persistence.

**Next:** [IC4 — Workflow Predictability](PROGRAMME-V-IC4-WORKFLOW-PREDICTABILITY.md) (complete — approved).
