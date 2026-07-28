# Contextual Workspace Understanding Architecture (Programme III — Batch 6)

| Field | Value |
|-------|-------|
| **Purpose** | Turn state + policy + history + temporal + explanation evidence into richer situational understanding — without prediction, simulation, autonomous correction, or decision ownership |
| **Owner** | `WorkspaceContextualUnderstandingService` (DurableStore — **understanding evidence only**) |
| **Status** | Active — Programme III Batch 6 accepted (architecture audit grade A) |
| **Lifecycle owner** | No |
| **Execution / replay authority** | No |
| **Simulation / forecast / correction authority** | No |
| **Decision / policy / permission authority** | No |
| **Source of truth** | No |

## Core principle

**Contextual understanding organises situational meaning from durable evidence. It does not decide, predict, simulate, or change reality.**

```
Unified Workspace State
Policy Governance
Historical Reconstruction
Temporal Intelligence
Workspace Explanation Layer
        │
        ▼
Contextual Workspace Understanding
        │
        ▼
Situational understanding surfaces (read-only)
        │
        ▼
Human or Gateway-authorised decision (elsewhere)
```

Not:

```
Contextual Understanding
        │
        ▼
auto-decide / plan / mutate / forecast / simulate
```

## Architectural position

Programme III stack after Batch 6:

```
Unified Workspace State Model
        ↓
Policy & Governance Engine
        ↓
Historical Workspace Reconstruction
        ↓
Temporal Intelligence
        ↓
Workspace Explanation Layer
        ↓
Contextual Workspace Understanding
        ↓
Situational understanding artefacts (read-only)
```

Batch 6 **extends** Batch 5 explanation synthesis into richer situational framing.
It does not replace explanation, reconstruction, policy, or state authorities.

It answers:

> Given the durable evidence available now (and what is missing / conflicting),
> what is the situational picture of this workspace — and how certain is that picture
> as understanding coverage, not as truth?

It does **not** answer:

- what will happen next (prediction / forecasting)
- what would happen if (simulation)
- what the system should do now (decision ownership / autonomous correction)
- what must be true when evidence is missing (invented certainty)

## Why this batch

Batch 5 acceptance opened the Batch 6 gate with recommended direction:

> Contextual Workspace Understanding — turning state + policy + history + temporal +
> explanation evidence into richer situational understanding.

Constraints remain absolute:

- no prediction
- no simulation
- no autonomous correction
- no decision ownership

Collaboration / multi-actor understanding and simulation remain **later** candidates.

## Locked constraints (non-negotiable)

Do **not** introduce:

| Forbidden | Why |
|-----------|-----|
| Event sourcing | Competing authoritative log — still rejected |
| Authoritative replay | Replay must never mutate or re-decide |
| Simulation / forecasting | Futures and counterfactuals are not evidence |
| Autonomous correction | Understanding ≠ mutation authority |
| Decision ownership | Gateway / humans remain decision authorities |
| Planner / task creation | Understanding is not a planning engine |
| Policy re-decision | Policy evaluations may be referenced, never re-authorised |
| Causal invention | Unsupported “because” remains forbidden |
| Conflict resolution | Explain / frame disagreement; never declare a winner |
| Foreign silent `generate` | No gap-filling by regenerating upstream authorities |
| Opaque summaries | Every understanding claim needs evidence lineage |
| Certainty engine | Understanding confidence ≠ truth confidence |

## Ownership

### Contextual Understanding owns

- situational understanding packages / views
- context frames (what situation is being understood)
- theme / focus organisation over explanation + upstream evidence
- understanding completeness and uncertainty roll-ups
- provenance aggregation (references only — revision-bound)
- situational narratives under evidence language rules
- understanding history (append-only evidence)

### Contextual Understanding does **not** own

- lifecycle state / transitions
- execution / replay / restoration
- policy authority / Gateway decisions
- source mutation / envelope generation
- explanation / reconstruction / temporal analysis authority (Batches 3–5 remain owners)
- simulation, forecasting, or autonomous correction
- collaborative multi-actor decision authority
- planning / task creation / recommendation acceptance

