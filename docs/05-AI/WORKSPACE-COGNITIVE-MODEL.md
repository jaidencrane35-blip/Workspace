# Workspace Cognitive Model (Programme II — Batch 1)

| Field | Value |
|-------|-------|
| **Purpose** | Durable semantic layer for objectives, intent relationships, importance, confidence, uncertainty, and focus |
| **Owner** | `WorkspaceCognitiveModelService` (DurableStore) |
| **Status** | Active |
| **Authority effect** | Cognitive nodes/relations never execute, grant, or bypass Gateway |

## What this is

The Cognitive Model is the Workspace's **internal understanding of the user's work** — not another read-only aggregator over Intent/Task Graph.

It stores semantic nodes and relations that existing services (Attention, Purpose, RE, DE, Planner) can reason over.

## What this is not

- Not a second WorkGoal / Project / Task store (Intent remains SoT for those payloads)
- Not a replacement for Milestone Engine projections (those remain coordination views)
- Not embeddings / vector search
- Not a lifecycle system for recommendations, decisions, or execution
- Not executable — selecting focus does not dispatch commands

## Core concepts

| Concept | Kind | Notes |
|---------|------|-------|
| **Goal** | Node (`goal`) | Must reference a durable `WorkGoal` via `external_ref` (`work_goal:<id>`) |
| **Objective** | Node (`objective`) | Measurable outcome under a Goal/Initiative |
| **Initiative** | Node (`initiative`) | Coordinated effort spanning objectives/milestones |
| **Milestone** | Node (`milestone`) | Semantic outcome checkpoint (distinct from Milestone Engine projection) |
| **Context** | Node (`context`) | Situation frame (tools, constraints, environment cues) |
| **Working Set** | Node (`working_set`) | Current material set under focus |
| **Constraint** | Node (`constraint`) | Hard/soft limit the work must respect |
| **Risk** | Node (`risk`) | Uncertain downside with confidence/uncertainty |
| **Opportunity** | Node (`opportunity`) | Uncertain upside |
| **Dependency** | Relation (`depends_on`, `blocks`, …) | Edge between cognitive nodes (and optional external refs) |

## Epistemic fields (every node)

| Field | Meaning |
|-------|---------|
| `importance` | 0–100 relative priority signal |
| `confidence` | 0–100 belief the node is correctly understood |
| `uncertainty` | 0–100 residual unknown (not merely `100 - confidence`) |
| `is_current_focus` | Explicit focus flag (at most one primary focus recommended per workspace) |
| `authority_effect` | Always `none` |

## Ownership

| Concern | Owner |
|---------|-------|
| Cognitive node/relation lifecycle | WorkspaceCognitiveModelService |
| WorkGoal / Project / Task payloads | WorkspaceIntentService |
| Task Graph structural deps | TaskGraphService |
| Milestone Engine coordination view | WorkspaceMilestoneService (Aggregator — consumes cognitive milestones when present) |
| Persistence guards | CognitiveModelRepository |

## Mutation path

```
Actor → CommandPipeline → PermissionGateway → WorkspaceCognitiveModelService → Repository
```

Capability token: `work_context.write` (shared write token — not a lifecycle ownership transfer).

## Snapshot

`CognitiveModelState` is assembled by the owner service for consumers. It is evidence of semantic understanding, not a command envelope. Aggregators must not treat it as mutation authority.

## Programme II forward links

- Batch 2 Planner reads Cognitive Model for decomposition targets
- Batch 3 Reasoning Memory cites cognitive node ids as subjects
- Batch 4 Cognitive Graph generalizes relations across Intent/Task/RE/DE identities
- Batches 5–8 consume this layer; they do not redefine it
