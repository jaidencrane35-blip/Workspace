# Workspace Experience Contract

Sprint 133 — unified translation boundary between cognition and UI.

## Principle

Experience is the **only** layer that turns structured workspace meaning into human-facing
display models. Cognition produces facts and reasoning; UI renders Experience output.

```
Domain (meaning)  →  Experience (translation)  →  UI (rendering)
```

Experience is not a cognition model. It does not score, rank, infer, or decide.

## Ownership

### Domain owns meaning

| Owns | Examples |
|------|----------|
| Facts | Task status, window focus, queue items, session fields |
| Signals | `AttentionSignal`, source types, lifecycle states |
| Structured reasoning | `AttentionReason`, `DecisionReason`, `explanation_key`, weights |
| Scoring | Attention `score`, `score_factors`; Decision `DecisionScore` |

Domain types carry **identity and inference**, not curated presentation copy (except
engine-level summaries explicitly marked as projection metadata).

### Experience owns translation

| Owns | Examples |
|------|----------|
| Presentation contracts | `explanation-catalog.json`, resolution order, unknown templates |
| Display models | `DisplayReason`, `DisplayImportance` |
| Translation algorithms | Catalog lookup, deterministic fallback hierarchy |
| Wording | Titles and descriptions resolved from catalog or safe templates |

Experience **reads** structured reasoning; it never replaces or mutates it.

### UI owns rendering

| Owns | Does not own |
|------|----------------|
| Layout, typography, visibility bands | Rationale wording |
| Binding to `DisplayReason` fields | Catalog lookup or prefix rules |
| Diagnostic toggles (Operator only) | Attention / Decision scoring interpretation |

UI keeps source `AttentionReason[]` / `DecisionReason[]` alongside display output for
identity and debugging — but **must not render structured reasoning as user-facing copy**.

## Translation boundary

Canonical implementations:

| Layer | Entry point |
|-------|-------------|
| Kernel | `packages/kernel/src/services/explanation_resolver.rs` |
| UI | `app/src/lib/experienceTranslation.ts` (re-exports resolver) |
| Catalog | `packages/kernel/resources/explanation-catalog.json` |

Resolution order (catalog-owned): exact → prefix suffix → prefix pattern → prefix
fallback → unknown.

Shared UI components:

- `DisplayReasonList` — Attention-backed reasoning
- `DecisionReasonList` — Decision reasoning (Attention path first)

## Forbidden patterns

These are **never** allowed on user-facing surfaces (Work, Assistant):

| Pattern | Why forbidden |
|---------|----------------|
| Raw `explanation_key` as primary copy | Bypasses Experience catalog |
| Rendering `AttentionReason` fields directly | UI reconstructs cognition |
| Rendering `DecisionReason.summary` / `.kind` without Experience | Bypasses translation boundary |
| `score_factors` as rationale | Arithmetic evidence, not meaning |
| Interpreting Attention `score` or Decision `score.total` as “why” | Score is not reasoning |
| UI-composed rationale strings | Duplicates or replaces cognition |
| Duplicating `DecisionExplanation.headline` when `DecisionReasonList` is shown | Double rationale |

### Allowed exceptions

| Surface | Allowed |
|---------|---------|
| Operator Console | Raw scores, unresolved keys (`showUnresolvedKey`), diagnostic headlines |
| Engine `summary` lines | High-level projection metadata (not item-level rationale) |
| Fact narratives | Source `AttentionItem.explanation` when **no** structured reasons exist |
| Structured metadata | Priority, urgency, confidence, outcome — not rationale |

## DisplayReason vs ExperienceItem

Two Experience-related types serve **different roles** — no migration required.

| Type | Role | Scope |
|------|------|-------|
| `DisplayReason` | Translates one structured reason into display copy | Attention / Decision rationale |
| `ExperienceItem` | Session presentation grouping pointer | Work surface sections |

`ExperienceItem` (`WorkspaceExperienceState`) projects Session fields into calm Work
groupings (`title`, `why`, `source_session_field`). It is not a general display envelope
for cognition reasoning.

`DisplayReason` remains sufficient because:

1. Structured reasoning already arrives as typed `AttentionReason` / `DecisionReason`.
2. Translation output is a stable, testable display model with identity echo fields.
3. Session presentation (`ExperienceItem`) is a separate projection concern.
4. Forcing a unified `ExperienceItem { type, source, meaning_ref, display_payload }`
   would merge two boundaries without removing duplication — catalog + resolver already
   centralize translation.

Future surfaces should add new **translation functions** and catalog entries, not new
cognition fields in Domain.

## Contract guarantees (tested)

1. UI surfaces consume Experience outputs (`DisplayReasonList`, `DecisionReasonList`).
2. Domain reasoning remains structured and unchanged after translation.
3. Display output is deterministic for identical inputs.
4. Unknown meaning degrades safely (`known: false`, key visible in description template).

Verification:

- `cargo test` — kernel contract tests (`workspace_experience_contract_tests`)
- `pnpm test` — catalog + Experience contract vitest
- `pnpm verify:explanation-catalog` — generated artifact sync

## Audit reference (Sprint 133)

| Finding | Surface | Class | Action |
|---------|---------|-------|--------|
| `DisplayReasonList` / `DecisionReasonList` on Attention, Decision, Recommendations | Work, Assistant, Operator | Valid | Keep |
| Engine `summary` projection lines | All | Valid diagnostic | Keep |
| Operator score / headline / `showUnresolvedKey` | Operator | Valid diagnostic | Keep |
| Attention `score` beside reasons | Work Intelligence | Invalid leakage | Removed |
| Decision candidate `score.total` + `headline` beside reasons | Work Intelligence | Invalid leakage | Removed |
| `rec.explanation` when `reasons` present | Work Intelligence | Invalid leakage | Removed (reasons only) |
| `score_factors` in UI | — | — | Not present (type-only) |

Automated check: `pnpm verify:ui-experience-boundary`

## Sprint 134 — semantic integrity

See [WORKSPACE-SEMANTIC-INTEGRITY.md](./WORKSPACE-SEMANTIC-INTEGRITY.md) for the
full meaning-flow contract, reasoning-model relationship audit, and coverage table.

## Related docs

- [WORKSPACE-EXPERIENCE-LAYER.md](./WORKSPACE-EXPERIENCE-LAYER.md)
- [WORKSPACE-ATTENTION-ENGINE.md](./WORKSPACE-ATTENTION-ENGINE.md)
- [WORKSPACE-INTELLIGENCE-FOUNDATION.md](./WORKSPACE-INTELLIGENCE-FOUNDATION.md)
