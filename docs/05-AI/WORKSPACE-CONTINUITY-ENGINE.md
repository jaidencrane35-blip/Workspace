# Workspace Continuity Engine (Phase 5 Batch 1)

| Field | Value |
|-------|-------|
| **Purpose** | Explain where work left off, what changed, and what naturally resumes |
| **Status** | Foundation — read model only |
| **Owner** | Lead Software Engineer |

---

## Permanent rule

```
Human Intent
  → Workspace Intelligence
  → Decision Queue
  → Prepare Intent
  → Command Pipeline
  → Permission Gateway
  → Execution
  → Audit
```

Continuity never executes, grants permissions, or stores a second payload table.

---

## Architectural fit

```
WorkflowContext          → current project/task
Decision Queue           → what needs attention
Activity Graph           → how work connects
Continuity Engine        → where you left off / what changed / resume narrative
Workspace Intelligence   → consumes Continuity (+ DQ + AG)
Assistant                → same Continuity model (explain only)
```

---

## What this is

| Is | Is not |
|----|--------|
| Read-only Continuity Engine | A second source of truth |
| Projection of existing systems | Scheduler / worker / autonomous resume |
| Explainable facets (why / evidence / what changed) | Hidden workflows |
| Work / session continuity | Runtime diagnostic continuity / archive (`RuntimeDiagnosticContinuityRecord`, `RuntimeDiagnosticArchive`) |

Restart survival = durable Phase 4 sources regenerate Continuity on read.

---

## Facets

Current Focus · Interrupted Work · Resumable Work · Outstanding Decisions · Dormant Projects · Active Commitments · Recent Progress · Recent Outcomes · Blockers · Suggested Next Step

---

## Audits (`authority_effect: none`)

- `workspace.continuity.generated`
- `workspace.continuity.summary.generated`

---

## Related

- [Workspace Platform Coherence](WORKSPACE-PLATFORM-COHERENCE.md)
- [Workspace Runtime Context](WORKSPACE-RUNTIME-CONTEXT.md) (diagnostic provenance/continuity ≠ this engine)
- [Workspace Vocabulary](WORKSPACE-VOCABULARY.md)
- [Workspace Activity Graph](WORKSPACE-ACTIVITY-GRAPH.md)
- [Governed Decision Queue](GOVERNED-DECISION-QUEUE.md)
