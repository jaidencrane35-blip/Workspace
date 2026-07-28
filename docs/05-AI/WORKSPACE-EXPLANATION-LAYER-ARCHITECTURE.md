# Workspace Explanation Layer Architecture (Programme III — Batch 5)

| Field | Value |
|-------|-------|
| **Purpose** | Compose evidence-backed explanations across Programme III surfaces — present what is known, unknown, conflicting, and limited without becoming the authority that changes reality |
| **Owner** | `WorkspaceExplanationService` (DurableStore — **explanation evidence only**) |
| **Status** | Active — Programme III Batch 5 accepted |
| **Lifecycle owner** | No |
| **Execution / replay authority** | No |
| **Simulation / forecast / correction authority** | No |
| **Audit / policy / source authority** | No |
| **Source of truth** | No |

## Core principle

**Explain evidence. Do not become the authority that changes reality.**

```
Unified Workspace State
Policy Governance
Historical Reconstruction
Temporal Intelligence
        │
        ▼
Workspace Explanation Layer
        │
        ▼
Operator / consumer explanation surfaces (read-only)
        │
        ▼
Human or Gateway-authorised decision (elsewhere)
```

Not:

```
Explanation Layer
        │
        ▼
auto-decide / mutate / repair / predict / simulate
```

## Architectural position

