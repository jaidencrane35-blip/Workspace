# Planning Architecture (Programme II — Batch 2)

| Field | Value |
|-------|-------|
| **Purpose** | Higher-order, durable, permanently non-executing planning layer |
| **Owner** | `WorkspacePlanningService` (DurableStore) |
| **Status** | Active |
| **Authority effect** | Planning artefacts never execute, dispatch, claim, approve, or grant |

## What this is

The Cognitive Planning Engine converts workspace cognitive state into structured
planning artefacts: goals pursued, sequenced steps, assumptions, constraints,
risks, gaps, and recommended next inspection points.

It **composes** existing systems. It does not replace them.

```
Cognitive Model → Intent → Context → Task Graph → RE → DE → Attention → Purpose → Memory
        ↓
WorkspacePlanningService
        ↓
PlanningSnapshot (current proposal + history + history_count)
        ↓
React (projection-only)
```

Planning terminates at the snapshot. Execution begins elsewhere.

## What this is not

- Not a replacement for Intent, Task Graph, Decision Engine, Recommendation Engine,
  AiPlan / AiOrchestration, or Execution Lifecycle
- Not an executor — never dispatch / claim / complete / accept / dismiss / approve /
  spawn / launch / mutate foreign lifecycles
- Not a PermissionGateway client for `require`
- Not a second ownership surface for WorkGoals, Tasks, Recommendations, or Decisions

## Ownership

| Concern | Owner |
|---------|-------|
| Sequencing, decomposition, ordering, rationale, assumptions, uncertainty | WorkspacePlanningService |
| Intent WorkGoal / Project / Task payloads | WorkspaceIntentService |
| Task Graph structural lifecycle | TaskGraphService |
| Recommendation lifecycle | WorkspaceRecommendationEngineService |
| Decision overlays / candidates | DecisionEngineService / DecisionQueueService |
| Execution claim/complete/fail | ExecutionLifecycleService |
| Permissions | PermissionGateway via CommandPipeline |
| Persistence guards (planning tables only) | WorkspacePlanningRepository |

## Planning model (durable)

| Entity | Role |
|--------|------|
| PlanningPlan | Durable plan root (`active` / `superseded` / `abandoned`) |
| PlanningStep | Ordered decomposition step with evidence refs |
| PlanningSection | Presentation grouping of steps |
| PlanningAssumption | Stated belief with confidence |
| PlanningRisk | Downside / uncertainty signal |
| PlanningGap | Missing information |
| PlanningDependency | Step ordering edge (acyclic) |
| PlanningAlternative | Advisory alternate sequence |
| PlanningConstraintReference | Reference to foreign constraint |
| PlanningEvidenceReference | Reference to Goal/Task/RE/DE/Memory/… |
| PlanningExplanation | Why / next (non-command) |
| PlanningSnapshot | Dual-channel projection |
| PlanningHistoryEntry | Terminal evidence only |
| PlanningSummary | Compact projection with authoritative `history_count` |

All snapshots are **read-only projections**. No command fields. No executable state.

## Projection contract

| Channel | Contents | Actionable? |
|---------|----------|-------------|
| `current` | Active `PlanningProposal` (at most one) | View/inspect only — never execute |
| `history` | Superseded / abandoned plans | Never (`actionable: false`) |
| `history_count` | Full terminal count | Scalar authority |

React may view, expand, collapse, compare plans, and inspect rationale / risks /
assumptions. No execution controls. No approval buttons. No planning mutation
ownership in the UI.

## Mutation path

```
Actor → CommandPipeline → PermissionGateway → WorkspacePlanningService → Repository
```

| Command | Kind | Capability |
|---------|------|------------|
| `GeneratePlanningSnapshot` | Mutation | `work_context.write` |
| `GetPlanningSnapshot` | Query | `work_context.read` |
| `GetPlanningSummary` | Query | `work_context.read` |

## Relationships (reference-only)

Planning may **reference** Goal, Objective, Initiative, Task, Decision,
Recommendation, Execution, Memory, Constraint, Risk, Opportunity, Context.
It must not duplicate canonical ownership or mutate those stores.

## Recovery

Planning participates in recovery only as durable state. Restart reconstructs
snapshots from planning tables. No planning replay. No execution recovery. No
planner-owned lifecycle.

## Governance

- Planning services must not import repositories from outside planning tables for
  lifecycle writes of foreign domains (composition reads via owning services only).
- Planning must not invoke execution, process launch, or `PermissionGateway::require`.
- History / summary DTOs scanned for forbidden authority fields.
- Mutation inventory baseline includes `GeneratePlanningSnapshot`.
- `WorkspacePlanningService` is sole planning lifecycle owner.

## Related

- [Programme II — Cognitive Workspace](./PROGRAMME-II-COGNITIVE-WORKSPACE.md)
- [Workspace Cognitive Model](./WORKSPACE-COGNITIVE-MODEL.md)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
