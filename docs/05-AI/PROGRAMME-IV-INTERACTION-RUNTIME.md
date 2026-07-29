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
| 5 | Workspace Evidence Consistency Engine | Observable agreement/disagreement across evidence |
| 6 | Workspace Evidence Dependency Engine | Recorded dependency structure across evidence |
| 7 | Workspace Evidence Freshness Engine | Observable freshness of available evidence |
| 8 | Workspace Evidence Completeness Engine | Observable completeness and recorded omissions |
| 9 | Workspace Evidence Reliability Engine | Observable reliability characteristics across recorded evidence |
| 10 | Evidence Observational Scaffold | Shared Programme IV helpers — not a new evidence question |
| 11 | Conversational / Assistant Surface | Human-facing presentation of recorded evidence — **implemented** |
| 12 | Assistant Context Intelligence | Context selection / continuity packaging — **implemented** |
| 13 | Assistant Retrieval Intelligence | Retrieval request packaging / evidence presentation — **implemented** |
| 14 | Assistant Explanation Intelligence | Explanation packaging / citation clarity — **implemented** |
| 15 | Assistant Interaction Intelligence | Conversation flow packaging / response routing — **implemented** |
| 16 | Assistant Personalisation Boundary | Explicit presentation preference packaging — **charter only** |

Future Programme IV batches build conversational and assistant surfaces **on top of** this retrieval layer — never beside it, never replacing it. Batch 10 exists so those surfaces can reuse contracts without cloning Batches 1–9. Batch 11 implements that conversational surface as presentation, not authority. Batch 12 implements how session context is selected and packaged without becoming memory or planning authority. Batch 13 implements how assistant retrieval packages and presents existing Semantic Query / Evidence results without ranking truth or recommending. Batch 14 implements how the assistant packages explanation clarity over recorded Explanation Layer and evidence without concluding truth or inventing causality. Batch 15 implements how interaction flow is coordinated across Batches 11–14 without becoming memory, Intent, or autonomous agency. Batch 16 charters how explicit presentation preferences are packaged without hidden user modelling or autonomous adaptation.

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

---

## Ownership boundary (Batch 5)

`WorkspaceEvidenceConsistencyService` owns only:

- consistency snapshots
- consistency observations
- consistency diagnostics
- detected inconsistencies
- consistency gaps
- consistency summaries

It never owns semantic retrieval, navigation, provenance, coverage, explanations, contextual understanding, knowledge, planning, reasoning, policy, decisions, execution, permissions, or lifecycle.

---

## Ownership boundary (Batch 6)

`WorkspaceEvidenceDependencyService` owns only:

- dependency snapshots
- dependency graphs
- dependency observations
- dependency diagnostics
- dependency gaps
- dependency summaries

It never owns semantic retrieval, navigation, provenance, coverage, consistency, explanations, contextual understanding, knowledge, planning, reasoning, policy, decisions, execution, permissions, or lifecycle.

---

## Ownership boundary (Batch 7)

`WorkspaceEvidenceFreshnessService` owns only:

- freshness snapshots
- freshness observations
- freshness diagnostics
- freshness gaps
- freshness summaries
- freshness metadata

It never owns semantic retrieval, navigation, provenance, coverage, consistency, dependency, explanations, contextual understanding, knowledge, planning, reasoning, policy, decisions, execution, permissions, or lifecycle.

---

## Ownership boundary (Batch 8)

`WorkspaceEvidenceCompletenessService` owns only:

- completeness snapshots
- completeness observations
- completeness diagnostics
- completeness gaps
- completeness summaries
- completeness metadata

It never owns semantic retrieval, navigation, provenance, coverage, consistency, dependency, freshness, explanations, contextual understanding, knowledge, planning, reasoning, policy, decisions, execution, permissions, or lifecycle.

---

## Ownership boundary (Batch 9)

`WorkspaceEvidenceReliabilityService` owns only:

- reliability snapshots
- reliability observations
- reliability diagnostics
- reliability gaps
- reliability summaries
- reliability metadata

It never owns semantic retrieval, navigation, provenance, coverage, consistency, dependency, freshness, completeness, explanations, contextual understanding, knowledge, planning, reasoning, policy, decisions, execution, permissions, or lifecycle.

---

## Ownership boundary (Batch 10)

Programme IV Batch 10 is **scaffolding**, not an evidence assessment engine.

It owns only:

- shared observational helpers (`workspace_evidence_contract`)
- data-driven evidence-family governance specs
- shared React projection contract (+ thin engine wrappers)

It never owns evidence snapshots, migrations, mutations, classification semantics, or authority over Batches 1–9.

See [Workspace Evidence Observational Scaffold Architecture](./WORKSPACE-EVIDENCE-OBSERVATIONAL-SCAFFOLD-ARCHITECTURE.md).

---

## Ownership boundary (Batch 11 — implemented)

The **Conversational / Assistant Surface** is chartered in
[Conversational / Assistant Surface Architecture](./CONVERSATIONAL-ASSISTANT-SURFACE-ARCHITECTURE.md).

**Status: implemented.**

It owns human-facing presentation of retrieval, explanation, navigation, context, and citation-bound synthesis.

It will never own decisions, approvals, execution, Intent, Decision Engine, Recommendation authority, hidden memory, or silent workspace mutation.

---

## Ownership boundary (Batch 12 — implemented)

The **Assistant Context Intelligence Contract** is implemented in
[Assistant Context Intelligence Architecture](./ASSISTANT-CONTEXT-INTELLIGENCE-ARCHITECTURE.md).

