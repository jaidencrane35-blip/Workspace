# Temporal Intelligence Architecture (Programme III — Batch 4)

| Field | Value |
|-------|-------|
| **Purpose** | Deepen historical understanding — organise, scope, and surface durable reconstruction evidence without inventing futures or rewriting pasts |
| **Owner** | `WorkspaceTemporalIntelligenceService` (DurableStore — **understanding evidence only**) |
| **Status** | Charter draft — pending review before implementation |
| **Lifecycle owner** | No |
| **Execution / replay authority** | No |
| **Simulation / forecast authority** | No |
| **Audit / policy / source authority** | No |
| **Source of truth** | No |

## Core principle

**Temporal intelligence organises and deepens historical understanding. It does not predict, simulate, or correct.**

```
Durable Evidence
        │
        ▼
Historical Reconstruction (Batch 3)
        │
        ▼
Temporal Intelligence (Batch 4)
        │
        ▼
Understanding Surfaces (scoped, evidence-backed, uncertainty-preserving)
```

Not:

```
Temporal Intelligence
        │
        ▼
forecast / simulate / auto-correct / invent causal stories
```

## Architectural position

Programme III stack after Batch 4:

```
Unified Workspace State Model
        ↓
Policy & Governance Engine
        ↓
Historical Workspace Reconstruction
        ↓
Temporal Intelligence / Historical Understanding Extensions
        ↓
Operator understanding surfaces (read-only)
```

Batch 4 **extends** Batch 3 reconstruction. It does not replace it.

It answers:

> Given durable reconstruction evidence, what longer chains, conflicts, scopes, and
> evidence-quality signals can we surface — without inventing what evidence does not support?

It does **not** answer:

- what will happen next (forecasting)
- what would happen if (simulation)
- what the system should do now (autonomous correction / execution)
- what must have happened when evidence is missing (invented history)

## Locked constraints (non-negotiable)

Do **not** introduce:

| Forbidden | Why |
|-----------|-----|
| Event sourcing | Competing authoritative log — rejected in Batch 3; remains rejected |
| Authoritative replay | Replay must never mutate or re-decide |
| Simulation engines | Out of scope until reconstruction is exercised against real complexity |
| Forecasting / prediction models | Futures are not evidence |
| Autonomous correction | Understanding ≠ mutation authority |
| Lifecycle replacement | Sources remain sole lifecycle owners |
| Hidden source scraping | Reconstruction input remains durable envelopes / declared evidence channels |
| Foreign service regeneration | No silent `generate` to “fill” understanding gaps |
| Causal narratives without evidence | “The user/system did X because Y” only when Y exists as evidence |

## Ownership

### Temporal Intelligence owns

- scoped temporal understanding views (windows / chains / themes)
- long-chain reconstruction summaries derived from Batch 3 artefacts
- conflict / contradiction explanation envelopes (evidence-backed)
- evidence-quality and understanding-completeness signals
- provenance aggregation across reconstruction artefacts (references only)
- temporal intelligence history (append-only evidence)
- operator-facing understanding summaries (non-actionable)

### Temporal Intelligence does **not** own

- lifecycle state
- execution / replay / restoration
- audit authority (audit remains observational input by reference)
- policy authority (policy evaluations may be referenced, never re-decided)
- source mutation / envelope generation
- simulation or forecasting outputs as truth
- permission grants / Gateway decisions
- autonomous repair of missing history

## Relationship to Batch 3

| Batch 3 owns | Batch 4 extends |
|--------------|-----------------|
| Point / pair reconstruction | Multi-revision chain understanding |
| `StateChangeEvidence` / gaps | Conflict explanation surfaces over those gaps |
| `ReconstructionCompleteness` | Understanding completeness over a requested window |
| `ExplainHistoricalChange` | Scoped / themed understanding narratives (still evidence-backed) |
| Durable envelope dependency | **Preserved** — no expansion into hidden sources |

### Accepted Batch 3 P2 constraints (carried forward)

1. **Durable envelope dependency is intentional.**  
   Temporal intelligence can only deepen what durable reconstruction evidence contains.
2. **Completeness stays aggressively conservative.**  
   Missing evidence ≠ no change. Unknown cause ≠ generated explanation.
3. **Explain surfaces remain evidence-disciplined.**  
   No causal “because” without supporting evidence refs.

## Core model (contract)

### `TemporalUnderstandingView`

Primary composed artefact for a temporal intelligence request.

| Field (conceptual) | Role |
|--------------------|------|
| `understanding_id` | Identity of this understanding artefact |
| `workspace_id` | Workspace scope |
| `window` | Requested temporal / revision window |
| `generated_at` | Wall-clock composition time |
| `chain_summary` | Ordered multi-revision understanding summary |
| `conflict_explanations` | Evidence-backed contradiction surfaces |
| `evidence_quality` | Quality / coverage signals (not authority) |
| `completeness` | `UnderstandingCompleteness` |
| `gaps` | Inherited + newly scoped `EvidenceGap` refs |
| `provenance_links` | Links to reconstruction / envelope / policy / audit refs |
| `narrative` | Evidence-backed understanding text |
| `authority_effect` | Always `"none"` |
| `actionable` | Always `false` |

