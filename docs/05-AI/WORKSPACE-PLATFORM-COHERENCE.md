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
| Workspace understanding narrative | **Workspace Intelligence** (consumes the above) |

Intelligence and Assistant must consume, never re-own, these answers.

---

### Audits (`authority_effect: none`)

- `workspace.intelligence.generated`
- `workspace.coherence.updated` (Batch 9.5 — informational integration signal)

### Related

- [Workspace Vocabulary](WORKSPACE-VOCABULARY.md)
- [Governed Decision Queue](GOVERNED-DECISION-QUEUE.md)
- [Workspace Activity Graph](WORKSPACE-ACTIVITY-GRAPH.md)
- [Workspace Intelligence Foundation](WORKSPACE-INTELLIGENCE-FOUNDATION.md)
