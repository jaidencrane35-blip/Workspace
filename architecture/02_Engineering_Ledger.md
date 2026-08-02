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

### LEDGER-0013

Entry ID: LEDGER-0013

Capability: Product strategy — pre-implementation Product Proof gate

Research: Strategic product-evidence review only. Reviewed the complete
repository and architecture pack, existing product vision and MVP, engineering
history, prior architecture and product reviews, supplied UI direction, current
Windows integration, and implemented user surfaces. No technology evaluation,
architecture redesign, runtime change, or implementation plan was performed.

Decision: Sustained capability expansion and large-scale implementation are not
justified until Workspace proves one measurable product hypothesis. The
strongest current hypothesis is trusted interruption recovery:

> Help an interruption-heavy Windows professional return to a recurring work
> context and intended next action materially faster than existing tools,
> without privacy surprise.

This is a hypothesis, not an accepted customer fact. Product proof precedes the
remaining Capability Technology Research milestone. `ROADMAP-001` remains the
valid dependency order if product evidence later justifies resuming that work.

Product Proof Strategy:

- Primary user: a Windows-based independent consultant managing at least three
  recurring client contexts, switching at least five times per day, and
  currently losing at least five minutes reconstructing each context.
- Primary workflow: explicitly save one bounded resume point containing selected
  applications, supported resources, window arrangement, and a short
  user-authored handoff note; later preview the proposed restore, resume
  supported items, receive honest per-item outcomes, and inspect or delete the
  retained context.
- Measurable objective: in a four-week pilot with 15 target users, reduce median
  return-to-work time by at least 50 percent relative to each user's baseline,
  with zero unpreviewed capture or action.
- Success metric: median percentage reduction in return-to-work time; threshold
  at least 50 percent.
- Failure metric: fewer than 50 percent of pilot users use Resume on at least
  three days during week four. Crossing this threshold means the hypothesis did
  not establish habitual value.
- Trust invalidation: any unpreviewed capture or action, serious unexpected
  window disturbance, or misleading claim of successful restoration invalidates
  the proof regardless of the success metric.
- Minimum UI: named context list, explicit save checklist, retained-data review,
  restore preview, one Resume action, per-item progress and partial-result
  recovery, and inspect/delete controls.
- Minimum AI behavior: no AI dependency in the critical path. At most one
  optional, evidence-cited, non-executing summary or context-update suggestion
  is tested separately after the deterministic resume workflow succeeds.
- Smallest proof milestone: one installable Windows experience that saves and
  resumes one supported context end to end and produces the defined pilot
  evidence. It does not claim arbitrary application-state restoration.

Minimum capability set:

- Runtime Host for reliable local startup and shutdown
- Workspace Management for one named bounded context
- Permission Authority limited to capture and restore consent
- Context Sensing limited to explicit user-initiated capture
- Memory limited to the user-authored handoff and inspectable retained context
- Action limited to supported open, launch, reuse, and window placement effects
- thin Companion Orchestration for the save/preview/resume sequence
- Experience for the complete primary workflow and trust controls

Intelligence is optional and non-critical for the proof. Extension Host remains
dormant.

Work stopped by this decision:

- new capability layers and cognition projections
- further hardening of Decision, Attention, Recommendation, Adaptation,
  Evolution, Readiness, Working Style, Milestone, Transition, Profile, Session,
  and similar engines unrelated to the proof
- Extension Host, plugin marketplace, voice, cloud sync, phone, audio, device,
  broad automation, multi-provider, and broad local-model work
- Architecture Guardian activation and additional governance expansion
- product UI that exposes capability, engine, queue, proof, contract, or
  diagnostic concepts as the primary experience

Work accelerated by this decision:

- observed customer baseline and substitute comparison
- privacy containment and explicit-capture trust behavior
- truthful green build and installable Windows validation
- supported application/resource discovery, launch/reuse, placement, and honest
  partial restoration needed by the primary workflow
- calm first-class save, preview, resume, inspect, and delete experience
- consented measurement of return-to-work time, correction, repeat use, and
  proof invalidation

Validation:

- Confirmed the existing MVP already points toward save and restore, but joins
  that value proposition to passive pattern observation and app-launch
  automation without customer evidence.
- Confirmed no repository evidence of customer interviews, baseline task-time
  studies, comparative usability tests, retained-use cohorts, willingness to
  pay, or public product metrics.
- Confirmed credible substitutes include Windows-native behavior, app-native
  restoration, PowerToys Workspaces, and manual notes.
- Confirmed the current implementation can enumerate windows and launch
  processes but does not yet prove installed-application discovery, real window
  placement, supported resource restoration, or end-to-end interruption
  recovery.
- Confirmed the proof uses existing capability ownership and requires no
  Blueprint, ADR, Capability Architecture, contract, interaction, research
  record, or technology change.
- Confirmed product proof can falsify the wedge: if semantic context adds no
  value beyond geometry, the result is a window-management product; if neither
  changes return-to-work behavior, capability expansion remains unjustified.

Knowledge Gained:

- Trusted continuity is not yet the product wedge; it is the most credible
  hypothesis to test.
- Permission quality is necessary for trust but is not itself a reason to adopt
  Workspace.
- Layout restoration proves too little unless it returns the user to a
  meaningful artifact and intended next action.
- General AI is unnecessary for the first product proof and may reduce trust
  before it adds value.
- Capability completeness, sprint count, contract depth, and internal engine
  operability are not evidence of product demand.

Unlocks: A bounded Product Proof milestone. Sustained capability research and
large-scale implementation unlock only if the product objective, success
metric, habitual-use threshold, and trust conditions pass.

Supersedes: Current State's recommendation to begin Capability Technology
Research immediately. It does not supersede `ROADMAP-001` dependency ordering,
the Blueprint, ADRs, capability ownership, contracts, or research records.

Status: Complete; Product Proof required before sustained engineering

### LEDGER-0014

Entry ID: LEDGER-0014

Capability: Runtime Host (kernel build integrity) — Product Proof blocker
`PP-B01`

Research: N/A — repair of an existing compile blocker. No technology research,
no design change.

Decision: Restore a truthful green kernel baseline by correcting
`resilience_validation.rs` against the domain API that actually exists, rather
than adding the missing domain types to satisfy the stale references. Product
Proof cannot be measured on a tree that does not build, and inventing domain
types to preserve dead assertions would encode an assumption nobody had
validated.

Implementation:
- `packages/kernel/src/services/resilience_validation.rs`: replaced the
  unresolved import and parameter type `workspace_domain::DecisionCandidateProgression`
  with the real `DecisionCandidateProgressionRequest`.
- Removed the guard reading `creation.score`, a field that does not exist on
  `DecisionEngineCandidateCreation`.
- No other file touched; the change is 4 insertions and 11 deletions.

Validation:
- `cargo check -p workspace-kernel` compiles, having previously failed with
  `E0432` and `E0609`.
- `cargo build --workspace` succeeds.
- The repository-wide `cargo fmt --all -- --check` failure is pre-existing and
  unrelated; only the touched import was rewrapped to the local column limit.

Knowledge Gained:
- `main` had been red long enough for a safety-critical invariant module to
  reference domain types that no longer exist, and CI on `windows-latest` was
  red for the same reason.
- A library compile failure masks defects in downstream test code; repairing it
  exposed, rather than caused, the follow-on failure recorded as `PP-B04`.

Unlocks: Every later Product Proof blocker, all of which need a building tree
to validate against.

Supersedes: None.

Status: Complete (commit `2d88331`)

### LEDGER-0015

Entry ID: LEDGER-0015

Capability: Runtime Host (kernel test integrity) — Product Proof blocker
`PP-B04`

Research: N/A — repair of pre-existing test code exposed by `PP-B01`.

Decision: Rewrite the stale assertions in `test_candidate_selection_immutability`
to express the original behavioural intent against the current public API, and
stop rather than weaken the test if the invariant could not be observed. Do not
change production code, domain models, or handler signatures to accommodate the
test.

Implementation:
- `packages/kernel/src/commands/resilience_tests.rs`: passed
  `candidate.id.to_string()` where a `String` is required instead of a
  `DecisionCandidateId`.
- Replaced the assertion on `DecisionEngineActionResult::created`, a field that
  does not exist, with assertions that selection moves the `DecisionOutcome` to
  `Selected`, grants no execution authority, and leaves candidate identity,
  provenance, and score unchanged.

Validation:
- `cargo test -p workspace-kernel` and `cargo test --workspace` pass.
- Negative probes confirmed each new assertion fails when the behaviour it
  describes is broken, so the test is load-bearing rather than decorative.

