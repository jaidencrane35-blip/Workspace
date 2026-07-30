# Programme V — Implementation Contract 1  
# Operator Workflow Composition

| Field | Value |
|-------|-------|
| **Authority** | Principal Architect |
| **Implementation agent** | Cursor (after approval to implement) |
| **Programme** | [Programme V — Operator Workflows](PROGRAMME-V-OPERATOR-WORKFLOWS.md) |
| **Contract** | Implementation Contract 1 |
| **Status** | **Draft** — awaiting Principal Architect approval to implement |
| **Date** | 2026-07-30 |
| **Nature** | Implementation contract (docs only until approved) |
| **Depends on** | Programme V charter; Workspace Programme I (complete); AI Programmes II–IV foundations |

---

## STOP

**Do not begin implementation.**

This document defines IC1 for Principal Architect review.  
**No code changes** may proceed until this contract is explicitly approved to commence.

---

## Objective

Compose existing **Desktop**, **Arrangement**, **Preview**, and **Restore** capabilities into coherent **operator workflows**, and expose workflow progress as **projections of existing state**.

IC1 must improve operator comprehension of the end-to-end task path without adding architectural capability.

Canonical composition (product language):

```text
Desktop
  ↓
Arrangement
  ↓
Preview
  ↓
Restore
```

Operators should recognise this as **one workflow**, not four unrelated features.

---

## Existing authorities consumed

| Authority | Role in IC1 | Change? |
|-----------|-------------|---------|
| **WorkspaceState** | Sole runtime desktop truth; live windows / monitors / observation readiness | **Unchanged** |
| **Observation** | Desktop fact source | **Unchanged** |
| **Arrangement persistence** | Saved layouts (`DesktopArrangement`) | **Unchanged** |
| **Capture IPC** | Save / Update Arrangement | **Unchanged** |
| **Restore IPC** | Sole product OS positioning (`set_bounds` via Gateway → WindowController) | **Unchanged** |
| **Permission Gateway** | Privileged Restore / control | **Unchanged** |
| **Desktop integration** | OS interaction owner | **Unchanged** |
| **Assistant** | Sidecar only | **Unchanged** |
| **Programme I IC3–IC6 product projections** | Edit lifecycle, metadata, activity, currency, pre-Restore explanations | **Consumed / composed** — not replaced |

---

## Existing runtime state consumed

IC1 may read and project only:

| State | Source |
|-------|--------|
| Live desktop windows / monitors / load readiness | WorkspaceState / observation |
| Profile selection | Existing workspace/profile product state |
| Arrangement list / selection / entries / `updated_at` | Existing Arrangement model |
| Preview on/off; edit lifecycle phase | Interaction State (Programme I IC4/IC5 — session-only) |
| Pre-Restore / currency / activity explanations | Programme I IC5/IC6 pure helpers |
| In-flight Save / Update / Restore (session) | Interaction State — never persisted |
| Last Restore result (session) | Existing restore DTO held as Interaction State |

**No new runtime fields, stores, or background state.**

---

## Projection / composition performed

### 1. Operator workflow model (derived only)

Define a **workflow progress projection** over existing facts, for example:

| Step | Meaning | Derived from |
|------|---------|--------------|
| Desktop | Desktop is observable / ready | Observation load + WorkspaceState |
| Arrangement | An Arrangement is selected (or none) | Existing selection |
| Preview | Saved layout preview is active | Interaction State `layoutPreview` |
| Restore | Restore is available / executing / complete | IC6 pre-Restore + in-flight + last result |

This is a **view**, not a workflow engine:

- No step machine persistence  
- No orchestration runtime  
- No automatic step advancement that executes Restore/Capture  
- Progress changes only when underlying Product / Interaction State changes  

### 2. Workflow surface composition

Compose existing UI capabilities so the operator sees one path:

- Desktop map + Arrangement select (Programme I IC2)  
- Preview (IC3/IC4)  
- Pre-Restore explanation + Restore (IC6 + Restore IPC)  
- Shared verbs / Explain Ownership lines (IC5/IC6)  

IC1 may add **workflow chrome** (step list, “you are here”, next-step hint) that is entirely derived.

