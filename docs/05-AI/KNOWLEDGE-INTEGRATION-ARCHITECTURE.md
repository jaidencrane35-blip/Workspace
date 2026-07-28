# Workspace Knowledge Integration / Retrieval Intelligence Architecture (Programme III — Batch 8)

| Field | Value |
|-------|-------|
| **Purpose** | Integrate and retrieve across accumulated Programme III evidence layers into provenance-bound retrieval views — without becoming a new source of truth, Memory system, Cognitive Model, decision authority, planner, or executor |
| **Owner** | `WorkspaceKnowledgeIntegrationService` (DurableStore — **integration / retrieval artefacts only**) |
| **Status** | Charter draft — pending review before implementation |
| **Lifecycle owner** | No |
| **Execution / replay authority** | No |
| **Simulation / forecast / correction authority** | No |
| **Decision / policy / permission authority** | No |
| **Memory / Cognitive Model / Knowledge Synthesis replacement** | No |
| **Source of truth** | No |
| **Autonomous authority** | No |

## Core principle

**Retrieve and integrate evidence. Do not invent truth. Do not decide what happens next.**

```
Unified Workspace State
Policy Governance
Historical Reconstruction
Temporal Intelligence
Workspace Explanation Layer
Contextual Workspace Understanding
Workspace Knowledge Synthesis
        │
        ▼
Workspace Knowledge Integration / Retrieval
        │
        ▼
Integrated retrieval views (read-only)
        │
        ▼
Human or Gateway-authorised decision (elsewhere)
```

Not:

```
Knowledge Integration
        │
        ▼
new SoT / Memory / decide / execute / auto-act
```

## Architectural position

