# Projection Integrity & Immutable Evidence Consumption

| Field | Value |
|-------|-------|
| **Purpose** | Protect consumption of protected lifecycle evidence |
| **Owner** | Platform Kernel |
| **Status** | Active |

## Principle

Domain state remains canonical. Projections and React surfaces may **display**
lifecycle evidence but must not invent, mutate, or erase it.

| Layer | Responsibility |
|-------|----------------|
| Services / domain | Lifecycle authority and continuity writes |
| Repositories | Persistence guards for transitions and immutable identity |
| Summaries / IPC | Project canonical state; actionable surfaces exclude terminals without dropping history |
| React | Projection-only rendering; no lifecycle ownership |

## Unified projection contract

Every major lifecycle projection exposes three channels:

| Channel | Contents | Actionable? |
|---------|----------|-------------|
| Actionable / current | Active work only (`items`, `top_candidates`, `candidates`, `top_nodes`, execution `actionable`) | Yes |
| Historical evidence | `history` window | Never |
| Count authority | **`history_count`** | N/A (scalar) |

Required rules:

- **`items` / actionable collections mean actionable only.** Terminals never share that channel.
- **`history` means evidence only.** History DTOs are never convertible into command inputs
  (no execute / dismiss / select / mutate / transition handlers).
- **`history_count` is authoritative.** `history.length` is only the returned window
  (especially under `summary(limit)`). Empty actionable ≠ no evidence.
- **React is projection-only.** Action buttons may exist only on actionable projections.
- **Unknown provenance stays unknown.** Never default unknown → known (`native`, invent origin, etc.).

```
actionable current state
        |
        |
historical evidence projection
        |
        |
immutable authoritative artifacts
```

## Identity vocabulary

| Kind | Meaning | Examples |
|------|---------|----------|
| **Source identity** | Owning subsystem key for a live decision | DQ `source_type` + `source_id`; DE `candidate_key` |
| **Overlay identity** | Lifecycle row for presentation continuity | DQ `DecisionOverlayHistoryEntry`; DE `DecisionArtifactHistoryEntry` |
| **Outcome identity** | Recommendation Engine terminal outcome evidence | `RecommendationHistoryEntry.outcome.outcome_id` |
| **Task identity** | Task Graph node / terminal history | `TaskHistoryEntry.task_id` |
| **Execution identity** | Execution request lifecycle | `ExecutionLifecycleHistoryEntry.execution_request_id` |

Do not conflate RE outcome identity with DQ/DE overlay/artifact identity.

## Surface contracts

### Recommendation Engine

- **Actionable:** `candidates` (full state) and `top_candidates` (summary) — open lifecycle only
  (`created` / `available` / `presented`).
- **History:** `history` + `history_count` — accepted / rejected / expired / superseded outcomes
  (and orphan overlays). Entries carry `terminal: true`, `actionable: false`.
- Frontend TypeScript contracts require `history` / `history_count` (Rust always emits them).
- Consumers must not invent counts via `?? 0` / `?? []`.

### Decision Queue

- **Actionable:** `items` — pending/open overlays only.
- **History:** dismissed / expired / orphan overlays via `DecisionOverlayHistoryEntry` only.
- Full queue and summaries share identical actionable filtering.
- History cannot trigger view / dismiss / execute / transition.
- No lifecycle deletion API (`delete_overlay` removed).

### Decision Engine

- **Actionable:** `candidates` / `top_candidates` — `open` / `postponed` only.
- **History:** selected / dismissed / expired via `DecisionArtifactHistoryEntry` only.
- History cannot select / dismiss / execute / mutate.
- Orphan overlay provenance uses `origin = unknown` — never invent `native` /
  `recommendation_intake`.
- No DE overlay deletion API — retention is transition-based.

### Task Graph

- **Actionable:** `nodes` / `top_nodes` — open statuses only (`proposed` … `blocked`).
- **History:** completed / cancelled via `TaskHistoryEntry` with progress + explanation retained.
- Aggregate counts (`completed_count`, etc.) remain truthful scalars; they do not put
  terminals back into actionable lists.
- Historical task evidence is never editable state.

### Execution Lifecycle

- **Actionable:** `ExecutionLifecycleProjection.actionable` — `in_progress` only.
- **History:** completed / failed / cancelled via `ExecutionLifecycleHistoryEntry`.
- Failed history preserves `retry_allowed`, `failure_reason`, and state classification.
- `Unknown` remains unknown — never invented into actionable or terminal channels.
- Durable lifecycle outcomes cannot disappear because an actionable list is empty.
- `get_execution_states` returns the dual-channel projection (not a flat mixed list).

