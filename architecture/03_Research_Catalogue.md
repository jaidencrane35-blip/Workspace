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

---

## MEM-001 — Local-first AI Memory patterns and candidate categories

Capability: Memory

Status: Research complete; decision pending

Candidates:

- Workspace-owned canonical local store with same-engine lexical/vector
  projections
- Workspace-owned canonical local store with separate embedded lexical, vector,
  and optional graph libraries
- Embedded multimodel retrieval store behind Memory contracts
- Canonical local store with local sidecar retrieval services
- AI-memory/retrieval frameworks as non-authoritative pipeline components
- Revisioned source/claim store with temporal graph and hybrid projections
- Optional synchronization layer over canonical operations and deletion barriers
- Internal Memory orchestration with external commodity storage, retrieval,
  cryptography, model, and synchronization primitives
- Open-source comparators: SQLite 3.53.4/FTS5, Tantivy 0.26.1, sqlite-vec
  0.1.9, FAISS 1.14.3, USearch 2.26.0, LanceDB 0.33.0, Qdrant 1.18.3,
  Meilisearch 1.51.0, Chroma 1.5.9, LadybugDB 0.18.3, Mem0 2.0.14,
  LangGraph 1.2.10, Graphiti 0.29.3, LlamaIndex 0.14.23, Haystack 3.0.0,
  SQLCipher 4.17.0, Automerge, Yjs, libSQL, and Syncthing

Decision: No technology selected or rejected. Retain all categories and
comparators for bounded evaluation after Memory policy and architecture
questions are resolved.

Reason: Modern approaches converge on a Workspace-owned canonical record with
provenance and one or more replaceable derived projections. Lexical, dense,
metadata/time, graph, fusion, reranking, compression, encryption, and
synchronization mechanisms solve distinct problems. No examined framework or
engine supplies the complete Workspace propose-write, permission, scope,
provenance, correction, retention, exact redaction/forgetting, operation
recovery, and explainability contract unchanged.

Integration Notes:

- Preserve Memory as the sole durable user-knowledge owner.
- Keep Companion working/task context ephemeral; durable retention requires a
  separate authorized Memory proposal.
- Treat full-text, vectors, graphs, summaries, caches, and framework stores as
  rebuildable projections with complete derivation lineage, never independent
  authorities.
- Apply permission and workspace scope before candidate content crosses a
  retrieval, model, process, or caller boundary.
- Define forgetting as explicit guarantees across canonical records,
  projections, keys, backups, tombstones, and replicas rather than search
  invisibility alone.
- Keep optional synchronization subordinate to local authority and unavailable
  without changing core local Memory behavior.
- Selection remains blocked on reproducible Windows, offline, retrieval-quality,
  deletion, encryption, failure, migration, and contract-acceptance evidence.
- Detailed record: `architecture/research/MEMORY_RESEARCH.md`

Licence: The detailed record captures current exact-version licence snapshots.
The candidate set includes public-domain, MIT, Apache-2.0, BSD-style,
MPL-2.0, and ISC components, plus separately licensed commercial/enterprise
features. No component, model, or transitive dependency is approved; exact-scope
legal review remains required before adoption.

Security Review: Research-level threat and pattern review complete. Mandatory
unknowns remain for memory poisoning, scope isolation, encryption/key recovery,
backup leakage, rollback, derived-index erasure, SSD deletion limits, stale
replicas, sidecar hardening, and malicious/corrupt index handling. No security
approval issued.

Maintenance: Current 2026 releases were verified for representative embedded
storage, lexical/vector search, graph, framework, encryption, and synchronization
projects. Rapid framework evolution, pre-1.0 components, successor projects,
legacy/deprecated lines, native packaging, security advisories, and
cross-component migration require exact-version refresh before selection.

Rejected Options: None. Maintenance, licensing, security, platform, and
complexity concerns are recorded as comparative evidence and unknowns, not
technology rejections.

Unresolved Risks: Memory taxonomy; archived-workspace visibility; correction and
contradiction semantics; retention and pruning authority; retrieval-quality and
hardware thresholds; graph necessity; exact redaction/forget guarantees;
provenance that may survive deletion; encryption attacker/recovery model;
backup expiry; synchronization requirement, trust, conflict, tombstone, and
retired-device policy; operation commit/cancellation semantics.

Evidence: `architecture/research/MEMORY_RESEARCH.md`, including primary official
sources and research literature accessed 2026-08-01.

Review Date: 2027-02-01, or earlier on architecture or policy resolution,
material candidate release/deprecation/advisory/licence change, Windows
incompatibility, changed scale/hardware needs, or new reproducible retrieval,
deletion, encryption, or synchronization evidence.
