# Workspace Management Capability Research

Research ID: WM-001

Status: Research complete; architecture review and candidate validation pending

Evidence review date: 2026-08-01

This record applies the canonical template in
`architecture/12_Capability_Technology_Research_Framework.md`. It is technology
research only. It does not select, recommend, approve, conditionally approve,
or reject a technology, change capability ownership, resolve product semantics,
or authorize implementation.

---

## A. Identity and Scope

- Research ID: `WM-001`
- Capability: Workspace Management
- Technology category or question: modern local-first workspace organization,
  scope, persistence, portability, and recovery patterns
- Research status: Research complete; decision pending
- Research owner: Workspace engineering
- Date opened: 2026-08-01
- Last reviewed: 2026-08-01
- Catalogue entry: `WM-001` in `architecture/03_Research_Catalogue.md`
- Related contracts: `WSP-REQ-001` through `WSP-REQ-005`, `WSP-CMD-001`,
  and `WSP-EVT-001` through `WSP-EVT-003`
- Related interactions: `IC-013` through `IC-018`
- Required acceptance cases: Contract Specification ownership, identity,
  authority, scope, lifecycle, error, event, compatibility, privacy, and
  recovery invariants; offline core scenario 3; applicable duplicate,
  conflict, lost-event, delayed-reconciliation, migration, and Local First
  cases
- Relevant ADRs: ADR-0001 through ADR-0008

In scope:

- workspace and zone identity, lifecycle, hierarchy, membership, and metadata
- multiple workspaces and active-scope models
- the meaning and ownership boundary of profiles
- organizational definitions or references related to desktop arrangements
- active-scope and operation session continuity
- immutable versioned scope snapshots and invalidation
- local storage, aggregate isolation, schema evolution, and migration
- logical export/import, physical backup/restore, domain archive/restore, and
  merge as separate operations
- privacy, offline behavior, explainability, failure, corruption, and recovery
- Windows paths, locking, display/application identity, virtual desktops, and
  standard-user operation

Out of scope:

- selecting or implementing a database, migration runner, archive format,
  synchronization system, profile system, or desktop arrangement engine
- Memory contents, retention, retrieval, or archived-Memory policy
- file contents, desktop indexing, or live observation
- launching, moving, resizing, hiding, or otherwise controlling applications
- UI layout, application-internal tabs/documents, or presentation state
- Companion planning, task policy, or automatic workspace switching
- product permission policy or proof issuance
- synchronization or multi-device operation as an assumed requirement
- deleting external files or durable Memory when a workspace is archived

## B. Capability Objective

Workspace Management must own durable local workspaces, zones, organizational
membership, active scope, and minimized organizational metadata. It must expose
authorized summaries and immutable scope snapshots, enforce hierarchy and scope
invariants, commit protected mutations with owner-authoritative outcomes,
invalidate stale consumers, and operate fully offline.

It must not become:

- Memory or a store for retained user knowledge
- a file index or owner of file contents
- Context Sensing or a source of live desktop observations
- Action or an executor of desktop arrangements
- Companion Orchestration or a policy engine
- Experience or a store for presentation/application-session state
- Permission Authority or a source of product authorization

The technology boundary must preserve stable Workspace-owned organizational
semantics while keeping storage, archive, migration, and desktop-integration
mechanisms replaceable.

## C. Research Questions

### Findings

1. **What constitutes workspace identity?** Mature models separate opaque,
   stable identity from display name, path, hierarchy location, ordering, and
   active state. Rename, move, path relocation, archive, restore, schema
   migration, and import must not silently change identity.
2. **What lifecycle is required?** Create, active/available, archived, restoring,
   and unavailable/corrupt are distinct states. Archive is not deletion:
   it changes eligibility and visibility but must not silently delete Memory,
   files, or external application state.
3. **How should first-run creation behave?** Default workspace creation and
   establishment of a valid or explicitly unscoped active pointer must be
   idempotent and committed together.
4. **How many workspaces may be active?** Mature products use incompatible
   models: one global active workspace, one per window, one per monitor, an
   active set plus focus, or multiple overlapping launch recipes. The current
   architecture describes an active workspace/zone but leaves simultaneous
   presence and `unscoped` semantics unresolved. Storage must not decide this.
5. **What is a profile?** The term can mean an OS user identity, an
   organizational persona, a settings/extension bundle, or temporary session
   configuration. VS Code separates workspace identity from profiles and
   associates a profile per window/workspace. Workspace Management may own only
   an organizational profile identity or association if architecture accepts
   that concept; UI settings, provider settings, permissions, and OS identity
   remain with their owners.
6. **How should hierarchy and membership be represented?** Adjacency lists,
   closure tables, materialized paths, and nested sets trade simple writes,
   subtree reads, move cost, and validation complexity. A separate membership
   relation supports many-to-many membership. Tree versus DAG, depth, ordering,
   exclusivity, and inherited archive behavior are product semantics.
7. **How should scope cross capability boundaries?** A scope snapshot is an
   immutable value containing stable IDs, lifecycle/archive state, a revision,
   and only the minimum authorized closure needed by the caller. It is not a
   shared mutable object or permanent authority. Consumers compare revisions,
   invalidate on `WSP-EVT-001`, and re-query/revalidate at meaningful use.
8. **Which revision strategy is viable?** A global organization revision makes
   invalidation simple but causes unrelated churn. Per-workspace/zone revisions
   reduce churn but require composed dependency versions for moves, shared
   membership, active sets, and cross-workspace operations.
