# Cognitive Graph Architecture (Programme II — Batch 4)

| Field | Value |
|-------|-------|
| **Purpose** | Cross-domain integration topology over existing identities |
| **Owner** | `WorkspaceCognitiveGraphService` (DurableStore — **topology / evidence only**) |
| **Status** | Active |
| **Lifecycle owner** | No |
| **Execution authority** | No |
| **Planning authority** | No |
| **Reasoning authority** | No |

## What this is

The Cognitive Graph answers:

- What is connected?
- Why is it connected?
- What evidence supports that connection?
- What broken references are observable?

It is an **observational topology**, not an ownership model.

## What this is not

- Not a second source of truth for Goal / Task / Plan / Decision / Recommendation
- Not a planner, executor, lifecycle system, or permission authority
- Not allowed to invent missing relationships
- Not allowed to delete underlying domain objects when graph projections change

## Ownership

| Concern | Owner |
|---------|-------|
| Node / edge / snapshot projections + graph metadata | WorkspaceCognitiveGraphService |
| Cognitive semantic nodes | WorkspaceCognitiveModelService |
| Planning | WorkspacePlanningService |
| Reasoning evidence | WorkspaceReasoningMemoryService |
| Intent / Task Graph / RE / DE / Execution | Existing domain services |
| Persistence guards | CognitiveGraphRepository |

## Model

| Type | Role |
|------|------|
| `CognitiveGraphNode` | Projection of an existing identity (`external_ref` is canonical) |
| `CognitiveGraphEdge` | Typed meaning-only edge (`depends_on`, `supports`, `derived_from`, …) |
| `CognitiveGraphView` | Current snapshot contents |
| `CognitiveGraphSnapshot` | Dual-channel projection |
| `CognitiveGraphHistoryEntry` | Terminal evidence only |
| `CognitiveGraphSummary` | Compact projection with authoritative `history_count` |

Edges never imply authority. Broken endpoints become `broken_reference` evidence —
never fabricated relationships.

## Projection contract

| Channel | Contents | Actionable? |
|---------|----------|-------------|
| `current` | Active graph view | View/inspect only |
| `history` | Superseded snapshots (append-only) | Never |
| `history_count` | Full terminal count | Scalar authority |

## Mutation path

```
Actor → CommandPipeline → PermissionGateway → WorkspaceCognitiveGraphService → Repository
```

| Command | Kind | Capability |
|---------|------|------------|
| `GenerateCognitiveGraph` | Mutation | `work_context.write` |
| `GetCognitiveGraph` | Query | `work_context.read` |
| `GetCognitiveGraphSummary` | Query | `work_context.read` |

Generate supersedes prior `current` and appends history inside `Database::run_in_transaction`.

## Prohibited patterns

- execute / launch / dispatch / claim / complete
- create tasks / recommendations / decisions
- mutate foreign lifecycle
- invent missing relationships
- Gateway / Pipeline bypass
- replace domain identities with graph-owned ids
- delete underlying domain objects when removing graph nodes

## Recovery

Restart reconstructs durable current + history. Missing graph remains missing.
Broken references stay observable. Never fabricate structure or invent edges.

## Related

- [Programme II — Cognitive Workspace](./PROGRAMME-II-COGNITIVE-WORKSPACE.md)
- [Planning Architecture](./PLANNING-ARCHITECTURE.md)
- [Reasoning Memory Architecture](./REASONING-MEMORY-ARCHITECTURE.md)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
