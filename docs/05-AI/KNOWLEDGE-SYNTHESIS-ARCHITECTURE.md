# Workspace Knowledge Synthesis Architecture (Programme III — Batch 7)

| Field | Value |
|-------|-------|
| **Purpose** | Create a provenance-bound synthesis layer that turns accumulated workspace evidence into structured knowledge representations — without becoming memory truth, decision authority, planner, or executor |
| **Owner** | `WorkspaceKnowledgeSynthesisService` (DurableStore — **synthesized knowledge artefacts only**) |
| **Status** | Active — Programme III Batch 7 implemented |
| **Lifecycle owner** | No |
| **Execution / replay authority** | No |
| **Simulation / forecast / correction authority** | No |
| **Decision / policy / permission authority** | No |
| **Memory / Cognitive Model replacement** | No |
| **Source of truth** | No |

## Core principle

**Synthesize understanding from evidence. Do not create reality.**

```
Unified Workspace State
Policy Governance
Historical Reconstruction
Temporal Intelligence
Workspace Explanation Layer
Contextual Workspace Understanding
        │
        ▼
Workspace Knowledge Synthesis
        │
        ▼
Structured knowledge artefacts (read-only)
        │
        ▼
Human or Gateway-authorised decision (elsewhere)
```

Not:

```
Knowledge Synthesis
        │
        ▼
memory truth / decide / plan / mutate / invent facts
```

## Architectural position