Knowledge Gained:
- The obsolete assertions described a candidate-creation shape the Decision
  Engine no longer has; the invariant worth keeping is that selection is a
  non-authorising, non-mutating transition.
- Tests hidden behind a broken library can drift arbitrarily far from the code
  they claim to protect without anyone noticing.

Unlocks: A trustworthy kernel test suite as the regression baseline for
`PP-B02`, `PP-B03`, and Product Proof Milestone 1.

Supersedes: None.

Status: Complete (commit `da6a85e`)

### LEDGER-0016

Entry ID: LEDGER-0016

Capability: Context Sensing / Permission Authority — Product Proof blocker
`PP-B02`, zero ambient capture

Research: N/A — enforcement of an existing Blueprint principle. No new sensing,
memory, or permission design.

Decision: Workspace performs no desktop observation, window enumeration,
context capture, persistence, or background sensing until the user explicitly
initiates a Save. Retain the ambient implementations rather than delete them,
and close them at the existing admission choke point so the trust property does
not depend on remembering not to call them.

Implementation:
- `packages/kernel/src/lib.rs`: `WorkspaceKernel::initialize` no longer fires
  `ObservationStartupTrigger` and no longer starts the scheduler with
  `ObservationScheduleConfig::enabled_default()`; it now starts it explicitly
  with `ObservationScheduleConfig::disabled()`.
- `packages/kernel/src/services/observation_trigger_admission.rs`: added an
  `AMBIENT_CAPTURE_AUTHORIZED` flag defaulting to closed. `source_is_admitted`
  now admits `Manual` only; `System` and `Scheduled` are refused with an
  explanation stating that observation requires explicit user-initiated
  capture.
- `observation_startup_trigger.rs`, `observation_scheduled_trigger.rs`,
  `observation_scheduler.rs`, `observation_trigger_authority.rs`,
  `services/mod.rs`: retained ambient paths marked dormant, their re-export
  removed, and their test helpers authorised explicitly so the dormant code
  stays exercised.

Validation:
- New acceptance test proves a fresh kernel persists no observation pass, no
  window, monitor, or memory rows, and no observation audit event, and that the
  scheduler is disabled and not running.
- New acceptance test proves `System` and `Scheduled` triggers are refused and
  persist nothing while an explicit `Manual` trigger is admitted and persists
  exactly one pass.
- `cargo test -p workspace-kernel` (1127) and `cargo test --workspace` (1443)
  pass; negative probes confirmed both acceptance tests are load-bearing.

Knowledge Gained:
- Two independent startup paths performed unconsented capture: a one-shot
  startup trigger and a scheduler defaulting to a 300-second capture loop.
- Gating at the admission policy rather than only at the call sites means a
  future caller cannot reintroduce ambient capture by accident.
- Keeping the dormant implementations testable requires an explicit test-only
  authorisation; without it the retained code would rot exactly as the
  resilience tests did.

Unlocks: An honest privacy claim for the Product Proof pilot, and the explicit
Save flow that Milestone 1 builds on.

Supersedes: The previous default-on observation behaviour.

Status: Complete (commit `12573da`)

### LEDGER-0017

Entry ID: LEDGER-0017

Capability: Experience / Runtime Host (WebView trust boundary) — Product Proof
blocker `PP-B03`

Research: N/A — configuration hardening. Read the installed Tauri 2.11.5 and
`tauri-utils` 2.9.3 sources to establish what the runtime requires rather than
copying a policy from documentation.

Decision: Replace `"csp": null` with the minimum policy the shipped bundle
actually needs, and pin it with an automated verifier. The WebView holds full
IPC reach into the kernel, so an unrestricted document was the widest remaining
trust gap. Prefer tightening over exceptions: the audit found exactly one
exception in the repository, the disabled policy itself, and it was an
unnecessary scaffold default rather than a requirement.

Implementation:
- `app/src-tauri/tauri.conf.json`: set `app.security.csp` to
  `default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self'; font-src 'self'; connect-src 'self' ipc: http://ipc.localhost; frame-src 'none'; worker-src 'none'; object-src 'none'; base-uri 'self'; form-action 'none'; frame-ancestors 'none'`.
- `scripts/content-security-policy-lib.mjs` and
  `scripts/verify-content-security-policy.mjs`: audit the shipped policy
  against an allowed source set per directive, reject `'unsafe-inline'`,
  `'unsafe-eval'`, wildcards and remote origins, reject a `devCsp` override or
  disabled Tauri nonce injection, and reject HTML the policy would block.
- `tests/content-security-policy.test.ts` and the root `test` script: wire the
  audit into `pnpm test`.

Validation:
- `cargo test --workspace` (1443), `pnpm test` (29), and `pnpm build` pass.
- `pnpm exec tauri build` produces the MSI and NSIS bundles, and the exact
  policy string is present in the shipped `workspace-app.exe`.
- Serving the production bundle under the identical policy in headless
  Chromium: the module script executed, React mounted, the stylesheet applied,
  and zero `securitypolicyviolation` events fired. A deliberately broken policy
  produced a violation, confirming the probe was load-bearing.
- The only console error is the expected absence of Tauri IPC outside the
  desktop shell.

Knowledge Gained:
- No CSP exception was ever required. The frontend loads one same-origin module
  script and one same-origin stylesheet, has no inline script or style, no
  remote origin, no image, font, media, worker, frame, or form, and React's
  `style` props are applied through the CSSOM, which CSP does not govern.
- `connect-src` must name `ipc:` and `http://ipc.localhost`; without them Tauri
  silently degrades from custom-protocol IPC to the slower postMessage
  fallback, which is a functional regression a security-only review would miss.
- Tauri applies the policy only to assets it serves, so `tauri dev` against the
  Vite dev server is unaffected and no weaker `devCsp` is needed.
- `custom-protocol` is a default feature, so `cfg(dev)` is already false in
  ordinary builds and the production CSP path is what the test suite exercises.

Unlocks: A defensible trust claim for the pilot build, and a regression guard
that fails `pnpm test` if the policy is disabled or widened.

Supersedes: The disabled `"csp": null` configuration.

Status: Complete (commit `f77200e`)

---

### LEDGER-0018

Entry ID: LEDGER-0018

Capability: Workspace Management (owner) with Context Sensing (capture) and
Experience (consent presentation) — Product Proof milestone task `PP-M1-01`

Research: N/A — first user-visible Product Proof feature, built on the existing
command pipeline, permission gateway, capture coordinator, and observation
capture path. No new capability, contract, or ownership was introduced.

Decision: Make the capture scope a kernel-served artefact and make the user's
confirmation of it a validated precondition of capture, rather than treating the
preview as interface copy and the capture as a separate act.

`SavedContextCaptureScope` is returned by a query command and declares, in the
user's language, every field the capture will record and the categories it will
not. `SaveContextRequest` carries back the scope identifier the user actually
confirmed, and the save is refused if it is absent, blank, or not the scope this
build would capture. A future widening of the scope therefore invalidates
previously displayed consent instead of silently inheriting it.

A saved context stores its own copy of the windows and monitors it was built
from. Referencing `observation_pass_id` alone was rejected: observation passes
are a rolling perception buffer purged by `purge_older_than_keep`, so a saved
context would have quietly emptied itself and misrepresented what the user
agreed to keep. The pass id is retained as provenance only.

`SaveWorkspaceContext` is a mutation requiring `workspace.write`, because its
lasting effect is a workspace-owned record. Reading the desktop is a separate
effect, so `desktop.read` is checked explicitly as well; saving must not become
a route to desktop state that bypasses the capability governing it. Both checks
complete, along with name validation, consent validation, and workspace
existence, before anything is observed — a refused save observes nothing.

Implementation:
- `packages/database/migrations/041_saved_context.sql`: `saved_contexts`,
  `saved_context_windows`, `saved_context_monitors`, cascading from the owning
  workspace.
- `packages/domain/src/saved_context/mod.rs`: `SavedContextCaptureScope`,
  `SAVED_CONTEXT_SCOPE_ID`, `SavedContext`, `SavedContextWindow`,
  `SavedContextMonitor`, `SaveContextRequest`, `SavedContextError`.
- `packages/database/src/repositories/saved_context.rs`: single-transaction
  write, so a partial context cannot exist.
- `packages/kernel/src/services/saved_context.rs`: validate, then capture once
  through `CaptureCoordinator` with `CaptureRequest::manual()`, then persist.
- `packages/kernel/src/commands/saved_context.rs`:
  `GetSavedContextCaptureScope` (query, observes nothing) and
  `SaveWorkspaceContext` (mutation, audited with the scope and counts).
