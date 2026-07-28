# Historical Workspace Reconstruction Architecture (Programme III — Batch 3)

| Field | Value |
|-------|-------|
| **Purpose** | Temporal understanding — explain how workspace evidence changed over time |
| **Owner** | `WorkspaceHistoricalReconstructionService` (DurableStore — **reconstruction evidence only**) |
| **Status** | Active — Programme III Batch 3 accepted |
| **Lifecycle owner** | No |
| **Execution / replay authority** | No |
| **Audit / policy / source authority** | No |
| **Source of truth** | No |

## Core principle

**Reconstruction explains change over time. It does not become the source of truth.**

```
Authoritative Sources
        │
        ▼
Unified Workspace State (snapshots / revisions)
        │
        ▼
Historical Reconstruction
        │
        ▼
Explanation Evidence
```

Not:

```
Historical Reconstruction
        │
        ▼
creates truth / invents history / replays mutations
```

## Architectural position

Programme III stack after Batch 3:

```
Unified Workspace State Model
        ↓
Policy & Governance Engine
        ↓
Historical Workspace Reconstruction
        ↓
Temporal Understanding / Explanation Surfaces
```

Historical Reconstruction is an observational composition layer over durable evidence.
It answers:

> How did the workspace arrive here, and what evidence changed between revisions?

It does **not** answer by inventing missing events.

## Locked constraints (non-negotiable)

Do **not** introduce:

| Forbidden | Why |
|-----------|-----|
| Event sourcing | Would create a competing authoritative log |
| Authoritative replay logs | Replay must never mutate or re-decide |
| Lifecycle replacement | Sources remain sole lifecycle owners |
| Mutation through history | History is evidence, never a control surface |
| Synthetic certainty from missing evidence | Unknown stays Unknown |

## Ownership

### Historical Reconstruction owns

- temporal views
- change explanations
- evidence timelines
- revision comparison artefacts
- provenance links between compared revisions
- confidence about **reconstruction completeness** (not about source truth)
- reconstruction history (append-only evidence)

### Historical Reconstruction does **not** own

- lifecycle state
- execution history authority
- audit authority (audit remains observational evidence input)
- policy authority (policy evaluations may be *referenced*, never re-decided)
- source mutation
- replay execution
- permission grants / Gateway decisions

## Core model (contract)

### `WorkspaceHistoricalView`

Primary composed artefact for a reconstruction request.

| Field (conceptual) | Role |
|--------------------|------|
| `reconstruction_id` | Identity of this reconstruction artefact |
| `workspace_id` | Workspace scope |
| `from_revision` / `to_revision` | Compared envelope (or evidence) revisions |
| `generated_at` | Wall-clock composition time |
| `timeline` | Ordered `StateChangeEvidence` entries |
| `comparisons` | `RevisionComparison` results |
| `completeness` | `ReconstructionCompleteness` |
| `gaps` | `EvidenceGap` list |
| `provenance_links` | Links to source snapshots / audit / projection history |
| `authority_effect` | Always `"none"` |
| `actionable` | Always `false` |

### `TemporalSnapshot`

A point-in-time reference used in reconstruction — **not** a new SoT.

| Field (conceptual) | Role |
|--------------------|------|
| `snapshot_ref` | Canonical reference (e.g. envelope `state_id` / revision) |
| `observed_at` | Source-declared time |
| `source_kind` | Envelope / audit window / projection history / policy evaluation |
| `availability` | Available / Unavailable / Unknown |
| `authority_effect` | `"none"` |

### `StateChangeEvidence`

Explainable delta evidence between two temporal points.

| Field (conceptual) | Role |
|--------------------|------|
| `change_id` | Evidence identity |
| `from_ref` / `to_ref` | Compared temporal refs |
| `description` | Human-readable change explanation |
| `evidence_refs` | Provenance to durable artefacts |
| `confidence` | Confidence about the **explanation**, not invented facts |
| `authority_effect` | `"none"` |
| `actionable` | `false` |

