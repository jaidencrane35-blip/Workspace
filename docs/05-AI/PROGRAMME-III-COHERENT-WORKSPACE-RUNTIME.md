# Programme III — Coherent Workspace Runtime

| Field | Value |
|-------|-------|
| **Purpose** | Establish one provenance-rich, read-only runtime view of operational, cognitive, governance, and evidence state while preserving all existing lifecycle and persistence authorities |
| **Status** | Active — Batch 7 accepted; Batch 8 charter pending |
| **Depends on** | Programme I governance / recovery; Programme II cognitive stack |
| **Non-goals** | Event sourcing; authoritative replay logs; simulation; model hosting; autonomous execution; replacement permission system; replacement for domain-owned projections |

## Programme principles (non-negotiable)

1. **Sources remain authoritative** — unified state owns composition only.
2. **No duplicated lifecycle payloads** — carry typed slices or canonical references.
3. **Every slice declares** source identity, source revision, observed/generated time, freshness, and completeness.
4. **Unknown, stale, missing, truncated, and contradictory are distinct states.**
5. **Composition is deterministic** for the same declared source revisions.
6. **Evidence and actionable state remain separate channels.**
7. **Unified state cannot** execute, approve, grant, repair, or silently refresh foreign sources.
8. **Recovery loads source-owned state** — it never fabricates a complete envelope.
9. **Consumer-specific views derive from the canonical envelope** — consumers do not rebuild it.
10. **Policy explains authority; PermissionGateway remains the final authoriser.**
11. **Unknown context fails closed** — never assume Compliant / safe.
12. **Reconstruction explains change over time** — it does not become the source of truth; no event sourcing; no replay authority.
13. **Temporal intelligence organises evidence over time** — it does not predict, simulate, correct, or become truth.
14. **Explanation Layer explains evidence** — it does not become the authority that changes reality.
15. **Contextual understanding organises situational meaning from durable evidence** — it does not decide, predict, simulate, or change reality.
16. **Knowledge synthesis derives structured concepts from evidence** — it does not become memory truth, Cognitive Model, or decision authority.

## Batch map

| Batch | Theme | Role | Status |
|-------|-------|------|--------|
| **1** | Unified Workspace State Model | Canonical composition envelope | Done (accepted) |
| **2** | Policy & Governance Engine | Context-aware policy reasoning / evidence | Done (accepted) |
| **3** | Historical Workspace Reconstruction | Temporal comparison / change explanation | Done (accepted) |
| **4** | Temporal Intelligence | Historical understanding extensions | Done (accepted) |
| **5** | Workspace Explanation Layer | Cross-surface evidence-backed explanation | Done (accepted) |
| **6** | Contextual Workspace Understanding | Richer situational understanding | Done (accepted) |
| **7** | Workspace Knowledge Synthesis | Provenance-bound structured knowledge artefacts | Done (accepted) |
| **8** | Knowledge Integration / Retrieval Intelligence | Integrate/retrieve over accumulated evidence layers | Gate open — charter next |

## Batch 6 summary

`WorkspaceContextualUnderstandingService` composes `ContextualWorkspaceSnapshot` themes
(`SituationalTheme`, `ContextualInsight`, `ContextualGap`) from durable state / policy /
reconstruction / temporal / explanation snapshots via `load_snapshot` only.
Diagnostic understanding confidence never becomes decision authority. Dual-channel
projection: `current` + `history` + authoritative `history_count`.
Commands: `GenerateContextualWorkspaceUnderstanding`, `GetContextualWorkspaceUnderstanding`,
`GetContextualWorkspaceUnderstandingSummary`, `ExplainWorkspaceContext`
(`work_context.write` / `work_context.read`). Migration `057`.
See [Contextual Workspace Understanding Architecture](./CONTEXTUAL-WORKSPACE-UNDERSTANDING-ARCHITECTURE.md).

## Batch 7 summary

`WorkspaceKnowledgeSynthesisService` derives provenance-bound
`KnowledgeConcept` / `KnowledgeCluster` / `KnowledgeRelationship` / `KnowledgeGap`
artefacts from durable Programme III projections via `load_snapshot` only
(state, policy, reconstruction, temporal, explanation, **and contextual**).
Diagnostic `KnowledgeConfidence` never becomes Memory truth, Cognitive Model mutation,
policy outcome, automation, or priority authority. Relationships are meaning-only
(`relates_to` / `overlaps` / …) — never causation or action.
Derived knowledge ≠ truth; relationships ≠ causation; confidence ≠ authority.
Commands: `GenerateWorkspaceKnowledgeSynthesis`, `GetWorkspaceKnowledgeSynthesis`,
`GetWorkspaceKnowledgeSummary`, `ExplainKnowledgeSynthesis`
(`work_context.write` / `work_context.read`). Migration `058`.
See [Knowledge Synthesis Architecture](./KNOWLEDGE-SYNTHESIS-ARCHITECTURE.md).

**Do not expand into Memory replacement, Cognitive Model mutation, prediction, simulation,
autonomous correction, or decision ownership.**

## Sequencing rule

Ship Batch *N* only when Batch *N−1* has durable contracts, tests, and governance ownership entries.
Batch 7 is accepted (architecture audit grade A). Batch 8 gate is open — start only with an
explicit Batch 8 charter. Preserve P2 constraints from Batches 6–7:

- descriptive themes / concepts only (no advice / prioritisation / recommendations)
- diagnostic-only confidence
- meaning-only relationships (no causation / action)
- no interpretive layer that merely duplicates Contextual Understanding or Knowledge Synthesis
- Knowledge Synthesis must not become a second editable workspace ontology
- every concept retains evidence → revision → origin domain lineage
- no interpretive layer that merely duplicates Contextual Understanding
  (Batch 7 answers structured knowledge derivation — not “what is happening now”)

## Related

- [Unified Workspace State Architecture](./UNIFIED-WORKSPACE-STATE-ARCHITECTURE.md) (Batch 1)
- [Policy & Governance Architecture](./POLICY-GOVERNANCE-ARCHITECTURE.md) (Batch 2)
- [Historical Workspace Reconstruction Architecture](./HISTORICAL-WORKSPACE-RECONSTRUCTION-ARCHITECTURE.md) (Batch 3)
- [Temporal Intelligence Architecture](./TEMPORAL-INTELLIGENCE-ARCHITECTURE.md) (Batch 4)
- [Workspace Explanation Layer Architecture](./WORKSPACE-EXPLANATION-LAYER-ARCHITECTURE.md) (Batch 5)
- [Contextual Workspace Understanding Architecture](./CONTEXTUAL-WORKSPACE-UNDERSTANDING-ARCHITECTURE.md) (Batch 6)
- [Knowledge Synthesis Architecture](./KNOWLEDGE-SYNTHESIS-ARCHITECTURE.md) (Batch 7)
- [Programme II — Cognitive Workspace](./PROGRAMME-II-COGNITIVE-WORKSPACE.md)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
- [Operational Recovery](../03-Engineering/OPERATIONAL-RECOVERY.md)
