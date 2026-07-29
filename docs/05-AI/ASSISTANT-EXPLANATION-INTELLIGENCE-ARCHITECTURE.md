# Assistant Explanation Intelligence Architecture (Programme IV — Batch 14)

**Status:** Charter only — not accepted for implementation  
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

Batch 14 asks the next architectural question:

> **"How does the assistant explain retrieved evidence and workspace situations without becoming a reasoning authority, decision engine, or interpretation authority?"**

This charter defines the **Assistant Explanation Intelligence Contract** — human-facing explanation packaging that composes the existing Workspace Explanation Layer and evidence lineage **without** drawing conclusions, determining truth, inventing causality, or recommending action.

**Implementation is blocked until this charter is reviewed and accepted.**

---

## Primary principle

**Clarify recorded evidence. Never conclude what it means.**

Assistant explanation intelligence:

- **packages** explanations from existing explanation and evidence contracts
- **formats** citations, gaps, conflicts, and limitations for humans
- **preserves** provenance, uncertainty, and incompleteness
- **never** becomes a reasoning SoT, causal authority, Recommendation system, Decision authority, or policy interpreter

Capability growth must not equal code duplication (Batch 10 direction lock; Batches 11–13 Grade A acceptance). Batch 14 must evaluate reuse of:

1. Programme III Workspace Explanation Layer (`WorkspaceExplanationService` / explanation packages)
2. Batch 13 Assistant Retrieval packages (retrieved evidence presentation inputs)
3. Batch 12 Assistant Context (scope / continuity inputs)
4. Batch 11 Assistant Surface (citation / turn presentation patterns)
5. Evidence Trace / Evidence Navigation (`load_snapshot` only)
6. Batch 10 observational scaffolding (`workspace_evidence_contract`, `evidenceProjectionContract`)

before introducing any new module, migration, or guard family.

---

## Status of this document

| State | Meaning |
|---|---|
| **Charter only** | Architecture proposed; no domain/kernel/database/React implementation in this batch until acceptance |
| **Depends on Batches 11–13 + Programme III Explanation** | Composes existing explanation/evidence/assistant contracts — does not replace them |
| **No baseline change yet** | Mutation baseline remains **85**; history/projection DTO inventory remains **36** until an accepted implementation design exists |

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

| Concern | Prior owner | Batch 14 (this charter) |
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

### Proposed owner (post-acceptance)

`WorkspaceAssistantExplanationService` *(name provisional — prefer composition beside/near assistant surface/context/retrieval modules if a separate full stack would clone rather than clarify)*

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

### Must not create

| Forbidden creation | Why |
|---|---|
| New reasoning engine | Reasoning authority lives elsewhere |
| New explanation authority / SoT | Programme III Explanation Layer remains owner |
| New knowledge model | Would duplicate ontology / intelligence substrate |
| Duplicate intelligence layer | Clone-wave anti-pattern |
| Autonomous analysis loop | No self-directed reinterpretation authority |

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

### Clarity vs conclusion

| Clarity (allowed) | Conclusion (forbidden) |
|---|---|
| “Recorded explanation package states…” | “Therefore the workspace is healthy / broken” |
| “Evidence Trace links A → B as recorded” | “A caused B” without upstream causal claim |
| “Coverage reports partial for subject X” | “X is incomplete and must be fixed” as directive |
| “Consistency reports disagreement” | “Trust source Y” |

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

## Memory and context boundary

| Layer | Batch 14 role |
|---|---|
| Temporary explanation package | May package for presentation; dual-channel evidence only if accepted later |
| Batch 13 retrieval packages | Input — cited, not rewritten as conclusions |
| Batch 12 conversation context | Input scope |
| Programme III explanation snapshots | Read-only via `load_snapshot` |
| Durable memory / cognitive / Intent / Decision | Never written; never concluded here |

Displayed explanation ≠ durable memory ≠ truth conclusion.

---

## UI boundary

### Projection-only

- React renders explanation packages via Batch 10–13 projection helpers (extend; do not invent another parallel contract family without audit justification).
- No mutation from render paths.
- Explanation panels are informational — not command affordances.

### Information vs action

| Information (explanation-owned) | Action (other owners) |
|---|---|
| “Here is what recorded explanation/evidence says…” | “Approve contract” |
| “Conflict remains unresolved in evidence…” | “Accept recommendation” |
| “Gap: upstream explanation unavailable…” | “Open Decision Queue item” |

UI must not imply the assistant has resolved meaning or chosen a side.

---

## Governance

### Permissions (proposed — post-acceptance)

- Explanation package read / compose inspection: `work_context.read` (or existing assistant read paths)
- Any durable dual-channel package mutation (if accepted later): existing write capabilities — **not** `assistant.explanation.superuser`
- No capability grants issued by assistant explanation

### Audit expectations

- Observational events for explanation packaging (e.g. `workspace.assistant.explanation.packaged`) with workspace id, upstream refs, `authority_effect: none`
- Never audit explanation packages as conclusions, approvals, or recommendations
- Append-only evidence only

### Provenance requirements

- Every explanatory claim must attach to a cited upstream package or an explicit gap
- Missing upstreams produce diagnostics — never silent omission
- Conflicts and incompleteness remain visible

---

## Maintainability requirements (before implementation)

Implementation (when unblocked) must document in the maintainability audit:

1. **Reusable contracts first**
   - Compose Programme III Explanation + Trace/Navigation + Batches 11–13
   - Reuse `workspace_evidence_contract` and thin React wrappers
   - Prefer extending assistant modules when a separate full stack would clone
2. **Avoid subsystem clone**
   - No second Explanation Layer / reasoner / knowledge model
   - One governance guard entry via `EVIDENCE_ENGINE_GUARD_SPECS` if a new service file appears
   - No migration that duplicates Programme III explanation tables as “assistant conclusions”
3. **Folder ownership**
   - Clear naming (`assistant_explanation` vs surface / context / retrieval)
   - Docs linked from Programme IV indexes
   - Minimal abstractions; composition over expansion
4. **LOC honesty**
   - Justify any new files against “capability ≠ duplication”
   - Prefer thin packaging over re-implementing explanation synthesis

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

## Out of scope for Batch 14 charter

- Model provider / prompt engineering details
- Product UX layouts beyond information vs action
- Implementing explanation services, migrations, or commands
- Resolving case5 / case11 (non–Programme IV debt)
- Replacing Programme III Explanation Layer
- Replacing Batches 11–13

---

## Acceptance criteria for this charter

Before implementation may begin, reviewers must confirm:

- [ ] Ownership / non-ownership tables are unambiguous
- [ ] Reuse of Programme III Explanation + Trace/Navigation + Batches 11–13 is explicit
- [ ] Clarity vs conclusion rules forbid truth/causal/recommendation authority
- [ ] Explainability boundary (exists / origin / change / missing / conflicts) is enforceable
- [ ] Maintainability reuse path vs Batches 10–13 is explicit
- [ ] No mutation baseline / DTO inventory change is implied by charter acceptance alone
- [ ] Status remains **Charter only** until a separate implementation ACCEPT

---

## Related documents

- [Assistant Retrieval Intelligence Architecture](./ASSISTANT-RETRIEVAL-INTELLIGENCE-ARCHITECTURE.md)
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
> Implementation remains blocked until this charter is accepted.