Programme III stack after Batch 5:

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
Cross-surface explanation artefacts (read-only)
```

Batch 5 **composes** explanation surfaces from Batches 1–4. It does not replace them.

It answers:

> Across current state, policy evaluation, reconstruction, and temporal analysis —
> what can be explained from durable evidence, and what remains unknown or conflicting?

It does **not** answer by inventing missing causes, futures, or preferred resolutions.

## Why this batch (and not collaboration / simulation yet)

Batch 4 acceptance explicitly deferred:

- simulation
- prediction
- autonomous action

Recommended next direction: **Historical/Temporal Explanation Layer** (or collaborative understanding later).

Batch 5 chooses the explanation layer first because:

1. Batches 1–4 already produce discrete explanation fragments
2. Operators need a coherent, provenance-preserving synthesis of those fragments
3. Collaboration / simulation would expand authority risk before explanation discipline is exercised end-to-end

Collaborative Workspace Understanding remains a **later** candidate after this layer is accepted and exercised.

## Locked constraints (non-negotiable)

Do **not** introduce:

| Forbidden | Why |
|-----------|-----|
| Event sourcing | Competing authoritative log — still rejected |
| Authoritative replay | Replay must never mutate or re-decide |
| Simulation / forecasting | Futures and counterfactuals are not evidence |
| Autonomous correction | Explanation ≠ mutation authority |
| Lifecycle replacement | Sources remain sole lifecycle owners |
| Policy re-decision | Policy evaluations may be referenced, never re-authorised |
| Causal invention | “X because Y” only when Y exists as durable evidence |
| Conflict resolution | Explain disagreement; never declare a winner |
| Foreign silent `generate` | No gap-filling by regenerating upstream authorities |
| Command conversion of explanations | Explanation DTOs never become actionable commands |

## Ownership

### Explanation Layer owns

- cross-surface explanation packages / envelopes
- explanation section assembly (state / policy / history / temporal)
- explanation completeness and uncertainty roll-ups
- provenance aggregation (references only)
- narrative composition under sequence / evidence language rules
- explanation history (append-only evidence)

### Explanation Layer does **not** own

- lifecycle state / transitions
- execution / replay / restoration
- policy authority / Gateway decisions
- source mutation / envelope generation
- reconstruction or temporal analysis authority (those remain Batch 3/4 owners)
- simulation, forecasting, or autonomous correction
- collaborative multi-actor decision authority

## Relationship to prior batches

| Prior owner | Explanation Layer may |
|-------------|----------------------|
| Unified Workspace State | Reference current envelope via `load_snapshot` only |
| Policy Governance | Reference evaluation / explanation via load only — never re-evaluate to invent Compliant |
| Historical Reconstruction | Reference reconstruction artefacts via load only |
| Temporal Intelligence | Reference temporal analysis via load only |

| Prior owner | Explanation Layer must not |
|-------------|----------------------------|
| Any upstream | Call `::generate` to fill gaps |
| Any lifecycle service | Mutate / transition / execute |
| Policy / Gateway | Grant, deny, or bypass |

### Carried-forward P2 disciplines

1. **Evidence quality remains diagnostic** — never confidence authority or auto-decision input.
2. **Temporal / historical persistence remains an evidence cache** — not canonical truth tables.
3. **Conflicts are explained, not resolved.**
4. **Observed sequence ≠ cause** unless causal evidence explicitly exists.

## Core model (contract)

### `WorkspaceExplanationView`

Primary composed artefact for an explanation request.

| Field (conceptual) | Role |
|--------------------|------|
| `explanation_id` | Identity of this explanation package |
| `workspace_id` | Workspace scope |
| `generated_at` | Wall-clock composition time |
| `scope` | `ExplanationScope` (what surfaces were requested) |
| `sections` | Ordered `ExplanationSection` list |
| `completeness` | `ExplanationCompleteness` |
| `gaps` | Explicit unknowns / missing upstream evidence |
| `conflicts` | Preserved disagreement summaries (refs only) |
| `provenance_links` | Links to upstream durable artefacts |
| `narrative` | Evidence-backed synthesis (sequence language) |
| `limitations` | Explicit limitation statements |
| `authority_effect` | Always `"none"` |
| `actionable` | Always `false` |

Not a decision package. Not a command envelope. Not a replacement workspace state.

### `ExplanationScope`

Requested composition bounds.

| Field (conceptual) | Role |
|--------------------|------|
| `include_state` | Include unified state summary section |
| `include_policy` | Include governance explanation section |
| `include_reconstruction` | Include historical reconstruction section |
| `include_temporal` | Include temporal intelligence section |
| `theme` | Optional focus (e.g. conflicts-only, gaps-only) |
| `max_sections` | Hard bound — never unbounded scrape |

Scope constrains reading. It does not invent coverage.

### `ExplanationSection`

One upstream-derived explanation fragment.

| Field (conceptual) | Role |
|--------------------|------|
| `section_id` | Identity |
| `surface` | `state` / `policy` / `reconstruction` / `temporal` |
| `title` | Human-readable section title |
| `body` | Evidence-backed text |
| `evidence_refs` | Upstream durable refs |
| `completeness` | Section-local completeness |
| `authority_effect` | `"none"` |
| `actionable` | `false` |

### `ExplanationCompleteness`

Explicit uncertainty model — **never collapse**:

| State | Meaning |
|-------|---------|
| `Complete` | All requested surfaces were available with usable evidence |
| `Partial` | Some requested surfaces available; gaps recorded |
| `Unknown` | Completeness cannot be determined |
| `Contradictory` | Upstream surfaces disagree; conflicts preserved |
| `Unavailable` | Required upstream evidence could not be loaded |

**A complete-looking answer is worse than an explicit unknown.**

### `ExplanationLimitation`

First-class statement of what the package cannot claim.

Examples:

- “No reconstruction artefact present — historical section unavailable.”
- “Policy evaluation missing — never assume Compliant.”
- “Conflict preserved; no winner selected.”

## Explanation principle

### Allowed

```
State snapshot (load)
+ Policy explanation (load)
+ Reconstruction view (load)
+ Temporal analysis (load)
= composed explanation package with gaps/conflicts preserved
```

Same durable upstream revisions + same scope ⇒ deterministic explanation content
(`sections`, `gaps`, `conflicts`, `completeness`). Identity / `generated_at` may differ.

### Forbidden

```
Missing upstream
+ assumption / preferred story / forecast
= invented explanation
```

Language rules:

| Allowed | Forbidden without evidence |
|---------|----------------------------|
| “Revision A was followed by Revision B.” | “Revision A caused Revision B.” |
| “Source A reported Available while Source B reported Unknown.” | “Source A was wrong.” |
| “Policy evaluation is Unknown because context was unavailable.” | “Policy would have approved.” |
| “Evidence quality is partial (diagnostic only).” | “High quality ⇒ safe to act.” |

## Evidence inputs (read-only)

Explanation Layer **reads** durable upstream artefacts; it never generates/refreshes foreign authorities as a side effect of explaining.

| Input | Access pattern |
|-------|----------------|
| Workspace state envelope snapshot | `WorkspaceStateCompositionService::load_snapshot` only |
| Policy governance snapshot / explanation | load / explain only — never `generate` |
| Historical reconstruction snapshot | load / explain only — never `generate` |
| Temporal intelligence snapshot | load / explain only — never `generate` |

**No hidden source scraping. No lifecycle inspection shortcuts. No silent regeneration.**

## Service contract

### `WorkspaceExplanationService`

Responsibilities:

- accept an `ExplanationScope`
- load requested upstream snapshots (read-only)
- assemble sections, gaps, conflicts, limitations
- compose evidence-backed narrative
- persist explanation artefacts (read-model)

Must **not**:

- execute / replay / dispatch / restore
- simulate / forecast / auto-correct
- mutate sources or lifecycles
- call foreign `generate` paths to fill gaps
- grant permissions or call `PermissionGateway`
- invent causal “because” without evidence refs
- resolve contradictions by dropping a side
- convert explanation packages into commands

Negative guards (tests): `attempt_execute`, `attempt_approve`, `attempt_mutate_policy`,
`attempt_mutate_lifecycle`, `attempt_create_task`, `attempt_resolve_conflict`,
`attempt_fabricate_evidence`, `attempt_silent_refresh`, `attempt_emit_command`.

## Commands

| Command | Kind | Capability | Purpose |
|---------|------|------------|---------|
| `GenerateWorkspaceExplanation` | Mutation | `work_context.write` | Persist an explanation package for a scope |
| `GetWorkspaceExplanation` | Query | `work_context.read` | Load current explanation snapshot |
| `GetWorkspaceExplanationSummary` | Query | `work_context.read` | Summary + authoritative `history_count` |
| `ExplainWorkspaceSituation` | Query | `work_context.read` | Explanation surface over current/last package |

All commands:

```
IPC → CommandPipeline → PermissionGateway → WorkspaceExplanationService
```

No repair / apply / replay / predict commands.

## Persistence

| Artefact | Role |
|----------|------|
| Migration `056_workspace_explanation_layer.sql` | Read-model tables only |
| Repository `workspace_explanation.rs` | Persist / retrieve / supersede / history append |

Repository does **not** evaluate, repair, replay, simulate, forecast, resolve conflicts, or mutate sources.

No authoritative event log. No decision ledger. No command queue.

Repository remains an **explanation evidence cache**, not a truth or control database.

## Projection contract

Follow established dual-channel pattern:

| Channel | Contents | Actionable? |
|---------|----------|-------------|
| `current` | Active `WorkspaceExplanationView` | View / inspect only |
| `history` | Superseded explanation packages (append-only) | Never |
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
| Missing upstream surface | Section `Unavailable` + gap — not invented |
| Partial upstream coverage | `Partial` package — gaps explicit |
| Contradictory upstreams | `Contradictory` + conflict preserved |
| Corrupt / unusable inputs | Explanation failure evidence — not auto-repaired |
| Restart | Same durable evidence + same scope ⇒ same explanation content |
| Fabrication attempt | Fail closed — never invent causes, winners, or futures |

Recovery helpers (domain predicates):

- `recovery_must_not_fabricate_workspace_explanation`
- `recovery_must_not_fabricate_actionable_explanation_history`

## Governance additions

Mutation baseline includes `GenerateWorkspaceExplanation`.

Guards detect:

- explanation service importing lifecycle / execution / launch services
- simulate / forecast / replay / repair command paths
- history / projection DTO command / authority fields
- foreign `::generate` silent refresh from explanation compose path
- repository → service ownership leaks
- explanation DTO fields that imply execute / approve / dispatch

Ownership registry entries:

- `workspace_explanation`
- `workspace_explanation_snapshot`

History / projection DTO inventories include:

- `WorkspaceExplanationHistoryEntry`
- `WorkspaceExplanationSummary`

## Required tests (acceptance)

### Domain

- deterministic explanation from identical scope + upstream refs
- missing upstream → Unavailable / gaps (no invented sections)
- conflicts preserved without resolution
- sequence language only (no unsupported causality)
- evidence-quality / policy Unknown never promoted to authority
- history separation / non-actionability

### Kernel

- command routing + Gateway enforcement
- persistence rollback (no partial explanation write)
- no simulate / forecast / replay / lifecycle / command-emit paths
- restart continuity of durable explanation evidence
- scope bounds respected

### Projection

- DTO non-commandability
- history separation
- `history_count` authority
- explanation separation from commands

### Governance

- import restrictions
- architecture map refresh
- mutation baseline + DTO inventory updates

### Recovery

- missing evidence does not become Complete
- restart preserves evidence without fabricating explanation

## Documentation deliverables (on implementation)

- This charter → status `Active` after review approval
- Update `PROGRAMME-III-COHERENT-WORKSPACE-RUNTIME.md`
- Update `ARCHITECTURE-GOVERNANCE.md`, `PROJECTION-INTEGRITY.md`, `OPERATIONAL-RECOVERY.md`
- Vocabulary: Explanation Layer ≠ decision authority / simulator / command surface

## Batch 5 acceptance criteria

Accept implementation only when:

1. Explanation Layer exists as composed explanation evidence only
2. Batches 1–4 remain authoritative in their domains
3. No simulation, forecasting, event sourcing, replay, or autonomous correction introduced
4. No lifecycle replacement or mutation through explanation history
5. Missing / conflicting upstream evidence stays explicit — never synthetic certainty
6. Upstream read boundaries (`load` / list only) remain absolute
7. Projection integrity and Gateway/Pipeline boundaries remain absolute
8. Governance detects explanation-layer boundary violations
9. Recovery never fabricates Complete explanations from gaps

## Review gate

**Implementation contract accepted.** Implementation proceeds only against this contract —
no simulator, no forecaster, no SoT elevation, no autonomous correction, no collaborative
decision authority.

Explain evidence. Do not become the authority that changes reality.

Do not start Batch 6 until Batch 5 is audited and accepted.

## Related

- [Programme III — Coherent Workspace Runtime](./PROGRAMME-III-COHERENT-WORKSPACE-RUNTIME.md)
- [Temporal Intelligence Architecture](./TEMPORAL-INTELLIGENCE-ARCHITECTURE.md) (Batch 4)
- [Historical Workspace Reconstruction Architecture](./HISTORICAL-WORKSPACE-RECONSTRUCTION-ARCHITECTURE.md) (Batch 3)
- [Policy & Governance Architecture](./POLICY-GOVERNANCE-ARCHITECTURE.md) (Batch 2)
- [Unified Workspace State Architecture](./UNIFIED-WORKSPACE-STATE-ARCHITECTURE.md) (Batch 1)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
- [Operational Recovery](../03-Engineering/OPERATIONAL-RECOVERY.md)
