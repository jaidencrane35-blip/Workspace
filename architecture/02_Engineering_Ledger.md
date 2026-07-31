# Workspace Engineering Ledger

Every engineering change is appended.

Never rewrite history.

---

## Entry schema

Entry ID

Capability

Research

Decision

Implementation

Validation

Knowledge Gained

Unlocks

Supersedes

Status

---

## Entries

### LEDGER-0001

Entry ID: LEDGER-0001

Capability: Architecture documentation (engineering process)

Research: N/A — documentation consistency review only; no technology research performed.

Decision: Convert `05_Architecture_Decision_Records.md` from a partial ADR restatement into an index of all accepted ADRs. Leave accepted ADR files in `architecture/decisions/` unchanged as the source of truth. Do not alter product architecture or Blueprint meaning.

Implementation:
- Replaced incomplete ADR-0001/0002 body copy in `architecture/05_Architecture_Decision_Records.md` with a full accepted-ADR index (ADR-0001 through ADR-0008), one-sentence summaries, and links to `architecture/decisions/`.
- Clarified that `05` is index-only authority.
- Updated Current State to record this documentation refinement.

Validation:
- All eight accepted ADR files remain byte-content authorities; none rewritten.
- Index introduces no new architectural decisions.
- Blueprint, Current State active task (Capability Map), and product principles unchanged.

Knowledge Gained:
- Restating ADR bodies in `05` created duplicate authority and fell out of date when ADR-0003–0008 were added.
- An index document maintains discoverability without competing with `architecture/decisions/`.
- Remaining soft inconsistencies: ADR short titles vs Blueprint principle phrasing (e.g. Permission First vs Permission Before Automation; Explainability vs Explain Every Action; Integrate Before Reinvent vs Integrate Before Reinventing) — naming alignment only, not conflicting decisions.

Unlocks: Clearer onboarding path for new AI/engineering sessions reading the architecture pack.

Supersedes: Prior content of `05_Architecture_Decision_Records.md` as a decision-detail document (role change to index only).

Status: Complete

### LEDGER-0002

Entry ID: LEDGER-0002

Capability: Workspace Capability Architecture (system-wide)

Research: N/A — architecture decomposition only; no technology research or vendor selection performed.

Decision: Adopt `architecture/08_Workspace_Capability_Architecture.md` as the authoritative product capability decomposition. Ten capabilities with single responsibilities, contract-based interaction, fail-closed permissions, local-first operation, and a strict sense/act split. Intelligence may propose but never execute environment actions. Voice is an Experience modality, not a separate capability. Optional cloud is a permissioned Intelligence provider class, not a separate capability.

Implementation:
- Created `architecture/08_Workspace_Capability_Architecture.md` defining Runtime Host, Permission Authority, Workspace Management, Memory, Context Sensing, Action, Intelligence, Companion Orchestration, Experience, and Extension Host.
- Included per-capability contracts, permissions, data ownership, lifecycle, failure behaviour, explainability, research unknowns, and open-source categories (categories only).
- Included system dependency graph, interaction summary, data ownership map, permission boundary map, user interaction flow, startup/shutdown sequences, lifecycle overview, and architectural validation.
- Updated Current State to mark Capability Architecture complete and set next milestone to per-capability technology research via the Research Catalogue.

Validation:
- Reviewed for duplicate responsibilities, circular hard dependencies, ownership conflicts, unnecessary complexity, Blueprint violations, and ADR violations; residual risks resolved via sense/act split and intelligence non-execution rule.
- No runtime code or behaviour modified.
- No ADR bodies rewritten; no technology selections made.

Knowledge Gained:
- Separating Context Sensing from Action is required for permission clarity and explainability.
- Companion Orchestration must remain the sole core coordinator; Intelligence and Extension Host must not become shadow orchestrators with execution power.
- Collapsing voice/cloud/privacy into existing capabilities reduces complexity without losing ADR coverage.

Unlocks: Per-capability technology research entries in the Research Catalogue; subsequent implementation planning against stable ownership boundaries.

Supersedes: None (first capability architecture authority).

Status: Complete

### LEDGER-0003

