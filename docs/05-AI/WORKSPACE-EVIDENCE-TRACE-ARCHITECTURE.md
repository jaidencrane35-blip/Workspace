# Workspace Evidence Trace Architecture (Programme IV — Batch 3)

**Status:** Active  
**Audience:** Architecture, Kernel, Frontend, Governance  
**Depends on:** [Programme IV Interaction Runtime](./PROGRAMME-IV-INTERACTION-RUNTIME.md), [Workspace Evidence Navigation Architecture](./WORKSPACE-EVIDENCE-NAVIGATION-ARCHITECTURE.md), [Workspace Semantic Query Architecture](./WORKSPACE-SEMANTIC-QUERY-ARCHITECTURE.md)

---

## Purpose

Where Batch 2 answers *how evidence objects are connected*, Batch 3 answers:

> **"Exactly where did this single conclusion, observation, or artefact originate?"**

This is deterministic provenance tracing — not explanation, reasoning, or interpretation.

---

## Primary principle

**Trace provenance. Never infer provenance.**

The engine:

- reconstructs the exact recorded lineage for an artefact
- never fills missing history
- never invents intermediate steps
- never explains why something happened
- never recommends action
- never executes

---

## Ownership

`WorkspaceEvidenceTraceService` owns only:

| Owns | Does not own |
|---|---|
| evidence traces | semantic retrieval |
| provenance chains | evidence navigation |
| lineage packages | explanations / contextual / knowledge |
| trace diagnostics | planning / reasoning / policy |
| trace completeness / gaps | execution / permissions / lifecycle |

---

## Domain model

| Type | Role |
|---|---|
| `WorkspaceEvidenceTraceSnapshot` | Dual-channel durable artefact |
| `EvidenceTraceRequest` | Non-executable request (target, scope, depth, constraints) |
| `ProvenanceChain` | Recorded lineage only — no inferred nodes |
| `TraceSegment` | One recorded provenance hop |
| `TraceGap` | Broken / unavailable / truncated / missing — never auto-repaired |
| `TraceDiagnostics` | Completeness / reachable / missing / unavailable (diagnostic) |

History is **evidence only**.

---

## Upstream access

All upstream access uses **`load_snapshot` only**.

Permitted: Evidence Navigation, Semantic Query, Intelligence Hub, Knowledge Integration/Synthesis, Contextual, Explanation, Temporal, Reconstruction, State.

Never call foreign `::generate`, refresh upstream state, create/repair lineage, infer provenance, or execute.

---

## Commands

| Command | Capability |
|---|---|
| `GenerateWorkspaceEvidenceTrace` | `work_context.write` |
| `GetWorkspaceEvidenceTrace` | `work_context.read` |
| `GetWorkspaceEvidenceTraceSummary` | `work_context.read` |
| `ExplainEvidenceTrace` | `work_context.read` |

---

## Persistence

- Migration: `066_workspace_evidence_trace.sql`
- Repository: `WorkspaceEvidenceTraceRepository`
- Transactional supersede + append-only history

---

## Governance

Mutation baseline **76** · DTO inventory **27**.

---

## Explicit confirmation

> Workspace Evidence Trace Engine reconstructs recorded provenance only.
> It never invents lineage, bridges missing history, performs reasoning, makes recommendations,
> or becomes the authority for any upstream intelligence layer.

---

## Successor

- [Workspace Evidence Coverage Architecture](./WORKSPACE-EVIDENCE-COVERAGE-ARCHITECTURE.md) (Batch 4 — observable completeness)