- `app/src-tauri/src/commands/saved_context.rs`, `app/src/types/domain.ts`,
  `app/src/components/SaveContextPanel.tsx`: name, review, confirm or cancel,
  then a summary of exactly what was kept.

Validation:
- `cargo test --workspace` (1466) and `pnpm test` (33) pass; `pnpm typecheck`
  and `pnpm build` pass; `git diff --check` clean.
- 23 new Rust tests. Five prove that an unnamed context, a missing
  confirmation, a stale confirmation, an unauthorised actor, and an unknown
  workspace each leave `observation_passes` and `saved_contexts` empty.
- One test purges the originating observation pass and shows the saved context
  still returns all four windows and both monitors.
- `tests/saved-context-consent.test.ts` fails if the interface hardcodes a scope
  identifier instead of forwarding the one the kernel served. Verified
  load-bearing by inserting that defect and observing the failure.
- The capture path itself is exercised through `StubDesktopCapturer`, so no test
  reads the real machine.

Knowledge Gained:
- Consent is only meaningful if it names a specific scope version. Recording
  "the user agreed" without recording what they agreed to would let a later
  build widen capture under old consent.
- Durability of a user artefact and retention of a perception buffer are
  different lifetimes; a user-authored record must not depend on a buffer
  designed to be discarded.
- A command with two effects needs both capabilities enforced at the command,
  because the pipeline authorises only the one the command declares.
- Windows reports window titles but not program names or paths, and titles
  routinely name the document or page behind them. The preview states this
  rather than implying titles are anonymous.

Unlocks: `PP-M1-02` (inspect and delete a saved context) and the Resume path,
both of which read the durable record this task establishes.

Supersedes: Nothing.

Status: Complete (commit `868ce12`)

---

### LEDGER-0019

Entry ID: LEDGER-0019

Capability: Action — missing declared action type and implementation. Recorded
while attempting Product Proof milestone task `PP-M1-02` (Resume a bounded
Workspace Context).

Research: Repository and architecture-pack audit only. No technology
evaluation, no architecture change, no runtime change, and no implementation
was performed. `PP-M1-02` was stopped before any code was written.

Decision: Resume requires environment mutation. Environment mutation is owned
solely by Action. The generic Action contract exists, but the declared action
type Resume needs does not, and Action has no implementation at all. Under the
governing ruling for this task — do not create or relocate OS mutation logic
inside `PP-M1-02`, do not bypass capability ownership, and do not implement the
capability inside another subsystem — `PP-M1-02` cannot proceed. It is stopped
and the missing contract is recorded here.

What exists:

- `10_Capability_Contracts.md` defines the Action public interface in full:
  `ACT-CMD-001 Action.execute`, `ACT-CMD-002 Action.cancel`,
  `ACT-REQ-001 Action.describe`, `ACT-REQ-002 Action.getStatus`,
  `ACT-REQ-003 Action.lookupTerminalOutcome`, and `ACT-EVT-001`–`ACT-EVT-007`.
- `ACT-EVT-006 ActionPartiallyCompleted` and the recorded error condition
  "unsupported action" already express the honest partial and unsupported
  reporting `PP-M1-02` requires. That vocabulary does not need inventing.
- `IC-029`–`IC-031` define Companion → Action execution and Action → Companion
  outcome reporting, including "report partial effects honestly".
- `08_Workspace_Capability_Architecture.md` records that Action owns "Action
  allow-list binding to permission scopes" and must "execute only declared
  action types".
- `LEDGER-0013` already scopes Product Proof's Action use to "supported open,
  launch, reuse, and window placement effects", and Permission Authority to
  "capture and restore consent". Window placement is therefore already within
  the accepted Product Proof scope.

What is missing, and blocks `PP-M1-02`:

1. No declared action type for window placement. `ACT-REQ-001` describes an
   `action_type`, and Action may "execute only declared action types", but no
   action-type catalogue exists in the architecture pack or in code. The
   window-placement type, its target grammar, and its preconditions are
   undefined.
2. No safety class, commit points, or compensation rules for that type.
   `11_Contract_Schema_and_Acceptance_Specification.md` records this as an open
   pre-implementation risk: "Each action class still needs specific
   irreversible commit points and compensation rules." Current State carries
   the same gap as the "Action safety taxonomy" known unknown.
3. No per-action-class permission. The Action contract requires "per-action
   class permission; no omnibus grant." The 24 capabilities in
   `packages/domain/src/capability/mod.rs` include `desktop.read` but no
   counterpart governing desktop mutation.
4. No Action implementation of any kind. There is no `Action.execute` in the
   codebase. What the kernel calls "execution" — `ExecuteIntentRequest`,
   `ExecutionOutcome`, `ExecutionReconciliation`, `ExecutionGuardService` — is
   an audit-derived state machine over database mutations. Its dispatch bottoms
   out in `CreateZone`, `GetLayoutSnapshot`, and `GetAuditHistory`. It never
   reaches an OS effect.
5. No OS write adapter. `packages/windows-integration`, the only crate
   permitted to call OS APIs under DEC-008, exposes exactly three traits:
   `DesktopCapturer` (read), `WindowEnumerator` (read), and `ProcessLauncher`
   (spawn). A repository-wide search for `SetWindowPos`, `MoveWindow`,
   `ShowWindow`, `SetForegroundWindow`, `SetWindowPlacement`,
   `BringWindowToTop`, `ShellExecute`, and `CreateProcess` returns zero
   matches. The `windows` crate already enables
   `Win32_UI_WindowsAndMessaging`, so this is an absent implementation rather
   than an absent dependency.

Existing drift that must not be extended: `LaunchApplication` mutates the
environment — it spawns a real process through `ApplicationLaunchService` →
`ProcessLauncher` → `Command::spawn` — as an ordinary kernel `MutationCommand`
guarded by `application.launch`. It does not go through `ACT-CMD-001`, produces
no `ACT-EVT-*` events, carries no operation-control proof, and appears in no
action catalogue. It is therefore environment mutation performed outside the
capability that "alone mutates the environment". This is the precedent that
makes putting window placement into a kernel command look reasonable. Following
it would place a second, larger environment mutation outside Action and
entrench the drift, which is what the governing ruling forbids.

Second gap, introduced by `PP-M1-01` and owned by Workspace Management rather
than Action: a saved context cannot be matched back to a live window.
`saved_context_windows` persists title, `process_id`, geometry, monitor index,
minimized, focused, and z-order. It does not persist `hwnd` or
`stable_window_id`, although the observation layer already computes
`stable_window_id` with a four-level confidence rating
(`High`/`Medium`/`Low`/`Ephemeral`). `process_id` does not survive a restart,
and no executable identity is captured at all — `process_name` is hardcoded to
`None` in `map_window`. A saved context therefore cannot identify which live
window corresponds to a saved one, nor relaunch anything that has closed. This
is a schema omission in `PP-M1-01`, correctable within Workspace Management,
and it is independent of the Action gap above.

Implementation: None. No runtime code, schema, contract, or capability was
added or changed. This entry is a record only.

Validation: `git diff --check` clean. The repository is unchanged apart from
this entry and the corresponding Current State record, so the `PP-M1-01`
baseline of 1466 Rust tests and 33 frontend tests stands unaltered.

Knowledge Gained:
- The Action contract is more complete than the codebase suggests. Partial
  completion, indeterminate outcomes, unsupported-action errors, cancellation,
  and per-action-class permission are all already specified. `PP-M1-02` needs a
  declared action type and an implementation, not a new contract shape.
- The absence of an action-type catalogue is the real blocker. Without it,
  "execute only declared action types" cannot be satisfied by any
  implementation, so no amount of Win32 code would make a restore compliant.
- Environment mutation has already escaped Action once, through
  `LaunchApplication`. Capability ownership erodes through precedent rather
  than through decision, and the second instance is always easier to justify
  than the first.
- Restore is not one gap but two in different capabilities: Action cannot
  perform the effect, and Workspace Management did not retain the identity
  needed to aim it. Closing only one leaves Resume impossible.

Unlocks: Nothing yet. `PP-M1-02` remains blocked pending a declared
window-placement action type, its safety class and permission binding, and an
Action implementation. `PP-M1-03` (Placement Undo & Recovery) depends on the
same prerequisites and is therefore blocked behind it.

Supersedes: Nothing. `LEDGER-0018` incorrectly recorded that `PP-M1-01`
unlocked "the Resume path"; that claim is corrected here. `PP-M1-01` unlocked
the durable record Resume will read, but not the ability to act on it.

Status: Blocked — reported for architectural decision, no implementation
performed

