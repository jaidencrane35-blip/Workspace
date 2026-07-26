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

## Determinism (Sprint 128)

Insight ids are `insight:{kind}:{workspace}` — stable across wording changes, since titles
and explanations restate content that ids must not depend on. Insights are sorted by id;
events by timestamp then id.

Evolution reads the Activity Graph through `cognitive_timeline()`, which excludes
`AuditSignal` entries **before** the window is taken. Filtering inside a `take(n)` loop was
the bug: one Intelligence generate writes ~50 audit events, those became the newest
timeline entries, and they filled the whole window on the next evaluation. Real work fell
out of view and the insight set changed — most visibly, Purpose lost `recent_progress`, so
`insight:purpose_progression` stopped being emitted.

Evaluation telemetry is history, not a cognitive source fact. It stays in the timeline and
in the audit log; it just never enters an analysis window.

---

## Boundaries

May: observe, aggregate, explain, feed Attention / Intelligence.  
Must not: predict, profile, store a second history, execute, grant permissions, change Gateway behavior.

Audit: `workspace.evolution.generated` with `authority_effect: none`.
