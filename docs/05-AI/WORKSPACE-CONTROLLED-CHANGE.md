# Workspace Controlled Change & Evaluation

Sprint 142 — how approved adaptations become governed, versioned, and evaluated changes.

**Do not mutate runtime cognition.** Draft behaviour versions and evaluations are architecture
records only. Permission Gateway remains the sole execution Allow path.

## Principle

```
Approved OutcomeAdaptationProposal
    ↓
ControlledChangeSurface
    ↓
BehaviourVersion (Draft — not runtime-active)
    ↓
ChangeEvaluation (observational)
    ↓
[future] Governance Ledger + PublishRequest (no runtime activation)
```

See [WORKSPACE-GOVERNANCE-LEDGER.md](./WORKSPACE-GOVERNANCE-LEDGER.md) (Sprint 143).

Forbidden:

```
Approved Adaptation
    ↓
silent runtime scoring / cognition mutation   ← NEVER
```

---

## ControlledChangeSurface contract

Required fields:

| Field | Meaning |
|-------|---------|
| `originating_proposal_id` | Source `OutcomeAdaptationProposal` |
| `approved_change` | Human-approved change text |
| `reviewer` | `AdaptationReviewerIdentity` (local_user) |
| `previous_version_id` | Version to roll back to (baseline if first) |
| `target_version_id` | Draft behaviour version id |
| `rollback` | `ChangeRollbackMetadata` (no auto-rollback) |
| `audit` | `ControlledChangeAuditMetadata` |
| `provenance` | Frozen recommendation provenance |

Guards:

- `from_unapproved_proposal` → `UnapprovedControlledChangeForbidden`
- `may_mutate_runtime_cognition() == false`
- `may_bypass_permission_gateway() == false`
- `attempt_apply()` hard-fails

---

## BehaviourVersion architecture

Does **not** mutate runtime. Draft only in Sprint 142.

| Field | Meaning |
|-------|---------|
| `id` | Version identity (`behaviour:from:…` / rollback ids) |
| `creation_source` | e.g. `outcome_adaptation` |
| `change_reason` | Why this version exists |
| `approval_reference` | Originating proposal id |
| `previous_version_id` | Prior version / baseline |
| `lifecycle` | `Draft` → (`Published` / `Superseded` / `RolledBack` — future) |
| `provenance` | Frozen clone from surface |

Rules:

- `from_unapproved_proposal` hard-fails
- `attempt_publish()` → `BehaviourVersionPublishNotImplemented`
- `prepare_rollback_draft` clones provenance (does not rewrite history)
- `may_bypass_permission_gateway() == false`

Baseline id: `behaviour:v0_baseline`.

---

## ChangeEvaluation model

| Field | Meaning |
|-------|---------|
| `change_reference` | Behaviour version id |
| `observed_effects` | What was observed after a future apply |
| `success_criteria` | Expected success measures |
| `rollback_recommendation` | Optional human guidance |
| `provenance_snapshot` | Immutable copy at evaluation time |

Rules:

- `may_rewrite_history() == false`
- `attempt_rewrite_provenance()` hard-fails
- Evaluation never mutates `BehaviourVersion.provenance` or Attention/Decision scores

---

## Audit of existing version / change systems

| System | Role today | Class for adaptation |
|--------|------------|----------------------|
| **SQLite migrations** (`MigrationRunner`) | Schema evolution; failed migration blocks init | **Reusable:** ordered, audited, non-silent version steps — **not** cognition learning |
| **Observation / WorkspaceState `schema_version`** | Capture/format versioning | **Reusable:** identity of data shape — **not** behaviour adaptation |
| **Capability grants** | Permission changes after `DecideApproval` | **Separate:** execution authority; adaptation must not mint grants |
| **Policy / automation contract revisions** | Definition approval ≠ run | **Reusable:** approve-definition ≠ apply-runtime |
| **Feature flags** | **Not present** as a product system | Do not invent silent flag flips from outcomes |
| **User preferences** | Explicit user-authored config | **Separate:** not outcome-driven versioning |
| **Explanation catalog version** | Experience catalog `version` | **Experience boundary** — unchanged by adaptation drafts |

**Finding:** Versioning patterns exist for schema and permissions. None may be reused as a silent path from Outcome → runtime cognition. Controlled change must stay proposal → surface → draft version → (future Gateway) → evaluate.

---

## Provenance & rollback

```
Evidence → … → Outcome → OutcomeAdaptationProposal → Review
    → ControlledChangeSurface (provenance frozen)
    → BehaviourVersion draft (provenance cloned)
    → rollback draft (provenance preserved)
    → ChangeEvaluation (provenance_snapshot; no rewrite)
```

Rollback prepares a new draft targeting `previous_version_id`. It does not edit historical
reasoning origins.

---

## Related docs

- [WORKSPACE-GOVERNANCE-LEDGER.md](./WORKSPACE-GOVERNANCE-LEDGER.md)
- [WORKSPACE-ADAPTATION-REVIEW.md](./WORKSPACE-ADAPTATION-REVIEW.md)
- [WORKSPACE-ADAPTATION-GOVERNANCE.md](./WORKSPACE-ADAPTATION-GOVERNANCE.md)
- [WORKSPACE-ADAPTATION-PROPOSAL.md](./WORKSPACE-ADAPTATION-PROPOSAL.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
- [WORKSPACE-AUTOMATION-READINESS.md](./WORKSPACE-AUTOMATION-READINESS.md)
