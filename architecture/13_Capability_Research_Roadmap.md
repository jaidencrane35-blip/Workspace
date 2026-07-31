# Workspace Capability Research Roadmap v1.0

Status: Complete; pending architecture review
Authority: Research-sequencing plan subordinate to the active architecture and
`12_Capability_Technology_Research_Framework.md`
Version: 1.0

This document orders the remaining Workspace capability research. It does not
perform technology research, name or recommend technologies, select or reject a
candidate, authorize implementation, activate Extension Host, or change
capability ownership, contracts, interactions, or acceptance semantics.

Authority order remains:

1. `00_Workspace_Blueprint.md`
2. Accepted ADRs in `architecture/decisions/`
3. `08_Workspace_Capability_Architecture.md`
4. `09_Capability_Interaction_Matrix.md`
5. `10_Capability_Contracts.md`
6. `11_Contract_Schema_and_Acceptance_Specification.md`
7. `12_User_Journey_Architecture_Validation.md`
8. `12_Capability_Technology_Research_Framework.md`
9. This roadmap

Higher authority wins on conflict.

---

## Purpose

The roadmap minimizes architectural rework by researching shared foundations
before dependent capability choices and by delaying cross-capability
orchestration, presentation, and optional extension decisions until their
inputs are known.

It orders research by:

1. architectural dependency and trust-boundary fan-out
2. technical, security, privacy, and lifecycle risk
3. unresolved requirements and evidence uncertainty
4. impact on implementation feasibility and acceptance
5. influence on later candidate comparison and technology selection

The order is a research plan, not an implementation order or technology
decision.

---

## Completed Research Foundation

Two capability research units are complete and remain prerequisites throughout
the roadmap:

- `PA-001` — Permission Authority patterns and candidate categories
- `MEM-001` — local-first AI Memory patterns and candidate categories

They establish constraints rather than selections.

`PA-001` establishes that authorization evaluation, challenge handling, proof
use, revocation ordering, safety control, audit, and explanation are distinct
mechanisms. Every protected capability research unit must preserve exact
point-of-use authorization and the effect/control-authority split.

`MEM-001` establishes that working context, canonical durable knowledge, and
derived retrieval projections have different ownership and lifecycle
semantics. Every remaining capability research unit must prevent logs, task
state, prompts, observations, execution records, UI buffers, and extension
partitions from becoming alternate Memory stores.

Neither completed unit authorizes adoption. Their unresolved architecture
questions remain explicit inputs and mandatory selection gates.

---

## Remaining Capabilities

The remaining capabilities requiring research are:

1. Runtime Host
2. Workspace Management
3. Context Sensing
4. Action
5. Intelligence
6. Companion Orchestration
7. Experience
8. Extension Host, conditionally and only after separate activation
   justification

No additional core capability is required. Voice remains an Experience
modality. Optional remote inference remains an Intelligence provider path.
Privacy, recovery, shutdown, and attention remain responsibilities and
invariants within existing capabilities rather than new subsystems.

---

## Architecture Gates Applied to the Roadmap

Bounded research may begin before every user-journey correction is complete,
but research must state unresolved assumptions and must not select against
incomplete requirements.

Final comparison, recommendation, selection, or implementation for an affected
area remains blocked until its relevant architecture changes are accepted:

- essential versus degradable startup dependencies
- Experience access to status, permission administration, and shutdown during
  degraded startup
- Experience-to-Companion cancellation, status, and explanation purposes
- sensing subscription, unsubscribe, session reference, and control lifecycle
- Companion child-operation cancellation and terminal reconciliation
- protected custody and recovery of in-flight operation control state
- local Intelligence provider administration and resource-cap contracts
- independently revocable compound authorization for optional remote use
- attention, interruption, bounded prompting, and neutral-consent invariants
- archived-workspace active-scope fallback and Memory visibility
- per-operation commit, cancellation, partial-effect, compensation, and
  recovery declarations

These are architecture prerequisites, not technology research tasks.

---

## Dependency Model

The roadmap uses six waves.

### Wave 1 — Root runtime foundation

1. Runtime Host

Runtime Host is the root lifecycle dependency. Its research constrains process
or in-process boundaries, local configuration, capability supervision, health,
packaging, updates, recovery, and ordered shutdown for every later integration.

### Wave 2 — Shared scope foundation

2. Workspace Management

