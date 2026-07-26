# Workspace Adaptation Proposal

| Field | Value |
|-------|-------|
| **Purpose** | Read-only proposals for possible Workspace improvements |
| **Owner** | Architecture |
| **Status** | Phase 5 Batch 12 foundation (Sprint 91) |
| **Authority** | Never creates authority (`authority_effect: none`) |

---

## Principle

Adaptation is a proposal.

It is not execution, automation, permission, or autonomous improvement.

```
Pattern + Recommendation Engine + Operating State
  + Composition + Environment + Continuity + Purpose
        ↓
Adaptation Proposal   ← this document
        ↓
Human Decision (review / accept / reject)
        ↓
Intent → Planning → Command Pipeline → Permission Gateway → Execution → Audit
```

Accept returns an Intent handoff only. Adaptation never skips governance.

**Sprint 139:** `RecommendationOutcome` may inform future Adaptation as evidence only —
never auto-applies or rescoring. See
[WORKSPACE-RECOMMENDATION-OUTCOME.md](./WORKSPACE-RECOMMENDATION-OUTCOME.md).

**Sprint 140:** Adaptation governance —
[WORKSPACE-ADAPTATION-GOVERNANCE.md](./WORKSPACE-ADAPTATION-GOVERNANCE.md).
`OutcomeAdaptationProposal` requires explicit review; never silent scoring mutation.

**Sprint 141:** Adaptation review & change authority —
[WORKSPACE-ADAPTATION-REVIEW.md](./WORKSPACE-ADAPTATION-REVIEW.md).
LocalUser reviewer required; self-approval forbidden; approve ≠ apply ≠ Gateway grant.

**Sprint 142:** Controlled change & evaluation —
[WORKSPACE-CONTROLLED-CHANGE.md](./WORKSPACE-CONTROLLED-CHANGE.md).
BehaviourVersion drafts + ChangeEvaluation; no runtime cognition mutation.

**Sprint 143:** Governance ledger & publication contract —
[WORKSPACE-GOVERNANCE-LEDGER.md](./WORKSPACE-GOVERNANCE-LEDGER.md).
Full chain traceability; publish requests never activate runtime.

**Sprint 144:** Governance policy & human review —
[WORKSPACE-GOVERNANCE-POLICY.md](./WORKSPACE-GOVERNANCE-POLICY.md).
Policy → Review Decision → Ledger → PublishRequest; never execution authority.

**Sprint 145:** Governance risk classification & review routing —
[WORKSPACE-GOVERNANCE-RISK.md](./WORKSPACE-GOVERNANCE-RISK.md).
High-risk requires stronger review; cognition mutation impacts rejected.

**Sprint 146:** Governance decision evidence & publication readiness —
[WORKSPACE-GOVERNANCE-EVIDENCE.md](./WORKSPACE-GOVERNANCE-EVIDENCE.md).
Evidence required for approval; readiness never activates runtime.

**Sprint 147:** Governance workspace & publication environment —
[WORKSPACE-GOVERNANCE-WORKSPACE.md](./WORKSPACE-GOVERNANCE-WORKSPACE.md).
Human operating surface + publication boundary; UI cannot approve execution.

**Sprint 148:** Governance lifecycle integrity & timeline —
[WORKSPACE-GOVERNANCE-LIFECYCLE.md](./WORKSPACE-GOVERNANCE-LIFECYCLE.md).
End-to-end stages preserve provenance; timeline events immutable.

**Sprint 149:** Publication safety contract —
[WORKSPACE-PUBLICATION-SAFETY.md](./WORKSPACE-PUBLICATION-SAFETY.md).
Validation/migration/rollback gates; activation still blocked.

**Sprint 150:** Governance resilience & failure contract —
[WORKSPACE-GOVERNANCE-FAILURE-HANDLING.md](./WORKSPACE-GOVERNANCE-FAILURE-HANDLING.md).
Failure/recovery lifecycle; never deletes provenance or grants authority.

**Sprint 151:** Governance conditions & obligations —
[WORKSPACE-GOVERNANCE-CONDITIONS.md](./WORKSPACE-GOVERNANCE-CONDITIONS.md).

**Sprint 152:** Governance compatibility & dependencies —
[WORKSPACE-GOVERNANCE-COMPATIBILITY.md](./WORKSPACE-GOVERNANCE-COMPATIBILITY.md).

**Sprint 153:** Governance integrity verification —
[WORKSPACE-GOVERNANCE-INTEGRITY.md](./WORKSPACE-GOVERNANCE-INTEGRITY.md).

**Sprint 154:** Governance archive & historical preservation —
[WORKSPACE-GOVERNANCE-ARCHIVE.md](./WORKSPACE-GOVERNANCE-ARCHIVE.md).

**Sprint 155–159:** Review workflow, conflict, decision package, compliance, dashboard —
[WORKSPACE-GOVERNANCE-REVIEW-WORKFLOW.md](./WORKSPACE-GOVERNANCE-REVIEW-WORKFLOW.md),
[WORKSPACE-GOVERNANCE-CONFLICT.md](./WORKSPACE-GOVERNANCE-CONFLICT.md),
[WORKSPACE-GOVERNANCE-DECISION-PACKAGE.md](./WORKSPACE-GOVERNANCE-DECISION-PACKAGE.md),
[WORKSPACE-GOVERNANCE-COMPLIANCE.md](./WORKSPACE-GOVERNANCE-COMPLIANCE.md),
[WORKSPACE-GOVERNANCE-DASHBOARD.md](./WORKSPACE-GOVERNANCE-DASHBOARD.md).

**Sprint 160–164:** Notifications, delegation, metrics, reporting, export —
[WORKSPACE-GOVERNANCE-NOTIFICATIONS.md](./WORKSPACE-GOVERNANCE-NOTIFICATIONS.md),
[WORKSPACE-GOVERNANCE-DELEGATION.md](./WORKSPACE-GOVERNANCE-DELEGATION.md),
[WORKSPACE-GOVERNANCE-METRICS.md](./WORKSPACE-GOVERNANCE-METRICS.md),
[WORKSPACE-GOVERNANCE-REPORTING.md](./WORKSPACE-GOVERNANCE-REPORTING.md),
[WORKSPACE-GOVERNANCE-EXPORT.md](./WORKSPACE-GOVERNANCE-EXPORT.md).

---

## Ownership

| Concept | Kind | Owner |
|---------|------|-------|
| Adaptation proposal | Aggregator | `WorkspaceAdaptationService` |

Distinct from Recommendation Engine (next-step suggestions) and Decision Engine (planner candidates).

---

## Lifecycle

`proposed` → `reviewed` → `accepted` | `rejected`

- **Review** required before accept
- **Accept** → `submit_assistant_goal` handoff (never executes)
- **Reject** → audit only; workspace unchanged

Audits: `workspace.adaptation.proposal_generated`, `.reviewed`, `.accepted`, `.rejected` — all `authority_effect: none`.

IPC: `generate_workspace_adaptation`, `review_adaptation_proposal`, `accept_adaptation_proposal`, `reject_adaptation_proposal`
