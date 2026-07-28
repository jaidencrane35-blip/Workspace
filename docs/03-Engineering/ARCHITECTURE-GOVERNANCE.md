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