Entry ID: LEDGER-0003

Capability: Formal review of Workspace Capability Architecture and capability interactions

Research: N/A — architectural review only; no technology research or vendor selection performed.

Decision: Revise the Capability Architecture to v1.1 and adopt `architecture/09_Capability_Interaction_Matrix.md` as the authoritative communication, intermediary, authority, trust, permission, and event-flow constraint document subordinate to the Blueprint and accepted ADRs.

Implementation:
- Removed the hard Experience/Companion dependency cycle by making Experience the caller/subscriber and Companion presentation-independent.
- Replaced generic permission references with purpose-, capability-, subject-, target-, and operation-bound authorization proofs validated at the point of use.
- Removed direct Context Sensing-to-Memory persistence proposals; observation candidates now flow through Companion and a separate Memory authorization path.
- Removed Extension Host access to arbitrary domain capabilities; all extension-originated domain effects flow through Companion.
- Removed direct Intelligence-to-Memory access and direct Experience-to-Workspace mutations.
- Revised shutdown ordering so Experience remains available until consequential operations report cancellation/completion.
- Replaced the ambiguous dependency diagram and created the formal interaction matrix with required intermediaries and forbidden paths.
- Required purpose-limited minimization for every Context Sensing event, including deep-sensing paths.
- Extended bound, point-of-use authorization validation to Workspace Management, Memory, Context Sensing, Intelligence, and activated Extension Host operations.
- Restricted Intelligence persistence to metadata-only transaction records so it cannot become an alternate Memory store.
- Added an authorized Companion-to-Extension-Host management path while keeping extension contributions event-only.
- Routed capability health only to Runtime Host and aggregate host status to Experience.
- Defined cross-capability correlation semantics and offline core-functionality acceptance scenarios.
- Reconciled matrix validation, scope-event, revocation, and health-event permissions with the capability contracts.
- Added an explicit Experience path for authorized read-only Workspace queries and protected Memory item explanations with `memory.read`.
- Restricted persisted Companion records to metadata-only outcomes/correlation; retained user content must flow through Memory.
- Completed the Experience Workspace-read authorization path across outputs, dependencies, graph, and interaction summary.
- Corrected Runtime Host mode delivery to Intelligence, declared Permission Authority for protected Workspace reads, and separated Extension Host events from call dependencies.
- Separated domain-call graph notation from lifecycle/event paths, added the Runtime mode event policy, and completed Experience's Permission Authority dependency description.

Validation:
- Reviewed every capability for responsibility, ownership, boundary, cohesion, coupling, dependency direction, permissions, explainability, data/lifecycle ownership, failure isolation, Local First, Privacy First, Complexity Budget, and Blueprint alignment.
- Re-ran the review after revisions: no ownership conflicts, circular hard dependencies, Blueprint violations, or accepted ADR violations remain.
- No Critical findings and no unresolved Major findings remain.
- No runtime code or behaviour changed; no technologies selected.

Knowledge Gained:
- Public contracts are insufficient as trust boundaries without explicit caller and intermediary restrictions.
- Observation permission must remain separate from retention permission.
- Authorization must be bound and revalidated at the execution boundary to preserve revocability.
- Event return paths must not be represented as reverse hard dependencies.
- Deep-sensing authorization does not waive data minimization.
- Explainability needs shared correlation semantics without shared data ownership.

Unlocks: Capability contract specification and offline/degraded acceptance scenarios; technology research can proceed against explicit communication boundaries.

Supersedes: Capability Architecture v1.0 boundary definitions where revised by v1.1 and the Interaction Matrix.

Status: Complete

### LEDGER-0004

Entry ID: LEDGER-0004

Capability: Workspace capability communication contracts (system-wide)

Research: N/A — contract architecture only; no technology research or vendor selection performed.

Decision: Adopt `architecture/10_Capability_Contracts.md` as the authoritative public contract definition subordinate to the Blueprint, accepted ADRs, Capability Architecture, and Interaction Matrix. Standardize request, command, response, event, authorization, correlation, error, state exposure, and explainability semantics. Define every capability's public/internal boundary and every interaction permitted by the matrix.