9. **How should conflicts work?** Storage locks do not express domain conflict.
   Mutations need an operation ID, expected revision, stable targets, requested
   change, requester/correlation, and bound proofs. The owner compares revision
   and validates invariants in the same commit, returning minimized current
   revision and conflict details rather than silently rebasing.
10. **How are lost outcomes recovered?** Aggregate state, new revision,
    operation status, idempotency record, terminal result, and event outbox can
    be committed atomically in one local aggregate. Lost responses/events are
    reconciled by operation identity; retry deduplicates and never converts
    unknown into success.
11. **What does session continuity mean here?** Workspace Management owns
    continuity of active scope, accepted mutation status, and organizational
    definitions. Open tabs, unsaved documents, focus, live windows, process
    sessions, and application-internal state belong elsewhere.
12. **What is a desktop arrangement?** Mature tools distinguish a durable
    definition or launch/geometry recipe from observed windows and applied
    effects. PowerToys Workspaces captures app identity and geometry, then
    launches/reuses and moves windows; it cannot reproduce arbitrary
    application-internal state or Windows Snap state. Workspace Management
    could own only an organizational association or immutable arrangement
    intent if architecture approves it. Context Sensing captures live state,
    Action performs effects, Experience presents progress, and Companion
    decides when to coordinate.
13. **How stable are desktop references?** `HWND` is live-session identity, not
    durable identity. AppUserModelID/package identity is stronger but not
    universal; path, process, class, title, version, and command-line signals
    drift and can be ambiguous. Monitor identity/topology and DPI also change.
    User correction and honest ambiguity are mandatory.
14. **What can Windows virtual-desktop integration guarantee?** The supported
    `IVirtualDesktopManager` surface is narrow and does not provide full public
    desktop enumeration/creation/switching. Private Shell APIs are not a sound
    portability or support boundary.
15. **Which local storage boundary is mature?** A shared embedded transactional
    store makes hierarchy, active scope, operation outcomes, and an outbox
    locally atomic. Per-profile or per-workspace stores strengthen some physical
    separation but multiply migrations and make cross-workspace operations and
    active selection multi-file transactions.
16. **What is schema versioning?** Engine schema version, application migration
    history, contract version, and logical export format are separate.
    SQLite's `user_version` is only an application-managed integer; it does not
    supply ordering, checksums, rollback, or semantic validation.
17. **How should migration work?** Use monotonic IDs/checksums, compatibility
    gates, staged/transactional transformation, pre-migration recovery state,
    post-migration structural and domain checks, crash markers where needed,
    deterministic resume/rollback, and tests from every supported version.
18. **How do export, backup, archive, and merge differ?**
    - export/import transfers a versioned semantic representation;
    - physical backup/restore recovers an exact store;
    - archive/restore changes domain lifecycle;
    - merge reconciles two identity graphs under product rules.
    Treating them as synonyms would create identity and privacy errors.
19. **What is a portable export?** A versioned logical manifest with producer
    and schema compatibility, payload inventory, checksums, bounded sizes,
    archive identity, and explicit machine-specific fields. Physical database
    files remain engine-coupled. Checksums establish integrity, not authority.
20. **How should import identity work?** Preserve source identity, target
    identity, and import-operation identity separately. UUIDs reduce accidental
    collision but do not remove same-ID/different-history cases. Preserve,
    remap, clone, merge, restore, or reject are product decisions.
21. **What does backup include?** Workspace backup includes only Workspace-owned
    organization, revisions, active/archive state, operation history needed for
    safe recovery, and import provenance. Memory, permissions, Experience
    state, Action history, diagnostics, and application content require their
    owners' separate export/backup contracts.
22. **Is synchronization necessary?** CRDT and sync systems demonstrate offline
    convergence, but convergence does not enforce hierarchy, one active scope,
    archive policy, authorization, forgetting, or semantic conflict resolution.
    Synchronization is not currently required and cannot be assumed.

### Assumptions requiring architecture review

- profile definition, ownership, namespace, inheritance, and lifetime
- hierarchy depth, tree/DAG model, membership cardinality, ordering, and types
- one active scope, active set, per-window/session scope, and `unscoped`
- whether Workspace Management owns an arrangement definition/association
- what arrangement fields may be retained without owning sensing or Action
- restore/import identity and collision/merge semantics
- deletion/removal lifecycle, if it exists at all
- synchronization, downgrade, and backup policy requirements

Capability Architecture v1.2 resolves active archive fallback, stale-scope
invalidation, and archived/restored Memory visibility. Merge-specific identity
and visibility semantics remain open.

### Unacceptable outcomes for future candidates

- names, paths, `HWND`s, display order, or hierarchy location used as identity
- storage schema convenience defining product hierarchy or archive semantics
- silent cross-profile or cross-workspace access
- stale scope accepted after archive, move, restore, or active-scope change
- shared mutable scope passed across capability boundaries
- lost mutation outcome reported as failure or success without reconciliation
- retry that duplicates an accepted mutation
- workspace archive deleting Memory or external files implicitly
- raw file copying of an active WAL database described as a backup
- physical backup described as a replaceable logical export
- import that silently overwrites, merges, or remaps colliding identities
- desktop capture described as full application-session recovery
- Workspace Management launching or moving windows
- private Windows virtual-desktop APIs treated as a supported contract
- required cloud, synchronization, activation, or telemetry for core operation
- exports containing secrets, authority proofs, Memory, diagnostics, or
  machine-specific identifiers without explicit owner and policy

## D. Required Functional Capabilities

