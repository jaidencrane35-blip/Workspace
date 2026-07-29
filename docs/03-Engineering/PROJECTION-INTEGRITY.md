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

### Cognitive Graph (Programme II Batch 4)

- **Current:** `CognitiveGraphSnapshot.current` — reference-only nodes/edges.
  Observational topology; never execute / create tasks / invent relationships.
- **History:** superseded snapshots via append-only `cognitive_graph_history` →
  `CognitiveGraphHistoryEntry` (`terminal: true`, `actionable: false`, `authority_effect: none`).
- **`history_count`** is authoritative; summary windows may truncate `history`.
- Nodes use existing `external_ref` identities; broken refs are observable evidence only.
- React may inspect topology / broken references — no mutation or command conversion.
- Restart reconstructs durable current + history; missing graph remains missing.

### Cognitive Orchestration (Programme II Batch 5)

- **Current:** `WorkspaceOrchestrationSnapshot.current` — refresh plan / dependency order /
  staleness observations. Coordination only; never execute refreshes or foreign lifecycles.
- **History:** superseded snapshots via append-only `workspace_orchestration_history` →
  `OrchestrationHistoryEntry` (`terminal: true`, `actionable: false`, `authority_effect: none`).
- **`history_count`** is authoritative; summary windows may truncate `history`.
- Execution links are references only. Cycles emit evidence — never invent resolutions.
- React may inspect orchestration observations — no mutation or command conversion.
- Restart reconstructs durable current + history; missing orchestration remains missing.

### Learning & Adaptation (Programme II Batch 6)

- **Current:** `LearningSnapshot.current` — observations, patterns, confidence updates,
  adaptation candidates. Meta-evidence only; never auto-apply or mutate foreign domains.
- **History:** superseded snapshots via append-only `workspace_learning_history` →
  `LearningHistoryEntry` (`terminal: true`, `actionable: false`, `authority_effect: none`).
- **`history_count`** is authoritative; summary windows may truncate `history`.
- Adaptation candidates are suggestions (`actionable: false`). Missing outcomes stay unknown.
- React may inspect learning evidence — no mutation or command conversion.
- Restart reconstructs durable current + history; missing learning remains missing.

### Cognitive Agent Cast (Programme II Batch 7)

- **Current:** `CognitiveAgentCastSnapshot.current` — agents, perspectives, critiques,
  syntheses. Role representations only; never execute or self-authorise.
- **History:** superseded snapshots via append-only `workspace_cognitive_agent_cast_history` →
  `CognitiveAgentCastHistoryEntry` (`terminal: true`, `actionable: false`, `authority_effect: none`).
- **`history_count`** is authoritative; summary windows may truncate `history`.
- Critiques are informational (cannot reject/block). Syntheses are summaries (not decisions).
- React may inspect cast evidence — no mutation or command conversion.
- Restart reconstructs durable current + history; missing cast remains missing.

### Cognitive Autonomy (Programme II Batch 8)

- **Current:** `CognitiveAutonomySnapshot.current` — opportunities, proposals,
  recommendations, safety assessments. Suggestions only; never execute or self-approve.
- **History:** superseded snapshots via append-only `workspace_cognitive_autonomy_history` →
  `CognitiveAutonomyHistoryEntry` (`terminal: true`, `actionable: false`, `authority_effect: none`).
- **`history_count`** is authoritative; summary windows may truncate `history`.
- Opportunities always require approval. Proposals have no executable command payload.
- Safety assessments are evidence, not enforcement. Confidence ≠ permission.
- React may inspect autonomy evidence — no mutation or command conversion.
- Restart reconstructs durable current + history; missing autonomy remains missing.

### Unified Workspace State Envelope (Programme III Batch 1)

- **Current:** `WorkspaceStateSnapshot.current` — `WorkspaceStateEnvelope` with source
  references, revision, freshness, completeness, consistency, unknowns, contradictions.
- **History:** superseded envelopes via append-only `workspace_state_envelope_history` →
  `WorkspaceStateHistoryEntry` (`terminal: true`, `actionable: false`, `authority_effect: none`).
- **`history_count`** is authoritative; summary windows may truncate `history`.
- Fresh / Stale / Unavailable / Unknown remain distinct. Conflicts are evidence, not repair.
- Envelope ≠ source of truth. React may inspect — no mutation or command conversion.
- Restart reconstructs durable current + history; missing envelope remains missing.
- Missing sources remain Unavailable — never assumed Fresh.

### Policy & Governance (Programme III Batch 2)

- **Current:** `PolicyGovernanceSnapshot.current` — policy definitions used, evaluations,
  recommendation. Advisory evidence only; never grants or executes.
- **History:** superseded evaluations via append-only `workspace_policy_governance_history` →
  `PolicyGovernanceHistoryEntry` (`terminal: true`, `actionable: false`, `authority_effect: none`).
- **`history_count`** is authoritative; summary windows may truncate `history`.
- Results: Compliant / Violation / RequiresReview / Unknown / NotApplicable.
- Unknown context fails closed. Policy ≠ permission. Evaluation ≠ authority.
- `ExplainGovernanceDecision` is explanation-only.
- React may inspect — no mutation or command conversion.
- Restart reconstructs durable current + history; missing evaluation remains missing.