Implementation:
- Defined public responsibilities/interfaces, internal responsibilities, consumed/emitted events, accepted/produced commands, accepted requests, returned responses, owned/exposed state, errors, permissions, and explainability for all ten capabilities.
- Defined 38 exhaustive inter-capability interaction contracts with initiator, receiver, purpose/trigger, expected response, failure behaviour, permission boundary, exchanged data, and conceptual sync/async mode.
- Required bound point-of-use authorization for protected reads, explanations, mutations, inference, Action status/cancellation, and extension management.
- Preserved Memory as sole owner of durable user knowledge; restricted Intelligence, Action, Companion, and Extension Host records/partitions to metadata, minimized summaries, configuration, or disposable cache as appropriate.
- Clarified that sensing start/resume require proof while pause/stop require owning task/session identity and only reduce observation.
- Reconciled Capability Architecture and Interaction Matrix language for Companion-mediated Workspace mutation, permission catalogue/explanation, event inventories, provider/extension state, and protected explanation paths.

Validation:
- Performed repeated adversarial reviews against Blueprint, ADRs, Capability Architecture, and Interaction Matrix.
- Confirmed no circular ownership, duplicate authority, Permission Authority bypass, forbidden direct interaction, alternate Memory owner, or hard dependency cycle remains.
- Confirmed Intelligence never performs actions and Experience remains presentation-only.
- Confirmed every allowed interaction and public interface has a defined permitted contract; every unlisted interaction remains forbidden.
- No runtime code or behaviour changed; no technology research or selection performed.

Knowledge Gained:
- Public method names are insufficient without explicit caller, event subscriber, authorization, failure, and state-exposure semantics.
- Cancellation and safety-reducing stop operations need authority tied to the originating task/session without requiring permission to continue the risky operation.
- Extension isolation must constrain retained data as well as execution authority.
- Event inventories and interaction matrices must remain mechanically reconcilable to prevent undocumented communication paths.

Unlocks: Versioned field-schema definition, compatibility rules, and executable contract/offline/cancellation acceptance specifications.

Supersedes: None. Refines contract details in `08_Workspace_Capability_Architecture.md` and allowed-purpose wording in `09_Capability_Interaction_Matrix.md`.

Status: Complete

### LEDGER-0005

Entry ID: LEDGER-0005

Capability: Workspace contract schema and architectural acceptance specification (system-wide)

Research: N/A — architecture specification and validation only; no technology research, format selection, programming-language type, or implementation performed.

Decision: Adopt `architecture/11_Contract_Schema_and_Acceptance_Specification.md` as the authoritative conceptual schema, evolution policy, and pre-implementation acceptance gate subordinate to the Blueprint, accepted ADRs, Capability Architecture, Interaction Matrix, and Capability Contracts.

Implementation:
- Defined message identity, correlation, causation, task/operation/authorization/extension identity, purpose, request/response, command, event, error, state-reference, explainability, versioning, compatibility, cancellation, timeout, retry, idempotency, audit, and trace semantics without selecting an implementation format.
- Defined required ownership, communication, authority, state, lifecycle, metadata, permission, privacy, Local First, Blueprint, and ADR invariants.
- Defined 42 conceptual acceptance cases covering duplicates, ordering, event loss, challenge redemption, point-of-use authorization, revocation races, multi-effect operations, cancellation, owner-indeterminate outcomes, audit minimization, offline behavior, compatibility, delayed reconciliation, and control-proof revocation/outage.
- Refined Capability Contracts with owner-authoritative status recovery, challenge proof redemption, authorization use/consumption ordering, Memory redaction outcomes, shared partial/indeterminate semantics, and content-free historical terminal lookup.
- Introduced Permission Authority-issued operation-control proofs for non-immediate protected work. They grant only minimized status and safety-reducing control, have an independently revocable bounded offline lease, cannot create effects, and preserve safe control during temporary Authority unavailability.
- Reconciled Capability Architecture and Interaction Matrix language for point-of-use use/consumption, sensing/action safety controls, partial/indeterminate outcomes, and offline control validation.

