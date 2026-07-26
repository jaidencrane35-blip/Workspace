# Workspace Governance Workspace & Publication Environment

Sprint 147 — human governance operating surface and publication environment boundary
before any future activation.

**UI cannot approve execution. Publication environment cannot activate runtime.**

## Principle

```
GovernanceRecord
    ↓
GovernanceWorkspace (visibility / review operating surface)
    ↓
PublicationEnvironment (target / scope / rollout boundary)
    ↓
PublishRequest
    ↓
Published Version (still blocked)
```

---

## GovernanceWorkspace architecture

Architecture only — not a privileged command surface.

| View | Contents |
|------|----------|
| `proposal_view` | Proposal id, affected area, proposed change, expected effect |
| `evidence_view` | Evidence refs, dissent count, final rationale |
| `risk_view` | Risk level, impact class, required reviewer count |
| `reviewer_view` | Reviewer actor ids + required actor type |
| `decision_history` | Append-only decision rows |
| `readiness_state` | Publication readiness label |

Guards:

- `may_approve_execution() == false`
- `attempt_approve_execution()` → `GovernanceWorkspaceCannotApproveExecution`
- `preserves_provenance` against ledger provenance
- `authority_effect: none`

Builder: `GovernanceWorkspace::from_governance_bundle(...)`.

---

## PublicationEnvironment architecture

| Field | Meaning |
|-------|---------|
| `publication_target` | e.g. `future_behaviour_version` |
| `workspace_scope` | Which workspace the change would target |
| `compatibility_requirements` | Preconditions (schema / behaviour baseline) |
| `rollback_scope` | Version / baseline rollback target |
| `staged_rollout` | `none` / `staged_canary` / `staged_percent` / `full` |

Guards:

- `may_activate_runtime() == false`
- `attempt_activate()` hard-fails
- `bind_publish_request` only validates record linkage — never publishes
- Provenance cloned from GovernanceWorkspace

---

## Audit of existing UI and environment concepts

| System | Pattern | Boundary vs governance surfaces |
|--------|---------|----------------------------------|
| **Operator Console** | Aggregated read-only panels; shows `authority_effect: none` | **Reusable:** multi-panel visibility without grant; must not become execution Approve UI |
| **Experience surfaces** | DisplayReason / catalog translation for cognition | **Separate:** experience boundary unchanged; governance workspace is not Experience |
| **Workspace Environment Model** | Live desktop read model from WorkspaceState | **Separate:** observation/environment facts — not publication targets |
| **Permission Gateway surfaces** | Approvals / grants for command retry | **Separate:** execution authority; governance UI must not call Allow |
| **Audit views / AuditService** | Append-only event history | **Reusable:** decision history shape; governance history is change-review trail |

**Finding:** Operator-style read panels + append-only history are the right UX pattern.
Permission approval UI remains the only path that can produce execution Allow.

---

## Related docs

- [WORKSPACE-GOVERNANCE-EVIDENCE.md](./WORKSPACE-GOVERNANCE-EVIDENCE.md)
- [WORKSPACE-GOVERNANCE-LEDGER.md](./WORKSPACE-GOVERNANCE-LEDGER.md)
- [WORKSPACE-ENVIRONMENT-MODEL.md](./WORKSPACE-ENVIRONMENT-MODEL.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
- [WORKSPACE-AUTOMATION-READINESS.md](./WORKSPACE-AUTOMATION-READINESS.md)