---

### LEDGER-0020

Entry ID: LEDGER-0020

Capability: Action — declared action types and desktop mutation semantics.
Produced to close the contract gap recorded in LEDGER-0019.

Research: Architecture-pack audit only. Reviewed every Action reference across
the Blueprint, `08_Workspace_Capability_Architecture.md`,
`09_Capability_Interaction_Matrix.md`, `10_Capability_Contracts.md`,
`11_Contract_Schema_and_Acceptance_Specification.md`, LEDGER-0013, and the
existing codebase. No runtime code was written and no technology was selected.

Decision: Add `15_Action_Desktop_Mutation_Contract.md` as the authoritative
declaration of Action's action types, and amend `10` and `11` minimally to
reference it. Nothing already defined was redefined.

The audit's central finding is that the Action contract was far more complete
than the codebase implied. Partial completion, indeterminate outcomes,
unsupported-action errors, cancellation classes, idempotency classes, retry
philosophy, the explainability envelope, operation recovery, and per-action
class permission were all already specified. The gap was never a missing
contract shape. It was that no action type had ever been declared, which made
the standing requirement to "execute only declared action types" impossible to
satisfy. The contract therefore declares types and binds them to existing
classes rather than introducing new machinery.

Scope declared for Product Proof:

- `window.place` — position, size, monitor assignment, and minimized/restored
  state for one declared window; scope `action.window.place`
- `window.focus` — foreground assignment for one declared window; scope
  `action.window.focus`

Separated because placement rearranges while focus changes where the next
keystroke lands. A user may permit one and refuse the other. Application launch,
application reuse, file open, URL open, and workspace activation are named as
reserved and deliberately left unspecified.

Four decisions carry the design:

1. **Preview is a contract element, not a UI courtesy.** One new request,
   `ACT-REQ-004 Action.resolvePlan`, resolves a proposed request into a per-item
   plan without mutating anything. Execution carries the approved plan identity
   and digest, re-resolves, and refuses per item where resolution changed. Only
   the resolver that will execute can guarantee the preview matches, which is
   why this could not be assembled by the caller. It is the same consent binding
   already proven in `PP-M1-01`.
2. **Ambiguity fails rather than resolves.** A descriptor matching more than one
   live window fails the item; Action never chooses. LEDGER-0013 records serious
   unexpected window disturbance as a trust invalidation that voids the proof
   regardless of the success metric, so guessing is not a usability trade-off.
3. **Per-item commit points make honest partial success structural.** An
   operation carries an ordered item set; each item has its own commit point and
   terminal disposition, and the operation outcome is derived, never asserted.
   `completed` requires every approved item to have committed, and any
   unestablished item forces `indeterminate` regardless of how many succeeded.
4. **Matching is bounded so it cannot become observation.** Resolution may
   examine candidate windows, but only match, no-match, ambiguous, and the
   matched window's minimized summary may leave the operation. No candidate
   list, no unmatched attributes, no examined count. Plans expire, because an
   unexpiring plan is a retained picture of the environment.

Self-review found and corrected four defects in the first draft:

- The original resolution rules forbade enumeration while requiring descriptor
  matching that cannot avoid examining candidates — a contradiction. Replaced
  with a bounded matching operation defined by what may leave it.
- Plans had no expiry, which would have given Action a durable environment
  model through the back door.
- Nothing prevented passing a saved-context identifier into Action, which is the
  shortest path to a working restore and would couple Action to Workspace
  Management permanently. Now explicitly prohibited.
- The event sequence implied one operation-wide proof validation, which would
  have made revocation between items unenforceable. Corrected to point-of-use
  validation per item, consistent with `IC-009`.

Implementation: Documentation only.
- `architecture/15_Action_Desktop_Mutation_Contract.md` — new; 19 sections, 27
  acceptance cases, and a contract acceptance record.
- `architecture/10_Capability_Contracts.md` — added `ACT-REQ-004`, a declared
  action types subsection pointing to `15`, and bounded target resolution as an
  internal responsibility.
- `architecture/11_Contract_Schema_and_Acceptance_Specification.md` — recorded
  that the action-class commit-point and compensation risk is closed for the two
  declared types and open for reserved ones.

Validation: `git diff --check` clean. No runtime code changed, so the
`PP-M1-01` baseline of 1466 Rust and 33 frontend tests is unaffected.

Knowledge Gained:
- Auditing an existing contract before extending it removed most of the work.
  Roughly four fifths of what this session was asked to define already existed
  generically; writing it again would have created a second source of truth that
  drifts.
- Preview integrity is an execution-side property. Any design where the caller
  assembles the preview can only approximate what will happen.
- Ownership erodes through convenience. The three most tempting shortcuts —
  passing a context id into Action, caching resolutions, retaining prior
  placement for undo — each individually reasonable, would together have given
  Action observation and memory it may not own.
- Undo cannot be deferred silently. Deferring `PP-M1-03` while leaving the
  pre-effect state unowned would have led the first implementation to write an
  undo buffer into Action, so the open question is named rather than left
  implicit.

Unlocks: `PP-M1-02` has an implementation authority. It is not yet startable;
two prerequisites remain, recorded in Current State and in §19 of the contract:
the `IC-030` amendment in `09`, and the `PP-M1-01` identity schema gap.

Supersedes: Nothing. Closes the contract gap recorded in LEDGER-0019; that
entry's other findings stand.

Status: Accepted

---

### LEDGER-0021

Entry ID: LEDGER-0021

Capability: Workspace Management saved-context identity with Action target
matching — final architecture dependencies for Product Proof task `PP-M1-02`.

Research: Architecture and implementation-state audit only. Read the complete
Cursor Protocol authority set, accepted ADRs, Capability Architecture,
Interaction Matrix, Capability Contracts, Contract Schema, Product Proof
Strategy, accepted Action Desktop Mutation Contract, and the existing
observation/saved-context identity models. No technology research or runtime
implementation was performed.

Decision: `PP-M1-02` is architecture-ready. Close the three remaining
dependencies without adding a capability or broadening Product Proof:

1. Permit Companion to call `ACT-REQ-004 Action.resolvePlan` through the
   existing Companion → Action path. `action.plan.resolve` grants bounded target
   matching only and no environment effect. The returned plan is an immutable
   expiring Companion task value; Action retains no hidden plan aggregate.
2. Adopt `16_Saved_Context_Restore_Identity_Specification.md` as the canonical
   identity contract for a saved window. Newly saved restorable windows carry a
   versioned, consented descriptor containing desktop-session identity,
   captured handle/process id, title fingerprint, and capture time. Legacy and
   incomplete identities are never backfilled and are reported unsupported per
   item.
3. Assign the matching confidence threshold solely to Action. For
   `window.place` and `window.focus`, Product Proof admits only an exact
   same-session handle/process/title-fingerprint match. Permission Authority
   still alone owns effect authorization; Workspace Management stores but never
   evaluates identity; Companion copies but never scores it.

The threshold is Action-owned because it controls whether the executor can
safely commit against its exact target. Putting it in Permission Authority would
mix permission truth with execution safety; putting it in Workspace Management
would embed action semantics in organization; putting it in Companion would
create a second safety authority able to lower Action's floor.

`stable_window_id` and observation-time confidence were removed from the
restore descriptor during adversarial review. Windows does not expose that
Context Sensing correlation id on a live window, and neither field participates
in exact-session matching. Retaining them would add unused capture and create a
second confidence vocabulary. `PP-M1-02` supports a still-existing window in
the same continuing interactive Windows desktop session. Process/window
recreation, title change, logoff, reboot, another device, and cross-session
restore are honest non-successes. Application launch and reuse remain
undeclared.

Implementation:

- `architecture/09_Capability_Interaction_Matrix.md`: narrowed the existing
  Companion → Action cell to name plan resolution and its non-effecting proof.
- `architecture/10_Capability_Contracts.md`: reconciled Action request,
  response, permission, per-item effect proofs, and `IC-029`/`IC-030` text with
  `ACT-REQ-004`.
- `architecture/15_Action_Desktop_Mutation_Contract.md`: revised to v1.1,
  assigned match confidence to Action, bound the two declared types to the
  exact-session floor, made the plan stateless and effect-complete, required
  per-item proofs, added missing identity errors, and closed the two architecture
  risks.
- `architecture/16_Saved_Context_Restore_Identity_Specification.md`: added the
  canonical identity, consent, matching, lifetime, portability, failure,
  explainability, ownership, and acceptance specification.
- Updated Current State. No Blueprint, ADR, Capability Architecture, Contract
  Schema, research, or runtime file changed.

Implementation prerequisites:

- add nullable canonical identity storage for legacy saved contexts
- introduce and present a new Save capture-scope version before identity capture
- copy explicit Context Sensing identity evidence into new saved contexts
- issue `action.plan.resolve` authorization and one exact effect proof per
  attempted item
- implement Action's exact-session matcher and fixed safety floor
- keep handles/candidate details inside Action
- report legacy, stale, changed, ambiguous, and non-portable items honestly
- add automated acceptance evidence for the `SCRI-AC-*` and applicable
  `ADM-AC-*` cases

Validation:

- `git diff --check` clean.
- Second-pass review found no architecture needed beyond the current
  `PP-M1-02` implementation.
- No runtime tests were run because repository mutation was documentation only.

Knowledge Gained:

- A local correlation id named "stable" is not necessarily a portable operating
  system identity. Because it cannot be compared at Action's live boundary, the
  minimum design excludes it instead of overstating its value.
- Consent must version identity metadata as well as visible window metadata;
  adding handles after the user approved an older scope would be hidden capture.
- Exact equality is enough to prove a narrow same-session restore. Fuzzy
  matching, intelligence, and automatic repair are not required to begin
  Product Proof and would weaken its trust signal.
- Execution safety and permission truth are independent gates: either may refuse
  an effect, and neither may override the other.
- An approval cannot bind only an item list; it must bind each proposed effect.
  Mixed placement/focus batches require one proof per item because there is no
  omnibus action grant.

Unlocks: `PP-M1-02` implementation of deterministic same-session preview,
approval, placement/focus, and honest per-item results. Does not unlock
`PP-M1-03`, application launch/reuse, resource opening, cross-session restore,
or broader Action types.

Supersedes: The unresolved `IC-030`, saved identity, and matching-threshold
prerequisites in Current State and `15` §19. It does not supersede
LEDGER-0019's runtime implementation findings or LEDGER-0020's accepted Action
contract.

Status: Accepted; `PP-M1-02` architecture-ready

---

### LEDGER-0022

Entry ID: LEDGER-0022
Timestamp: 2026-08-02
Capability: Action + Workspace Management + Companion Orchestration + Experience
  — Product Proof task `PP-M1-02` deterministic Resume
Related ADRs: DEC-008
Related Research: None
Decision: Implement the first deterministic Resume workflow against the approved
Action Desktop Mutation Contract v1.1 and Saved Context Restore Identity
Specification v1.0, without redesigning architecture or inventing contracts.

Implementation:

- Capture scope advanced to `saved-context-scope-v2` with restore-identity
  checklist text; migration `042` adds nullable identity columns and unavailable
  reason; legacy rows are never backfilled.
- Save copies desktop-session, hwnd, process id, and title fingerprint evidence
  from the explicit capture; incomplete windows keep an explicit unavailable
  reason.
- Windows integration supplies `desktop_session_id` and a `WindowMutator` for
  bounded hwnd lookup, place, and focus (DEC-008).
- Action `resolvePlan` / `execute` implement plan digest, expiry, exact-session
  matching, re-resolution, per-item effect proofs, and honest dispositions
  including partial success, refused-changed, and unsupported z-order.
- Resume UI/IPC: list → preview → approve digest → restore → per-item outcomes.
- Automated `SCRI-AC-*` and applicable `ADM-AC-*` acceptance tests in
  `packages/kernel/src/commands/resume_acceptance_tests.rs`.

Validation:

- `cargo test --workspace` passed
- `pnpm test` passed
- `pnpm typecheck` passed
- `pnpm build` passed
- `git diff --check` clean for implementation content

Knowledge Gained:

- Exact same-session restore is implementable without AI, ambient observation,
  or silent retry when identity capture is consent-versioned and Action keeps
  matching inside a bounded hwnd lookup.
- Holding the approved plan as an expiring Companion/Experience value, with
  Action retaining no plan aggregate, is enough for preview-equals-execution
  integrity when digest and re-resolution are enforced at execute time.
- Per-item proofs are load-bearing: plan-resolve authority alone cannot place or
  focus a window.

Unlocks: Measured Product Proof use of deterministic Resume on the same
continuing Windows desktop session. Does not unlock `PP-M1-03`, application
launch/reuse, resource opening, cross-session restore, or undeclared Action
types.

Supersedes: The architecture-ready-but-unimplemented status of `PP-M1-02` in
Current State after LEDGER-0021. Does not supersede LEDGER-0019's
`LaunchApplication` drift finding.

Status: Accepted; `PP-M1-02` implemented

---

### LEDGER-0023

Entry ID: LEDGER-0023
Timestamp: 2026-08-02
Capability: Product strategy — Product Proof pilot readiness after `PP-M1-02`
Related ADRs: None new
Related Research: Strategic product review only; no technology evaluation
Decision: `PP-M1-02` is engineering-complete and must not be treated as
Product Proof pilot-ready. Verdict: **NOT READY** for the LEDGER-0013 pilot
until the smallest pilot package gaps are closed. Sequence **PP-P01 Pilot
Package** before `PP-M1-03` Undo and before declared application launch/reuse.

Rationale:

- LEDGER-0013 defines success as faster return to a recurring context **and
  intended next action**, with inspect/delete, consented measurement, and a
  primary UI that is not an engine/diagnostic console.
- Current Resume restores placement/focus for still-open same-session windows
  only. That is a trustworthy geometry loop, not yet the interruption product.
- Against PowerToys Workspaces plus notes, today's wedge risks proving
  ceremony without continuity and falsifying the hypothesis for the wrong
  reason.
- Trust increases at scope review, plan preview, digest approval, and honest
  per-item outcomes. Trust is still at risk from capability surprise (reboot /
  closed apps), architecture leakage in chrome, and missing disposal controls.

Smallest blockers (PP-P01):

1. User-authored handoff / intended next action on Save and Resume
2. Inspect/delete retained contexts in product UI
3. Consented measurement + interview kit (baseline, leave→resume time,
   correction, week-four habit) with zero ambient observation
4. Pilot-safe primary chrome (Save/Resume; hide engine tabs by default)
5. Explicit user-language restore limits (same session; still-open windows;
   no silent relaunch)

Do not build yet: Undo as the next gate, application.launch declaration,
cross-session heuristics, AI on the critical path, cloud, voice, plugins, or
fuzzy matching.

Validation:

- Reviewed Blueprint, Current State, Engineering Ledger (esp. LEDGER-0013,
  0021, 0022), Action/restore-identity contracts, Save/Resume UI, and App
  chrome.
- No runtime or architecture redesign. Documentation-only strategy resequence.
- Self-review: a narrow soft-interrupt dogfood may validate consent UX but
  must not be labeled Product Proof success under LEDGER-0013.

Knowledge Gained:

- Engineering acceptance of deterministic Resume is not product evidence.
- Layout without next-action memory remains the failure mode LEDGER-0013
  already warned about.
- Honest fail-closed matching must be framed as a product limit or pilots
  will read it as product failure.

Unlocks: A bounded PP-P01 engineering slice aimed at making the defined
pilot runnable. Does not unlock sustained capability research, `PP-M1-03` as
the immediate next gate, or broader Action types.

Supersedes: Current State's implication that validation with professionals
could begin immediately after `PP-M1-02`, and the sequencing of `PP-M1-03`
ahead of pilot-package completeness. Does not supersede LEDGER-0013's
hypothesis, metrics, or trust invalidation rules.

Status: Accepted; Product Proof pilot **NOT READY**; next milestone **PP-P01**
(implementation slices authorised by LEDGER-0025)

---

### LEDGER-0024

Entry ID: LEDGER-0024
Timestamp: 2026-08-02
Capability: Engineering governance — Engineering Session Protocol
Related ADRs: ADR-0007, ADR-0008
Related Research: None
Decision: Add `architecture/17_Engineering_Session_Protocol.md` as the
authoritative engineering-session governance document, and insert it into the
Cursor Protocol reading order immediately before Blueprint/Current State so
every session begins from repository authority rather than conversation.

Implementation:

- Created `17_Engineering_Session_Protocol.md` v1.0 covering purpose, authority
  hierarchy, startup procedure, reading order, model responsibilities, prompt
  standard, stop conditions, validation, git discipline, review cadence, and
  engineering invariants.
- Updated `04_Cursor_Protocol.md` to require reading the Engineering Session
  Protocol as step 1 of the implementation process and to state that
  conversation history is never authoritative.
- Updated Current State completed list. No capability, contract, Product Proof,
  or runtime file changed.

Validation:

- Confirmed no pre-existing Engineering Session Protocol or equivalent
  session-governance document existed in the architecture pack.
