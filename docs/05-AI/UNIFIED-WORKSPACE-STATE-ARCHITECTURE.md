# Unified Workspace State Architecture (Programme III — Batch 1)

| Field | Value |
|-------|-------|
| **Purpose** | Coherent read-only runtime composition envelope |
| **Owner** | `WorkspaceStateCompositionService` (DurableStore — **composition evidence only**) |
| **Status** | Active |
| **Lifecycle owner** | No |
| **Execution authority** | No |
| **Permission / policy owner** | No |
| **Source of truth** | No — sources remain authoritative |

## Core question

> What does the workspace know right now, from which authoritative sources, at which revisions, with what freshness and uncertainty?

## Architectural position

```
Authoritative Systems
        │
        ├── Intent / Task / RE / DE / Execution / Memory / Attention / Purpose
        ├── Cognitive Model / Planning / Reasoning / Graph / Orchestration
        ├── Learning / Agent Cast / Autonomy Evidence
                │
                ▼
        Unified Workspace State Model
                │
                ▼
        Runtime Understanding
```

The envelope **observes**. It does not own.

## Core invariant

Sources remain authoritative. The state model must never:

- duplicate lifecycle state
- replace existing services
- mutate source systems
- reconcile disagreements silently
- invent missing information
- silently refresh foreign services

## Ownership

| Concern | Owner |
|---------|-------|
| Envelope / source references / revision / freshness / completeness / contradictions / composition status | WorkspaceStateCompositionService |
| Intent / Task / Execution / Cognitive truth / Permissions / Policies / Decisions / Memory contents | Existing domain services |
| Persistence guards | WorkspaceStateEnvelopeRepository |

## Model

| Type | Role |
|------|------|
| `WorkspaceStateEnvelope` | Composed current view |
| `WorkspaceStateSource` | One authoritative input reference |
| `WorkspaceStateConflict` | Contradiction evidence (never auto-repair) |
| `WorkspaceStateSnapshot` | Dual-channel projection |
| `WorkspaceStateHistoryEntry` | Terminal evidence only |

### Freshness (never collapse)

`Fresh` · `Stale` · `Unavailable` · `Unknown`

Bad: “state missing”  
Good: “Execution lifecycle unavailable because source revision 421 could not be loaded.”

### Completeness

`Complete` · `Partial` · `Unknown` · `Contradictory`

Uncertainty is preserved.

## Projection contract

| Channel | Contents | Actionable? |
|---------|----------|-------------|
| `current` | Active envelope | View/inspect only |
| `history` | Superseded envelopes (append-only) | Never |
| `history_count` | Full terminal count | Scalar authority |

No state replay. No event sourcing. History is evidence only.

## Naming note

Desktop observation already owns `GetWorkspaceState` / domain `WorkspaceState`.
Programme III commands are therefore:

| Command | Kind | Capability |
|---------|------|------------|
| `GenerateWorkspaceStateEnvelope` | Mutation | `work_context.write` |
| `GetWorkspaceStateEnvelope` | Query | `work_context.read` |
| `GetWorkspaceStateEnvelopeSummary` | Query | `work_context.read` |

## Mutation path

```
Actor → CommandPipeline → PermissionGateway → WorkspaceStateCompositionService → Repository
```

Composition **reads** registered side-effect-free projections only.
Sources without a registered revisioned load path are recorded as **Unavailable** — never assumed current.

## Determinism

Same source revisions + same inputs ⇒ same composition revision / source set / freshness / completeness / conflicts.
`state_id` / wall-clock `generated_at` may differ per generation.

## Prohibited patterns

- execute / approve / grant / dispatch / mutate / repair sources
- silently refresh via foreign `generate`
- repository calling composition service
- fabricating revisions, freshness, availability, completeness, or conflicts on recovery

## Recovery

Restart reconstructs durable current + history. Missing envelope remains missing.
Missing source remains **Unavailable** — never assumed fresh/current.
Stale never becomes Fresh without source evidence.

## Related

- [Programme III — Coherent Workspace Runtime](./PROGRAMME-III-COHERENT-WORKSPACE-RUNTIME.md)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
- [Operational Recovery](../03-Engineering/OPERATIONAL-RECOVERY.md)
