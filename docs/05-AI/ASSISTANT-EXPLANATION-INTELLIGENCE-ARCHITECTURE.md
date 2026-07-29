# Assistant Explanation Intelligence Architecture (Programme IV — Batch 14)

**Status:** Active — implemented  
**Audience:** Architecture, Kernel, Frontend, Product, Governance  
**Depends on:**  
- [Assistant Retrieval Intelligence Architecture](./ASSISTANT-RETRIEVAL-INTELLIGENCE-ARCHITECTURE.md) (Batch 13 — accepted / implemented)  
- [Assistant Context Intelligence Architecture](./ASSISTANT-CONTEXT-INTELLIGENCE-ARCHITECTURE.md) (Batch 12 — accepted / implemented)  
- [Conversational / Assistant Surface Architecture](./CONVERSATIONAL-ASSISTANT-SURFACE-ARCHITECTURE.md) (Batch 11 — accepted / implemented)  
- [Workspace Explanation Layer Architecture](./WORKSPACE-EXPLANATION-LAYER-ARCHITECTURE.md) (Programme III Batch 5 — accepted)  
- [Workspace Evidence Trace Architecture](./WORKSPACE-EVIDENCE-TRACE-ARCHITECTURE.md)  
- [Workspace Evidence Navigation Architecture](./WORKSPACE-EVIDENCE-NAVIGATION-ARCHITECTURE.md)  
- [Workspace Evidence Observational Scaffold](./WORKSPACE-EVIDENCE-OBSERVATIONAL-SCAFFOLD-ARCHITECTURE.md) (Batch 10 — accepted)  
- [Programme IV Interaction Runtime](./PROGRAMME-IV-INTERACTION-RUNTIME.md)  
- [Programme IV Maintainability Audit](../03-Engineering/PROGRAMME-IV-MAINTAINABILITY-AUDIT.md)

## Purpose

Batches 11–13 made the assistant a **presentation, context, and retrieval packaging** layer over recorded Programme II–IV intelligence.

Batch 14 answers:

> **"How does the assistant explain retrieved evidence and workspace situations without becoming a reasoning authority, decision engine, or interpretation authority?"**

This document defines the **Assistant Explanation Intelligence Contract** — human-facing explanation packaging that composes the existing Workspace Explanation Layer and evidence lineage **without** drawing conclusions, determining truth, inventing causality, or recommending action.

---

## Primary principle

**Clarify recorded evidence. Never conclude what it means.**

Assistant explanation intelligence:

- **packages** explanations from existing explanation and evidence contracts
- **formats** citations, gaps, conflicts, and limitations for humans
- **preserves** provenance, uncertainty, and incompleteness
- **never** becomes a reasoning SoT, causal authority, Recommendation system, Decision authority, or policy interpreter

---

## Status of this document

| State | Meaning |
|---|---|
| **Active — implemented** | `WorkspaceAssistantExplanationService`, migration `076`, kernel commands, projection helpers, and governance guard are implemented |
| **Depends on Batches 11–13 + Programme III Explanation** | Composes existing explanation/evidence/assistant contracts — does not replace them |
| **Governed baseline** | Mutation baseline **86**; history/projection DTO inventory **37** |

---

## Relationship to prior batches

```text
Assistant Retrieval / Context / Surface (Batches 11–13)
        │
        ▼
Assistant Explanation (Batch 14)  ← packages clarity over recorded explanation + evidence
        │
        ├── Workspace Explanation Layer (Programme III)
        ├── Evidence Trace / Navigation
        ├── load_snapshot only
        ▼
Human-facing clarity (non-actionable)
Existing authorities                 ← remain sole owners of truth / cause / decide / recommend
```

| Concern | Prior owner | Batch 14 |
|---|---|---|
| Cross-surface explanation packages | Programme III Explanation Layer | Consumes — does not reimplement |
| Evidence lineage / navigation | Programme IV Trace / Navigation | Consumes via existing contracts |
| Retrieval presentation | Batch 13 | Inputs retrieved packages; does not redefine retrieval |
| Session / context scope | Batch 12 | Inputs scope; does not redefine ownership |
| Turn / utterance composition | Batch 11 | May supply explanation packages for presentation |
| Conclusions / truth / causality | Forbidden for assistant | Still forbidden |
| Recommendations / decisions / policy interpretation | Existing owners | Still forbidden |