Validation:
- Performed repeated adversarial reviews against Blueprint, ADR-0001–0008, Capability Architecture, Interaction Matrix, and Capability Contracts.
- Resolved ambiguity between requester-observed outcome-unknown and owner-authoritative indeterminate terminal outcomes.
- Resolved missing recovery paths for non-immediate domain operations, control-plane operations, lost acceptance responses, expired control proofs, and lost/unknown terminal events.
- Resolved control-proof authority, revocation, Permission Authority outage, irreversible-effect, audit privacy, error taxonomy, and exhaustive event-path conflicts.
- Confirmed no blocking or actionable architecture findings remain.
- No runtime code or behavior changed; no technology selected.

Knowledge Gained:
- Timeout, requester uncertainty, and owner terminal state must remain distinct.
- Side-effect authorization and status/safety-control authority require independent lifecycles.
- Revocation must order against each meaningful effect; multi-effect operations cannot rely on one stale validation.
- Auditability requires content-free classes and bounded tombstones, not durable copies of user purpose/content.
- Compatibility is semantic: parsing success is insufficient when authority, privacy, ownership, or outcome meaning differs.

Unlocks: Capability technology research against stable conceptual contracts and acceptance gates; later implementation-specific representations and executable validation artifacts.

Supersedes: None. Refines conditional envelope presence and recovery/control semantics in `08_Workspace_Capability_Architecture.md`, `09_Capability_Interaction_Matrix.md`, and `10_Capability_Contracts.md`.

Status: Complete

### LEDGER-0006

Entry ID: LEDGER-0006

Capability: Blueprint user-journey architecture validation (system-wide)

Research: N/A — architecture validation only; no technology research, selection, implementation, or runtime change performed.

Decision: Adopt `architecture/12_User_Journey_Architecture_Validation.md` as the current validation of whether Blueprint-implied user journeys are supported end-to-end. Confirm the capability decomposition is complete, but record ten interaction/guidance gap groups consolidated into nine existing-capability change packages.

Implementation:
- Traced 13 core user journeys plus optional remote Intelligence, voice, and extension paths.
- Validated participating capabilities, interactions, permission checks, explanations, persistence, recovery/failure behavior, Blueprint principles, and ADR compliance for each journey.
- Confirmed no new core capability is required.
- Identified confirmed gaps in Experience task control/inspection, user shutdown, sensing subscription/session control, task cancellation/crash recovery, local provider administration, optional remote multi-scope authorization, startup dependency classification, attention/consent behavior, archived-workspace Memory visibility, and per-operation cancellation declarations.
- Identified simplification opportunities: stop duplicating detailed contract signatures in Capability Architecture, mechanically reconcile the Interaction Matrix with interaction contracts, and keep voice/extensions within existing optional boundaries.

Validation:
- Performed independent journey tracing and adversarial Blueprint/UX review.
- Removed unsupported findings for default-workspace bootstrap authority, mandatory retention-policy editing, deletion/bulk forgetting, and missing historical-explanation architecture.
- Confirmed permission isolation, Local First structure, data ownership, honest partial/unknown/indeterminate outcomes, and extension isolation are strong.
- Confirmed current architecture authorities contradict each other for Experience cancellation/status/explanation and user-initiated shutdown lacks a permitted caller.
- Confirmed bounded Capability Technology Research may begin, while final selection/implementation in affected areas remains gated by relevant architecture corrections and acceptance.

Knowledge Gained:
- Capability completeness does not imply user-journey completeness.
- Human control requires permitted invocation paths for cancellation, inspection, degraded administration, and shutdown—not only receiver-side contracts.
- Calm/non-manipulative behavior needs architecture acceptance invariants while leaving minimal metadata design to later Experience research.
- Local First requires a user-manageable local provider path, not only an unset/degraded provider state.
- Architecture validation must distinguish required Blueprint behavior from optional product choices.

Unlocks: Nine bounded architecture change packages and bounded Capability Technology Research that records unresolved assumptions without selecting against incomplete requirements.

Supersedes: None. Challenges prior “no actionable architecture findings” readiness claims only at the user-journey layer; capability ownership remains stable.

Status: Complete

### LEDGER-0007

