# Programme V — Implementation Contract 1  
# Operator Workflow Composition

| Field | Value |
|-------|-------|
| **Authority** | Principal Architect |
| **Implementation agent** | Cursor |
| **Programme** | [Programme V — Operator Workflows](PROGRAMME-V-OPERATOR-WORKFLOWS.md) |
| **Contract** | Implementation Contract 1 |
| **Status** | Complete — **Approved** by Principal Architect |
| **Date** | 2026-07-30 |
| **Approved to commence** | 2026-07-30 |
| **Review** | 2026-07-30 — workflow composition without workflow ownership; proceed to IC2 planning |
| **Depends on** | Programme V charter; Workspace Programme I (complete); AI Programmes II–IV foundations |

---

## Objective (satisfied)

Compose existing **Desktop**, **Arrangement**, **Preview**, and **Restore** capabilities into one coherent **operator workflow**, exposing progress as a **projection of existing state**.

```text
Desktop
  ↓
Arrangement
  ↓
Preview
  ↓
Restore
```

No workflow runtime, persistence, or execution authority was introduced.

---

## Existing authorities consumed

| Authority | Role | Change? |
|-----------|------|---------|
| WorkspaceState | Live desktop truth | Unchanged |
| Observation | Desktop fact | Unchanged |
| Arrangement persistence / capture IPC | Saved layouts | Unchanged |
| Restore IPC + Gateway + desktop integration | Sole OS positioning | Unchanged |
| Programme I IC3–IC6 projections | Preview, currency, pre-Restore, activity | Consumed |
| Assistant | Sidecar | Unchanged |

---

## Existing runtime state consumed

Observation load · Profile · Arrangement selection · Preview/edit Interaction State · pre-Restore availability · in-flight op · last Restore result (session).

**No new stores.**

---

## Projection / composition performed

### Workflow progress (`operatorWorkflowUi.ts`)

Pure `projectOperatorWorkflow` derives step status for Desktop / Arrangement / Preview / Restore from existing facts. Current step and next-action hint are recomputed on every render input change.

### Workflow surface (Desktop)

Stage presents the composed path, step chips (capability-owned details), Explain Ownership line, and next-action hint — without owning Desktop, Arrangement, Preview, or Restore behaviour.

### Graceful interruption

When Preview closes, Arrangement is deselected, Desktop changes, or Restore finishes/cancels, **no recovery logic runs**. Inputs to the projection change; the workflow **re-projects** the next valid action. No resume, transaction, or workflow checkpoint.

---

## Ownership unchanged

| Boundary | Affirmation |
|----------|-------------|
| Desktop / Arrangement / Preview / Restore | Remain independent capabilities |
| Workflow progress | Projection only — no stored step index |
| Restore | Sole product `set_bounds` path |
| Interaction State | Session-only |

Architectural test: WorkspaceState ✓ · Compose existing ✓ · Deterministic ✓ · Ownership unchanged ✓

---

## Validation

- `pnpm typecheck` · `pnpm test` · `pnpm build`
- Architecture / IPC / UI experience verifies as applicable
- Working tree clean

---

## Architectural risks addressed

| Risk | Mitigation |
|------|------------|
| Second step engine | Pure function; no persisted step |
| Hints feel like automation | Next-action copy never auto-invokes Restore/Capture |
| Capability boundaries dissolve | Steps label owners; existing controls unchanged |
| Resume/recovery creep | Explicitly re-project only |

---

## Success criteria

1. Operator sees Desktop → Arrangement → Preview → Restore as one workflow  
2. Progress is clearly derived, not a new runtime  
3. Interruption re-projects without recovery machinery  
4. Ownership unchanged; validation green  

---

## Files touched

- `app/src/lib/operatorWorkflowUi.ts`
- `app/src/components/WorkspaceApplicationStage.tsx`
- `app/src/App.css`
- `tests/operator-workflow-ui.test.ts`
- This document; Programme V charter link

---

## Stop condition

IC1 complete and **approved** by the Principal Architect (2026-07-30).

Review confirmed: coherent operator workflow via deterministic composition; derived projection (not a workflow engine); graceful interruption by re-projection; capability boundaries preserved; runtime ownership unchanged.

**Next:** [IC2 — Workflow Decision Support](PROGRAMME-V-IC2-WORKFLOW-DECISION-SUPPORT.md) (complete — approved).