Workspace Management provides the authoritative scope consumed by Memory,
Context Sensing, Action, Companion Orchestration, and Experience. Its archive,
scope-version, conflict, and recovery semantics must be understood before
dependent capability comparisons can be finalized.

### Wave 3 — Parallel domain-boundary research

3. Context Sensing
4. Action
5. Intelligence

These units may proceed in parallel after their shared Runtime Host,
Permission Authority, Memory-boundary, and applicable Workspace scope
assumptions are explicit. None may be allowed to redefine another:

- Context Sensing remains read-only and ephemeral.
- Action remains the sole environment mutation boundary.
- Intelligence remains a non-executing reasoning/provider boundary.

Their findings converge as prerequisites for Companion task sequencing,
cancellation, degradation, and explanation.

### Wave 4 — Cross-capability coordination

6. Companion Orchestration

Companion research follows the domain capabilities because it must coordinate
their actual operation, cancellation, recovery, permission, and degradation
semantics without absorbing their ownership.

### Wave 5 — Human-facing integration

7. Experience

Experience research follows Companion and the domain capabilities because it
must present their authoritative states, permission challenges, uncertainty,
partial effects, control options, and degraded behavior accurately and
accessibly.

### Wave 6 — Conditional optional boundary

8. Extension Host

Extension Host research remains conditional. A separate Complexity Budget
decision must first establish user value and justify activation. If that gate
passes, Extension Host research follows all core capability research so its
identity, isolation, cancellation, permission, attention, explanation, and
zero-extension behavior inherit stable core semantics.

---

## 1. Runtime Host

### Why it must be researched

Runtime Host owns startup, wiring, capability availability, domain-free health,
local mode, configuration, failure isolation, and ordered shutdown. Every
embedded component, native library, sidecar, provider runtime, sensor, action
adapter, and optional extension boundary introduces lifecycle obligations that
must fit this host model.

Researching it first reduces the risk that later candidate comparisons assume
incompatible process, packaging, update, recovery, or shutdown models.

### Prerequisite completed research

- `PA-001`, for fail-closed Authority availability, isolation, local state, and
  control-proof outage assumptions
- `MEM-001`, for local storage, native component, sidecar, backup, recovery, and
  shutdown implications
- completed capability, interaction, contract, acceptance, and user-journey
  architecture

No remaining capability research is a prerequisite.

### Expected outputs

- a canonical Runtime Host research unit and Catalogue entry
- comparison categories for lifecycle, capability registration, configuration,
  health aggregation, failure isolation, and packaging boundaries
- explicit embedded versus isolated-process trade-offs without selecting one
- Windows lifecycle, installer, update, signing, standard-user, sleep/resume,
  crash, and shutdown evidence requirements
- essential versus degradable dependency assumptions recorded for architecture
  review
- reproducible acceptance plan for IC-001 through IC-005 and offline scenarios
  1 and 8
- build-versus-integrate boundary for unique host policy versus commodity
  runtime and observability functions

### Key unanswered questions

- Which capabilities are startup-essential, independently available, or
  degradable?
- What isolation boundary is sufficient for each risk class?
- Which failures may be restarted safely, and which require explicit user
  recovery?
- How are capability readiness, stale health, crash loops, and shutdown timeout
  represented without domain data?
- How is local host configuration migrated, backed up, restored, and rolled
  back?
- How does Experience remain available for status, permissions, and shutdown
  when Companion or another dependency is unavailable?
- How are engineering artifacts provably excluded from runtime packaging?

### Architectural risks

- lifecycle authority acquiring domain visibility or product permission
- a sidecar or native component silently becoming an unmanaged subsystem
- startup ordering hard-coding optional capabilities as core dependencies
- restart behavior duplicating effects or violating owner recovery
- shutdown closing Experience before consequential outcomes are visible
- logs or crash diagnostics retaining protected content
- packaging or update assumptions invalidating Local First operation

### Estimated research complexity

**High.** The capability is conceptually narrow but has system-wide fan-out
across process, packaging, failure, recovery, and Windows lifecycle behavior.

---

## 2. Workspace Management

### Why it must be researched

Workspace Management owns durable workspaces, zones, membership, active scope,
and scope validation. Its immutable scope snapshots and change events constrain
Memory retrieval, sensing tags, Action targets, Companion planning, and
Experience navigation.

Research must precede final dependent comparisons so candidates cannot invent
scope, hierarchy, archive, or conflict semantics.

### Prerequisite completed research

