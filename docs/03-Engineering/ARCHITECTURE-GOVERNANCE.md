# Architecture Governance & System Integrity

| Field | Value |
|-------|-------|
| **Purpose** | Authoritative rules for architectural invariants, capability boundaries, and integrity failure modes |
| **Owner** | Platform Kernel |
| **Status** | Active |
| **Dependencies** | [Permission Architecture](../07-Security/PERMISSION-ARCHITECTURE.md), [Lifecycle Governance](./LIFECYCLE-GOVERNANCE.md), [Persistence Boundary Governance](./PERSISTENCE-BOUNDARY-GOVERNANCE.md), [Projection Integrity](./PROJECTION-INTEGRITY.md), [Governed Audit Durability](./GOVERNED-AUDIT-DURABILITY.md) |
| **Enforcement** | `pnpm verify:architecture-governance` · `tests/architecture-governance.test.ts` · `cargo test -p workspace-kernel governance_failure` |

## Principle

Data correctness is necessary but not sufficient. As the system grows, **every
layer must remain inside its architectural lane**. This document describes those
lanes. Related docs specialize lifecycle, persistence, and projection detail;
they do not override this map.

**Governance tooling is a detector, not an authority.**
`verify:architecture-governance` and the generated map do not own lifecycle,
persistence, or permission decisions. Domain services remain lifecycle owners;
the Permission Gateway remains the sole `require()` decision point; repositories
remain persistence guards. The verifier only fails closed when inventory or
edges cannot be proven compliant.

## Ownership model

| Layer | Owns | Must not own |
|-------|------|--------------|
| **Domain** | Canonical models, transition vocabulary, projection DTO shapes | Persistence, process spawn, permission decisions |
| **Services** | Lifecycle authority and continuity writes (after Allow) | Self-authorization via `PermissionGateway::require`, OS execution, React contracts |
| **Kernel commands** | Dispatch, capability declaration, pipeline entry | Lifecycle truth invention, repository policy |
| **Permission Gateway** | Allow / Deny / ApprovalRequired via `require()` | Execution side effects |
| **Repositories** | Persistence guards, immutable identity fields | Transition choice, service calls |
| **Windows integration** | Process spawn / desktop capture | Capability grants, lifecycle state |
| **React / IPC consumers** | Projection rendering, invoke of registered commands | Database access, lifecycle mutation APIs, fabricated evidence |

Domain services remain lifecycle owners. Repositories enforce stored-state
guards as defense-in-depth. Audit remains append-only evidence.

## Authority boundaries

```
React / IPC / future AI / plugin / automation
        ↓
CommandHandler (actor + intent)
        ↓
CommandPipeline
        ↓
PermissionGateway.require
        ↓
Policy + Gate + allow-once grants
        ↓
Allow | Deny | ApprovalRequired
        ↓
Service (lifecycle owner) → Repository → Domain
        ↓
Execution (only ApplicationLaunchService / windows-integration after Allow)
```

Rules:

1. **Every mutation has a capability path** via `MutationCommand::required_capability`.
2. **Every capability maps to an authority owner** (permission-token scope owner in
   the capability catalog — not a lifecycle ownership transfer).
3. **`PermissionGateway::require` is pipeline/shutdown-only.** Discovery may call
   `PermissionGateway::evaluate` for capability probes without writing permission
   audit records. Services must not call `require`.
4. Lifecycle mutators on sealed services are **`pub(crate)`** — not callable from
   outside the kernel crate. Callers inside the crate are still expected to enter
   via CommandPipeline for privileged mutations; visibility is not a second gateway.
5. Recommendation Engine **must not** spawn processes or call launch integration.

### Capability semantics

Capabilities are **permission tokens**, not domain ownership. Broad tokens such
as `work_context.write` intentionally gate multiple RE / DQ / DE / TaskGraph /
adaptation commands. That does **not** move lifecycle authority to Work Context.
Intentional broad capabilities are listed in
`scripts/generated/architecture-map.json` → `intentional_broad_capabilities`.

Unused catalog ids (e.g. `system.startup` / `system.shutdown` used outside
`MutationCommand`/`QueryCommand` inventory) are reported in the map for review;
they do not auto-fail verification.

## Fail-closed verification

`pnpm verify:architecture-governance` **fails** when:

- required source trees or architecture inputs are missing / empty
- history or projection summary DTOs cannot be located
- mutation inventory drops below the committed baseline
- crate or import forbidden edges are present
- `PermissionGateway::require` appears outside pipeline/shutdown
- the committed architecture map **drifts** from the live canonical inventory

It never treats “nothing found” as clean. Successful runs print explicit evidence,
e.g. “Found 5 history DTOs and verified command-field restrictions.”

Refresh the committed map only after intentional inventory changes:

```bash
pnpm verify:architecture-governance -- --write
```

CI runs `pnpm verify:architecture-governance` (drift-checked) and Vitest governance tests.

Machine inventory: `scripts/generated/architecture-map.json` (must match live
generation; not documentation-only).

## Lifecycle rules

See [Lifecycle Governance](./LIFECYCLE-GOVERNANCE.md). Summary:

- Invalid transitions fail closed; they do not mutate or emit success events.
- Terminal states do not reopen except where an explicit controlled reopen exists
  (e.g. recommendation generation reopen with changed fingerprint).