Programme III stack after Batch 7:

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
Structured knowledge artefacts (read-only)
```

Batch 7 **extends** Batch 6 situational framing into durable, provenance-bound
**knowledge representations** (concepts, clusters, relationships, gaps).

It does **not** replace:

- Contextual Understanding (situational themes / current picture)
- Explanation Layer (cross-surface explanation packages)
- Reasoning Memory (hypotheses / reasoning evolution)
- Cognitive Model (durable semantic workspace structure)
- Memory systems (stored experience / retrieval)

### Distinction from Batch 6 (P2 discipline)

Batch 6 audit P2 warned against additional interpretive layers that **duplicate**
Contextual Understanding. Batch 7 is intentionally a **different responsibility**:

| Layer | Question it answers |
|-------|---------------------|
| Contextual Understanding (Batch 6) | What is the **situational picture now**? |
| Knowledge Synthesis (Batch 7) | What **structured concepts / clusters / relationships** can be derived from accumulated evidence? |

Batch 7 must not re-emit situational themes as “knowledge” without evidence lineage,
and must not claim to be the current situational authority.

It answers:

> Given durable Programme III evidence available now (and what is missing /
> conflicting), what structured knowledge concepts, clusters, and relationships
> can be derived — and how strong is that derivation as diagnostic confidence,
> not as memory truth?

It does **not** answer:

- what the workspace “really is” as a second SoT
- what should be remembered as canonical Memory
- what the Cognitive Model should become
- what will happen next / what would happen if
- what the system should do now
- unsupported facts when evidence is missing

## Why this batch

Batch 6 acceptance opened the Batch 7 gate. Direction:

> Workspace Knowledge Synthesis Layer — provenance-bound synthesis of accumulated
> evidence into structured knowledge representations.

Constraints remain absolute:

- no source-of-truth elevation
- no Memory / Cognitive Model replacement
- no unsupported fact invention
- no mutation of upstream systems
- no decisions / execution
- no silent evidence refresh
- no correlation-as-causation

Batch 8 (Knowledge Integration / Retrieval Intelligence) remains gated on
Batch 7 audit ACCEPT.

## Locked constraints (non-negotiable)

Do **not** introduce:

| Forbidden | Why |
|-----------|-----|
| Source of truth elevation | Synthesized artefacts are derived — never authoritative reality |
| Memory replacement | Memory systems retain stored experience / retrieval |
| Cognitive Model replacement | Cognitive Model retains durable semantic structure |
| Reasoning Memory replacement | Hypotheses / reasoning evolution stay owned elsewhere |
| Event sourcing | Competing authoritative log — still rejected |
| Authoritative replay | Replay must never mutate or re-decide |
| Simulation / forecasting | Futures and counterfactuals are not evidence |
| Autonomous correction | Synthesis ≠ mutation authority |
| Decision ownership | Gateway / humans remain decision authorities |
| Planner / task creation | Knowledge is not a planning engine |
| Policy re-decision | Policy evaluations may be referenced, never re-authorised |
| Causal invention | `causes` / unsupported “because” remain forbidden |
| Conflict resolution | Preserve disagreement; never declare a winner |
| Foreign silent `generate` | No gap-filling by regenerating upstream authorities |
| Opaque concept invention | Every concept needs evidence lineage |
| Confidence → authority | Diagnostic confidence ≠ approval / execution / policy / priority |
| Correlation as causation | `relates_to` / `overlaps` ≠ `causes` / `requires_action` |

## Ownership

### Knowledge Synthesis owns

- synthesized knowledge artefacts / views
- `KnowledgeConcept` derivations (evidence-bound)
- `KnowledgeCluster` groupings
- `KnowledgeRelationship` meaning-only links
- evidence lineage / provenance aggregation (references only — revision-bound)
- diagnostic `KnowledgeConfidence` from evidence quality
- unresolved `KnowledgeGap` records
- knowledge synthesis history (append-only evidence)

### Knowledge Synthesis does **not** own

| System | Authority retained |
|--------|--------------------|
| Workspace State | Current composed reality |
| Cognitive Model | Durable semantic workspace structure |
| Reasoning Memory | Hypotheses and reasoning evolution |
| Historical Reconstruction | Past evidence interpretation |
| Temporal Intelligence | Time-based organisation |
| Explanation Layer | Situation explanations |
| Contextual Understanding | Situational themes |
| Memory systems | Stored experience / retrieval |
| Policy Engine | Governance decisions |
| CommandPipeline | Mutation authority |
| PermissionGateway | Final authorisation |
| Lifecycle / Task Graph / Intent | Mutation / execution ownership |

## Relationship to prior batches

| Prior owner | Knowledge Synthesis may |
|-------------|-------------------------|
| Unified Workspace State | Reference via `load_snapshot` only |
| Policy Governance | Reference via `load_snapshot` only — never re-evaluate to invent Compliant |
| Historical Reconstruction | Reference via `load_snapshot` only |
| Temporal Intelligence | Reference via `load_snapshot` only |
| Explanation Layer | Reference via `load_snapshot` only |
| Contextual Understanding | Reference via `load_snapshot` only — never replace situational authority |

| Prior owner | Knowledge Synthesis must not |
|-------------|------------------------------|
| Any upstream | Call `::generate` to fill gaps |
| Memory / Cognitive Model / Reasoning Memory | Mutate, replace, or silently rewrite |
| Any lifecycle / planning service | Mutate / transition / create tasks |
| Policy / Gateway | Grant, deny, bypass, or decide |

### Carried-forward disciplines (Batches 4–6)

1. **Confidence remains diagnostic** — never truth certainty, approval, execution, policy, or priority authority.
2. **Cross-surface composition must keep explicit provenance:**  
   statement → evidence references → source revision → origin domain.
3. **Persisted synthesis artefacts are revision-bound caches** — never “previous synthesis = current truth / memory”.
4. **Conflicts and gaps are preserved, not resolved.** Correlation ≠ causation.
5. **Theme / concept taxonomies stay descriptive** — no advice, prioritisation, or hidden recommendations.

## Core model

### `WorkspaceKnowledgeSynthesis` (artefact)

Primary composed artefact for a synthesis request — **read-only analytical artefact**.

| Field | Role |
|-------|------|
| `synthesis_id` | Identity of this synthesis artefact |
| `workspace_id` | Workspace scope |
| `generated_at` | Wall-clock composition time |
| `status` / `superseded_at` | Active vs superseded lifecycle of the artefact |
| `frame` / `scope` | What evidence surfaces / bounds were requested |
| `source_revisions` | Revision-bound upstream refs used for this compose |
| `concepts` | Ordered `KnowledgeConcept` list |
| `clusters` | Ordered `KnowledgeCluster` list |
| `relationships` | Ordered `KnowledgeRelationship` list |
| `gaps` | Explicit `KnowledgeGap` records |
| `confidence` | Diagnostic `KnowledgeConfidence` |
| `completeness` | Explicit completeness state |
| `provenance_links` | Links to upstream durable artefacts (revision-bound) |
| `narrative` / `summary` | Evidence-backed synthesis narrative (optional) |
| `limitations` | Explicit limitation statements |
| `authority_effect` | Always `"none"` |
| `actionable` | Always `false` |
| `terminal` | Terminal when superseded into history |

Not a Memory write. Not a Cognitive Model mutation. Not a decision package. Not a plan.

### `KnowledgeConcept`

A derived concept — must reference evidence; cannot invent entities.

| Field | Role |
|-------|------|
| `id` | Identity |
| `label` | Short descriptive label |
| `description` | Evidence-backed description |
| `evidence_refs` | Upstream durable refs (required — empty ⇒ invalid) |
| `confidence` | Diagnostic strength from evidence quality |
| `uncertainty` | Explicit unknowns |
| `provenance` | Lineage: concept → evidence → revision → origin domain |
| `authority_effect` | `"none"` |
| `actionable` | `false` |

### `KnowledgeCluster`

Groups related concepts. Examples:

- recurring workspace patterns
- operational themes
- historical themes
- capability themes

| Field | Role |
|-------|------|
| `cluster_id` | Identity |
| `kind` | Cluster category (descriptive) |
| `label` / `description` | Human-readable framing |
| `concept_ids` | Member concepts |
| `evidence_refs` | Supporting upstream refs |
| `authority_effect` | `"none"` |
| `actionable` | `false` |

Must reference evidence. Cannot invent entities.

### `KnowledgeRelationship`

Meaning-only connection between concepts / clusters / evidence.

| Allowed kinds (examples) | Forbidden kinds |
|--------------------------|-----------------|
| `relates_to` | `causes` |
| `reinforces` | `requires_action` |
| `overlaps` | `should_execute` |
| `depends_on_evidence` | `approve` / `deny` / `dispatch` |

| Field | Role |
|-------|------|
| `relationship_id` | Identity |
| `kind` | Meaning-only link type |
| `from_ref` / `to_ref` | Concept / cluster / evidence anchors |
| `evidence_refs` | Supporting upstream refs |
| `uncertainty` | Explicit limits on the link |
| `authority_effect` | `"none"` |
| `actionable` | `false` |

### `KnowledgeGap`

Explicit absence — Unknown remains unknown.

| Examples | Never become |
|----------|--------------|
| insufficient evidence | inferred concept |
| conflicting evidence | silently resolved winner |
| unavailable source | assumed current |

| Field | Role |
|-------|------|
| `gap_id` | Identity |
| `surface` | Which upstream / theme is incomplete |
| `description` | Why knowledge is missing / conflicting |
| `severity` | Gap pressure signal |
| `evidence_refs` | Upstream refs when any |
| `authority_effect` | `"none"` |
| `actionable` | `false` |

### `KnowledgeConfidence`

Diagnostic only — derived from evidence quality / coverage.

It **cannot** authorize:

- action
- policy outcome
- automation
- priority

Maintain:

```
Confidence of synthesis
        ≠
