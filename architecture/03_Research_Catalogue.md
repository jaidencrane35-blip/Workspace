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

Unresolved Risks: Memory taxonomy; merge/reorganization scope visibility;
correction and contradiction semantics; retention and pruning authority;
retrieval-quality and hardware thresholds; graph necessity; exact
redaction/forget guarantees;
provenance that may survive deletion; encryption attacker/recovery model;
backup expiry; synchronization requirement, trust, conflict, tombstone, and
retired-device policy; operation commit/cancellation semantics.

Evidence: `architecture/research/MEMORY_RESEARCH.md`, including primary official
sources and research literature accessed 2026-08-01.

Review Date: 2027-02-01, or earlier on architecture or policy resolution,
material candidate release/deprecation/advisory/licence change, Windows
incompatibility, changed scale/hardware needs, or new reproducible retrieval,
deletion, encryption, or synchronization evidence.

---

## ROADMAP-001 — Remaining capability research sequence

Record Type: Research planning record; not a technology research unit

Capability: System-wide capability research planning

Status: Roadmap complete; architecture review accepted (`LEDGER-0011`)

Candidates: Not applicable. This record orders future research and does not
discover, compare, select, recommend, or reject technologies.

Decision: Use `architecture/13_Capability_Research_Roadmap.md` as the
dependency-driven plan for the eight remaining capability research areas:
Runtime Host, Workspace Management, Context Sensing, Action, Intelligence,
Companion Orchestration, Experience, and conditionally Extension Host.

Reason: Runtime lifecycle and Workspace scope have the greatest upstream
fan-out. Context Sensing, Action, and Intelligence can then be researched in a
bounded parallel wave. Companion depends on those domain findings; Experience
depends on the states and controls it must present; Extension Host remains last
and blocked on separate activation justification.

Integration Notes:

- `PA-001` and `MEM-001` remain completed research prerequisites, not selected
  technologies.
- Relevant user-journey architecture gaps must be recorded as assumptions
  during bounded research and resolved before final comparison or selection in
  affected areas.
- Each future bounded capability/category question receives its own canonical
  Catalogue research unit under the existing framework.
- At roadmap acceptance Runtime Host was the first remaining unit. Runtime Host
  and Workspace Management research are now complete; Context Sensing, Action,
  and Intelligence are the next bounded wave after shared assumptions are
  accepted.
- Context Sensing, Action, and Intelligence may proceed in parallel only after
  shared lifecycle, permission, Memory-boundary, and applicable scope
  assumptions are explicit.
- Companion Orchestration and Experience follow their dependencies.
- Extension Host research cannot activate or justify the subsystem by itself.

Licence: Not applicable; no component, dependency, model, or licence was
evaluated or approved.

Security Review: Planning-level dependency review only. The sequence places
root lifecycle and scope before high-risk sensing, action, and intelligence
comparisons, then delays orchestration, presentation, and optional untrusted
extension research until their security inputs are known. No security approval
issued.

Maintenance: Re-evaluate the sequence when architecture, contracts, accepted
research findings, capability activation, or implementation milestones change.

Rejected Options: None. No technology or research area was rejected.

Unresolved Risks: User-journey architecture corrections; Permission Authority
proof, isolation, consent, and recovery assumptions; Memory taxonomy,
merge/reorganization scope, deletion, encryption, synchronization, and lifecycle
assumptions; Runtime Host process/lifecycle boundaries; per-action-class scope;
local Intelligence viability; and Extension Host activation justification.

Evidence: Existing architecture and completed research only, synthesized in
`architecture/13_Capability_Research_Roadmap.md`. No new external technology
research was performed.

Review Date: At architecture approval, after any user-journey gap resolution,
when a completed research unit changes downstream assumptions, and before each
new research wave.

---

## RH-001 — Runtime Host architectures and candidate categories

Capability: Runtime Host

Status: Research complete; decision pending

Candidates:

- Workspace-owned lifecycle coordinator over an in-process modular host
- Desktop framework event loop with explicit Workspace host policy
- Generic-host/hosted-service composition adapted to a user-session desktop app
- Supervisor-tree-inspired task and capability policy
- Hybrid modular host with selected isolated local workers
- Windows Job Object-managed worker topology
- Constrained Windows workers using AppContainer or lower-privilege tokens
- Sandboxed WebAssembly component boundary for separately justified use
- Typed local RPC over named pipes or another OS-local transport
- Structured local logging/tracing with optional, separately governed export
- Versioned typed configuration with atomic local persistence
- Representative comparators: Tauri 2.11.5, Electron 43.2.0, .NET 10.0.10
  Generic Host, Tokio 1.53.1, Erlang/OTP 29.0.4 supervision, Wasmtime 47.0.2,
  Rust `tracing`, `libloading`, OpenTelemetry Rust SDK 0.32.1, and Windows
  lifecycle/process/IPC/diagnostic/packaging APIs

Decision: No technology selected, recommended, approved, conditionally
approved, or rejected. Retain the patterns, categories, and comparators for
bounded evaluation after Runtime Host architecture assumptions are resolved.

Reason: Modern mature approaches separate host policy from runtime mechanisms.
Explicit lifecycle state, dependency-ordered startup, reverse shutdown,
structured cancellation, bounded supervision, freshness-aware domain-free
health, typed recoverable configuration, owned background work, and local
diagnostics are stable patterns. No examined category supplies Workspace's
complete lifecycle, safe-restart, health-minimization, explanation, offline,
Windows shutdown, update, and contract semantics unchanged.

Integration Notes:

- Preserve Runtime Host as the owner of lifecycle, registration, host
  configuration, health aggregation, connectivity presence, and ordered
  shutdown only.
- Keep essential/degradable classification, safe-restart policy, crash-loop
  escalation, resource ownership, and explanation mapping Workspace-owned.
- Select process boundaries by risk and recovery class; a worker process is a
  crash boundary but not automatically a privilege boundary.
- Treat graceful worker control and hard containment as separate paths. Parent
  exit or a dropped child handle does not terminate Windows workers; Job Object
  kill-on-close can prevent orphans but is forced termination, not completion.
- Require every task, process, endpoint, handle, timer, and diagnostic sink to
  have one owner, cancellation path, completion signal, and release deadline.
- Keep diagnostics local by default, domain-free, bounded, erasable, and
  independent from optional export.
- Treat Windows shutdown, Job Objects, power events, named-pipe security,
  single-instance behavior, packaging, signing, update, and crash diagnostics
  as explicit Windows adapters rather than hidden framework behavior.
- Do not treat native dynamic loading as isolation or this research as
  Extension Host activation.
- Selection remains blocked on architecture review and reproducible Windows,
  offline, startup, degradation, crash, recovery, IPC, shutdown, update,
  privacy, resource, migration, and contract-acceptance evidence.
- Detailed record: `architecture/research/RUNTIME_HOST_RESEARCH.md`

Licence: Current licence snapshots were recorded for representative
comparators: MIT or Apache-2.0, MIT, Apache-2.0,
Apache-2.0 WITH LLVM-exception, and ISC as applicable. Windows platform,
build-tool, WebView, installer, signing, bundled runtime, plugin, and transitive
terms remain separate. No component or transitive dependency is approved;
exact-scope legal review remains required before adoption.

Security Review: Research-level threat and pattern review complete. Mandatory
unknowns remain for process/isolation classes, standard-user and elevation
boundaries, worker identity, IPC ACLs and anti-squatting, handle inheritance,
binary verification, sandbox compatibility, restart reconciliation,
configuration recovery, diagnostic leakage, update trust, and forced
termination. No security approval issued.

Maintenance: Active 2026 releases were verified for the representative desktop,
runtime, supervision, sandbox, and observability comparators. Electron's rapid
major cadence, Wasmtime/component evolution, framework/plugin versioning,
Windows regressions, and transitive supply-chain changes require exact-version
refresh before selection.

Rejected Options: None. No project was approved or rejected.

Unresolved Risks: Startup classes outside the audited foundation;
capability-specific process and safe-restart classes; host configuration
backup/recovery; health freshness and lifecycle deadlines; operation
reconciliation after crash; supported Windows and hardware
baseline; packaging/update/signing/rollback; single-instance trust;
suspend/resume; IPC protocol; diagnostic retention and dump privacy; resource
budgets; existing/nested Job Object and breakaway behavior; user-session versus
service hosting; offline repair; certificate/update-key recovery; and
cross-platform parity.