- `PA-001`, for protected reads, mutations, point-of-use validation, and
  operation control
- `MEM-001`, for archived-workspace Memory visibility and strict separation
  between organization and retained knowledge
- Runtime Host research, for persistence, migration, health, and shutdown
  boundary assumptions

### Expected outputs

- a canonical Workspace Management research unit and Catalogue entry
- bounded data-model and persistence categories for hierarchy, active scope,
  membership, archive, conflict, import/export, and migration
- explicit owner-authoritative mutation and recovery model
- scope snapshot, versioning, invalidation, and stale-reference evaluation
  criteria
- archive lifecycle assumptions submitted for architecture acceptance
- reproducible acceptance plan for IC-013 through IC-018 and offline scenario 3
- build-versus-integrate boundary preserving Workspace-owned organization

### Key unanswered questions

- What hierarchy depth and membership model are required?
- Can more than one workspace or zone be active, and what does “unscoped” mean?
- What stable identity links OS resources to Workspace organization without
  making Workspace Management a file index?
- What happens when the active workspace is archived?
- How are in-flight operations and dependent scope snapshots invalidated?
- How is Memory scoped to an archived, restored, merged, or reorganized
  workspace exposed?
- Which mutations are atomic, and how are conflicts, partial outcomes,
  deduplication, and terminal history represented?
- Which logical export is sufficient for replacement without semantic loss?

### Architectural risks

- duplicating Memory structure or file-index ownership
- stale scope permitting cross-workspace reads, sensing, or effects
- archive behavior making retained Memory unexpectedly visible or inaccessible
- shared mutable scope leaking across capability boundaries
- storage schema convenience redefining zone or membership semantics
- migration restoring archived scope or losing conflict history

### Estimated research complexity

**Medium.** The data and interaction surface is bounded, but scope correctness
has broad privacy and authorization consequences.

---

## 3. Context Sensing

### Why it must be researched

Context Sensing owns read-only observation sessions, ephemeral raw data,
minimization, and purpose-limited context output. It is the highest-volume
privacy boundary and must remain structurally separate from Action and durable
Memory.

Research must determine which signal classes can provide value while preserving
continuous authorization, immediate stop, bounded resource use, and verifiable
buffer clearing.

### Prerequisite completed research

- `PA-001`, for basic/deep scopes, continuous validation, revocation, and
  safety-reducing session control
- `MEM-001`, for the observation-versus-retention boundary, candidate
  provenance, and prohibition on raw capture archives
- Runtime Host research, for sensor lifecycle, isolation, health, and shutdown
- Workspace Management research, for optional scope tags and invalidation
- accepted sensing subscription/session lifecycle architecture before final
  comparison or selection

### Expected outputs

- a canonical Context Sensing research unit and Catalogue entry
- a minimum-signal taxonomy separated into basic and deep observation classes
- comparison categories for read-only local signals, minimization, redaction,
  OCR/capture where later justified, and session control
- explicit proof that mutation APIs and durable retention remain outside the
  capability
- Windows permission, accessibility, privacy-setting, and unavailable-sensor
  evidence requirements
- reproducible stop, revoke, buffer-clear, resource, and offline validation plan
- acceptance mapping for IC-023 through IC-025 and privacy cases 25 through 27

### Key unanswered questions

- Which minimum local signals provide useful assistance?
- Which signals require deep sensing, and what data classes may cross the
  boundary after minimization?
- Where do capture, minimization, redaction, and provenance assignment occur?
- Is subscription a bounded stream or an owned stateful operation?
- How are subscribe, unsubscribe, session reference, pause, stop, resume, and
  terminal state represented?
- What latency bound applies to revocation, stop, and buffer clearing?
- How are inaccessible, protected, secure-desktop, multi-monitor, virtual
  desktop, and accessibility states handled?
- What evidence proves raw observations never enter logs, crash reports, task
  history, provider history, or Memory without a separate proposal?

### Architectural risks

- raw or unminimized observations crossing the boundary
- deep permission being treated as a waiver of minimization
- read and mutation APIs entering the same adapter or privilege boundary
- stuck sensors or stale subscriptions surviving revocation or shutdown
- high CPU, battery, memory, or storage use becoming ambient surveillance
- inaccessible or partial signals being presented as complete context
- sensing candidates being retained without separate authorization

### Estimated research complexity

**High.** Privacy, Windows API variation, accessibility, continuous
authorization, and immediate lifecycle control all require reproducible
evidence.