### `RevisionComparison`

Deterministic comparison of two known revisions.

| Field (conceptual) | Role |
|--------------------|------|
| `comparison_id` | Identity |
| `left_revision` / `right_revision` | Compared revisions |
| `added` / `removed` / `changed` | Structured deltas (references only) |
| `unchanged` | Optional reference counts |
| `result_status` | Complete / Partial / Unknown / Contradictory / Unavailable |

### `ReconstructionCompleteness` / `EvidenceGap`

Explicit uncertainty model — **never collapse**:

| State | Meaning |
|-------|---------|
| `Complete` | All required evidence channels for the requested window were available |
| `Partial` | Some channels available; gaps recorded |
| `Unknown` | Completeness cannot be determined |
| `Contradictory` | Evidence channels disagree; conflict preserved |
| `Unavailable` | Required evidence channel could not be loaded |

### `EvidenceGap`

| Field (conceptual) | Role |
|--------------------|------|
| `gap_id` | Identity |
| `channel` | Missing/unavailable evidence channel |
| `description` | Why reconstruction is incomplete |
| `severity` | Informational severity for explanation |
| `authority_effect` | `"none"` |

## Reconstruction principle

### Allowed

```
Snapshot A
+ Snapshot B
+ Audit evidence (observational)
+ Projection history (evidence-only)
= explanation of change
```

Same inputs + same declared revisions ⇒ deterministic reconstruction artefact content
(`timeline`, `comparisons`, `gaps`, `completeness`). Identity/`generated_at` may differ.

### Forbidden

```
Missing event
+ assumption
= invented history
```

Also forbidden:

- fabricating transitions not supported by evidence
- treating projection history as executable replay
- “repairing” corrupted timelines into Complete
- elevating reconstruction confidence when gaps exist

## Evidence inputs (read-only)

Reconstruction **reads** durable evidence; it never generates/refreshes foreign authorities as a side effect of explaining history.

Primary inputs (Batch 3 minimum):

| Input | Access pattern |
|-------|----------------|
| Unified Workspace State envelopes + envelope history | `load_snapshot` / history channels only |
| Projection history of Programme II/III dual-channel surfaces | load / list history only |
| Audit events (observational windows) | read-only query; no mutation |

Policy evaluations may be **referenced** as provenance when present; reconstruction must not re-run policy to invent compliance.

## Service contract

### `WorkspaceHistoricalReconstructionService`

Responsibilities:

- load temporal refs / envelope revisions
- compose timelines and revision comparisons
- record evidence gaps and completeness
- produce explanations
- persist reconstruction artefacts (read-model)

Must **not**:

- execute / replay / dispatch
- mutate sources or lifecycles
- call foreign `generate` paths to “fill gaps”
- grant permissions or call `PermissionGateway`
- assume missing evidence is continuity

Negative guards (tests): `attempt_execute`, `attempt_replay`, `attempt_mutate_lifecycle`, `attempt_fabricate_transition`, `attempt_silent_refresh`.

## Commands

| Command | Kind | Capability | Purpose |
|---------|------|------------|---------|
| `GenerateHistoricalWorkspaceView` | Mutation | `work_context.write` | Persist a reconstruction artefact for a revision window |
| `GetHistoricalWorkspaceView` | Query | `work_context.read` | Load current reconstruction snapshot |
| `GetHistoricalWorkspaceSummary` | Query | `work_context.read` | Summary + authoritative `history_count` |
| `CompareWorkspaceRevisions` | Query | `work_context.read` | Deterministic revision comparison without requiring a new persisted current |
| `ExplainHistoricalChange` | Query | `work_context.read` | Explanation surface over last/current reconstruction |

All commands:

```
IPC → CommandPipeline → PermissionGateway → WorkspaceHistoricalReconstructionService
```

## Persistence