Confidence of truth / memory / approval / execution
```

### Completeness states

Explicit uncertainty model — **never collapse**:

| State | Meaning |
|-------|---------|
| `Complete` | Requested surfaces available with usable evidence for synthesis |
| `Partial` | Some surfaces available; gaps recorded |
| `Unknown` | Completeness cannot be determined |
| `Contradictory` | Upstream surfaces disagree; conflicts preserved |
| `Unavailable` | Required upstream evidence could not be loaded |

**A complete-looking knowledge graph is worse than an explicit gap.**

## Synthesis principle

### Allowed

```
State snapshot (load_snapshot)
+ Policy evaluation (load_snapshot)
+ Reconstruction (load_snapshot)
+ Temporal analysis (load_snapshot)
+ Explanation package (load_snapshot)
+ Contextual understanding (load_snapshot)
= knowledge synthesis artefact with concepts/clusters/relationships/gaps/provenance
```

Same durable upstream revisions + same frame ⇒ deterministic synthesis content
(`concepts`, `clusters`, `relationships`, `gaps`, `completeness`).
Identity / `generated_at` may differ.

### Forbidden

```
Missing upstream
+ preferred ontology / “likely fact” / correlation story
= invented knowledge certainty
```

Language / lineage rules:

| Allowed | Forbidden without evidence |
|---------|----------------------------|
| “Concept X is supported by explanation E and theme T.” | “Concept X exists because the user intended …” |
| “Clusters A and B overlap on evidence refs …” | “A causes B.” |
| “Sources disagree; gap preserved.” | “Source A was wrong.” |
| “Synthesis coverage is partial (diagnostic).” | “High confidence ⇒ safe to act / remember as truth.” |
| “depends_on_evidence” | “requires_action” / “should_execute” |

Every concept / cluster / relationship claim must carry:

```
Knowledge statement
        ↓