---

## 4. Action

### Why it must be researched

Action is the sole environment-mutation boundary. It converts exact,
permissioned requests into real effects and therefore carries the greatest
direct safety, integrity, privilege, and outcome-recovery risk.

Generic automation research is insufficient. Research must be organized by
declared action class because commit points, cancellation, partial effects,
idempotency, compensation, privilege, and target validation differ materially.

### Prerequisite completed research

- `PA-001`, for exact effect authorization, per-effect revocation ordering,
  replay protection, and operation control
- `MEM-001`, for metadata-only audit and prohibition on retained target content
- Runtime Host research, for executor isolation, lifecycle, crash containment,
  and shutdown
- Workspace Management research, for target scope validation
- the Context Sensing boundary definition, to prevent read/write API collapse;
  candidate research may proceed in parallel, but both must cross-check the
  final boundary
- accepted per-operation cancellation declarations before final comparison or
  selection for an action class

### Expected outputs

- one roadmap-backed Action research programme with separate canonical research
  units for materially different action classes
- action taxonomy and risk-class framing before candidate discovery
- per-class commit point, cancellation class, partial-effect boundary,
  idempotency, compensation availability, and recovery requirements
- target identity, substitution resistance, privilege, and scope-validation
  evaluation criteria
- Windows standard-user/elevation and isolation evidence requirements
- acceptance mapping for IC-029 through IC-031 and cases 12 through 20
- explicit internal execution policy versus commodity automation boundary

### Key unanswered questions

- Which action classes are required for the first implementation milestone?
- What exact target identity and effect description are stable enough for
  authorization?
- Where is the irreversible commit point for each class?
- Which operations are fully cancellable, race completion, or become
  irreversible?
- Which partial effects are possible, and how are they detected and explained?
- When is compensation available, and how is it authorized as a new action?
- Which privileges and process boundaries are acceptable?
- How are unknown outcomes reconciled without blind retry?
- What content-free audit is sufficient for accountability?

### Architectural risks

- broad or unstable action classes creating omnibus authority
- target substitution between approval and effect
- privileged automation enlarging the attack surface
- duplicate delivery or crash producing repeated effects
- unsafe cancellation being presented as successful rollback
- sensing and mutation APIs sharing hidden authority
- audit, diagnostics, or effect summaries retaining user content
- candidate-native retry or macro behavior bypassing per-effect authorization

### Estimated research complexity

**High.** Research is security-critical and must be repeated by materially
different action class rather than collapsed into one generic comparison.

---

## 5. Intelligence

### Why it must be researched

Intelligence owns local-first reasoning and generation provider invocation,
structured non-executing proposals, uncertainty, resource controls, and the
optional explicit remote boundary. It strongly influences usefulness and local
resource requirements but must not acquire orchestration, Memory, sensing, or
Action authority.

Research must establish viable local service classes and provider replacement
boundaries without allowing candidate framework defaults to redefine contracts.

### Prerequisite completed research

- `PA-001`, for local, remote, data-sharing, administration, and operation
  control authority
- `MEM-001`, for caller-supplied context, no prompt/response persistence, local
  model boundaries, and retrieval-derived provenance
- Runtime Host research, for provider discovery, process/model lifecycle,
  resource supervision, mode events, and shutdown
- accepted local-provider administration, inference cancellation, and optional
  compound remote authorization architecture before final comparison or
  selection in those areas

Workspace Management, Context Sensing, and Action are not direct Intelligence
dependencies. Their data reaches Intelligence only as minimized caller-supplied
context or returns as non-executing proposal evaluation through Companion.

### Expected outputs

- a canonical Intelligence research unit and Catalogue entry
- declared local viability classes by supported hardware and workload
- comparison categories for provider invocation, local execution, structured
  output, uncertainty, cancellation, resource control, and optional remote use
- reproducible quality, latency, resource, startup, failure, and offline
  evaluation plan
- provider administration, integrity, acquisition, update, and replacement
  evidence requirements
- no-persistence and no-silent-remote-fallback verification plan
- acceptance mapping for IC-005 and IC-026 through IC-028, cases 29 through 32,
  and applicable cancellation/recovery cases

### Key unanswered questions

- Which hardware and workload classes define viable local reasoning?
- What quality, reliability, latency, memory, storage, power, and thermal
  thresholds are required?
- How are providers configured, validated, updated, removed, and resource
  capped without combining administration with inference authority?
