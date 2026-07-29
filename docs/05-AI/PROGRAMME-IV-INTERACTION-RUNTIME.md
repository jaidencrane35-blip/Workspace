# Programme IV — Interaction Runtime

**Status:** Active  
**Audience:** Architecture, Kernel, Frontend, Governance  
**Depends on:** Programme II (Cognitive Workspace), Programme III (Coherent Workspace Runtime)

---

## Purpose

Programme IV begins **interaction**.

Before conversational reasoning or assistants can safely operate, there must be exactly one governed semantic retrieval layer that answers:

> **"What do we already know?"**

—not—

> **"What should we do?"**

Programme IV starts with the **Workspace Semantic Query Engine** (Batch 1): the canonical read-only semantic retrieval layer over everything produced by Programmes II and III.

Batch 2 adds the **Workspace Evidence Navigation Engine**: deterministic navigation through existing evidence paths — answering how evidence is connected, never what it means.

---

## Primary principle

**Retrieve meaning. Never create meaning.**

The Query Engine:

- retrieves existing evidence
- never synthesises new truth
- never reasons
- never plans
- never recommends
- never executes

---

## Batch map

| Batch | Capability | Role |
|---|---|---|
| 1 | Workspace Semantic Query Engine | Canonical read-only semantic retrieval |
| 2 | Workspace Evidence Navigation Engine | Deterministic navigation of existing evidence paths |
| 3 | Workspace Evidence Trace Engine | Deterministic provenance tracing for a single artefact |
| 4 | Workspace Evidence Coverage Engine | Observable evidence completeness for a subject |

Future Programme IV batches build conversational and assistant surfaces **on top of** this retrieval layer — never beside it, never replacing it.

---

## Ownership boundary (Batch 1)

`WorkspaceSemanticQueryService` owns only:

- semantic query execution (retrieve-only)
- semantic retrieval
- retrieval packages
- retrieval provenance
- retrieval completeness
- retrieval diagnostics

It never owns cognitive model, planning, reasoning, graph, orchestration, learning, agent cast, autonomy, workspace state, policy, reconstruction, temporal intelligence, explanation, contextual understanding, knowledge, insight coordination, cross-workspace intelligence, decision support, intelligence hub, memory truth, execution, permissions, or lifecycle.

---


---

## Ownership boundary (Batch 2)

`WorkspaceEvidenceNavigationService` owns only:

- evidence paths
- navigation sessions
- navigation summaries
- traversal metadata
- navigation lineage
- navigation gaps

It never owns semantic retrieval, knowledge, contextual understanding, explanation, planning, reasoning, cognitive model, workspace state, policy, recommendations, decisions, execution, permissions, or lifecycle.


---

## Ownership boundary (Batch 3)

`WorkspaceEvidenceTraceService` owns only:

- evidence traces
- provenance chains
- lineage packages
- trace diagnostics
- trace completeness
- trace gaps

It never owns semantic retrieval, evidence navigation, explanations, contextual understanding, knowledge, planning, reasoning, policy, execution, permissions, or lifecycle.

---

## Ownership boundary (Batch 4)

`WorkspaceEvidenceCoverageService` owns only:

- coverage snapshots
- coverage metrics
- evidence completeness
- evidence availability
- coverage gaps
- coverage diagnostics

It never owns semantic retrieval, navigation, provenance, explanations, contextual understanding, knowledge, planning, reasoning, policy, decisions, execution, permissions, or lifecycle.

## Upstream access rule

All upstream access uses **`load_snapshot` only**.

Never:

- call foreign `::generate`
- refresh upstream state
- mutate repositories
- invoke lifecycle
- invoke execution
- perform reasoning
- infer relationships

---

## Dual-channel contract

Every semantic query projection exposes:

- `current`
- `history`
- `history_count` (authoritative)

History is **evidence only** — never actionable (`actionable: false`, `authority_effect: "none"`).

---

## Forbidden behaviours

Semantic Query must never:

- execute
- approve
- recommend
- optimise
- choose
- dispatch
- automate
- own lifecycle
- own permissions
- invent matches
- fabricate relevance
- invent lineage
- generate missing answers
- infer absent evidence

Unknown remains unknown. Unavailable remains unavailable. Contradictions remain contradictions.

---

## Related documents

- [Workspace Semantic Query Architecture](./WORKSPACE-SEMANTIC-QUERY-ARCHITECTURE.md)
- [Workspace Evidence Navigation Architecture](./WORKSPACE-EVIDENCE-NAVIGATION-ARCHITECTURE.md)
- [Workspace Evidence Trace Architecture](./WORKSPACE-EVIDENCE-TRACE-ARCHITECTURE.md)
- [Workspace Evidence Coverage Architecture](./WORKSPACE-EVIDENCE-COVERAGE-ARCHITECTURE.md)
- [Programme III — Coherent Workspace Runtime](./PROGRAMME-III-COHERENT-WORKSPACE-RUNTIME.md)
- [Programme II — Cognitive Workspace](./PROGRAMME-II-COGNITIVE-WORKSPACE.md)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
