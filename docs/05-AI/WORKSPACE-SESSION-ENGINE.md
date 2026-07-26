# Workspace Session Engine

Phase 6 Batch 1 / Sprint 94.

## Principle

The Session Engine answers: **What is happening in this workspace right now?**

It is a runtime orchestration projection over Workspace Intelligence. It is not a planner, executor, automation engine, or second intelligence system.

## Domain

- `WorkspaceSessionState` — full runtime snapshot
- `SessionSummary`, `SessionMember`, `SessionFocus`, `SessionTimeline`
- `SessionDecisionRef`, `SessionRecommendationRef`, `SessionReadinessView`
- `SessionRisk`, `SessionHealth`, `SessionInterruption`, `SessionMomentum`

Every element references an existing projection (`source_projection` + `source_ref` + `why`).

## Ownership

Session **owns nothing**. Intelligence feeds Session. Session feeds UI.

Independent remain: Decision Queue, Recommendation Engine, Readiness, Adaptation, Continuity, Operating State, Pattern, …

## Governance

- Zero authority (`authority_effect: none`)
- Audits: `workspace.session.generated`, `.projected`, `.updated`
- `attempt_execute` always fails
- No launch, restore, prepare, plan, or Gateway bypass

## Position

```
Human Intent
  → Workspace Understanding (Intelligence + cognition)
  → Session (runtime orchestration for UI)
  → Planning (human-initiated)
  → Permission Gateway
  → Execution
```
