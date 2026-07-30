# Programme V — Implementation Contract 6  
# Workflow Observability

| Field | Value |
|-------|-------|
| **Authority** | Principal Architect |
| **Implementation agent** | Cursor |
| **Programme** | [Programme V — Operator Workflows](PROGRAMME-V-OPERATOR-WORKFLOWS.md) |
| **Contract** | Implementation Contract 6 |
| **Status** | Complete — implemented; awaiting Principal Architect review |
| **Date** | 2026-07-30 |
| **Approved to commence** | 2026-07-30 |
| **Depends on** | [IC1](PROGRAMME-V-IC1-OPERATOR-WORKFLOW-COMPOSITION.md)–[IC5](PROGRAMME-V-IC5-WORKFLOW-EXPLAINABILITY.md) (approved) |
| **Nature** | Concluding implementation contract for Programme V Phase 1 |

---

## Objective (satisfied)

Help operators understand **what changed and why the explanation changed** by projecting meaningful transitions between the **current** and **immediately preceding** Programme V projections — without a history, timeline, or analytics subsystem.

Answers: *What changed, and why did the explanation change?*

---

## Existing authorities consumed

| Authority | Role | Change? |
|-----------|------|---------|
| IC1–IC5 projection outputs | Semantic snapshot fields | **Consumed** |
| Interaction State (`inFlight`) | Transient vs stable execution | **Consumed** |
| Immediately preceding snapshot | One prior Interaction State value only | **Session-only** |
| WorkspaceState / Capture / Restore | Unchanged | **Unchanged** |

---

## Projection performed

`captureWorkflowObservabilitySnapshot` builds a semantic fingerprint from IC1–IC5 outputs + `inFlight`.

`projectWorkflowObservability` diffs **previous → current** and emits meaningful transitions in fixed declaration order:

| Id | When |
|----|------|
| `phase_changed` | IC1 phase id/detail changed |
| `recommendation_changed` | IC2 primary recommendation identity/wording changed |
| `recoverability_changed` | IC3 primary condition identity/classification/wording changed |
| `predictability_changed` | IC4 primary outcome semantics changed |
| `execution_transient` | Entered or switched in-flight execution |
| `execution_stable` | In-flight execution finished |

Each includes: what · because · owner · stability (stable / transient) · sourceProjection · sourceId · Explain Ownership line.

### Immediate predecessor only

Stage holds at most one preceding snapshot in Interaction State (`useRef`). On meaningful fingerprint change, transitions are projected and the predecessor advances to the current snapshot. No accumulation, persistence, or timeline.

### Observation minimality

Identical fingerprints (semantic equivalence) produce **no** new transitions. Internal recomputation and cosmetic re-renders are not surfaced.

### Stable vs transient

- **Transient** — derived when current (or the transition itself) reflects in-flight execution (`observe` / `save` / `update` / `restore`)  
- **Stable** — derived when projections reflect non-executing Product / Interaction State  

Classification is descriptive — not a lifecycle manager.

### Explanation traceability

Every transition carries `sourceProjection` / `sourceId` (IC5 invariant preserved).

---

## Explicit non-goals (honoured)

No event history, timeline engine, activity log, behavioural analytics, replay, audit history, persistent observability state, or metrics collection.

Programme V continues to **explain** current reality — not record it.

---

## Ownership unchanged

| Boundary | Affirmation |
|----------|-------------|
| IC1–IC5 | Remain authorities for their dimensions |
| Observability (IC6) | Projection only — owns no history store |
| WorkspaceState / Restore | Unchanged |

**Architectural test:** existing state + one predecessor? **Yes** · compose? **Yes** · deterministic? **Yes** · ownership unchanged? **Yes**

---

## Validation

- `pnpm typecheck` · `pnpm test` · `pnpm build`
- Working tree clean
- Documentation complete

---

## Files touched

- `app/src/lib/workflowObservabilityUi.ts`
- `app/src/components/WorkspaceApplicationStage.tsx`
- `app/src/App.css`
- `tests/workflow-observability-ui.test.ts`
- This document; Programme V charter link

---

## Success criteria

IC6 succeeds when an operator can answer:

- What changed?  
- Why did it change?  
- Which projection changed?  
- Is this change stable or transient?  
- Which subsystem owns this change?  

…without Workspace introducing any historical subsystem, timeline, or behavioural analysis.

---

## Stop condition

IC6 implementation complete for Principal Architect review as the concluding contract of **Programme V Phase 1**.

**Do not commence Programme V Phase 2 (or programme conclusion formalities) until the Principal Architect approves IC6 and authorises the next step.**
