# Programme V — Implementation Contract 2 (Planning)  
# Workflow Decision Support

| Field | Value |
|-------|-------|
| **Authority** | Principal Architect |
| **Implementation agent** | Cursor (after approval to commence) |
| **Programme** | [Programme V — Operator Workflows](PROGRAMME-V-OPERATOR-WORKFLOWS.md) |
| **Contract** | Implementation Contract 2 |
| **Status** | **Planned** — awaiting Principal Architect approval to commence |
| **Date** | 2026-07-30 |
| **Nature** | Planning contract — scopes IC2; does not authorise implementation until approved |
| **Depends on** | [IC1 — Operator Workflow Composition](PROGRAMME-V-IC1-OPERATOR-WORKFLOW-COMPOSITION.md) (approved) |

---

## STOP

**Do not begin implementation.**

This document defines IC2 for Principal Architect review.  
**No code changes** may proceed until this contract is explicitly approved to commence.

---

## Context

Programme V IC1 established operator workflow composition as a **derived projection** over Desktop → Arrangement → Preview → Restore — without a workflow controller, persistence, or execution authority.

IC2 should add **workflow decision support**: help operators understand the **best next action** within that composed workflow — still as explanatory projections, never as autonomous decisions.

---

## Objective

Help operators understand the best next action within a composed workflow by surfacing **state-derived recommendations** that:

1. Derive entirely from existing WorkspaceState and Programme I / V projections  
2. Explain **why** they are shown (Explain Ownership)  
3. Disappear naturally when underlying state changes  
4. Never execute automatically  

Focus: **decision support, not workflow automation.**

---

## Existing authorities consumed

| Authority | Role in IC2 | Change? |
|-----------|-------------|---------|
| WorkspaceState | Live desktop facts | **Unchanged** |
| Arrangement / capture / Restore IPC | Existing product actions | **Unchanged** |
| Programme I IC4–IC6 projections | Change diff, currency, pre-Restore, activity | **Consumed** |
| Programme V IC1 workflow projection | Current step / next-action context | **Consumed / enriched** |
| Assistant / Intelligence | Not expanded | **Unchanged** |

---

## Existing runtime state consumed

| State | Use |
|-------|-----|
| Observation / window set | “New windows detected”, desktop differs |
| Arrangement selection + entries | Completeness, Restore readiness |
| IC4/IC6 change & currency diffs | Update / out-of-date recommendations |
| IC6 pre-Restore | Restore available because… |
| IC1 workflow projection | Align recommendation with current workflow step |
| Interaction State (preview, edit, in-flight) | Suppress or retarget recommendations while ops run |

**No recommendation persistence. No background decision state.**

---

## Projection / composition performed

### Recommendation projection (derived only)

Examples (deterministic, explanatory):

| Recommendation | Because (examples) |
|----------------|-------------------|
| Capture recommended | New windows detected on Desktop vs selected Arrangement / none saved |
| Restore available | Arrangement is complete and Restore readiness projects ready |
| Update recommended | Current desktop differs from saved Arrangement (IC4/IC6 diff) |
| Preview unavailable | No Arrangement is selected |
| Preview recommended | Arrangement selected; Preview not active; Restore not yet chosen |

Each recommendation must include:

- **Statement** (what)  
- **Because** (why — facts)  
- **Owner** (which subsystem’s facts justify it)  
- **Related workflow step** (Desktop / Arrangement / Preview / Restore)

Compose with IC1 next-action where helpful; do not replace the workflow projection with a scored ranker.

### Surface

Present recommendations on the Desktop workflow surface (and Arrangements if natural) as dismissible **Interaction State** only if needed for noise — never persisted onboarding/recommendation history.

---

## Explicit non-goals

IC2 must **not** introduce:

- Recommendation engines  
- Scoring / prioritisation algorithms  
- Automation or auto-execution of Capture / Restore / Update  
- Background decision making  
- Recommendation persistence or history  
- Speculative planning  
- Hidden orchestration  
- New workflow or desktop ownership  

Recommendations remain **explanatory projections**, never autonomous decisions.

---

## Ownership unchanged

| Boundary | Affirmation |
|----------|-------------|
| WorkspaceState | Sole runtime desktop truth |
| Restore | Sole product OS positioning path |
| Workflow (IC1) | Remains a projection, not a controller |
| Recommendations | Projection only — no authority to act |

**Architectural test (expected answers for IC2 as scoped):**

1. Existing WorkspaceState? **Yes**  
2. Compose existing capabilities? **Yes**  
3. Deterministic? **Yes** (rule-based projection from known facts)  
4. Ownership unchanged? **Yes**  

---

## Proposed deliverables (when approved to implement)

1. Pure helpers for workflow recommendations (consume IC1 + Programme I projections).  
2. Desktop (primary) surface for recommendation lines with because / owner.  
3. Natural disappearance when state changes; optional session dismiss only.  
4. Tests for recommendation derivation.  
5. This contract updated to **Complete** with validation evidence.

---

## Validation (implementation phase)

- `git status` — working tree clean  
- `pnpm typecheck` · `pnpm test` · `pnpm build`  
- No recommendation engine/persistence/automation  
- Ownership boundaries unchanged  

---

## Architectural risks

| Risk | Mitigation |
|------|------------|
| Soft recommendation engine | Fixed deterministic rules from existing diffs/readiness only |
| Implicit prioritisation as scoring | At most a stable rule order documented in code; no weights/ML |
| Auto-acting on recommendations | UI copy + disabled auto-invoke; operator must press existing verbs |
| Duplicating IC1 next-action | Recommendations enrich/explain; workflow projection remains source of step progress |

---

## Success criteria

IC2 succeeds when an operator can see **why** a next action is suggested within the composed workflow, and those suggestions:

- remain fully reconstructible from existing state,  
- never execute themselves,  
- vanish when the justifying facts vanish,  
- leave ownership and architecture unchanged.

---

## Stop condition (this planning document)

IC2 planning is complete when objective, constraints, non-goals, consumed authorities/state, and success criteria are recorded.

**Cursor must not commence IC2 implementation until the Principal Architect approves this contract for execution.**

---

## Recommendation to Principal Architect

Approve IC2 to commence as **workflow decision support**: deterministic, explainable recommendations composed from WorkspaceState and Programme I/V projections — without engines, scoring, automation, or persistence.
