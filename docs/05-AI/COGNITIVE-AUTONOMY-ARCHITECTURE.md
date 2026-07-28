# Cognitive Autonomy Architecture (Programme II — Batch 8)

| Field | Value |
|-------|-------|
| **Purpose** | Governed decision-support / recommendation coordination layer |
| **Owner** | `WorkspaceCognitiveAutonomyService` (DurableStore — **suggestion evidence only**) |
| **Status** | Complete — Programme II final batch |
| **Lifecycle owner** | No |
| **Execution authority** | No |
| **Permission owner** | No |
| **Decision / Task / Planning authority** | No |

## Core principle

**Autonomy may suggest. Authority must still approve.**

This is not an autonomous agent. It is the top observational layer that evaluates
prior Programme II evidence and surfaces opportunities, proposals, confidence,
safety constraints, and approval requirements.

## What this is

The Cognitive Autonomy layer can:

- identify automation opportunities from cognitive stack evidence
- suggest non-executable workflow proposals
- assess confidence and uncertainty
- record safety evidence (risks, constraints, failure modes, required controls)
- require approval on every opportunity and recommendation
- retain append-only autonomy history

## What this is not

- Not an AI that decides and executes
- Not a replacement for CommandPipeline / PermissionGateway
- Not a lifecycle owner for planning, reasoning, tasks, decisions, or execution
- Not allowed to self-approve, grant permissions, create hidden tasks, or rewrite evidence

## Valid path (absolute)

```
Autonomy Suggestion
        ↓
User/System Approval
        ↓
Intent / Task / Command
        ↓
CommandPipeline
        ↓
PermissionGateway
        ↓
Service
        ↓
Execution
```

Forbidden path:

```
AI decides → AI executes
```

## Ownership

| Concern | Owner |
|---------|-------|
| Autonomy assessments / opportunities / proposals / confidence / safety / approval requirements / history | WorkspaceCognitiveAutonomyService |
| Execution / permissions / task / decision / planning / reasoning / memory / graph / intent / application control | Existing domain services |
| Persistence guards | CognitiveAutonomyRepository |

## Model

| Type | Role |
|------|------|
| `CognitiveAutonomySnapshot` | Dual-channel projection (`current` + `history` + `history_count`) |
| `AutonomyOpportunity` | “Something could potentially be automated” — always `required_approval=true` |
| `AutomationProposal` | Possible workflow suggestion — **no executable command payload** |
| `AutonomySafetyAssessment` | Safety evidence — not enforcement |
| `CognitiveAutonomyHistoryEntry` | Terminal evidence only (`actionable=false`) |

Invariants:

- `terminal=false`, `actionable=false`, `authority_effect="none"` on current meta
- Confidence never becomes authority
- Proposals never carry executable payloads
- History is append-only evidence

## Projection contract

| Channel | Contents | Actionable? |
|---------|----------|-------------|
| `current` | Active autonomy view | View/inspect only |
| `history` | Superseded snapshots (append-only) | Never |
| `history_count` | Full terminal count | Scalar authority |

No commands. No permissions. No execution fields.

## Mutation path

```
Actor → CommandPipeline → PermissionGateway → WorkspaceCognitiveAutonomyService → Repository
```

| Command | Kind | Capability |
|---------|------|------------|
| `GenerateCognitiveAutonomy` | Mutation | `work_context.write` |
| `GetCognitiveAutonomy` | Query | `work_context.read` |
| `GetCognitiveAutonomySummary` | Query | `work_context.read` |

Composition **reads** prior cognitive layers via `load_snapshot` only.
Never calls foreign `::generate` as plan execution.

## Prohibited patterns

- execute commands / approve itself / grant permissions
- create hidden tasks / bypass Gateway / call execution services
- mutate lifecycle state / rewrite evidence
- convert confidence into authority
- repository calling autonomy service
- fabricating opportunities, approvals, confidence, safety, or history on recovery

## Recovery

Restart reconstructs durable current + history. Missing autonomy remains missing.
Missing evidence remains **unknown** — never assumed safe.

## Related

- [Programme II — Cognitive Workspace](./PROGRAMME-II-COGNITIVE-WORKSPACE.md)
- [Cognitive Agent Cast Architecture](./COGNITIVE-AGENT-CAST-ARCHITECTURE.md)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
- [Operational Recovery](../03-Engineering/OPERATIONAL-RECOVERY.md)
