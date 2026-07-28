# Workspace Decision Support Architecture (Programme III — Batch 11)

| Field | Value |
|-------|-------|
| **Purpose** | Organise existing Programme III evidence into **decision-ready support packages** — without making decisions, recommending actions, or replacing Decision Engine / Recommendation Engine / Policy / Intent / human judgement |
| **Owner** | `WorkspaceDecisionSupportService` (DurableStore — **decision support artefacts only**) |
| **Status** | Active — Programme III Batch 11 implemented |
| **Lifecycle owner** | No |
| **Decision / recommendation / execution authority** | No |
| **Policy / permission / autonomy / planning** | No |
| **Memory / Cognitive Model / Knowledge / Insight / Cross-Workspace replacement** | No |
| **Source of truth** | No |

## Core principle

**Support decisions. Never become the decision-maker.**

```
Programme III evidence stack (Batches 1–10)
        │  load_snapshot only
        ▼
Workspace Decision Support
        │
        ▼
Decision support packages (read-only)
        │
        ▼
Human judgement / Decision Engine / Gateway (elsewhere)
```

### Locked invariants

- **support ≠ decision**
- **trade-off ≠ recommendation**
- **comparison ≠ ranking-as-authority**
- **completeness ≠ permission**
- **confidence ≠ approval**

## Ownership

### Owns

- decision context packages (`DecisionContext`)
- supporting evidence collections (`EvidenceBundle`)
- trade-off summaries (descriptive only)
- option comparisons (descriptive only)
- evidence completeness / uncertainty summaries
- decision dependencies (evidence-only)
- evidence lineage
- dual-channel projection

### Never owns

decisions, recommendations, execution, permissions, policy authority, lifecycle,
autonomy, planning, task management, Cognitive Model, knowledge, insight
coordination, cross-workspace intelligence.

## Allowed inputs

Via `load_snapshot` only: State, Policy, Reconstruction, Temporal, Explanation,
Contextual, Knowledge Synthesis, Knowledge Integration, Insight Coordination,
Cross-Workspace Intelligence.

Never: `generate`, foreign repositories, lifecycle, Decision Engine, Recommendation execution paths.

## Domain model

`WorkspaceDecisionSupportSnapshot` with `DecisionContext`, `EvidenceBundle`,
`TradeoffSummary`, `DecisionDependency`, `DecisionSupportGap`.

Projection: `current` / `history` / authoritative `history_count`.
History always `actionable: false`.

## Commands

| Command | Capability |
|---------|------------|
| `GenerateWorkspaceDecisionSupport` | `work_context.write` |
| `GetWorkspaceDecisionSupport` | `work_context.read` |
| `GetWorkspaceDecisionSupportSummary` | `work_context.read` |
| `ExplainWorkspaceDecisionSupport` | `work_context.read` |

## Persistence

Migration `062_workspace_decision_support.sql` · Repository `WorkspaceDecisionSupportRepository`.

## Governance

Mutation baseline **72** · DTO inventory **23**.

## Governing rule

> Workspace Decision Support organises evidence for human and system understanding only.
> It never becomes the decision-maker, recommendation engine, or execution authority.

## Related

- [Programme III](./PROGRAMME-III-COHERENT-WORKSPACE-RUNTIME.md)
- [Cross-Workspace Intelligence](./CROSS-WORKSPACE-INTELLIGENCE-ARCHITECTURE.md) (Batch 10)
- [Insight Coordination](./INSIGHT-COORDINATION-ARCHITECTURE.md) (Batch 9)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