- What cancellation class and recovery behavior apply to inference?
- How is in-flight operation control recovered after Companion restart?
- What structured result and uncertainty evidence is sufficient before
  Companion can use a proposal?
- How are malformed output, prompt injection, model-file tampering, and resource
  exhaustion contained?
- If optional remote use is later enabled, how are remote-run and data-sharing
  scopes independently revoked and jointly validated?

### Architectural risks

- provider or framework becoming a shadow orchestrator
- proposal output being treated as Action authority
- prompt, response, or Memory content persisting in provider history or logs
- claimed local operation depending on network acquisition, activation, or
  recovery
- silent remote fallback or incomplete data-sharing authorization
- model/provider coupling making contracts or stored state non-replaceable
- unbounded local resource use degrading the desktop
- result confidence being presented as factual certainty

### Estimated research complexity

**High.** Quality is workload-dependent, local hardware varies, and provider,
privacy, resource, lifecycle, cancellation, and optional remote boundaries all
require measured evidence.

---

## 6. Companion Orchestration

### Why it must be researched

Companion Orchestration owns intent interpretation, bounded task state, public
contract sequencing, permission waiting, child-operation control, degraded
paths, and presentation-neutral progress. It is unique Workspace value and the
point where all prior capability findings converge.

Researching it before the domain capabilities would force orchestration
candidates to invent downstream semantics and would create the greatest risk of
shadow ownership and rework.

### Prerequisite completed research

- `PA-001` and `MEM-001`
- Runtime Host and Workspace Management research
- Context Sensing, Action, and Intelligence research
- accepted Experience-to-Companion control/inspection purpose correction
- accepted task cancellation fan-out and protected in-flight control-state
  recovery architecture

### Expected outputs

- a canonical Companion Orchestration research unit and Catalogue entry
- bounded task/state-machine and workflow categories compared without
  delegating Companion authority
- explicit task and child-operation lifecycle model
- permission challenge, wait, resume, deny, revoke, cancel, timeout, recovery,
  and degraded-path evaluation criteria
- policy for treating Intelligence proposals and extension contributions as
  untrusted input
- proof that task content remains in-flight and durable content follows Memory
- deterministic scenario suite spanning IC-006, IC-014, and IC-019 through
  IC-035 where active
- build-versus-integrate analysis that isolates unique orchestration policy from
  commodity workflow primitives

### Key unanswered questions

- What is the smallest orchestration mechanism that supports required task
  states without becoming a generic automation platform?
- How are plans bounded by depth, duration, effects, and resource use?
- How does one task enumerate every accepted child operation?
- How are cancellation, residual irreversible work, and owner terminal outcomes
  aggregated?
- What protected state is required to recover control after Companion crash?
- Which flows remain deterministic when Intelligence is unavailable?
- How are stale, duplicate, late, missing, incompatible, or conflicting results
  handled?
- How are purpose, requester, extension identity, scope, and permission context
  preserved through every call?
- What task detail is inspectable while remaining ephemeral after completion?

### Architectural risks

- absorbing Memory, Action, sensing, permission, provider, or UI ownership
- acquiring ambient superuser authority
- persisting content-bearing task history as alternate Memory
- candidate workflow defaults silently retrying unknown effects
- unbounded autonomous plans increasing risk and cognitive load
- proposal injection or stale results producing unintended downstream work
- incomplete cancellation leaving hidden active children
- dependence on Experience creating a hard cycle

### Estimated research complexity

**High.** It has the broadest contract fan-in/fan-out and must preserve many
owner-authoritative states without becoming their owner.

---

## 7. Experience

### Why it must be researched

Experience is the primary human-control and explainability surface. It must
capture intent and decisions, present authoritative states without rewriting
them, remain useful during degraded startup, and satisfy accessibility,
calmness, predictability, respect, and non-manipulation requirements.

Research must follow the capabilities whose states it presents so UI
convenience cannot simplify away permission, uncertainty, cancellation,
partial-effect, or recovery meaning.

### Prerequisite completed research

- `PA-001` and `MEM-001`
- Runtime Host and Workspace Management research
- Context Sensing, Action, and Intelligence research
- Companion Orchestration research
- accepted task inspection, shutdown, degraded startup, attention, and neutral
  consent architecture

Optional voice research is a separate Experience research unit and remains
blocked until modality authorization, listening control, buffer clearing,
turn-taking, output cancellation, fallback, and attention requirements are
accepted.