- Audit-derived projections never become mutable lifecycle owners.

## Projection rules

See [Projection Integrity](./PROJECTION-INTEGRITY.md). Summary:

- Dual channels: actionable current state + historical evidence + authoritative
  `history_count`.
- History **and** projection summary DTOs are scanned for authority fields
  (`execute`, `dispatch`, `mutate`, `command_envelope`, capability grants, …).
- React is projection-only. Action buttons bind only to actionable projections
  via IPC commands.
- Unknown provenance stays unknown. Never invent `native` origin or fill missing
  outcome identities.

## Persistence rules

See [Persistence Boundary Governance](./PERSISTENCE-BOUNDARY-GOVERNANCE.md). Summary:

- Repositories reject illegal transitions and immutable-artifact mutations.
- Cross-domain repository access remains read-only.
- Interrupted migrations roll back schema and ledger — no partial authority tables.
- Production writers call the owning service; repository guards do not transfer
  authority to the database layer.

## Forbidden patterns

| Pattern | Why forbidden |
|---------|---------------|
| React → Database | UI must never open SQLite or import `workspace-database` |
| React → lifecycle mutation APIs | No local authority; mutations go React → IPC → CommandPipeline |
| React → Domain / Kernel crates | Frontend consumes IPC DTOs only |
| Handler mutations without pipeline / `*_inner` pipeline chain | Silent authority downgrade |
| Repositories → Services / Kernel | Persistence must not call lifecycle owners |
| Domain → windows-integration / process spawn | Models are not executors |
| Recommendation Engine → Process Spawn | RE records decisions; launch is a separate gated command |
| Planning Engine → Execution / launch / Gateway.require | Planning is permanently non-executing; mutations via CommandPipeline only |
| Services → `PermissionGateway::require` | Self-authorize bypass; `evaluate` discovery probes remain allowed |
| History/projection DTO gaining authority fields | Evidence must never become executable |
| Projection inventing terminal evidence | No fabricated outcomes on stale/partial recovery |
| Production `AllowAll` / `AlwaysAllow` wiring | Test-only stubs; never on `WorkspaceKernel` |
| Unsafe retry after non-retryable failure | Execution reconciliation owns retry_allowed |

## Planning Engine (Programme II Batch 2)

`WorkspacePlanningService` is the sole planning lifecycle owner. Governance verifies:

- Planning history (`PlanningHistoryEntry`) and summary (`PlanningSummary`) contain no
  authority/command fields.
- Mutation inventory includes `GeneratePlanningSnapshot` (baseline floor raised).
- Services never call `PermissionGateway::require`.
- Planning cannot become an execution or process-launch path.

See [Planning Architecture](../05-AI/PLANNING-ARCHITECTURE.md).

## Reasoning Memory (Programme II Batch 3)

`WorkspaceReasoningMemoryService` is the sole **reasoning evidence** owner — not a
lifecycle, planning, or execution authority. Governance verifies:

| Property | Value |
|----------|-------|
| Lifecycle owner | No |
| Execution authority | No |
| Planning authority | No |
| Evidence owner | Yes |

- `ReasoningHistoryEntry` / `ReasoningSummary` contain no authority/command fields.
- Mutation inventory includes `GenerateReasoningRecord`.
- Reasoning service cannot launch execution or call `PermissionGateway::require`.
- Repository persists only; it does not own reasoning composition logic.
- React cannot mutate reasoning (projection helpers only).

See [Reasoning Memory Architecture](../05-AI/REASONING-MEMORY-ARCHITECTURE.md).

## Cognitive Graph (Programme II Batch 4)

`WorkspaceCognitiveGraphService` owns **topology projections only**.

| Property | Value |
|----------|-------|
| Lifecycle owner | No |
| Execution authority | No |
| Planning authority | No |
| Evidence / topology owner | Yes |

- `CognitiveGraphHistoryEntry` / `CognitiveGraphSummary` contain no authority/command fields.
- Mutation inventory includes `GenerateCognitiveGraph`.
- Graph cannot execute, create tasks/recommendations/decisions, or invent relationships.
- Graph introduces no repository authority over foreign domains.
- Node `external_ref` remains the domain identity — never replaced.

See [Cognitive Graph Architecture](../05-AI/COGNITIVE-GRAPH-ARCHITECTURE.md).

## Cognitive Orchestration (Programme II Batch 5)

`WorkspaceCognitiveOrchestrationService` owns **coordination projections only**.

| Property | Value |
|----------|-------|
| Lifecycle owner | No |
| Execution authority | No |
| Planning authority | No |
| Evidence / coordination owner | Yes |

- `OrchestrationHistoryEntry` / `WorkspaceOrchestrationSummary` contain no authority/command fields.
- Mutation inventory includes `GenerateWorkspaceOrchestration` (baseline floor raised).
- Orchestration cannot execute, mutate Intent/Task, accept recommendations, select decisions, or run refresh plans.
- Orchestration never imports execution/launch services; repository cannot call the service.
- Composition reads existing artefact identities only — never fabricates dependencies.

See [Cognitive Orchestration Architecture](../05-AI/COGNITIVE-ORCHESTRATION-ARCHITECTURE.md).

## Learning & Adaptation (Programme II Batch 6)

