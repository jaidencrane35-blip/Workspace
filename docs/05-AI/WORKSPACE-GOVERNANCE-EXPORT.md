# Workspace Governance Export Contract

Sprint 164 — immutable read-only export packages.

**No import. No synchronisation. No publication.**

## GovernanceExportPackage

| Field | Meaning |
|-------|---------|
| `provenance_bundle` | Frozen recommendation provenance |
| `evidence_references` | Evidence ids |
| `archive_references` | Historical snapshot ids |
| `compatibility_metadata` | Declared dependency markers |
| `version_metadata` | Export schema marker |
| `integrity_hash_placeholder` | Architecture placeholder only |
| `read_only` | Always true |

---

## Related docs

- [WORKSPACE-GOVERNANCE-REPORTING.md](./WORKSPACE-GOVERNANCE-REPORTING.md)
- [WORKSPACE-GOVERNANCE-METRICS.md](./WORKSPACE-GOVERNANCE-METRICS.md)
- [WORKSPACE-GOVERNANCE-ARCHIVE.md](./WORKSPACE-GOVERNANCE-ARCHIVE.md)
- [WORKSPACE-GOVERNANCE-DECISION-PACKAGE.md](./WORKSPACE-GOVERNANCE-DECISION-PACKAGE.md)
- [WORKSPACE-RECOMMENDATION-PROVENANCE.md](./WORKSPACE-RECOMMENDATION-PROVENANCE.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