| ID | Requirement | Source | Class | Pattern-level support |
|---|---|---|---|---|
| WM-F-01 | Create, rename, reorganize, select, and archive workspace/zone aggregates | `WSP-CMD-001` | Mandatory | Relational/document/journal patterns support mechanics; semantics remain Workspace-owned |
| WM-F-02 | Return active scope and authorized workspace summaries | `WSP-REQ-001/002` | Mandatory | Indexed local stores support fast reads; minimization adapter required |
| WM-F-03 | Validate scope for a capability request | `WSP-REQ-003`, `IC-015`–`017` | Mandatory | Stable identity/revision and invariant evaluation required |
| WM-F-04 | Publish immutable versioned scope changes | `WSP-EVT-001`, `IC-018` | Mandatory | Transactional outbox/journal patterns support delivery/recovery |
| WM-F-05 | Publish minimized workspace mutation outcomes, including indeterminate | `WSP-EVT-002/003` | Mandatory | Owner outcome state machine and operation identity required |
| WM-F-06 | Recover authoritative operation status and bounded terminal history | `WSP-REQ-004/005` | Mandatory | Idempotency/outcome records required |
| WM-F-07 | Reject invalid hierarchy, archived targets, stale scope, conflicts, and invalid proof | Contracts | Mandatory | Database constraints plus aggregate validation |
| WM-F-08 | Maintain exactly the approved active/unscoped invariant | Architecture | Mandatory | Atomic active-pointer/set update required after semantics are approved |
| WM-F-09 | Persist organizational membership without indexing member content | Architecture | Mandatory | Opaque descriptors or stable external references |
| WM-F-10 | Version and migrate local state with recovery | Research framework | Mandatory | Migration runner plus Workspace invariant checks |
| WM-F-11 | Export/import a versioned logical Workspace representation | Requested research scope | Conditional evaluation requirement until public administration contracts are accepted | Archive/manifest adapter required |
| WM-F-12 | Create/restore a consistent physical backup of Workspace-owned state | Requested research scope | Conditional evaluation requirement until backup/recovery policy and authority are accepted | Engine-supported snapshot required |
| WM-F-13 | Explain active scope, structural change, requester, permission class, outcome, and conflict | Contracts | Mandatory | Workspace-owned reason model required |
| WM-F-14 | Associate profile or arrangement metadata only if architecture assigns it here | Requested scope | Conditional | Ownership gate precedes technology comparison |
| WM-F-15 | Operate with multiple workspaces under approved active-scope semantics | Requested scope | Mandatory | Global/per-session/set model remains unknown |

No category supplies every row unchanged. The comparison unit is Workspace-owned
domain policy plus replaceable persistence, migration, archive, and optional
desktop-definition adapters.

## E. Required Non-Functional Capabilities

- **Durability:** committed organization, active scope, operation identity, and
  revisions survive crash and forced shutdown.
- **Consistency:** hierarchy, membership, archive, profile partition, active
  selection, operation outcome, and event-outbox invariants commit together
  where the approved aggregate permits.
- **Responsiveness:** active scope and navigation summaries remain interactive
  under representative workspace counts and hierarchy depth.
- **Conflict honesty:** stale expected revisions and ambiguous commits never
  become silent last-writer-wins success.
- **Availability:** store failure produces explicit constrained unscoped mode;
  no wider scope or cached authority fallback.
- **Privacy:** organizational metadata is minimized, protected, and excluded
  from logs/exports unless explicitly part of the operation.
- **Testability:** deterministic IDs/clocks, fault injection, migration matrices,
  archive fuzzing, and invariant validation.
- **Portability:** if logical export is accepted, it is independent of physical
  storage layout; OS-specific arrangement references are optional and explicit.
- **Replaceability:** domain model and scope contract do not expose database
  row/page identifiers or framework types; any future export schema follows the
  same boundary.

Representative thresholds for workspace count, zones, membership, archive
size, operation history, and latency remain unknown.

## F. Local First Requirements

With all network access blocked, the user must be able to:

1. start with a valid local active scope or explicit unscoped state
2. create, list, rename, reorganize, select, and archive workspaces/zones
3. validate scope for local Memory, sensing, and Action operations
4. recover accepted mutation status after lost responses or restart
5. migrate and repair accepted local state
6. explain conflicts, unavailable scope, and recovery options

If logical export/import or physical backup/restore is later accepted through
architecture contracts, those administration paths must also work entirely
offline. Research and candidate comparison may test them now, but may not treat
them as approved product contracts.

Optional synchronization, online profile sharing, remote templates, and cloud
backup must not be prerequisites or silent fallbacks. Offline-first evidence
must include installation, schema migration, repair, and recovery, not merely
steady-state reads.

## G. Privacy Requirements

Workspace metadata can reveal projects, clients, interests, timing, app use,
paths, machine topology, and working relationships even without file content.

Required controls:

- separate stable opaque IDs from user-authored names and resource paths
- store only organizational membership needed for scope
- never ingest file contents or build a file index
- minimize summaries per caller and authorization
- partition profiles/workspaces according to accepted isolation semantics
- exclude absolute paths, usernames, hostnames, drive letters, UNC paths,
  command lines, tokens, credentials, DPAPI blobs, recent-file history, and
  diagnostics from portable export by default
- treat app titles, executable paths, AUMIDs, monitor IDs, and geometry as
  potentially sensitive machine-specific arrangement metadata
- redact before logs, conflict details, error messages, and telemetry
- keep backup/export retention, deletion, encryption, and user inspection
  explicit
- prevent logical export or journal history from becoming alternate Memory
- ensure archive does not silently change Memory retention