Owner: `WorkspaceAssistantContextService` (`packages/domain/src/workspace_assistant_context/`).

**Status: implemented.**

It owns conversation context packaging, active session context, retrieval scope, and displayed context selection.
It packages selected context and continuity from recorded projections via `load_snapshot` only.
It never owns durable memory authority, workspace truth, Intent, decisions, plans, autonomous goals, invents continuity, or composes utterances.
It reuses Batch 10/11 contracts rather than cloning a second assistant subsystem.

---

## Ownership boundary (Batch 13 — implemented)

The **Assistant Retrieval Intelligence Contract** is owned by
`WorkspaceAssistantRetrievalService` — see
[Assistant Retrieval Intelligence Architecture](./ASSISTANT-RETRIEVAL-INTELLIGENCE-ARCHITECTURE.md).

**Status: Active — implemented.**

It owns retrieval request packaging, query/context translation for presentation, evidence selection presentation, retrieval diagnostics, and provenance display.

It never owns truth ranking, relevance authority, reasoning, recommendations, decisions, memory, policy, or execution. It composes Semantic Query, Evidence engines, and Batch 12 context rather than create a second search substrate.

---

## Ownership boundary (Batch 14 — implemented)

The **Assistant Explanation Intelligence Contract** is owned by
`WorkspaceAssistantExplanationService` — see
[Assistant Explanation Intelligence Architecture](./ASSISTANT-EXPLANATION-INTELLIGENCE-ARCHITECTURE.md).

**Status: Active — implemented.**

It owns explanation packaging, evidence citation formatting, explanation structure, visible gaps/conflicts, and user-facing clarity.

It never owns conclusions, truth determination, causal reasoning, recommendations, decisions, policy interpretation, or autonomous analysis. It composes the Programme III Explanation Layer, Evidence Trace/Navigation, and Batches 11–13 rather than create a second reasoning or explanation authority.

---

## Ownership boundary (Batch 15 — implemented)

The **Assistant Interaction Intelligence Contract** is owned by
`WorkspaceAssistantInteractionService` — see
[Assistant Interaction Intelligence Architecture](./ASSISTANT-INTERACTION-INTELLIGENCE-ARCHITECTURE.md).

**Status: Active — implemented.**

It owns conversation state packaging, interaction session structure, user-visible flow coordination, response composition routing, and interaction diagnostics.

It never owns memory, identity, Intent authority, decisions, planning, recommendations, execution, or autonomous behaviour. It composes Batches 11–14 rather than create a cognitive engine, hidden memory, or agent loop.

---

## Ownership boundary (Batch 16 — charter only)

The **Assistant Personalisation Boundary Contract** is chartered in
[Assistant Personalisation Boundary Architecture](./ASSISTANT-PERSONALISATION-BOUNDARY-ARCHITECTURE.md).

**Status: charter only — implementation blocked until acceptance.**

It will own presentation preference packaging, explicit user preference display, interface adaptation metadata, interaction style configuration, and personalisation diagnostics.

It will never own hidden user modelling, personality inference, identity, memory, behavioural prediction, psychological profiling, autonomous adaptation, or decision-making. It must compose the AI Personalization Foundation and Batches 11–15 rather than create a user-model engine or hidden profile database.

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
- [Workspace Evidence Consistency Architecture](./WORKSPACE-EVIDENCE-CONSISTENCY-ARCHITECTURE.md)
- [Workspace Evidence Dependency Architecture](./WORKSPACE-EVIDENCE-DEPENDENCY-ARCHITECTURE.md)
- [Workspace Evidence Freshness Architecture](./WORKSPACE-EVIDENCE-FRESHNESS-ARCHITECTURE.md)
- [Workspace Evidence Completeness Architecture](./WORKSPACE-EVIDENCE-COMPLETENESS-ARCHITECTURE.md)
- [Workspace Evidence Reliability Architecture](./WORKSPACE-EVIDENCE-RELIABILITY-ARCHITECTURE.md)
- [Workspace Evidence Observational Scaffold Architecture](./WORKSPACE-EVIDENCE-OBSERVATIONAL-SCAFFOLD-ARCHITECTURE.md)
- [Conversational / Assistant Surface Architecture](./CONVERSATIONAL-ASSISTANT-SURFACE-ARCHITECTURE.md) *(Batch 11 — implemented)*
- [Assistant Context Intelligence Architecture](./ASSISTANT-CONTEXT-INTELLIGENCE-ARCHITECTURE.md) *(Batch 12 — implemented)*
- [Assistant Retrieval Intelligence Architecture](./ASSISTANT-RETRIEVAL-INTELLIGENCE-ARCHITECTURE.md) *(Batch 13 — implemented)*
- [Assistant Explanation Intelligence Architecture](./ASSISTANT-EXPLANATION-INTELLIGENCE-ARCHITECTURE.md) *(Batch 14 — implemented)*
- [Assistant Interaction Intelligence Architecture](./ASSISTANT-INTERACTION-INTELLIGENCE-ARCHITECTURE.md) *(Batch 15 — implemented)*
- [Assistant Personalisation Boundary Architecture](./ASSISTANT-PERSONALISATION-BOUNDARY-ARCHITECTURE.md) *(Batch 16 — charter only)*
- [Programme III — Coherent Workspace Runtime](./PROGRAMME-III-COHERENT-WORKSPACE-RUNTIME.md)
- [Programme II — Cognitive Workspace](./PROGRAMME-II-COGNITIVE-WORKSPACE.md)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
