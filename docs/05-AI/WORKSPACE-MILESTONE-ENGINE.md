# Workspace Milestone Engine

Phase 6 Batch 5 / Sprint 98.

## Principle

The Milestone Engine answers: **What meaningful outcome am I progressing toward?**

It is a read-only coordination layer that projects milestones from existing Workspace understanding. It is not a planner, scheduler, executor, or cognition system.

## Domain

- `WorkspaceMilestoneState` — full milestone snapshot
- `WorkspaceMilestone` / `MilestoneRelationship` / `MilestoneSummary`
- Statuses — Current · Upcoming · Blocked · Completed
- Relations — Current · Upcoming · Blocked · Completed · Depends On · Contributes To · Supersedes · Supports
- Readiness bands — Ready · Partially Ready · Blocked · Unknown (observational only)

Every milestone explains **why** it exists. No hidden reasoning.

## Ownership

Milestones **own nothing**. They are deterministic pointers into Session, Experience, Work Context, Navigation, Task Graph, Purpose, Decision Queue, Readiness, Evolution, and related Intelligence outputs.

## Intelligence

`WorkspaceIntelligenceState.milestones` embeds after Navigation.

Attention, Recommendation Engine, Adaptation, Session, and Navigation may **reference** Milestones as evidence only. Milestones never become a source of truth.

## Governance

- Zero authority (`authority_effect: none`)
- Audits: `workspace.milestones.generated`, `.updated`, `.validated`
- `attempt_execute` always fails
- Reuses existing `work_context.read` — no new Gateway paths
- Must never: plan, schedule, auto-complete, predict, execute, persist, or authorize

## Position

```
Human Intent
  → Workspace Understanding
  → Workspace Navigation
  → Workspace Milestones
  → Planning
  → Decision
  → Permission Gateway
  → Execution
```
