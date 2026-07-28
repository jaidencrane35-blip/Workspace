# Cross-Workspace Intelligence Architecture (Programme III — Batch 10)

| Field | Value |
|-------|-------|
| **Purpose** | Discover **derived aggregate observations** across independent workspaces (recurring patterns, themes, risks, constraints, statistics) while preserving each workspace as authoritative for its own state |
| **Owner** | `WorkspaceCrossIntelligenceService` (DurableStore — **cross-workspace aggregate artefacts only**) |
| **Status** | Active — Programme III Batch 10 implemented |
| **Lifecycle owner** | No |
| **Execution / replay authority** | No |
| **Planner / Recommendation / Decision / Autonomy** | No |
| **Policy / permission authority** | No |
| **Per-workspace SoT / Memory / Cognitive Model replacement** | No |
| **Knowledge Synthesis / Insight Coordination replacement** | No |
| **Source of truth** | No — aggregate observations only |
| **Autonomous authority** | No |

## Core principle

**Aggregate understanding. Never centralise authority.**

```
Workspace A / B / C … (each authoritative for own state)
        │
        ▼  load_snapshot only
Cross-Workspace Intelligence
        │
        ▼
Derived aggregate observations (read-only)
```

Not:

```
Cross-Workspace Intelligence
        │
        ▼
global brain / central SoT / execute / decide / replace workspace truth
```

### Locked invariants

- **aggregation ≠ authority**
- **frequency ≠ priority-as-action**
- **common theme ≠ shared truth**
- **confidence ≠ permission**
- **statistics ≠ recommendations**

## Architectural position

```
… Programme III Batches 1–9 (per-workspace evidence stack) …
        ↓
Cross-Workspace Intelligence (Batch 10)
        ↓
Cross-workspace aggregate artefacts (read-only)
```

Batch 10 sits **above** per-workspace layers. It never mutates them and never becomes a second ontology for any workspace.

## Ownership

### `WorkspaceCrossIntelligenceService` owns

- recurring patterns (`CrossWorkspacePattern`)
- aggregate themes (`CrossWorkspaceTheme`)
- repeated risk signals (`CrossWorkspaceRiskSignal`) — diagnostic only
- repeated constraint patterns (`CrossWorkspaceConstraintPattern`)
- cross-workspace knowledge clusters / evidence statistics
- confidence + evidence lineage
- dual-channel projection (`current` / `history` / authoritative `history_count`)
- persistence of **derived aggregate artefacts only** (global scope)

### Does **not** own

workspace state, intent, execution, planning, policy, recommendations, permissions,
autonomy, lifecycle, Cognitive Model, Knowledge Synthesis, Insight Coordination,
or any per-workspace source of truth.

## Allowed inputs

Per workspace, via `load_snapshot()` only:

Unified Workspace State, Policy & Governance, Historical Reconstruction,
Temporal Intelligence, Explanation, Contextual Understanding, Knowledge Synthesis,
Knowledge Integration, Insight Coordination.

**Never:** `generate`, foreign repositories, lifecycle services, direct mutation.

## Domain model

### `CrossWorkspaceIntelligenceSnapshot`

Primary aggregate artefact — **read-only**.

Contains: patterns, themes, risk signals, constraint patterns, gaps, assessment,
provenance, diagnostic confidence, `authority_effect: none`, `actionable: false`.

Projection: `CrossWorkspaceIntelligenceProjection` with `current` / `history` /
authoritative `history_count`. History always evidence-only / non-actionable.

### Required concepts

| Type | Role |
|------|------|
| `CrossWorkspacePattern` | Recurring pattern across participating workspaces |
| `CrossWorkspaceTheme` | Semantic theme spanning workspaces |
| `CrossWorkspaceRiskSignal` | Repeated risk — diagnostic only, never proposes action |
| `CrossWorkspaceConstraintPattern` | Recurring constraint evidence — no optimisation / planning |
| `CrossWorkspaceGap` | Missing cross-workspace understanding |

## Commands

| Command | Kind | Capability |
|---------|------|------------|
| `GenerateCrossWorkspaceIntelligence` | Mutation | `work_context.write` |
| `GetCrossWorkspaceIntelligence` | Query | `work_context.read` |
| `GetCrossWorkspaceSummary` | Query | `work_context.read` |
| `ExplainCrossWorkspacePattern` | Query | `work_context.read` |

Path: React → IPC → CommandPipeline → PermissionGateway → Service.

## Persistence

- Migration `061_workspace_cross_intelligence.sql`
- Repository `WorkspaceCrossIntelligenceRepository` (persistence only)
- Transactional supersede + append-only history
- Global scope key (not a replacement for per-workspace stores)

## Recovery

Missing / unavailable workspaces remain missing / unavailable.
Never invent global patterns, fabricate statistics, or synthesise absent evidence.

## Governance (implementation)

| Knob | Value |
|------|-------|
| Mutation baseline | **71** |
| History / projection DTO inventory | **22** |
| Guards | no foreign `::generate`, no Recommendation/Decision/lifecycle imports, repo≠service |
| IPC error | `cross_workspace_intelligence_validation_error` |

## Governing rule

> Cross-Workspace Intelligence aggregates understanding only.
> It never centralises authority or replaces the authority of individual workspaces.

## Related

- [Programme III — Coherent Workspace Runtime](./PROGRAMME-III-COHERENT-WORKSPACE-RUNTIME.md)
- [Insight Coordination Architecture](./INSIGHT-COORDINATION-ARCHITECTURE.md) (Batch 9)
- [Knowledge Integration Architecture](./KNOWLEDGE-INTEGRATION-ARCHITECTURE.md) (Batch 8)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
