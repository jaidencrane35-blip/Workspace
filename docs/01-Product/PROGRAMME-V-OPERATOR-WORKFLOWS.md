# Programme V — Operator Workflows

| Field | Value |
|-------|-------|
| **Status** | **Phase 1 formally accepted and closed** — IC1–IC6 approved; integration review **PASS**; awaiting merge into `main`; next phase awaiting distinct charter |
| **Phase 1 conclusion** | [Phase 1 conclusion & next-phase handoff](PROGRAMME-V-PHASE-1-CONCLUSION-AND-NEXT-PHASE-HANDOFF.md) (formally accepted 2026-07-30) |
| **Phase 1 integration review** | [Phase 1 Final Integration Review](PROGRAMME-V-PHASE-1-INTEGRATION-REVIEW.md) (PASS — 2026-07-31) |
| **Authority** | Principal Architect |
| **Audience** | Principal Architect, Engineering, Cursor agents |
| **Nature** | Formal product architecture programme charter |
| **Not** | A continuation of Workspace Programme I; not an AI programme; not a feature backlog |
| **Depends on** | AI Programmes II–IV (foundations); [Workspace Programme I](PROGRAMME-I-WORKSPACE-PRODUCT-CAPABILITY.md) (complete — product capability) |
| **Related** | [Programme I conclusion & handoff](PROGRAMME-I-CONCLUSION-AND-NEXT-PROGRAMME-HANDOFF.md), [Architectural Evidence Report](../02-Architecture/ARCHITECTURAL-EVIDENCE-REPORT.md) |
| **Implementation contracts** | [IC1 — Operator Workflow Composition](PROGRAMME-V-IC1-OPERATOR-WORKFLOW-COMPOSITION.md) (complete — approved); [IC2 — Workflow Decision Support](PROGRAMME-V-IC2-WORKFLOW-DECISION-SUPPORT.md) (complete — approved); [IC3 — Workflow Recoverability](PROGRAMME-V-IC3-WORKFLOW-RECOVERABILITY.md) (complete — approved); [IC4 — Workflow Predictability](PROGRAMME-V-IC4-WORKFLOW-PREDICTABILITY.md) (complete — approved); [IC5 — Workflow Explainability](PROGRAMME-V-IC5-WORKFLOW-EXPLAINABILITY.md) (complete — approved); [IC6 — Workflow Observability](PROGRAMME-V-IC6-WORKFLOW-OBSERVABILITY.md) (complete — approved; Phase 1 concluding contract) |

---

## Programme lineage

| Programme | Role |
|-----------|------|
| AI Programme I | Foundation |
| AI Programme II | Governed Cognitive Layer |
| AI Programme III | Unified WorkspaceState & Governance |
| AI Programme IV | Semantic Query Capability (**Frozen**) |
| **Workspace Programme I** | Product Capability (**Complete**) |
| **Workspace Programme V** | **Operator Workflows** (this charter) |

This charter begins an **entirely new Workspace programme**.

It is **not** a continuation of Programme I.

It inherits Programme I’s architecture but has independent:

- objectives  
- implementation contracts  
- acceptance criteria  
- completion conditions  

---

## Title

**Programme V — Operator Workflows**

---

## Mission

Transform Workspace from a collection of excellent capabilities into an **operator-centred system** that composes those capabilities into coherent, explainable workflows.

Workspace should increasingly help operators complete meaningful tasks rather than requiring them to manually orchestrate individual features.

---

## Guiding principle

> Before introducing any new authority, determine whether the desired capability can be expressed as a **deterministic projection or composition** of existing Workspace state.

**Composition precedes expansion.**  
**Projection precedes ownership.**

---

## Authority

**WorkspaceState** remains the sole runtime desktop model.

Programme V does **not** change ownership established by:

- AI Programmes II–IV  
- Workspace Programme I  

Those programmes are architectural **foundations**.

Programme V **consumes** them.  
It does **not** replace them.

---

## Non-negotiable constraints

Do **not** introduce:

- additional runtime desktop models  
- duplicate persistence  
- duplicate workflow engines  
- duplicate recommendation engines  
- hidden automation  
- implicit execution  
- speculative planning  
- workflow caches  
- workflow history  
- background orchestration state  

