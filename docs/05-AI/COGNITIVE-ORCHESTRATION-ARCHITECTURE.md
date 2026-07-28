# Cognitive Orchestration Architecture (Programme II — Batch 5)

| Field | Value |
|-------|-------|
| **Purpose** | Governed coordination of cognitive artefact refresh ordering and staleness |
| **Owner** | `WorkspaceCognitiveOrchestrationService` (DurableStore — **coordination / evidence only**) |
| **Status** | Active |
| **Lifecycle owner** | No |
| **Execution authority** | No |
| **Planning authority** | No |
| **Reasoning authority** | No |
| **Graph authority** | No |

## What this is

The Orchestration Engine answers:

- Which cognitive artefacts are stale?
- Which artefact should regenerate first?
- Which artefacts depend on each other?
- Can regeneration happen independently?
- What ordering minimizes inconsistency?
- What evidence is blocking regeneration?
- What refreshes were intentionally skipped?

It produces **orchestration recommendations only** — ordered refresh stages and observations.

## What this is not

- Not another planner
- Not another lifecycle owner
- Not an autonomy layer
- Not an execution engine
- Not allowed to refresh Cognitive Model / Planning / Reasoning / Graph by itself
- Not allowed to invent dependency resolutions for cycles

## Ownership

| Concern | Owner |
|---------|-------|
| Orchestration state / plans / history / evidence | WorkspaceCognitiveOrchestrationService |
| Cognitive Model | WorkspaceCognitiveModelService |
| Planning | WorkspacePlanningService |
| Reasoning | WorkspaceReasoningMemoryService |
| Cognitive Graph | WorkspaceCognitiveGraphService |
| Intent / Task / RE / DE / Execution | Existing domain services |
| Persistence guards | CognitiveOrchestrationRepository |

## Model

| Type | Role |
|------|------|
| `WorkspaceOrchestrationView` | Current coordination snapshot |
| `WorkspaceOrchestrationSnapshot` | Dual-channel projection |
| `OrchestrationHistoryEntry` | Terminal evidence only |
| `WorkspaceOrchestrationSummary` | Compact projection with authoritative `history_count` |
| `OrchestrationRefreshStage` | Descriptive refresh stage (never executed here) |
| `OrchestrationCycleDetected` | Observable cycle evidence — no invented break |

## Dependency scheduling

Builds a DAG over existing artefacts (default: Cognitive Model → Planning → Reasoning → Graph).
Cycles emit `CycleDetected` evidence and leave remaining nodes unordered — never invent a resolution.

## Projection contract

| Channel | Contents | Actionable? |
|---------|----------|-------------|
| `current` | Active orchestration view | View/inspect only (`terminal=false`, `actionable=false`) |
| `history` | Superseded snapshots (append-only) | Never |
| `history_count` | Full terminal count | Scalar authority |

`authority_effect` is always `"none"`. No command fields. No lifecycle. No execution.

## Mutation path

```
Actor → CommandPipeline → PermissionGateway → WorkspaceCognitiveOrchestrationService → Repository
```

| Command | Kind | Capability |
|---------|------|------------|
| `GenerateWorkspaceOrchestration` | Mutation | `work_context.write` |
| `GetWorkspaceOrchestration` | Query | `work_context.read` |
| `GetWorkspaceOrchestrationSummary` | Query | `work_context.read` |

Generate supersedes prior `current` and appends history inside `Database::run_in_transaction`.
Composition **reads** foreign snapshots only — it does not call their generate paths as plan execution.

## Prohibited patterns

- execute / launch / dispatch / claim / complete
- mutate Intent / Task Graph
- accept recommendations / select decisions
- execute refresh plans
- Gateway / Pipeline bypass
- fabricate orchestration, dependencies, or refresh plans
- repository calling orchestration service

## Recovery

Restart reconstructs durable current + history. Missing orchestration remains missing.
Missing evidence remains missing. Transaction rollback leaves no partial snapshot.
Never fabricate orchestration state.

## Related

- [Programme II — Cognitive Workspace](./PROGRAMME-II-COGNITIVE-WORKSPACE.md)
- [Cognitive Graph Architecture](./COGNITIVE-GRAPH-ARCHITECTURE.md)
- [Planning Architecture](./PLANNING-ARCHITECTURE.md)
- [Reasoning Memory Architecture](./REASONING-MEMORY-ARCHITECTURE.md)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
- [Operational Recovery](../03-Engineering/OPERATIONAL-RECOVERY.md)
