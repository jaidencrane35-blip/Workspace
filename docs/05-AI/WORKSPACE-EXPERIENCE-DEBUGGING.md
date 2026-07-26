# Workspace Experience Debugging

Sprint 135 — developer observability for the Experience translation boundary.

This is **not** user-facing telemetry. Traces are architecture debugging tools for
kernel tests, Operator diagnostics, and catalog maintenance.

## Principle

When a displayed explanation looks wrong, ask:

> What cognition produced this displayed explanation?

Experience answers with an optional **translation trace** — without mutating Domain
reasoning and without changing Attention/Decision scoring.

```
AttentionReason / DecisionReason
        ↓ resolve_*_traced (optional)
ExperienceTranslationTrace
  - source_reasoning_type
  - source_identifier
  - explanation_key
  - resolver_path { kind, match_key }
  - display: DisplayReason
  - rendering_surface?
        ↓ (normal UI ignores traces)
DisplayReasonList / DecisionReasonList
```

## Resolver paths

Catalog-owned order (same as Sprint 132):

| Kind | Meaning | Example `match_key` |
|------|---------|---------------------|
| `exact` | Exact catalog key | `exact:decision.base.outstanding` |
| `prefix_suffix` | Prefix + known suffix | `prefix_suffix:task.base.blocked` |
| `prefix_pattern` | Prefix + pattern (`starts_with`) | `prefix_pattern:purpose.obstacle.composition:*` |
| `prefix_fallback` | Prefix rule fallback | `prefix_fallback:purpose.obstacle.*` |
| `unknown` | No catalog match | `unknown:future.debug.unknown` |
| `decision_native` | Decision without Attention reason | `decision_native:decision.goal_alignment` |

Developer label format:

```
exact:
exact:decision.base.outstanding
```

```
prefix_pattern:
prefix_pattern:purpose.obstacle.composition:*
```

## How to use traces

### Rust (kernel)

```rust
use crate::services::explanation_resolver::resolve_attention_reason_traced;

let trace = resolve_attention_reason_traced(&reason, Some("test"));
assert_eq!(trace.display, resolve_attention_reason(&reason));
println!("{}", trace.resolver_path.label());
```

Decision:

```rust
resolve_decision_reason_traced(&decision_reason, Some("operator"));
```

`rendering_surface` is an optional caller tag (`test`, `operator`, …). Normal Work/Assistant
paths must **not** call traced APIs for rendering.

### TypeScript (developer / Operator tooling only)

```ts
import {
  resolveAttentionReasonTraced,
  formatResolverPathLabel,
} from "../lib/experienceTranslation";

const trace = resolveAttentionReasonTraced(reason, "operator");
console.debug(formatResolverPathLabel(trace.resolver_path), trace);
```

**Do not** pass traces into Work or Assistant components. Use `DisplayReasonList` /
`DecisionReasonList` for user-facing rationale.

## Debugging playbooks

### Translation failures / unexpected wording

1. Capture the `explanation_key` from the source `AttentionReason`.
2. Call `resolve_*_traced` and inspect `resolver_path.kind` + `match_key`.
3. Compare `trace.display` with the UI `DisplayReason`.
4. If `known: false`, the catalog has no match — add an exact key, suffix, or pattern.

### Missing explanation keys

Symptoms: `resolver_path.kind == unknown`, description contains the unresolved key.

1. Confirm Domain emits the expected `explanation_key` (cognition owns the key).
2. Add the key to `packages/kernel/resources/explanation-catalog.json`.
3. Run `node scripts/sync-explanation-catalog.mjs`.
4. Verify with `pnpm verify:explanation-catalog`.

### Catalog mismatch (stale generated TS)

Symptoms: Rust and TypeScript resolve differently, or verify script fails.

```bash
pnpm verify:explanation-catalog
# if stale:
node scripts/sync-explanation-catalog.mjs
pnpm verify:explanation-catalog
```

Rust also guards sync via `generated_ts_catalog_matches_json_source`.

### UI boundary violations

Symptoms: panels import `explanationResolver` or Domain `AttentionReason` /
`DecisionReason` for rendering.

```bash
pnpm verify:ui-experience-boundary
```

Fix: route through `DisplayReasonList` / `DecisionReasonList`. Only
`DisplayReasonList.tsx` and `experienceTranslation.ts` may touch resolver internals.

### Resolver fallback behaviour

| Observation | Likely cause |
|-------------|--------------|
| Prefix pattern hit for dynamic key | Expected for `composition:*` style keys |
| Prefix fallback for unlisted suffix | Expected — generic wording for that prefix |
| Unknown for brand-new key | Catalog gap — add entry, do not invent UI copy |
| `decision_native` | Decision Engine reason without Attention backing |

Fallback wording still surfaces the unresolved key in the unknown description template —
never hide identity.

## Architecture constraints

- Traces **do not** mutate Domain objects.
- Traces **do not** change scoring or ranking.
- Traces **are not** rendered on Work / Assistant.
- Operator may use traces for diagnostics (same class as scores / unresolved keys).
- Catalog remains presentation metadata only.

## Related docs

- [WORKSPACE-EXPERIENCE-CONTRACT.md](./WORKSPACE-EXPERIENCE-CONTRACT.md)
- [WORKSPACE-SEMANTIC-INTEGRITY.md](./WORKSPACE-SEMANTIC-INTEGRITY.md)
- [WORKSPACE-EXPERIENCE-LAYER.md](./WORKSPACE-EXPERIENCE-LAYER.md)
- [WORKSPACE-COGNITION-PIPELINE-CONTRACT.md](./WORKSPACE-COGNITION-PIPELINE-CONTRACT.md)
- [WORKSPACE-AUTOMATION-READINESS.md](./WORKSPACE-AUTOMATION-READINESS.md)
