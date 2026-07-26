# Workspace Governance Ledger & Change Publication

Sprint 143 — complete historical governance traceability before any change publication exists.

**No automatic learning. No runtime activation. Governance never grants execution authority.**

## Principle

```
Evidence → Reasoning → Recommendation → Experience → Decision → Outcome
    → Adaptation Proposal → Review → Change → Evaluation → Publication (contract only)
```

Publication is architecture-only:

```
Evaluated BehaviourVersion
    ↓
PublishRequest
    ↓
Governance Review
    ↓
Published Version (ledger label — not runtime-active)
```

`PublishedVersionRecord::attempt_from_publish_request` and
`PublishRequest::attempt_activate_published_version` hard-fail. Runtime cognition stays unchanged.

---

## GovernanceRecord (ledger model)

| Field | Meaning |
|-------|---------|
| `proposal_reference` | `OutcomeAdaptationProposal` id |
| `review_reference` | Submit-for-review audit key |
| `approval_reference` | Approval key when approved |
| `change_reference` | Controlled change / surface key |
| `version_reference` | `BehaviourVersion` id |
| `evaluation_reference` | `ChangeEvaluation` id |
| `rollback_reference` | Rollback draft version id (if any) |
| `actors` | Proposer / reviewer / publisher ids |
| `timestamps` | Proposed → … → publish_requested; `published_at` stays `None` |
| `provenance` | Frozen recommendation provenance |

Guards:

- `may_grant_execution_authority() == false`
- `attempt_grant_execution_authority()` → `GovernanceCannotGrantAuthority`
- `authority_effect: none`

Builder: `GovernanceRecord::from_adaptation_chain(...)`.
Policy attachment: `with_policy_and_decisions` — see
[WORKSPACE-GOVERNANCE-POLICY.md](./WORKSPACE-GOVERNANCE-POLICY.md) (Sprint 144).

Ledger fields (Sprint 144): `policy_reference`, `review_decision_references`.

---

## Publish contract

| Type | Role |
|------|------|
| `PublishRequest` | Request to publish an evaluated draft version |
| `PublishRequestStatus` | Proposed → AwaitingGovernanceReview → ApprovedForPublish \| Rejected |
| `PublishedVersionRecord` | Architecture marker — **cannot be created** in Sprint 143 |

Rules:

- `from_evaluated_version` requires governance `approval_reference` + matching evaluation
- Without approval → `PublicationRequiresApproval`
- Approve-for-publish still does **not** activate runtime
- Activation → `PublicationActivationNotImplemented`
- Publish path never bypasses Permission Gateway

---

## Audit of existing audit systems

| System | Pattern | Reuse for ledger |
|--------|---------|------------------|
| **AuditService** | Durable `AuditEvent` append; command / permission / AI planning events | **Reusable:** actor + timestamp + event_type + metadata; append-only |
| **Permission audit** (`permission.allowed` / `.denied` / `.approval_required`) | Gateway decisions with capability | **Separate:** execution authority — ledger must not mint these |
| **Decision / AI planning events** | `record_ai_planning_event` (ids only, no chain-of-thought) | **Reusable:** reference ids without mutating cognition |
| **Experience traces** | Translation diagnostics (`match_key` / DisplayReason) | **Reusable:** provenance attachment — not authority |
| **Command history** (`command.executed` / `.failed`) | Post-pipeline outcomes | **Separate:** only after Gateway Allow |
| **Adaptation review audit_events** | In-proposal review trail | **Direct input** into GovernanceRecord review/approval refs |

**Finding:** Append-only event patterns and reference-id metadata are the right shape for a
governance ledger. Permission and command audits remain the execution trail; the governance
ledger is the **adaptation change** trail. They must not be conflated.

---

## Complete provenance chain

```
Evidence
    ↓
Reasoning
    ↓
Recommendation (+ Identity / Lifecycle)
    ↓
Experience (DisplayReason / traces)
    ↓
Decision (human)
    ↓
RecommendationOutcome
    ↓
OutcomeAdaptationProposal
    ↓
Review Authority
    ↓
ControlledChangeSurface
    ↓
BehaviourVersion (Draft)
    ↓
ChangeEvaluation
    ↓
GovernanceRecord (ledger)
    ↓
PublishRequest → Governance Review → [future] Published Version
```

Every ledger entry retains frozen provenance. Rollback drafts clone provenance; they do not
rewrite history. Publication does not activate Attention/Decision scoring.

---

## Related docs

- [WORKSPACE-GOVERNANCE-POLICY.md](./WORKSPACE-GOVERNANCE-POLICY.md)
- [WORKSPACE-CONTROLLED-CHANGE.md](./WORKSPACE-CONTROLLED-CHANGE.md)
- [WORKSPACE-ADAPTATION-REVIEW.md](./WORKSPACE-ADAPTATION-REVIEW.md)
- [WORKSPACE-ADAPTATION-GOVERNANCE.md](./WORKSPACE-ADAPTATION-GOVERNANCE.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
- [WORKSPACE-AUTOMATION-READINESS.md](./WORKSPACE-AUTOMATION-READINESS.md)
