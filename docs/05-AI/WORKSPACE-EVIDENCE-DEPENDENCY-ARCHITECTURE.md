# Workspace Evidence Dependency Architecture (Programme IV — Batch 6)

**Status:** Active  
**Audience:** Architecture, Kernel, Frontend, Governance  
**Depends on:** [Programme IV Interaction Runtime](./PROGRAMME-IV-INTERACTION-RUNTIME.md), [Workspace Evidence Consistency Architecture](./WORKSPACE-EVIDENCE-CONSISTENCY-ARCHITECTURE.md), [Workspace Evidence Coverage Architecture](./WORKSPACE-EVIDENCE-COVERAGE-ARCHITECTURE.md)

---

## Purpose

Programme IV now provides retrieval, navigation, trace, coverage, and consistency. Batch 6 answers:

> **"Which pieces of evidence depend upon which other pieces of evidence?"**

This is dependency observation — not execution order, causality, workflow planning, scheduling, or authority.

---

## Primary principle

**Observe dependency. Never create dependency.**

The engine reports only dependencies already recorded within the existing evidence graph.

It never:

- infers new dependencies
- creates dependency chains
- schedules work
- establishes causality
- executes

---

## Ownership

`WorkspaceEvidenceDependencyService` owns only:

| Owns | Does not own |
|---|---|
| dependency snapshots | semantic retrieval |
| dependency graphs | navigation / provenance / coverage / consistency |
| dependency observations | explanations / contextual / knowledge |
| dependency diagnostics / gaps / summaries | planning / reasoning / policy / decisions / execution / permissions / lifecycle |

---

## Domain model

| Type | Role |
|---|---|
| `WorkspaceEvidenceDependencySnapshot` | Dual-channel durable artefact |
| `DependencyAssessment` | Requested scope, participating artefacts, observed dependency set |
| `DependencyNode` | One recorded evidence artefact — reference only |
| `DependencyRelationship` | One recorded dependency (source, target, type, recorded lineage) — never inferred |
| `DependencyGap` | Unavailable / broken / incomplete / missing — never auto-repaired |
| `DependencyDiagnostics` | Completeness, reachable count, unavailable references — diagnostic only |

History is **evidence only**.

---

## Upstream access

All upstream access uses **`load_snapshot` only**.

Permitted: Evidence Consistency, Evidence Coverage, Evidence Trace, Evidence Navigation, Semantic Query, Intelligence Hub, Knowledge Integration, Contextual Understanding, Explanation, Temporal Intelligence, Historical Reconstruction, Workspace State.

Never call foreign `::generate`, infer dependency, invent relationships, repair missing links, create execution graphs, refresh upstream state, or execute.

---

## Commands

| Command | Capability |
|---|---|
| `GenerateWorkspaceEvidenceDependency` | `work_context.write` |
| `GetWorkspaceEvidenceDependency` | `work_context.read` |
| `GetWorkspaceEvidenceDependencySummary` | `work_context.read` |
| `ExplainEvidenceDependency` | `work_context.read` |

---

## Persistence

- Migration: `069_workspace_evidence_dependency.sql`
- Repository: `WorkspaceEvidenceDependencyRepository`
- Transactional supersede + append-only history

---

## Behaviour

- Exposes only recorded dependency structure
- Missing links remain missing
- Broken links remain broken
- Unavailable sources remain unavailable
- Dependency never implies execution, causation, or recommendation

---

## Recovery

Recovery never invents dependency, repairs dependency, bridges broken links, or estimates missing structure.

Contracts:

- `recovery_must_not_fabricate_evidence_dependency`
- `recovery_must_not_fabricate_actionable_evidence_dependency_history`

---

## Governance

Mutation baseline **79** · DTO inventory **30**.

Governance rejects: inferred dependency, execution ownership, workflow ownership, lifecycle authority, recommendation ownership.

---

## Explicit confirmation

> Workspace Evidence Dependency Engine observes recorded evidence dependencies only.
> It never invents dependencies, establishes causality, creates workflows, performs reasoning, makes recommendations,
> or becomes the authority for any upstream intelligence layer.