Entry ID: LEDGER-0007

Capability: Workspace Capability Technology Research Framework (system-wide)

Research: N/A — research planning and governance only; no candidate technology research, library recommendation, vendor selection, or implementation performed.

Decision: Adopt `architecture/12_Capability_Technology_Research_Framework.md` as the authoritative research-planning and technology-evaluation process subordinate to the Blueprint, accepted ADRs, Capability Architecture, Interaction Matrix, Capability Contracts, and Contract Schema and Acceptance Specification. Require one canonical Research Catalogue record per bounded capability/category question, one shared research template, mandatory evidence gates, explicit build-versus-integrate comparison, preserved rejection history, and triggered re-evaluation.

Implementation:
- Created the canonical research lifecycle and template covering capability objective, questions, functional/non-functional requirements, Local First, privacy, security, performance, explainability, licensing, maintenance, community maturity, platform compatibility, integration complexity, extensibility, failure modes, migration, build/integrate reasoning, and acceptance.
- Defined capability-specific research profiles for all ten capabilities without duplicating the shared process.
- Defined required adoption evidence, comparison criteria, documentation standards, decision recording, rejected-technology records, and future re-evaluation.
- Kept the Research Catalogue as the single investigation record, the Open Source Registry as approved dependency inventory, ADRs as durable architecture decisions, the Ledger as engineering consequence history, and Current State as present status.
- Recommended Permission Authority as the first capability to research because its local authorization, revocation, replay, and operation-control semantics constrain all protected capability integrations.
- Updated Current State with framework completion, research-stage risks, and the next research task.

Validation:
- Reviewed against ADR-0002: every commodity option requires research, internal construction requires evidence of unique Workspace value, and integration remains replaceable.
- Reviewed against the Blueprint: Local First, Human First, Privacy First, Permission Before Automation, explainability, unique-value construction, integration, Complexity Budget, documentation isolation, and cognitive-load principles are mandatory evaluation gates.
- Reviewed against the Contract Schema and Acceptance Specification: candidate evidence must map to contract semantics, invariants, failure behavior, and applicable acceptance cases; syntactic compatibility and happy paths are insufficient.
- Confirmed no duplicate research process: the framework owns method, while Catalogue, Registry, ADRs, Ledger, and Current State retain distinct record authorities.
- Confirmed no technology research, recommendation, selection, runtime code, capability ownership, interaction path, authority boundary, or contract meaning changed.

Knowledge Gained:
- Technology fitness must be evaluated against semantic authority, privacy, failure, recovery, and migration behavior, not only API or feature fit.
- Mandatory gates must precede weighted comparison so popularity or aggregate scores cannot hide architecture, security, licence, or Local First failures.
- Logs, telemetry, caches, indexes, provider histories, and prototypes are research risks because they can create alternate data ownership or misleading evidence.
- Offline and Windows compatibility require reproducible lifecycle evidence, not broad support claims.
- Extension Host feasibility research cannot activate the dormant capability.

Unlocks: Evidence-based capability technology research beginning with Permission Authority; later adoption decisions recorded once in the Research Catalogue and validated against stable architecture contracts.

Supersedes: None.

Status: Complete

### LEDGER-0008

Entry ID: LEDGER-0008

Capability: Permission Authority capability research

Research: Completed canonical research unit `PA-001` in
`architecture/research/PERMISSION_AUTHORITY_RESEARCH.md`. Compared
authorization architecture patterns, mature open-source policy evaluators,
relationship authorization services, capability-token primitives, approval
workflows, Local First permission models, auditability, explainability,
revocation, temporary and persistent grants, and desktop application permission
models.

Decision: No technology selected. Retain six candidate solution categories for
later bounded evaluation. Keep permission/consent lifecycle, grant and challenge
state, effect/control-proof semantics, point-of-use consumption and revocation
ordering, content-free audit, and stable explanations Workspace-owned. Treat
policy evaluation, relationship evaluation, cryptographic proof primitives,
transactional persistence, and Windows key protection as potential commodity
integration boundaries only.

Implementation:

- Created the detailed Permission Authority research record using the canonical
  Capability Technology Research Framework.