## Relationship to prior batches

| Prior owner | Contextual Understanding may |
|-------------|------------------------------|
| Unified Workspace State | Reference via `load_snapshot` only |
| Policy Governance | Reference via load / explain only — never re-evaluate to invent Compliant |
| Historical Reconstruction | Reference via load only |
| Temporal Intelligence | Reference via load only |
| Explanation Layer | Reference via load / situation explain only — never regenerate to invent completeness |

| Prior owner | Contextual Understanding must not |
|-------------|-----------------------------------|
| Any upstream | Call `::generate` to fill gaps |
| Any lifecycle / planning service | Mutate / transition / create tasks |
| Policy / Gateway | Grant, deny, bypass, or decide |

### Carried-forward P2 disciplines (Batches 4–5)

1. **Explanation / understanding confidence remains diagnostic** — never truth certainty or auto-decision input.
2. **Cross-surface composition must keep explicit provenance:**  
   statement → evidence references → source revision → origin domain.
3. **Persisted understanding artefacts are revision-bound caches** — never “previous understanding = current truth”.
4. **Conflicts are framed, not resolved.** Observed sequence ≠ cause.

## Core model (shipped)

### `ContextualWorkspaceSnapshot`

Primary composed artefact for a situational understanding request.

| Field | Role |
|-------|------|
| `understanding_id` | Identity of this understanding artefact |
| `workspace_id` | Workspace scope |
| `generated_at` | Wall-clock composition time |
| `status` / `superseded_at` | Active vs superseded lifecycle of the artefact |
| `frame` | `ContextFrame` describing the situation being understood |
| `source_revisions` | Revision-bound upstream refs used for this compose |
| `situation_summary` | Evidence-backed situational framing |
| `themes` | Ordered `SituationalTheme` sections |
| `completeness` | `ContextualCompleteness` |
| `gaps` | Explicit `ContextualGap` unknowns / missing upstream evidence |
| `confidence` | Diagnostic coverage signal — not truth score |
| `provenance_links` | Links to upstream durable artefacts (revision-bound) |
| `narrative` | Evidence-backed situational narrative |
| `limitations` | Explicit limitation statements |
| `authority_effect` | Always `"none"` |
| `actionable` | Always `false` |
| `terminal` | Terminal when superseded into history |

Not a decision package. Not a plan. Not a replacement workspace state.

### `ContextFrame`

What situation is being understood.

| Field | Role |
|-------|------|
| `focus` | Optional focus theme (e.g. readiness, conflict-pressure, continuity) |
| `include_state` / `include_policy` / `include_reconstruction` / `include_temporal` / `include_explanation` | Upstream surface selection |
| `max_themes` | Hard bound — never unbounded scrape |
| `as_of_refs` | Optional explicit upstream revision / artefact refs (when present) |

Frames constrain reading. They do not invent coverage.

### `SituationalTheme`

One situational theme synthesised from evidence.

| Field | Role |
|-------|------|
| `theme_id` | Identity |
| `kind` | e.g. `current_state`, `governance_posture`, `historical_continuity`, `temporal_pressure`, `explanation_synthesis` |
| `title` | Human-readable theme title |
| `body` | Evidence-backed framing |
| `insights` | Ordered `ContextualInsight` observations under this theme |
| `evidence_refs` | Upstream durable refs with origin domain |
| `completeness` | Theme-local completeness |
| `authority_effect` | `"none"` |
| `actionable` | `false` |

### `ContextualInsight`

Evidence-backed observation nested under a theme — descriptive only; never “do X / execute Y / approve Z”.

| Field | Role |
|-------|------|
| `insight_id` | Identity |
| `kind` | Observation category |
| `body` | Evidence-backed claim |
| `evidence_refs` | Upstream durable refs |
| `explanation_lineage` | Optional lineage into explanation artefacts |
| `confidence` | Diagnostic coverage signal (0–100) — not truth |
| `uncertainty` / `limitations` | Explicit unknowns / bounds |
| `authority_effect` | `"none"` |
| `actionable` | `false` |

