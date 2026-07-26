# Workspace Governance Integrity Verification Contract

Sprint 153 — verification model producing diagnostics only.

**Never repairs automatically. Never mutates governance. Never executes.**

## Principle

```
Governance artifacts
    ↓
GovernanceIntegrityVerification
    ↓
Diagnostics (Info | Warning | Error)
```

Checks cover:

- provenance completeness
- missing evidence
- invalid review chains
- invalid lifecycle ordering
- broken references
- timeline consistency

## GovernanceIntegrityVerification

| API | Behaviour |
|-----|-----------|
| `verify_timeline` | Lifecycle + provenance + ref diagnostics |
| `verify_bundle` | Proposal / evidence / decisions / record |
| `attempt_repair` | Hard-fail |
| `attempt_mutate_governance` | Hard-fail |

`has_errors()` reports Error-severity diagnostics. Callers may act; the verifier does not.

---

## Related docs

- [WORKSPACE-GOVERNANCE-LIFECYCLE.md](./WORKSPACE-GOVERNANCE-LIFECYCLE.md)
- [WORKSPACE-GOVERNANCE-CONDITIONS.md](./WORKSPACE-GOVERNANCE-CONDITIONS.md)
- [WORKSPACE-GOVERNANCE-COMPATIBILITY.md](./WORKSPACE-GOVERNANCE-COMPATIBILITY.md)
- [WORKSPACE-GOVERNANCE-ARCHIVE.md](./WORKSPACE-GOVERNANCE-ARCHIVE.md)
- [WORKSPACE-GOVERNANCE-EVIDENCE.md](./WORKSPACE-GOVERNANCE-EVIDENCE.md)
- [WORKSPACE-RECOMMENDATION-PROVENANCE.md](./WORKSPACE-RECOMMENDATION-PROVENANCE.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
