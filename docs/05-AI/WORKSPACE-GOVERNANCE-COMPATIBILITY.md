# Workspace Governance Compatibility & Dependency Contract

Sprint 152 — dependency and compatibility requirements for publication preparation.

**No runtime publishing. Declared verification only.**

## Principle

```
Prerequisite contracts
    +
Schema / version compatibility
    +
Migration ordering mindset
    +
Publication environment deps
    →
GovernanceCompatibilityContract.verify_declared()
```

`verify_declared` checks declared markers — it does not apply migrations or publish.

## GovernanceCompatibilityContract

| Area | Pattern source |
|------|----------------|
| Schema floor | Observation / WorkspaceState `schema_version` |
| Migration ordering | MigrationRunner ordered versions |
| Version compatibility | BehaviourVersion identity |
| Package compatibility | Environment compatibility requirements |
| Publication dependency | PublicationEnvironment binding |
| Prerequisite contracts | Prior governance / safety contract refs |

Guards:

- `may_publish_runtime() == false`
- `attempt_publish_runtime()` → activation not implemented
- Gateway remains separate

---

## Related docs

- [WORKSPACE-GOVERNANCE-CONDITIONS.md](./WORKSPACE-GOVERNANCE-CONDITIONS.md)
- [WORKSPACE-PUBLICATION-SAFETY.md](./WORKSPACE-PUBLICATION-SAFETY.md)
- [WORKSPACE-GOVERNANCE-INTEGRITY.md](./WORKSPACE-GOVERNANCE-INTEGRITY.md)
- [WORKSPACE-GOVERNANCE-ARCHIVE.md](./WORKSPACE-GOVERNANCE-ARCHIVE.md)
- [WORKSPACE-GOVERNANCE-WORKSPACE.md](./WORKSPACE-GOVERNANCE-WORKSPACE.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
