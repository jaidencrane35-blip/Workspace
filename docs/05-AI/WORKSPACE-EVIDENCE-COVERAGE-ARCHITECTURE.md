# Workspace Evidence Coverage Architecture (Programme IV — Batch 4)

**Status:** Active  
**Audience:** Architecture, Kernel, Frontend, Governance  
**Depends on:** [Programme IV Interaction Runtime](./PROGRAMME-IV-INTERACTION-RUNTIME.md), [Workspace Evidence Trace Architecture](./WORKSPACE-EVIDENCE-TRACE-ARCHITECTURE.md), [Workspace Evidence Navigation Architecture](./WORKSPACE-EVIDENCE-NAVIGATION-ARCHITECTURE.md), [Workspace Semantic Query Architecture](./WORKSPACE-SEMANTIC-QUERY-ARCHITECTURE.md)

---

## Purpose

Where previous Programme IV batches answer:

| Batch | Question |
|---|---|
| 1 | What evidence matches this query? |
| 2 | How is evidence connected? |
| 3 | Where did this evidence originate? |

Batch 4 answers:

> **"How complete is our evidence for this subject?"**

This is an evidence completeness capability — not interpretation, confidence reasoning, recommendation, or execution.

---

## Primary principle

**Measure evidence coverage. Never measure truth.**

The Coverage Engine evaluates only the observable completeness of recorded evidence.

It never:

- concludes whether evidence is correct
- infers missing information
- predicts
- recommends
- executes

---

## Ownership

`WorkspaceEvidenceCoverageService` owns only:

| Owns | Does not own |
|---|---|
| coverage snapshots | semantic retrieval |
| coverage metrics | navigation |
| evidence completeness | provenance |
| evidence availability | explanations / contextual / knowledge |
| coverage gaps | planning / reasoning / policy |
| coverage diagnostics | decisions / execution / permissions / lifecycle |

---

## Domain model

| Type | Role |
|---|---|
| `WorkspaceEvidenceCoverageSnapshot` | Dual-channel durable artefact (`current` / `history` / `history_count`) |
| `CoverageAssessment` | Requested scope, observed sources, available / unavailable evidence — no interpretation |
| `CoverageMetric` | Observable quantities only (sources, artefacts, missing refs, unavailable snapshots, lineage completeness) |
| `CoverageGap` | Unavailable / broken / missing provenance / incomplete retrieval — never auto-resolved |
| `CoverageLineage` | Exact upstream artefacts that contributed |
| `CoverageDiagnostics` | Completeness state, observed limitations, unavailable sources — no recommendations |

History is **evidence only**.

---

## Upstream access

All upstream access uses **`load_snapshot` only**.

Permitted: Evidence Trace, Evidence Navigation, Semantic Query, Intelligence Hub, Knowledge Integration, Contextual Understanding, Explanation, Temporal Intelligence, Historical Reconstruction, Workspace State.

Never call foreign `::generate`, infer evidence, repair missing lineage, fabricate completeness, refresh upstream state, or execute.

---

## Commands

| Command | Capability |
|---|---|
| `GenerateWorkspaceEvidenceCoverage` | `work_context.write` |
| `GetWorkspaceEvidenceCoverage` | `work_context.read` |
| `GetWorkspaceEvidenceCoverageSummary` | `work_context.read` |
| `ExplainEvidenceCoverage` | `work_context.read` |

---

## Persistence

- Migration: `067_workspace_evidence_coverage.sql`
- Repository: `WorkspaceEvidenceCoverageRepository`
- Transactional supersede + append-only history
- Persistence only — never service logic in the repository

---

## Behaviour

- Coverage reports observable completeness only
- Unknown remains unknown
- Unavailable remains unavailable
- Missing evidence remains missing
- Coverage never implies correctness
- Coverage never implies confidence
- Coverage never implies recommendation

---

## Projection contract

Dual-channel: `current` + `history` + authoritative `history_count`.

History: evidence only, never actionable.

Forbidden: execute, recommend, approve, optimise, automate, dispatch, planning, reasoning, permissions, lifecycle.

---

## Recovery

Recovery never invents missing evidence, fills unavailable sources, repairs broken lineage, or estimates coverage.

Coverage reflects only observable state.

Contracts:

- `recovery_must_not_fabricate_evidence_coverage`
- `recovery_must_not_fabricate_actionable_evidence_coverage_history`

---

## Governance

Mutation baseline **77** · DTO inventory **28**.

Governance rejects: inferred completeness, fabricated coverage, recommendation ownership, execution authority, planner ownership, lifecycle authority.

---

## Explicit confirmation

> Workspace Evidence Coverage Engine measures observable evidence completeness only.
> It never infers missing evidence, evaluates truth, performs reasoning, makes recommendations,
> or becomes the authority for any upstream intelligence layer.

---

## Successor

- [Workspace Evidence Consistency Architecture](./WORKSPACE-EVIDENCE-CONSISTENCY-ARCHITECTURE.md) (Batch 5 — observe consistency)
