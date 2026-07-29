# Workspace Evidence Reliability Architecture (Programme IV — Batch 9)

**Status:** Active  
**Audience:** Architecture, Kernel, Frontend, Governance  
**Depends on:** [Workspace Evidence Completeness Architecture](./WORKSPACE-EVIDENCE-COMPLETENESS-ARCHITECTURE.md)

---

## Purpose

Batch 9 observes recorded reliability characteristics across Programme IV evidence surfaces. It answers:

> **"What reliability characteristics are observable from recorded evidence metadata?"**

Reliability is descriptive only. It is not truth, trust, conflict settlement, repair, ranking, or execution.

---

## Primary principle

**Observe reliability. Never establish truth.**

`Reliable`, `Limited`, `Unknown`, and `Unavailable` are observation states. Limited evidence remains limited; unknown remains unknown; unavailable remains unavailable.

---

## Ownership

`WorkspaceEvidenceReliabilityService` owns only:

| Owns | Does not own |
|---|---|
| reliability snapshots | upstream evidence generation |
| reliability observations | semantic retrieval / navigation / trace / coverage / consistency / dependency / freshness / completeness |
| reliability diagnostics / gaps / summaries / metadata | planning / reasoning / policy / permissions / lifecycle / execution |

---

## Domain model

| Type | Role |
|---|---|
| `WorkspaceEvidenceReliabilitySnapshot` | Durable artefact with `current`, `history`, and authoritative `history_count` |
| `ReliabilityAssessment` | Observed scope, participating artefacts, observable reliability states |
| `ReliabilityObservation` | One observed state: Reliable / Limited / Unknown / Unavailable |
| `ReliabilityGap` | Unavailable evidence, missing provenance, incomplete observation, or insufficient characteristics |
| `ReliabilityDiagnostics` | Observable reliability coverage and characteristic limitations |

History is evidence only: `terminal: true`, `actionable: false`, `authority_effect: none`.

---

## Upstream access

All upstream access uses `load_snapshot` only. Batch 9 observes 15 surfaces:

1. Evidence Completeness
2. Evidence Freshness
3. Evidence Dependency
4. Evidence Consistency
5. Evidence Coverage
6. Evidence Trace
7. Evidence Navigation
8. Semantic Query
9. Intelligence Hub
10. Knowledge Integration
11. Contextual Understanding
12. Explanation
13. Temporal Intelligence
14. Historical Reconstruction
15. Workspace State

Recorded completeness, freshness, consistency, gaps, and provenance shape the reliability observation. The engine never refreshes, repairs, ranks, or changes upstream artefacts.

---

## Commands

| Command | Capability |
|---|---|
| `GenerateWorkspaceEvidenceReliability` | `work_context.write` |
| `GetWorkspaceEvidenceReliability` | `work_context.read` |
| `GetWorkspaceEvidenceReliabilitySummary` | `work_context.read` |
| `ExplainEvidenceReliability` | `work_context.read` |

---

## Persistence

- Migration: `072_workspace_evidence_reliability.sql`
- Repository: `WorkspaceEvidenceReliabilityRepository`
- Transactional supersede plus append-only history

---

## Recovery

Recovery never fabricates reliability, evidence, provenance, or history. Contracts:

- `recovery_must_not_fabricate_evidence_reliability`
- `recovery_must_not_fabricate_actionable_evidence_reliability_history`

---

## Governance

- Mutation baseline: **82**
- History / projection DTO inventory length: **33**
- Guard rejects execution ownership, conflict settlement ownership, repair ownership, ranking ownership, and foreign `generate` calls.
