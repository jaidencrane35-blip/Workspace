# Workspace Interaction Model

Phase 6 Batch 2 / Sprint 101.

## Principle

The Interaction Model answers: **What meaningful things can the user interact with right now?**

It is a read-only projection layer that converts existing Workspace understanding into user-facing interaction opportunities. It is not a planner, executor, recommendation engine, or automation system.

## Domain

- `WorkspaceInteractionState` — full interaction snapshot
- `InteractionItem` / `InteractionSummary` / `InteractionHandoff`
- Kinds — Continue Work · Review Decision · Inspect Recommendation · Review Adaptation · Resolve Blocker · Open Context · Review Progress · Understand Change

Every item includes title, description, explanation, available action, required intent, priority, evidence, and **why** (“Why am I seeing this?”). No hidden reasoning.

## Ownership

Interaction **owns nothing**. Decision Queue, Recommendation Engine, Adaptation, Continuity, Readiness, Milestones, Navigation, Session, Experience, and Transition remain sources of truth. Interaction only aggregates and surfaces.

Selecting an interaction creates an **Intent handoff** (`submit_assistant_goal`) — never executes.

## Intelligence

`WorkspaceIntelligenceState.interaction` embeds after Transition:

```
Transition → Interaction → Intelligence envelope
```

## Governance

- Zero authority (`authority_effect: none`)
- Audits: `workspace.interaction.generated`, `.updated`, `.selected`, `.handoff_created`
- `attempt_execute` always fails
- Gates: `GateWorkspaceInteractionRead` / `GateWorkspaceInteractionWrite` reuse `work_context.read` / `work_context.write` — no new Gateway paths
- Must never: plan, execute, automate, grant authority, duplicate Decision Queue or Recommendation Engine logic, or bypass Permission Gateway

## Position

```
Human Intent
  → Interaction (opportunity surface)
  → Intent request (handoff)
  → Command Pipeline
  → Permission Gateway
  → Execution
```

Understanding remains separate from authority. The Workspace helps the user act. It does not act for them.
