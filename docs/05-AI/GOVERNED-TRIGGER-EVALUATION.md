# Governed Trigger Evaluation (Phase 4 Batch 7)

| Field | Value |
|-------|-------|
| **Purpose** | Evaluate when an approved Automation Contract may be relevant and emit Intent Proposals |
| **Status** | Foundation — evaluation only, no execution |
| **Owner** | Lead Software Engineer |

---

## Permanent rule

```
Stored Intent
  → Trigger Evaluation
  → Intent Proposal
  → Command Pipeline
  → Permission Gateway
  → Execution
```

Nothing bypasses the Command Pipeline or Permission Gateway.

---

## What this batch is

| Is | Is not |
|----|--------|
| `TriggerEvent` informational objects | A scheduler / worker / background agent |
| `TriggerEvaluator` validation pipeline | Automatic execution |
| `AutomationIntentProposal` suggestions | Silent approvals |
| Explainable rejection reasons | Hidden monitoring |

Trigger evaluation is intelligence. Execution remains governed.

---

## TriggerEvent

Represents: “Something happened that may be relevant to an Automation Contract.”

Types include `application_opened`, `workspace_changed`, `project_context_changed`,
`task_state_changed`, `user_requested_evaluation`, `manual_evaluation_requested`.

Always `authority_effect: none`.

---

## Validation pipeline

```
Trigger Event
  → Find Candidate Contracts
  → Validate Contract State
  → Validate Approval Binding
  → Validate Trigger Definition
  → Validate Scope
  → Validate Capabilities
  → Generate Intent Proposal
```

Example rejections:

- Contract paused.
- Contract revoked.
- Approval fingerprint no longer matches.
- Workspace / project / task scope mismatch.
- Required capability unavailable.

---

## Intent proposals

`AutomationIntentProposal` means a governed contract appears relevant.
Statuses: `pending_review`, `accepted`, `rejected`, `expired`.

Accepting a proposal is **not** execution authority. Callers must still:

1. Prepare contract intent (active definition + fingerprint)
2. Enter Command Pipeline
3. Pass Permission Gateway

---

## Integration boundaries

| Surface | May | Must not |
|---------|-----|----------|
| TriggerEvaluator | Record events, evaluate, propose | Execute, approve contracts, grant capabilities |
| Workspace Intelligence | Display proposals / rejections | Accept, reject, execute |
| Assistant | Explain matches and reasons | Silently approve or execute |

---

## Audits (`authority_effect: none`)

- `automation.trigger.received`
- `automation.trigger.evaluated`
- `automation.trigger.rejected`
- `automation.intent_proposal.created`
- `automation.intent_proposal.accepted`
- `automation.intent_proposal.rejected`

Execution audits remain owned by existing execution governance.

---

## Related

- [Governed Automation Contracts](GOVERNED-AUTOMATION-CONTRACTS.md)
- [Governed Automation Contract Integrity Audit](GOVERNED-AUTOMATION-CONTRACT-INTEGRITY-AUDIT.md)
- [IPC Surface](../03-Engineering/IPC-SURFACE.md)