Evidence references
        ↓
Source revision / artefact id
        ↓
Origin domain
```

No opaque generated ontologies without evidence lineage.
No fabricated concepts without evidence.
No treating correlation as causation.

## Evidence inputs (read-only)

Knowledge Synthesis **reads** durable upstream artefacts; it never generates/refreshes
foreign authorities as a side effect of synthesis.

| Input | Access pattern |
|-------|----------------|
| Workspace state envelope snapshot | `load_snapshot` only — never `generate` |
| Policy governance snapshot | `load_snapshot` only — never `generate` |
| Historical reconstruction snapshot | `load_snapshot` only — never `generate` |
| Temporal intelligence snapshot | `load_snapshot` only — never `generate` |
| Workspace explanation snapshot | `load_snapshot` only — never `generate` |
| Contextual understanding projection | `load_snapshot` only — never `generate` |

**Forbidden input patterns:**

- direct repository reads from other domains
- lifecycle reads bypassing envelopes
- generating upstream state
- mutating Memory / Cognitive Model / Reasoning Memory

**No hidden source scraping. No lifecycle inspection shortcuts. No silent regeneration.**

## Service contract

### `WorkspaceKnowledgeSynthesisService`

Responsibilities:

- accept a synthesis frame / scope
- load requested upstream snapshots via `load_snapshot` only
- derive concepts, clusters, relationships, gaps
- attach provenance lineage to every concept / claim
- compute diagnostic confidence from evidence quality
- persist synthesis artefacts (read-model)

Must **not** (forbidden behaviours):

- execute / replay / dispatch / restore
- simulate / forecast / auto-correct
- mutate sources, Memory, Cognitive Model, or Reasoning Memory
- create tasks / plans / approvals
- call foreign `generate` paths to fill gaps
- grant permissions or call `PermissionGateway`
- invent concepts without evidence refs
- repair contradictions by dropping a side
- convert confidence into authority (action / policy / automation / priority)
- treat correlation as causation
- treat prior synthesis cache as current truth / memory without revision binding

Negative guards (required tests):

- `attempt_execute`
- `attempt_create_task`
- `attempt_approve`
- `attempt_mutate_intent`
- `attempt_modify_cognitive_model`
- `attempt_alter_memory`
- `attempt_repair_contradictions`
- `attempt_convert_confidence_to_authority`
- `attempt_fabricate_concepts_without_evidence`
- `attempt_treat_correlation_as_causation`
- `attempt_silent_refresh`
- `attempt_emit_command`

## Commands

| Command | Kind | Capability | Purpose |
|---------|------|------------|---------|
| `GenerateWorkspaceKnowledgeSynthesis` | Mutation | `work_context.write` | Persist a synthesis artefact for a frame |
| `GetWorkspaceKnowledgeSynthesis` | Query | `work_context.read` | Load current synthesis snapshot |
| `GetWorkspaceKnowledgeSummary` | Query | `work_context.read` | Summary + authoritative `history_count` |
| `ExplainKnowledgeSynthesis` | Query | `work_context.read` | Explanation surface over current/last synthesis |

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
WorkspaceKnowledgeSynthesisService
```