- Confirmed ADR-0008 already required replaceable sessions and non-authoritative
  AI conversations; this protocol operationalizes that decision without
  amending ADR text.
- Confirmed Architecture Guardian remains proposed and is not elevated to a
  mandatory gate by this change.
- Documentation-only; no runtime mutation.

Knowledge Gained:

- Process authority (how sessions start) can be documented without inventing
  product or capability authority.
- Explicit stop-on-contradiction rules reduce the risk that models “complete”
  missing contracts from chat context.

Unlocks: Deterministic session startup for architecture, implementation, and
review work. Does not unlock Product Proof pilot readiness, PP-P01, or any
runtime feature.

Supersedes: None. Complements `04_Cursor_Protocol.md` and ADR-0008.

Status: Accepted

---

### LEDGER-0025

Entry ID: LEDGER-0025
Timestamp: 2026-08-02
Capability: Product strategy / engineering governance — PP-P01 sequencing
Related ADRs: ADR-0008
Related Research: None
Decision: Keep **PP-P01 Pilot Package** as the milestone identity and pilot
gate. Formally decompose it into repository-authorised implementation slices
`PP-P01A`–`PP-P01E` so engineering sessions can proceed from Current State
without inventing milestone IDs from conversation. Decomposition changes
implementation sequencing only; it does not alter architecture, capability
ownership, contracts, or LEDGER-0013 success metrics.

Authorised slices and order:

1. **`PP-P01A`** — User-authored handoff / intended next action on Save and
   Resume. Active next implementation slice. Extends the existing saved-context
   artifact (Workspace Management + Experience presentation). Does not activate
   broader Memory research; LEDGER-0013's "Memory limited to the user-authored
   handoff" remains a Product Proof scope limit, not a mandate to stand up a
   new Memory subsystem for this slice.
2. **`PP-P01B`** — Explicit restore-limits copy in user language (same
   continuing desktop session; still-open windows only; no silent relaunch).
3. **`PP-P01C`** — Inspect and delete retained saved contexts in the product
   UI.
4. **`PP-P01D`** — Pilot-safe primary chrome (Save / Resume + minimal help;
   Canvas, Work, Assistant, and Diagnostic out of default pilot surface).
5. **`PP-P01E`** — Consented measurement and interview kit (baseline,
   leave→resume time, correction, week-four habit) with zero ambient
   observation.

Pilot gate unchanged: LEDGER-0013 recruitment requires complete `PP-P01`
(`PP-P01A` through `PP-P01E`). Completing any single slice is not Product
Proof success and does not unlock `PP-M1-03` or declared application
launch/reuse.

Rationale:

- LEDGER-0023's five blockers are separable surfaces with different risk and
  owners; a single undivided milestone forced prompts to invent slice names
  (`PP-P01A`) that were not repository-authoritative.
- Product Proof Milestone 1 already used numbered slices (`PP-M1-01`,
  `PP-M1-02`); the same pattern keeps session prompts deterministic under the
  Engineering Session Protocol.
- Ordering places handoff first (LEDGER-0013 core value), restore-limits copy
  second (trust framing before more chrome work), then disposal, pilot shell,
  and measurement.

Validation:

- Confirmed this is sequencing governance, not architecture mutation.
- Confirmed no capability ownership, contract, or runtime change.
- Confirmed Current State names `PP-P01A` as the active implementation slice
  while `PP-P01` remains the recommended milestone and pilot gate.

Knowledge Gained:

- Milestone identity and implementation-slice identity must both live in the
  repository, or Engineering Session Protocol stop conditions will correctly
  refuse conversational slice names.

Unlocks: Repository-authorised implementation of `PP-P01A` next. Does not
unlock the Product Proof pilot until `PP-P01A`–`PP-P01E` are complete.

Supersedes: The undivided "implement all of PP-P01 at once" reading of
LEDGER-0023's next-task wording. Does not supersede LEDGER-0023's NOT READY
verdict, blocker set, or pilot gate.

Status: Accepted; active slice **PP-P01A**; milestone **PP-P01** incomplete

### LEDGER-0026

Entry ID: LEDGER-0026
Timestamp: 2026-08-02
Capability: Workspace Management + Experience — PP-P01A user-authored handoff
Related ADRs: ADR-0008
Related Research: None
Decision: Implement **PP-P01A** by extending the existing saved-context artifact
with a required user-authored handoff note. Persist and restore the note
unchanged through Save review, Resume browse/preview/outcomes. Do not infer,
generate, or rewrite the note. Do not activate Memory as a new subsystem; do
not change Action ownership or restore identity contracts. Deterministic
window restore remains Action-owned and unchanged.

Implementation:
- Domain: `SaveContextRequest.handoff_note` / `SavedContext.handoff_note`;
  validation refuses empty/whitespace and notes over `HANDOFF_NOTE_MAX_CHARS`
  (2000); fail-closed errors `HandoffMissing` / `HandoffTooLong`.
- Persistence: migration `043_saved_context_handoff_note.sql`; repository
  read/write of `handoff_note`.
- Kernel save service: trims and stores the user text as authored; empty
  handoff refuses before desktop observation.
- Resume preview: surfaces `handoff_note` on plan preview for Experience.
- Experience UI: Save panel captures name + handoff, includes handoff in
  review before capture; Resume list/preview/outcomes display the note.
- Tauri IPC: `save_workspace_context` accepts `handoff_note`.
- Tests: domain validation; DB round-trip; kernel persistence and fail-closed
  empty-handoff; Vitest consent forwarding; resume acceptance coverage.

Validation:
- `cargo test --workspace` passed.
- `pnpm test`, `pnpm typecheck`, `pnpm build` passed (prior to docs close).
- No accepted architecture contract files modified.
- No capability ownership change; Action restore path unchanged.
- PP-P01B–PP-P01E not implemented.

Knowledge Gained:
- Handoff as a typed field on the saved-context artifact satisfies LEDGER-0013
  "Memory limited to the user-authored handoff" without standing up Memory
  capability runtime for Product Proof.
- Refusing empty handoff before observation keeps capture fail-closed and
  avoids writing partial artifacts.

Unlocks: Active implementation slice advances to **PP-P01B** (restore-limits
copy). Does not unlock the Product Proof pilot until `PP-P01A`–`PP-P01E` are
complete.

Supersedes: LEDGER-0025 status wording that named **PP-P01A** as the active
slice. Does not supersede LEDGER-0023 NOT READY, LEDGER-0025 slice order, or
the PP-P01 pilot gate.

Status: Complete; active slice **PP-P01B**; milestone **PP-P01** incomplete

### LEDGER-0027

Entry ID: LEDGER-0027
Timestamp: 2026-08-02
Capability: Experience — PP-P01B restore-limits copy
Related ADRs: ADR-0008
Related Research: None
Decision: Implement **PP-P01B** as Experience-only explanatory copy stating
Product Proof restore boundaries in user language: same continuing Windows
desktop session; still-open windows only; no silent relaunch; no file/link
opening; handoff shown as authored, never invented. Do not change Action
restore behaviour, capture scope identity, capability ownership, or contracts.

Implementation:
- Shared copy module `app/src/lib/restoreLimits.ts` as the single source of
  restore-limit wording.
- `RestoreLimitsNotice` component mounted on Save review, Save confirmation,
  Resume preview, and Resume outcomes; Resume browse shows the shared summary.
- Vitest audit `tests/restore-limits-copy.test.ts` for presence, consistency,
  and non-overclaim.
- Current State advanced; active slice becomes **PP-P01C**.

Validation:
- `pnpm test` (including new restore-limits tests), `pnpm typecheck`,
  `pnpm build` passed.
- Kernel resume/saved-context tests re-run after docs close (no Action change).
- No accepted architecture contract files modified.
- PP-P01C–PP-P01E not implemented.

Knowledge Gained:
- After PP-P01A remembers what the user wrote, explicit “what is not remembered
  / not restored” copy is the natural trust reinforcement before disposal UI
  and pilot chrome.

Unlocks: Active implementation slice advances to **PP-P01C** (inspect/delete).
Does not unlock the Product Proof pilot until `PP-P01A`–`PP-P01E` are complete.

Supersedes: LEDGER-0026 status wording that named **PP-P01B** as the active
slice. Does not supersede LEDGER-0023 NOT READY, LEDGER-0025 slice order, or
the PP-P01 pilot gate.

Status: Complete; active slice **PP-P01C**; milestone **PP-P01** incomplete

### LEDGER-0028