`WorkspaceLearningAdaptationService` owns **meta-evidence / adaptation suggestions only**.

| Property | Value |
|----------|-------|
| Lifecycle owner | No |
| Execution authority | No |
| Planning / decision authority | No |
| Evidence / suggestion owner | Yes |

- `LearningHistoryEntry` / `LearningSummary` contain no authority/command fields.
- Mutation inventory includes `GenerateLearningSnapshot` (baseline floor raised).
- Learning cannot import execution/launch services or call foreign `generate` paths.
- Adaptation candidates remain `actionable=false` — never auto-applied.
- Repository cannot call the learning service.

See [Learning & Adaptation Architecture](../05-AI/LEARNING-ADAPTATION-ARCHITECTURE.md).

## Cognitive Agent Cast (Programme II Batch 7)

`WorkspaceCognitiveAgentCastService` owns **role / perspective / critique / synthesis evidence only**.

| Property | Value |
|----------|-------|
| Lifecycle owner | No |
| Execution authority | No |
| Permission owner | No |
| Evidence / coordination owner | Yes |

- `CognitiveAgentCastHistoryEntry` / `CognitiveAgentCastSummary` contain no authority/command fields.
- Mutation inventory includes `GenerateCognitiveAgentCast` (baseline floor raised).
- Agent cast cannot import execution/launch services or mutate lifecycle domains.
- Agents cannot become permission owners or call `PermissionGateway`.
- Repository cannot call the agent cast service.

See [Cognitive Agent Cast Architecture](../05-AI/COGNITIVE-AGENT-CAST-ARCHITECTURE.md).

## Cognitive Autonomy (Programme II Batch 8)

`WorkspaceCognitiveAutonomyService` owns **suggestion / opportunity / safety evidence only**.

| Property | Value |
|----------|-------|
| Lifecycle owner | No |
| Execution authority | No |
| Permission owner | No |
| Evidence / suggestion owner | Yes |

- `CognitiveAutonomyHistoryEntry` / `CognitiveAutonomySummary` contain no authority/command fields.
- Mutation inventory includes `GenerateCognitiveAutonomy` (baseline floor raised).
- Autonomy cannot import execution/launch services or call foreign `generate` paths.
- Autonomy cannot own permissions or call `PermissionGateway`.
- Opportunities always require approval; proposals never carry executable payloads.
- Confidence must never become authority.
- Repository cannot call the autonomy service.

See [Cognitive Autonomy Architecture](../05-AI/COGNITIVE-AUTONOMY-ARCHITECTURE.md).

## Unified Workspace State (Programme III Batch 1)

`WorkspaceStateCompositionService` owns **composition envelope evidence only**.

| Property | Value |
|----------|-------|
| Lifecycle owner | No |
| Execution authority | No |
| Permission / policy owner | No |
| Source of truth | No |
| Composition owner | Yes |

- `WorkspaceStateHistoryEntry` / `WorkspaceStateSummary` contain no authority/command fields.
- Mutation inventory includes `GenerateWorkspaceStateEnvelope` (baseline floor raised).
- Envelope cannot import execution/launch services or call foreign `generate` mutation paths.
- Envelope cannot own permissions or call `PermissionGateway`.
- Missing sources remain Unavailable — never assumed current.
- Repository cannot call the composition service.

See [Unified Workspace State Architecture](../05-AI/UNIFIED-WORKSPACE-STATE-ARCHITECTURE.md).

## Policy & Governance Engine (Programme III Batch 2)

`PolicyGovernanceService` owns **policy evaluation evidence only**.

| Property | Value |
|----------|-------|
| Lifecycle owner | No |
| Execution authority | No |
| Permission owner | No — Gateway remains final |
| Policy reasoning / evidence owner | Yes |

- `PolicyGovernanceHistoryEntry` / `PolicyGovernanceSummary` contain no authority/command fields.
- Mutation inventory includes `GenerateGovernanceEvaluation` (baseline floor raised).
- Policy cannot import lifecycle/execution services or call `PermissionGateway`.
- Policy outputs never contain capability grants.
- Unknown / missing context fails closed — never assumed Compliant.
- Repository cannot call the policy service.

See [Policy & Governance Architecture](../05-AI/POLICY-GOVERNANCE-ARCHITECTURE.md).

## Historical Workspace Reconstruction (Programme III Batch 3)

`WorkspaceHistoricalReconstructionService` owns **reconstruction explanation evidence only**.

| Property | Value |
|----------|-------|
| Lifecycle owner | No |
| Execution / replay authority | No |
| Source of truth | No |
| Temporal comparison / explanation owner | Yes |

- `HistoricalReconstructionHistoryEntry` / `HistoricalReconstructionSummary` contain no authority/command fields.
- Mutation inventory includes `GenerateHistoricalWorkspaceView` (baseline floor raised).
- Reconstruction cannot import lifecycle/execution services or call `PermissionGateway`.
- Reconstruction cannot silently refresh foreign sources via `generate`.
- Missing evidence stays Unknown / gapped — never invented transitions.
- Repository cannot call the reconstruction service.

See [Historical Workspace Reconstruction Architecture](../05-AI/HISTORICAL-WORKSPACE-RECONSTRUCTION-ARCHITECTURE.md).

## Temporal Intelligence (Programme III Batch 4)