No repair / apply / replay / remember-as-truth / decide / retrieve-as-SoT commands.
(Retrieval intelligence is Batch 8 — only after this boundary is accepted.)

## Persistence

| Artefact | Role |
|----------|------|
| Migration `058_workspace_knowledge_synthesis.sql` | Read-model tables only |
| Repository `WorkspaceKnowledgeSynthesisRepository` | Persist / retrieve / supersede / history append |

### Allowed

- supersede current synthesis
- append historical synthesis evidence
- transactional supersede + insert
- rollback leaves no partial synthesis state

### Forbidden

- deleting history
- editing previous synthesis
- rewriting evidence lineage
- lifecycle tables
- source duplication
- foreign authority storage

Repository does **not** evaluate, repair, replay, simulate, forecast, decide,
resolve conflicts, mutate Memory/Cognitive Model, or mutate sources.

Persisted artefacts must remain:

- revision-bound
- reproducible
- evidence-linked

They must **not** become:

```
previous synthesis = current truth / memory
```

Repository remains a **knowledge synthesis evidence cache**, not a truth or control database.

## Projection contract

Dual-channel pattern (required):

| Channel | Contents | Actionable? |
|---------|----------|-------------|
| `current` | Active `WorkspaceKnowledgeSynthesis` view | View / inspect only |
| `history` | Superseded synthesis via history entries (append-only) | Never |
| `history_count` | Full terminal count | Scalar authority |

History remains:

- evidence only
- immutable
- non-actionable
- no commands / actions
- no lifecycle controls
- `authority_effect = "none"`

DTO names:

- `KnowledgeSynthesisHistoryEntry`
- `KnowledgeSynthesisProjection` / snapshot dual-channel
- `KnowledgeSynthesisSummary`

## Recovery contract

| Scenario | Result |
|----------|--------|
| Missing evidence | `missing` → `missing` / gap — never inferred |
| Unavailable source | `Unavailable` — not assumed current |
| Conflicting evidence | Conflict preserved — not resolved |
| Insufficient evidence | `KnowledgeGap` — not invented concept |
| Missing provenance | Unknown — never opaque certainty |
| Corrupt / unusable inputs | Synthesis failure evidence — not auto-repaired |
| Restart | Same durable evidence + same frame ⇒ same synthesis content |
| Fabrication attempt | Fail closed — never invent concepts, relationships, confidence, or historical claims |

Recovery **cannot** create:

- concepts
- relationships
- confidence
- historical claims

Recovery helper (expected):

- `recovery_must_not_fabricate_knowledge_synthesis`
- `recovery_must_not_fabricate_actionable_knowledge_synthesis_history`

## Governance

| Knob | Delta |
|------|-------|
| Mutation command baseline | +1 (`GenerateWorkspaceKnowledgeSynthesis`) → **68** |
| History / projection DTO inventory | +1 each → **19** |
| Lifecycle service files | `workspace_knowledge_synthesis.rs` |
| Ownership registry | `knowledge_synthesis` / `knowledge_synthesis_snapshot` |
| Architecture map | refresh via `--write` |
| Guards | import boundary + no foreign `::generate` + repo≠service |
| IPC public error | `knowledge_synthesis_validation_error` |

Docs:

- this architecture → Active
- Programme III roadmap
- Projection integrity
- Architecture governance
- Operational recovery
- Vocabulary

## Required tests (acceptance)

### Domain

