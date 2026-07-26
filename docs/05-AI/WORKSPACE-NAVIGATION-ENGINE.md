# Workspace Navigation Engine

Phase 6 Batch 4 / Sprint 97.

## Principle

The Navigation Engine answers: **Where should I go next in this work?**

It is an interaction projection over existing Workspace understanding. It is not a planner, executor, router, or cognition system.

## Domain

- `WorkspaceNavigationState` — full navigation snapshot
- `NavigationPath` / `NavigationNode` / `NavigationEdge`
- `NavigationPathKind` — Current Focus · Suggested Destination · Related Work · Blocking Items · Connected Tasks/Projects/Contexts · Decisions · Recommendations · Recent Changes · Next Inspection · Dependency Chain · Breadcrumbs
- `NavigationRelationKind` — Current · Related · Depends On · Blocked By · Supports · Leads To · Recently Visited · Suggested Next · Dormant · Disconnected

Every node and edge explains **why**. No hidden reasoning.

## Ownership

Navigation **owns nothing**. Paths are deterministic pointers into Session, Experience, Work Context, and Intelligence outputs.

## Intelligence

`WorkspaceIntelligenceState.navigation` embeds after Work Context.

Attention, Recommendation Engine, Adaptation, and Session may **reference** Navigation as evidence only.

## Governance

- Zero authority (`authority_effect: none`)
- Audits: `workspace.navigation.generated`, `.updated`, `.validated`
- `attempt_execute` always fails
- Reuses existing `work_context.read` — no new Gateway paths
- Must never: plan, execute, launch, restore, autonomously route, persist, or predict

## Position

```
Human Intent
  → Workspace Understanding
  → Workspace Navigation
  → Planning
  → Decision
  → Permission Gateway
  → Execution
```