### `ContextualGap`

Explicit missing / unavailable / conflicting coverage — never filled by invention.

| Field | Role |
|-------|------|
| `gap_id` | Identity |
| `surface` | Which upstream surface is incomplete |
| `description` | Why coverage is missing / partial / conflicting |
| `severity` | Gap pressure signal |
| `evidence_refs` | Upstream durable refs (when any) |
| `authority_effect` | `"none"` |
| `actionable` | `false` |

### `ContextualUnderstandingConfidence`

Diagnostic coverage about the understanding package — **never truth confidence**.

| Field | Role |
|-------|------|
| `coverage` | How many requested surfaces were available |
| `available_surfaces` / `requested_surfaces` | Coverage denominator |
| `gap_count` / `conflict_count` | Uncertainty pressure |
| `uncertainty` / `limitations` | Explicit “not a truth / decision score” statements |
| `authority_effect` | `"none"` |

Maintain:

```
Confidence of understanding
        ≠
Confidence of truth
```

### `ContextualCompleteness`

Explicit uncertainty model — **never collapse**:

| State | Meaning |
|-------|---------|
| `Complete` | All requested surfaces were available with usable evidence |
| `Partial` | Some requested surfaces available; gaps recorded |
| `Unknown` | Completeness cannot be determined |
| `Contradictory` | Upstream surfaces disagree; conflicts preserved |
| `Unavailable` | Required upstream evidence could not be loaded |

**A complete-looking answer is worse than an explicit unknown.**

## Understanding principle

### Allowed

```
State snapshot (load_snapshot)
+ Policy evaluation (load_snapshot)
+ Reconstruction (load_snapshot)
+ Temporal analysis (load_snapshot)
+ Explanation package (load_snapshot)
= situational understanding artefact with themes/insights/gaps/provenance preserved
```

Same durable upstream revisions + same frame ⇒ deterministic understanding content
(`themes`, `insights`, `gaps`, `completeness`). Identity / `generated_at` may differ.

### Forbidden

```
Missing upstream
+ preferred story / forecast / “likely intent”
= invented situational certainty
```

Language / lineage rules:

| Allowed | Forbidden without evidence |
|---------|----------------------------|
| “Current envelope reports freshness=stale.” | “The workspace is unhealthy because …” (unsupported) |
| “Policy aggregate is Unknown.” | “Policy would approve.” |
| “Revision A was followed by Revision B.” | “Revision A caused Revision B.” |
| “Sources disagree; conflict preserved.” | “Source A was wrong.” |
| “Understanding coverage is partial (diagnostic).” | “High confidence ⇒ safe to act.” |

Every theme / summary claim must carry:

```
Understanding statement
        ↓
Evidence references
        ↓
Source revision / artefact id
        ↓
Origin domain
```

No opaque generated summaries without evidence lineage.

## Evidence inputs (read-only)

Contextual Understanding **reads** durable upstream artefacts; it never generates/refreshes foreign authorities as a side effect of understanding.

| Input | Access pattern |
|-------|----------------|
| Workspace state envelope snapshot | `load_snapshot` only — never `generate` |
| Policy governance snapshot | `load_snapshot` / explain only — never `generate` |
| Historical reconstruction snapshot | `load_snapshot` only — never `generate` |
| Temporal intelligence snapshot | `load_snapshot` only — never `generate` |
| Workspace explanation snapshot | `load_snapshot` / situation explain only — never `generate` |

**No hidden source scraping. No lifecycle inspection shortcuts. No silent regeneration.**

## Service contract (shipped)

### `WorkspaceContextualUnderstandingService`

Responsibilities:

- accept a `ContextFrame`
- load requested upstream snapshots via `load_snapshot` only (state / policy / reconstruction / temporal / explanation)
- assemble themes, insights, gaps, limitations
- attach provenance lineage to every theme / claim
- compose evidence-backed situational narrative
- persist understanding artefacts (read-model)