#### Outcome-only execution history (fallback)

When **no durable lifecycle row** exists for an execution id, projections may fall back to
audit-derived `ExecutionOutcome` evidence via `ExecutionLifecycleHistoryEntry::from_outcome`.

Rules:

- Fallback exists **only** when no durable lifecycle row is present.
- **Reduced provenance is intentional** (e.g. empty `suggestion_id`, claimed_at derived from
  outcome timestamp) — never invent a full lifecycle claim that did not exist.
- **Unknown remains unknown** — empty/unknown outcome streams do not become fabricated
  terminals.
- Fallback is **evidence projection only** — it does not replace
  `ExecutionLifecycleService` ownership of claim / complete / fail / cancel.
- Consumers must not treat outcome-only history as a second lifecycle authority.

### Recommendation Engine sealing

- **Consumer / IPC APIs** (`generate`, `generate_with_inputs`, enrich paths) always return
  **sealed** projections: `candidates` actionable-only; terminals only in `history`.
- **Internal mutation views** (`generate_for_lifecycle_mutation` / unsealed assembly) may
  retain terminal candidates so accept → confirm flows can address overlays.
- Unsealed views must never be returned from IPC or React-facing summaries.
- `WorkspaceRecommendationEngineState::is_consumer_sealed` asserts the consumer invariant.

### Observation layer

- Observations are **derived evidence only** — disposable classifications of recorded activity.
- Observations are not lifecycle owners and cannot become command authorities.
- Projections must not imply observations caused state changes; durable audit / domain
  events remain the source of truth.
- Observation execute attempts fail by contract.

### Planning Engine (Programme II Batch 2)

- **Current:** `PlanningSnapshot.current` — at most one active `PlanningProposal`.
  View/inspect only; never execute / approve / dispatch from the projection.
- **History:** superseded / abandoned plans via `PlanningHistoryEntry` only
  (`terminal: true`, `actionable: false`, `authority_effect: none`).
- **`history_count`** is authoritative; summary windows may truncate `history`.
- Planning references Goal / Task / RE / DE / Memory / Attention / Purpose by identity —
  no payload duplication, no foreign lifecycle mutation.
- React may expand / collapse / compare plans and inspect rationale, risks, and
  assumptions. No planning mutation ownership in the UI.
- Restart reconstructs durable snapshots — no planning replay, no fabricated evidence.

### Reasoning Memory (Programme II Batch 3)

- **Current:** `ReasoningSnapshot.current` — at most one `current` reasoning record.
  Evidence only; never execute / approve / dispatch / convert to commands.
- **History:** superseded / archived via append-only `reasoning_history` →
  `ReasoningHistoryEntry` (`terminal: true`, `actionable: false`, `authority_effect: none`).
- **`history_count`** is authoritative; summary windows may truncate `history`.
- Records reference planning / intent / task / RE / DE / memory by identity only.
- Confidence and uncertainty evolution trails are retained on the durable record.
- React may display current reasoning, history, confidence, uncertainty, reflection,
  and lessons — no action buttons or mutation controls.
- Restart reconstructs durable current + history; missing reasoning remains missing
  (never fabricate).

## Serde defaults vs TypeScript required fields

Rust history fields often use `#[serde(default)]` so older persisted / in-flight JSON
without `history` still deserializes. **Producers always emit** `history` + `history_count`.
TypeScript marks these fields **required** so frontend consumers cannot invent absence via
`?? 0` / `?? []`. Do not remove Rust defaults solely for symmetry — they are backwards
compatibility for deserialize, not permission to omit on emit.

## Frontend responsibilities

- Prefer projected actionable lists (`top_candidates` / DQ `items` / Task `top_nodes` /
  execution `actionable`).
- Use `history_count` for evidence presence; never infer absence from empty actionable
  lists or truncated `history` arrays.
- History helpers only verify projected non-actionability — they do not invent
  lifecycle or provenance.
- History rendering has no mutation buttons, command handlers, or execution paths.
- History sections must read as **evidence** (muted labels, “not a command” for retry
  facts) — never styled as active work queues.

## Related

- [Architecture Governance](./ARCHITECTURE-GOVERNANCE.md)
- [Persistence Boundary Governance](./PERSISTENCE-BOUNDARY-GOVERNANCE.md)
- [Lifecycle Governance](./LIFECYCLE-GOVERNANCE.md)
- [Workspace Experience Contract](../05-AI/WORKSPACE-EXPERIENCE-CONTRACT.md)
