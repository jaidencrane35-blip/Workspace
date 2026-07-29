# Assistant Retrieval Intelligence Architecture (Programme IV — Batch 13)

**Status:** Charter only — not accepted for implementation  
**Audience:** Architecture, Kernel, Frontend, Product, Governance  
**Depends on:**  
- [Assistant Context Intelligence Architecture](./ASSISTANT-CONTEXT-INTELLIGENCE-ARCHITECTURE.md) (Batch 12 — accepted / implemented)  
- [Conversational / Assistant Surface Architecture](./CONVERSATIONAL-ASSISTANT-SURFACE-ARCHITECTURE.md) (Batch 11 — accepted / implemented)  
- [Workspace Evidence Observational Scaffold](./WORKSPACE-EVIDENCE-OBSERVATIONAL-SCAFFOLD-ARCHITECTURE.md) (Batch 10 — accepted)  
- [Workspace Semantic Query Architecture](./WORKSPACE-SEMANTIC-QUERY-ARCHITECTURE.md) (Batch 1)  
- Programme IV Evidence Navigation / Trace / Coverage / Consistency (Batches 2–5) and related evidence engines as needed  
- [Programme IV Interaction Runtime](./PROGRAMME-IV-INTERACTION-RUNTIME.md)  
- [Programme IV Maintainability Audit](../03-Engineering/PROGRAMME-IV-MAINTAINABILITY-AUDIT.md)

## Purpose

Batches 11–12 made the assistant a **presentation and context packaging** layer over recorded Programme II–IV intelligence.

Batch 13 asks the next architectural question:

> **"How does the assistant select, package, and present existing workspace evidence without becoming a reasoning engine, ranking authority, or recommendation system?"**

This charter defines the **Assistant Retrieval Intelligence Contract** — human-facing retrieval request packaging and evidence presentation composition over existing Semantic Query and Evidence engines **without** inventing relevance authority, truth ranking, recommendations, or a second search substrate.

**Implementation is blocked until this charter is reviewed and accepted.**

---

## Primary principle

**Present retrieved evidence. Never rank truth or recommend action.**

Assistant retrieval intelligence:

- **packages** retrieval requests against existing Programme IV engines
- **presents** recorded matches, gaps, lineage, and diagnostics
- **preserves** provenance, completeness, and uncertainty
- **never** becomes a ranking SoT, reasoning engine, Recommendation system, Decision authority, or Memory owner

Capability growth must not equal code duplication (Batch 10 direction lock; Batches 11–12 Grade A acceptance). Batch 13 must evaluate reuse of:

1. Batch 1 Semantic Query + Batches 2–9 evidence engines (`load_snapshot` / existing query paths)
2. Batch 10 observational scaffolding (`workspace_evidence_contract`, `evidenceProjectionContract`)
3. Batch 11 assistant surface packages (citation / utterance presentation patterns)
4. Batch 12 assistant context packages (scope / continuity / displayed selection)

before introducing any new module, migration, or guard family.

---

## Status of this document

| State | Meaning |
|---|---|
| **Charter only** | Architecture proposed; no domain/kernel/database/React implementation in this batch until acceptance |
| **Depends on Batches 1–12** | Composes existing retrieval/evidence/assistant contracts — does not replace them |
| **No baseline change yet** | Mutation baseline remains **84**; history/projection DTO inventory remains **35** until an accepted implementation design exists |

---

## Relationship to prior batches

```text
Human ask / context scope (Batch 12)
        │
        ▼
Assistant Retrieval (Batch 13)   ← packages retrieval requests / presents results
        │
        ├── Semantic Query (Batch 1)
        ├── Evidence Navigation / Trace / Coverage / Consistency / …
        ├── load_snapshot only
        ▼
Assistant Surface (Batch 11)     ← may present packaged retrieval in turns
Existing authorities             ← remain sole owners of truth / rank / recommend / decide
```