Must **not** (forbidden behaviours):

- execute / replay / dispatch / restore
- simulate / forecast / auto-correct
- mutate sources or lifecycles
- create tasks / plans / approvals
- call foreign `generate` paths to fill gaps
- grant permissions or call `PermissionGateway`
- invent causal “because” without evidence refs
- resolve contradictions by dropping a side
- convert understanding packages into commands
- treat prior understanding cache as current truth without re-binding to revisions

Negative guards (tests): `attempt_execute`, `attempt_approve`, `attempt_mutate_lifecycle`,
`attempt_create_task`, `attempt_mutate_intent`, `attempt_mutate_task_graph`,
`attempt_alter_workspace_state_envelope`, `attempt_convert_insight_to_recommendation`,
`attempt_invent_causal_explanations`, `attempt_silent_refresh`, `attempt_emit_command`.

## Commands (shipped)

| Command | Kind | Capability | Purpose |
|---------|------|------------|---------|
| `GenerateContextualWorkspaceUnderstanding` | Mutation | `work_context.write` | Persist an understanding artefact for a frame |
| `GetContextualWorkspaceUnderstanding` | Query | `work_context.read` | Load current understanding snapshot |
| `GetContextualWorkspaceUnderstandingSummary` | Query | `work_context.read` | Summary + authoritative `history_count` |
| `ExplainWorkspaceContext` | Query | `work_context.read` | Situational understanding surface over current/last package |

All commands:

```
IPC → CommandPipeline → PermissionGateway → WorkspaceContextualUnderstandingService
```

No repair / apply / replay / predict / decide commands.

## Persistence (shipped)

| Artefact | Role |
|----------|------|
| Migration `057_workspace_contextual_understanding.sql` | Read-model tables only |
| Repository `workspace_contextual_understanding.rs` | Persist / retrieve / supersede / history append |

Repository does **not** evaluate, repair, replay, simulate, forecast, decide, resolve conflicts, or mutate sources.

Persisted artefacts must remain:

- revision-bound
- reproducible
- evidence-linked

They must **not** become:

```
previous understanding = current truth
```

Repository remains an **understanding evidence cache**, not a truth or control database.

## Projection contract

Dual-channel pattern (shipped as `ContextualUnderstandingProjection`):

| Channel | Contents | Actionable? |
|---------|----------|-------------|
| `current` | Active `ContextualWorkspaceSnapshot` | View / inspect only |
| `history` | Superseded understandings via `ContextualUnderstandingHistoryEntry` (append-only) | Never |
| `history_count` | Full terminal count | Scalar authority |

History remains:

- evidence only
- immutable
- non-actionable
- no commands
- no lifecycle controls
- `authority_effect = "none"`

## Recovery requirements

| Scenario | Result |
|----------|--------|
| Missing upstream surface | Theme / section `Unavailable` + `ContextualGap` — not invented |
| Partial upstream coverage | `Partial` understanding — gaps explicit |
| Contradictory upstreams | `Contradictory` + conflict preserved |
| Stale source | Uncertainty preserved — not silently refreshed |
| Missing provenance | Unknown — never opaque certainty |
| Corrupt / unusable inputs | Understanding failure evidence — not auto-repaired |
| Restart | Same durable evidence + same frame ⇒ same understanding content |
| Fabrication attempt | Fail closed — never invent certainty, winners, or futures |

Recovery helpers (domain predicates):

- `recovery_must_not_fabricate_contextual_understanding`

## Governance (shipped)

Mutation baseline includes `GenerateContextualWorkspaceUnderstanding`.

Guards detect:

- contextual understanding importing lifecycle / execution / launch / planning services
- simulate / forecast / replay / repair / decide command paths
- history / projection DTO command / authority fields
- foreign `::generate` silent refresh from understanding compose path
- repository → service ownership leaks
- understanding DTO fields that imply execute / approve / dispatch / plan