Workflow composition must derive entirely from existing Workspace state.

---

## Programme objectives

### Objective 1 — Operator Workflows

Compose existing capabilities into meaningful operator tasks.

Example composition (no new orchestration authority):

```text
Desktop
  ↓
Arrangement
  ↓
Preview
  ↓
Restore
```

### Objective 2 — State-Derived Recommendations

Recommend actions only when recommendations are **deterministic projections** of existing state.

Examples:

- Restore recommended because…  
- Capture recommended because…  
- Arrangement outdated because…  
- Desktop differs because…  

Recommendations must **explain their reasoning**.

### Objective 3 — Recoverability

Operators should understand:

- what can be recovered  
- what cannot  
- why  
- what Workspace can do next  

Recoverability is **explanatory** — not another persistence layer.

### Objective 4 — Workflow Transparency

Every workflow should answer:

- What is happening?  
- Why?  
- What will happen next?  
- Which subsystem owns this?  

…using existing runtime truth (building on Programme I IC6 Explain Ownership).

### Objective 5 — Operator Confidence

Eliminate uncertainty without introducing additional architecture.

---

## Success criteria

Programme V succeeds when operators naturally perform **complete workflows** without needing to mentally assemble individual Workspace capabilities.

The product should feel **cohesive** rather than feature-oriented.

---

## Architectural test

Every implementation contract must satisfy:

1. Can this be expressed using existing WorkspaceState?  
2. Can this be expressed by composing existing capabilities?  
3. Can this remain deterministic?  
4. Does ownership remain unchanged?  

If any answer is **no**, the implementation contract must **explicitly justify** introducing new authority before implementation may proceed, and obtain Principal Architect approval.

---

## Programme governance

Every Implementation Contract must include:

- Objective  
- Existing authorities consumed  
- Existing runtime state consumed  
- Projection/composition performed  
- Ownership unchanged  
- Validation  
- Architectural risks  
- Success criteria  

No Implementation Contract may create new runtime ownership without explicit Principal Architect approval.

| Role | Responsibility |
|------|----------------|
| **Principal Architect** | Programme direction; approve implementation contracts |
| **Cursor** | Implement approved contracts only, within Programme V boundaries |

Cursor is **not** responsible for determining programme direction.

---

## First implementation contract

**IC1 — Operator Workflow Composition**  
Document: [PROGRAMME-V-IC1-OPERATOR-WORKFLOW-COMPOSITION.md](PROGRAMME-V-IC1-OPERATOR-WORKFLOW-COMPOSITION.md)

Scope (summary):

- Compose existing Desktop, Arrangement, Preview, and Restore capabilities into coherent operator workflows  
- Expose workflow progress as projections of existing state  
- Introduce **no** new workflow runtime, workflow persistence, or execution authority  

Success is measured by improved **operator comprehension**, not additional architectural capability.

---

## Programme completion

### Phase 1 (formally accepted and closed)

**Programme V Phase 1 is formally accepted and closed** as of Principal Architect acknowledgement (2026-07-30), following approval of IC6.

Phase 1 demonstrated that a sophisticated operator experience—composition, guidance, recoverability, predictability, explainability, and observability—can be built entirely through deterministic projections and compositions of existing Workspace state.

See [Phase 1 conclusion & next-phase handoff](PROGRAMME-V-PHASE-1-CONCLUSION-AND-NEXT-PHASE-HANDOFF.md).

### Programme V overall

Programme V as a whole completes when Workspace demonstrates that complex operator tasks emerge naturally from **composed existing capabilities** while preserving the architectural discipline established across AI Programmes II–IV and Workspace Programme I. Further phases require new Principal Architect charters.

---

## Stop condition (this charter)

This document established Programme V and authorised planning of implementation contracts.

**Phase 1 is formally accepted and closed.** No further implementation work should commence until the next phase receives its own charter, implementation contract, and Principal Architect approval. Final integration review / merge of the Phase 1 branch remains a Principal Architect / maintainer decision.

---

*End of Programme V charter — Phase 1 formally accepted and closed.*
