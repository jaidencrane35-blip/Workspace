# Workspace Transition Engine

Phase 6 Batch 7 / Sprint 100.

## Principle

The Transition Engine answers: **Where was I, what am I entering, and what changed?**

It is a read-only semantic layer that explains movement between Workspace states. It is not restoration, automation, scheduling, or execution.

## Domain

- `WorkspaceTransitionState` — full transition snapshot
- `WorkspaceTransition` / `TransitionRelationship` / `TransitionSummary`
- Kinds — Entering Context · Leaving Context · Returning To Work · Switching Focus · Continuing Interrupted Work · Completing Work State · Starting New Work State
- Relations — Previous · Current · Returned From · Interrupted By · Continued Into · Related To · Blocked By

Every transition includes previous/current state, changed elements, evidence, confidence, explanation, and why. No hidden reasoning.

## Ownership

Transition **owns nothing**. Activity Graph, Session, Experience, Work Context, Continuity, and Working Style remain sources of truth. Transition only compares and explains.

## Intelligence

`WorkspaceIntelligenceState.transition` embeds after Working Style.

Attention, Recommendation Engine, Session, Experience, Navigation, and Adaptation may **reference** Transitions as evidence only. Transitions never trigger actions.

## Governance

- Zero authority (`authority_effect: none`)
- Audits: `workspace.transition.generated`, `.updated`, `.validated`
- `attempt_execute` always fails
- Reuses existing `work_context.read` — no new Gateway paths
- Must never: restore windows, launch apps, move layouts, automate, schedule, persist, or authorize

## Position

```
Human Intent
  → Workspace Understanding
  → Transition Understanding
  → Planning
  → Decision
  → Permission Gateway
  → Execution
```
