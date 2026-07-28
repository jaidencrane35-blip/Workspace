# Workspace Insight Coordination Architecture (Programme III — Batch 9)

| Field | Value |
|-------|-------|
| **Purpose** | Provide a governed **read-only coordination surface** that identifies relationships between existing Programme III evidence products and presents **diagnostic insight opportunities** — without becoming a planner, decision engine, autonomy layer, policy authority, recommendation executor, or knowledge / Cognitive Model replacement |
| **Owner** | `WorkspaceInsightCoordinationService` (DurableStore — **coordination artefacts only**) |
| **Status** | Charter draft — pending review before implementation |
| **Lifecycle owner** | No |
| **Execution / replay authority** | No |
| **Simulation / forecast / correction authority** | No |
| **Decision / policy / permission authority** | No |
| **Planner / Recommendation Engine / Decision Engine** | No |
| **Autonomy / Cognitive Autonomy replacement** | No |
| **Memory / Cognitive Model / Knowledge Synthesis / Knowledge Integration replacement** | No |
| **Source of truth** | No |
| **Autonomous authority** | No |

## Core principle

**Coordinate understanding. Never create authority.**

```
Programme III evidence stack (Batches 1–8)
        │
        ▼
Workspace Insight Coordination
        │
        ▼
Coordinated insight views (read-only)
        │
        ▼
Human or Gateway-authorised decision (elsewhere)
```

Not:

```
Insight Coordination
        │
        ▼
plan / decide / approve / execute / auto-act / ranked truth
```

The layer **may** say:

- “multiple evidence sources indicate this theme”
- “these explanations overlap”
- “these knowledge areas intersect”
- “these gaps remain unresolved”

The layer **may not** say:

- “do this”
- “execute this”
- “approve this”
- “change this”
- “this is definitely true”

### Locked invariants (must remain visible in docs and tests)

- **coordination ≠ authority**
- **prioritisation ≠ recommendation**
- **intersection ≠ causation**
- **confidence ≠ permission**
- **attention metadata ≠ action queue**

## Architectural position

Programme III stack after Batch 9 (extension beyond the Batch 8 evidence-stack close):

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
Workspace Knowledge Synthesis
        ↓
Workspace Knowledge Integration / Retrieval Intelligence
        ↓
Workspace Insight Coordination
        ↓