`WorkspaceTemporalIntelligenceService` owns **temporal analysis / understanding evidence only**.

| Property | Value |
|----------|-------|
| Lifecycle owner | No |
| Execution / replay / simulation / forecast authority | No |
| Source of truth | No |
| Temporal analysis / projection owner | Yes |

- `TemporalIntelligenceHistoryEntry` / `TemporalIntelligenceSummary` contain no authority/command fields.
- Mutation inventory includes `GenerateTemporalAnalysis` (baseline floor raised).
- Temporal intelligence cannot import lifecycle/execution services or call `PermissionGateway`.
- Temporal intelligence cannot silently refresh foreign sources via `generate`.
- Observed sequence ≠ cause. Evidence quality ≠ correctness. Conflicts explained, not resolved.
- Repository cannot call the temporal intelligence service.

See [Temporal Intelligence Architecture](../05-AI/TEMPORAL-INTELLIGENCE-ARCHITECTURE.md).

## Workspace Explanation Layer (Programme III Batch 5)

`WorkspaceExplanationService` owns **cross-surface explanation evidence only**.

| Property | Value |
|----------|-------|
| Lifecycle owner | No |
| Execution / approve / policy / task authority | No |
| Source of truth | No |
| Evidence synthesis / projection owner | Yes |

- `WorkspaceExplanationHistoryEntry` / `WorkspaceExplanationSummary` contain no authority/command fields.
- Mutation inventory includes `GenerateWorkspaceExplanation` (baseline floor raised).
- Explanation layer cannot import lifecycle/execution services or call `PermissionGateway`.
- Explanation layer cannot silently refresh foreign sources via `generate`.
- Conflicts explained, not resolved. Confidence is diagnostic only.
- Repository cannot call the explanation service.

See [Workspace Explanation Layer Architecture](../05-AI/WORKSPACE-EXPLANATION-LAYER-ARCHITECTURE.md).

## Contextual Workspace Understanding (Programme III Batch 6)

`WorkspaceContextualUnderstandingService` owns **situational understanding evidence only**.

| Property | Value |
|----------|-------|
| Lifecycle owner | No |
| Execution / approve / policy / task / plan authority | No |
| Simulation / forecast / correction authority | No |
| Source of truth | No |
| Situational understanding / projection owner | Yes |

- `ContextualUnderstandingHistoryEntry` / `ContextualUnderstandingSummary` /
  `ContextualUnderstandingProjection` contain no authority/command fields.
- Mutation inventory includes `GenerateContextualWorkspaceUnderstanding` (baseline floor raised).
- Contextual understanding cannot import lifecycle/execution/planning services or call `PermissionGateway`.
- Contextual understanding cannot silently refresh foreign sources via `generate`
  (state / policy / reconstruction / temporal / explanation are `load_snapshot` only).
- Gaps and conflicts framed, not resolved. Confidence is diagnostic only.
- Repository cannot call the contextual understanding service.

See [Contextual Workspace Understanding Architecture](../05-AI/CONTEXTUAL-WORKSPACE-UNDERSTANDING-ARCHITECTURE.md).

## Workspace Knowledge Synthesis (Programme III Batch 7)

`WorkspaceKnowledgeSynthesisService` owns **synthesized knowledge artefacts only**
(concepts / clusters / relationships / gaps).

| Property | Value |
|----------|-------|
| Lifecycle owner | No |
| Execution / approve / policy / task / plan authority | No |
| Memory / Cognitive Model replacement | No |
| Source of truth | No |
| Knowledge synthesis / projection owner | Yes |

- `KnowledgeSynthesisHistoryEntry` / `KnowledgeSynthesisSummary` /
  `KnowledgeSynthesisProjection` contain no authority/command fields.
- Mutation inventory includes `GenerateWorkspaceKnowledgeSynthesis` (baseline floor raised).
- Knowledge synthesis cannot import lifecycle/execution/planning services or call `PermissionGateway`.
- Knowledge synthesis cannot silently refresh foreign sources via `generate`
  (state / policy / reconstruction / temporal / explanation / contextual are `load_snapshot` only).
- Concepts require evidence refs; relationships meaning-only (≠ causation);
  confidence diagnostic only (≠ authority). Derived knowledge ≠ truth.
- Repository cannot call the knowledge synthesis service.

See [Knowledge Synthesis Architecture](../05-AI/KNOWLEDGE-SYNTHESIS-ARCHITECTURE.md).

## Workspace Knowledge Integration (Programme III Batch 8)

`WorkspaceKnowledgeIntegrationService` owns **integration / retrieval artefacts only**
(`KnowledgeIntegrationResult` / `KnowledgeEvidenceLink` / `IntegrationGap`).
Integration ≠ authority; retrieval ≠ truth; relevance ≠ correctness; confidence ≠ permission.

| Property | Value |
|----------|-------|
| Lifecycle owner | No |
| Execution / approve / policy / task / plan / autonomous authority | No |
| Memory / Cognitive Model / Knowledge Synthesis replacement | No |
| Source of truth | No |
| Knowledge integration / retrieval projection owner | Yes |

Enforced:

