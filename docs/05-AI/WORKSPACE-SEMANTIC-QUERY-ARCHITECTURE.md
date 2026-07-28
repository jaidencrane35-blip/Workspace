# Workspace Semantic Query Architecture (Programme IV — Batch 1)

**Status:** Active  
**Audience:** Architecture, Kernel, Frontend, Governance  
**Depends on:** Programme II, Programme III (all accepted batches), [Programme IV Interaction Runtime](./PROGRAMME-IV-INTERACTION-RUNTIME.md)

---

## Purpose

The **Workspace Semantic Query Engine** is the canonical read-only semantic retrieval layer over everything produced by Programmes II and III.

It answers:

> **"What do we already know?"**

It never answers:

> **"What should we do?"**

---

## Primary principle

**Retrieve meaning. Never create meaning.**

The engine:

- retrieves existing evidence
- never synthesises new truth
- never reasons
- never plans
- never recommends
- never executes

---

## Ownership

`WorkspaceSemanticQueryService` owns only:

| Owns | Does not own |
|---|---|
| semantic query execution (retrieve-only) | cognitive model |
| semantic retrieval | planning / reasoning / graph |
| retrieval packages | orchestration / learning / agent cast |
| retrieval provenance | autonomy / workspace state / policy |
| retrieval completeness | reconstruction / temporal / explanation |
| retrieval diagnostics | contextual / knowledge / insight |
| | cross-workspace / decision support / hub |
| | memory truth / execution / permissions / lifecycle |

---

## Domain model

| Type | Role |
|---|---|
| `WorkspaceSemanticQuerySnapshot` | Dual-channel durable artefact (`current` / history) |
| `SemanticQuery` | Non-executable request (query, scope, filters, provenance requirements) |
| `SemanticQueryResult` | Retrieved evidence only — no interpretation |
| `SemanticMatch` | Matched source + diagnostic relevance + evidence ref + revision |
| `RetrievalGap` | Unavailable / incomplete / missing lineage — never auto-filled |
| `RetrievalLineage` | Exact upstream snapshots that contributed |
| `RetrievalDiagnostics` | Completeness / gap diagnostics only |

History is **evidence only** (`actionable: false`, `authority_effect: "none"`).

---

## Upstream access

All upstream access uses **`load_snapshot` only**.

Never call foreign `::generate`, refresh upstream state, mutate repositories, invoke lifecycle/execution, reason, or infer relationships.

Surfaces (when scoped in):

- Workspace state envelope, policy governance, historical reconstruction, temporal intelligence
- Explanation, contextual understanding, knowledge synthesis / integration
- Insight coordination, cross-workspace intelligence, decision support, intelligence hub

---

## Commands

Via `CommandPipeline` → `PermissionGateway`:

| Command | Capability |
|---|---|
| `GenerateWorkspaceSemanticQuery` | `work_context.write` |
| `GetWorkspaceSemanticQuery` | `work_context.read` |
| `GetWorkspaceSemanticQuerySummary` | `work_context.read` |
| `ExplainWorkspaceSemanticQuery` | `work_context.read` |

---

## Persistence

- Migration: `064_workspace_semantic_query.sql`
- Repository: `WorkspaceSemanticQueryRepository`
- Transactional supersede + append-only history
- Persistence only — no service calls from repository

---

## Projection contract

Dual-channel: `current` + `history` + authoritative `history_count`.

Forbidden affordances: execute, approve, recommend, optimise, choose, dispatch, automate, lifecycle, permissions.

React helpers are projection-only (`semanticQueryProjection.ts`).

---

## Recovery

Recovery never invents matches, fabricates relevance, invents lineage, generates missing answers, or infers absent evidence.

Missing evidence remains missing.

---

## Governance

Mutation baseline **74** · DTO inventory **25**.

Governance rejects recommendation, planning, reasoning, execution, lifecycle, and policy ownership on this surface.

---

## Explicit confirmation

> Workspace Semantic Query Engine retrieves existing evidence only.
> It never creates knowledge, performs reasoning, makes recommendations,
> or becomes the authority for any upstream intelligence layer.
