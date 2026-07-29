# Workspace Evidence Freshness Architecture (Programme IV — Batch 7)

**Status:** Active  
**Audience:** Architecture, Kernel, Frontend, Governance  
**Depends on:** [Programme IV Interaction Runtime](./PROGRAMME-IV-INTERACTION-RUNTIME.md), [Workspace Evidence Dependency Architecture](./WORKSPACE-EVIDENCE-DEPENDENCY-ARCHITECTURE.md)

---

## Purpose

Programme IV now provides retrieval, navigation, trace, coverage, consistency, and dependency. Batch 7 answers:

> **"How current is the available evidence?"**

This is evidence freshness observation — not scheduling, refreshing, synchronisation, prioritisation, or execution.

---

## Primary principle

**Observe freshness. Never refresh evidence.**

The engine measures only the observable freshness state of durable evidence.

It never:

- reloads upstreams
- regenerates snapshots
- estimates freshness
- schedules updates
- executes

---

## Ownership

`WorkspaceEvidenceFreshnessService` owns only:

| Owns | Does not own |
|---|---|
| freshness snapshots | semantic retrieval |
| freshness observations | navigation / provenance / coverage / consistency / dependency |
| freshness diagnostics / gaps / summaries / metadata | explanations / contextual / knowledge / planning / reasoning / policy / execution / permissions / lifecycle |

---

## Domain model

| Type | Role |
|---|---|
| `WorkspaceEvidenceFreshnessSnapshot` | Dual-channel durable artefact (`current` / `history` / `history_count`) |
| `FreshnessAssessment` | Observed scope, participating artefacts, observable freshness state — no interpretation |
| `FreshnessObservation` | One observed evidence state: Fresh / Stale / Unknown / Unavailable — descriptive only |
| `FreshnessGap` | Unavailable timestamp / missing revision / unknown age / inaccessible — never auto-repaired |
| `FreshnessDiagnostics` | Observed coverage, unavailable observations, missing timestamps — diagnostic only |

History is **evidence only**.

---

## Upstream access

All upstream access uses **`load_snapshot` only**.

Permitted: Evidence Dependency, Evidence Consistency, Evidence Coverage, Evidence Trace, Evidence Navigation, Semantic Query, Intelligence Hub, Knowledge Integration, Contextual Understanding, Explanation, Temporal Intelligence, Historical Reconstruction, Workspace State.

Never call foreign `::generate`, refresh upstreams, regenerate evidence, infer freshness, fabricate timestamps, repair stale data, or execute.

---

## Commands

| Command | Capability |
|---|---|
| `GenerateWorkspaceEvidenceFreshness` | `work_context.write` |
| `GetWorkspaceEvidenceFreshness` | `work_context.read` |
| `GetWorkspaceEvidenceFreshnessSummary` | `work_context.read` |
| `ExplainEvidenceFreshness` | `work_context.read` |

---

## Persistence

- Migration: `070_workspace_evidence_freshness.sql`
- Repository: `WorkspaceEvidenceFreshnessRepository`
- Transactional supersede + append-only history

---

## Behaviour

- Freshness is observed only
- Unknown remains unknown
- Unavailable remains unavailable
- Stale remains stale
- The engine never refreshes evidence, requests regeneration, or schedules updates
- Freshness never implies validity

---

## Recovery

Recovery never refreshes stale evidence, fabricates timestamps, invents revisions, or estimates freshness.

Contracts:

- `recovery_must_not_fabricate_evidence_freshness`
- `recovery_must_not_fabricate_actionable_evidence_freshness_history`

---

## Governance

Mutation baseline **80** · DTO inventory **31**.

Governance rejects: refresh ownership, regeneration ownership, inferred freshness, execution authority, lifecycle authority.

---

## Next

Batch 8 — [Workspace Evidence Completeness Architecture](./WORKSPACE-EVIDENCE-COMPLETENESS-ARCHITECTURE.md) (*How complete is the available evidence, and where are the observable omissions?*).

---

## Explicit confirmation

> Workspace Evidence Freshness Engine observes recorded evidence freshness only.
> It never refreshes evidence, regenerates snapshots, estimates freshness, performs reasoning, makes recommendations,
> or becomes the authority for any upstream intelligence layer.
