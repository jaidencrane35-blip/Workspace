# Workspace Operating State

| Field | Value |
|-------|-------|
| **Purpose** | Read-only unified snapshot of what is happening in the Workspace right now |
| **Owner** | Architecture |
| **Status** | Phase 5 Batch 10 foundation (Sprint 89) |
| **Authority** | Never creates authority (`authority_effect: none`) |

---

## Principle

Operating State is a snapshot of understanding.

It is not a controller, executor, planner, permission system, or automation system.

```
Environment + Composition + Task Graph + Purpose + Evolution
  + Continuity + Activity + Decision Queue + Attention + Recommendation Engine
        ↓
Workspace Operating State   ← this document
        ↓
Workspace Intelligence / UX / Assistant (explain)
```

Sources remain authoritative. Operating State invents nothing and persists nothing.

---

## Ownership

| Concept | Kind | Owner |
|---------|------|-------|
| Operating State | Aggregator | `WorkspaceOperatingStateService` |

Operating State owns **no durable store**.

---

## Represents

- Current purpose / active project / active work
- Current environment and composition
- Recent progress
- Current blockers and pending decisions
- Attention priorities and recommendation suggestions
- Explainable signals and relationships

---

## Boundaries

May: observe, aggregate, explain, feed Intelligence / Assistant.  
Must not: execute, approve, grant permissions, modify tasks/projects, trigger automation, bypass Permission Gateway, replace source models.

Audits:
- `workspace.operating_state.generated`
- `workspace.operating_state.updated` (ephemeral refresh marker)

Both carry `authority_effect: none`.

IPC: `generate_workspace_operating_state`
