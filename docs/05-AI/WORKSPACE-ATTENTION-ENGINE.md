# Workspace Attention Engine (Phase 5 Batch 2)

| Field | Value |
|-------|-------|
| **Purpose** | Deterministic, explainable prioritization of existing Workspace information |
| **Status** | Foundation — read model only |
| **Owner** | Lead Software Engineer |

---

## Permanent rule

Attention is informational. Attention grants nothing, authorizes nothing, executes nothing.

```
Human Intent → … → Permission Gateway → Execution → Audit
```

---

## Stack position

```
WorkflowContext / Decision Queue / Activity Graph / Continuity
  → Attention Engine (canonical prioritization)
  → Workspace Intelligence / Assistant
```

No new persistence. Synthetic ids: `attention:{source_type}:{source_id}`.

---

## Scoring

Deterministic integer scores with `score_factors` explanations:

| Source | Base guidance |
|--------|----------------|
| Blockers | high / immediate |
| Outstanding decisions | high / soon |
| Interrupted / commitments | medium |
| Resumable / current focus | normal |
| Dormant / recent progress | low / can wait |

Intelligence Recommendations are projected from Attention top items.

---

## Audits (`authority_effect: none`)

- `workspace.attention.generated`
- `workspace.attention.summary.generated`

---

## Related

- [Workspace Continuity Engine](WORKSPACE-CONTINUITY-ENGINE.md)
- [Workspace Platform Coherence](WORKSPACE-PLATFORM-COHERENCE.md)
- [Governed Decision Queue](GOVERNED-DECISION-QUEUE.md)
