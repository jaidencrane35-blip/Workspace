# Governed Automation Contracts (Phase 4 Batch 6)

| Field | Value |
|-------|-------|
| **Purpose** | Durable, inspectable, revocable records of user-approved *future intent* |
| **Status** | Foundation + integrity hardening (Batch 6.5) — definitions only |
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

Approval is bound to a `definition_fingerprint` (intent + trigger + capabilities + scope + project/task).
Material edits invalidate pending/approved consent. Cosmetic name/description edits do not.

Integrity audit: [Governed Automation Contract Integrity Audit](GOVERNED-AUTOMATION-CONTRACT-INTEGRITY-AUDIT.md).

---

## Integration boundary

`PrepareAutomationContractIntent` returns an `AutomationContractIntentRequest` for **active** contracts only:
`status=approved`, `approval_state=approved`, and fingerprint match.

The request includes actor, workspace/project/task context, required capabilities, fingerprint, and audit metadata.
It never launches processes. Callers must still enter Command Pipeline → Permission Gateway.

Triggers (`manual` / `scheduled` / `event` / `pattern`) are stored definitions.
Batch 7 evaluates trigger relevance into Intent Proposals only — see
[Governed Trigger Evaluation](GOVERNED-TRIGGER-EVALUATION.md).

---

## Audits (`authority_effect: none`)

- `automation.contract.created`
- `automation.contract.updated`
- `automation.contract.approval.requested`
- `automation.contract.approved`
- `automation.contract.revoked`
- `automation.contract.paused`
- `automation.contract.intent.prepared`

---

## Explicit non-goals (this batch)

- Background schedulers / workers
- Pattern monitoring services
- Autonomous execution
- Automation-specific permission systems
- Agent UI
