# Workspace Evidence Completeness Architecture (Programme IV — Batch 8)

**Status:** Active  
**Audience:** Architecture, Kernel, Frontend, Governance  
**Depends on:** [Programme IV Interaction Runtime](./PROGRAMME-IV-INTERACTION-RUNTIME.md), [Workspace Evidence Freshness Architecture](./WORKSPACE-EVIDENCE-FRESHNESS-ARCHITECTURE.md)

---

## Purpose

Programme IV now provides retrieval, navigation, trace, coverage, consistency, dependency, and freshness. Batch 8 answers:

> **"How complete is the available evidence, and where are the observable omissions?"**

This is completeness observation — not truth assessment, quality scoring, recommendations, repair, or execution.

---

## Primary principle

**Observe completeness. Never complete evidence.**

The engine measures only observable completeness from recorded evidence.

It never:

- fills gaps
- estimates missing information
- repairs incomplete evidence
- generates upstream snapshots
- executes

---

## Ownership

`WorkspaceEvidenceCompletenessService` owns only:

| Owns | Does not own |
|---|---|
| completeness snapshots | semantic retrieval |
| completeness observations | navigation / provenance / coverage / consistency / dependency / freshness |
| completeness diagnostics / gaps / summaries / metadata | explanations / contextual / knowledge / planning / reasoning / policy / execution / permissions / lifecycle |

---

## Domain model

| Type | Role |
|---|---|
| `WorkspaceEvidenceCompletenessSnapshot` | Dual-channel durable artefact (`current` / `history` / `history_count`) |
| `CompletenessAssessment` | Observed scope, participating artefacts, observable completeness state — no interpretation |
| `CompletenessObservation` | One observed evidence state: Complete / Partial / Unknown / Unavailable — descriptive only |
| `CompletenessGap` | Missing evidence / unavailable artefacts / incomplete lineage / incomplete observation — never auto-repaired |
| `CompletenessDiagnostics` | Observed coverage, missing observations, unavailable artefacts — diagnostic only |

History is **evidence only**.

---

## Upstream access

All upstream access uses **`load_snapshot` only**.

Permitted: Evidence Freshness, Evidence Dependency, Evidence Consistency, Evidence Coverage, Evidence Trace, Evidence Navigation, Semantic Query, Intelligence Hub, Knowledge Integration, Contextual Understanding, Explanation, Temporal Intelligence, Historical Reconstruction, Workspace State.

Never call foreign `::generate`, infer missing evidence, fabricate completeness, repair omissions, regenerate evidence, or execute.

---

## Commands

| Command | Capability |
|---|---|
| `GenerateWorkspaceEvidenceCompleteness` | `work_context.write` |
| `GetWorkspaceEvidenceCompleteness` | `work_context.read` |
| `GetWorkspaceEvidenceCompletenessSummary` | `work_context.read` |
| `ExplainEvidenceCompleteness` | `work_context.read` |

---

## Persistence

- Migration: `071_workspace_evidence_completeness.sql`
- Repository: `WorkspaceEvidenceCompletenessRepository`
- Transactional supersede + append-only history

---

## Behaviour

- Completeness is observed only
- Partial remains partial
- Unknown remains unknown
- Unavailable remains unavailable
- Missing evidence remains missing
- The engine never repairs or supplements evidence
- Completeness never implies truth

---

## Recovery

Recovery never fabricates evidence, fills gaps, estimates completeness, or repairs missing artefacts.

Contracts:

- `recovery_must_not_fabricate_evidence_completeness`
- `recovery_must_not_fabricate_actionable_evidence_completeness_history`

---

## Governance

Mutation baseline **81** · DTO inventory **32**.

Governance rejects: repair ownership, inferred completeness, evidence fabrication, execution authority, lifecycle authority.

---

## Explicit confirmation

> Workspace Evidence Completeness Engine observes recorded evidence completeness only.
> It never repairs evidence, fills gaps, fabricates missing information, performs reasoning, makes recommendations,
> or becomes the authority for any upstream intelligence layer.