| Concern | Prior owner | Batch 13 (this charter) |
|---|---|---|
| Semantic retrieval execution | Batch 1 Semantic Query | Consumes — does not reimplement |
| Evidence path navigation / trace / coverage / consistency | Batches 2–5 (+ later evidence engines as scoped) | Consumes via existing contracts |
| Session / retrieval scope | Batch 12 Assistant Context | Inputs scope; does not redefine ownership |
| Turn / utterance composition | Batch 11 Assistant Surface | May supply retrieval packages for presentation |
| Relevance / truth ranking | Forbidden for assistant | Still forbidden — no ranking authority |
| Recommendations / decisions | Recommendation / Decision owners | Still forbidden |

Batch 13 must **not** fork a second Semantic Query engine, hidden ranker, or independent knowledge graph when Programme IV contracts can be composed.

---

## Ownership

### Proposed owner (post-acceptance)

`WorkspaceAssistantRetrievalService` *(name provisional — prefer composition beside/near assistant surface/context modules if a separate full stack would clone rather than clarify)*

### May own

| Owns | Meaning |
|---|---|
| Retrieval request packaging | Structured ask → existing query/evidence request shape |
| Query / context translation | Map human ask + Batch 12 scope into existing Semantic Query / evidence consult parameters — presentation translation only |
| Evidence selection presentation | Ordered/grouped display of **already returned** recorded matches without inventing scores as truth |
| Retrieval diagnostics | Unavailable / empty / partial / scoped-out — diagnostic only |
| Provenance display | Lineage, source refs, completeness/uncertainty as recorded upstream |

### Must not own

| Does not own | Remains owned by |
|---|---|
| Truth ranking / relevance authority | Forbidden as assistant SoT — upstream engines may expose recorded signals only; assistant must not invent rank-as-truth |
| Reasoning | Programme II / explanation owners as applicable |
| Recommendations | Recommendation Engine |
| Decisions / approvals | Decision Engine / Decision Queue / PermissionGateway |
| Durable memory | Memory / preference owners |
| Policy / permissions | PermissionGateway / governance owners |
| Execution / lifecycle | ExecutionLifecycle / CommandPipeline owners |
| Semantic Query engine internals | Batch 1 |
| Evidence assessment semantics | Programme IV Batches 2–9 |
| Context packaging ownership | Batch 12 |
| Turn utterance synthesis | Batch 11 |
| Independent knowledge graph / second search index | Forbidden |

---

## Retrieval boundary

### Must reuse

| Contract | Role |
|---|---|
| Workspace Semantic Query Engine | Canonical semantic retrieval |
| Evidence Navigation | Path navigation of recorded evidence |
| Evidence Trace | Provenance for a single artefact |
| Evidence Coverage | Observable completeness for a subject |
| Evidence Consistency | Observable agreement/disagreement |
| Related evidence engines (dependency / freshness / completeness / reliability) | As scoped by Batch 12 context / request packaging |
| Assistant Context (Batch 12) | Scope and continuity inputs |
| Batch 10 scaffold | Digest / authority / projection helpers |

### Must not create

| Forbidden creation | Why |
|---|---|
| Second search engine | Batch 1 is the semantic retrieval authority |
| Hidden ranking model | Ranking-as-truth / relevance SoT is forbidden |
| Independent knowledge graph | Would duplicate ontology / evidence substrate |
| Autonomous retrieval optimisation | No self-tuning retrieval that mutates strategy as authority |
| Parallel evidence assessment engine | Batches 2–9 remain owners |

### Access rules

1. Upstream reads: `load_snapshot` and existing read/query commands only.
2. Never call foreign `::generate` to refresh results for nicer answers.
3. Never invent matches, scores-as-truth, or lineage when upstreams are unavailable.
4. Empty / partial / conflicting upstream results remain empty / partial / conflicting.

---

## Presentation vs ranking