Batch 14 must **not** fork a second Explanation Layer, reasoning engine, or interpretation authority when Programme III/IV contracts can be composed.

---

## Ownership

### Owner

`WorkspaceAssistantExplanationService` — clear folder `packages/domain/src/workspace_assistant_explanation/` for ownership clarification beside Batches 11–13, **not** a second Programme III Explanation Layer / reasoning engine.

### May own

| Owns | Meaning |
|---|---|
| Explanation packaging | Structured human-facing package composed from recorded explanation + evidence inputs |
| Evidence citation formatting | How citations / source refs are arranged for readability |
| Explanation structure | Sections for what exists, origin, change, missing, conflicts — presentation structure only |
| Visible gaps / conflicts | Surface upstream-reported unknowns and disagreements without resolving them |
| User-facing clarity | Wording constraints that keep explanations informational and non-conclusive |

### Must not own

| Does not own | Remains owned by |
|---|---|
| Conclusions / truth determination | Forbidden as assistant SoT |
| Causal reasoning (“why it happened”) beyond cited recorded explanation | Programme III Explanation / other reasoning owners — assistant must not invent causes |
| Recommendations | Recommendation Engine |
| Decisions / approvals | Decision Engine / Decision Queue / PermissionGateway |
| Policy interpretation as authority | Policy / governance owners |
| Autonomous analysis / continuous reinterpretation | Forbidden |
| Workspace Explanation Layer internals | Programme III `WorkspaceExplanationService` |
| Evidence assessment semantics | Programme IV evidence engines |
| Retrieval / context / surface ownership | Batches 13 / 12 / 11 |
| Memory / planning / execution / lifecycle | Existing owners |

---

## Reuse requirements

### Must compose

| Contract | Role |
|---|---|
| Assistant Surface (Batch 11) | Citation / presentation patterns; may render explanation packages |
| Assistant Context (Batch 12) | Scope / continuity inputs |
| Assistant Retrieval (Batch 13) | Retrieved evidence packages as explanation inputs |
| Workspace Explanation Layer (Programme III) | Canonical evidence-backed explanation packages |
| Evidence Trace | Provenance for artefacts |
| Evidence Navigation | Path navigation of recorded evidence |
| Batch 10 scaffold | Digest / authority / projection helpers |

### Access rules

1. Upstream reads: `load_snapshot` and existing read/query commands only.
2. Never call foreign `::generate` to refresh explanations for nicer narratives.
3. Never invent causes, conclusions, or missing evidence.
4. Conflicts and gaps remain conflicts and gaps.

---

## Explainability boundary

### The assistant may explain

| Allowed | Meaning |
|---|---|
| What evidence exists | Cite recorded packages / explanation artefacts |
| Where it came from | Provenance / origin domain / source refs |
| What changed | Only when upstream recorded change/history signals exist |
| What is missing | Gaps / unavailable / incomplete as recorded |
| What conflicts exist | Consistency / explanation conflict signals as recorded |

### The assistant must not state

| Forbidden claim | Why |
|---|---|
| What is definitely true | No truth determination |
| Why something happened without cited evidence | No invented causality |
| What action should be taken | No recommendations / decisions |
| Which side of a conflict is correct | No conflict resolution authority |
| Policy meaning as binding interpretation | Policy owners interpret |

---

## Authority boundaries

Assistant explanation intelligence **must never**:

| Forbidden | Why |
|---|---|
| Conclude truth | No interpretation SoT |
| Invent causal narratives | No reasoning authority |
| Recommend / decide / approve | Existing owners |
| Interpret policy as binding | Policy / PermissionGateway |
| Execute or mutate silently | ExecutionLifecycle / CommandPipeline |
| Bypass PermissionGateway | Sole `require()` authority |
| Replace Programme III Explanation Layer | Canonical explanation owner |
| Hide uncertainty to sound confident | Explainability honesty |

**Authority effect:** all assistant explanation packages use `authority_effect: "none"` and `actionable: false`.

---

## Implementation

| Layer | Location |
|---|---|
| Domain | `packages/domain/src/workspace_assistant_explanation/` |
| Kernel service | `packages/kernel/src/services/workspace_assistant_explanation.rs` |
| Commands | `PackageWorkspaceAssistantExplanation`, `GetWorkspaceAssistantExplanation`, `GetWorkspaceAssistantExplanationSummary`, `ExplainAssistantExplanation` |
| Migration | `packages/database/migrations/076_workspace_assistant_explanation.sql` |
| Repository | `packages/database/src/repositories/workspace_assistant_explanation.rs` |
| React | `app/src/components/assistantExplanationProjection.ts` |