### Historical Reconstruction (Programme III Batch 3)

- **Current:** `HistoricalReconstructionSnapshot.current` — temporal snapshots, change
  timeline, revision comparisons, completeness, evidence gaps. Explanation evidence only;
  never replay or restore authority.
- **History:** superseded reconstructions via append-only
  `workspace_historical_reconstruction_history` → `HistoricalReconstructionHistoryEntry`
  (`terminal: true`, `actionable: false`, `authority_effect: none`).
- **`history_count`** is authoritative; summary windows may truncate `history`.
- Completeness: Complete / Partial / Unknown / Contradictory / Unavailable.
- No evidence found ≠ Nothing happened. Gaps remain gaps.
- `CompareWorkspaceRevisions` / `ExplainHistoricalChange` are explanation-only.
- React may inspect — no mutation, replay, or command conversion.
- Restart reconstructs durable current + history; missing reconstruction remains missing.

### Temporal Intelligence (Programme III Batch 4)

- **Current:** `TemporalIntelligenceSnapshot.current` — windowed analysis, revision-chain
  summary, conflict explanations, diagnostic evidence-quality assessment. Understanding
  projection only; never simulate, forecast, repair, or replay.
- **History:** superseded analyses via append-only
  `workspace_temporal_intelligence_history` → `TemporalIntelligenceHistoryEntry`
  (`terminal: true`, `actionable: false`, `authority_effect: none`).
- **`history_count`** is authoritative; summary windows may truncate `history`.
- Completeness: Complete / Partial / Unknown / Contradictory / Unavailable.
- Observed sequence ≠ cause. Evidence quality ≠ correctness. Conflicts are explained, not resolved.
- `ExplainTemporalChange` is explanation-only.
- React may inspect — no mutation, forecast, or command conversion.
- Restart reconstructs durable current + history; missing analysis remains missing.

### Workspace Explanation Layer (Programme III Batch 5)

- **Current:** `WorkspaceExplanationSnapshot.current` — `ExplanationPackage` with sections,
  gaps, conflicts, diagnostic confidence. Evidence synthesis only; never execute, approve,
  or resolve.
- **History:** superseded packages via append-only `workspace_explanation_history` →
  `WorkspaceExplanationHistoryEntry` (`terminal: true`, `actionable: false`,
  `authority_effect: none`).
- **`history_count`** is authoritative; summary windows may truncate `history`.
- Completeness: Complete / Partial / Unknown / Contradictory / Unavailable.
- `ExplainWorkspaceSituation` is explanation-only.
- React may inspect — no mutation or command conversion.
- Restart reconstructs durable current + history; missing explanation remains missing.

### Contextual Workspace Understanding (Programme III Batch 6)

- **Current:** `ContextualUnderstandingProjection.current` — `ContextualWorkspaceSnapshot`
  with `SituationalTheme` / `ContextualInsight` / `ContextualGap`, diagnostic confidence.
  Situational understanding only; never execute, approve, predict, simulate, or decide.
- **History:** superseded understandings via append-only history →
  `ContextualUnderstandingHistoryEntry` (`terminal: true`, `actionable: false`,
  `authority_effect: none`).
- **`history_count`** is authoritative; summary windows may truncate `history`.
- Completeness: Complete / Partial / Unknown / Contradictory / Unavailable.
- `ExplainWorkspaceContext` is understanding-only.
- React may inspect — no mutation or command conversion.
- Restart reconstructs durable current + history; missing understanding remains missing.

### Workspace Knowledge Synthesis (Programme III Batch 7)

- **Current:** `KnowledgeSynthesisProjection.current` — `WorkspaceKnowledgeSynthesis` with
  `KnowledgeConcept` / `KnowledgeCluster` / `KnowledgeRelationship` / `KnowledgeGap`,
  diagnostic confidence. Evidence-derived knowledge only; never Memory truth, Cognitive
  Model mutation, execute, approve, or decide. Derived knowledge ≠ truth; relationships ≠
  causation; confidence ≠ authority.
- **History:** superseded synthesis via append-only history →
  `KnowledgeSynthesisHistoryEntry` (`terminal: true`, `actionable: false`,
  `authority_effect: none`).
- **`history_count`** is authoritative; summary windows may truncate `history`.
- Completeness: Complete / Partial / Unknown / Contradictory / Unavailable.
- `ExplainKnowledgeSynthesis` is explanation-only over synthesis artefacts.
- React may inspect — no mutation or command conversion.
- Restart reconstructs durable current + history; missing synthesis remains missing
  (never inferred concepts / relationships / confidence).

### Workspace Knowledge Integration (Programme III Batch 8)

- **Current:** `KnowledgeIntegrationProjection.current` — `KnowledgeIntegrationResult` with
  `KnowledgeEvidenceLink` / `IntegrationGap`, diagnostic `KnowledgeRetrievalConfidence`.
  Retrieval / join views only; never Memory truth, second ontology, execute, approve, or decide.
  Integration ≠ authority; retrieval ≠ truth; relevance ≠ correctness; confidence ≠ permission.