| Allowed presentation | Forbidden ranking authority |
|---|---|
| Grouping/ordering packages for readability using **explicit recorded fields** (e.g. timestamps, ids, upstream-reported order) | Claiming “most relevant” / “best evidence” as assistant truth |
| Showing upstream-reported scores/signals **as recorded diagnostics** with provenance | Treating assistant-invented scores as relevance SoT |
| Stating “N matches recorded; M unavailable” | Filling empty results with plausible hits |
| Preserving contradictions from Evidence Consistency | Resolving conflicts into a single truth |

If an upstream does not provide an ordering signal, the assistant may use stable deterministic ordering (e.g. artefact id) for display — never a learned or inferred relevance model owned here.

---

## Explainability requirements

Every retrieved item presented by Batch 13 **must** preserve:

| Requirement | Meaning |
|---|---|
| Provenance | Origin domain / owning engine |
| Lineage | Traceable artefact refs / revisions when available |
| Source references | Explicit refs to recorded packages |
| Completeness state | As reported by coverage/completeness/context diagnostics |
| Uncertainty / gaps | Unavailable, unknown, partial — never silently omitted |

Explainability never becomes a recommendation, trust directive, or approval affordance.

---

## Authority boundaries

Assistant retrieval intelligence **must never**:

| Forbidden | Why |
|---|---|
| Rank truth | No relevance/truth SoT |
| Reason beyond cited observational packages | Reasoning authorities live elsewhere |
| Recommend actions | Recommendation Engine |
| Decide / approve | Decision / PermissionGateway |
| Execute | ExecutionLifecycle |
| Bypass PermissionGateway | Sole `require()` authority |
| Replace Memory / Cognitive Model | Existing owners |
| Silently mutate workspace state | No background writes from “helpful” retrieval |
| Autonomously optimise retrieval strategy as authority | No hidden planner |

**Authority effect:** all retrieval packages use `authority_effect: "none"` and `actionable: false`.

---

## Memory and context boundary

| Layer | Batch 13 role |
|---|---|
| Temporary retrieval request / result package | May package for presentation; dual-channel evidence only if accepted later |
| Batch 12 conversation context | Input scope — not redefined |
| Retrieved evidence | Cite upstream only |
| Durable memory | Never written here; only display if already authorised elsewhere |
| Cognitive / Intent / Decision state | Read-only via existing snapshots if scoped; never mutated |

Displayed retrieval ≠ durable memory ≠ ranked truth.

---

## UI boundary

### Projection-only

- React renders retrieval packages via Batch 10/11/12 projection helpers (extend; do not invent a fourth parallel contract family without audit justification).
- No mutation from render paths.
- Result lists are informational — not command affordances.

### Information vs action

| Information (retrieval-owned) | Action (other owners) |
|---|---|
| “These recorded matches were returned…” | “Open Decision Queue item” |
| “Coverage is partial for this subject…” | “Create task” via existing command |
| “Consistency reports disagreement…” | “Accept recommendation” via Recommendation lifecycle |

UI must not imply the assistant chose the “right” evidence for action.

---

## Governance

### Permissions (proposed — post-acceptance)

- Retrieval package read / compose inspection: `work_context.read` (or existing assistant read paths)
- Any durable dual-channel package mutation (if accepted later): existing write capabilities — **not** `assistant.retrieval.superuser`
- No capability grants issued by assistant retrieval

### Audit expectations

- Observational events for retrieval packaging (e.g. `workspace.assistant.retrieval.packaged`) with workspace id, upstream refs, `authority_effect: none`
- Never audit retrieval packages as executions, approvals, or recommendations
- Append-only evidence only

### Provenance requirements

- Every presented item carries artefact ref + origin domain + optional revision
- Missing upstreams produce diagnostics — never silent omission
- Completeness and uncertainty remain visible

---

## Maintainability requirements (before implementation)

Implementation (when unblocked) must document in the maintainability audit:

1. **Reusable contracts first**
   - Compose Semantic Query + Evidence engines + Batch 12 scope
   - Reuse `workspace_evidence_contract` and thin React wrappers
   - Prefer extending assistant surface/context modules when a separate full stack would clone
