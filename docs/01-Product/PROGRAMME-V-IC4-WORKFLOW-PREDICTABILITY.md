# Programme V — Implementation Contract 4 (Planning)  
# Workflow Predictability

| Field | Value |
|-------|-------|
| **Authority** | Principal Architect |
| **Implementation agent** | Cursor (after approval to commence) |
| **Programme** | [Programme V — Operator Workflows](PROGRAMME-V-OPERATOR-WORKFLOWS.md) |
| **Contract** | Implementation Contract 4 |
| **Status** | **Planned** — awaiting Principal Architect approval to commence |
| **Date** | 2026-07-30 |
| **Nature** | Planning contract — scopes IC4; does not authorise implementation until approved |
| **Depends on** | [IC1](PROGRAMME-V-IC1-OPERATOR-WORKFLOW-COMPOSITION.md) (approved); [IC2](PROGRAMME-V-IC2-WORKFLOW-DECISION-SUPPORT.md) (approved); [IC3](PROGRAMME-V-IC3-WORKFLOW-RECOVERABILITY.md) (approved) |

---

## STOP

**Do not begin implementation.**

This document defines IC4 for Principal Architect review.  
**No code changes** may proceed until this contract is explicitly approved to commence.

---

## Context

Programme V has established:

- **IC1** — Compose Desktop → Arrangement → Preview → Restore as a derived workflow projection  
- **IC2** — Surface deterministic recommendations (what / why / owner) without automation  
- **IC3** — Explain interruptions and recoverability without a recovery engine  

IC4 should improve **operator predictability** by explaining **what will happen before the operator acts**, using existing comparison data and execution plans only.

---

## Objective

Help operators understand **what will happen before they act** by projecting:

1. **Deterministic previews** of workflow outcomes for the next available action  
2. **Expected state transitions** (what changes if the operator proceeds)  
3. **Unchanged versus affected** entities  
4. **Explicit ownership** of each predicted outcome  

Goal: improve **operator foresight**, not simulate alternative futures or plan speculative paths.

---

## Existing authorities consumed

| Authority | Role in IC4 | Change? |
|-----------|-------------|---------|
| WorkspaceState | Live desktop facts for comparison | **Unchanged** |
| Arrangement / Capture / Update / Restore IPC | Actions whose outcomes are explained | **Unchanged** |
| Programme I IC4/IC6 | Change diff, pre-Restore plan, currency | **Consumed** |
| Programme V IC1–IC3 | Workflow step, recommendations, recoverability context | **Consumed** |

---

## Existing runtime state consumed

| State | Use |
|-------|-----|
| Arrangement entries vs observed windows | Which entities move, stay, or are missing |
| Pre-Restore explanation (bullets / availability) | Expected Restore outcomes |
| Change-since-capture diff | Expected Update / Capture effects |
| Preview / edit Interaction State | Preview shows saved bounds — not a new simulator |
| Selected Arrangement completeness | What Restore can and cannot apply |

**No simulation engine. No planner. No speculative forecasting beyond existing comparison / plan projections.**

---

## Projection / composition performed

### Predictability projection (derived only)

Examples of explanatory projections (illustrative):

| Action context | Explanation pattern |
|----------------|---------------------|
| Before Restore | Restore will move N windows; leave M unchanged; ignore K unavailable · Owner: Restore Projection |
| Before Update | Update will replace saved Arrangement bounds with current desktop · Owner: Arrangement Comparison |
| Before Capture | Capture will create an Arrangement from currently selected / observed windows · Owner: Arrangement |
| Before Preview | Preview will show saved bounds as overlays without moving windows · Owner: Interaction |

Each predictability item should answer:

- **What action is being considered?** (existing verb only)  
- **What is expected to change?**  
- **What is expected to remain unchanged?**  
- **What cannot be applied / is skipped?**  
- **Which subsystem owns that prediction?**  

Compose with IC1 current step, IC2 recommendations, and IC3 recoverability where helpful; do not invent a separate prediction controller or outcome cache.

### Determinism and consistency

For identical underlying facts, Workspace must always produce the same:

- predicted outcome set  
- unchanged vs affected attribution  
- wording  
- ownership  

Predictions are reconstructions of existing comparison / pre-action plans — not scored forecasts.

---

## Explicit non-goals

IC4 must **not** introduce:

- Simulation engine or alternate-world modeller  
- Speculative planner / “what-if” branching  
- Predictive ML or confidence scores  
- Outcome history / prediction persistence  
- Automatic execution based on predicted outcomes  
- New OS apply paths beyond existing Restore  
- Duplicate comparison logic outside Programme I projections  

If the operator needs foresight, Workspace **projects** the existing plan — it does not invent a new future.

---

## Ownership unchanged

| Boundary | Affirmation |
|----------|-------------|
| WorkspaceState | Sole runtime desktop truth |
| Restore | Sole product OS positioning path |
| Workflow / recommendations / recoverability | Remain projections |
| Predictability (IC4) | Projection only — never executes predicted outcomes |

**Architectural test (expected for IC4 as scoped):**

1. Existing WorkspaceState? **Yes**  
2. Compose existing capabilities / comparison plans? **Yes**  
3. Deterministic? **Yes**  
4. Ownership unchanged? **Yes**  

---

## Proposed deliverables (when approved to implement)

1. Pure helpers for workflow-outcome predictability projections.  
2. Desktop workflow surface for before-action explanations (unchanged vs affected).  
3. Composition with Programme I pre-Restore / change-diff and Programme V IC1–IC3 chrome.  
4. Tests for stable predictability wording.  
5. This contract updated to **Complete** with validation evidence.

---

## Validation (implementation phase)

- `git status` — working tree clean  
- `pnpm typecheck` · `pnpm test` · `pnpm build`  
- No simulation / planner / prediction persistence  
- Ownership boundaries unchanged  

---

## Architectural risks

| Risk | Mitigation |
|------|------------|
| Soft simulation engine | Restrict inputs to existing diffs and pre-Restore plans only |
| Speculative branching | Forbid alternate futures; one projection per current facts + considered verb |
| Duplicating Programme I pre-Restore | Compose / re-present; do not fork comparison ownership |
| Implied auto-apply | Copy must require operator to use existing controls |

---

## Success criteria

IC4 succeeds when an operator can answer, before acting:

- What will happen if I proceed?  
- What stays the same?  
- What is affected?  
- What cannot be applied?  
- Which subsystem owns that prediction?  

…without Workspace introducing simulation or planning authority beyond projecting existing comparison data and execution plans.

---

## Stop condition (this planning document)

IC4 planning is complete when objective, constraints, non-goals, consumed authorities/state, and success criteria are recorded.

**Cursor must not commence IC4 implementation until the Principal Architect approves this contract for execution.**

---

## Recommendation to Principal Architect

Approve IC4 to commence as **workflow predictability**: explanatory projections of expected outcomes before operator action — composed from WorkspaceState and Programme I/V comparison plans, with no simulation engine, planner, or speculative forecasting.