## H. Security Considerations

### Trust boundaries

- Permission Authority issues proofs; Workspace Management validates at each
  protected read or mutation commit.
- Scope events and snapshots grant no authority.
- Stable identity is not authorization.
- Import provenance is evidence, not local authority.
- Desktop arrangement definitions cannot grant Action permission.

### Storage and archive threats

- hierarchy cycles, cross-profile links, stale revisions, replayed operation
  IDs, and malicious identity collisions
- corrupt or hostile database/archive input
- oversized entities, deep hierarchy, decompression bombs, duplicate names,
  and pathological conflict history
- Windows absolute, UNC, drive-relative, device-namespace, parent traversal,
  reserved-name, trailing-dot/space, case-folding, alternate-data-stream,
  symlink, junction, and reparse-point attacks
- side-effecting SQLite schema features or extensions in untrusted files
- leaked free pages, WAL/SHM files, backups, journals, and temporary exports
- restore or migration that revives archived active scope

Future candidates must apply hard entry/count/size/depth/time limits, extract
only into a new controlled directory, reject path escape and case collisions,
use restrictive per-user ACLs, avoid arbitrary extension loading, and validate
engine integrity plus Workspace invariants before activation.

## I. Performance Considerations

Future prototypes must measure on supported Windows x64 and Arm64 hardware:

- cold/warm open, migration, and first active-scope query
- list/navigation and scope-validation latency at representative scale
- workspace/zone create, move, archive, select, and conflict latency
- recursive hierarchy and membership closure cost
- global versus per-aggregate invalidation churn
- concurrent readers and mutation serialization
- database, WAL, journal, outcome, tombstone, and backup growth
- logical export/import throughput and peak memory
- physical backup, restore, integrity check, and migration duration
- archive bomb/pathological hierarchy rejection
- antivirus/indexer sharing conflicts, read-only storage, disk full, abrupt
  shutdown, and power-loss recovery

Average latency is insufficient. Tail latency, bounded memory/disk, cancellation,
and failure behavior are required.

## J. Explainability Considerations

Workspace Management must enable Experience and Companion to explain:

- which workspace/zone or explicit unscoped state is active
- stable display identity without exposing internal storage identity
- what structural change was requested and by whom
- which permission class was validated
- expected/current revision and minimized conflict reason
- accepted, pending, succeeded, conflicted, failed, indeterminate, or expired
  operation state
- why a scope became stale or unavailable
- whether an import will preserve, remap, clone, restore, merge, or refuse
  identity once that policy is accepted
- what an export/backup contains and excludes
- why an arrangement could not match an app/display, without claiming that
  Workspace Management performed the effect

## K. Licensing Evaluation

Representative comparators are evidence sources, not approved dependencies:

| Comparator | Evaluated revision | Licence snapshot | Legal note |
|---|---|---|---|
| SQLite | 3.53.4 | Public domain | Exact bundled build, extensions, compile flags, and wrappers require review |
| rusqlite | 0.40.1 | MIT | Bundled/system SQLite and transitive native build obligations differ |
| SQLx | 0.9.0 | MIT OR Apache-2.0 | Feature set, CLI, macros, migration tooling, and transitive tree require review |
| Diesel | 2.3.11 | MIT OR Apache-2.0 | CLI artifacts and transitive dependencies require exact review |
| redb | 4.1.0 | MIT OR Apache-2.0 | Exact format stability and dependency scope require review |
| LiteDB | 5.0.21 | MIT | .NET/runtime and current maintenance evidence require review |
| Apache CouchDB | 3.5.2 | Apache-2.0 | Server runtime and transitive notices materially expand distribution scope |
| Automerge | 3.x family | MIT | Exact JS/Rust/Wasm component versions and history behavior require review |
| Git | 2.55.0 | GPL-2.0 | Used as a data-model comparator; distribution would carry material obligations |
| Jujutsu | 0.43.0 | Apache-2.0 | Used as a revision/operation-log comparator |
| VS Code source | 1.131.0 | MIT | Microsoft binary product has separate proprietary terms |
| PowerToys | 0.100.2 | MIT | Used as a Windows arrangement/recovery comparator |
| GlazeWM | 3.10.1 | GPL-3.0 | Used as a transparent workspace/layout comparator |
| RFC 8493 BagIt | 1.0 specification | Informational RFC | A format pattern, not a dependency or authenticity mechanism |

No licence is approved. Exact features, transitive dependencies, notices,
patents, trademarks, commercial distribution, native binaries, and build tools
require legal review before adoption.

## L. Maintenance Evaluation

- SQLite 3.53.4 was released 2026-07-24 and fixed a WAL-reset corruption defect,
  illustrating the need for exact patch tracking and recovery tests.
- rusqlite 0.40.1, SQLx 0.9.0, Diesel 2.3.11, and redb 4.1.0 had active 2026
  releases; their SQLite version, MSRV, migration, and native-build choices
  differ.
- VS Code 1.131.0 was released 2026-07-29 and documents mature workspace/profile
  behavior, but source and product licensing differ.
- PowerToys 0.100.2 and its Workspaces/FancyZones documentation were current in
  2026; persisted arrangement compatibility and individual workspace export
  guarantees remain incomplete.
- CouchDB, Automerge, Git, and Jujutsu are actively maintained comparators for
  document, merge, and revision models, but their operational assumptions
  exceed or differ from a bounded local aggregate.
- workspacer's last substantive release is old; maintenance age is evidence,
  not a project rejection.

