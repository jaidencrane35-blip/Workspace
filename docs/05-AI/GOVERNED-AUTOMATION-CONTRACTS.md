# Governed Automation Contracts (Phase 4 Batch 6)

| Field | Value |
|-------|-------|
| **Purpose** | Durable, inspectable, revocable records of user-approved *future intent* |
| **Status** | Foundation complete — definitions only, no autonomous execution |
| **Owner** | Lead Software Engineer |

---

## Permanent rule

```
Human Intent
  → Workspace Intelligence
  → Planning / Evaluation
  → Command Pipeline
  → Permission Gateway
  → Execution
```

An Automation Contract remembers what the user approved as a **definition**.
It does **not** decide what the Workspace is allowed to do at execution time.

---

## What a contract is

| Is | Is not |
|----|--------|
| Durable work-context object attached to Project (+ optional Task) | An agent |
| Trigger + intent **definitions** | A background worker |
| Separate `status` and `approval_state` | A capability grant |
| Future Intent template | A Permission Gateway bypass |

Attachment:

```
Project → Task? → AutomationContract → (future) Intent request
  → Command Pipeline → Permission Gateway → Execution
```

---

## Lifecycle

| Status | Meaning |
|--------|---------|
| `draft` | Editable definition |
| `pending_approval` | Definition awaiting user approval |
| `approved` | Definition accepted — still not executable authority |
| `paused` | Approved definition temporarily inactive |
| `revoked` | Definition no longer usable |
| `completed` | Closed |

`approval_state` (`not_approved` / `pending` / `approved` / `revoked`) is orthogonal to status.
Approving a definition **must not** create `CapabilityGrant` rows.

---

## Integration boundary

`PrepareAutomationContractIntent` returns an `AutomationContractIntentRequest` for **active** contracts only (`status=approved` and `approval_state=approved`).

It never launches processes. Callers must still enter Command Pipeline → Permission Gateway.

Triggers (`manual` / `scheduled` / `event` / `pattern`) are stored definitions only in Batch 6.

---

## Audits (`authority_effect: none`)

- `automation.contract.created`
- `automation.contract.updated`
- `automation.contract.approval.requested`
- `automation.contract.approved`
- `automation.contract.revoked`
- `automation.contract.paused`

---

## Explicit non-goals (this batch)

- Background schedulers / workers
- Pattern monitoring services
- Autonomous execution
- Automation-specific permission systems
- Agent UI