Evidence: `architecture/research/RUNTIME_HOST_RESEARCH.md`, including primary
official sources accessed 2026-08-01.

Review Date: 2027-02-01, or earlier on architecture correction, process-model
decision, framework/runtime major release, security advisory, Windows support
change, packaging decision, or new reproducible evidence.

---

## WM-001 — Local-first Workspace Management patterns and candidate categories

Capability: Workspace Management

Status: Research complete; decision pending

Candidates:

- Normalized relational Workspace aggregate
- Document-oriented Workspace aggregate
- Per-workspace or per-profile physical stores
- Append-only/event-sourced organization model
- Content-addressed/revisioned organization model
- CRDT/local-first replicated organization model
- Hybrid authoritative snapshot plus bounded journal
- Versioned logical archive with manifest/checksums
- Engine-supported physical backup and staged restore
- Organizational profile identity/association model
- Desktop arrangement definition/association with separate sensing and Action
- Representative comparators: SQLite 3.53.4, rusqlite 0.40.1, SQLx 0.9.0,
  Diesel 2.3.11, redb 4.1.0, LiteDB 5.0.21, Apache CouchDB 3.5.2,
  Automerge 3.x, Git 2.55.0, Jujutsu 0.43.0, VS Code 1.131.0,
  PowerToys 0.100.2, GlazeWM 3.10.1, RFC 8493 BagIt, and RFC 9562 UUIDs

Decision: No technology selected, recommended, approved, conditionally
approved, or rejected. Retain the patterns, categories, and comparators for
bounded evaluation after Workspace profile, hierarchy, active-scope, archive,
arrangement, import/restore, and recovery semantics are resolved.

Reason: Mature approaches consistently separate stable organizational identity,
immutable versioned scope, owner-authoritative mutation state, logical
portability, and physical recovery. They also distinguish workspace definition
from profile configuration, live desktop observation, geometry/application
effects, and application-internal session state. No examined category supplies
Workspace's complete scope, permission, archive, stale-reference, conflict,
outcome-recovery, privacy, import, and explainability semantics unchanged.

Integration Notes:

- Preserve Workspace Management as sole owner of workspaces, zones,
  organizational membership, active scope, revisions, and organizational
  metadata.
- Keep names, paths, hierarchy positions, `HWND`s, process IDs, and monitor
  numbers separate from stable identity.
- Treat scope snapshots as immutable minimized values; events and IDs grant no
  authority, and consumers must invalidate/re-query stale versions.
- Commit aggregate changes, active scope, revision, operation status,
  idempotency, terminal outcome, and outbox together where the approved
  aggregate permits.
- Keep export/import, physical backup/restore, domain archive/restore, and merge
  as separate candidate concerns. Export/import and physical backup/restore
  remain conditional until administration contracts and policy are accepted.
- Keep Memory, file indexing, live observation, desktop effects, UI state,
  application sessions, orchestration, and permission truth outside this
  capability.
- Treat profiles and desktop-arrangement definitions as architecture ownership
  questions before evaluating storage or arrangement technologies.
- Selection remains blocked on reproducible Windows, offline, scope-isolation,
  migration, conflict, lost-result, corruption, privacy, and contract-acceptance
  evidence; import/archive and backup/restore evidence becomes mandatory if
  those conditional administration paths are accepted.
- Detailed record:
  `architecture/research/WORKSPACE_MANAGEMENT_RESEARCH.md`

Licence: Current licence snapshots were recorded for representative public
domain, MIT, MIT OR Apache-2.0, Apache-2.0, GPL-2.0, and GPL-3.0 comparators,
plus informational RFC specifications. Source and binary-product terms can
differ. No component or transitive dependency is approved; exact-scope legal
review remains required before adoption.

Security Review: Research-level threat and pattern review complete. Mandatory
unknowns remain for profile/workspace isolation, scope staleness, identity
collision, hostile databases, path/reparse-point handling, desktop identity
ambiguity, and migration integrity. Archive extraction, backup/export leakage,
import provenance, encryption/key recovery, and restore integrity become
mandatory if those conditional administration paths are accepted. No security
approval issued.

Maintenance: Active 2026 releases were verified for representative embedded
storage, Rust access/migration, revision, workspace/profile, and Windows
arrangement comparators. Exact patch-level corruption fixes, schema/format
stability, MSRV/runtime changes, Windows regressions, and transitive
supply-chain changes require refresh before selection.

