# Workspace Research Catalogue

Purpose

Store every technology investigation exactly once.

Never repeat research.

---

Capability

Status

Candidates

Decision

Reason

Integration Notes

Licence

Security Review

Maintenance

Review Date

---

## PA-001 — Permission Authority patterns and candidate categories

Capability: Permission Authority

Status: Research complete; decision pending

Candidates:

- Workspace-native stateful reference monitor and evaluator
- Workspace authority with an embedded policy-as-code evaluator
- Workspace authority with a local relationship authorization service
- Workspace authority with cryptographic capability-style proof primitives
- Hybrid evaluator plus stateful Workspace proof/consumption authority
- OS-broker-inspired approval and scoped-handle workflow
- Open-source comparators: Open Policy Agent 1.18.2, Cedar 4.11.2,
  Apache Casbin 3.10.0 / Casbin Rust 2.20.0, OpenFGA 1.18.1,
  SpiceDB 1.56.0, Eclipse Biscuit `biscuit-auth` 6.0.0, and legacy
  Oso 0.27.3

Decision: No technology selected. Retain the candidate categories for bounded
evaluation after relevant architecture questions are resolved.

Reason: No examined candidate supplies the complete Workspace challenge,
effect-proof, operation-control-proof, point-of-use consumption, revocation
ordering, replay prevention, and content-free audit semantics. Policy
evaluation, relationship evaluation, cryptographic proof primitives, local
transactional persistence, and Windows key protection remain credible commodity
integration categories behind Workspace-owned authority.

Integration Notes:

- Preserve Permission Authority as the sole owner of permission truth.
- Keep user decisions, grant state, challenge redemption, proof lifecycle,
  consumption ordering, audit projection, and explanation semantics
  Workspace-owned.
- Treat effect and operation-control proofs as independently governed authority.
- Do not treat OS permission prompts, policy decisions, relationship checks,
  events, or token signatures as sufficient point-of-use authority.
- Selection remains blocked on reproducible Windows, offline, failure,
  revocation-race, replay, migration, and contract-acceptance evidence.
- Detailed record:
  `architecture/research/PERMISSION_AUTHORITY_RESEARCH.md`

Licence: Evaluated open-source comparators are Apache-2.0 at the exact versions
recorded in the detailed research. No component or transitive dependency is
approved; exact dependency-level legal review remains required before adoption.

Security Review: Research-level threat and pattern review complete. Mandatory
unknowns remain for process isolation, proof representation, key lifecycle,
clock/storage rollback, offline-control lease behavior, audit attacker model,
compound authorization, and consent-surface trust. No security approval issued.

Maintenance: Active 2026 releases were verified for OPA, Cedar, Casbin,
OpenFGA, and SpiceDB. Biscuit's current evaluated stable release is from 2025
with active repository evidence. The legacy Oso open-source line is officially
deprecated. Exact-version maintenance and security evidence must be refreshed
before selection.

Rejected Options: None. Oso's deprecation is recorded as current maintenance
risk, not converted into a selection decision.

Unresolved Risks: Permission scope granularity, proof opacity and local
verification, same-process bypass, identity stability, revocation/consumption
transaction boundaries, rollback-resistant time/state, audit assurance,
compound scopes, grant lifetimes, batch effects, and non-manipulative consent.

Evidence: `architecture/research/PERMISSION_AUTHORITY_RESEARCH.md`, including
primary official sources accessed 2026-08-01.

Review Date: 2027-02-01, or earlier on architecture change, material candidate
release/deprecation/advisory/licence change, Windows incompatibility, or new
reproducible evidence.
