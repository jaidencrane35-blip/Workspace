# Workspace Adaptation Review & Change Authority

Sprint 141 — who can approve adaptation, and how future changes stay governed.

**No automatic learning. No self-approval. No apply-from-approval. No Gateway bypass.**

## Principle

```
OutcomeAdaptationProposal
    ↓
Pending Review (explicit reviewer)
    ↓
Approved | Rejected | Expired
    ↓
[Approved only] Controlled Change Surface (architecture)
    ↓
Versioned Behaviour (future — not runtime cognition mutation)
```

Approval unlocks a **future** controlled change path. It does **not** execute, grant
capabilities, or retune Attention/Decision scores.

---

## Reviewer identity

| Field | Rule |
|-------|------|
| `actor_id` | Explicit human reviewer id (e.g. `local_user`) |
| `actor_type` | Must be `local_user` for approve / reject |

Forbidden reviewers:

- The proposal itself (`actor_id == proposal.id`)
- The proposer (`system:outcome_adaptation` / `system`)
- AI / Automation / Plugin as adaptation approvers (Sprint 141)

Self-approval is always forbidden (`may_self_approve() == false`).

---

## Approval lifecycle

```
Proposed
    ↓
Pending Review          (wire: awaiting_review)
    ↓
Reviewed (optional)     (audit acknowledgement)
    ↓
Approved | Rejected | Expired
    ↓
Applied                 (future only — blocked today)
    ↓
Evaluated               (future only — after Applied)
```

| State | Meaning |
|-------|---------|
| Proposed | Created from `RecommendationOutcome` |
| Pending Review | Submitted; awaits human |
| Reviewed | Optional acknowledgement before Approved |
| Approved | Approved for Controlled Change Surface / handoff — **not Applied** |
| Rejected | Explicit rejection; cannot apply |
| Expired | Timed out / expired; cannot approve or apply |
| Applied | Future only — `attempt_mark_applied` hard-fails |
| Evaluated | Future only — after Applied |

Wire names keep Sprint 140 compatibility: `awaiting_review`, `approved_for_handoff`.

---

## Rejection & expiry

| Path | Effect |
|------|--------|
| Reject | Sets `rejection_reason`, audit `rejected`; `attempt_apply_after_rejection` fails |
| Expire | Terminal without apply; further approve/apply fails |

Rejected and expired proposals never enter Controlled Change Surface.

---

## Audit requirements

Every review transition that decides fate must record:

| Event | When |
|-------|------|
| `submitted_for_review` | Enter Pending Review |
| `approved` | Human approve |
| `rejected` | Human reject (+ reason) |
| `expired` | Expiry |

Audit events are **governance records**, not `CapabilityGrant`s and not Permission
Gateway decisions. Fields: `at`, `action`, `actor_id`, optional `note`.

---

## Audit of existing approval systems (reusable patterns)

| System | Pattern to reuse | Must not conflate |
|--------|------------------|-------------------|
| **Permission approvals** (`PermissionApprovalRequest` / `DecideApproval`) | LocalUser decides; pending → approved/denied; durable audit | Grants **capability** for command retry — execution authority |
| **Capability grants** | Allow-once / lasting; consumable; command-scoped | Adaptation approve ≠ grant |
| **Decision Queue / Decision Engine approvals** | Human inbox; accept → Intent handoff only | Recommendation accept ≠ adaptation apply |
| **User preference changes** | Explicit user-authored prefs | Not outcome-driven silent learning |
| **Workspace Adaptation accept** (Pattern/OS) | Review → accept → Intent handoff | Same non-execute rule; separate aggregator type |
| **Automation contract approval** | Definition approval ≠ capability grant | Same separation for adaptation |

**Reusable governance patterns:** explicit LocalUser actor, pending/decided states,
audit trail, accept ≠ execute, Gateway remains sole execution Allow path.

**Separation:** Adaptation review authority is **change-governance**, not Permission
Gateway authority. Approving adaptation never issues `CapabilityGrant`.

---

## Future change application boundary

Architecture only (Sprint 141–142 — not implemented as runtime mutation):

```
Approved Adaptation
    ↓
ControlledChangeSurface
    ↓
BehaviourVersion (Draft)
    ↓
ChangeEvaluation (observational)
    ↓
Versioned Behaviour (future publish via Gateway)
```

See [WORKSPACE-CONTROLLED-CHANGE.md](./WORKSPACE-CONTROLLED-CHANGE.md).

Rules:

- `ControlledChangeSurface::attempt_apply()` → hard-fail
- `may_mutate_runtime_cognition() == false`
- `may_bypass_permission_gateway() == false`
- Observation / WorkspaceState / Attention scoring / Decision scoring unchanged
- Any later real apply must still pass Intent → Command Pipeline → Permission Gateway
  when the change is privileged

---

## Domain types

| Type | Role |
|------|------|
| `OutcomeAdaptationProposal` | Proposal + review lifecycle |
| `AdaptationReviewerIdentity` | Who may approve/reject |
| `AdaptationReviewAuditEvent` | Review audit trail |
| `ControlledChangeSurface` | Future apply boundary (non-mutating) |
| `BehaviourVersion` | Versioned behaviour draft (Sprint 142) |
| `ChangeEvaluation` | Observational evaluation (Sprint 142) |

Guards:

- `may_self_approve() == false`
- `may_execute_from_approval() == false`
- `attempt_self_approve()` / `attempt_apply()` / `attempt_mark_applied()` hard-fail
- Rejected → `RejectedAdaptationCannotApply`

---

## Related docs

- [WORKSPACE-CONTROLLED-CHANGE.md](./WORKSPACE-CONTROLLED-CHANGE.md)
- [WORKSPACE-ADAPTATION-GOVERNANCE.md](./WORKSPACE-ADAPTATION-GOVERNANCE.md)
- [WORKSPACE-ADAPTATION-PROPOSAL.md](./WORKSPACE-ADAPTATION-PROPOSAL.md)
- [WORKSPACE-RECOMMENDATION-OUTCOME.md](./WORKSPACE-RECOMMENDATION-OUTCOME.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
- [WORKSPACE-AUTOMATION-READINESS.md](./WORKSPACE-AUTOMATION-READINESS.md)