### Expected outputs

- a canonical core Experience research unit and Catalogue entry
- interaction and presentation categories for local desktop shell,
  accessibility, state presentation, permission administration, navigation,
  notifications, and degraded operation
- user-state model covering requested, challenge, denied, waiting, active,
  timeout, degraded, partial, unknown, indeterminate, cancelled, failed, and
  completed
- attention, deferral, quiet/focus, bounded prompting, neutral consent, and
  safe-no-response evaluation criteria
- Windows packaging, accessibility, notification, overlay, locale, input, and
  multi-surface evidence requirements
- proof that UI state and content buffers remain local and appropriately
  ephemeral
- acceptance mapping for IC-003, IC-004, IC-007, IC-008, IC-010, IC-013, and
  IC-032 through IC-034

### Key unanswered questions

- What is the smallest interface that keeps all required states inspectable
  without increasing cognitive load?
- How does Experience start independently enough to expose status, permissions,
  recovery, and shutdown in degraded mode?
- How are authoritative domain states rendered without fabrication or semantic
  loss?
- Which interactions may interrupt, defer, expire, suppress, or re-prompt?
- How are grant and deny made equally accessible with no preselection or
  fabricated urgency?
- How are stale status, surface failure, notification leakage, and untrusted
  model/extension text handled?
- Which accessibility, keyboard, screen-reader, scaling, locale, and reduced
  motion requirements are mandatory?
- Which optional presence, voice, overlay, and notification surfaces are
  justified without becoming new capabilities?

### Architectural risks

- UI state being mistaken for domain truth or completion
- consent dark patterns violating Human First and “never manipulative”
- frequent narration or notifications increasing cognitive load
- protected content leaking through notifications, previews, logs, or buffers
- Experience acquiring orchestration, mutation, or grant authority
- optional voice or overlay failure degrading the core interface
- framework constraints hiding capability, permission, or uncertainty identity
- accessibility being deferred until after technology selection

### Estimated research complexity

**High.** The capability has significant Windows, accessibility, privacy, and
behavioral acceptance requirements and presents every other capability's state.

---

## 8. Extension Host

### Why it must be researched

Extension Host requires research only if a separate architecture decision
demonstrates user value and justifies activation under the Complexity Budget.
If activated, it becomes an untrusted-code boundary responsible for extension
identity, isolation, lifecycle, scope attenuation, disposable partitions, and
Companion-mediated contributions.

Research before that decision may characterize feasibility and risk only. It
must not be represented as activation evidence by itself.

### Prerequisite completed research

- separate approved Extension Host activation justification
- all core capability research
- stable Permission Authority proof/identity semantics
- stable Companion contribution, cancellation, recovery, and explanation
  semantics
- stable Experience attention and consent semantics
- accepted extension-specific contract and acceptance updates, if required

### Expected outputs

- if activation is approved, one or more bounded canonical Extension Host
  research units and Catalogue entries
- threat model and isolation-strength requirements
- comparison categories for sandboxing, manifest validation, signing/trust,
  update, revocation, unload, identity propagation, and partitioning
- proof that extensions have no direct domain path and inherit no ambient
  authority
- zero-extension core-operation and crash-containment validation plan
- acceptance mapping for IC-035 through IC-038 and boundary case 34
- explicit build-versus-integrate analysis for unique mediation versus commodity
  isolation and packaging primitives

### Key unanswered questions

- Is extension value sufficient to justify the subsystem?
- What extension classes and contribution types are actually required?
- What attacker and sandbox-escape resistance are in scope?
- How are publisher identity, signing, trust, installation, updates, rollback,
  revocation, and compromised releases handled?
- How is extension identity preserved through Companion and every downstream
  authorization?
- How are resource quotas, crash loops, unload, active invocation cancellation,
  and terminal recovery enforced?
- How are partitions prevented from retaining user knowledge, observations,
  tasks, prompts, responses, or unauthorized caches?
- How are compatibility and deprecation managed without weakening the core?

### Architectural risks

- feasibility work being mistaken for activation approval
- sandbox escape, confused deputy, or inherited Companion authority
- direct or covert domain-capability access
- extension partitions becoming alternate Memory
- supply-chain compromise or malicious update
- identity loss during mediation
- crash loops or resource exhaustion degrading core operation
- extension lifecycle multiplying Runtime Host, permission, cancellation,
  migration, and UX complexity

