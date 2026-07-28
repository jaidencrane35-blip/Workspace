# Learning & Adaptation Architecture (Programme II — Batch 6)

| Field | Value |
|-------|-------|
| **Purpose** | Evidence-based improvement signals from historical outcomes |
| **Owner** | `WorkspaceLearningAdaptationService` (DurableStore — **meta-evidence only**) |
| **Status** | Active |
| **Lifecycle owner** | No |
| **Execution authority** | No |
| **Planning / Reasoning / Graph / Orchestration authority** | No |

## What this is

Learning observes outcomes from Planning, Reasoning, Cognitive Graph, Orchestration,
Recommendations, Decisions, Tasks, and Execution evidence, and produces:

- learned patterns (`PatternObserved`)
- confidence evolution records
- effectiveness / success / failure signals
- adaptation suggestions

## What this is not

- Not an intelligence authority
- Not a decision / execution / planning owner
- Not allowed to auto-change plans, permissions, or lifecycles
- Not allowed to auto-apply adaptations or rewrite history
- Not upstream of Cognitive Model / Planning / Reasoning

**Learning is downstream evidence. It cannot become upstream authority.**

## Ownership

| Concern | Owner |
|---------|-------|
| Learning records / patterns / adaptation evidence / history | WorkspaceLearningAdaptationService |
| Intent / Task / RE / DE / Execution / Planning / Reasoning / Graph | Existing domain services |
| Persistence guards | LearningAdaptationRepository |

## Model

| Type | Role |
|------|------|
| `LearningView` | Current observational snapshot |
| `LearningSnapshot` | Dual-channel projection |
| `LearningHistoryEntry` | Terminal evidence only |
| `LearningSummary` | Compact projection with authoritative `history_count` |
| `LearningPattern` | Observational pattern (`pattern_observed`) |
| `ConfidenceUpdate` | before / evidence / after — does not own foreign confidence |
| `AdaptationCandidate` | Suggestion only (`actionable=false`) |

## Projection contract

| Channel | Contents | Actionable? |
|---------|----------|-------------|
| `current` | Active learning view | View/inspect only |
| `history` | Superseded snapshots (append-only) | Never |
| `history_count` | Full terminal count | Scalar authority |

`authority_effect` is always `"none"`. No command fields. No lifecycle. No execution.

## Mutation path

```
Actor → CommandPipeline → PermissionGateway → WorkspaceLearningAdaptationService → Repository
```

| Command | Kind | Capability |
|---------|------|------------|
| `GenerateLearningSnapshot` | Mutation | `work_context.write` |
| `GetLearningSnapshot` | Query | `work_context.read` |
| `GetLearningSummary` | Query | `work_context.read` |

Composition **reads** foreign snapshots / repository overlays only.
It never calls foreign `generate` paths and never imports `ExecutionLifecycleService`.

## Prohibited patterns

- auto-change plans / permissions
- auto-execute / launch
- rewrite history / modify lifecycle
- approve decisions / apply adaptations automatically
- fabricate success or failure evidence
- Gateway / Pipeline bypass
- repository calling learning service

## Recovery

Restart reconstructs durable current + history. Missing learning remains missing.
Missing outcomes remain unknown — never assumed. Never invent lessons.

## Related

- [Programme II — Cognitive Workspace](./PROGRAMME-II-COGNITIVE-WORKSPACE.md)
- [Cognitive Orchestration Architecture](./COGNITIVE-ORCHESTRATION-ARCHITECTURE.md)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
- [Operational Recovery](../03-Engineering/OPERATIONAL-RECOVERY.md)
