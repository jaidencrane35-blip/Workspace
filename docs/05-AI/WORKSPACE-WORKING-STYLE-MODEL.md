# Workspace Working Style Model

Phase 6 Batch 6 / Sprint 99.

## Principle

The Working Style Model answers: **How does work usually happen in this Workspace?**

It is a read-only projection of observable operating patterns. It is not profiling, surveillance, prediction, autonomous learning, or behavioural control.

## Domain

- `WorkspaceWorkingStyleState` — full working-style snapshot
- `WorkingStyleObservation` — one explainable observation
- Kinds — Rhythm · Organization · Workflow · Interaction Preference · Observed Usage · Context Switching
- Origins — **Observed Behaviour** vs **Explicit Preference** (never mixed)
- Confidence — High · Medium · Low (display-only consistency hint)

Every observation includes evidence source, confidence, explanation, affected context, and why. No hidden reasoning.

## Ownership

Working Style **owns nothing**. Activity Graph, Pattern Model, Purpose, Task Graph, and Preferences remain sources of truth. Working Style only aggregates existing evidence.

## Intelligence

`WorkspaceIntelligenceState.working_style` embeds after Milestones.

Attention, Recommendation Engine, Adaptation, Session, Navigation, and Experience may **reference** Working Style as evidence only. Working Style never controls behaviour.

## Governance

- Zero authority (`authority_effect: none`)
- Audits: `workspace.working_style.generated`, `.updated`, `.validated`
- `attempt_execute` always fails
- Reuses existing `work_context.read` — no new Gateway paths
- Must never: profile, score, surveil, predict, auto-adapt, persist a style store, or authorize

## Position

```
Human Intent
  → Workspace Understanding
  → Workspace Navigation
  → Workspace Milestones
  → Workspace Working Style
  → Planning
  → Decision
  → Permission Gateway
  → Execution
```