Programme III stack after Batch 8 (proposed — final Programme III capability batch):

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
Integrated retrieval artefacts (read-only)
```

Batch 8 **closes** Programme III’s coherent evidence stack by providing a controlled
**integration and retrieval** surface over layers already accepted — not a new ontology,
not a second Memory, not an autonomous agent.

### Distinction from prior batches (non-duplication)

| Layer | Question it answers |
|-------|---------------------|
| Contextual Understanding (Batch 6) | What is the **situational picture now**? |
| Knowledge Synthesis (Batch 7) | What **structured concepts / clusters / relationships** can be derived? |
| Knowledge Integration / Retrieval (Batch 8) | Given a retrieval frame, **what integrated evidence/knowledge slice** can be assembled — with lineage preserved? |

Batch 8 must not:

- re-derive concepts as if it owned Knowledge Synthesis
- re-frame situational themes as if it owned Contextual Understanding
- store experience as if it were Memory
- edit workspace semantics as if it were Cognitive Model

It answers:

> Given durable Programme III artefacts available now (and what is missing /
> conflicting), what integrated retrieval view can be assembled for a stated query
> frame — with every hit bound to evidence lineage — without claiming truth or
> authority?

It does **not** answer:

- what is “really true” as a second SoT
- what Memory should retain
- what the Cognitive Model should become
- what will happen next / what would happen if
- what the system should do now
- what to invent when evidence is missing

## Why this batch

Batch 7 acceptance opened the Batch 8 gate. Direction from architecture audit:

> Draft Programme III Batch 8 charter only before implementation.
> Focus on integration/retrieval intelligence over the accumulated evidence layers,
> while preserving: no new source of truth, no autonomous authority, no execution,
> no replacement of existing cognitive systems.

Constraints remain absolute:

- no new source of truth
- no autonomous authority
- no execution
- no replacement of Memory / Cognitive Model / Reasoning Memory / Knowledge Synthesis
- no silent evidence refresh
- no fabricated retrieval hits
- no conversion of retrieval confidence into action / policy / priority

Programme III is expected to close after Batch 8 is implemented and audited ACCEPT.
Further work (if any) requires a new programme charter.

## Locked constraints (non-negotiable)

Do **not** introduce:

| Forbidden | Why |
|-----------|-----|
| New source of truth | Integration views are derived projections over existing authorities |
| Memory replacement | Memory retains stored experience / retrieval ownership |
| Cognitive Model replacement | Cognitive Model retains durable semantic structure |
| Knowledge Synthesis replacement | Concepts / clusters remain Batch 7’s derivation authority |
| Reasoning Memory replacement | Hypotheses / reasoning evolution stay owned elsewhere |
| Autonomous authority | No self-acting agent, no hidden workflow owner |
| Event sourcing | Competing authoritative log — still rejected |
| Authoritative replay | Replay must never mutate or re-decide |
| Simulation / forecasting | Futures and counterfactuals are not evidence |
| Execution / task creation | Retrieval is not a planner or executor |
| Policy re-decision | Policy may be referenced, never re-authorised |
| Causal invention | Unsupported “because” / causes remain forbidden |
| Silent foreign `generate` | No gap-filling by regenerating upstream authorities |
| Opaque ranked “answers” | Every retrieval hit needs evidence lineage |
| Confidence → authority | Diagnostic retrieval confidence ≠ approval / execution / priority |

## Ownership

### Knowledge Integration owns

- integration / retrieval packages / views
- retrieval frames (what is being queried / scoped)
- integrated hit lists bound to upstream evidence / knowledge artefacts
- retrieval completeness and uncertainty roll-ups
- provenance aggregation for each hit (references only — revision-bound)
- integration history (append-only evidence)
- diagnostic retrieval confidence

### Knowledge Integration does **not** own

| System | Authority retained |
|--------|--------------------|
| Workspace State | Current composed reality |
| Cognitive Model | Durable semantic workspace structure |
| Reasoning Memory | Hypotheses and reasoning evolution |
| Memory systems | Stored experience / retrieval |
| Knowledge Synthesis | Derived concepts / clusters / relationships |
| Contextual Understanding | Situational themes |
| Explanation / Temporal / Reconstruction / Policy | Their respective evidence domains |
| CommandPipeline | Mutation authority |
| PermissionGateway | Final authorisation |
| Lifecycle / Task Graph / Intent | Mutation / execution ownership |

## Relationship to prior batches

| Prior owner | Knowledge Integration may |
|-------------|---------------------------|
| Unified Workspace State | Reference via `load_snapshot` only |
| Policy Governance | Reference via `load_snapshot` only |
| Historical Reconstruction | Reference via `load_snapshot` only |
| Temporal Intelligence | Reference via `load_snapshot` only |
| Explanation Layer | Reference via `load_snapshot` only |
| Contextual Understanding | Reference via `load_snapshot` only |
| Knowledge Synthesis | Reference via `load_snapshot` only |

| Prior owner | Knowledge Integration must not |
|-------------|-------------------------------|
| Any upstream | Call `::generate` to fill gaps |
| Memory / Cognitive Model / Reasoning Memory / Knowledge Synthesis | Mutate, replace, or silently rewrite |
| Any lifecycle / planning service | Mutate / transition / create tasks |
| Policy / Gateway | Grant, deny, bypass, or decide |

### Carried-forward disciplines (Batches 4–7)

1. **Confidence remains diagnostic** — never truth, approval, execution, policy, or priority authority.
2. **Every claim carries provenance:** statement → evidence refs → source revision → origin domain.
3. **Persisted integration artefacts are revision-bound caches** — never “previous retrieval = current truth”.
4. **Gaps and conflicts are preserved, not resolved.**
5. **Meaning-only structure** — no causation / action edges invented at retrieval time.
6. **Knowledge Synthesis remains descriptive** — Integration must not promote concepts into editable ontology.

## Core model (charter)

### `WorkspaceKnowledgeIntegration` (artefact)

Primary composed artefact for an integration / retrieval request — **read-only analytical artefact**.

| Field | Role |
|-------|------|
| `integration_id` | Identity of this integration artefact |
| `workspace_id` | Workspace scope |
| `generated_at` | Wall-clock composition time |
| `status` / `superseded_at` | Active vs superseded lifecycle of the artefact |
| `frame` | `KnowledgeRetrievalFrame` describing the query / scope |
| `source_revisions` | Revision-bound upstream refs used for this compose |
| `hits` | Ordered `KnowledgeRetrievalHit` list |
| `integrations` | Optional `KnowledgeIntegrationLink` cross-layer joins |
| `gaps` | Explicit `KnowledgeRetrievalGap` records |
| `confidence` | Diagnostic `KnowledgeRetrievalConfidence` |
| `completeness` | Explicit completeness state |
| `provenance_links` | Links to upstream durable artefacts |
| `summary` / `narrative` | Evidence-backed retrieval narrative |
| `limitations` | Explicit limitation statements |
| `authority_effect` | Always `"none"` |
| `actionable` | Always `false` |
| `terminal` | Terminal when superseded into history |

Not a Memory write. Not a Cognitive Model mutation. Not a decision package. Not a plan.
Not a ranked “do this next” list.

### `KnowledgeRetrievalFrame`

What is being retrieved / integrated.

| Field | Role |
|-------|------|
| `focus` | Optional focus text / theme (descriptive filter, not a command) |
| `include_state` / `include_policy` / `include_reconstruction` / `include_temporal` / `include_explanation` / `include_contextual` / `include_knowledge_synthesis` | Upstream surface selection |
| `max_hits` | Hard bound — never unbounded scrape |
| `as_of_refs` | Optional explicit upstream revision / artefact refs |

Frames constrain reading. They do not invent coverage.

### `KnowledgeRetrievalHit`

One provenance-bound retrieval result.

| Field | Role |
|-------|------|
| `hit_id` | Identity |
| `kind` | Surface / artefact kind (e.g. concept, theme, explanation_section, state_envelope) |
| `label` | Short descriptive label |
| `body` | Evidence-backed excerpt / framing |
| `evidence_refs` | Upstream durable refs (**required** — empty ⇒ invalid) |
| `provenance` | Lineage: hit → evidence → revision → origin domain |
| `uncertainty` | Explicit unknowns |
| `confidence` | Diagnostic coverage for this hit |
| `authority_effect` | `"none"` |
| `actionable` | `false` |

A hit without lineage must not exist as a trusted artefact.

### `KnowledgeIntegrationLink`

Cross-layer join — meaning-only.

| Allowed | Forbidden |
|---------|-----------|
| `relates_to` | `causes` |
| `overlaps` | `requires_action` |
| `associated_with` | `should_execute` |
| `observed_with` | `triggers` |
| `shares_evidence` | `leads_to_action` |

Links explain co-occurrence / shared evidence across layers. They do not create operational dependency.

### `KnowledgeRetrievalGap`

Explicit absence — Unknown remains unknown.

| Examples | Never become |
|----------|--------------|
| insufficient evidence for frame | invented hit |
| unavailable upstream surface | assumed current |
| conflicting upstream signals | silently resolved winner |

### `KnowledgeRetrievalConfidence`

Diagnostic only — derived from evidence availability / coverage for the frame.

It **cannot** authorize:

- action
- policy outcome
- automation
- priority
- “best answer” as truth

Maintain:

```
Confidence of retrieval
        ≠
