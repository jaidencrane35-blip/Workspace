# Governed Decision Queue (Phase 4 Batch 8)

| Field | Value |
|-------|-------|
| **Purpose** | Single Workspace inbox for governed decisions requiring human attention |
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

Nothing bypasses the Command Pipeline or Permission Gateway.

---

## What this is

| Is | Is not |
|----|--------|
| Workspace Decision Queue | An AI Inbox |
| Aggregation of existing sources | A duplicate source of truth |
| Ordered human attention | A scheduler / worker / execution queue |
| Lifecycle overlay (viewed/deferred/dismissed) | Permission or capability authority |

---

## Source adapters

| Source type | Source of truth |
|-------------|-----------------|
| `intent_proposal` | Automation Intent Proposals (`pending_review`) |
| `pending_approval` | Permission approvals + contract definition pending approval |
| `blocked_action` | Denied/failed orchestrated plan steps |
| `planning_continuation` | Non-terminal plans + assistant workflows |

Adapters never copy payload into a second authority store. Lifecycle overlay stores only `(workspace, source_type, source_id) → state`.

---

## Lifecycle

States: `pending`, `viewed`, `deferred`, `dismissed`, `accepted`, `rejected`, `expired`.

- **Dismiss** — overlay only; source unchanged  
- **Accept (proposal)** — delegates to TriggerEvaluator accept  
- **Accept (permission)** — returns handoff to `decide_approval` (queue cannot approve)  
- **Blocked / planning** — handoff to owning subsystem  

---

## Surfaces

| Surface | May | Must not |
|---------|-----|----------|
| Work tab Decision Queue | View / defer / dismiss; accept/reject where delegated | Execute or grant |
| Workspace Intelligence | Display counts + items | Mutate |
| Assistant | Explain / summarize / prioritize | Accept / reject / defer / execute |

---

## Audits (`authority_effect: none`)

- `decision.queue.generated`
- `decision.item.created`
- `decision.item.viewed`
- `decision.item.deferred`
- `decision.item.dismissed`

---

## Related

- [Governed Trigger Evaluation](GOVERNED-TRIGGER-EVALUATION.md)
- [Governed Automation Contracts](GOVERNED-AUTOMATION-CONTRACTS.md)
- [IPC Surface](../03-Engineering/IPC-SURFACE.md)