- Created canonical Research Catalogue entry `PA-001`.
- Recorded current open-source comparator versions, licences, maintenance
  evidence, platform implications, architectural trade-offs, failure modes,
  migration risks, unanswered questions, and mandatory acceptance criteria.
- Updated Current State to mark research complete and await architecture review.
- Made no runtime, Open Source Registry, ADR, capability ownership, interaction,
  contract, or technology-selection change.

Validation:

- Mapped research to Permission Authority contracts `PER-REQ-001` through
  `PER-EVT-003`, interactions `IC-006` through `IC-012`, and applicable
  acceptance cases.
- Verified current primary-source evidence for candidate releases, licensing,
  local/Windows support, permission lifetimes, consistency, audit behavior, and
  desktop approval models as of 2026-08-01.
- Corrected unreleased Cedar 4.12.0 evidence and evaluated released Cedar
  4.11.2 instead.
- Confirmed all examined candidates require Workspace-owned challenge,
  proof-consumption, operation-control, audit, and explanation layers.
- Confirmed no candidate was adopted, conditionally adopted, or rejected as a
  technology selection.

Knowledge Gained:

- Policy evaluation, relationship evaluation, proof authenticity, revocation,
  one-use consumption, consent workflow, audit integrity, and explanation are
  distinct mechanisms that must be composed explicitly.
- Self-contained proof verification cannot provide immediate effect revocation
  without authoritative mutable state.
- The effect/control-proof split gives effect authority strict local consistency
  while permitting only bounded, non-effecting safety control during Authority
  outage.
- Desktop permission systems converge on in-context least authority, explicit
  lifetime, independent revocation, trusted prompt ownership, and point-of-use
  enforcement, but their kernel/broker guarantees do not transfer to an
  ordinary in-app prompt.
- Content-rich decision traces are diagnostic artifacts, not acceptable
  default permission audit.

Unlocks: Architecture review of unresolved Permission Authority assumptions,
followed by bounded candidate validation against the recorded mandatory gates.

Supersedes: None.

Status: Complete

### LEDGER-0009

Entry ID: LEDGER-0009

Capability: Memory capability research

Research: Completed canonical research unit `MEM-001` in
`architecture/research/MEMORY_RESEARCH.md`. Compared local-first AI-memory
architecture, short-term working and long-term semantic memory, canonical and
structured knowledge storage, dense/vector and sparse/full-text indexes,
graph-based relationships, hybrid retrieval, reranking, compression,
consolidation, pruning, versioning, privacy, encryption, optional
synchronization, and retrieval explainability.

Decision: No technology selected or rejected. Retain eight candidate solution
categories and representative mature open-source comparators for later bounded
evaluation. Keep canonical identity, propose-write authorization, provenance,
scope, retention, correction, redaction/forgetting, derivation lineage,
retrieval orchestration/explanation, and operation recovery Workspace-owned.
Treat databases, lexical/vector/graph indexes, cryptography, model runtimes,
retrieval pipelines, and synchronization primitives as potential replaceable
commodity integrations only.

Implementation:

- Created the detailed Memory research record using the canonical Capability
  Technology Research Framework.
- Created canonical Research Catalogue entry `MEM-001`.
- Recorded current open-source comparator versions, licences, maintenance and
  platform implications, architectural trade-offs, failure modes, migration
  risks, unknowns, and mandatory acceptance criteria.
- Distinguished ephemeral Companion working context from durable Memory.
- Defined derived indexes, summaries, graphs, caches, and framework stores as
  rebuildable projections that cannot become alternate Memory authorities.
- Updated Current State to mark research complete and await architecture review.
- Made no runtime, Open Source Registry, ADR, capability ownership, interaction,
  contract, technology-selection, or technology-rejection change.

Validation:

- Mapped research to Memory contracts `MEM-CMD-001` through `MEM-EVT-005`,
  interactions `IC-015` and `IC-019` through `IC-022`, and applicable
  acceptance cases.
- Verified current primary-source evidence for representative storage, search,
  graph, framework, encryption, and synchronization releases as of 2026-08-01.
