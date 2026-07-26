# Workspace Work Context Engine

Phase 6 Batch 3 / Sprint 96.

## Principle

The Work Context Engine answers: **What kind of work is this?**

It is a semantic projection over Session, Experience, and Intelligence pipeline outputs. It is not a Project, Task, Session, planner, or executor.

## Domain

- `WorkspaceWorkContextState` — full semantic snapshot
- `WorkContext` — name, type, evidence, confidence, associations, status, focus, blockers, progress
- `WorkContextType` — Development · Research · Administration · Communication · Creative · Learning · Operations · Planning · Custom
- `WorkContextRelationship` — Primary · Supporting · Nested · Recently Active · Interrupted · Dormant · Candidate

Every field explains **why**. No hidden reasoning. No chain-of-thought.

## Ownership

Work Context **owns nothing**. Classifications are deterministic keyword evidence over existing projections.

Independent remain: Decision Queue, Recommendation Engine, Readiness, Adaptation, Continuity, Operating State, Pattern, Intelligence, Session, Experience.

## Inputs (consume only — never regenerate)

Session · Experience · Purpose · Composition · Environment · Task Graph · Continuity · Evolution · Activity · Decision Queue · Patterns · Recommendations · Readiness

## Intelligence

`WorkspaceIntelligenceState.work_context` embeds after Session → Experience projection, before future restoration systems.

Attention / Recommendation Engine / Adaptation may **reference** Context as evidence only. Nothing gains authority.

## Governance

- Zero authority (`authority_effect: none`)
- Audits: `workspace.work_context.generated`, `.updated`, `.validated`
- `attempt_execute` always fails
- Reuses existing `work_context.read` capability — no new Gateway paths
- Must never: execute, plan, launch, restore, persist, predict, or bypass Pipeline / Gateway

## Position

```
Human Intent
  → Workspace Understanding (Intelligence + cognition + Work Context)
  → Planning (human-initiated)
  → Decision
  → Permission Gateway
  → Execution
```

Work Context belongs exclusively in the Workspace Understanding layer.
