# Workspace Intelligence Hub Architecture (Programme III — Batch 12)

| Field | Value |
|-------|-------|
| **Purpose** | Single **read-only aggregation surface** over all Programme III intelligence artefacts — one coherent observational package for operators and AI systems |
| **Owner** | `WorkspaceIntelligenceHubService` (DurableStore — **hub packages only**) |
| **Status** | Active — Programme III Batch 12 implemented |
| **Lifecycle / decision / recommendation / execution authority** | No |
| **Upstream intelligence ownership** | No — references only |
| **Source of truth** | No |

## Core principle

**Aggregate intelligence. Never replace the intelligence that produced it.**

```
Batches 1–11 (each authoritative for own artefacts)
        │  load_snapshot only
        ▼
Workspace Intelligence Hub
        │
        ▼
Observational intelligence package (read-only)
```

### Locked invariants

- **aggregation ≠ reinterpretation**
- **hub package ≠ new SoT**
- **summary ≠ recommendation**
- **conflict record ≠ resolution**
- **lineage ≠ inferred provenance**
- **confidence ≠ permission**

## Ownership

### Owns

- intelligence packages / summaries
- cross-layer aggregation references
- provenance rollups / completeness summaries
- intelligence gaps / conflicts / lineage
- dual-channel projection

### Never owns

workspace state, policy, history, temporal, explanation, contextual, knowledge,
insight coordination, cross-workspace intelligence, decision support,
recommendations, decisions, execution, permissions, lifecycle, planning, autonomy.

## Allowed inputs

Via `load_snapshot` only — Batches 1–11 surfaces listed in the implementation brief.

Never: `generate`, foreign repositories, mutating services, execution paths, foreign refresh.

## Domain model

`WorkspaceIntelligenceHubSnapshot` with `IntelligencePackage`, `IntelligenceSummary`,
`IntelligenceLineage`, `IntelligenceGap`, `IntelligenceConflict`.

Projection: `current` / `history` / authoritative `history_count`.
History always `actionable: false`.

## Commands

| Command | Capability |
|---------|------------|
| `GenerateWorkspaceIntelligenceHub` | `work_context.write` |
| `GetWorkspaceIntelligenceHub` | `work_context.read` |
| `GetWorkspaceIntelligenceHubSummary` | `work_context.read` |
| `ExplainWorkspaceIntelligence` | `work_context.read` |

## Persistence

Migration `063_workspace_intelligence_hub.sql` · `WorkspaceIntelligenceHubRepository`.

## Governance

Mutation baseline **73** · DTO inventory **24**.

## Governing rule

> Workspace Intelligence Hub aggregates Programme III intelligence into a single
> observational package only. It never replaces, overrides, or becomes the
> authority for any upstream intelligence layer.

## Related

- [Programme III](./PROGRAMME-III-COHERENT-WORKSPACE-RUNTIME.md)
- [Workspace Decision Support](./WORKSPACE-DECISION-SUPPORT-ARCHITECTURE.md) (Batch 11)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
