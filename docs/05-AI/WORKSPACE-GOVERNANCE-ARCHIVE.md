# Workspace Governance Archive & Historical Preservation Contract

Sprint 154 — long-term governance history architecture.

**Nothing may be deleted. History is append-only. No activation.**

## Principle

```
Live governance artifacts
    ↓
GovernanceArchiveContract.append(...)
    ↓
GovernanceHistoricalSnapshot (immutable)
```

Kinds:

- `ProposalSnapshot`
- `ReviewRecord`
- `PublicationRecord`
- `TimelineSnapshot`
- `SupersededProposal`

Superseding appends a new marker; prior snapshots remain.

## GovernanceArchiveContract

| Guard | Value |
|-------|-------|
| `append_only` | `true` |
| `may_delete` | `false` |
| `attempt_delete` | `GovernanceArchiveImmutable` |
| `may_activate_runtime` | `false` |
| `authority_effect` | `none` |

---

## Related docs

- [WORKSPACE-GOVERNANCE-LIFECYCLE.md](./WORKSPACE-GOVERNANCE-LIFECYCLE.md)
- [WORKSPACE-GOVERNANCE-INTEGRITY.md](./WORKSPACE-GOVERNANCE-INTEGRITY.md)
- [WORKSPACE-GOVERNANCE-CONDITIONS.md](./WORKSPACE-GOVERNANCE-CONDITIONS.md)
- [WORKSPACE-GOVERNANCE-COMPATIBILITY.md](./WORKSPACE-GOVERNANCE-COMPATIBILITY.md)
- [WORKSPACE-GOVERNANCE-FAILURE-HANDLING.md](./WORKSPACE-GOVERNANCE-FAILURE-HANDLING.md)
- [WORKSPACE-PUBLICATION-SAFETY.md](./WORKSPACE-PUBLICATION-SAFETY.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
