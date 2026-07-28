# Reasoning Memory Architecture (Programme II — Batch 3)

| Field | Value |
|-------|-------|
| **Purpose** | Retain how the workspace arrived at conclusions — reusable reasoning evidence |
| **Owner** | `WorkspaceReasoningMemoryService` (DurableStore — **evidence owner only**) |
| **Status** | Active |
| **Lifecycle owner** | No |
| **Execution authority** | No |
| **Planning authority** | No |

## What this is

Reasoning Memory stores **how** conclusions were reached: hypothesis, assumptions,
alternatives considered and rejected, evidence references, confidence/uncertainty
evolution, reflection, lessons, and rationale.

Future planning and decision systems may **reuse** this evidence without re-executing
prior reasoning.

## What this is not

- Not conversational / vector / agent scratchpad memory
- Not a second Planning Engine
- Not a Decision, Recommendation, Task, Intent, AiPlan, or Execution owner
- Not executable — status is informational only (`draft` / `current` / `superseded` / `archived`)

## Ownership

| Concern | Owner |
|---------|-------|
| Reasoning history, reflection, reasoning evolution | WorkspaceReasoningMemoryService |
| Planning sequencing | WorkspacePlanningService |
| Decisions / overlays | DecisionEngine / DecisionQueue |
| Recommendations | WorkspaceRecommendationEngineService |
| Execution | ExecutionLifecycleService |
| Task Graph | TaskGraphService |
| Intent goals | WorkspaceIntentService |
| Persistence guards | ReasoningMemoryRepository |

## Model

| Type | Role |
|------|------|
| `ReasoningRecord` | Durable evidence record (reference-only links) |
| `ReasoningLink` / `ReasoningEvidenceReference` | Foreign identity pointers |
| `ReasoningSnapshot` | Dual-channel projection |
| `ReasoningHistoryEntry` | Terminal evidence only |
| `ReasoningSummary` | Compact projection with authoritative `history_count` |

Records reference planning / intent / task / recommendation / decision / execution /
purpose / memory / attention by ID. **Never duplicate payloads.**

## Projection contract

| Channel | Contents | Actionable? |
|---------|----------|-------------|
| `current` | Active reasoning record (at most one `current`) | View/inspect only |
| `history` | Superseded / archived via append-only `reasoning_history` | Never |
| `history_count` | Full terminal count | Scalar authority |

History DTO fields must never include execute / dispatch / approve / accept / command /
retry / cancel / transition / mutate. `actionable: false`, `authority_effect: none`.

## Mutation path

```
Actor → CommandPipeline → PermissionGateway → WorkspaceReasoningMemoryService → Repository
```

| Command | Kind | Capability |
|---------|------|------------|
| `GenerateReasoningRecord` | Mutation | `work_context.write` |
| `GetReasoningRecord` | Query | `work_context.read` |
| `GetReasoningSummary` | Query | `work_context.read` |

Generate supersedes prior `current` and appends history inside `Database::run_in_transaction`.
No partial writes. No delete lifecycle. Superseded records remain retained.

## Service may / may not

| May | Must not |
|-----|----------|
| Compose, summarise, compare, reflect, capture rationale | Choose actions, approve, dispatch, mutate foreign lifecycles |

## Recovery

Restart reconstructs current + history + confidence/uncertainty evolution from durable
tables. Missing reasoning remains missing — never fabricate. No reasoning replay. No
execution recovery owned by this layer.

## React

Projection-only: current reasoning, history, confidence, uncertainty, reflection,
lessons. No action buttons, mutation, lifecycle controls, or command conversion.

## Related

- [Programme II — Cognitive Workspace](./PROGRAMME-II-COGNITIVE-WORKSPACE.md)
- [Planning Architecture](./PLANNING-ARCHITECTURE.md)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
- [Operational Recovery](../03-Engineering/OPERATIONAL-RECOVERY.md)