Not a replacement historical workspace. Not a forecast.

### `TemporalWindow`

Requested scope for understanding.

| Field (conceptual) | Role |
|--------------------|------|
| `from_revision` / `to_revision` | Optional revision bounds |
| `from_observed_at` / `to_observed_at` | Optional time bounds |
| `max_chain_length` | Hard bound — never unbounded scrape |
| `theme` | Optional focus (e.g. conflicts-only, completeness-only) |

Windows constrain reading. They do not invent coverage.

### `RevisionChainSummary`

Long-chain understanding over ordered reconstruction / envelope revisions.

| Field (conceptual) | Role |
|--------------------|------|
| `chain_id` | Identity |
| `ordered_refs` | Provenance-ordered revision / reconstruction refs |
| `observed_transitions` | Counts / refs of evidenced changes only |
| `unchanged_spans` | Spans where evidence shows continuity **or** insufficient evidence to claim change — distinguished |
| `gap_spans` | Spans where evidence is missing (not “nothing happened”) |
| `completeness` | Chain-level completeness |

### `ConflictExplanation`

Evidence-backed surface for contradictory durable evidence.

| Field (conceptual) | Role |
|--------------------|------|
| `conflict_id` | Identity |
| `subject_refs` | Conflicting evidence refs |
| `description` | What disagrees (observed) |
| `evidence_refs` | Durable provenance |
| `uncertainty` | What remains unknown |
| `authority_effect` | `"none"` |
| `actionable` | `false` |

Must never resolve conflicts by preferring a side. Preservation > neatness.

### `EvidenceQualitySignal`

Understanding-quality metadata — not a confidence claim about source truth.

| Field (conceptual) | Role |
|--------------------|------|
| `coverage` | How much of the requested window had durable evidence |
| `contradiction_density` | Observed conflict pressure |
| `gap_severity` | Inherited gap severity roll-up |
| `limitations` | Explicit limitation statements |

### `UnderstandingCompleteness`

Explicit uncertainty model — **never collapse**:

| State | Meaning |
|-------|---------|
| `Complete` | All required evidence channels for the requested window were available |
| `Partial` | Some channels / spans available; gaps recorded |
| `Unknown` | Completeness cannot be determined |
| `Contradictory` | Evidence channels disagree; conflicts preserved |
| `Unavailable` | Required evidence channel / reconstruction input could not be loaded |

**A complete-looking answer is worse than an explicit unknown.**

## Understanding principle

### Allowed

```
Reconstruction A..N
+ declared temporal window
+ conflict / gap evidence
= scoped understanding artefact
```

Same durable inputs + same window ⇒ deterministic understanding content
(`chain_summary`, `conflict_explanations`, `gaps`, `completeness`).
Identity / `generated_at` may differ.

### Forbidden

```
Missing span
+ assumption / forecast / “likely cause”
= invented understanding
```

Also forbidden:

- fabricating transitions not supported by Batch 3 evidence
- treating understanding history as executable replay
- “repairing” contradictory chains into Complete
- elevating narrative confidence when gaps exist
- emitting forecasts or simulated alternate timelines

## Evidence inputs (read-only)

Temporal intelligence **reads** durable evidence; it never generates/refreshes foreign authorities as a side effect of deepening understanding.

Primary inputs (Batch 4 minimum):

| Input | Access pattern |
|-------|----------------|
| Historical reconstruction artefacts + history | `load_snapshot` / history only — never foreign `generate` |
| Durable workspace state envelopes | via reconstruction boundaries / `list_durable_envelopes` only |
| Policy evaluation artefacts (optional provenance) | load / reference only — never re-evaluate to invent compliance |
| Audit windows (observational) | read-only query; no mutation |

**No hidden source scraping. No lifecycle inspection shortcuts. No silent regeneration.**

## Service contract (planned)

### `WorkspaceTemporalIntelligenceService`

Responsibilities:

- accept a `TemporalWindow`
- load durable reconstruction / envelope evidence within bounds
- compose chain summaries and conflict explanations
- record understanding completeness and evidence-quality signals
- produce evidence-backed narratives
- persist understanding artefacts (read-model)

Must **not**:

- execute / replay / dispatch / restore
- simulate alternate histories
- forecast future states
- mutate sources or lifecycles
- call foreign `generate` paths to fill gaps
- grant permissions or call `PermissionGateway`
- invent causal “because” without evidence refs
- resolve contradictions by dropping a side

Negative guards (tests): `attempt_execute`, `attempt_replay`, `attempt_simulate`,
`attempt_forecast`, `attempt_mutate_lifecycle`, `attempt_fabricate_cause`,
`attempt_silent_refresh`.

## Commands (planned)

| Command | Kind | Capability | Purpose |
|---------|------|------------|---------|
| `GenerateTemporalUnderstanding` | Mutation | `work_context.write` | Persist an understanding artefact for a window |
| `GetTemporalUnderstanding` | Query | `work_context.read` | Load current understanding snapshot |
| `GetTemporalUnderstandingSummary` | Query | `work_context.read` | Summary + authoritative `history_count` |
| `ExplainTemporalConflicts` | Query | `work_context.read` | Conflict explanation surface (evidence-backed) |
| `SummariseRevisionChain` | Query | `work_context.read` | Long-chain summary without requiring a new persisted current |