Coordinated insight artefacts (read-only)
```

Batch 8 closed the **coherent evidence stack** (compose / retrieve / lineage).
Batch 9 adds a **coordination surface over that stack** — it does not reopen ownership
of upstream layers and does not invent a ninth source of truth.

### Distinction from prior batches (non-duplication)

| Layer | Question it answers |
|-------|---------------------|
| Contextual Understanding (Batch 6) | What is the **situational picture now**? |
| Knowledge Synthesis (Batch 7) | What **structured concepts / clusters / relationships** can be derived? |
| Knowledge Integration (Batch 8) | Given a frame, **what integrated evidence slice** can be assembled? |
| Insight Coordination (Batch 9) | Across already-derived products, **where do evidence products relate, overlap, or leave unresolved areas** — with diagnostic attention metadata only? |

Batch 9 must not:

- re-derive concepts as if it owned Knowledge Synthesis
- re-frame retrieval hits as if it owned Knowledge Integration
- re-author situational themes as if it owned Contextual Understanding
- re-decide policy as if it owned Policy Governance
- suggest / present / confirm recommendations as if it owned Recommendation Engine
- select / progress / evaluate candidates as if it owned Decision Engine / Decision Queue
- propose autonomy opportunities as if it owned Cognitive Autonomy
- store experience as if it were Memory
- edit workspace semantics as if it were Cognitive Model

It answers:

> Given durable Programme III artefacts available now (and what is missing /
> conflicting), what coordinated view of **intersections, clusters, and unresolved
> areas** can be assembled — with every claim bound to evidence lineage — without
> claiming truth, priority-as-action, or authority?

It does **not** answer:

- what the system should do next
- what to approve / execute / change
- what is “really true” as a second SoT
- what Memory / Cognitive Model should become
- what will happen next / what would happen if
- what to invent when evidence is missing

## Why this batch

Batches 1–8 accumulate durable evidence products. Without a coordination layer,
consumers either:

1. reinvent cross-layer joining ad hoc (drift / duplication), or
2. collapse coordination into Recommendation / Decision / Autonomy (authority leak).

Batch 9 exists to prevent both failure modes: a **governed, read-only coordination
surface** that stays evidence-only.

Constraints remain absolute:

- no new source of truth
- no autonomous authority
- no execution
- no planner / decision / recommendation ownership
- no replacement of Memory / Cognitive Model / Knowledge Synthesis / Knowledge Integration
- no silent evidence refresh
- no fabricated intersections or clusters
- no conversion of diagnostic prioritisation into action / policy / permission

**Do not begin Batch 10 until Batch 9 receives architecture acceptance.**
No Batch 10 is implied by this charter.

## Locked constraints (non-negotiable)

Do **not** introduce:

| Forbidden | Why |
|-----------|-----|
| New source of truth | Coordination views are derived projections over existing authorities |
| Memory replacement | Memory retains stored experience ownership |
| Cognitive Model replacement | Cognitive Model retains durable semantic structure |
| Knowledge Synthesis / Integration replacement | Concepts / retrieval remain Batches 7–8 |
| Recommendation Engine / Decision Engine / Decision Queue | Actionable suggestion / candidate ownership stays elsewhere |
| Cognitive Autonomy replacement | Governed opportunity / approval flow stays Programme II Batch 8 |
| Planner / Task Graph mutation | Coordination is not sequencing or task creation |
| Policy re-decision / PermissionGateway ownership | Gateway remains final authoriser |
| Autonomous authority | No self-acting agent, no hidden workflow owner |
| Event sourcing / authoritative replay | Still rejected |
| Simulation / forecasting | Futures are not evidence |
| Causal invention | Unsupported “because” / causes remain forbidden |
| Silent foreign `generate` | No gap-filling by regenerating upstream authorities |
| Opaque “top insights to act on” | Attention metadata ≠ action queue |
| Confidence → authority | Diagnostic confidence ≠ approval / execution / priority-as-permission |

## Ownership

### `WorkspaceInsightCoordinationService` owns

- insight grouping (`InsightCluster`)
- evidence intersection summaries (`EvidenceIntersection`)
- cross-layer relationship framing (meaning-only)
- unresolved-area detection (`InsightGap`)
- diagnostic prioritisation / attention metadata (`CoordinationAssessment` + cluster ordering fields)
- dual-channel projection: `current` / `history` / authoritative `history_count`
- persistence of **derived coordination artefacts only**

### Does **not** own

| Concern | Remains with |
|---------|--------------|
| Lifecycle | Domain lifecycle services |
| Tasks / Task Graph | Task Graph service |
| Intent | Intent / Intent Proposal owners |
| Recommendations | Recommendation Engine |
| Decisions / candidates | Decision Engine / Decision Queue |
| Policy decisions | Policy Governance (+ Gateway authorisation) |
| Permissions / capability grants | PermissionGateway |
| Execution / launch | Execution lifecycle / Application launch |
| Memory truth | Memory |
| Cognitive truth | Cognitive Model |
| Concept derivation | Knowledge Synthesis |
| Retrieval composition | Knowledge Integration |
| Situational themes | Contextual Understanding |

Existing services remain authoritative. Insight Coordination **reads**; it does not replace.

## Allowed inputs

Read only via `load_snapshot()` **only**:

| Upstream | Access |
|----------|--------|
| Workspace State Envelope | `load_snapshot` |
| Policy Governance | `load_snapshot` |
| Historical Reconstruction | `load_snapshot` |
| Temporal Intelligence | `load_snapshot` |
| Explanation Layer | `load_snapshot` |
| Contextual Understanding | `load_snapshot` |
| Knowledge Synthesis | `load_snapshot` |
| Knowledge Integration | `load_snapshot` |

**Never:**

- call `Generate*` / `::generate` on upstream services
- mutate upstream artefacts
- create hidden caches of truth (revision-unaware “previous coordination = current reality”)
- direct foreign repository reads that bypass service envelopes
- invent intersections without lineage

## Domain model (charter)

Minimal model aligned with Programme III dual-channel patterns.

### `InsightCoordinationSnapshot` (artefact)

Primary composed coordination artefact — **read-only analytical artefact**.

| Field | Role |
|-------|------|
| `coordination_id` | Identity of this coordination artefact |
| `workspace_id` | Workspace scope |
| `generated_at` | Wall-clock composition time |
| `status` / `superseded_at` | Active vs superseded lifecycle of the artefact |
| `frame` | Optional `InsightCoordinationFrame` (scope / surface selection) |
| `source_revisions` | Revision-bound upstream refs used for this compose |
| `clusters` | `InsightCluster[]` |
| `intersections` | `EvidenceIntersection[]` |
| `gaps` | `InsightGap[]` |
| `assessment` | `CoordinationAssessment` |
| `completeness` | Explicit completeness state |
| `provenance_links` | Links to upstream durable artefacts |
| `summary` / `narrative` | Evidence-backed coordination narrative |
| `limitations` | Explicit limitation statements |
| `authority_effect` | Always `"none"` |
| `actionable` | Always `false` |
| `terminal` | Terminal when superseded into history |

Not a Memory write. Not a Cognitive Model mutation. Not a decision package.
Not a recommendation list. Not a plan. Not an action queue.

### Dual-channel projection

| Channel | Type | Contract |
|---------|------|----------|
| `current` | `Option<InsightCoordinationSnapshot>` | Active coordination artefact |
| `history` | `InsightCoordinationHistoryEntry[]` | Append-only superseded evidence |
| `history_count` | `usize` | **Authoritative** count (summary windows may truncate `history`) |

`InsightCoordinationProjection` assembles the dual channel.
`InsightCoordinationSummary` may truncate `history` but must preserve authoritative `history_count`.

All history entries:

- `actionable: false`
- `authority_effect: "none"`
- `terminal: true`
- evidence only — **no** command / execute / dispatch / mutate / permission / approval / grant fields

### `InsightCluster`

Grouped insight opportunity — descriptive coordination unit.

| Field | Role |
|-------|------|
| `cluster_id` | Identifier |
| `label` / `body` | Descriptive framing (never imperative) |
| `evidence_refs` | Related evidence references (**required** — empty ⇒ invalid) |
| `contributing_domains` | Upstream domain tags |
| `confidence` | Diagnostic only (coverage / agreement signals) |
| `completeness` | Cluster-local completeness |
| `attention_rank` | Optional diagnostic ordering metadata (see prioritisation rules) |
| `uncertainty` | Explicit unknowns |
| `authority_effect` | `"none"` |
| `actionable` | `false` |

### `EvidenceIntersection`

Cross-product overlap summary with provenance lineage.

| Field | Role |
|-------|------|
| `intersection_id` | Identifier |
| `source_refs` | Source artefact references (**required**) |
| `shared_themes` | Descriptive theme tags / labels |
| `overlap_explanation` | Evidence-backed overlap narrative |
| `provenance` | Lineage: intersection → sources → revisions → origin domains |
| `authority_effect` | `"none"` |
| `actionable` | `false` |

Allowed relationship language (meaning-only): `relates_to`, `overlaps`, `associated_with`,
`observed_with`, `shares_evidence`, `references`, `derived_from`, `supported_by`.

Forbidden: `causes`, `requires`, `should_execute`, `authorises`, `triggers`,
`leads_to_action`, `approve`, `recommend_action`.

### `InsightGap`

Unresolved-area detection — Missing remains Missing.

| Field | Role |
|-------|------|
| `gap_id` | Identifier |
| `missing_evidence` | What is absent / insufficient |
| `affected_domains` | Domains impacted |
| `uncertainty_explanation` | Why the area remains unresolved |
| `severity` | Diagnostic severity label (not an action priority) |
| `evidence_refs` | Supporting refs for the gap claim (may cite absence markers) |
| `authority_effect` | `"none"` |
| `actionable` | `false` |

### `CoordinationAssessment`

Diagnostic coordination health — not a scorecard for “what to do”.

| Field | Role |
|-------|------|
| `assessment_id` | Identifier |
| `coverage` | Diagnostic coverage of requested surfaces |
| `contradictions_detected` | Count / flags of preserved contradictions |
| `unresolved_areas` | Count / refs of open gaps |
| `cluster_count` / `intersection_count` | Inventory diagnostics |
| `uncertainty` / `limitations` | Explicit limits |
| `authority_effect` | `"none"` |
| `actionable` | `false` |

### Completeness states

| State | Meaning |
|-------|---------|
| `Complete` | Requested surfaces available with usable evidence |
| `Partial` | Some surfaces available; gaps recorded |
| `Unknown` | Completeness cannot be determined |
| `Contradictory` | Upstream surfaces disagree; conflicts preserved |
| `Unavailable` | Required upstream evidence could not be loaded |

## Diagnostic prioritisation (bounded)

Batch 9 may attach **diagnostic attention metadata** so humans can scan crowded evidence.

### Allowed ordering signals

- evidence density / contributing-domain count
- gap severity / unresolved-area count
- contradiction presence
- source availability / freshness metadata
- matching / overlap confidence (diagnostic)

### Forbidden ordering semantics

| Forbidden label | Why |
|-----------------|-----|
| “recommended” / “do this next” | Recommendation Engine territory |
| “best action” / “preferred outcome” | Action authority |
| “most correct” / “definitely true” | Truth / SoT elevation |
| “approve” / “execute” / “priority work item” | Permission / execution / task ownership |

**prioritisation ≠ recommendation.**  
If a UI sorts clusters by `attention_rank`, copy must read as **attention / evidence density**,
never as a work queue or command list.

## Service contract (charter)

### `WorkspaceInsightCoordinationService`

Responsibilities:

- accept an optional coordination frame
- load upstream snapshots via `load_snapshot` only (Batches 1–8 surfaces)
- compose `InsightCoordinationSnapshot` packages
- maintain lineage on every cluster / intersection
- expose dual-channel projections and summaries
- preserve uncertainty, gaps, and contradictions
- persist derived coordination artefacts (read-model)

Must **not**:

- execute / replay / dispatch / restore
- create tasks / plans / recommendations / decisions
- mutate lifecycle / Intent / Memory / Cognitive Model
- mutate Knowledge Synthesis / Knowledge Integration / Contextual Understanding
- grant permissions or call `PermissionGateway`
- approve policies
- call foreign `::generate` to fill gaps
- invent missing evidence or fabricate relationships
- convert correlations into causes
- convert diagnostic prioritisation into action authority
- silently refresh foreign sources
- emit commands

Negative guards (required tests):

- `attempt_execute`
- `attempt_create_task`
- `attempt_mutate_lifecycle`
- `attempt_grant_permissions`
- `attempt_approve_policy`
- `attempt_create_recommendation`
- `attempt_become_memory_or_cognitive_model`
- `attempt_invent_missing_evidence`
- `attempt_convert_correlation_to_causation`
- `attempt_convert_prioritisation_to_action`
- `attempt_silent_refresh`
- `attempt_emit_command`

## Commands (charter)

Through `CommandPipeline` + `PermissionGateway`:

| Command | Kind | Capability | Purpose |
|---------|------|------------|---------|
| `GenerateInsightCoordinationSnapshot` | Mutation | `work_context.write` | Persist a coordination artefact |
| `GetInsightCoordinationSnapshot` | Query | `work_context.read` | Load current dual-channel projection |
| `GetInsightCoordinationSummary` | Query | `work_context.read` | Summary + authoritative `history_count` |
| `ExplainInsightCoordination` | Query | `work_context.read` | Explanation surface over current/last coordination |

All commands:

```
React
  ↓
