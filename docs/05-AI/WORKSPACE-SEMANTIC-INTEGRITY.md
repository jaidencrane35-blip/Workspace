# Workspace Semantic Integrity

Sprint 134 — meaning flows one way: Domain → Experience → UI.

## Principle

**Meaning has a single source of truth.** Each layer may add presentation, never
re-invent inference.

```
Domain (what is true, why it was inferred)
    ↓ structured reasoning + facts
Experience (how it is said)
    ↓ DisplayReason / presentation models
UI (what the user sees)
```

Semantic integrity fails when UI or Experience invent rationale, reinterpret scores,
or bypass structured reasoning with ad-hoc copy.

## Layer responsibilities

### Domain — source of truth for meaning

| Responsibility | Examples |
|----------------|----------|
| Facts | Tasks, windows, queue items, session snapshots |
| Signals & weights | `AttentionSignal`, reason `weight`, Attention `score` |
| Structured reasoning | `AttentionReason`, `DecisionReason`, `explanation_key` |
| Engine projections | `summary`, engine-composed `reason` strings for models without structured reasons yet |

Domain never owns curated Experience catalog wording.

### Experience — translates meaning

| May | Must not |
|-----|----------|
| Resolve `explanation_key` via catalog | Score, rank, or infer |
| Format `DisplayReason` copy | Mutate source reasoning |
| Select presentation bands (`DisplayImportance`) | Replace Domain facts |
| Delegate Decision → Attention when `attention_reason` is set | Merge cognition models |

See [WORKSPACE-EXPERIENCE-CONTRACT.md](./WORKSPACE-EXPERIENCE-CONTRACT.md) for import
boundary and forbidden UI patterns.

### UI — renders Experience outputs

| May | Must not |
|-----|----------|
| Render `DisplayReason.title` / `.description` | Compose item-level rationale strings |
| Show engine `summary` metadata | Display Attention `score` as “why” |
| Show confidence / status / impact metadata | Import resolver or catalog internals |
| Operator diagnostics | Render `AttentionReason` fields as copy (Work / Assistant) |

## Forbidden patterns

| Pattern | Violation |
|---------|-----------|
| UI-created rationale | Meaning invented in components |
| Score interpretation | `score`, `score.total`, `score_factors` shown as explanation |
| Meaning invention | Headlines/reason strings duplicating structured reasons |
| Resolver bypass | Import `explanationResolver.ts` or catalog from UI panels |
| Raw `explanation_key` as primary copy | Skips Experience catalog |

## Reasoning model relationships (Sprint 134 audit)

Three structured reasoning shapes exist. **No merge** — each serves a distinct
cognition boundary. Experience adapts between them; it does not unify the types.

| Model | Role | Experience path |
|-------|------|-----------------|
| `AttentionReason` | Atomic “why attention” unit with `explanation_key` | Catalog lookup → `DisplayReason` |
| `DecisionReason` | Decision synthesis unit; may embed `attention_reason` | Delegate to Attention path, or summary/kind path |
| Recommendation reasoning | **No `RecommendationReason` type** | `RecommendationItem.attention_reasons: AttentionReason[]` when sourced from Attention; engine `reason` string only when no structured reasons |

### Decision

**Remain separate.** Shared interface would blur Attention (scored focus) vs Decision
(synthesis) vs Recommendation (informational next-step).

**Adapters (existing, sufficient):**

- `DecisionReason.attention_reason: Option<AttentionReason>` — Decision delegates
  display to Attention translation when present.
- `RecommendationItem.attention_reasons: Vec<AttentionReason>` — Recommendation
  Engine carries Attention reasons verbatim; Experience uses the same resolver.

Future engines without structured reasons may add catalog keys or Experience formatters
— not a shared reasoning supertype in Domain.

## UI import boundary

Normal UI surfaces (`AssistantPanel`, `WorkspaceIntelligencePanel`, …):

- **Must** use `DisplayReasonList` / `DecisionReasonList` for structured rationale.
- **Must not** import `explanationResolver.ts`, generated catalog, or Domain reasoning
  types for rendering.

Allowed internals (not imported by panels):

| File | Role |
|------|------|
| `experienceTranslation.ts` | Public UI boundary export |
| `explanationResolver.ts` | Resolver implementation |
| `DisplayReasonList.tsx` | Boundary component |

Diagnostic exception: `OperatorConsole.tsx` — scores, headlines, unresolved keys
allowed; still uses `DisplayReasonList` for structured reasons.

Automated check: `pnpm verify:ui-experience-boundary`

## Sprint 134 coverage audit summary

| Usage | Surface | Class |
|-------|---------|-------|
| `DisplayReasonList` / `DecisionReasonList` | Work, Assistant | **B — Experience translated** |
| Engine `summary` lines | All | **A — Domain projection metadata** |
| Operator scores / headlines | Operator | **C — Diagnostic** |
| Readiness / Adaptation `item.reason` | Work | **A/B — No AttentionReason yet; engine copy** |
| Recommendation `item.reason` + `attention_reasons` | Work | **D — Fixed** (reason hidden when reasons present) |
| `score_factors` in UI | — | Absent |

## Related docs

- [WORKSPACE-EXPERIENCE-CONTRACT.md](./WORKSPACE-EXPERIENCE-CONTRACT.md)
- [WORKSPACE-EXPERIENCE-LAYER.md](./WORKSPACE-EXPERIENCE-LAYER.md)
- [WORKSPACE-ATTENTION-ENGINE.md](./WORKSPACE-ATTENTION-ENGINE.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