All commands:

```
IPC → CommandPipeline → PermissionGateway → WorkspaceTemporalIntelligenceService
```

No repair commands. No simulate / forecast commands. No lifecycle controls.

## Persistence (planned)

| Artefact | Role |
|----------|------|
| Migration `055_workspace_temporal_intelligence.sql` | Read-model tables only |
| Repository `temporal_intelligence.rs` | Persist / retrieve / supersede / history append |

Repository does **not** evaluate, repair, replay, simulate, forecast, or mutate sources.

No authoritative event log. No replay cursor. No simulation store. No forecast ledger.

Repository remains an **understanding evidence cache**, not a truth database.

## Projection contract

Follow established dual-channel pattern:

| Channel | Contents | Actionable? |
|---------|----------|-------------|
| `current` | Active `TemporalUnderstandingView` | View / inspect only |
| `history` | Superseded understandings (append-only) | Never |
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
| Missing reconstruction / envelope evidence | `Unavailable` / gaps — not confidently understood |
| Partial window coverage | `Partial` understanding — gap spans explicit |
| Contradictory revisions | `Contradictory` + `ConflictExplanation` preserved |
| Corrupt / unusable inputs | Understanding failure evidence — not auto-repaired |
| Restart | Same durable evidence + same window ⇒ same understanding content |
| Fabrication attempt | Fail closed — never invent causes or transitions |

Recovery helpers (domain predicates):

- `recovery_must_not_fabricate_temporal_understanding`
- `recovery_must_not_fabricate_actionable_temporal_history`

No recovery system may “repair” history or invent futures.

## Governance additions (planned)

Increase mutation baseline when `GenerateTemporalUnderstanding` lands.

Add guards detecting:

- temporal intelligence importing lifecycle / execution / launch services
- simulate / forecast / replay command paths
- history / projection DTO command / authority fields
- foreign `::generate` silent refresh from understanding compose path
- repository → service ownership leaks
- source-domain writes from temporal intelligence repository

Add ownership registry entries:

- `temporal_intelligence`
- `temporal_intelligence_snapshot`

History / projection DTO inventories gain:

- `TemporalIntelligenceHistoryEntry`
- `TemporalIntelligenceSummary`

## Required tests (acceptance)

### Domain

- deterministic understanding from identical window + inputs
- missing evidence → Unavailable / gaps (no invented transitions or causes)
- long chain summaries only cite evidenced transitions
- contradictory evidence preserved via `ConflictExplanation`
- completeness never collapses
- history separation / non-actionability
- narrative contains no unsupported causal claims

### Kernel

- command routing + Gateway enforcement
- persistence rollback (no partial understanding write)
- no simulate / forecast / replay / lifecycle mutation paths
- restart continuity of durable understanding evidence
- window bounds respected (no unbounded scrape)

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
- restart preserves evidence without fabricating understanding

## Documentation deliverables (on implementation)

- This charter → status `Active` after review approval
- Update `PROGRAMME-III-COHERENT-WORKSPACE-RUNTIME.md`
- Update `ARCHITECTURE-GOVERNANCE.md`, `PROJECTION-INTEGRITY.md`, `OPERATIONAL-RECOVERY.md`
- Vocabulary: Temporal Intelligence ≠ simulation / forecast / correction / event log

## Batch 4 acceptance criteria

Accept implementation only when:

1. Temporal Intelligence exists as understanding evidence only
2. Batch 3 reconstruction remains the temporal comparison authority it already is
3. No simulation, forecasting, event sourcing, or autonomous correction introduced
4. No lifecycle replacement or mutation through understanding history
5. Missing evidence stays Unknown / gapped — never synthetic certainty or invented causes
6. Durable envelope / reconstruction read boundaries remain absolute
7. Projection integrity and Gateway/Pipeline boundaries remain absolute
8. Governance detects temporal-intelligence boundary violations
9. Recovery never fabricates Complete understandings from gaps

## Review gate

**This document is a charter/contract draft.**

Do **not** begin Batch 4 implementation until this charter is reviewed and explicitly approved.
After approval, implement only the contract herein — no simulator, no forecaster, no SoT elevation,
no autonomous correction.

Exercise the reconstruction layer against real complexity before expanding scope further.

## Related

- [Programme III — Coherent Workspace Runtime](./PROGRAMME-III-COHERENT-WORKSPACE-RUNTIME.md)
- [Historical Workspace Reconstruction Architecture](./HISTORICAL-WORKSPACE-RECONSTRUCTION-ARCHITECTURE.md)
- [Unified Workspace State Architecture](./UNIFIED-WORKSPACE-STATE-ARCHITECTURE.md)
- [Policy & Governance Architecture](./POLICY-GOVERNANCE-ARCHITECTURE.md)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
- [Operational Recovery](../03-Engineering/OPERATIONAL-RECOVERY.md)