Entry ID: LEDGER-0028
Timestamp: 2026-08-02
Capability: Workspace Management + Experience — PP-P01C inspect/delete
Related ADRs: ADR-0008
Related Research: None
Decision: Implement **PP-P01C** by exposing inspect and explicit delete of
retained saved contexts in the existing Resume product surface. Reuse Workspace
Management persistence (`get` / `delete_by_id`) and Experience presentation.
Require confirmation before destructive delete. Fail closed on unknown ids.
Do not add background cleanup, retention policies, or restore-behaviour changes.

Implementation:
- Kernel mutation `DeleteSavedContext` (`workspace.write`); handler and Tauri
  IPC `delete_saved_context`.
- Resume UI: Inspect loads truthful metadata (handoff, windows, monitors,
  scope, timestamps); Delete → confirm → permanent removal; browse offers
  Inspect and Preview restore.
- Tests: SCRI-AC-14 via command path (list/get/resume fail after delete);
  unknown-id fail-closed; Vitest `saved-context-inspect-delete.test.ts`.
- Current State advanced; active slice becomes **PP-P01D**.

Validation:
- `cargo test -p workspace-kernel resume_acceptance` (33) passed.
- `pnpm test` (44), `pnpm typecheck`, `pnpm build` passed.
- No accepted architecture contract files modified.
- PP-P01D–PP-P01E not implemented.

Knowledge Gained:
- Repository already owned delete at the service/repository layer; the pilot
  gap was the governed command path and Experience confirmation surface, not
  a new storage system.

Unlocks: Active implementation slice advances to **PP-P01D** (pilot chrome).
Does not unlock the Product Proof pilot until `PP-P01A`–`PP-P01E` are complete.

Supersedes: LEDGER-0027 status wording that named **PP-P01C** as the active
slice. Does not supersede LEDGER-0023 NOT READY, LEDGER-0025 slice order, or
the PP-P01 pilot gate.

Status: Complete; active slice **PP-P01D**; milestone **PP-P01** incomplete

### LEDGER-0029

Entry ID: LEDGER-0029
Timestamp: 2026-08-02
Capability: Experience — PP-P01D pilot-safe chrome
Related ADRs: ADR-0008
Related Research: None
Decision: Implement **PP-P01D** by restricting default primary navigation to
Save, Resume, and minimal Help. Keep Canvas, Work, Assistant, and Diagnostic
out of the default pilot surface. Do not change restore behaviour, persistence,
contracts, or capability ownership. Prefer hiding engine affordances over new
interface concepts.

Implementation:
- `app/src/lib/pilotChrome.ts` defines primary vs hidden pilot chrome labels.
- `App.tsx` primary tablist is Save / Resume / Help only; engine panels are not
  mounted from the pilot shell.
- `PilotHelpPanel` explains the Save → Resume loop, restore limits, and user
  control without AI or engine-console claims.
- Save empty state creates a workspace in-place (`onCreateWorkspace`) instead of
  routing through Canvas.
- Vitest `tests/pilot-chrome.test.ts` audits primary labels, hidden engine tabs,
  and preserved Product Proof surfaces.
- Current State advanced; active slice becomes **PP-P01E**.

Validation:
- `pnpm test` (49), `pnpm typecheck`, `pnpm build` passed.
- `cargo test -p workspace-kernel resume_acceptance` (33) passed; restore
  unchanged.
- No accepted architecture contract files modified.
- PP-P01E not implemented.

Knowledge Gained:
- Pilot readiness required presentation discipline more than new product
  surfaces: removing engine tabs reduced cognitive noise while leaving Save /
  Resume trust messaging intact.

Unlocks: Active implementation slice advances to **PP-P01E** (consented
measurement). Does not unlock the Product Proof pilot until `PP-P01A`–`PP-P01E`
are complete.

Supersedes: LEDGER-0028 status wording that named **PP-P01D** as the active
slice. Does not supersede LEDGER-0023 NOT READY, LEDGER-0025 slice order, or
the PP-P01 pilot gate.

Status: Complete; active slice **PP-P01E**; milestone **PP-P01** incomplete

### LEDGER-0030

Entry ID: LEDGER-0030
Timestamp: 2026-08-02
Capability: Experience + local persistence — PP-P01E consented measurement kit
Related ADRs: ADR-0008
Related Research: None
Decision: Implement **PP-P01E** as a consented, local-only pilot measurement and
interview kit supporting LEDGER-0013 evidence collection: baseline return-to-work
minutes, participant-entered leave→resume times, correction notes, distinct-day
habit counting, and baseline/week-four interview responses. Require explicit
scope consent before any pilot record is written. Refuse ambient observation and
network upload. Keep pilot evaluation data distinct from saved-context product
data. Completing this slice completes the **PP-P01** implementation slices and
satisfies the LEDGER-0025 package gate for recruitment readiness review; it does
**not** declare Product Proof success or prove the hypothesis.

Implementation:
- Domain `pilot_measurement` scope, consent, baseline, leave→resume, interview
  types; fail-closed validation; median helper for leave→resume minutes.
- Migration `044_pilot_measurement.sql`; repository; kernel service and
  commands (`settings.read` / `settings.write`, System subject); Tauri IPC.
- Experience: Pilot primary tab with consent gate, measurement forms, interview
  prompts, summary, and withdraw/clear; Resume outcomes offer an optional link
  to record leave→resume (no background timing).
- Tests: domain, kernel service, Vitest `pilot-measurement.test.ts`; pilot
  chrome updated for the Pilot tab.
- Current State: PP-P01 slices complete; next milestone is Product Proof review.

Validation:
- `cargo test -p workspace-domain pilot_measurement`
- `cargo test -p workspace-kernel pilot_measurement`
- `cargo test -p workspace-kernel resume_acceptance` (Product Proof unchanged)
- `pnpm test` (54), `pnpm typecheck`, `pnpm build`
- No accepted architecture contract files modified.
- No ambient observation or network reporting introduced.

Knowledge Gained:
- Evaluation data must remain a separate store and consent surface from product
  Save/Resume artefacts, or pilot metrics will blur into Memory/Workspace
  Management ownership claims.

Unlocks: LEDGER-0025 package gate for LEDGER-0013 recruitment readiness review.
Does not unlock hypothesis success, `PP-M1-03`, or resumed capability research.

Supersedes: LEDGER-0029 status wording that named **PP-P01E** as the active
slice; LEDGER-0023's reading that the five package blockers remain open. Does
not supersede LEDGER-0013 success/failure metrics or trust invalidation rules.

Status: Complete; **PP-P01 implementation slices complete**; no active PP-P01
slice; next recommended milestone is Product Proof review

### LEDGER-0031

Entry ID: LEDGER-0031
Timestamp: 2026-08-02
Capability: Product strategy — Product Proof Review after PP-P01
Related ADRs: ADR-0008
Related Research: None (strategy review; no technology evaluation)
Decision: Accept `architecture/18_Product_Proof_Review_PP_P01.md` as the
authoritative post-PP-P01 Product Proof Review. Findings:

1. **PP-P01 is formally complete** as an implementation milestone
   (`PP-P01A`–`PP-P01E`, LEDGER-0030).
2. **Recruitment-readiness gate is satisfied** (LEDGER-0025 / LEDGER-0030).
3. **LEDGER-0013 hypothesis remains the governing objective** and remains
   **unproven**.
4. Implementation now **supports testing** the hypothesis (handoff, honest
   limits, inspect/delete, pilot chrome, consented local measurement) but does
   **not** establish that users will prefer Workspace.
5. **Highest-value next programme objective:** recruit and execute the
   LEDGER-0013 four-week pilot (~15 target users), then analyse evidence against
   LEDGER-0013 success, failure, and trust-invalidation rules.
6. **Next implementation slice: None.** Do not start `PP-M1-03`, declared
   application launch/reuse, ambient sensing, network telemetry, Intelligence on
   the critical path, or `ROADMAP-001` from completion momentum.

Residual non-blocking risks recorded in the review: live WebView2 CSP
observation on first pilot build; same-session restore limits as product truth;
open `LaunchApplication` drift kept out of pilot claims; local-only measurement
implies operational cohort rollup.

Rationale: LEDGER-0013 defines the measurable objective; LEDGER-0030 closed the
engineering package gate; Current State required this review before the next
milestone. Further building before evidence would confuse package completion
with hypothesis proof.

Validation:
- Re-read Cursor Protocol, ESP, Blueprint, Current State, Ledger through
  LEDGER-0030, and LEDGER-0013 Product Proof Strategy.
- Confirmed no contract, ownership, or architecture mutation required.
- Confirmed LEDGER-0023 NOT READY is superseded for the five package blockers
  by LEDGER-0030, without superseding LEDGER-0013 metrics.