- deterministic synthesis from identical frame + upstream refs
- missing upstream → Unavailable / gaps (no invented concepts)
- conflicts preserved without resolution
- provenance lineage present on every concept
- relationships meaning-only (no causation / action kinds)
- confidence never promoted to truth / authority
- history separation / non-actionability
- prior cache ≠ current truth without revision binding
- empty `evidence_refs` rejected for concepts

### Kernel

- command routing + Gateway enforcement
- persistence rollback (no partial synthesis write)
- all forbidden behaviour negative guards
- restart continuity of durable synthesis evidence
- frame bounds respected
- no Memory / Cognitive Model mutation paths

### Projection

- DTO non-commandability
- history separation
- `history_count` authority
- synthesis separation from commands

### Governance

- import restrictions
- architecture map refresh
- mutation baseline + DTO inventory updates

### Recovery

- missing evidence does not become inferred knowledge
- restart preserves evidence without fabricating synthesis

## Documentation deliverables

- This architecture — status **Active — Programme III Batch 7 implemented**
- `PROGRAMME-III-COHERENT-WORKSPACE-RUNTIME.md` — Batch 7 implemented; Batch 8 gated on Batch 7 audit ACCEPT
- Pointers in `ARCHITECTURE-GOVERNANCE.md`, `PROJECTION-INTEGRITY.md`, `OPERATIONAL-RECOVERY.md`
- Vocabulary: Knowledge Synthesis ≠ Memory / Cognitive Model / SoT / decision authority
- `docs/README.md` index entry

## Batch 7 acceptance criteria

Shipped when:

1. Knowledge Synthesis exists as evidence-derived artefacts only
2. No new authority boundary / duplicate source of truth
3. No mutation ownership (Memory / Cognitive Model / Intent / Task / State untouched)
4. Deterministic synthesis from same revisions + same frame
5. Provenance trace exists for every concept
6. Unknown / conflict / gap states preserved
7. Confidence remains diagnostic only
8. Relationships remain meaning-only (no causation / action)
9. Upstream read boundaries (`load_snapshot` only) remain absolute
10. Projection integrity and Gateway/Pipeline boundaries remain absolute
11. Governance detects knowledge-synthesis boundary violations
12. Recovery never fabricates concepts / relationships / confidence / historical claims

Explicit invariants (unchanged):

- **Derived knowledge ≠ truth** — synthesized artefacts are never source of truth or Memory
- **Relationships ≠ causation** — meaning-only links; never `causes` / action kinds
- **Confidence ≠ authority** — diagnostic only; never approval / execution / policy / priority

## Review gate

**Charter accepted; Batch 7 implementation shipped.** Further work proceeds only against this contract —
no Memory / Cognitive Model replacement, no SoT elevation, no autonomous correction, no decision ownership.

Do not start Batch 8 (Workspace Knowledge Integration / Retrieval Intelligence)
until Batch 7 is audited and accepted (Batch 8 gated on Batch 7 audit ACCEPT).

Later candidates remain under the same rule: synthesize understanding from evidence;
do not create reality.

## Related

- [Programme III — Coherent Workspace Runtime](./PROGRAMME-III-COHERENT-WORKSPACE-RUNTIME.md)
- [Contextual Workspace Understanding Architecture](./CONTEXTUAL-WORKSPACE-UNDERSTANDING-ARCHITECTURE.md) (Batch 6)
- [Workspace Explanation Layer Architecture](./WORKSPACE-EXPLANATION-LAYER-ARCHITECTURE.md) (Batch 5)
- [Temporal Intelligence Architecture](./TEMPORAL-INTELLIGENCE-ARCHITECTURE.md) (Batch 4)
- [Historical Workspace Reconstruction Architecture](./HISTORICAL-WORKSPACE-RECONSTRUCTION-ARCHITECTURE.md) (Batch 3)
- [Policy & Governance Architecture](./POLICY-GOVERNANCE-ARCHITECTURE.md) (Batch 2)
- [Unified Workspace State Architecture](./UNIFIED-WORKSPACE-STATE-ARCHITECTURE.md) (Batch 1)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
- [Operational Recovery](../03-Engineering/OPERATIONAL-RECOVERY.md)
