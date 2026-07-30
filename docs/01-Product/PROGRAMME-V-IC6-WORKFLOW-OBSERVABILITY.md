# Programme V — Implementation Contract 6 (Planning)  
# Workflow Observability

| Field | Value |
|-------|-------|
| **Authority** | Principal Architect |
| **Implementation agent** | Cursor (after approval to commence) |
| **Programme** | [Programme V — Operator Workflows](PROGRAMME-V-OPERATOR-WORKFLOWS.md) |
| **Contract** | Implementation Contract 6 |
| **Status** | **Planned** — awaiting Principal Architect approval to commence |
| **Date** | 2026-07-30 |
| **Nature** | Planning contract — scopes IC6; does not authorise implementation until approved |
| **Depends on** | [IC1](PROGRAMME-V-IC1-OPERATOR-WORKFLOW-COMPOSITION.md)–[IC5](PROGRAMME-V-IC5-WORKFLOW-EXPLAINABILITY.md) (approved) |

---

## STOP

**Do not begin implementation.**

This document defines IC6 for Principal Architect review.  
**No code changes** may proceed until this contract is explicitly approved to commence.

---

## Context

Programme V Phase 1 (IC1–IC5) established a complete operator explainability plateau:

| Contract | Operator question |
|----------|-------------------|
| **IC1** | Where am I in the workflow? |
| **IC2** | What should I do next? |
| **IC3** | Why can’t I continue? |
| **IC4** | What will happen if I continue? |
| **IC5** | How does all of this fit together? |

IC6 should shift from static explainability to **workflow observability**: helping operators understand how the current workflow is **evolving**, while remaining entirely grounded in existing state transitions.

---

## Objective

Help operators understand how the current workflow is evolving over time by projecting:

1. **Meaningful state transitions** already implied by successive Workspace / Interaction facts  
2. **Why a projection changed** (which input facts differ)  
3. **Stable state versus transient execution** (e.g. in-flight ops)  
4. **Change provenance** between the current projection and the **immediately preceding** projection snapshot  

Goal: improve **temporal comprehension**, not introduce history, analytics, or a timeline subsystem.

---

## Existing authorities consumed

| Authority | Role in IC6 | Change? |
|-----------|-------------|---------|
| WorkspaceState | Current observation / desktop facts | **Unchanged** |
| Interaction State | In-flight op, selection, preview/edit flags | **Unchanged** |
| Programme I projections | Currency / pre-Restore / activity (as inputs to V) | **Consumed** |
| Programme V IC1–IC5 | Current projection outputs to observe | **Consumed** |

---

## Existing runtime state consumed

| State | Use |
|-------|-----|
| Current IC1–IC5 projection outputs | What the workflow explains now |
| Immediately preceding projection snapshot (session / Interaction State only) | What changed since the last projection |
| In-flight operational flags | Transient execution vs stable projection |
| Load / selection / Arrangement / Restore readiness facts | Provenance of why projections changed |

**Immediately preceding** means at most one prior snapshot held in Interaction State (or recomputed equivalence markers) — **not** an event log, timeline, or persisted history.

---

## Projection / composition performed

### Observability projection (derived only)

Examples of explanatory projections (illustrative):

| Condition | Explanation pattern |
|-----------|---------------------|
| Phase changed | Workflow phase moved from Arrangement → Restore because Restore readiness became available · Owner: IC1 / Restore Projection |
| Recommendation changed | Next action changed because desktop now differs from Arrangement · Owner: IC2 / Arrangement Comparison |
| Entered in-flight | Projections suppressed / marked transient because Restore is executing · Owner: Interaction |
| Recoverability cleared | Recoverability no longer reports blockage because Arrangement was selected · Owner: IC3 |
| Predictability updated | Expected Restore move count changed because comparison counts changed · Owner: IC4 |

Each observability item should answer:

- **What changed** in the composed workflow explanation?  
- **Why** (which underlying facts / source projections differ)?  
- **Is the current view stable or transient?**  
- **Owner** of the justifying facts / source projection  
- **Traceability** to IC1–IC5 source projections (preserve IC5 invariant)

### Determinism and consistency

For identical current + immediately-preceding inputs, Workspace must always produce the same:

- transition set  
- wording  
- ownership  
- ordering  

No scoring, heuristics, or behavioural inference.

### Explanation traceability (carry forward from IC5)

Every observability statement must remain attributable to:

- a source Programme V projection (IC1–IC5), and/or  
- an existing Interaction / WorkspaceState fact  

IC6 must not become an independent source of truth.

---

## Explicit non-goals

IC6 must **not** introduce:

- Event history / audit timeline for workflows  
- Timeline engine  
- Persistent activity log  
- Behavioural analytics or usage metrics  
- Multi-step rewind / replay  
- Workflow memory beyond the immediately preceding snapshot  
- Background observers or polling authorities  
- New comparison, recommendation, recovery, prediction, or narrative logic  

Observability is a **projection of current and immediately preceding state**, not a historical subsystem.

---

## Ownership unchanged

| Boundary | Affirmation |
|----------|-------------|
| WorkspaceState | Sole runtime desktop truth |
| Restore | Sole product OS positioning path |
| IC1–IC5 | Remain authorities for their dimensions |
| Observability (IC6) | Projection / composition only — owns no history store |

**Architectural test (expected for IC6 as scoped):**

1. Existing WorkspaceState + prior projection snapshot? **Yes**  
2. Compose existing capabilities / projections? **Yes**  
3. Deterministic? **Yes**  
4. Ownership unchanged? **Yes**  

---

## Proposed deliverables (when approved to implement)

1. Pure helpers that diff current vs immediately preceding IC1–IC5 projection outputs / key facts.  
2. Desktop workflow surface for meaningful transitions, stable vs transient, and change provenance.  
3. Traceability fields preserved (`sourceProjection` / `sourceId` where applicable).  
4. Tests for stable transition wording and non-persistence of snapshots.  
5. This contract updated to **Complete** with validation evidence.

---

## Validation (implementation phase)

- `git status` — working tree clean  
- `pnpm typecheck` · `pnpm test` · `pnpm build`  
- No event history / timeline engine / persistent activity log  
- Ownership boundaries unchanged  

---

## Architectural risks

| Risk | Mitigation |
|------|------------|
| Soft history / timeline | Cap at one preceding snapshot; never persist across sessions |
| Analytics creep | Forbid metrics, funnels, behavioural scoring |
| Independent ownership | Every statement traces to IC1–IC5 or existing Interaction facts |
| Recomputing comparison | Diff projection outputs / readiness flags only — do not fork Programme I |

---

## Success criteria

IC6 succeeds when an operator can answer:

- What just changed in my workflow explanation?  
- Why did that projection change?  
- Am I looking at a stable state or a transient execution?  
- Which subsystem / projection owns that change?  

…without Workspace introducing a historical, timeline, or analytics authority.

---

## Stop condition (this planning document)

IC6 planning is complete when objective, constraints, non-goals, consumed authorities/state, and success criteria are recorded.

**Cursor must not commence IC6 implementation until the Principal Architect approves this contract for execution.**

---

## Recommendation to Principal Architect

Approve IC6 to commence as **workflow observability**: explanatory projections of meaningful transitions between the current and immediately preceding Programme V explanations — with no event history, timeline engine, persistent activity log, or behavioural analytics.