### 3. Next-step guidance (composition, not automation)

Expose deterministic “what can happen next” hints, for example:

- No Arrangement selected → choose or Save Arrangement  
- Arrangement selected, Preview off → Preview available  
- Arrangement out of date → Update or Restore (with IC6 why-lines)  
- Restore available → Restore will… (IC6 pre-Restore)  

**Hints must not execute.**  
**No hidden automation. No implicit execution.**

### 4. Workflow transparency

Each workflow presentation answers:

- What is happening?  
- Why?  
- What will happen next?  
- Which subsystem owns this?  

Reuse Programme I IC6 Explain Ownership pattern (`because … · Owner`).

---

## Explicit non-goals

IC1 must **not** introduce:

- Workflow runtime / engine / scheduler  
- Workflow persistence or history  
- Workflow cache or background orchestration state  
- Duplicate recommendation engine (Objective 2 may follow in a later IC)  
- Speculative planning  
- New execution authority or bypass of Restore ownership  
- New desktop / Arrangement persistence models  
- Changes to AI Programmes II–IV ownership  

---

## Ownership unchanged

| Boundary | Affirmation |
|----------|-------------|
| WorkspaceState | Remains sole runtime desktop model |
| Restore | Remains sole product `set_bounds` path |
| Capture / Arrangement store | Unchanged |
| Interaction State | Session-only; never becomes Product State |
| Workflow progress | Projection only — disappears/changes with underlying state |

**Architectural test answers for IC1:**

1. Existing WorkspaceState? **Yes**  
2. Compose existing capabilities? **Yes**  
3. Deterministic? **Yes** (pure derivation from known facts)  
4. Ownership unchanged? **Yes**  

No new authority is proposed; therefore no Principal Architect exception request is required for IC1 scope as written.

---

## Proposed deliverables (when approved to implement)

1. Pure helpers for operator workflow progress / next-step projection (no persistence).  
2. Desktop (and Arrangements, if needed) UI composition presenting Desktop → Arrangement → Preview → Restore as one workflow.  
3. Workflow transparency lines using existing activity / currency / pre-Restore explanations.  
4. Documentation update marking this contract **Complete** after validation.  
5. Tests for pure projection helpers.  

**Not in deliverables:** new IPC, new SQLite models, background jobs, auto-Restore.

---

## Validation (implementation phase)

Required before review:

- `git status` — working tree clean  
- `pnpm typecheck`  
- `pnpm test`  
- `pnpm build`  

All green. Documentation complete.

Architectural validation:

- No new workflow runtime or persistence  
- No new execution authority  
- Ownership boundaries unchanged  
- Workflow progress is fully derived  

---

## Architectural risks

| Risk | Mitigation |
|------|------------|
| Workflow chrome becomes a second step engine | Keep progress as pure function of Product + Interaction State; no stored step index |
| Next-step hints feel like automation | Copy and UX must require explicit operator action; never auto-invoke Restore/Capture |
| Duplicating IC6 explanations | Compose existing `operationalConfidenceUi` / arrangement helpers; do not fork comparison logic |
| Scope creep into recommendations / recoverability | Defer Objectives 2–3 to later Programme V contracts |

---

## Success criteria

IC1 succeeds when:

1. An operator can follow **Desktop → Arrangement → Preview → Restore** as one composed workflow on the product surface.  
2. Workflow progress and next-step hints are clearly **projections**, not a new runtime.  
3. Ownership and Restore authority remain unchanged.  
4. Operator comprehension of “where I am / what happens next” improves without architectural expansion.  
5. Validation gates are green.

---

## Stop condition (this contract)

IC1 planning is complete when this document records objective, consumed authorities/state, projection/composition, ownership affirmation, risks, validation, and success criteria.

**Cursor must not commence IC1 implementation until the Principal Architect approves this contract for execution.**

---

## Recommendation to Principal Architect

Approve IC1 to commence as a **composition-only** contract: operator workflow progress and surface cohesion over Desktop / Arrangement / Preview / Restore, consuming Programme I projections and existing IPC — with **no** workflow engine, persistence, or execution authority.
