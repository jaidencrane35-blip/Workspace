# Workspace Task Graph

| Field | Value |
|-------|-------|
| **Purpose** | Define the persistent canonical model of workspace work |
| **Owner** | Architecture |
| **Status** | Phase 5 Batch 2 foundation (Sprints 82–83) |
| **Authority** | Never creates authority (`authority_effect: none`) |

---

## Principle

```
Decision Engine
        ↓
Workspace Task Graph   ← this document
        ↓
Planner
        ↓
Proposal Evaluation
        ↓
Permission Gateway
        ↓
Execution
```

The Task Graph **models work**. It does not recommend, plan, authorize, or execute.

---

## Domain (Sprint 82)

- `WorkspaceTask` — durable work item (graph SoT)
- `TaskNode` — addressable node with dependency/blocker views
- `TaskRelationship` — `depends_on` | `blocks` | `related_to` | `child_of` | `parent_of`
- `TaskGraph` / `TaskGraphSummary`
- Lifecycle: `proposed` → `planned` → `waiting` / `in_progress` / `blocked` → `completed` | `cancelled`
- Persistence: migration `018_workspace_task_graph.sql`

Intent `work_tasks` remain project CRUD; the graph projects them into nodes and owns relationships/progress.

---

## Integration (Sprint 83)

| Layer | Consumption |
|-------|-------------|
| Attention | Prioritizes open graph nodes (blocked/waiting/in progress) |
| Decision Engine | References graph; skips completed work; emits graph-aware candidates |
| Intelligence | Embeds `task_graph` summary |
| Planner | `get_task_graph_planning_inputs` — open incomplete nodes only |

---

## Audits

`task_graph.created` / `task_graph.updated` / `task_node.created` / `task_node.updated` / `task_node.completed` / `task_relationship.created` / `task_relationship.removed` — all `authority_effect: none`.

---

## Boundaries

May: represent work, dependencies, progress, planning context.  
Must not: execute, grant permissions, call the OS, bypass Planner or Gateway.