- Covered every requested research scope and every canonical framework section.
- Confirmed retrieval quality, performance, Windows packaging, exact deletion,
  encryption, synchronization, and failure behavior remain evidence gates rather
  than inferred passes.
- Confirmed no technology was adopted, conditionally adopted, rejected, or added
  to the Open Source Registry.

Knowledge Gained:

- Bounded working context belongs to Companion's in-flight task state; durable
  retention remains an explicit, separately authorized Memory operation.
- Canonical records and rebuildable derived projections provide the clearest
  boundary for provenance, corruption recovery, migration, and forgetting.
- Lexical, dense, metadata/time, graph, fusion, reranking, and compression
  stages solve different retrieval problems and need separate evidence.
- Dense similarity is not an explanation; trustworthy recall requires source,
  scope, temporal, rank/fusion, graph-path, and transformation evidence.
- Search invisibility, logical deletion, cryptographic erasure, replica
  propagation, backup expiry, and physical-media sanitization are distinct
  guarantees.
- CRDT convergence does not provide authorization, semantic conflict resolution,
  or complete forgetting.

Unlocks: Architecture review of unresolved Memory taxonomy, archived-scope,
retention, correction, deletion, encryption, synchronization, and operation
assumptions, followed by bounded reproducible candidate validation.

Supersedes: None.

Status: Complete

### LEDGER-0010

Entry ID: LEDGER-0010

Capability: Remaining capability research roadmap (system-wide)

Research: Planning and architecture synthesis only. Reviewed the completed
Permission Authority and Memory research, all capability research profiles,
capability dependencies, interaction and contract boundaries, acceptance
requirements, user-journey gaps, Blueprint principles, and accepted ADRs. No new
technology research was performed.

Decision: Establish `architecture/13_Capability_Research_Roadmap.md` as the
dependency-driven sequence for the eight remaining research areas. Research
Runtime Host first, then Workspace Management; run Context Sensing, Action, and
Intelligence as a bounded parallel wave after shared prerequisites; then
research Companion Orchestration and Experience; research Extension Host only
after separate activation justification.

Implementation:

- Created the research roadmap with dependency waves, architecture gates,
  parallelism rules, stop conditions, and validation.
- Defined for every remaining capability why research is required, prerequisite
  completed research, expected outputs, unanswered questions, architectural
  risks, and estimated complexity.
- Preserved `PA-001` and `MEM-001` as completed prerequisites without treating
  either as a selection.
- Recorded user-journey corrections as architecture gates rather than silently
  resolving them through research.
- Added Catalogue planning record `ROADMAP-001`.
- Updated Current State to make Runtime Host the first remaining research area
  after roadmap review.
- Made no runtime, technology, Open Source Registry, ADR, ownership,
  interaction, contract, or acceptance-semantics change.

Validation:

- Confirmed root lifecycle and shared scope research precede dependent
  candidate comparisons.
- Confirmed Context Sensing, Action, and Intelligence have no direct dependency
  requiring arbitrary serialization and may proceed in parallel after shared
  prerequisites.
- Confirmed Companion follows the domain owners it coordinates and Experience
  follows the states it presents.
- Confirmed Extension Host remains dormant and conditional under the Complexity
  Budget.
- Confirmed every remaining capability is covered and no new capability or
  duplicate research process was introduced.
- Confirmed ADR-0002 is upheld by requiring explicit build, integrate, and
  hybrid comparison before implementation while preserving replaceable
  commodity boundaries.

Knowledge Gained:

- Research sequencing is a dependency graph rather than a fully serial list.
- Runtime lifecycle and Workspace scope have the largest remaining upstream
  influence on later integration evidence.
- Sensing privacy, Action safety, and Intelligence viability can be investigated
  concurrently, but all three must complete before Companion orchestration is
  compared.
- Experience selection depends on authoritative domain and orchestration states,
  not only UI requirements.
- Optional extension feasibility cannot substitute for activation
  justification.

Unlocks: Architecture review of `ROADMAP-001`, resolution or explicit bounding
of relevant user-journey gaps, and Runtime Host capability research as the first
remaining research unit.

