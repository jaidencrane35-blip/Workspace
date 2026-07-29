# Workspace Evidence Consistency Architecture (Programme IV — Batch 5)

**Status:** Active  
**Audience:** Architecture, Kernel, Frontend, Governance  
**Depends on:** [Programme IV Interaction Runtime](./PROGRAMME-IV-INTERACTION-RUNTIME.md), [Workspace Evidence Coverage Architecture](./WORKSPACE-EVIDENCE-COVERAGE-ARCHITECTURE.md), [Workspace Evidence Trace Architecture](./WORKSPACE-EVIDENCE-TRACE-ARCHITECTURE.md)

---

## Purpose

Previous Programme IV batches establish:

| Batch | Capability |
|---|---|
| 1 | Evidence retrieval |
| 2 | Evidence navigation |
| 3 | Provenance tracing |
| 4 | Evidence coverage |

Batch 5 answers:

> **"Are the available pieces of evidence internally consistent?"**

This is consistency observation — not conflict resolution, judgement, or authority.

---

## Primary principle

**Observe consistency. Never resolve consistency.**

The engine detects and records observable agreement, disagreement, incompleteness, and ambiguity across existing evidence.

It never:

- decides which evidence is correct
- chooses a winner
- repairs inconsistency
- creates consensus
- executes

---

## Ownership

`WorkspaceEvidenceConsistencyService` owns only:

| Owns | Does not own |
|---|---|
| consistency snapshots | semantic retrieval |
| consistency observations | navigation / provenance / coverage |
| consistency diagnostics | explanations / contextual / knowledge |
| detected inconsistencies | planning / reasoning / policy |
| consistency gaps / summaries | decisions / execution / permissions / lifecycle |

---

## Domain model

| Type | Role |
|---|---|
| `WorkspaceEvidenceConsistencySnapshot` | Dual-channel durable artefact |
| `ConsistencyAssessment` | Observed scope, participating sources, compared evidence, observable outcomes |
| `ConsistencyObservation` | One observed relationship (`Consistent` / `Inconsistent` / `Partial` / `Unknown` / `Unavailable`) |
| `ConsistencyConflict` | Observable disagreement — never a preferred result |
| `ConsistencyGap` | Unavailable / insufficient / incomplete / missing lineage — never auto-repaired |
| `ConsistencyDiagnostics` | Comparison completeness + limitations — diagnostic only |

History is **evidence only**.

---

## Upstream access

All upstream access uses **`load_snapshot` only**.

Permitted: Evidence Coverage, Evidence Trace, Evidence Navigation, Semantic Query, Intelligence Hub, Knowledge Integration, Contextual Understanding, Explanation, Temporal Intelligence, Historical Reconstruction, Workspace State.

Never call foreign `::generate`, modify upstream evidence, infer missing facts, resolve conflicts, fabricate agreement/disagreement, refresh upstream state, or execute.

---

## Commands

| Command | Capability |
|---|---|
| `GenerateWorkspaceEvidenceConsistency` | `work_context.write` |
| `GetWorkspaceEvidenceConsistency` | `work_context.read` |
| `GetWorkspaceEvidenceConsistencySummary` | `work_context.read` |
| `ExplainEvidenceConsistency` | `work_context.read` |

---

## Persistence

- Migration: `068_workspace_evidence_consistency.sql`
- Repository: `WorkspaceEvidenceConsistencyRepository`
- Transactional supersede + append-only history

---

## Behaviour

- Compares observable evidence only
- Agreement is recorded
- Disagreement is recorded
- Unknown remains unknown
- Unavailable remains unavailable
- Conflicts remain unresolved

---

## Recovery

Recovery never repairs conflicts, invents consistency/inconsistency, or assumes agreement.

Contracts:

- `recovery_must_not_fabricate_evidence_consistency`
- `recovery_must_not_fabricate_actionable_evidence_consistency_history`

---

## Governance

Mutation baseline **78** · DTO inventory **29**.

Governance rejects: conflict resolution, inferred agreement/disagreement, recommendation ownership, execution authority, lifecycle authority.

---

## Explicit confirmation

> Workspace Evidence Consistency Engine observes consistency across recorded evidence only.
> It never resolves conflicts, determines truth, performs reasoning, makes recommendations,
> or becomes the authority for any upstream intelligence layer.