- `KnowledgeIntegrationHistoryEntry` / `KnowledgeIntegrationSummary` contain no authority/command fields
- Mutation inventory includes `GenerateWorkspaceKnowledgeIntegration` (baseline 69)
- Knowledge integration cannot import lifecycle/execution services or call `PermissionGateway`
- Knowledge integration cannot silently refresh foreign sources via `generate`
  (state / policy / reconstruction / temporal / explanation / contextual / knowledge synthesis are `load_snapshot` only)
- Links require evidence refs; join kinds meaning-only; confidence diagnostic only
- Repository cannot call the knowledge integration service

See [Knowledge Integration Architecture](../05-AI/KNOWLEDGE-INTEGRATION-ARCHITECTURE.md).

## Workspace Insight Coordination (Programme III Batch 9)

`WorkspaceInsightCoordinationService` will own **coordination artefacts only**
(`InsightCluster` / `EvidenceIntersection` / `InsightGap` / `CoordinationAssessment`).
| Property | Value |
|----------|-------|
| Lifecycle owner | No |
| Execution / approve / policy / task / plan / autonomous authority | No |
| Planner / Recommendation / Decision / Autonomy ownership | No |
| Memory / Cognitive Model / Knowledge Synthesis / Integration replacement | No |
| Source of truth | No |
| Insight coordination projection owner | Yes |

Enforced:

- `InsightCoordinationHistoryEntry` / `InsightCoordinationSummary` contain no authority/command fields
- Mutation inventory includes `GenerateInsightCoordinationSnapshot` (baseline **70**)
- Insight coordination cannot import lifecycle/execution / Recommendation / Decision services or call `PermissionGateway`
- Insight coordination cannot silently refresh foreign sources via `generate`
- Clusters / intersections require evidence refs; relationships meaning-only; prioritisation diagnostic only
- Repository cannot call the insight coordination service

See [Insight Coordination Architecture](../05-AI/INSIGHT-COORDINATION-ARCHITECTURE.md).

## Cross-Workspace Intelligence (Programme III Batch 10)

`WorkspaceCrossIntelligenceService` owns **cross-workspace aggregate artefacts only**.
Aggregation ≠ authority; statistics ≠ recommendations.

| Property | Value |
|----------|-------|
| Lifecycle owner | No |
| Execution / recommend / decide / autonomy | No |
| Per-workspace SoT replacement | No |
| Cross-workspace aggregate projection owner | Yes |

Enforced:

- `CrossWorkspaceIntelligenceHistoryEntry` / `CrossWorkspaceIntelligenceSummary` contain no authority/command fields
- Mutation inventory includes `GenerateCrossWorkspaceIntelligence` (baseline **71**)
- Cannot import lifecycle/execution/recommendation services or call `PermissionGateway`
- Cannot silently refresh foreign sources via `generate`
- Repository cannot call the cross-workspace intelligence service

See [Cross-Workspace Intelligence Architecture](../05-AI/CROSS-WORKSPACE-INTELLIGENCE-ARCHITECTURE.md).

## Workspace Decision Support (Programme III Batch 11)

`WorkspaceDecisionSupportService` owns **decision support artefacts only**.
Support ≠ decision; trade-off ≠ recommendation.

| Property | Value |
|----------|-------|
| Lifecycle owner | No |
| Execution / recommend / decide / autonomy | No |
| Decision Engine / Recommendation replacement | No |
| Decision support projection owner | Yes |

Enforced:

- `DecisionSupportHistoryEntry` / `WorkspaceDecisionSupportSummary` contain no authority/command fields
- Mutation inventory includes `GenerateWorkspaceDecisionSupport` (baseline **72**)
- Cannot import lifecycle/execution/recommendation/decision-engine services or call `PermissionGateway`
- Cannot silently refresh foreign sources via `generate`
- Trade-off summaries descriptive only; forbidden recommendation phrases validated
- Repository cannot call the decision support service

See [Workspace Decision Support Architecture](../05-AI/WORKSPACE-DECISION-SUPPORT-ARCHITECTURE.md).

## Workspace Intelligence Hub (Programme III Batch 12)

`WorkspaceIntelligenceHubService` owns **intelligence hub artefacts only**.
Aggregation ≠ reinterpretation; conflict record ≠ resolution.

| Property | Value |
|----------|-------|
| Lifecycle owner | No |
| Execution / recommend / decide / autonomy | No |
| Upstream intelligence replacement | No |
| Intelligence hub projection owner | Yes |

Enforced:

- `IntelligenceHubHistoryEntry` / `WorkspaceIntelligenceHubSummary` contain no authority/command fields
- Mutation inventory includes `GenerateWorkspaceIntelligenceHub` (baseline **73**; superseded by Programme IV Batch 1 → **74**)
- Cannot import lifecycle/execution/recommendation/decision-engine services or call `PermissionGateway`
- Cannot silently refresh foreign sources via `generate` (including Decision Support / Insight / Cross-Workspace)
- Conflict records preserved — never resolved by hub
- Repository cannot call the intelligence hub service

See [Workspace Intelligence Hub Architecture](../05-AI/WORKSPACE-INTELLIGENCE-HUB-ARCHITECTURE.md).

## Workspace Semantic Query Engine (Programme IV Batch 1)

Canonical read-only semantic retrieval over Programmes II and III.

**Retrieve meaning. Never create meaning.**