Exact-version advisories, migration history, maintainer continuity, format
stability, Windows regressions, and replacement feasibility must be refreshed
before any decision.

## M. Community Maturity Evaluation

- SQLite, Git, VS Code, Eclipse workspace resources, and Windows APIs provide
  long-lived operational evidence for local data and workspace concepts.
- PowerToys provides current Windows-first evidence for application/monitor
  arrangement limitations and user-visible partial recovery.
- SQLx, Diesel, rusqlite, and redb show active Rust ecosystems with different
  abstraction and migration costs.
- CouchDB and Automerge provide mature conflict/convergence patterns, but
  synchronization semantics exceed current requirements.
- Jujutsu demonstrates operation-log and undo concepts without defining
  Workspace scope or permission authority.

Popularity is not fitness. Reproducible Workspace contract, Windows, privacy,
migration, and recovery evidence remains mandatory.

## N. Platform Compatibility

### Windows-first requirements

- use per-user local application data by default, not install directories,
  arbitrary workspace folders, roaming/network shares, or OneDrive assumptions
- test packaged and unpackaged local paths, ACLs, standard-user operation, x64,
  Arm64, long Unicode paths, case-insensitivity, reserved names, and
  localization
- coordinate database/WAL/SHM, antivirus, indexer, backup, and restore handles;
  open sharing modes can block rename/delete/replace
- never copy only the main file of a live WAL database
- stage and validate backup/export/restore before publishing or activation
- treat `ReplaceFileW` and same-volume replacement as primitives requiring
  power-loss and sharing tests, not universal durability proof
- use stable monitor identity where available and treat topology, DPI,
  connector, and identical-monitor cases as changeable
- use AppUserModelID/package identity when available but retain ambiguity
  handling for unpackaged/legacy applications
- treat `HWND`, process ID, title, and monitor number as transient evidence
- remain within supported virtual-desktop APIs
- distinguish standard and elevated application windows; integrity boundaries
  can prevent observation or repositioning

### Portability

Portable concepts: Workspace/zone/profile identities, lifecycle, hierarchy,
membership, revisions, scope snapshots, mutation outcomes, export manifest, and
domain reason codes.

Platform-specific adapters: local paths/ACLs, file replacement/locking,
application/monitor identity, desktop arrangement observation/effects, virtual
desktops, packaging, and repair.

## O. Integration Complexity

| Pattern | Added boundaries | Principal complexity |
|---|---|---|
| Normalized relational aggregate | schema, migrations, queries, constraints | hierarchy closure, domain revisions, mapper drift |
| Document aggregate | serialized aggregate and revision | coarse conflicts, referential integrity, large rewrites |
| Per-workspace/profile stores | store registry and multi-file lifecycle | cross-store atomicity, migrations, backup, active scope |
| Append-only/event-sourced model | events, projections, upcasters, snapshots | permanent replay, compaction, privacy, repair burden |
| Content-addressed/revisioned model | immutable objects, refs, GC | canonicalization, equality leakage, ref races, retention |
| CRDT/local-first model | replica identity, merge, tombstones, sync | domain conflicts, stale devices, deletion, schema evolution |
| Hybrid snapshot plus journal | snapshot, journal, checkpoints | divergence, authority ambiguity, dual migration |
| Logical archive | manifest, codec, import mapping, validation | complete semantic specification and hostile-input security |
| Desktop arrangement association | app/display references and effect handoff | heuristic drift, ownership, partial restore, Windows APIs |

Complexity is measured by permanent authorities and recovery obligations, not
only implementation size. Event sourcing, per-workspace databases, CRDTs, and
desktop recipe execution require explicit Complexity Budget justification.

## P. Extensibility

A viable approach should allow:

- new organizational metadata without changing stable identity
- hierarchy/membership evolution under versioned invariants
- new scope consumers without shared mutable access
- replacement of persistence and migration libraries
- logical export independent of physical storage
- optional arrangement/profile associations only after ownership approval
- future synchronization through a separate justified layer without making it
  core authority

It must not:

- expose database row/page IDs as contract identity
- turn arbitrary metadata into an unversioned extension store
- permit plugins or extensions to bypass Companion/Permission Authority
- use speculative synchronization to redefine local authority
- absorb application state, Memory, sensing, Action, or Experience ownership

## Q. Failure Modes

| Failure | Owner-authoritative response | Required recovery/explanation |
|---|---|---|
| Duplicate/invalid identity | reject before acceptance | explain identity class without exposing unrelated records |
| Hierarchy cycle/cross-profile link | reject transaction | preserve prior aggregate and active scope |
| Stale expected revision | conflict | return current revision and minimized conflict |
| Response lost after commit | operation remains owner-recorded | status lookup; deduplicate retry |
| Commit outcome ambiguous | mark/reconcile indeterminate | never infer success or failure |
| Scope-change event lost | consumer snapshot becomes stale by freshness/version policy | re-query authoritative scope |
| Active workspace archived | apply approved atomic fallback or reject | never leave accidental broad scope |
| Store missing/corrupt | constrained unscoped/recovery mode | distinguish missing, corruption, incompatibility, and permission failure |
| Disk full/read-only/sharing violation | fail mutation/backup honestly | no false persistence claim |
| Migration interrupted | rollback/resume from recorded stage | retain verified pre-migration recovery |
| Unsupported future schema | refuse activation | do not coerce or downgrade silently |
| Physical backup interrupted | staged artifact remains unpublished | validate before use |
| Logical export interrupted | incomplete archive is not published | manifest/checksum verification |
| Import repeated | idempotency record controls outcome | no unintended duplicate |
| Import identity collision | apply approved explicit collision policy | no silent overwrite/remap/merge |
| Hostile archive/database | reject under limits/sandboxed parser policy | no path escape, excessive allocation, or side effects |
| Arrangement app/display unmatched | organization remains intact; effect owner reports partial/failure | expose ambiguity/missing prerequisite |
| Optional sync unavailable | local authority continues | no startup or repair dependency |