Rejected Options: None. No project was approved or rejected.

Unresolved Risks: Profile ownership; tree/DAG and membership semantics;
single/set/per-session active scope and `unscoped`; merge/reorganization
invalidation and Memory identity/visibility; arrangement ownership;
external-resource and display/app identity; import/restore/merge identity;
conditional logical export content and backup retention/encryption/recovery;
downgrade; synchronization; and representative scale/performance limits.

Research Confidence: Medium. Storage, backup, migration, logical archive,
identity, Windows path/locking, and capability-boundary findings have strong
primary evidence. Product-specific profile, simultaneous active-scope,
arrangement, merge, and synchronization semantics remain unresolved.

Evidence: `architecture/research/WORKSPACE_MANAGEMENT_RESEARCH.md`, including
primary official sources accessed 2026-08-01.

Review Date: 2027-02-01, or earlier on architecture correction, accepted
archive/profile/arrangement semantics, storage-format/advisory change, Windows
support change, or new reproducible evidence.

---

## EXP-001 — Experience component library, patterns, and presentation stack

Capability: Experience (presentation research; not a domain technology selection)

Status: Research complete; decision pending

Candidates:

- Presentation stack: Tailwind CSS 4.x, shadcn/ui registry on Radix Primitives
  or Base UI, class-variance-authority, clsx, tailwind-merge
- Icons: Lucide React (ISC)
- Time: date-fns (MIT); or platform `Intl.RelativeTimeFormat`
- Dialogs/drawers: Radix Dialog/AlertDialog; optional Vaul
- Command palette (Phase 3+): cmdk (MIT)
- Motion (Phase 4): Motion / Framer Motion line (MIT)
- Virtualized lists (Phase 3 if needed): react-virtuoso core (MIT; not Message
  List commercial SKU)
- Pattern references: VS Code recents, Linear hierarchy, Notion empty
  structure, Obsidian local-vault metaphor, Fluent/HIG focus and density,
  concept-board masonry rhythm
- Rejected shell kits: Ant Design, Material UI as product chrome; GPL UI kits

Decision: No technology selected or approved. Retain the component catalogue,
licensing matrix, gap analysis, token proposal, and Phase 2 plan for an
authorised Experience Phase 2 implementation session.

Reason: Experience Phase 2 needs dashboard-first Home composition more than new
capabilities. A curated MIT/Apache/ISC-friendly stack accelerates accessible
primitives without forcing Material/Ant aesthetics. Exact-version legal and
CSP review remain required before `pnpm add`.

Integration Notes:

- Experience owns presentation only; Product Proof Save/Continue behaviour,
  handoff authorship, restore limits, and consented Check-in stay unchanged.
- Do not import ambient thumbnails, AI briefings, system gauges, or fabricated
  activity feeds from concept boards without separate authority.
- Prefer build-time CSS (Tailwind output) under existing CSP; no CDN scripts.
- Open Source Registry gains Approved rows only after adoption review — not by
  this research entry alone.
- Detailed package: `architecture/research/experience/` and
  `architecture/research/EXPERIENCE_COMPONENT_RESEARCH.md`

Licence: Evaluated candidates are MIT, Apache-2.0, or ISC at public registry
claims accessed 2026-08-02. No component or transitive dependency is approved.

Security Review: Research-level only. Supply-chain and CSP packaging checks
required at adoption. No security approval issued.

Maintenance: Active ecosystems verified at research time for Tailwind, Radix,
shadcn/ui pattern, Lucide, and Motion. Refresh before selection.

Rejected Options: GPL UI kits; Ant Design / Material as Experience shell;
react-virtuoso Message List commercial SKU; ambient/AI concept-board features
for Phases 2–4.

Unresolved Risks: shadcn primitive-base choice (Radix vs Base UI); Tailwind
adoption cost vs CSS-variable Path B; icon attribution process; transitive
licence drift.

Evidence: `architecture/research/EXPERIENCE_COMPONENT_RESEARCH.md`,
`architecture/research/experience/*`, concept boards, Experience Roadmap,
Participant #1 review after LEDGER-0034.

Review Date: 2027-02-01, or earlier on Experience Roadmap change, material
dependency licence/maintenance change, CSP policy change, or new pilot evidence.
