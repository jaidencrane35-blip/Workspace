# Workspace Evidence Navigation Architecture (Programme IV — Batch 2)

**Status:** Active  
**Audience:** Architecture, Kernel, Frontend, Governance  
**Depends on:** [Programme IV Interaction Runtime](./PROGRAMME-IV-INTERACTION-RUNTIME.md), [Workspace Semantic Query Architecture](./WORKSPACE-SEMANTIC-QUERY-ARCHITECTURE.md), Programme III

---

## Purpose

The **Workspace Evidence Navigation Engine** enables deterministic navigation through the evidence graph already produced by Programmes II–IV.

It answers:

> **"How are these pieces of evidence connected?"**

It never answers:

> **"What does this mean?"** or **"What should happen?"**

It is a **navigation** capability, not an interpretation capability.

---

## Primary principle

**Navigate evidence. Never interpret evidence.**

The engine:

- discovers and exposes existing evidence paths
- never infers relationships that do not exist
- never creates semantic meaning
- never reasons
- never recommends
- never executes

---

## Ownership

`WorkspaceEvidenceNavigationService` owns only:

| Owns | Does not own |
|---|---|
| evidence paths | semantic retrieval |
| navigation sessions | knowledge / contextual / explanation |
| navigation summaries | planning / reasoning / cognitive model |
| traversal metadata | workspace state / policy |
| navigation lineage | recommendations / decisions |
| navigation gaps | execution / permissions / lifecycle |

---

## Domain model

| Type | Role |
|---|---|
| `WorkspaceEvidenceNavigationSnapshot` | Dual-channel durable artefact |
| `EvidenceNavigationSession` | Non-executable request (entries, scope, depth) |
| `EvidencePath` | Existing provenance path only — no inferred edges |
| `EvidenceNavigationDiagSummary` | Reachable / unreachable / completeness (diagnostic) |
| `NavigationGap` | Broken lineage / unavailable / incomplete — never auto-repaired |
| `NavigationLineage` | Exact upstream snapshots that participated |

History is **evidence only** (`actionable: false`, `authority_effect: "none"`).

---

## Upstream access

All upstream access uses **`load_snapshot` only**.

Permitted surfaces (when scoped):

- Semantic Query, Intelligence Hub
- Knowledge Integration / Synthesis
- Contextual Understanding, Explanation
- Temporal Intelligence, Historical Reconstruction
- Workspace State

Never call foreign `::generate`, refresh upstream state, mutate repositories, infer new links, create synthetic evidence, or invoke execution.

---

## Commands

| Command | Capability |
|---|---|
| `GenerateWorkspaceEvidenceNavigation` | `work_context.write` |
| `GetWorkspaceEvidenceNavigation` | `work_context.read` |
| `GetWorkspaceEvidenceNavigationSummary` | `work_context.read` |
| `ExplainEvidenceNavigation` | `work_context.read` |

---

## Persistence

- Migration: `065_workspace_evidence_navigation.sql`
- Repository: `WorkspaceEvidenceNavigationRepository`
- Transactional supersede + append-only history
- Persistence only

---

## Behaviour

Navigation exposes existing relationships.

- Missing lineage remains missing
- Unavailable evidence remains unavailable
- Broken references remain broken
- Traversal never manufactures continuity

---

## Governance

Mutation baseline **75** · DTO inventory **26**.

Governance rejects inferred relationships, recommendation ownership, planner ownership, execution authority, lifecycle ownership, and semantic authority.

---

## Explicit confirmation

> Workspace Evidence Navigation Engine exposes existing evidence relationships only.
> It never interprets evidence, invents connections, performs reasoning, makes recommendations,
> or becomes the authority for any upstream intelligence layer.