Confidence of truth / memory / approval / execution
```

### Completeness states

| State | Meaning |
|-------|---------|
| `Complete` | Requested surfaces available with usable evidence for the frame |
| `Partial` | Some surfaces available; gaps recorded |
| `Unknown` | Completeness cannot be determined |
| `Contradictory` | Upstream surfaces disagree; conflicts preserved |
| `Unavailable` | Required upstream evidence could not be loaded |

## Integration / retrieval principle

### Allowed

```
load_snapshot(state)
+ load_snapshot(policy)
+ load_snapshot(reconstruction)
+ load_snapshot(temporal)
+ load_snapshot(explanation)
+ load_snapshot(contextual)
+ load_snapshot(knowledge_synthesis)
= integration artefact with hits/links/gaps/provenance
```

Same durable upstream revisions + same frame ⇒ deterministic integration content
(`hits`, `integrations`, `gaps`, `completeness`). Identity / `generated_at` may differ.

### Forbidden

```
Missing upstream
+ preferred answer / ranking story / “likely truth”
= invented retrieval certainty
```

Language / lineage rules:

| Allowed | Forbidden without evidence |
|---------|----------------------------|
| “Hit H is supported by concept C and theme T.” | “H is true because the user intended …” |
| “Surfaces A and B share evidence refs …” | “A causes B.” |
| “Sources disagree; gap preserved.” | “Source A was wrong.” |
| “Retrieval coverage is partial (diagnostic).” | “High confidence ⇒ safe to act / remember as truth.” |

Every hit must carry:

```
Retrieval statement
        ↓