IPC
  ↓
CommandPipeline
  ↓
PermissionGateway
  ↓
WorkspaceInsightCoordinationService
```

## Persistence (charter)

| Artefact | Role |
|----------|------|
| Migration `060_workspace_insight_coordination.sql` | Read-model tables only |
| Repository `InsightCoordinationRepository` | Persist / retrieve / supersede / history append |

### Allowed

- supersede current coordination
- append-only history
- transactional writes (rollback leaves no partial state)
- derived coordination artefacts / lineage snapshots / cached evidence views

### Must preserve

- append-only history
- provenance
- revision awareness

### Must not introduce

- canonical knowledge / Memory storage
- event sourcing
- actionable history rows
- repository → service calls

Repository remains **persistence only**.

## Projection requirements

Continue the established contract:

- `current`
- `history`
- `history_count` (authoritative)

History:

- evidence only
- `actionable = false`
- `authority_effect = "none"`
- no command / execute / dispatch / mutate / permission / approval / grant fields

## Recovery contract

| Condition | Behaviour |
|-----------|-----------|
| Missing source | Missing |
| Unavailable source | Unavailable |
| Contradiction | Contradiction preserved as evidence |
| Unknown provenance | Unknown |
| Partial evidence | Partial |

Never:

- fill gaps silently
- infer truth
- fabricate relationships / clusters / intersections

Expected helpers:

- `recovery_must_not_fabricate_insight_coordination`
- `recovery_must_not_fabricate_actionable_insight_coordination_history`

## Frontend (charter)

Projection helpers only (e.g. `insightCoordinationProjection.ts`).

UI must communicate:

- evidence
- relationships / intersections
- uncertainty / gaps

Never:

- action buttons
- approval affordances
- execution language
- “recommended next steps” copy derived from this projection

## Governance (expected on implementation)

| Knob | Expected delta |
|------|----------------|
| Mutation command baseline | +1 (`GenerateInsightCoordinationSnapshot`) → **70** |
| History / projection DTO inventory | +1 each → **21** |
| Lifecycle service files | `workspace_insight_coordination.rs` |
| Ownership registry | `insight_coordination` / `insight_coordination_snapshot` |
| Architecture map | refresh via `--write` |
| Guards | import boundary + no foreign `::generate` (incl. Knowledge Integration / Synthesis) + repo≠service |
| IPC public error | `insight_coordination_validation_error` |

Docs on implementation:

- this architecture → Active
- Programme III roadmap (Batch 9 Done; no Batch 10 implied until accepted)
- Projection integrity / Architecture governance / Operational recovery
- Vocabulary: Insight Coordination ≠ planner / decision / recommendation / autonomy / Memory / SoT

Required documentation statement (this charter already states):

- coordination ≠ authority
- prioritisation ≠ recommendation
- intersection ≠ causation
- confidence ≠ permission

## Required tests (acceptance — on implementation)

### Domain

- deterministic coordination from identical upstream refs
- missing upstream → Unavailable / gaps (no invented clusters)
- contradictions preserved without resolution
- provenance lineage present on every cluster / intersection
- meaning-only relationship kinds
- diagnostic prioritisation never promotes to actionable
- history separation / non-actionability
- empty `evidence_refs` rejected for clusters / intersections

### Kernel

- command routing + Gateway enforcement
- persistence rollback leaves no partial write
- negative authority guards (execute / task / lifecycle / permission / approve / recommend / invent / cause)
- restart continuity without fabrication
- recomputation supersedes and separates history

### Frontend / governance

- projection helpers non-commandable
- `history_count` authoritative
- architecture governance + IPC verification green

## Implementation brief (gated)

Implementation must create:

1. Domain module `workspace_insight_coordination` with model above
2. Migration `060` + `InsightCoordinationRepository`
3. `WorkspaceInsightCoordinationService` (`generate` / `load_snapshot` / `explain`)
4. Commands listed above + negative guards
5. Recovery + platform coherence ownership entries
6. Governance baseline / DTO / guards / map refresh
7. React projection helpers + domain TS types
8. Docs → Active after acceptance gates

**Do not implement until this charter is reviewed and accepted.**

## Documentation deliverables (this charter turn)

- This architecture — status **Charter draft**
- `PROGRAMME-III-COHERENT-WORKSPACE-RUNTIME.md` — Batch 9 charter drafted (extension after Batch 8 evidence-stack close)
- Pointers in governance / projection / recovery / vocabulary / README

## Review gate

**Charter only — do not implement until this contract is reviewed and approved.**

Governing rule:

> Coordinate understanding. Never create authority.

> The workspace may understand more. It must never silently gain the power to decide more.

## Related

- [Programme III — Coherent Workspace Runtime](./PROGRAMME-III-COHERENT-WORKSPACE-RUNTIME.md)
- [Knowledge Integration Architecture](./KNOWLEDGE-INTEGRATION-ARCHITECTURE.md) (Batch 8)
- [Knowledge Synthesis Architecture](./KNOWLEDGE-SYNTHESIS-ARCHITECTURE.md) (Batch 7)
- [Contextual Workspace Understanding Architecture](./CONTEXTUAL-WORKSPACE-UNDERSTANDING-ARCHITECTURE.md) (Batch 6)
- [Workspace Explanation Layer Architecture](./WORKSPACE-EXPLANATION-LAYER-ARCHITECTURE.md) (Batch 5)
- [Temporal Intelligence Architecture](./TEMPORAL-INTELLIGENCE-ARCHITECTURE.md) (Batch 4)
- [Historical Workspace Reconstruction Architecture](./HISTORICAL-WORKSPACE-RECONSTRUCTION-ARCHITECTURE.md) (Batch 3)
- [Policy & Governance Architecture](./POLICY-GOVERNANCE-ARCHITECTURE.md) (Batch 2)
- [Unified Workspace State Architecture](./UNIFIED-WORKSPACE-STATE-ARCHITECTURE.md) (Batch 1)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
- [Operational Recovery](../03-Engineering/OPERATIONAL-RECOVERY.md)