2. **Avoid subsystem clone**
   - No second search index / graph / ranker / guard family
   - One governance guard entry via `EVIDENCE_ENGINE_GUARD_SPECS` if a new service file appears
   - No migration that duplicates Semantic Query tables as “assistant search memory”
3. **Folder ownership**
   - Clear naming (`assistant_retrieval` vs `assistant_surface` / `assistant_context`)
   - Docs linked from Programme IV indexes
   - Minimal abstractions; composition over expansion
4. **LOC honesty**
   - Justify any new files against “capability ≠ duplication”
   - Prefer thin packaging over re-implementing retrieval

---

## Forbidden behaviour (explicit)

1. **Second search engine** — no parallel index or query planner owned by the assistant.
2. **Hidden ranking model** — no assistant-owned relevance SoT.
3. **Invented matches** — no fabricated evidence hits.
4. **Autonomous retrieval optimisation** — no self-modifying retrieval authority.
5. **Recommendation / decision language** — no “you should” / approve / execute affordances from retrieval packages.
6. **Silent enrichment** — no filling gaps with plausible results.
7. **Clone-wave architecture** — no parallel Batch 1–9 stack wrapped as “assistant retrieval.”

---

## Out of scope for Batch 13 charter

- Model provider / prompt engineering details
- Product UX layouts beyond information vs action
- Implementing retrieval services, migrations, or commands
- Resolving case5 / case11 (non–Programme IV debt)
- Replacing Semantic Query or Evidence engines
- Replacing Batches 11–12

---

## Acceptance criteria for this charter

Before implementation may begin, reviewers must confirm:

- [ ] Ownership / non-ownership tables are unambiguous
- [ ] Retrieval boundary reuses Batches 1–5 (+ scoped evidence) and Batch 12 without a second engine
- [ ] Presentation vs ranking rules forbid relevance/truth authority
- [ ] Explainability requirements (provenance, lineage, completeness, uncertainty) are enforceable
- [ ] Maintainability reuse path vs Batches 10–12 is explicit
- [ ] No mutation baseline / DTO inventory change is implied by charter acceptance alone
- [ ] Status remains **Charter only** until a separate implementation ACCEPT

---

## Related documents

- [Assistant Context Intelligence Architecture](./ASSISTANT-CONTEXT-INTELLIGENCE-ARCHITECTURE.md)
- [Conversational / Assistant Surface Architecture](./CONVERSATIONAL-ASSISTANT-SURFACE-ARCHITECTURE.md)
- [Workspace Semantic Query Architecture](./WORKSPACE-SEMANTIC-QUERY-ARCHITECTURE.md)
- [Workspace Evidence Navigation Architecture](./WORKSPACE-EVIDENCE-NAVIGATION-ARCHITECTURE.md)
- [Workspace Evidence Trace Architecture](./WORKSPACE-EVIDENCE-TRACE-ARCHITECTURE.md)
- [Workspace Evidence Coverage Architecture](./WORKSPACE-EVIDENCE-COVERAGE-ARCHITECTURE.md)
- [Workspace Evidence Consistency Architecture](./WORKSPACE-EVIDENCE-CONSISTENCY-ARCHITECTURE.md)
- [Workspace Evidence Observational Scaffold](./WORKSPACE-EVIDENCE-OBSERVATIONAL-SCAFFOLD-ARCHITECTURE.md)
- [Programme IV Interaction Runtime](./PROGRAMME-IV-INTERACTION-RUNTIME.md)
- [Workspace Vocabulary](./WORKSPACE-VOCABULARY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Programme IV Maintainability Audit](../03-Engineering/PROGRAMME-IV-MAINTAINABILITY-AUDIT.md)

---

## Explicit confirmation

> Assistant Retrieval Intelligence packages and presents recorded Programme IV retrieval and evidence results for humans.
> It never ranks truth, never owns relevance authority, never reasons as an engine, never recommends,
> never decides or approves, never executes, never bypasses PermissionGateway,
> never creates a second search substrate or hidden knowledge graph,
> and never silently mutates workspace state.
> Implementation remains blocked until this charter is accepted.