- Owns retrieval / provenance / completeness / diagnostics only
- Never owns reasoning, planning, recommendation, execution, lifecycle, or policy
- Upstream access via `load_snapshot` only — never foreign `::generate`
- History is evidence-only (`actionable: false`, `authority_effect: "none"`)
- Mutation inventory includes `GenerateWorkspaceSemanticQuery` (baseline **74**)
- History / projection DTO inventory length **25**

See [Workspace Semantic Query Architecture](../05-AI/WORKSPACE-SEMANTIC-QUERY-ARCHITECTURE.md).

## Workspace Evidence Navigation Engine (Programme IV Batch 2)

Deterministic navigation of existing evidence paths.

**Navigate evidence. Never interpret evidence.**

- Owns paths / sessions / summaries / lineage / gaps only
- Never owns semantic retrieval, reasoning, recommendation, execution, lifecycle, or policy
- Upstream access via `load_snapshot` only — never foreign `::generate`
- Never infers relationships or bridges missing lineage
- History is evidence-only (`actionable: false`, `authority_effect: "none"`)
- Mutation inventory includes `GenerateWorkspaceEvidenceNavigation` (baseline **75**)
- History / projection DTO inventory length **26**

See [Workspace Evidence Navigation Architecture](../05-AI/WORKSPACE-EVIDENCE-NAVIGATION-ARCHITECTURE.md).

## Workspace Evidence Trace Engine (Programme IV Batch 3)

Deterministic provenance tracing for a single artefact.

**Trace provenance. Never infer provenance.**

- Owns traces / chains / segments / diagnostics / gaps only
- Never owns navigation, semantic retrieval, reasoning, recommendation, execution, or lifecycle
- Upstream access via `load_snapshot` only — never foreign `::generate`
- Never invents hops or bridges missing history
- History is evidence-only (`actionable: false`, `authority_effect: "none"`)
- Mutation inventory includes `GenerateWorkspaceEvidenceTrace` (baseline **76**)
- History / projection DTO inventory length **27**

See [Workspace Evidence Trace Architecture](../05-AI/WORKSPACE-EVIDENCE-TRACE-ARCHITECTURE.md).

## Workspace Evidence Coverage Engine (Programme IV Batch 4)

Observable evidence completeness for a subject.

**Measure evidence coverage. Never measure truth.**

- Owns coverage snapshots / metrics / gaps / diagnostics / completeness only
- Never owns semantic retrieval, navigation, provenance, reasoning, recommendation, execution, or lifecycle
- Upstream access via `load_snapshot` only — never foreign `::generate`
- Never infers missing evidence, fabricates completeness, or evaluates truth
- History is evidence-only (`actionable: false`, `authority_effect: "none"`)
- Mutation inventory includes `GenerateWorkspaceEvidenceCoverage` (baseline **77**)
- History / projection DTO inventory length **28**

See [Workspace Evidence Coverage Architecture](../05-AI/WORKSPACE-EVIDENCE-COVERAGE-ARCHITECTURE.md).

## Workspace Evidence Consistency Engine (Programme IV Batch 5)

Observable agreement and disagreement across recorded evidence.

**Observe consistency. Never resolve consistency.**

- Owns consistency snapshots / observations / conflicts / gaps / diagnostics only
- Never owns retrieval, navigation, provenance, coverage, reasoning, recommendation, execution, or lifecycle
- Upstream access via `load_snapshot` only — never foreign `::generate`
- Never resolves conflicts, fabricates agreement/disagreement, or determines truth
- History is evidence-only (`actionable: false`, `authority_effect: "none"`)
- Mutation inventory includes `GenerateWorkspaceEvidenceConsistency` (baseline **78**)
- History / projection DTO inventory length **29**

See [Workspace Evidence Consistency Architecture](../05-AI/WORKSPACE-EVIDENCE-CONSISTENCY-ARCHITECTURE.md).

## Workspace Evidence Dependency Engine (Programme IV Batch 6)

Recorded dependency structure across evidence artefacts.

**Observe dependency. Never create dependency.**

- Owns dependency snapshots / graphs / relationships / gaps / diagnostics only
- Never owns retrieval, navigation, provenance, coverage, consistency, reasoning, recommendation, execution, or lifecycle
- Upstream access via `load_snapshot` only — never foreign `::generate`
- Never infers dependencies, repairs links, creates workflows, or establishes causation
- History is evidence-only (`actionable: false`, `authority_effect: "none"`)
- Mutation inventory includes `GenerateWorkspaceEvidenceDependency` (baseline **79**)
- History / projection DTO inventory length **30**

See [Workspace Evidence Dependency Architecture](../05-AI/WORKSPACE-EVIDENCE-DEPENDENCY-ARCHITECTURE.md).

## Workspace Evidence Freshness Engine (Programme IV Batch 7)

Observable freshness of available evidence.

**Observe freshness. Never refresh evidence.**

- Owns freshness snapshots / observations / gaps / diagnostics / summaries only
- Never owns retrieval, navigation, provenance, coverage, consistency, dependency, reasoning, recommendation, execution, or lifecycle
- Upstream access via `load_snapshot` only — never foreign `::generate`
- Never refreshes evidence, regenerates snapshots, estimates freshness, fabricates timestamps, or schedules updates
- History is evidence-only (`actionable: false`, `authority_effect: "none"`)
- Mutation inventory includes `GenerateWorkspaceEvidenceFreshness` (baseline **80**)
- History / projection DTO inventory length **31**

