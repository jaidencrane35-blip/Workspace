# Cognitive Agent Cast Architecture (Programme II — Batch 7)

| Field | Value |
|-------|-------|
| **Purpose** | Governed internal representation of specialised cognitive roles |
| **Owner** | `WorkspaceCognitiveAgentCastService` (DurableStore — **evidence / coordination only**) |
| **Status** | Active |
| **Lifecycle owner** | No |
| **Execution authority** | No |
| **Permission owner** | No |
| **Decision / Task / Planning authority** | No |

## What this is

The Cognitive Agent Cast provides role representations that can:

- analyse evidence
- produce perspectives
- critique assumptions
- compare alternatives
- contribute reasoning artefacts
- provide synthesis inputs

## What this is not

- Not an agent swarm or autonomous executor
- Not a replacement for existing services
- Not users, actors, or operating authorities
- Not allowed to call commands, request permissions, execute tools, create tasks, accept decisions, or mutate lifecycles

**Agents are perspectives. Not permissions.**

## Ownership

| Concern | Owner |
|---------|-------|
| Agent identities / roles / perspectives / critiques / syntheses / cast history | WorkspaceCognitiveAgentCastService |
| Intent / Task / RE / DE / Execution / Planning / Reasoning / Graph / Learning | Existing domain services |
| Persistence guards | CognitiveAgentCastRepository |

## Model

| Type | Role |
|------|------|
| `CognitiveAgent` | Role representation (constraints; never actionable) |
| `AgentPerspective` | Observational contribution |
| `AgentCritique` | Informational concerns — cannot reject/block systems |
| `AgentSynthesis` | Summary of agreement / disagreement / open questions — not a decision |
| `CognitiveAgentCastSnapshot` | Dual-channel projection |
| `CognitiveAgentCastHistoryEntry` | Terminal evidence only |

Default roles: Planner Critic, Risk Analyst, Systems Architect, User Advocate, Evidence Reviewer, Efficiency Analyst.

## Projection contract

| Channel | Contents | Actionable? |
|---------|----------|-------------|
| `current` | Active cast view | View/inspect only |
| `history` | Superseded snapshots (append-only) | Never |
| `history_count` | Full terminal count | Scalar authority |

`authority_effect` is always `"none"`. No command fields. No lifecycle. No execution.

## Mutation path

```
Actor → CommandPipeline → PermissionGateway → WorkspaceCognitiveAgentCastService → Repository
```

| Command | Kind | Capability |
|---------|------|------------|
| `GenerateCognitiveAgentCast` | Mutation | `work_context.write` |
| `GetCognitiveAgentCast` | Query | `work_context.read` |
| `GetCognitiveAgentCastSummary` | Query | `work_context.read` |

Composition **reads** prior cognitive layers only. Agents never become actors.

## Prohibited patterns

- execute / launch / dispatch
- call commands directly / bypass Gateway or Pipeline
- request permissions / self-authorise
- create tasks / accept decisions
- mutate lifecycle state
- fabricate agents, perspectives, agreement, or confidence
- repository calling agent cast service

## Recovery

Restart reconstructs durable current + history. Missing cast remains missing.
Missing evidence remains unknown — never inferred.

## Related

- [Programme II — Cognitive Workspace](./PROGRAMME-II-COGNITIVE-WORKSPACE.md)
- [Learning & Adaptation Architecture](./LEARNING-ADAPTATION-ARCHITECTURE.md)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
- [Operational Recovery](../03-Engineering/OPERATIONAL-RECOVERY.md)