### Maintainability decisions

1. **Ownership clarification folder** — `workspace_assistant_explanation/` clarifies ownership beside surface/context/retrieval; it is not a second Explanation Layer.
2. **Reuse** — `AssistantSurfaceScope::explanation_default()`, `workspace_evidence_contract`, thin React wrappers; one `EVIDENCE_ENGINE_GUARD_SPECS` entry.
3. **Compose via `load_snapshot` only** — Programme III Explanation + Trace/Navigation (+ Consistency) + Batches 11–13.
4. **Dual-channel migration `076`** with explanation-specific columns — not a clone of Programme III explanation tables as “assistant conclusions”.

---

## Forbidden behaviour (explicit)

1. **Conclusive language** — no “definitely true” / resolved-truth claims.
2. **Invented causality** — no unsupported “why” narratives.
3. **Recommendations / decisions** — no action directives from explanation packages.
4. **Policy reinterpretation** — no binding policy meaning owned here.
5. **Autonomous analysis** — no background reinterpretation loops.
6. **Silent enrichment** — no filling gaps with plausible explanations.
7. **Clone-wave architecture** — no parallel Programme III Explanation stack wrapped as “assistant explanation.”

---

## Out of scope for Batch 14

- Interaction flow packaging / response composition routing (see Batch 15 charter)

---

## Next

Batch 15 — [Assistant Interaction Intelligence Architecture](./ASSISTANT-INTERACTION-INTELLIGENCE-ARCHITECTURE.md) (**implemented** — coordinates Batches 11–14 flow packaging without acting for the user).

---

## Acceptance criteria

- [x] Ownership / non-ownership tables are unambiguous
- [x] Reuse of Programme III Explanation + Trace/Navigation + Batches 11–13 is explicit
- [x] Clarity vs conclusion rules forbid truth/causal/recommendation authority
- [x] Explainability boundary (exists / origin / change / missing / conflicts) is enforceable
- [x] Maintainability reuse path vs Batches 10–13 is explicit
- [x] Mutation baseline **86** / DTO inventory **37** for `PackageWorkspaceAssistantExplanation`
- [x] Status is **Active — implemented**

---

## Related documents

- [Assistant Retrieval Intelligence Architecture](./ASSISTANT-RETRIEVAL-INTELLIGENCE-ARCHITECTURE.md)
- [Assistant Interaction Intelligence Architecture](./ASSISTANT-INTERACTION-INTELLIGENCE-ARCHITECTURE.md)
- [Assistant Context Intelligence Architecture](./ASSISTANT-CONTEXT-INTELLIGENCE-ARCHITECTURE.md)
- [Conversational / Assistant Surface Architecture](./CONVERSATIONAL-ASSISTANT-SURFACE-ARCHITECTURE.md)
- [Workspace Explanation Layer Architecture](./WORKSPACE-EXPLANATION-LAYER-ARCHITECTURE.md)
- [Workspace Evidence Trace Architecture](./WORKSPACE-EVIDENCE-TRACE-ARCHITECTURE.md)
- [Workspace Evidence Navigation Architecture](./WORKSPACE-EVIDENCE-NAVIGATION-ARCHITECTURE.md)
- [Workspace Evidence Observational Scaffold](./WORKSPACE-EVIDENCE-OBSERVATIONAL-SCAFFOLD-ARCHITECTURE.md)
- [Programme IV Interaction Runtime](./PROGRAMME-IV-INTERACTION-RUNTIME.md)
- [Workspace Vocabulary](./WORKSPACE-VOCABULARY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Programme IV Maintainability Audit](../03-Engineering/PROGRAMME-IV-MAINTAINABILITY-AUDIT.md)

---

## Explicit confirmation

> Assistant Explanation Intelligence packages clarity over recorded Programme III explanation and Programme IV evidence for humans.
> It never concludes truth, never invents causality, never recommends, never decides or approves,
> never interprets policy as authority, never executes, never bypasses PermissionGateway,
> never replaces the Workspace Explanation Layer, and never silently mutates workspace state.
