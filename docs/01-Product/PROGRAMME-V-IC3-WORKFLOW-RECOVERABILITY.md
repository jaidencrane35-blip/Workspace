# Programme V — Implementation Contract 3 (Planning)  
# Workflow Recoverability

| Field | Value |
|-------|-------|
| **Authority** | Principal Architect |
| **Implementation agent** | Cursor (after approval to commence) |
| **Programme** | [Programme V — Operator Workflows](PROGRAMME-V-OPERATOR-WORKFLOWS.md) |
| **Contract** | Implementation Contract 3 |
| **Status** | **Planned** — awaiting Principal Architect approval to commence |
| **Date** | 2026-07-30 |
| **Nature** | Planning contract — scopes IC3; does not authorise implementation until approved |
| **Depends on** | [IC1](PROGRAMME-V-IC1-OPERATOR-WORKFLOW-COMPOSITION.md) (approved); [IC2](PROGRAMME-V-IC2-WORKFLOW-DECISION-SUPPORT.md) (approved) |

---

## STOP

**Do not begin implementation.**

This document defines IC3 for Principal Architect review.  
**No code changes** may proceed until this contract is explicitly approved to commence.

---

## Context

Programme V has established:

- **IC1** — Compose Desktop → Arrangement → Preview → Restore as a derived workflow projection  
- **IC2** — Surface deterministic recommendations (what / why / owner) without automation  

IC3 should improve **operator resilience** by explaining how to recover from interruptions or incomplete workflows using **existing capabilities only**.

---

## Objective

Help operators understand how to recover from interruptions or incomplete workflows by projecting:

1. **Why** a workflow cannot currently continue  
2. **Recoverable next steps** derived from current state  
3. When an Arrangement **can or cannot** be restored  
4. **Missing prerequisites**  
5. **Recoverable versus non-recoverable** conditions  

Goal: improve **operator understanding**, not automate recovery.

---

## Existing authorities consumed

| Authority | Role in IC3 | Change? |
|-----------|-------------|---------|
| WorkspaceState | Live desktop / observation readiness | **Unchanged** |
| Arrangement / Capture / Restore IPC | Existing recovery actions operators may choose | **Unchanged** |
| Programme I IC5/IC6 | Currency, pre-Restore, post-Restore, activity | **Consumed** |
| Programme V IC1 workflow projection | Current step / blocked steps | **Consumed** |
| Programme V IC2 recommendations | Related guidance (compose, do not replace) | **Consumed** |

---

## Existing runtime state consumed

| State | Use |
|-------|-----|
| Observation load / runtime availability | Why Desktop step is blocked |
| Profile presence | Prerequisite for Arrangement save |
| Arrangement selection / emptiness | Why Preview / Restore cannot proceed |
| Pre-Restore availability + gaps | Can / cannot Restore; missing windows |
| Currency (Current / Out of date / Partial / Unavailable) | Recoverable Update vs non-restorable |
| Change-since-capture diff | What must be reconciled |
| In-flight / last Restore result (session) | Transient interruption vs completed Restore |
| Preview / edit Interaction State | Interrupted Preview — re-project only |

**No recovery engine. No resumable workflow state. No diagnostic persistence.**

---

## Projection / composition performed

### Recoverability projection (derived only)

Examples of explanatory projections:

| Condition | Explanation pattern |
|-----------|---------------------|
| Runtime unavailable | Workflow cannot continue because Desktop observation requires the app runtime · Owner: Observation |
| No Profile | Arrangement save unavailable because no Profile is selected · Owner: Arrangement |
| No Arrangement selected | Preview / Restore blocked because no Arrangement is selected · Owner: Arrangement |
| Arrangement unavailable / none open | Restore cannot apply because no tracked windows are on the desktop · Owner: Desktop |
| Restore partial / gaps | Restore may proceed with gaps because some windows are missing · Owner: Restore Projection |
| Out of date | Recoverable by Update or Restore because desktop differs from Arrangement · Owner: Arrangement Comparison |
| Preview closed / Arrangement deselected | No resume required — workflow re-projects from current facts · Owner: Interaction |

Each recoverability item should answer:

- **What is blocked or incomplete?**  
- **Why?**  
- **Is it recoverable?** (yes / partial / no)  
- **What can the operator do next?** (existing verbs only)  
- **Owner** of the justifying facts  

Compose with IC1 step status and IC2 recommendations where helpful; do not invent a separate recovery controller.

### Graceful interruption (carry forward from IC1)

Interruptions remain **re-projection**, not resume:

- Preview closed  
- Arrangement deselected  
- Desktop changes  
- Restore cancelled / completed  

IC3 explains the resulting condition; it does not checkpoint or roll back a workflow.

---

## Explicit non-goals

IC3 must **not** introduce:

- Recovery engine  
- Resumable / transactional workflow state  
- Diagnostic persistence or recovery history  
- Automatic recovery execution  
- Background healing / reconciliation jobs  
- New persistence for “incomplete workflows”  
- Scoring of recoverability  

---

## Ownership unchanged

| Boundary | Affirmation |
|----------|-------------|
| WorkspaceState | Sole runtime desktop truth |
| Restore | Sole product OS positioning path |
| Workflow (IC1) | Remains a projection |
| Recommendations (IC2) | Remain explanatory |
| Recoverability (IC3) | Projection only — never executes recovery |

**Architectural test (expected for IC3 as scoped):**

1. Existing WorkspaceState? **Yes**  
2. Compose existing capabilities? **Yes**  
3. Deterministic? **Yes**  
4. Ownership unchanged? **Yes**  

---

## Proposed deliverables (when approved to implement)

1. Pure helpers for recoverability / blocked-prerequisite projections.  
2. Desktop workflow surface for recoverable vs non-recoverable explanations.  
3. Composition with IC1 steps and IC2 recommendations (no duplication of comparison logic).  
4. Tests for stable recoverability wording.  
5. This contract updated to **Complete** with validation evidence.

---

## Validation (implementation phase)

- `git status` — working tree clean  
- `pnpm typecheck` · `pnpm test` · `pnpm build`  
- No recovery engine / resumable state / diagnostic persistence  
- Ownership boundaries unchanged  

---

## Architectural risks

| Risk | Mitigation |
|------|------------|
| Soft recovery engine | Declarative rules from existing readiness/currency/diff only |
| Resume/checkpoint creep | Explicitly forbid stored workflow position; re-project only |
| Duplicating IC2 recommendations | Recoverability explains blockage; recommendations suggest actions — compose both |
| Auto-healing perception | Copy must require operator to use existing Save / Update / Restore |

---

## Success criteria

IC3 succeeds when an operator can answer:

- Why can’t this workflow continue?  
- What is recoverable vs not?  
- What existing action can I take next?  
- Which subsystem owns that truth?  

…without Workspace introducing recovery authority beyond projecting existing state.

---

## Stop condition (this planning document)

IC3 planning is complete when objective, constraints, non-goals, consumed authorities/state, and success criteria are recorded.

**Cursor must not commence IC3 implementation until the Principal Architect approves this contract for execution.**

---

## Recommendation to Principal Architect

Approve IC3 to commence as **workflow recoverability**: explanatory projections of blocked prerequisites and recoverable next steps — composed from WorkspaceState and Programme I/V projections, with no recovery engine or resumable workflow state.
