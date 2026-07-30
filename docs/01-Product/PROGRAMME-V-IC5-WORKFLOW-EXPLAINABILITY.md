# Programme V — Implementation Contract 5 (Planning)  
# Workflow Explainability

| Field | Value |
|-------|-------|
| **Authority** | Principal Architect |
| **Implementation agent** | Cursor (after approval to commence) |
| **Programme** | [Programme V — Operator Workflows](PROGRAMME-V-OPERATOR-WORKFLOWS.md) |
| **Contract** | Implementation Contract 5 |
| **Status** | **Planned** — awaiting Principal Architect approval to commence |
| **Date** | 2026-07-30 |
| **Nature** | Planning contract — scopes IC5; does not authorise implementation until approved |
| **Depends on** | [IC1](PROGRAMME-V-IC1-OPERATOR-WORKFLOW-COMPOSITION.md) (approved); [IC2](PROGRAMME-V-IC2-WORKFLOW-DECISION-SUPPORT.md) (approved); [IC3](PROGRAMME-V-IC3-WORKFLOW-RECOVERABILITY.md) (approved); [IC4](PROGRAMME-V-IC4-WORKFLOW-PREDICTABILITY.md) (approved) |

---

## STOP

**Do not begin implementation.**

This document defines IC5 for Principal Architect review.  
**No code changes** may proceed until this contract is explicitly approved to commence.

---

## Context

Programme V has established four complementary projections:

| Contract | Operator question |
|----------|-------------------|
| **IC1** | Where am I in the workflow? |
| **IC2** | What should I do next? |
| **IC3** | Why can’t I continue? |
| **IC4** | What will happen if I continue? |

Each is surfaced independently on Desktop workflow chrome. IC5 should improve **operator explainability** by composing those projections into a **single coherent narrative** of the current workflow — without introducing a narrative engine or new ownership.

---

## Objective

Enable an operator to understand an **entire workflow** from one explanatory view rather than piecing together multiple UI surfaces.

Project a unified workflow summary that identifies:

1. **Current phase** (from IC1)  
2. **Recommendation** (from IC2)  
3. **Recoverability** (from IC3)  
4. **Predicted outcome** (from IC4)  

…using **consistent What → Why → Owner** terminology across the composed explanation.

Goal: improve **narrative coherence**, not invent new facts or a storytelling subsystem.

---

## Existing authorities consumed

| Authority | Role in IC5 | Change? |
|-----------|-------------|---------|
| Programme V IC1 workflow projection | Current phase / progress / next action | **Consumed** |
| Programme V IC2 recommendations | Recommended next action + ownership | **Consumed** |
| Programme V IC3 recoverability | Blockage classification + next step | **Consumed** |
| Programme V IC4 predictability | Before-action outcome projection | **Consumed** |
| Programme I IC5/IC6 explanations | Activity / currency / pre-Restore (if composed) | **Consumed** |
| WorkspaceState / Capture / Restore | Unchanged execution authorities | **Unchanged** |

---

## Existing runtime state consumed

| State | Use |
|-------|-----|
| Outputs of `projectOperatorWorkflow` | Phase, path, next action |
| Outputs of `projectWorkflowRecommendations` | Recommendation lines |
| Outputs of `projectWorkflowRecoverability` | Recoverability conditions |
| Outputs of `projectWorkflowPredictability` | Predicted outcomes |
| In-flight op (session) | Suppress or note transient state |

**No narrative engine. No summary cache. No new ownership.**

IC5 composes projection **outputs** already produced by IC1–IC4. It must not re-derive comparison, restore planning, or recommendation logic.

---

## Projection / composition performed

### Explainability summary (derived only)

Illustrative structure (not a mandated layout):

```text
Workflow · Desktop → Arrangement → Preview → Restore
Phase · Arrangement (active)
Recommendation · Update recommended because … · Owner: Arrangement Comparison
Recoverability · Update unavailable / Restore available · Owner: …
Predictability · Restore will move N · leave M unchanged · Owner: Restore Projection
```

Each section must preserve:

- **What**  
- **Why**  
- **Owner**  

Terminology must align with IC1–IC4 wording (no paraphrasing that invents new claims).

### Determinism and consistency

For identical IC1–IC4 projection inputs/outputs, the summary must always produce the same:

- section ordering  
- selected primary recommendation / recoverability / predictability items (fixed selection rules, not ranking scores)  
- wording (prefer verbatim reuse of projection lines)  
- ownership attribution  

Selection of “primary” items, if needed for brevity, must use **fixed declaration-order rules** (e.g. first recommendation, first recoverable condition, first predictability outcome) — never scoring.

---

## Explicit non-goals

IC5 must **not** introduce:

- Narrative / storytelling engine  
- Summary cache or persisted workflow narrative  
- LLM or generative explanation  
- New comparison, recommendation, recovery, or prediction logic  
- Re-ranking / priority scoring of projections  
- Automatic execution based on the summary  
- Replacement of IC1–IC4 surfaces (compose; may still keep detail lists)

The summary **composes** existing explanations — it does not author new truth.

---

## Ownership unchanged

| Boundary | Affirmation |
|----------|-------------|
| WorkspaceState | Sole runtime desktop truth |
| Restore | Sole product OS positioning path |
| IC1–IC4 projections | Remain the authorities for their dimensions |
| Explainability (IC5) | Composition only — owns no execution and no new facts |

**Architectural test (expected for IC5 as scoped):**

1. Existing WorkspaceState / IC1–IC4 projections? **Yes**  
2. Compose existing capabilities? **Yes**  
3. Deterministic? **Yes**  
4. Ownership unchanged? **Yes**  

---

## Proposed deliverables (when approved to implement)

1. Pure helper that composes IC1–IC4 projection outputs into a unified workflow summary.  
2. Desktop workflow surface for the end-to-end explanation (alongside or above detail lists).  
3. Consistent What → Why → Owner terminology across the summary.  
4. Tests for stable composition / selection rules / wording reuse.  
5. This contract updated to **Complete** with validation evidence.

---

## Validation (implementation phase)

- `git status` — working tree clean  
- `pnpm typecheck` · `pnpm test` · `pnpm build`  
- No narrative engine / summary cache / new ownership  
- No duplicate comparison logic  

---

## Architectural risks

| Risk | Mitigation |
|------|------------|
| Soft narrative engine | Verbatim reuse of IC1–IC4 lines; fixed section templates |
| Summary cache | Derive only; never persist |
| Paraphrase drift | Prefer projection `line` / fields unchanged |
| Hidden ranking | Declaration-order selection rules only |
| Replacing detail surfaces | Compose; keep IC1–IC4 detail chrome available |

---

## Success criteria

IC5 succeeds when an operator can answer from a **single coherent explanation**:

- Where am I in the workflow?  
- What is recommended next?  
- Why can’t I continue (if blocked)?  
- What will happen if I continue?  
- Who owns each of those truths?  

…without Workspace introducing narrative authority beyond composing existing Programme I/V projections.

---

## Stop condition (this planning document)

IC5 planning is complete when objective, constraints, non-goals, consumed authorities/state, and success criteria are recorded.

**Cursor must not commence IC5 implementation until the Principal Architect approves this contract for execution.**

---

## Recommendation to Principal Architect

Approve IC5 to commence as **workflow explainability**: a unified compositional summary of IC1–IC4 projections with consistent What → Why → Owner terminology — no narrative engine, summary cache, or new ownership.