Supersedes: The non-binding suggested subsequent order in
`12_Capability_Technology_Research_Framework.md` only where this roadmap adds
dependency waves, explicit prerequisites, parallelism, and architecture gates;
the framework's research process remains authoritative.

Status: Complete

### LEDGER-0011

Entry ID: LEDGER-0011

Capability: Engine and companion experience architecture principle (system-wide)

Research: N/A — architecture documentation review only; no technology research,
product redesign, implementation planning, or runtime change performed.

Decision: Make explicit that the engine exists to serve the companion
experience and that internal capabilities and their operational surfaces must
not become the primary user interface. Record this in the highest-authority
Blueprint and apply it as a user-journey validation criterion.

Implementation:

- Revised the Blueprint to v1.1 and added the engine/companion experience
  separation principle.
- Revised User Journey Architecture Validation to v1.1, adding the separation
  criterion and clarifying that capability, administration, and diagnostic
  surfaces are supporting or inspectable paths rather than substitutes for the
  primary Companion experience.
- Updated Current State to record the explicit distinction.
- Made no ADR, capability ownership, interaction, contract, acceptance-schema,
  product design, technology, or runtime change.

Validation:

- Reviewed the complete Cursor Protocol reading set and the requested Blueprint,
  Current State, Capability Architecture, User Journey Architecture Validation,
  and architecture-guardian concerns.
- Confirmed the principle was strongly implied by the Blueprint mission and
  Companion principles, by Companion Orchestration as the sole coordinator, and
  by Experience as the human-facing presentation owner.
- Confirmed implication alone did not prevent internal capability and diagnostic
  surfaces from being mistaken for the product experience.
- Confirmed the minimum change belongs in the Blueprint as governing authority
  and in User Journey Architecture Validation as the place where end-to-end
  experience completeness is tested.

Knowledge Gained:

- Capability completeness and internal operability do not establish a mature
  user-facing companion experience.
- The architecture must judge internal surfaces by how they support user
  journeys, not treat those surfaces as journey completion.
- This distinction constrains presentation without changing capability
  ownership or runtime architecture.

Unlocks: Future architecture and journey reviews can reject engine-first
surfaces as substitutes for the Companion experience without introducing new
capabilities or changing contracts.

Supersedes: None.

Status: Complete

### LEDGER-0012

Entry ID: LEDGER-0012

Capability: Runtime Host research repository audit

Research: N/A — repository audit only; no technology research, architecture
change, product evaluation, or implementation performed.

Decision: Classify `RH-001` as missing. No accepted Runtime Host research
artifact, Research Catalogue entry, introducing commit, supersession record,
alternate filename, branch copy, or recoverable Git object exists in the
available repository. No repository reference incorrectly assumes `RH-001` is
complete; the unsupported completion claim came from a prior engineering
session rather than a committed project artifact.

Implementation:

- Updated Current State to record the missing artifact and block evaluations
  that require `RH-001`.
- Did not recreate Runtime Host research or alter architecture, contracts,
  acceptance criteria, ADRs, or runtime code.

Validation:

- Read the complete Cursor Protocol startup set: Blueprint, Current State,
  Engineering Ledger, and Research Catalogue.
- Fetched all remote refs and scanned 500 commits across all refs and reflogs
  for the `RH-001` identifier, Runtime Host research completion claims, commit
  messages, and alternate research filenames.
- Searched every repository reference to Runtime Host research. All describe it
  as future, prerequisite, or in progress; none records completion.
- Checked unreachable Git objects; no unreachable commit or blob containing a
  possible missing artifact was available.
- Confirmed no Research Catalogue identifier, introducing commit hash, or
  superseding record can be reported.

Knowledge Gained:

- The repository consistently records Runtime Host research as the first
  remaining research unit.
- A session report is not project authority without a committed canonical
  artifact, Catalogue entry, and Ledger/Current State record.
- Repository history can establish that `RH-001` is absent from available Git
  evidence; it cannot prove whether uncommitted work once existed outside the
  repository.

Unlocks: Architecture review can resolve the process inconsistency without
treating the missing research as completed or recreating it during an audit.

Supersedes: None.

Status: Complete
