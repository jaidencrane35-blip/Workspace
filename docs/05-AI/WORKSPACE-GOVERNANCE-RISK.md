# Workspace Governance Risk Classification & Review Routing

Sprint 145 — how governed changes receive appropriate review requirements before publication.

**Risk does not execute. Risk metadata does not mutate cognition. Gateway stays separate.**

## Principle

```
Change (OutcomeAdaptationProposal / ControlledChangeSurface)
    ↓
Risk Classification (GovernanceRisk)
    ↓
Governance Policy (from risk)
    ↓
Required Reviewers
    ↓
GovernanceReviewDecision
```

High-risk changes require **stronger** review (more reviewers, conditions). Forbidden impact
classes (cognition scoring mutation, execution authority) are rejected at routing time.

---

## GovernanceRisk architecture

| Field | Meaning |
|-------|---------|
| `risk_level` | `low` / `medium` / `high` (`AdaptationRiskClass`) |
| `affected_domain` | Change area (from proposal) |
| `impact_classification` | What kind of impact is proposed |
| `review_requirements` | Min reviewers, local_user, no self-approval, rationale |
| `approval_threshold` | Approving decisions required |
| `requires_conditions` | High-risk must include decision conditions |

### Impact classes

| Class | Routing |
|-------|---------|
| `presentation_only` | Low — standard review |
| `workflow_hint` | Medium — standard review |
| `behaviour_version_draft` | High — stronger review |
| `cognition_scoring_mutation` | **Rejected** — cannot mutate cognition |
| `execution_authority` | **Rejected** — Gateway-only path |

### High-risk stronger review

| Setting | High | Low/Medium |
|---------|------|------------|
| `min_reviewers` | 2 | 1 |
| `approval_threshold` | 2 | 1 |
| `requires_conditions` | true | false |

Guards:

- `may_execute() == false`
- `may_mutate_cognition() == false`
- `attempt_mutate_cognition()` hard-fails
- `enforce_not_cognition_mutation()` rejects forbidden impacts

---

## Review routing

`GovernanceReviewRouting::route_change(proposal, optional_risk)`:

1. Classify risk (or use provided)
2. Reject forbidden cognition/execution impacts
3. Build `GovernancePolicy::from_risk`
4. Expose `required_reviewer_count` / actor type
5. `enforce_decisions` checks policy threshold + high-risk conditions + provenance

---

## Audit of existing risk concepts

| System | Pattern | Boundary |
|--------|---------|----------|
| **ActionProposalRisk** | Informational risk metadata on proposals | Not review routing; no authority |
| **Permission / capability sensitivity** | Empty defaults for AI/Automation/Plugin; LocalUser standard set | **Execution** sensitivity — not adaptation review tiers |
| **Command `GovernanceClass`** | Governed vs ungoverned read/command paths | Reusable binary “governed” idea; not low/medium/high change review |
| **Automation readiness classes (A–D)** | Observation / recommendation / gated / unsafe | Reusable taxonomy mindset; adaptation risk is change-review only |
| **Security RISK-REGISTER** | Project delivery risks | Ops/process — not runtime cognition |

**Reusable:** explicit classification, deny-by-default for dangerous classes, escalate
requirements with severity.

**Preserve:** Permission Gateway remains the only execution Allow path; adaptation risk
never grants capabilities.

---

## Related docs

- [WORKSPACE-GOVERNANCE-EVIDENCE.md](./WORKSPACE-GOVERNANCE-EVIDENCE.md)
- [WORKSPACE-GOVERNANCE-POLICY.md](./WORKSPACE-GOVERNANCE-POLICY.md)
- [WORKSPACE-GOVERNANCE-LEDGER.md](./WORKSPACE-GOVERNANCE-LEDGER.md)
- [WORKSPACE-CONTROLLED-CHANGE.md](./WORKSPACE-CONTROLLED-CHANGE.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
- [WORKSPACE-AUTOMATION-READINESS.md](./WORKSPACE-AUTOMATION-READINESS.md)