### Estimated research complexity

**High, conditional.** Untrusted-code isolation and supply-chain lifecycle are
high-risk, but no implementation research is authorized without the preceding
activation gate.

---

## Parallelism and Stop Conditions

Research sessions may overlap only where the dependency model permits.

- Runtime Host framing starts first.
- Workspace Management may frame requirements while Runtime Host research is in
  progress, but its final integration-complexity comparison must use the
  established host boundary assumptions.
- Context Sensing, Action, and Intelligence may run as parallel research units
  after shared prerequisites are recorded.
- Companion Orchestration waits for those three units because their lifecycle,
  cancellation, and failure findings define its coordination obligations.
- Experience waits for Companion and domain-state findings before finalizing
  candidate comparison.
- Extension Host waits for explicit activation justification and all core
  research.

Every unit stops without selection when:

- a higher-authority architecture question materially changes requirements
- a mandatory gate cannot yet be evaluated
- evidence would require implementation outside a bounded disposable spike
- a candidate would change ownership, interaction paths, or contract meaning
- the research scope would duplicate an existing Catalogue unit

---

## Expected Roadmap Outputs

When complete, the roadmap produces:

- eight capability research outcomes, with Extension Host conditional
- one canonical Catalogue research unit for each bounded capability/category
  question
- explicit architecture assumptions and blockers rather than silent decisions
- comparable build, integrate, and hybrid boundaries under ADR-0002
- reproducible validation plans before adoption
- a dependency trace from foundational lifecycle and scope through domain
  capabilities, orchestration, presentation, and optional extension isolation

The roadmap does not require one monolithic research record per capability.
Materially different categories, especially Action classes and optional
Experience modalities, may require separate bounded units under the framework's
single-record rule.

---

## Validation

### Minimizes architectural rework

- Runtime Host is first because process, lifecycle, packaging, health, and
  shutdown assumptions affect every candidate integration.
- Workspace Management follows because authoritative scope affects multiple
  domain capabilities.
- Domain boundaries are researched before orchestration and presentation.
- Companion follows its dependencies instead of inventing their behavior.
- Experience follows the states and controls it must present.
- Extension Host remains last and conditional.

Result: shared constraints are established before high-fan-out dependent
comparisons.

### Reduces future uncertainty

- High-risk permission and Memory research is already complete.
- The next waves resolve lifecycle and scope uncertainty.
- Parallel domain research then resolves sensing privacy, action safety, and
  intelligence viability.
- Cross-capability cancellation, recovery, degradation, and explanation are
  evaluated only after owner behavior is known.

Result: each wave converts upstream unknowns into explicit inputs for the next.

### Respects dependencies

- Permission Authority and Memory findings constrain every remaining unit.
- Runtime Host precedes integration-lifecycle comparison.
- Workspace scope precedes dependent scope comparison.
- Companion follows every core domain owner it coordinates.
- Experience follows the capabilities whose state it presents.
- Extension Host follows all core semantics and a separate activation decision.

Result: no downstream research is permitted to redefine an upstream authority.

### Upholds ADR-0002

- Every remaining common capability is researched before implementation.
- Internal construction remains a comparison option, never a default.
- Unique Workspace authority, policy, and user meaning remain Workspace-owned.
- Commodity functions are evaluated for integration behind replaceable
  adapters.
- Build, integrate, and hybrid options use the same mandatory gates.
- Complexity, lifecycle ownership, migration, licensing, maintenance, security,
  Windows support, and replacement are required evidence.
- No research-plan statement selects, recommends, rejects, or approves a
  technology.

Result: consistent with Integrate Before Reinventing.

### Scope validation

- No new technology research was performed.
- No candidate technology was named or recommended.
- No implementation or runtime code was created.
- No capability ownership, interaction, contract, acceptance case, or ADR was
  changed.
- No Extension Host or optional modality activation was authorized.
- The existing research framework remains the sole research process.

Result: roadmap scope is valid.

---

## Review and Maintenance

Review this roadmap:

- after any capability ownership, interaction, contract, or acceptance change
- after resolution of the user-journey architecture gaps
- when a completed research unit materially changes a downstream assumption
- when a capability is removed, split, merged, or activated
- before beginning Companion, Experience, or Extension Host research
- at the start of each research wave

Revisions preserve completed research records. They update sequence and
dependencies; they do not repeat evidence or rewrite prior Catalogue decisions.
