# Workspace Evolution Model

| Field | Value |
|-------|-------|
| **Purpose** | Read-only projection of how Workspace work changed over time |
| **Owner** | Architecture |
| **Status** | Phase 5 Batch 8 foundation |
| **Authority** | Never creates authority (`authority_effect: none`) |

---

## Principle

Evolution is not memory, not a second Activity Graph, and not prediction.

Evolution is the explainable history of change across existing Workspace systems.

```
Activity Graph (history SoT)
  + Task Graph + Purpose + Composition + Continuity + Decision Queue
        ↓
Workspace Evolution Model   ← this document
        ↓
Attention / Intelligence / Assistant consumers
```

---

## Ownership

| Concept | Kind | Owner |
|---------|------|-------|
| Activity / timeline | Aggregator (history SoT) | `WorkspaceActivityGraphService` |
| Evolution narrative | Aggregator | `WorkspaceEvolutionService` |

Evolution owns **no persistence**.

---

## Represents

- Change events (what / evidence / impact / when)
- Deterministic insights: task progression, purpose progression, composition shift, interrupted work, decision outcomes, focus change
- Relationships between purpose, tasks, and composition changes

---

## Boundaries

May: observe, aggregate, explain, feed Attention / Intelligence.  
Must not: predict, profile, store a second history, execute, grant permissions, change Gateway behavior.

Audit: `workspace.evolution.generated` with `authority_effect: none`.