See [Workspace Evidence Freshness Architecture](../05-AI/WORKSPACE-EVIDENCE-FRESHNESS-ARCHITECTURE.md).

## Workspace Evidence Completeness Engine (Programme IV Batch 8)

Observable completeness of available evidence and recorded omissions.

**Observe completeness. Never complete evidence.**

- Owns completeness snapshots / observations / gaps / diagnostics / summaries only
- Never owns retrieval, navigation, provenance, coverage, consistency, dependency, freshness, reasoning, recommendation, execution, or lifecycle
- Upstream access via `load_snapshot` only — never foreign `::generate`
- Never fills gaps, repairs incomplete evidence, fabricates completeness, or estimates missing information
- History is evidence-only (`actionable: false`, `authority_effect: "none"`)
- Mutation inventory includes `GenerateWorkspaceEvidenceCompleteness` (baseline **81**)
- History / projection DTO inventory length **32**

See [Workspace Evidence Completeness Architecture](../05-AI/WORKSPACE-EVIDENCE-COMPLETENESS-ARCHITECTURE.md).

## Workspace Evidence Reliability Engine (Programme IV Batch 9)

Observable reliability characteristics across recorded evidence.

**Observe reliability. Never establish truth.**

- Owns reliability snapshots / observations / gaps / diagnostics / summaries only
- Never owns retrieval, navigation, provenance, coverage, consistency, dependency, freshness, completeness, reasoning, execution, or lifecycle
- Upstream access via `load_snapshot` only — never foreign `::generate`
- Never settles conflicts, ranks by opinion, repairs evidence, fabricates reliability, or grants trust authority
- History is evidence-only (`actionable: false`, `authority_effect: "none"`)
- Mutation inventory includes `GenerateWorkspaceEvidenceReliability` (baseline **82**)
- History / projection DTO inventory length **33**

See [Workspace Evidence Reliability Architecture](../05-AI/WORKSPACE-EVIDENCE-RELIABILITY-ARCHITECTURE.md).

## Workspace Evidence Observational Scaffold (Programme IV Batch 10)

Shared Programme IV helpers — not a new evidence assessment engine.

**Share contracts. Do not collapse ownership.**

- Owns shared domain helpers, data-driven evidence-family governance specs, shared React projection contract
- Never owns evidence snapshots, migrations, mutations, or classification semantics
- No new Generate command; mutation baseline remains **82**; DTO inventory remains **33**
- Engines remain authoritative for their DTOs

See [Workspace Evidence Observational Scaffold Architecture](../05-AI/WORKSPACE-EVIDENCE-OBSERVATIONAL-SCAFFOLD-ARCHITECTURE.md).

## Conversational / Assistant Surface (Programme IV Batch 11)

Human-facing conversation surface over recorded Programme II–IV evidence.

**Present intelligence. Never become authority.**

- Owns conversation turn projections, assistant utterance packages, presentation lineage, surface diagnostics only
- Never owns decisions, approvals, execution, Intent, Decision Engine, Recommendation systems, or hidden memory
- Upstream reads via `load_snapshot` only — never foreign `::generate` for refresh
- Mutation inventory includes `ComposeWorkspaceAssistantTurn` (baseline **83**)
- History / projection DTO inventory length **34**

See [Conversational / Assistant Surface Architecture](../05-AI/CONVERSATIONAL-ASSISTANT-SURFACE-ARCHITECTURE.md).

## Assistant Context Intelligence (Programme IV Batch 12)

Context selection and conversation continuity packaging without memory or planning authority.

**Select and package context. Never invent or own it.**

- Owns conversation context packaging, active session context, retrieval scope, displayed context selection only
- Never owns durable memory, workspace truth, Intent, decisions, plans, or autonomous goals
- Reuses Batch 10/11 contracts (`AssistantSurfaceScope`, evidence contract helpers, thin React wrappers) — no second assistant subsystem clone
- Upstream reads via `load_snapshot` / existing queries only — never foreign `::generate` for refresh
- Continuity from `WorkspaceAssistantSurfaceService::load_snapshot` only — never invents turns
- Mutation inventory includes `PackageWorkspaceAssistantContext` (baseline **84**)
- History / projection DTO inventory length **35**

See [Assistant Context Intelligence Architecture](../05-AI/ASSISTANT-CONTEXT-INTELLIGENCE-ARCHITECTURE.md).

## Assistant Retrieval Intelligence (Programme IV Batch 13)

Retrieval request packaging and evidence presentation without ranking, reasoning, or recommendation authority.

**Present retrieved evidence. Never rank truth or recommend action.**

- Owns retrieval request packaging, query/context translation for presentation, evidence selection presentation, retrieval diagnostics, provenance display only
- Never owns truth ranking, relevance authority, reasoning, recommendations, decisions, memory, policy, or execution
- Composes Semantic Query, Evidence engines, and Batch 12 context — no second search engine, hidden ranker, or independent knowledge graph
- Upstream reads via `load_snapshot` / existing queries only — never foreign `::generate` for refresh
- Presentation order is stable by `artefact_ref` — never “best” / “most relevant” authority
- Mutation inventory includes `PackageWorkspaceAssistantRetrieval` (baseline **85**)
- History / projection DTO inventory length **36**

