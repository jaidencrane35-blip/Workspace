# Workspace Activity Graph (Phase 4 Batch 9)

| Field | Value |
|-------|-------|
| **Purpose** | Pure aggregation read model linking work objects into timeline and relationships |
| **Status** | Foundation — aggregation only |
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

Nothing bypasses the Command Pipeline or Permission Gateway. The Activity Graph never executes, grants permissions, or becomes a second source of truth.

---

## What this is

| Is | Is not |
|----|--------|
| Workspace Activity Graph | A second entity database |
| Aggregation of existing sources | Authority or permission state |
| Chronological timeline + relationships | A scheduler / worker / execution log of record |
| Synthetic IDs (`activity:{type}:{source_id}`) | Durable activity payload rows |

Restart survival follows Decision Queue CASE 10: durable SQLite sources reappear on regenerate; in-memory plans/workflows are absent until recreated.

Do **not** extend resource `GraphRepository` `Contains` edges — that graph is containment only.

---

## Activity types (initial)

`project`, `task`, `work_goal`, `automation_contract`, `trigger_event`, `intent_proposal`, `decision_item`, `permission_approval`, `planning_continuation`, `blocked_action`, `execution_outcome`, `audit_signal` (gap-fill only when not already represented).

---

## Adapter order

1. Work context — projects / tasks / goals  
2. Automation contracts under project/task  
3. Trigger events + intent proposals  
4. Decision Queue items (thin nodes → underlying source)  
5. Permission approvals (workspace-attributable)  
6. Blocked / planning from in-memory plan/workflow stores  
7. Execution outcomes when workspace-attributable  
8. Audit gap-fill — workspace-tagged events not already covered  

Relationships: nearest durable parent; bidirectional `related_activity_ids` in a second pass. Timeline: `timestamp` ASC then `id` ASC (deterministic).

---

## Surfaces

| Surface | May | Must not |
|---------|-----|----------|
| Work tab Activity view | Refresh / inspect timeline | Execute or grant |
| Workspace Intelligence | Display summary + recent timeline | Mutate |
| Assistant | Explain history and blockers | Mutate or execute |

---

## Audits (`authority_effect: none`)

- `workspace.timeline.generated`
- `workspace.relationship.generated`
- `workspace.activity.related`
- `workspace.activity.summary.generated`

Skip per-node `workspace.activity.created` spam; emit summary audits once per generate.

---

## Related

- [Governed Decision Queue](GOVERNED-DECISION-QUEUE.md)
- [Governed Trigger Evaluation](GOVERNED-TRIGGER-EVALUATION.md)
- [IPC Surface](../03-Engineering/IPC-SURFACE.md)