Ownership registry entries:

- `contextual_understanding`
- `contextual_understanding_snapshot`

History / projection DTO inventories:

- `ContextualUnderstandingHistoryEntry`
- `ContextualUnderstandingSummary`
- `ContextualUnderstandingProjection`

## Required tests (acceptance)

### Domain

- deterministic understanding from identical frame + upstream refs
- missing upstream → Unavailable / gaps (no invented themes)
- conflicts preserved without resolution
- provenance lineage present on every theme
- sequence language only (no unsupported causality)
- understanding confidence never promoted to truth / decision authority
- history separation / non-actionability
- prior cache ≠ current truth without revision binding

### Kernel

- command routing + Gateway enforcement
- persistence rollback (no partial understanding write)
- no simulate / forecast / replay / lifecycle / task / command-emit paths
- restart continuity of durable understanding evidence
- frame bounds respected

### Projection

- DTO non-commandability
- history separation
- `history_count` authority
- understanding separation from commands

### Governance

- import restrictions
- architecture map refresh
- mutation baseline + DTO inventory updates

### Recovery

- missing evidence does not become Complete
- restart preserves evidence without fabricating understanding

## Documentation deliverables

- This architecture — status **Active** (Batch 6 implemented)
- `PROGRAMME-III-COHERENT-WORKSPACE-RUNTIME.md` — Batch 6 accepted / implemented
- `ARCHITECTURE-GOVERNANCE.md`, `PROJECTION-INTEGRITY.md`, `OPERATIONAL-RECOVERY.md`
- Vocabulary: Contextual Understanding ≠ decision authority / planner / simulator / certainty engine

## Batch 6 acceptance criteria

Accepted when:

1. Contextual Understanding exists as situational understanding evidence only
2. Batches 1–5 remain authoritative in their domains
3. No simulation, forecasting, event sourcing, replay, autonomous correction, or decision ownership introduced
4. No lifecycle replacement or mutation through understanding history
5. Missing / conflicting / stale upstream evidence stays explicit — never synthetic certainty
6. Provenance lineage remains attached to every understanding claim
7. Upstream read boundaries (`load_snapshot` only) remain absolute
8. Projection integrity and Gateway/Pipeline boundaries remain absolute
9. Governance detects contextual-understanding boundary violations
10. Recovery never fabricates Complete understandings from gaps
11. Persisted understanding remains revision-bound — never previous-cache-as-truth

## Review gate

**Implementation contract accepted and shipped.** Further work proceeds only against this contract —
no simulator, no forecaster, no SoT elevation, no autonomous correction, no decision ownership.

Batch 6 accepted (architecture audit grade A). Next: [Knowledge Synthesis](./KNOWLEDGE-SYNTHESIS-ARCHITECTURE.md)
(Batch 7 — Active / implemented; Batch 8 gated on Batch 7 audit ACCEPT).

## Related

- [Programme III — Coherent Workspace Runtime](./PROGRAMME-III-COHERENT-WORKSPACE-RUNTIME.md)
- [Knowledge Synthesis Architecture](./KNOWLEDGE-SYNTHESIS-ARCHITECTURE.md) (Batch 7)
- [Workspace Explanation Layer Architecture](./WORKSPACE-EXPLANATION-LAYER-ARCHITECTURE.md) (Batch 5)
- [Temporal Intelligence Architecture](./TEMPORAL-INTELLIGENCE-ARCHITECTURE.md) (Batch 4)
- [Historical Workspace Reconstruction Architecture](./HISTORICAL-WORKSPACE-RECONSTRUCTION-ARCHITECTURE.md) (Batch 3)
- [Policy & Governance Architecture](./POLICY-GOVERNANCE-ARCHITECTURE.md) (Batch 2)
- [Unified Workspace State Architecture](./UNIFIED-WORKSPACE-STATE-ARCHITECTURE.md) (Batch 1)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
- [Operational Recovery](../03-Engineering/OPERATIONAL-RECOVERY.md)
