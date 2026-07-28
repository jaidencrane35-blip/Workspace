# Programme III — Coherent Workspace Runtime

| Field | Value |
|-------|-------|
| **Purpose** | Establish one provenance-rich, read-only runtime view of operational, cognitive, governance, and evidence state while preserving all existing lifecycle and persistence authorities |
| **Status** | Active — Batches 1–9 accepted (Batch 9 Insight Coordination implemented) |
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
17. **Knowledge integration retrieves and joins evidence layers** — it does not invent truth, act autonomously, or replace Memory / Cognitive Model.
18. **integration ≠ authority; retrieval ≠ truth; relevance ≠ correctness; confidence ≠ permission.**
19. **Insight coordination coordinates already-derived understanding** — it does not plan, decide, recommend execution, or create authority.
20. **coordination ≠ authority; prioritisation ≠ recommendation; intersection ≠ causation.**

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
| **8** | Knowledge Integration / Retrieval Intelligence | Integrate/retrieve over accumulated evidence layers | Done (accepted) |
| **9** | Workspace Insight Coordination | Coordinate cross-layer evidence relationships / unresolved areas | Done (accepted) |

## Batch 8 summary (evidence-stack close)

`WorkspaceKnowledgeIntegrationService` assembles provenance-bound
`KnowledgeIntegrationResult` artefacts over durable Programme III projections via
`load_snapshot` only. Batch 8 closed the coherent **evidence stack**.
See [Knowledge Integration Architecture](./KNOWLEDGE-INTEGRATION-ARCHITECTURE.md).

## Batch 9 summary

`WorkspaceInsightCoordinationService` assembles provenance-bound coordination
views (`InsightCluster` / `InsightIntersection` / `InsightGap` /
`InsightAttentionSignal` / `CoordinationAssessment`) over durable Programme III
projections via `load_snapshot` only — including Knowledge Integration.
Diagnostic prioritisation metadata never becomes recommendation, task, policy,
autonomy, or permission authority. Intersections remain meaning-only.
Clusters without lineage are invalid.
Commands: `GenerateInsightCoordination`, `GetInsightCoordination`,
`GetInsightCoordinationSummary`, `ExplainInsightCoordination`
(`work_context.write` / `work_context.read`). Migration `060`.
See [Insight Coordination Architecture](./INSIGHT-COORDINATION-ARCHITECTURE.md).

**Do not begin Batch 10 until Batch 9 receives architecture acceptance.**

**Preserve:** no new source of truth, no autonomous authority, no execution,
no planner / decision / recommendation ownership, no replacement of Memory /
Cognitive Model / Knowledge Synthesis / Knowledge Integration.

Governing rule:

> Coordinate understanding. Never create authority.

> The workspace may understand more. It must never silently gain the power to decide more.

## Sequencing rule

Ship Batch *N* only when Batch *N−1* has durable contracts, tests, and governance ownership entries.
Batches 1–9 are accepted. Batch 10 is gated on Batch 9 architecture acceptance.

Preserve P2 constraints from Batches 6–9:

- descriptive themes / concepts / hits / clusters only (no advice / action queues)
- diagnostic-only confidence and attention metadata
- meaning-only relationships (no causation / action)
- prioritisation ≠ recommendation
- no interpretive layer that merely duplicates Contextual Understanding, Knowledge Synthesis, or Knowledge Integration
- every cluster / intersection retains evidence → revision → origin domain lineage

## Related

- [Unified Workspace State Architecture](./UNIFIED-WORKSPACE-STATE-ARCHITECTURE.md) (Batch 1)
- [Policy & Governance Architecture](./POLICY-GOVERNANCE-ARCHITECTURE.md) (Batch 2)
- [Historical Workspace Reconstruction Architecture](./HISTORICAL-WORKSPACE-RECONSTRUCTION-ARCHITECTURE.md) (Batch 3)
- [Temporal Intelligence Architecture](./TEMPORAL-INTELLIGENCE-ARCHITECTURE.md) (Batch 4)
- [Workspace Explanation Layer Architecture](./WORKSPACE-EXPLANATION-LAYER-ARCHITECTURE.md) (Batch 5)
- [Contextual Workspace Understanding Architecture](./CONTEXTUAL-WORKSPACE-UNDERSTANDING-ARCHITECTURE.md) (Batch 6)
- [Knowledge Synthesis Architecture](./KNOWLEDGE-SYNTHESIS-ARCHITECTURE.md) (Batch 7)
- [Knowledge Integration Architecture](./KNOWLEDGE-INTEGRATION-ARCHITECTURE.md) (Batch 8)
- [Insight Coordination Architecture](./INSIGHT-COORDINATION-ARCHITECTURE.md) (Batch 9)
- [Programme II — Cognitive Workspace](./PROGRAMME-II-COGNITIVE-WORKSPACE.md)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
- [Operational Recovery](../03-Engineering/OPERATIONAL-RECOVERY.md)