## R. Migration Risk

Migration can affect:

- stable identity and references
- hierarchy, membership, ordering, and archive state
- active scope and scope revisions
- accepted operation status and idempotency tombstones
- profile/arrangement associations
- database schema and engine format
- logical export/import format
- backups, journals, and recovery markers

Required protections include monotonic checksummed migrations, pre-migration
verified backup, one-step deterministic transformations, crash-at-every-step
tests, post-migration `integrity_check`, separate `foreign_key_check`, Workspace
invariant validation, explicit downgrade/refusal policy, and restore tests from
every supported version.

SQLite `.recover` or equivalent salvage is not a faithful restore: constraints
may be violated and deleted data may reappear. Salvaged data must enter a new
staged store and pass full structural, privacy, and domain validation.

## S. Reasons to Build Internally

Workspace-owned code is justified for:

- identity, lifecycle, hierarchy, membership, and archive semantics
- active/unscoped and profile-partition rules
- immutable scope snapshot and revision meaning
- expected-revision conflict detection and reason codes
- mutation acceptance, idempotency, indeterminate outcome, and tombstones
- archive/restore/import/merge identity policy
- logical export schema and ownership allowlist
- domain invariant validation and explanations
- arrangement/profile ownership boundaries

These encode unique Workspace scope and trust semantics. They should remain
small and use commodity persistence/archive mechanisms underneath.

## T. Reasons to Integrate Externally

Credible commodity integration categories include:

- embedded transactional storage
- typed query/data-access libraries
- schema migration runners
- checksum, archive, and serialization libraries
- online physical backup and integrity-check mechanisms
- UUID generation
- safe path/archive extraction primitives
- optional desktop arrangement observation/effect adapters behind other
  capability contracts
- optional synchronization primitives after separate justification

Integration is justified only through adapters that preserve Workspace
authority, allow replacement, and do not import another product's workspace
semantics.

## U. Candidate Comparison

The categories below remain open; no disposition is made.

| Category | Strength | Material trade-off | Evidence still required |
|---|---|---|---|
| Normalized relational aggregate | constraints, transactions, joins, atomic outbox/outcome | recursive hierarchy and migrations need deliberate design | invariant, scale, corruption, migration, Windows tests |
| Document aggregate | self-contained snapshot/export and revision | coarse conflict, large rewrites, weaker cross-aggregate integrity | concurrent moves, global active scope, recovery |
| Per-workspace/profile stores | physical isolation and portable units | cross-store atomicity and migration/backup multiplication | multi-workspace operations and restore |
| Append-only/event-sourced model | provenance, expected revisions, replay | projections, upcasting, retention, privacy complexity | deterministic replay, compaction, deletion |
| Content-addressed/revisioned model | immutable snapshots, branching, deduplication | equality leakage, GC, ref races, canonical format | identity/authority mapping and bounded history |
| CRDT/local-first model | offline concurrent merge | convergence does not enforce Workspace semantics | sync requirement, domain conflict, stale-replica policy |
| Hybrid snapshot plus journal | fast current reads plus bounded recovery evidence | snapshot/journal divergence and dual migration | authority, checkpoint, rebuild, truncation |
| Versioned logical archive | replaceability, selective privacy, portable validation | semantic omission and hostile-input surface | complete round trip, identity collision, security |
| Physical snapshot/backup adapter | exact local recovery | engine/schema coupling and free-page/privacy behavior | live backup, restore, retention, encryption |
| Profile association model | separates organization from settings overlays | ownership, inheritance, temporary state unresolved | architecture acceptance and isolation tests |
| Desktop arrangement definition/association | durable user organization over apps/displays | heuristic identity, topology drift, partial effects | ownership, supported APIs, user correction, privacy |

Representative projects and standards prove mechanisms exist. They do not prove
Workspace fitness or receive a project disposition.

## V. Required Acceptance Criteria

Before any candidate can be adopted:

1. `WSP-*` contracts and `IC-013` through `IC-018` map without ownership change.
2. Stable IDs survive rename, move, archive, restore, path/topology changes, and
   schema migration.
3. Approved profile, hierarchy, membership, archive, active, and unscoped
   invariants are enforced in every read/mutation/import/restore path.
4. Protected reads and writes validate bound authority at access/commit.
5. Immutable scope snapshots contain minimized identity and deterministic
   revision/invalidation semantics.
6. Stale scope is rejected after move, archive, restore, merge, profile change,
   or active-scope transition.
7. Expected-revision conflicts, operation idempotency, owner status, terminal
   tombstones, and indeterminate outcomes pass loss/duplicate/crash tests.
8. Aggregate state, active scope, revisions, outcome, and outbox are atomic or
   honestly partial under an approved boundary.
9. Core create/read/change/select/archive/status/migrate paths pass with all
   network access blocked.
10. Migration passes from every supported historical version under crash,
    disk-full, read-only, sharing-violation, and restart injection.
11. If physical backup/restore administration is accepted, it uses a consistent
    engine-supported snapshot and verifies engine integrity, foreign keys, and
    Workspace invariants.
