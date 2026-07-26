# Workspace Platform Coherence (Phase 4 Batch 9.5)

| Field | Value |
|-------|-------|
| **Purpose** | Architectural integration — one Workspace platform, not parallel subsystems |
| **Status** | Assessment + coherence fixes |
| **Owner** | Lead Software Engineer |

---

## Permanent rule

```
Human Intent
  → Workspace Intelligence
  → Decision Queue
  → Prepare Intent
  → Command Pipeline
  → Permission Gateway
  → Execution
  → Audit
```

This batch does not introduce execution paths, modify authority, or weaken governance.

**Sprint 136:** Full cognition layer map —
[WORKSPACE-COGNITION-PIPELINE-CONTRACT.md](./WORKSPACE-COGNITION-PIPELINE-CONTRACT.md).
Automation readiness (no autonomy yet) —
[WORKSPACE-AUTOMATION-READINESS.md](./WORKSPACE-AUTOMATION-READINESS.md).

**Sprints 170–175:** Runtime integration of governance via read-only projections —
[WORKSPACE-RUNTIME-CONTEXT.md](./WORKSPACE-RUNTIME-CONTEXT.md).

---

## Assessment summary

### Strengths

- Durable concepts (Workspace, Project, Task, Work Goal, Automation Contract, Trigger Event, Intent Proposal, Permission Approval, Memory, Preferences) each have one SQLite owner.
- Decision Queue and Activity Graph are aggregators with synthetic IDs — not second entity stores.
- Workspace Intelligence and Assistant consume aggregators; they do not own authority.

### Weaknesses (pre–9.5)

- Pending/blocked/planning work was recomputed independently in Intelligence, Decision Queue, and Activity Graph.
- Workspace-scoping helpers were copied in three services (drift risk).
- Work UI stacked batch features and duplicated Intelligence mirrors of Activity / Decisions / Contracts.
- Product metrics mixed `pending_approvals` with Decision Queue counts (Approval ⊂ Decision).
- Dual “recent activity” surfaces (audit lines vs Activity Graph timeline).
- Intelligence inferred current project/task when WorkflowContext had none set.

### Coherence fixes shipped

1. Shared `workspace_scope` helpers for plan/approval attribution.
2. Decision Queue `aggregate_readonly` for nested consumers (no overlay spam from Activity Graph).
3. Intelligence derives pending/blocked/recent from Decision Queue + Activity Graph; current work follows WorkflowContext only.
4. Work tab presents one continuous flow; Decision Queue is the inbox; Activity Graph is the timeline.
5. Canonical [Workspace Vocabulary](WORKSPACE-VOCABULARY.md).

---

## Ownership (exactly one authoritative owner)

| Concept | Owner | Consumers |
|---------|-------|-----------|
| Workspace | `WorkspaceService` / `workspaces` | Projection, Intelligence, UI |
| Project / Task / Work Goal | `WorkspaceIntentService` / `work_*` | Intelligence, Contracts, Activity, UI |
| Automation Contract | `AutomationContractService` | Triggers, Decision Queue, Activity, Intelligence |
| Trigger Event / Intent Proposal | `TriggerEvaluatorService` | Decision Queue, Activity, Intelligence |
| Decision Item | Aggregator only (`DecisionQueueService` + lifecycle overlay) | Intelligence, Activity, Work inbox |
| Activity | Aggregator only (`WorkspaceActivityGraphService`) | Intelligence, Work Activity, Assistant |
| Continuity | Aggregator only (`WorkspaceContinuityService`) | Intelligence, Work Continuity, Assistant |
| Attention | Aggregator only (`WorkspaceAttentionService`) | Intelligence, Work Attention, Assistant |
| Permission Approval | `PermissionApprovalService` | Gateway, Decision Queue, Activity |
| Execution outcome | Audit + `ExecutionOutcomeService` | Activity (attributable), diagnostics |
| Memory / Preferences | AI memory / personalization services | Intelligence (highlights), planning |

---

## Source-of-truth rules

| Product question | Authoritative answer |
|------------------|----------------------|
| What needs my attention? | **Decision Queue** |
| How does work connect over time? | **Activity Graph** |
| What is the current project/task? | **WorkflowContext** (`set_active_work`) |
| Where did I leave off / what changed? | **Continuity Engine** |
| What deserves attention now? | **Attention Engine** |
| What might help next? | **Recommendation Engine** (not Attention tops) |
| What is happening now? | **Operating State** |
| What commonly repeats? | **Pattern Model** |
| How could the Workspace improve? | **Adaptation Proposal** |
| Can I continue working? | **Readiness Model** (≠ WorkspaceHealth) |
| What is happening right now (runtime UI)? | **Workspace Session** (consumes Intelligence) |
| Workspace understanding narrative | **Workspace Intelligence** (consumes the above) |