Evidence references
        ↓
Source revision / artefact id
        ↓
Origin domain
```

## Evidence inputs (read-only)

Knowledge Integration **reads** durable upstream artefacts; it never generates/refreshes
foreign authorities as a side effect of retrieval.

| Input | Access pattern |
|-------|----------------|
| Workspace state envelope | `load_snapshot` only |
| Policy governance | `load_snapshot` only |
| Historical reconstruction | `load_snapshot` only |
| Temporal intelligence | `load_snapshot` only |
| Workspace explanation | `load_snapshot` only |
| Contextual understanding | `load_snapshot` only |
| Knowledge synthesis | `load_snapshot` only |

**Forbidden input patterns:**

- direct repository reads from other domains
- lifecycle reads bypassing envelopes
- generating upstream state / synthesis
- mutating Memory / Cognitive Model / Reasoning Memory / Knowledge Synthesis

## Service contract (charter)

### `WorkspaceKnowledgeIntegrationService`

Responsibilities:

- accept a `KnowledgeRetrievalFrame`
- load requested upstream snapshots via `load_snapshot` only
- assemble hits / integration links / gaps
- attach provenance lineage to every hit
- compute diagnostic retrieval confidence
- persist integration artefacts (read-model)

Must **not** (forbidden behaviours):

- execute / replay / dispatch / restore
- simulate / forecast / auto-correct
- mutate sources, Memory, Cognitive Model, Reasoning Memory, or Knowledge Synthesis
- create tasks / plans / approvals / recommendations
- call foreign `generate` paths to fill gaps
- grant permissions or call `PermissionGateway`
- invent hits without evidence refs
- repair contradictions by dropping a side
- convert confidence into authority
- treat correlation as causation
- treat prior integration cache as current truth without revision binding
- become a second editable ontology

Negative guards (required tests):

- `attempt_execute`
- `attempt_create_task`
- `attempt_approve`
- `attempt_mutate_intent`
- `attempt_modify_cognitive_model`
- `attempt_alter_memory`
- `attempt_mutate_knowledge_synthesis`
- `attempt_repair_contradictions`
- `attempt_convert_confidence_to_authority`
- `attempt_fabricate_hits_without_evidence`
- `attempt_treat_correlation_as_causation`
- `attempt_silent_refresh`
- `attempt_emit_command`

## Commands (charter)

| Command | Kind | Capability | Purpose |
|---------|------|------------|---------|
| `GenerateWorkspaceKnowledgeIntegration` | Mutation | `work_context.write` | Persist an integration artefact for a frame |
| `GetWorkspaceKnowledgeIntegration` | Query | `work_context.read` | Load current integration snapshot |
| `GetWorkspaceKnowledgeIntegrationSummary` | Query | `work_context.read` | Summary + authoritative `history_count` |
| `ExplainKnowledgeIntegration` | Query | `work_context.read` | Explanation surface over current/last integration |

Optional read-oriented companion (same service, still non-authoritative):

| Command | Kind | Capability | Purpose |
|---------|------|------------|---------|
| `RetrieveWorkspaceKnowledge` | Query | `work_context.read` | Frame-scoped retrieval view without requiring a new persist (may return last matching artefact or compose ephemerally **without** inventing evidence) |

If ephemeral compose is allowed for `RetrieveWorkspaceKnowledge`, it must still:

- use `load_snapshot` only
- never invent hits
- never claim authority
- never bypass Gateway

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
WorkspaceKnowledgeIntegrationService
```

## Persistence (charter)

| Artefact | Role |
|----------|------|
| Migration `059_workspace_knowledge_integration.sql` | Read-model tables only |
| Repository `WorkspaceKnowledgeIntegrationRepository` | Persist / retrieve / supersede / history append |

### Allowed

- supersede current integration
- append historical integration evidence
- transactional supersede + insert
- rollback leaves no partial integration state

### Forbidden

- deleting history
- editing previous integration
- rewriting evidence lineage
- lifecycle tables
- source duplication
- foreign authority storage
- Memory / Cognitive Model tables

## Projection contract