12. If logical export/import administration is accepted, it round-trips every
    required Workspace semantic without Memory, permissions, UI state,
    diagnostics, or external file content.
13. If import is accepted, repeated import and same-ID
    same/different-history collision cases produce only an approved explicit
    outcome.
14. If archive-container import/export is accepted, traversal, reparse point,
    alternate stream, case collision, reserved name, and decompression-bomb
    tests pass on Windows.
15. Desktop arrangement support, if owned here in any form, preserves the
    sensing/Action/Experience/Companion boundaries and reports per-item
    ambiguity/partial failure.
16. No private Windows virtual-desktop API is required for supported behavior.
17. Windows standard-user, packaged/unpackaged, local paths/ACLs, x64, Arm64,
    antivirus/indexer, abrupt shutdown, and power-loss evidence is reproducible.
18. Organizational metadata, conflicts, and logs pass privacy leakage inspection
    and retention/deletion controls; accepted export/backup paths pass the same
    gate.
19. Exact licences, transitive dependencies, advisories, maintenance, migration,
    replacement, and Complexity Budget are accepted by named authorities.
20. Unknown product semantics are resolved or explicitly bounded by architecture
    before final comparison or selection.

### Remaining unknowns

- profile definition, owner, namespace, sharing, inheritance, and temporary form
- tree versus DAG, maximum depth, ordering, and membership cardinality/types
- one active scope, active set, focused scope, per-window/session scope, and
  exact `unscoped` behavior
- in-flight invalidation and Memory identity/visibility for merge or
  reorganization beyond the accepted archive/restore lifecycle
- deletion lifecycle, if any, versus archive
- arrangement definition ownership and permitted retained fields
- application/display match precedence, ambiguity, and user correction
- application-internal continuity expectations
- stable external resource representation without indexing
- global versus aggregate/scope revision strategy
- atomicity of multi-workspace moves or memberships
- restore/import preserve/remap/clone/merge/refuse semantics
- logical export history, tombstone, and provenance requirements
- backup retention, encryption, key recovery, removable media, RPO, and RTO
- downgrade and supported compatibility window
- synchronization and multi-device requirement
- representative scale, size, depth, and latency thresholds

## W. Decision and Review

- Decision: No technology selected, recommended, approved, conditionally
  approved, or rejected. Research findings and comparator categories recorded.
- Selected scope: None.
- Decision rationale: Required Workspace profile, hierarchy, membership,
  simultaneous active/unscoped, arrangement, merge/reorganization,
  import/restore collision, conditional backup, downgrade, and synchronization
  semantics remain unresolved, and no reproducible candidate evidence has been
  produced.
- Rejected candidates and reasons: None; project approval/rejection was
  explicitly out of scope.
- Conditions or controls: Section V gates future adoption; architecture must
  resolve the ownership and product-semantic unknowns first.
- Remaining unknowns: Section V.
- Required ADR: None at research completion. A durable active-scope,
  profile/arrangement ownership, archive, merge, synchronization, or storage
  boundary decision may require an ADR.
- Open Source Registry action: None.
- Engineering Ledger action: Record completion of `WM-001`.
- Review date: 2027-02-01, or earlier on architecture correction, accepted
  archive/profile/arrangement semantics, storage-format/advisory change,
  Windows support change, or new reproducible evidence.
- Approver: Pending.

---

## Research Findings

- Stable identity must be independent of names, paths, hierarchy, windows, and
  display topology.
- Scope snapshots are immutable, versioned, minimized values; neither events nor
  IDs grant continuing authority.
- Domain revisions and operation IDs are required even when the storage engine
  supplies transactions and locks.
- Export/import, physical backup/restore, domain archive/restore, and merge are
  separate operations with different identity, privacy, and recovery semantics.
- Profiles and desktop arrangements cross current capability boundaries and
  require architecture decisions before technology comparison.
- Desktop arrangement capture can preserve recipes and geometry, not arbitrary
  application-internal sessions.
- Local transactional aggregates, logical manifests, journals, revision graphs,
  and CRDTs solve different problems; no one pattern defines Workspace
  semantics.

## Architectural Trade-offs

1. **Shared store versus physical isolation:** one store simplifies atomic
   global scope; per-workspace/profile stores contain some failures but create
   multi-file migration and transaction problems.
2. **Relational constraints versus aggregate portability:** normalized data
   enforces references efficiently; documents are easier to snapshot/export but
   shift integrity to application code.
3. **Global versus local revisions:** global invalidation is simple but noisy;
   local revisions reduce churn but need dependency composition.
4. **Snapshot versus history:** current-state snapshots are bounded and fast;
   journals/revision graphs aid recovery and undo but expand privacy, migration,
   and compaction obligations.
5. **Declarative arrangement versus capture:** declarations are portable and
   reviewable; capture is easier but imports transient paths, titles, geometry,
   versions, and topology.
6. **Exact geometry versus adaptive layout:** pixels reproduce one setup;
   relative layouts tolerate topology changes but cannot guarantee the same
   result.
7. **Stable app identity versus broad compatibility:** package/AUMID identity is
   stronger but incomplete; heuristic path/title matching is broad but
   ambiguous.
8. **Physical backup versus logical export:** physical snapshots maximize
   fidelity; logical archives maximize replacement and selective privacy.
9. **Automatic restore versus user control:** automatic continuity reduces
   effort but can restore stale scope, move the wrong window, or hide conflict.
10. **Local authority versus synchronization:** local-only is simpler and
    private; synchronization adds replica, conflict, schema, deletion, and
    retired-device policy.

## Candidate Solution Categories