Intelligence and Assistant must consume, never re-own, these answers. Session consumes Intelligence and feeds UI — never the reverse.

---

## Cognition ownership (Phase 5 → 5.5)

| Concept | Owner | Kind |
|---------|-------|------|
| environment | `WorkspaceEnvironmentService` | Aggregator |
| composition | `WorkspaceCompositionService` | Aggregator |
| workspace_task | `TaskGraphService` | DurableStore |
| purpose | `WorkspacePurposeService` | Aggregator |
| activity | `WorkspaceActivityGraphService` | Aggregator |
| evolution | `WorkspaceEvolutionService` | Aggregator |
| continuity | `WorkspaceContinuityService` | Aggregator |
| decision_item | `DecisionQueueService` | Aggregator (+ lifecycle overlay) |
| attention | `WorkspaceAttentionService` | Aggregator |
| recommendation_candidate | `WorkspaceRecommendationEngineService` | Aggregator |
| operating_state | `WorkspaceOperatingStateService` | Aggregator |
| pattern | `WorkspacePatternService` | Aggregator |
| adaptation_proposal | `WorkspaceAdaptationService` | Aggregator (+ process-local status) |
| readiness | `WorkspaceReadinessService` | Aggregator |
| session | `WorkspaceSessionService` | Aggregator (runtime projection; owns nothing) |
| experience | `WorkspaceExperienceService` | Aggregator (presentation over Session; owns nothing) |
| work_context | `WorkspaceWorkContextService` | Aggregator (semantic kind-of-work; owns nothing) |
| navigation | `WorkspaceNavigationService` | Aggregator (interaction paths; owns nothing) |
| milestones | `WorkspaceMilestoneService` | Aggregator (progress coordination; owns nothing) |
| working_style | `WorkspaceWorkingStyleService` | Aggregator (operating patterns; owns nothing) |
| transition | `WorkspaceTransitionService` | Aggregator (movement explanation; owns nothing) |
| interaction | `WorkspaceInteractionService` | Aggregator (interaction opportunities; owns nothing; select → Intent handoff) |
| workspace_profile | `WorkspaceProfileService` | DurableStore (user-owned setups; comparison informational) |
| decision_candidate | `DecisionEngineService` | Aggregator (+ outcome overlay) |
| Workspace Intelligence | consumer only | Envelope |

### Canonical Intelligence assemble order

```
Decision Queue → Activity → Continuity → Task Graph
→ Environment → Composition → Purpose → Evolution
→ Attention → Recommendation Engine → Operating State → Pattern
→ Readiness → Adaptation → Decision Engine → assemble
→ Session → Experience → Work Context → Navigation → Milestones
→ Working Style → Transition (embed + evidence-only enrich)
→ Interaction (embed after Transition; opportunities only)
→ Profiles (embed after Interaction; durable user-owned setups; comparison only)
```

**Rule:** Prefer Intelligence `generate_with_inputs`. Standalone `generate` is for IPC refresh / diagnostics. Nested Decision Queue consumers must use `aggregate_readonly`. Work Context, Navigation, Milestones, Working Style, Transition, and Interaction never regenerate upstream projections. Profiles are a DurableStore — they persist user-owned data and never own Task Graph, Memory, or permissions.

See [Workspace Cognition Integrity Audit](WORKSPACE-COGNITION-INTEGRITY-AUDIT.md).

---

### Audits (`authority_effect: none`)

- `workspace.intelligence.generated`
- `workspace.coherence.updated` (Batch 9.5 — informational integration signal)

### Related

- [Workspace Vocabulary](WORKSPACE-VOCABULARY.md)
- [Governed Decision Queue](GOVERNED-DECISION-QUEUE.md)
- [Workspace Activity Graph](WORKSPACE-ACTIVITY-GRAPH.md)
- [Workspace Intelligence Foundation](WORKSPACE-INTELLIGENCE-FOUNDATION.md)
- [Workspace Cognition Integrity Audit](WORKSPACE-COGNITION-INTEGRITY-AUDIT.md)
