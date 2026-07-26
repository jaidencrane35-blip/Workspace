# Workspace Governance Policy & Human Review

Sprint 144 — policy layer that governs how adaptation changes are reviewed.

**Policy does not execute. Reviewers cannot bypass provenance. Gateway remains separate.**

## Principle

```
Change
    ↓
Risk Classification (Sprint 145)
    ↓
GovernancePolicy
    ↓
GovernanceReviewDecision (human)
    ↓
GovernanceRecord (ledger)
    ↓
PublishRequest
```

See [WORKSPACE-GOVERNANCE-RISK.md](./WORKSPACE-GOVERNANCE-RISK.md).

Permission / CapabilityBound policies remain the **execution** policy path.
Adaptation GovernancePolicy is the **change-review** policy path.

---

## GovernancePolicy architecture

| Field | Meaning |
|-------|---------|
| `governed_domain` | e.g. `outcome_adaptation` |
| `risk_classification` | `low` / `medium` / `high` |
| `reviewer_requirements` | Actor type, min reviewers, no self-approval, rationale required |
| `approval_threshold` | Count of Approve decisions required |
| `expiry_rules` | Proposal / review / publish TTLs; expired blocks approval |

Default: `GovernancePolicy::for_outcome_adaptation()` — medium risk, 1 LocalUser reviewer,
rationale required, self-approval forbidden, `authority_effect: none`.

Guards:

- `may_execute() == false`
- `may_grant_execution_authority() == false`
- `may_bypass_permission_gateway() == false`
- `enforce_approval_requirements` → threshold / reviewer / rationale / expiry

---

## GovernanceReviewDecision

| Field | Meaning |
|-------|---------|
| `reviewer` | `AdaptationReviewerIdentity` (local_user) |
| `decision` | `approve` / `reject` / `request_changes` |
| `rationale` | Required when policy says so |
| `timestamp` | Decision time |
| `conditions` | Optional constraints (e.g. no scoring mutation) |
| `provenance_snapshot` | Frozen clone — must match ledger provenance |

Guards:

- `may_bypass_provenance() == false`
- `attempt_bypass_provenance()` hard-fails
- `attempt_execute()` hard-fails

---

## Audit of existing policy systems

| System | Role | Boundary vs adaptation policy |
|--------|------|-------------------------------|
| **PermissionPolicy / CapabilityBoundPolicy** | Gateway Allow/Deny from actor capabilities | **Execution authority** — must not be reused to auto-approve adaptation |
| **PolicyContext / PolicyDecision** | Intent + capability + resource evaluation | Reusable shape (context → decision); different domain |
| **GovernanceClass / `read_is_governed`** | Read path governance for non-LocalUser | Reusable “governed vs ungoverned” idea; not change publication |
| **Intent policies** (intent type on commands) | Classify privileged intents | Separate — Intent → Pipeline → Gateway |
| **Observation admission / refresh policies** | Rate-limit observation triggers | Separate — observation only |
| **Workspace / confidence / memory policies** (docs) | Product guidance | Not runtime adaptation approval |
| **Capability grants** | Post-`DecideApproval` execution grants | **Forbidden** as adaptation approve side-effect |

**Reusable patterns:** explicit evaluator, actor-typed decisions, deny-by-default thresholds,
rationale/metadata on decisions.

**Hard boundary:** Adaptation GovernancePolicy never calls Gateway Allow and never mints
`CapabilityGrant`.

---

## Connection to ledger

```
GovernancePolicy
    ↓
GovernanceReviewDecision
    ↓
GovernanceRecord.with_policy_and_decisions(...)
    ↓
PublishRequest.from_evaluated_version_under_policy(...)
```

Ledger fields added (Sprint 144):

- `policy_reference`
- `review_decision_references`

Publish approval can re-check policy via `approve_for_publish_under_policy`.

---

## Related docs

- [WORKSPACE-GOVERNANCE-RISK.md](./WORKSPACE-GOVERNANCE-RISK.md)
- [WORKSPACE-GOVERNANCE-LEDGER.md](./WORKSPACE-GOVERNANCE-LEDGER.md)
- [WORKSPACE-CONTROLLED-CHANGE.md](./WORKSPACE-CONTROLLED-CHANGE.md)
- [WORKSPACE-ADAPTATION-REVIEW.md](./WORKSPACE-ADAPTATION-REVIEW.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
- [WORKSPACE-AUTOMATION-READINESS.md](./WORKSPACE-AUTOMATION-READINESS.md)