- normalized relational Workspace aggregate
- document-oriented Workspace aggregate
- per-workspace or per-profile physical stores
- append-only/event-sourced organization model
- content-addressed/revisioned organization model
- CRDT/local-first replicated organization model
- hybrid authoritative snapshot plus bounded journal
- versioned logical archive with manifest/checksums
- engine-supported physical backup and staged restore
- organizational profile identity/association model
- desktop arrangement definition/association with separate sensing and Action

## Research Confidence

Overall confidence: **Medium**

Justification:

- **High confidence** in local transactional, SQLite WAL/backup/integrity,
  migration, versioned archive, UUID, Windows path/locking, and scope-snapshot
  patterns because they are documented by primary specifications and mature
  projects.
- **High confidence** that desktop definitions, live observation, Action
  effects, and application-internal session state are separate concerns.
- **Medium confidence** in exact comparator version/maintenance snapshots and
  arrangement schema portability because these evolve rapidly.
- **Medium-to-low confidence** in a concrete Workspace data model because
  profile, hierarchy, simultaneous active scope, arrangement ownership, import
  identity, merge/reorganization visibility, downgrade, and synchronization
  semantics are unresolved architecture questions.
- **Low confidence by design** in universal third-party desktop session restore;
  supported Windows APIs and mature tools do not establish such a guarantee.

## Evidence Register

Primary and official sources accessed 2026-08-01:

1. SQLite 3.53.4 release:
   https://sqlite.org/releaselog/3_53_4.html
2. SQLite isolation and WAL:
   https://sqlite.org/isolation.html and https://sqlite.org/wal.html
3. SQLite atomic commit:
   https://sqlite.org/atomiccommit.html
4. SQLite Online Backup API:
   https://sqlite.org/backup.html
5. SQLite `VACUUM INTO`:
   https://sqlite.org/lang_vacuum.html
6. SQLite integrity and foreign-key checks:
   https://sqlite.org/pragma.html
7. SQLite recovery and security:
   https://sqlite.org/recovery.html and https://sqlite.org/security.html
8. SQLite sessions/changesets:
   https://sqlite.org/session.html
9. SQLite recursive CTEs and foreign keys:
   https://sqlite.org/lang_with.html and https://sqlite.org/foreignkeys.html
10. SQLite public-domain status:
    https://sqlite.org/copyright.html
11. rusqlite 0.40.1:
    https://github.com/rusqlite/rusqlite/releases/tag/v0.40.1
12. SQLx 0.9.0:
    https://github.com/transact-rs/sqlx/blob/main/CHANGELOG.md
13. Diesel 2.3.11:
    https://github.com/diesel-rs/diesel/releases/tag/v2.3.11
14. redb design and 4.1.0:
    https://github.com/cberner/redb/blob/master/docs/design.md and
    https://github.com/cberner/redb/releases/tag/v4.1.0
15. Apache CouchDB consistency/conflicts:
    https://docs.couchdb.org/en/stable/intro/consistency.html and
    https://docs.couchdb.org/en/latest/replication/conflicts.html
16. Automerge concepts:
    https://automerge.org/docs/reference/concepts/
17. Git data model/worktrees:
    https://git-scm.com/docs/gitdatamodel and
    https://git-scm.com/docs/git-worktree
18. Jujutsu 0.43.0:
    https://github.com/jj-vcs/jj/releases/tag/v0.43.0
19. VS Code workspaces:
    https://code.visualstudio.com/docs/editing/workspaces/workspaces
20. VS Code profiles:
    https://code.visualstudio.com/docs/configure/profiles
21. VS Code 1.131:
    https://code.visualstudio.com/updates/v1_131
22. PowerToys Workspaces:
    https://learn.microsoft.com/windows/powertoys/workspaces
23. PowerToys FancyZones:
    https://learn.microsoft.com/windows/powertoys/fancyzones
24. PowerToys Workspaces data schema:
    https://raw.githubusercontent.com/microsoft/PowerToys/main/src/modules/Workspaces/WorkspacesLib/WorkspacesData.h
25. Windows AppUserModelIDs:
    https://learn.microsoft.com/windows/win32/shell/appids
26. Windows virtual desktop manager:
    https://learn.microsoft.com/windows/win32/api/shobjidl_core/nn-shobjidl_core-ivirtualdesktopmanager
27. Windows stable monitor identity:
    https://learn.microsoft.com/uwp/api/windows.devices.display.core.displaytarget.stablemonitorid
28. Windows local/roaming known folders:
    https://learn.microsoft.com/windows/win32/shell/knownfolderid
29. Windows file/path and sharing rules:
    https://learn.microsoft.com/windows/win32/fileio/naming-a-file and
    https://learn.microsoft.com/windows/win32/api/fileapi/nf-fileapi-createfilew
30. Windows file replacement:
    https://learn.microsoft.com/windows/win32/api/winbase/nf-winbase-replacefilew
31. RFC 8493 BagIt:
    https://www.rfc-editor.org/rfc/rfc8493.html
32. RFC 9562 UUIDs:
    https://www.rfc-editor.org/rfc/rfc9562.html

### Evidence limitations

- Documentation and comparator behavior establish mechanisms, not Workspace
  acceptance.
- No prototype, benchmark, migration, archive-fuzz, corruption, power-loss,
  Windows packaging, or security validation was performed.
- Exact profile, archive, arrangement, merge, and synchronization semantics are
  architecture questions, not technology findings.
- Issue reports informed failure discovery but are not treated as proof that a
  current release passes or fails Workspace requirements.