| Artefact | Role |
|----------|------|
| Migration `054_workspace_historical_reconstruction.sql` | Read-model tables only |
| Repository `historical_reconstruction.rs` | Persist / retrieve / supersede / history append |

Repository does **not** evaluate, repair, replay, or mutate sources.

No authoritative event log table. No replay cursor. No “event → apply” pathway.

## Projection contract

Follow established dual-channel pattern:

| Channel | Contents | Actionable? |
|---------|----------|-------------|
| `current` | Active `WorkspaceHistoricalView` | View / inspect only |
| `history` | Superseded reconstructions (append-only) | Never |
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
| Missing evidence | `Unknown` / `EvidenceGap` — **not** confidently reconstructed |
| Corrupted / unusable timeline inputs | `Incomplete` / `Unavailable` — **not** auto-repaired |
| Restart | Durable reconstruction artefacts reload; missing remains missing |
| Fabrication attempt | Fail closed — never invent transitions |

Recovery helpers (domain predicates):

- `recovery_must_not_fabricate_historical_reconstruction`
- `recovery_must_not_fabricate_actionable_historical_history`

## Governance additions

Mutation baseline includes `GenerateHistoricalWorkspaceView`.

Guards detect:

- reconstruction service importing lifecycle mutators / execution / launch services
- replay / execution paths (`ExecutionLifecycleService`, process spawn, etc.)
- history / projection DTO command / authority fields
- mutation ownership leaks
- direct source writes from reconstruction repository
- foreign `::generate` silent refresh from reconstruction compose path

Ownership registry entries:

- `historical_reconstruction`
- `historical_reconstruction_snapshot`

History / projection DTO inventories include:

- `HistoricalReconstructionHistoryEntry`
- `HistoricalReconstructionSummary`

## Required tests (acceptance)

### Domain

- deterministic reconstruction from identical revisions
- missing evidence → Unknown / gaps (no invented transitions)
- contradictory evidence preserved
- completeness states never collapse
- history separation / non-actionability

### Kernel

- command routing + Gateway enforcement
- persistence rollback (no partial reconstruction write)
- missing evidence handling
- no replay authority / no lifecycle mutation path
- restart continuity of durable reconstruction evidence

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
- restart preserves evidence without fabricating transitions

## Documentation deliverables (on implementation)

- This charter → status `Active` after review approval
- Update `PROGRAMME-III-COHERENT-WORKSPACE-RUNTIME.md`
- Update `ARCHITECTURE-GOVERNANCE.md`, `PROJECTION-INTEGRITY.md`, `OPERATIONAL-RECOVERY.md`
- Vocabulary entry: Historical Reconstruction ≠ event log / replay authority

## Batch 3 acceptance criteria

Accept implementation only when:

1. Historical Reconstruction exists as explanation evidence only
2. Sources / Unified State / Policy remain authoritative in their domains
3. No event sourcing or authoritative replay log introduced
4. No lifecycle replacement or mutation through history
5. Missing evidence stays Unknown / gapped — never synthetic certainty
6. Projection integrity and Gateway/Pipeline boundaries remain absolute
7. Governance detects reconstruction boundary violations
8. Recovery never fabricates transitions or Complete reconstructions from gaps

## Review gate

**Charter accepted (Architecture grade A).** Implementation proceeds only against this contract —
no event store, no replay executor, no SoT elevation.

## Related

- [Programme III — Coherent Workspace Runtime](./PROGRAMME-III-COHERENT-WORKSPACE-RUNTIME.md)
- [Temporal Intelligence Architecture](./TEMPORAL-INTELLIGENCE-ARCHITECTURE.md) (Batch 4 charter)
- [Unified Workspace State Architecture](./UNIFIED-WORKSPACE-STATE-ARCHITECTURE.md)
- [Policy & Governance Architecture](./POLICY-GOVERNANCE-ARCHITECTURE.md)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
- [Operational Recovery](../03-Engineering/OPERATIONAL-RECOVERY.md)