| Channel | Contents | Actionable? |
|---------|----------|-------------|
| `current` | Active `WorkspaceKnowledgeIntegration` view | View / inspect only |
| `history` | Superseded integrations via history entries (append-only) | Never |
| `history_count` | Full terminal count | Scalar authority |

History remains evidence-only, immutable, non-actionable, non-commandable,
`authority_effect = "none"`.

Expected DTOs:

- `KnowledgeIntegrationHistoryEntry`
- `KnowledgeIntegrationProjection`
- `KnowledgeIntegrationSummary`

## Recovery contract

| Scenario | Result |
|----------|--------|
| Missing evidence | Gap — never inferred hit |
| Unavailable source | Unavailable — not assumed current |
| Conflicting evidence | Conflict preserved |
| Unknown provenance | Unknown |
| Partial inputs | Partial integration |
| Fabrication attempt | Fail closed |

Recovery **cannot** create:

- hits
- integration links
- confidence
- historical claims

Expected helpers:

- `recovery_must_not_fabricate_knowledge_integration`
- `recovery_must_not_fabricate_actionable_knowledge_integration_history`

## Governance (expected on implementation)

| Knob | Expected delta |
|------|----------------|
| Mutation command baseline | +1 (`GenerateWorkspaceKnowledgeIntegration`) → **69** |
| History / projection DTO inventory | +1 each → **20** |
| Lifecycle service files | `workspace_knowledge_integration.rs` |
| Ownership registry | `knowledge_integration` / `knowledge_integration_snapshot` |
| Architecture map | refresh via `--write` |
| Guards | import boundary + no foreign `::generate` + repo≠service |
| IPC public error | `knowledge_integration_validation_error` |

Docs on implementation:

- this architecture → Active
- Programme III roadmap (Batch 8 Done; programme close note)
- Projection integrity / Architecture governance / Operational recovery
- Vocabulary: Knowledge Integration ≠ Memory / SoT / decision authority / retrieval-as-truth

## Required tests (acceptance — on implementation)

### Domain

- deterministic integration from identical frame + upstream refs
- missing upstream → Unavailable / gaps (no invented hits)
- conflicts preserved without resolution
- provenance lineage present on every hit
- integration links meaning-only (no causation / action kinds)
- confidence never promoted to truth / authority
- history separation / non-actionability
- empty `evidence_refs` rejected for hits

### Kernel

- command routing + Gateway enforcement
- persistence rollback
- all forbidden behaviour negative guards
- restart continuity
- no Memory / Cognitive Model / Knowledge Synthesis mutation paths

### Projection / Governance / Recovery

- DTO non-commandability + `history_count` authority
- import restrictions + baseline / inventory updates
- missing evidence does not become inferred retrieval truth

## Documentation deliverables (this charter turn)

- This architecture — status **Charter draft**
- `PROGRAMME-III-COHERENT-WORKSPACE-RUNTIME.md` — Batch 8 charter drafted
- Pointers in governance / projection / recovery / vocabulary / README

## Batch 8 acceptance criteria (implementation later)

Accepted when:

1. Knowledge Integration exists as evidence-derived retrieval artefacts only
2. No new authority boundary / duplicate source of truth
3. No mutation ownership (Memory / Cognitive Model / Knowledge Synthesis / Intent / Task / State untouched)
4. Deterministic integration from same revisions + same frame
5. Provenance trace exists for every hit
6. Unknown / conflict / gap states preserved
7. Confidence remains diagnostic only
8. Integration links remain meaning-only
9. Upstream read boundaries (`load_snapshot` only) remain absolute
10. Projection integrity and Gateway/Pipeline boundaries remain absolute
11. Governance detects knowledge-integration boundary violations
12. Recovery never fabricates hits / links / confidence / historical claims

## Review gate

**Charter only — do not implement until this contract is reviewed and approved.**

Programme III closes when Batch 8 is implemented, audited, and accepted.
No Batch 9 is implied by this charter.

Governing rule (Programme III close):

> Knowledge may organise and retrieve evidence.
> It may never become the authority that decides what is true or what happens next.

## Related

- [Programme III — Coherent Workspace Runtime](./PROGRAMME-III-COHERENT-WORKSPACE-RUNTIME.md)
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