- **History:** superseded integrations via append-only history →
  `KnowledgeIntegrationHistoryEntry` (`terminal: true`, `actionable: false`,
  `authority_effect: none`).
- **`history_count`** authoritative; summary windows may truncate `history`.
- Completeness: Complete / Partial / Unknown / Contradictory / Unavailable.
- `ExplainKnowledgeIntegration` / `RetrieveWorkspaceKnowledge` are non-authoritative.
- React may inspect — no mutation or command conversion.
- Restart reconstructs durable current + history; missing integration remains missing
  (never inferred hits / links / confidence).

### Workspace Insight Coordination (Programme III Batch 9)

- **Current:** dual-channel coordination projection with `InsightCluster` /
  `EvidenceIntersection` / `InsightGap` / `CoordinationAssessment`.
  Coordination views only; never Memory truth, planner, recommendation executor,
  decision authority, or autonomy. Coordination ≠ authority; prioritisation ≠
  recommendation; intersection ≠ causation; confidence ≠ permission.
- **History:** superseded coordination via append-only history
  (`terminal: true`, `actionable: false`, `authority_effect: none`).
- **`history_count`** authoritative; summary windows may truncate `history`.
- Completeness: Complete / Partial / Unknown / Contradictory / Unavailable.
- `ExplainInsightCoordination` is non-authoritative.
- React may inspect — no mutation, approval, or execution affordances.
- Restart reconstructs durable current + history; missing coordination remains missing
  (never inferred clusters / intersections / attention ranks).

### Cross-Workspace Intelligence (Programme III Batch 10)

- **Current:** `CrossWorkspaceIntelligenceProjection.current` — patterns / themes / risk signals /
  constraint patterns / gaps. Aggregate observations only; never centralises authority.
- **History:** append-only (`terminal: true`, `actionable: false`, `authority_effect: none`).
- **`history_count`** authoritative.
- React may inspect — no execute / approve / recommend / prioritise / automate affordances.

### Workspace Decision Support (Programme III Batch 11)

- **Current:** `WorkspaceDecisionSupportProjection.current` — contexts / evidence bundles /
  trade-offs / dependencies / gaps. Organises evidence only; never decides.
- **History:** append-only (`terminal: true`, `actionable: false`, `authority_effect: none`).
- **`history_count`** authoritative.
- Trade-off summaries descriptive only — support ≠ decision; trade-off ≠ recommendation.
- React may inspect — no decide / approve / recommend / execute affordances.

### Workspace Intelligence Hub (Programme III Batch 12)

- **Current:** `WorkspaceIntelligenceHubProjection.current` — packages / summary /
  lineage / gaps / conflicts. Aggregates intelligence only; never replaces upstream authority.
- **History:** append-only (`terminal: true`, `actionable: false`, `authority_effect: none`).
- **`history_count`** authoritative.
- Conflict records preserved — aggregation ≠ reinterpretation; conflict record ≠ resolution.
- React may inspect — no decide / approve / recommend / execute / resolve affordances.


### Workspace Semantic Query Engine (Programme IV Batch 1)

- Dual-channel: `current` + `history` + authoritative `history_count`
- History is evidence-only; never actionable
- Projection helpers expose search results, provenance, lineage, evidence refs, completeness, diagnostics
- Forbidden: execute, recommend, approve, mutate, automate
- Relevance is diagnostic only — never ranking-as-authority



### Workspace Evidence Navigation Engine (Programme IV Batch 2)

- Dual-channel: `current` + `history` + authoritative `history_count`
- History is evidence-only; never actionable
- Projection helpers expose evidence paths, traversal trees, lineage views, completeness, diagnostics
- Forbidden: execute, recommend, mutate, approve, automate
- Paths are existing provenance only — never inferred edges



### Workspace Evidence Trace Engine (Programme IV Batch 3)

- Dual-channel: `current` + `history` + authoritative `history_count`
- History is evidence-only; never actionable
- Projection helpers expose provenance chains, segments, lineage, diagnostics, completeness
- Forbidden: execute, recommend, approve, mutate, automate
- Segments are recorded hops only — never inferred

### Workspace Evidence Coverage Engine (Programme IV Batch 4)

- Dual-channel: `current` + `history` + authoritative `history_count`
- History is evidence-only; never actionable
- Projection helpers expose coverage summaries, completeness indicators, evidence counts, gap lists, diagnostics, lineage
- Forbidden: execute, recommend, approve, mutate, automate
- Coverage is observable completeness only — never truth, confidence, or inferred evidence

### Workspace Evidence Consistency Engine (Programme IV Batch 5)

- Dual-channel: `current` + `history` + authoritative `history_count`
- History is evidence-only; never actionable
- Projection helpers expose consistency summaries, comparison tables, conflict lists, diagnostics, lineage, completeness
- Forbidden: execute, recommend, resolve, approve, mutate, automate
- Consistency is observational only — conflicts remain unresolved


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