- No runtime code changed in this entry.

Knowledge Gained:
- Recruitment readiness and hypothesis proof are distinct gates; PP-P01 closes
  the first and enables the second to be attempted.

Unlocks: Operational LEDGER-0013 pilot recruitment and execution as the active
programme objective. Does not unlock Product Proof success, `PP-M1-03`, or
capability research.

Supersedes: Current State / LEDGER-0030 recommendation that the next milestone
is still “Product Proof review”. Does not supersede LEDGER-0013 hypothesis,
metrics, or trust invalidation.

Status: Accepted; next programme objective **LEDGER-0013 pilot recruitment and
execution**; next implementation slice **None**

### LEDGER-0032

Entry ID: LEDGER-0032
Timestamp: 2026-08-02
Capability: Runtime Host / persistence — Participant #1 installable build
Related ADRs: ADR-0008
Related Research: None
Decision: Fix a blocking install defect discovered while preparing the
Participant #1 (developer) pilot-ready local build. `DatabaseService::initialize`
previously applied migrations by reading `CARGO_MANIFEST_DIR/migrations` at
runtime. Release binaries therefore depended on the developer checkout path and
could start with an empty schema on a clean machine (`load_from_dir` formerly
returned success with zero migrations when the directory was missing).

Implementation:
- `packages/database/build.rs` embeds all `.sql` migrations via `include_str!`.
- `MigrationRunner::bundled()` is the production path; `load_from_dir` fails
  closed if the directory is missing or empty.
- Kernel in-memory init uses bundled migrations.
- App setup creates the app-data directory explicitly and logs the DB path.
- `19_Pilot_Participant_1_Local_Build.md` operational notes.
- No new capabilities, contracts, or Product Proof behaviour changes.

Validation:
- `cargo test -p workspace-database` (50)
- `cargo test -p workspace-kernel` initialize / resume_acceptance /
  saved_context / pilot_measurement
- `pnpm tauri:build` produces MSI + NSIS; release exe embeds
  `CREATE TABLE IF NOT EXISTS pilot_consent` / `saved_contexts`
- `pnpm test` / typecheck remain green from prior suite

Knowledge Gained:
- Installable Product Proof builds must carry schema in-binary; checkout-relative
  migration loading is a silent pilot blocker.

Unlocks: Participant #1 can install/run a local build without the git tree
present for schema apply. Does not unlock hypothesis proof or cohort evidence.

Supersedes: None for product strategy. Corrects the runtime migration loading
assumption left implicit after MSI/NSIS packaging (LEDGER-0017 era).

Status: Complete; Participant #1 local build ready for daily dogfood

### LEDGER-0033

Entry ID: LEDGER-0033
Timestamp: 2026-08-02
Capability: Runtime Host / settings — active workspace persistence
Related ADRs: ADR-0008
Related Research: None
Decision: Fix the Participant #1 blocking defect where creating/activating a
workspace showed `One or more settings values were invalid` and never persisted
`active_workspace_id`. `UpdateSettings::validate` required `theme` or
`first_run`, rejecting the Experience `active_workspace_id`-only update used by
pilot chrome.

Implementation:
- Allow updates that set `active_workspace_id` or `personalization_enabled`
  without theme/first_run.
- Add regression test `accepts_active_workspace_id_only_update`.
- No contract or Product Proof behaviour change.

Validation: `cargo test -p workspace-kernel active_workspace_id_only`

Unlocks: Active workspace survives relaunch for Participant #1 dogfood.

Status: Complete (commit `589a8e2`)

### LEDGER-0034

Entry ID: LEDGER-0034
Timestamp: 2026-08-02
Capability: Experience — fidelity convergence for Product Proof chrome
Related ADRs: ADR-0008
Related Research: None
Decision: Accept Experience Fidelity Review (`20_Experience_Fidelity_Review.md`)
and converge presentation toward the original Workspace vision without adding
capabilities or changing trust/Product Proof behaviour.

Implementation (Experience presentation only):
- Product-oriented navigation: Home / Save / Continue / Check-in / Guide.
- Home hub with workspace identity, recent handoffs, empty-state invitation.
- Save/Resume/Pilot/Help redesigned as card-based spatial UI; technical
  metadata moved behind inspect disclosures.
- Empty-state copy no longer uses “Nowhere to keep this yet.”
- Visual system: calm dark charcoal, soft blue accent, card hierarchy.
- Pilot measurement and restore-limits content preserved truthfully.
- Chrome/presentation tests updated for product labels.

Validation: `pnpm test`, `pnpm typecheck` green; trust/consent/restore-limits
audits unchanged in substance.

Does not unlock hypothesis proof. Does not change Action restore semantics.

Status: Complete for this convergence pass

### LEDGER-0035

Entry ID: LEDGER-0035
Timestamp: 2026-08-02
Capability: Experience — Experience Roadmap after Phase 1 convergence
Related ADRs: ADR-0008
Related Research: None
Decision: Accept `architecture/21_Experience_Roadmap.md` as the sequencing
authority for Experience presentation after Participant #1 review of
LEDGER-0034. Standing: ~6.5–7/10 — product visible, personality not yet found.
Diagnosis: concept boards are dashboard-first; implementation remains
page-first.

Phases:
1. Companion identity — largely achieved (LEDGER-0034)
2. Dashboard-first Home with meaningful recent activity — next Experience
   milestone when authorised
3. Rich visual continuation and saved moments
4. Refined polish, motion, and transitions

Constraints unchanged: no new capabilities, no ambient observation, no contract
or Product Proof behaviour changes. Prefer existing saved-context/handoff data
and honest empty structure. Does not supersede LEDGER-0013 pilot execution or
`ROADMAP-001`.

Rationale: Further gains come from intentional Experience phases, not
capability growth or ad-hoc UI tweaks.

Validation: Documentation-only acceptance; no runtime change in this entry.

Unlocks: Authorised Experience Phase 2 as the next presentation milestone when
an implementation session is opened for it. Does not unlock hypothesis proof.

Status: Accepted

### LEDGER-0036

Entry ID: LEDGER-0036
Timestamp: 2026-08-02
Capability: Experience — component library research (EXP-001)
Related ADRs: ADR-0008
Related Research: EXP-001
Decision: Accept the Experience Research Package as the repository authority
for Experience presentation components, licensing evaluation, interaction
patterns, design tokens, OSS candidates, and Phase 2 planning:

- `architecture/research/EXPERIENCE_COMPONENT_RESEARCH.md`
- `architecture/research/experience/` (catalogue, licensing matrix, inventory/
  gap analysis, patterns, tokens, OSS recommendations, build plan, Phase 2
  plan)
- Catalogue entry `EXP-001` in `03_Research_Catalogue.md`

No runtime, capability, contract, or architecture ownership changes. No npm
dependency approved. Experience Phase 2 implementation may begin from
`experience/08_Phase_2_Implementation_Plan.md` when separately authorised;
stack adoption remains decision-pending.

Rationale: Experience evolution needs a curated component layer so Phase 2+ is
not ad-hoc screen design, and so capability invention is not used to “feel
better.”

Validation: Documentation-only; no `app/` / `packages/` mutation in this entry.

Unlocks: Repository-ready Phase 2 planning inputs. Does not unlock hypothesis
proof or dependency installation.

Status: Complete

### LEDGER-0037

Entry ID: LEDGER-0037
Timestamp: 2026-08-02
Capability: Experience — Phase 2 dashboard-first Home convergence
Related ADRs: ADR-0008
Related Research: EXP-001
Decision: Implement Experience Roadmap Phase 2 presentation: dashboard-first
Home as product centre, shared moment cards, honest empty structure, unified
design tokens, and continuation-first Continue/Save/Guide/Check-in tone —
without capability, contract, or Product Proof behaviour changes.

Implementation:
- Design tokens (colour, space, radius, elevation, motion) in `App.css`
- `MomentCard` (hero/standard/compact/placeholder) + `EmptyStructure`
- Home dashboard grid (12-col rhythm, hero + rail + recents, trust strip)
- Continue library uses shared cards; Inspect remains secondary
- Save as note-first composition; Guide onboarding cards; Check-in feedback tone
- Adopt `lucide-react` 0.511.0 (ISC) for icon language; relative time via
  `Intl.RelativeTimeFormat` (no date-fns required)
- Fidelity audit note: `research/experience/PHASE_2_FIDELITY_AUDIT.md`

Validation: `pnpm typecheck`, `pnpm test` (54), CSP/boundary verifiers green.

Does not unlock ambient activity, AI, or hypothesis proof.

Status: Complete
