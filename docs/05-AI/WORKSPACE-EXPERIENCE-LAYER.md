# Workspace Experience Layer

Phase 6 Batch 2 / Sprint 95.

## Principle

The Experience Layer answers: **How should existing Workspace understanding be presented?**

It is a presentation projection over `WorkspaceSessionState`. It is not a cognition model, planner, executor, or second session.

## Domain

- `WorkspaceExperienceState` — calm Work-surface presentation snapshot
- `ExperienceSection` / `ExperienceItem` — groupings with visibility
- `ExperienceVisibility` — Immediate · Highlighted · Collapsed · Deferred
- `ExperienceSummary` — focus / matters / blocked / ready / next lines

Every item references a Session field (`source_session_field` + `source_ref` + `why`).

## Ownership

Experience **owns nothing**. Session remains the canonical runtime model. Experience feeds UI and Assistant explanation only.

Independent remain: Decision Queue, Recommendation Engine, Readiness, Adaptation, Continuity, Operating State, Pattern, Intelligence, Session.

## Smart presentation

Deterministic rules only — not AI reasoning:

| Visibility | Meaning |
|------------|---------|
| Immediate | Default Work surface |
| Highlighted | Emphasize (e.g. blocked, high-priority attention, top next step) |
| Collapsed | Available but out of the way when empty or secondary |
| Deferred | Timeline / progress kept off the primary plane |

"High-priority attention" means Attention's own `priority` band. Experience presents that
banding; it never re-derives one from a raw `score` threshold.

### Rendering rationale

Structured reasons reach Experience intact: `AttentionItem.reasons`, and downstream
`DecisionReason.attention_reason` / `RecommendationItem.attention_reasons` (Sprint 129).
Where one is present, render its `explanation_key` — that key is the translation contract.
The sibling `summary` strings are factual fallbacks for surfaces without a string table,
not the intended display text.

Wording, translation, and formatting live here. Domain models carry only `source`,
`signal`, `weight`, and `explanation_key`; Experience explains **how it is shown**, never
what deserves focus or what to consider next.

## Experience groupings (presentation only)

- Primary Focus
- Today's Work
- Session decisions (Decision Queue pointers from Session — not Attention Engine)
- Waiting On
- Blocked Work
- Recent Progress
- Recommended Next Step
- Helpful Improvements
- Session Health

## Governance

- Zero authority (`authority_effect: none`)
- Audits: `workspace.experience.generated`, `.projected`, `.updated`
- `attempt_execute` always fails
- Must never: launch, prepare, restore, move windows, create tasks, execute intents, approve, automate, or bypass Pipeline / Gateway

## Position

```
Human Intent
  → Workspace Understanding (Intelligence + cognition)
  → Session (canonical runtime orchestration)
  → Experience (presentation for Work / Assistant)
  → Work Context (semantic kind-of-work)
  → Planning (human-initiated)
  → Permission Gateway
  → Execution
```

## Surfaces

| Surface | Role |
|---------|------|
| Work | Default calm product view over Experience |
| Assistant | Explains the same Experience — never edits |
| Operator | Generate / Explain / Validate / Compare diagnostics |