See [Assistant Retrieval Intelligence Architecture](../05-AI/ASSISTANT-RETRIEVAL-INTELLIGENCE-ARCHITECTURE.md).

## Assistant Explanation Intelligence (Programme IV Batch 14)

Explanation packaging and citation clarity without conclusion, causal, or recommendation authority.

**Clarify recorded evidence. Never conclude what it means.**

- Owns explanation packaging, evidence citation formatting, explanation structure, visible gaps/conflicts, user-facing clarity only
- Never owns conclusions, truth determination, causal reasoning, recommendations, decisions, policy interpretation, or autonomous analysis
- Composes Programme III Explanation Layer, Evidence Trace/Navigation, and Batches 11–13 — no second reasoning engine or explanation SoT
- Upstream reads via `load_snapshot` / existing queries only — never foreign `::generate` for refresh
- Mutation inventory includes `PackageWorkspaceAssistantExplanation` (baseline **86** at Batch 14; superseded by Batch 15 baseline **87**)
- History / projection DTO inventory length **37** at Batch 14 (Batch 15 → **38**)

See [Assistant Explanation Intelligence Architecture](../05-AI/ASSISTANT-EXPLANATION-INTELLIGENCE-ARCHITECTURE.md).

## Assistant Interaction Intelligence (Programme IV Batch 15)

Conversation flow packaging and response routing without memory, Intent, decision, or autonomous agency.

**Coordinate interaction flow. Never act for the user.**

- Owns conversation state packaging, interaction session structure, user-visible flow coordination, response composition routing, interaction diagnostics only
- Never owns memory, identity, Intent authority, decisions, planning, recommendations, execution, or autonomous behaviour
- Composes Batches 11–14 — no cognitive engine, hidden memory, agent loop, or decision layer
- Upstream reads via `load_snapshot` / existing queries only — never foreign `::generate` for refresh
- Mutation inventory includes `PackageWorkspaceAssistantInteraction` (baseline **87** at Batch 15; superseded by Batch 16 baseline **88**)
- History / projection DTO inventory length **38** at Batch 15 (Batch 16 → **39**)

See [Assistant Interaction Intelligence Architecture](../05-AI/ASSISTANT-INTERACTION-INTELLIGENCE-ARCHITECTURE.md).

## Assistant Personalisation Boundary (Programme IV Batch 16)

Explicit presentation preference packaging without hidden user modelling or autonomous adaptation.

**Adapt presentation from explicit preferences. Never invent who the user is.**

- Owns presentation preference packaging, explicit preference display, interface adaptation metadata, interaction style configuration, personalisation diagnostics only
- Never owns hidden user modelling, personality inference, identity, memory, behavioural prediction, psychological profiling, autonomous adaptation, or decision-making
- Composes AI Personalization Foundation and Batches 11–15 — no user-model engine, preference inference system, or hidden profile database
- Preference writes remain existing authorised preference owners only (`personalization.write`)
- Packaging mutation uses `work_context.write` for dual-channel assistant evidence — does **not** write `user_preferences`
- Upstream reads via `AiPersonalizationService` read paths + Batches 11–15 `load_snapshot` only — never foreign `::generate` for refresh
- Mutation inventory includes `PackageWorkspaceAssistantPersonalisation` (baseline **88**)
- History / projection DTO inventory length **39**

See [Assistant Personalisation Boundary Architecture](../05-AI/ASSISTANT-PERSONALISATION-BOUNDARY-ARCHITECTURE.md).

## Capability boundary audit

Enforced by `scripts/architecture-governance-lib.mjs`:

1. Require expected trees/files (fail closed if missing).
2. Enumerate every `impl MutationCommand` (baseline floor).
3. Resolve `required_capability()` to a catalog id and authority owner.
4. Confirm owner consistency; report unused catalog ids.
5. Confirm `PermissionGateway::require` call sites are only
   `commands/pipeline.rs` and shutdown in `commands/handler.rs`.
6. Confirm services never call `PermissionGateway::require`.

## Failure-mode verification

Rust tests in `packages/kernel/src/commands/governance_failure_tests.rs` prove:

| Failure | Required behaviour |
|---------|-------------------|
| Database lock unavailable / poisoned | Error — no Allow |
| Empty capability set | Deny / ApprovalRequired — no silent Allow |
| Corrupted / forged history JSON | Non-commandable; missing evidence identity fails closed |
| Interrupted migration | Migration error; schema + ledger rolled back |
| Stale AllowAll stubs | Not wired on production kernel |
| Partial claim / missing projection data | No fabricated history; no guessed terminal; no unsafe retry |
| Verifier missing targets / empty inventory / map drift | Vitest + `verify:architecture-governance` fail closed |

## Related automation

| Check | Command |
|-------|---------|
| Architecture governance (incl. map drift) | `pnpm verify:architecture-governance` |
| Refresh committed map | `pnpm verify:architecture-governance -- --write` |
| IPC contracts | `pnpm verify:ipc-contract` |
| UI ↔ Experience boundary | `pnpm verify:ui-experience-boundary` |
| Projection integrity (Vitest) | `pnpm test` (`projection-integrity.test.ts`) |
| Domain projection contract | `cargo test -p workspace-domain projection_contract` |
| Governance failure modes | `cargo test -p workspace-kernel governance_failure` |
