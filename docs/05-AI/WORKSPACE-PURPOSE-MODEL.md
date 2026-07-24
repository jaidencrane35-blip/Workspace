# Workspace Purpose Model

| Field | Value |
|-------|-------|
| **Purpose** | Read-only model of why Workspace work exists — durable meaning behind outcomes |
| **Owner** | Architecture |
| **Status** | Phase 5 Batch 7 foundation |
| **Authority** | Never creates authority (`authority_effect: none`) |

---

## Principle

Purpose is not execution intent, not an AI command, and not authorization.

Purpose is the durable meaning behind work — projected from existing models.

```
WorkGoal (DurableStore SoT)
  + Project / WorkflowContext
  + Task Graph + Composition + Continuity + Activity + Decision Queue
        ↓
Workspace Purpose Model   ← this document
        ↓
Attention / Intelligence / Assistant consumers
```

---

## Ownership

| Concept | Kind | Owner |
|---------|------|-------|
| Declared goals | DurableStore | `WorkspaceIntentService` (`work_goals`) |
| Purpose projection | Aggregator | `WorkspacePurposeService` |

Purpose does **not** replace Projects, Tasks, WorkGoals, Workflow, Composition, or Continuity.

---

## Represents

- Outcome label (prefer active WorkGoal → active Project → Composition label)
- Supporting projects, compositions, Task Graph nodes
- Progress and recent completed work
- Obstacles (blocked tasks, interrupted Continuity, outstanding decisions, composition gaps)
- Explainable relationships with evidence

---

## Boundaries

May: observe, aggregate, explain, feed Attention / Intelligence.  
Must not: create goals, auto-plan life, profile users, execute, grant permissions, change Gateway behavior.

Audit: `workspace.purpose.generated` with `authority_effect: none`.
