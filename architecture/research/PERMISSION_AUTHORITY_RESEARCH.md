# Permission Authority Capability Research

Status: Research complete; decision pending
Research ID: PA-001
Version: 1.0
Date opened: 2026-08-01
Last reviewed: 2026-08-01
External evidence accessed: 2026-08-01

This record follows `architecture/12_Capability_Technology_Research_Framework.md`.
It compares patterns and mature open-source approaches without selecting a
technology, approving implementation, or changing architecture.

---

## A. Identity and Scope

- Capability: Permission Authority
- Technology category or question: local authorization architecture, policy
  evaluation, permission proof, approval workflow, revocation, and audit
- Research status: complete for discovery and comparative analysis; no adoption
  decision
- Research owner: Workspace engineering research
- Catalogue entry: `PA-001` in `architecture/03_Research_Catalogue.md`
- Related contracts:
  - `PER-REQ-001` through `PER-REQ-004`
  - `PER-CMD-001` through `PER-CMD-003`
  - `PER-EVT-001` through `PER-EVT-003`
  - `IC-006` through `IC-012`
- Required acceptance cases:
  - Permission cases 5 through 11
  - Status and safety-control cases 16 through 18
  - Audit/privacy case 28
  - Local First cases 29 through 31
  - Compatibility cases 35 through 37
  - Audit/trace cases 38 and 39
  - Control-proof cases 40 through 42
- Relevant ADRs: ADR-0001 through ADR-0008, with primary emphasis on ADR-0001,
  ADR-0002, ADR-0003, ADR-0004, ADR-0005, and ADR-0006

### In scope

- Current authorization and permission-system patterns
- Mature open-source policy and authorization approaches
- Local-first authorization and local persistence
- Permission approval, denial, challenge, redemption, and administration
- Temporary, one-operation, session, lease, and persistent-until-revoked grants
- Point-of-use validation, revocation ordering, and replay resistance
- Effect proofs and independently governed operation-control proofs
- Auditability, explainability, privacy minimization, and tamper evidence
- Windows desktop constraints and lessons from other desktop, browser, and
  mobile permission systems
- Build, integrate, and hybrid solution categories

### Out of scope

- Technology selection or adoption
- Runtime code, prototype, benchmark, schema, token format, or process-boundary
  implementation
- Final permission scope taxonomy
- Resolution of compound multi-scope authorization, consent fairness, or other
  open architecture gaps
- Action-specific irreversible commit points and cancellation policy
- Extension Host activation

---

## B. Capability Objective

Permission Authority must remain the sole local source of explicit, revocable,
and explainable permission truth. It must issue and validate authority bound to
the subject, requester capability, purpose, target, operation, workspace scope,
optional extension identity, lifetime, and usage constraints. Every protected
owner must validate at the exact use or effect boundary. New meaningful effects
fail closed when authority is unavailable.

The capability owns the permission catalogue, grants, denials, policies,
challenge state, proof state, use/consumption state, and permission audit. It
does not execute domain actions, choose user goals, render consent surfaces, or
own another capability's operation result.

---

## C. Research Questions

### Answered findings

1. No examined policy engine, relationship database, token system, or desktop
   permission model provides the complete Workspace Permission Authority
   contract.
2. Mature systems consistently separate declaration or eligibility, user
   consent, authoritative decision, enforcement, grant lifetime, revocation,
   and observable use.
3. A policy evaluator can contribute decision logic, but it does not by itself
   own challenge state, issue and consume proofs, order revocation against an
   effect, or create the required privacy-minimized audit.
4. A relationship permissions database is strongest when authorization depends
   on durable subject-resource relationships. Workspace additionally requires
   purpose, requester, operation, one-use consumption, and local desktop
   lifecycle semantics.
5. A cryptographic capability token can authenticate and attenuate bound
   authority. Signature validity alone cannot establish current non-revocation,
   single use, policy freshness, or effect ordering.
6. Immediate effect revocation and disconnected verification are conflicting
   goals. Workspace's existing separation between effect authority and
   safety-reducing operation-control authority is a credible way to assign
   different consistency requirements without weakening effect authorization.
7. Operating-system permission prompts are useful workflow precedents, not
   interchangeable security mechanisms. Their guarantees depend on a kernel,
   broker, sandbox, process identity, trusted UI, or independently protected
   store that an ordinary in-process prompt does not inherit.
8. Temporary grants need an explicit lifetime model. “Permanent” should mean
   persistent until revoked, not irrevocable.
9. Revocation events improve responsiveness but cannot prove ordering. The
   authoritative point-of-use validation or consumption transaction must decide
   whether a use precedes or follows revocation.
10. Content-rich policy traces and default logs conflict with Workspace audit
    minimization. Stable reason codes, policy revisions, authorization IDs, and
    controlled purpose/effect classes are more suitable than retained inputs.

### Unanswered questions

- What exact permission scope taxonomy is understandable without becoming
  over-broad?
- Is a Workspace proof a stateful opaque reference, a signed typed envelope, or
  a hybrid whose cryptographic representation remains opaque to callers?
- Does “opaque outside Permission Authority” permit designated operation owners
  to verify a typed operation-control envelope locally?
- What process or isolation boundary prevents same-process bypass of the
  reference monitor?
- How are requester, user, extension, and process identities authenticated and
  kept stable across updates, restart, reinstall, and executable replacement?
- What durable transaction defines the ordering of grant, revoke, proof
  consumption, policy change, and audit append?
- How are state rollback and clock rollback detected after backup restoration,
  hibernation, database copy, OS reinstall, or TPM clearing?
- What is the offline-control lease duration, and does it survive restart or
  hibernation?
- Must operation-control revocation be locally replicated during Authority
  outage, or is bounded lease expiry the sole stale-use bound?
- What audit attacker is in scope: accidental corruption, another same-user
  process, local administrator, or full storage rollback?
- Which grants may be persistent, and which require one-use, task, foreground,
  process, inactivity, or fixed-duration lifetimes?
- How are batch and multi-effect requests split so one approval does not hide
  materially different effects?
- How will independently revocable scope sets be represented if one operation
  requires several scopes?
- Which approval surface properties are required to prevent spoofing,
  preselection, urgency inflation, repeated prompting, and requester-controlled
  prompt injection?
- What recovery path follows permission-store corruption or loss without
  silently recreating authority?

---

## D. Required Functional Capabilities

- `PA-FR-001` — Maintain a local default-deny permission catalogue.
  Mandatory; no candidate category supplies the Workspace catalogue unchanged.
- `PA-FR-002` — Authorize, deny, or challenge using exact subject, requester,
  purpose, target, operation, scope, extension, lifetime, and use constraints.
  Mandatory; policy engines can evaluate these attributes with adapters.
- `PA-FR-003` — Resolve a user decision and require challenge redemption against
  the unchanged original context. Mandatory; Workspace-owned workflow is needed
  for every examined candidate.
- `PA-FR-004` — Issue opaque effect authority and separate operation-control
  authority for protected non-immediate work. Mandatory; no examined candidate
  supplies this split natively.
- `PA-FR-005` — Validate and consume effect authority at each independently
  meaningful effect. Mandatory; an authoritative Workspace transaction is
  needed even when an external evaluator or token is used.
- `PA-FR-006` — Reject expiry, revocation, replay, requester mismatch, subject
  mismatch, purpose mismatch, target mismatch, operation mismatch, scope
  mismatch, and extension mismatch. Mandatory; fields are representable in
  several candidates, but atomic replay/revocation enforcement is not complete.
- `PA-FR-007` — Grant, deny, revoke, and update narrowly scoped automatic
  policy. Mandatory; automatic policy must still pass authorize and validate.
- `PA-FR-008` — Expose requester-relevant catalogue and explanations without
  exposing proofs, secrets, unrelated grants, or protected content. Mandatory;
  requires a Workspace-owned explanation projection.
- `PA-FR-009` — Publish minimized challenge, decision, and revocation events.
  Mandatory; events never grant authority.
- `PA-FR-010` — Retain bounded, content-free permission audit and trace
  metadata. Mandatory; no examined candidate's default log is sufficient.
- `PA-FR-011` — Support one-operation, task/session, fixed-lease, and persistent-
  until-revoked grant policies where architecture permits. Mandatory as a
  policy capability; exact permitted lifetimes remain unresolved.
- `PA-FR-012` — Support locally verifiable, owner-bound safety control during a
  bounded Authority outage without authorizing continuation or new effects.
  Mandatory; no examined candidate supplies the complete behavior.

---

## E. Required Non-Functional Capabilities

- `PA-NFR-001` — Fail closed for every new protected effect when authority or
  authoritative state is unavailable or uncertain.
- `PA-NFR-002` — Provide low interactive decision and validation latency.
  Numerical thresholds require later reproducible measurement.
- `PA-NFR-003` — Preserve durable local grants, policy, consumption, and audit
  ordering across crash and restart.
- `PA-NFR-004` — Resist tampering, stale cache acceptance, storage rollback,
  proof theft, proof substitution, and replay.
- `PA-NFR-005` — Remain operable without internet for authorization,
  administration, revocation, audit, and explanation.
- `PA-NFR-006` — Be testable against every applicable contract acceptance case,
  including races and corrupted state.
- `PA-NFR-007` — Keep policy/evaluator, proof representation, and persistence
  replaceable behind Permission Authority contracts.
- `PA-NFR-008` — Avoid logs, traces, caches, and diagnostics becoming alternate
  Memory stores.
- `PA-NFR-009` — Support deterministic, versioned decision semantics and fail
  closed on unknown required policy, status, or invariant.
- `PA-NFR-010` — Fit the Windows-targeted local desktop lifecycle without an
  unjustified service, external database, or cloud control plane.

---

## F. Local First Requirements

All core authorization, challenge state, grant administration, revocation,
point-of-use validation, audit, and explanation must work with the network
unavailable.

Candidate implications:

- In-process evaluators can operate offline when policy and entity data are
  local.
- A local sidecar or permissions service can operate offline, but introduces
  process startup, health, IPC, packaging, upgrade, and shutdown obligations.
- A relationship service backed by a local database can operate offline only if
  every durable dependency is shipped and managed locally.
- Self-contained tokens can be verified offline, but effect authority still
  requires current local revocation and consumption state.
- Optional network policy distribution, telemetry, hosted administration, or
  remote audit is not acceptable for the core path.
- Installation, activation, policy acquisition, key recovery, migration, and
  disaster recovery must also be demonstrably local; “offline evaluation” alone
  is insufficient.

Required future evidence:

- Cold start, restart, grant, revoke, challenge, validate, consume, explain, and
  audit with all network interfaces unavailable
- Authority outage behavior for effect and control proofs
- Recovery from interrupted local database and key operations
- Proof that no candidate silently uses a hosted control plane or telemetry

---

## G. Privacy Requirements

Permission Authority receives sensitive metadata about what the user and
capabilities intend to do. It must not retain raw user intent, target names,
document content, prompts, observations, secrets, or proof bytes in ordinary
audit.

The durable audit should prefer:

- authorization ID
- controlled purpose and effect classes
- requester and responsible capability classes
- permission scope
- policy revision and stable decision-basis identifiers
- validation or consumption outcome
- operation ID
- expiry, revocation, partial, and terminal classes
- correlation and causal identifiers
- bounded timing and sequence metadata

Candidate policy traces, request logs, relationship identifiers, and authorizer
snapshots may contain more information than this. They are diagnostic evidence,
not approved production audit formats. Masking support lowers risk but does not
transfer audit ownership away from Permission Authority.

No permission or denial history may be reused for training, analytics, or remote
telemetry by default. Detailed long-term human-readable rationale belongs only
in separately authorized Memory.

---

## H. Security Considerations

### Reference-monitor model

The strongest common architectural pattern is a local policy decision and state
owner with enforcement at every protected capability. Complete mediation,
tamper resistance, and a small verifiable enforcement boundary matter more than
the policy language alone.

### Policy models

- RBAC is useful for administrative groupings but too coarse on its own.
- ABAC directly represents subject, resource, operation, requester, purpose,
  environment, workspace, and extension attributes. Attribute provenance and
  freshness become security-critical.
- ReBAC represents durable relationships and delegation well, but does not
  inherently provide one-use effect consumption or purpose truth.
- Policy-as-code supports review, versioning, validation, and reason IDs, but
  engine errors and unknown inputs must be adapted to Workspace fail-closed
  semantics.
- Capability-style proofs support least authority and attenuation, but copied
  bearer authority is replayable unless current state rejects it.

### Proof security

A signed or MACed proof can establish authenticity and context binding. It
cannot alone establish current grant state, non-revocation, unused status,
policy freshness, or ordering against an effect.

Proof-of-possession can reduce use by the wrong process or extension, but does
not prevent the legitimate holder from submitting a duplicate effect. Exact
single-use state remains necessary.

Effect and operation-control proofs require distinct type, validation, keying,
and audience rules so one can never be substituted for the other.

### Revocation and replay

Suitable pattern categories include:

- authoritative opaque reference or allowlist lookup
- current grant/proof state plus atomic use consumption
- short expiry or bounded lease
- grant, policy, subject, or key epochs
- revocation denylist
- signing-key rotation for exceptional broad invalidation
- push revocation events for acceleration only

The effect path requires authoritative local ordering. The control path may
tolerate bounded stale revocation only because it can observe minimized status
or reduce risk and cannot create effects.

Atomic local proof consumption cannot make an arbitrary OS effect exactly once.
If consumption occurs first, a crash can consume authority without the effect.
If the effect occurs first, a crash can leave authority apparently unused.
Owners still need idempotency, operation state, reconciliation, and honest
partial or indeterminate outcomes.

### Key and local-storage security

- Windows DPAPI can protect local secret material for a user and machine, but it
  does not isolate secrets from all code running as that user and does not
  prevent rollback to an older valid blob.
- Windows CNG supports software and TPM-backed key storage and non-exportable
  keys. ACLs, process identity, recovery, hardware absence, and TPM reset remain
  material concerns.
- Windows Credential Locker must not be assumed to isolate one ordinary desktop
  application from all other applications running as the same user.
- Monotonic time is useful within a boot. Durable lease and rollback semantics
  across restart need a protected epoch, refreshed authority, or fail-closed
  recovery rule.

### Audit integrity

Hash chains, signatures, and Merkle structures can reveal mutation or omission
only when a trustworthy checkpoint or key survives independently of the log.
Local append-only storage is not automatically tamper-evident. The required
attacker model and checkpoint recovery policy remain open.

---

## I. Performance Considerations

No candidate benchmark was performed in this research. Performance remains an
evidence gap rather than an inferred pass.

Later comparison must measure on the supported Windows hardware classes:

- cold and warm Authority readiness
- authorize, validate, and consume latency
- challenge creation and redemption latency
- grant/revoke transaction latency
- throughput under concurrent protected effects
- latency with high-consistency or cache-bypassing checks
- database growth and compaction under bounded audit retention
- CPU, memory, disk, battery, and background process cost
- policy reload and migration pause
- control-proof verification during Authority outage
- shutdown, cancellation, and crash-recovery responsiveness

Tests must include representative policy size, grant count, revocation races,
one-use proofs, multi-effect operations, corrupted input, offline mode, and
resource exhaustion. A sidecar/service candidate must include IPC and lifecycle
cost; an embedded candidate must include host-language and Wasm overhead.

---

## J. Explainability Considerations

The approval and explanation model must distinguish:

- user grant, user denial, dismissal, policy denial, system denial, unknown
  scope, expired challenge, expired grant, revoked grant, replay, context
  mismatch, unavailable Authority, and unavailable target
- requested, waiting, active, completed, partial, outcome-unknown,
  indeterminate, cancelled, failed, and denied operation states
- permission granted from resource actually used
- temporary from persistent-until-revoked authority
- effect authority from safety-control authority

Policy IDs, relationship paths, evaluation traces, and matching rules can help
derive a reason, but none should be exposed directly without stable,
privacy-minimized Workspace reason classes.

Every explanation must preserve what, why, responsible capability, requester,
permission scope, authorization ID, minimized target class, lifetime, decision
basis, revocation path, uncertainty, and available user action. It must never
contain proof bytes, secrets, raw target content, unrelated grants, or
unminimized policy inputs.

---

## K. Licensing Evaluation

All licence statements below are research snapshots and require dependency-level
legal review before adoption.

- Open Policy Agent 1.18.2: Apache-2.0.
- Cedar 4.11.2: Apache-2.0. Version 4.12.0 was listed as “coming soon” on the
  research date and was not treated as a released version.
- Apache Casbin 3.10.0 and Casbin Rust 2.20.0: Apache-2.0.
- OpenFGA 1.18.1: Apache-2.0.
- SpiceDB 1.56.0: Apache-2.0.
- Eclipse Biscuit `biscuit-auth` 6.0.0: Apache-2.0.
- Legacy Oso open-source library 0.27.3: Apache-2.0; officially deprecated.

No licence was approved. Transitive dependencies, notices, trademarks, patent
terms, binary redistribution, and any optional hosted or enterprise components
remain to be reviewed for a selected evaluation scope.

---

## L. Maintenance Evaluation

- OPA has recent 2026 releases, official Windows artifacts, extensive
  documentation, and a broad policy ecosystem.
- Cedar has recent 2026 releases, a Rust implementation, schema validation, and
  formal-analysis tooling. Toolchain compatibility must be checked against the
  repository's Rust baseline at evaluation time.
- Casbin has recent Go and Rust releases and a broad multi-language ecosystem.
  Feature parity and adapter quality vary by language and require port-specific
  evidence.
- OpenFGA has recent 2026 releases, official Windows artifacts, signed release
  metadata, SDKs, and active documentation.
- SpiceDB has recent 2026 releases and official Windows artifacts. Its
  production-style durable deployment assumes a separate supported datastore.
- Biscuit has an active Rust implementation and bindings, but a smaller
  ecosystem than the general policy engines and relationship services.
- Legacy Oso remains available but is officially deprecated. Repository
  availability is not evidence of a current feature-development roadmap.

Release recency and repository popularity are not acceptance evidence. A future
evaluation must inspect security policy, advisories, maintainer concentration,
issue response, platform support, release signatures, dependency freshness, and
migration quality for the exact version tested.

---

## M. Community Maturity Evaluation

The candidate set includes established general-purpose policy projects,
relationship-authorization services, a capability-token project under the
Eclipse Foundation, and a deprecated legacy library.

Community maturity differs by category:

- Policy engines have broader deployment knowledge and policy tooling.
- Relationship services have strong modeling and consistency literature but
  are oriented toward service infrastructure.
- Capability-token ecosystems have focused cryptographic and attenuation
  expertise but less complete application authority workflow.
- Multi-language projects increase integration options but can hide behavioral
  divergence between ports.

Before adoption, production-use claims must be supported by public operational
evidence or a reproducible local validation. Stars, downloads, or foundation
membership alone do not prove Workspace fit.

---

## N. Platform Compatibility

Windows is mandatory.

- OPA publishes a Windows executable. Local process integration is available;
  embedded Go is not a Rust option, while Wasm needs host integration and
  built-in compatibility validation.
- Cedar embeds directly in Rust and provides a Wasm route for JavaScript and
  TypeScript. Exact Rust toolchain requirements must be verified.
- Casbin has Rust and Node implementations, allowing in-process integration.
  Required feature parity must be established for the chosen port.
- OpenFGA publishes Windows binaries and supports SQLite, but is a separate
  local service. TypeScript SDK support is official; Rust integration is not an
  equivalent first-party path in the reviewed evidence.
- SpiceDB publishes a Windows binary and Node client. Durable supported
  datastores add process and packaging burden; SQLite was not listed as a
  supported durable datastore.
- Biscuit is Rust-first and has Wasm bindings; Windows release artifacts exist.
- Legacy Oso has Rust core and language bindings, but deprecation creates
  support risk independent of platform feasibility.

Windows packaging, signing, installer behavior, update, rollback, service
lifecycle, user-profile separation, accessibility, standard-user operation, and
secure key-store behavior remain untested.

Windows OS permissions are an external prerequisite layer, not a replacement
for product permissions. Classic desktop applications do not receive a uniform
per-application Windows consent model, and several prompt-style WinRT APIs are
not supported in ordinary desktop apps. Protected resource APIs must still be
handled at actual access.

---

## O. Integration Complexity

### Embedded evaluator

Lowest additional lifecycle footprint, but Workspace must still provide the
grant store, challenge workflow, proof state, consumption transaction, audit,
key lifecycle, explanation projection, and contract adapters.

### Local sidecar or service

Adds process startup, readiness, authentication, IPC, health, crash recovery,
binary packaging, updates, migrations, logs, shutdown, and attack surface.
Service APIs must not become a second public authorization authority.

### Relationship authorization service

Adds model/schema management and usually service/datastore lifecycle. It may be
disproportionate if Workspace relationships remain small and local, but this
requires measurement rather than assumption.

### Capability-token component

Adds key protection, token serialization, transport, audience/type separation,
revocation state, one-use state, operation binding, and leakage controls.
Cryptographic locality does not remove the stateful Authority.

### Hybrid

A Workspace-owned authority could compose a policy evaluator or token
primitive with Workspace-owned lifecycle and state. This preserves ownership
only if external components remain replaceable implementation details and
cannot bypass or redefine contracts.

---

## P. Extensibility

A viable direction must permit:

- adding permission scopes without changing ownership or creating omnibus grants
- versioned policy and proof evolution with fail-closed unknown semantics
- future extension identity and compound scope support
- replacement of evaluator, proof representation, and storage through adapters
- export and migration of grants, policies, audit metadata, and decision-basis
  identifiers
- coexistence during bounded migration without accepting weaker semantics

Extensibility does not justify a general remote authorization platform, plugin
subsystem, or policy DSL unless required by demonstrated Workspace needs.

---

## Q. Failure Modes

### Authority unavailable

New meaningful effects deny. Existing operation-control authority may provide
only bounded local status and safety reduction.

### Policy or schema error

Unknown required rules, invalid policy, missing attributes, or incompatible
versions fail closed. A policy engine's native “skip erroneous rule” behavior
must not silently weaken a required prohibition.

### Stale cache or relationship read

A revoked grant may appear valid. Effect validation must bypass stale caches or
use an equivalent authoritative consistency guarantee.

### Revocation race

Every meaningful effect orders validation/consumption against revocation.
Effects ordered earlier may complete only their already bound effect.

### Replay or duplicate delivery

The same semantic delivery retains identity and cannot produce a second use.
Changed purpose, target, operation, or content is a new authorization attempt.

### Crash between consumption and effect

The owner may reach outcome-unknown or indeterminate state. No blind retry;
owner-specific idempotency and reconciliation apply.

### Clock rollback or restart

Effect validation fails closed when expiry state is uncertain. Control-lease
restart semantics require an architecture decision.

### Store rollback, corruption, or lost key

No grant is silently recreated. Recovery must identify loss of authority state,
invalidate uncertain proofs, explain the degraded condition, and require an
explicit recovery policy.

### Audit leakage

Content-rich default logs, traces, snapshots, or telemetry may violate Privacy
First. Production evidence must prove allowlisted fields and bounded retention.

### Audit tampering

Hashing without an independently protected checkpoint does not prove complete
history. The assurance claim must match the threat model.

### Approval surface unavailable or spoofed

The challenge remains denied. Requester-controlled text cannot be treated as
trusted explanation, and repeated prompting cannot bypass denial.

### Update or migration incompatibility

Unsupported required permission, proof type, terminal status, or invariant fails
closed. Migration cannot broaden authority or lose revocation state.

---

## R. Migration Risk

Material migration surfaces include:

- permission catalogue and scope identifiers
- grants, denials, automatic policy, expiry, and revocation state
- challenge state and redemption semantics
- proof and key versions
- consumption and replay state
- policy/schema revisions
- content-free audit and terminal retention
- process/service configuration

Relationship services, policy languages, and capability tokens each introduce
different data and semantic lock-in. Exporting syntactically valid policies or
tuples is insufficient if the replacement changes deny precedence, unknown
handling, consistency, expiry, replay, or explanation meaning.

A migration must support bounded coexistence, exact authority preservation,
reproducible equivalence cases, rollback without resurrecting revoked authority,
and invalidation of proof formats that cannot be safely translated.

---

## S. Reasons to Build Internally

Evidence supports keeping these responsibilities Workspace-owned:

- permission and consent lifecycle
- grant, denial, challenge, and automatic-policy state
- exact architecture-specific authorization context
- effect/control-proof split
- point-of-use consumption and revocation ordering
- operation-control binding and bounded safety lease
- privacy-minimized audit schema
- stable user-facing reason and explanation model

These are unique Workspace trust and contract semantics. Internal ownership does
not imply implementing a policy language, cryptographic primitive, database, or
secure key provider from first principles.

---

## T. Reasons to Integrate Externally

Commodity components may reduce risk for:

- policy parsing, evaluation, schema validation, and static analysis
- relationship graph evaluation
- cryptographic signing, verification, and attenuation primitives
- local transactional storage
- Windows key protection
- structured policy testing

External integration is justified only if adapters preserve Workspace ownership,
fail-closed semantics, local operation, privacy, explanation, replacement, and
the Complexity Budget. No examined component can be delegated the complete
Permission Authority role without substantial Workspace-owned control logic.

---

## U. Candidate Comparison

### Candidate solution categories

1. Workspace-native stateful reference monitor and evaluator
2. Workspace authority with an embedded policy-as-code evaluator
3. Workspace authority with a local relationship authorization service
4. Workspace authority with cryptographic capability-style proof primitives
5. Hybrid evaluator plus stateful Workspace proof/consumption authority
6. OS-broker-inspired approval and scoped-handle workflow

### Open Policy Agent 1.18.2

- Category: general policy evaluator; local process, Go embedding, or Wasm
- Strengths: expressive policy, local evaluation, policy bundles, decision IDs,
  official Windows binary, policy tests, and decision-log masking/drop controls
- Trade-offs: no native grant/challenge/proof/consumption lifecycle; Rego and
  service/Wasm integration add policy and lifecycle surface; default decision
  logs contain inputs/results unless deliberately minimized
- Required adapter: complete Workspace state, proof, audit, and explanation
  layer
- Disposition: retain for comparative evaluation; no selection

### Cedar 4.11.2

- Category: embedded policy-as-code evaluator
- Strengths: default-deny and forbid-overrides semantics, typed schema
  validation, determining policy identifiers, Rust implementation, Wasm route,
  and formal-analysis tooling
- Trade-offs: no persistence, challenge, proof, consumption, replay ledger, or
  audit; erroneous policies are excluded from decisions, so admission
  validation and fail-closed adaptation are mandatory
- Required adapter: complete Workspace authority state and workflow
- Disposition: retain for comparative evaluation; no selection

### Apache Casbin 3.10.0 / Casbin Rust 2.20.0

- Category: embedded access-control library
- Strengths: ACL/RBAC/ABAC/domain models, custom matchers, policy adapters,
  multi-language ecosystem, Rust/Node options, and matching-rule information
- Trade-offs: flexible model/effect configuration can weaken defaults if
  misconfigured; feature parity varies by port; enforcement can be disabled in
  the main API; the Go `Explain` feature uses an LLM and is not a deterministic
  local authority explanation
- Required adapter: locked model, validated configuration, and complete
  Workspace proof/workflow/audit layer
- Disposition: retain for comparative evaluation; no selection

### OpenFGA 1.18.1

- Category: local relationship authorization service
- Strengths: ReBAC model, conditions, local Windows binaries, SQLite support,
  higher-consistency cache bypass, and active SDK/tooling ecosystem
- Trade-offs: separate process/API, relationship model does not natively
  represent proof consumption, challenge, or effect/control split; contextual
  tuples must not become caller-asserted grants; mutation history is not
  decision audit
- Required adapter: trusted context construction plus complete Workspace proof,
  challenge, consumption, audit, and explanation state
- Disposition: retain for relationship-heavy comparative evaluation; no
  selection

### SpiceDB 1.56.0

- Category: relationship permissions database/service
- Strengths: ReBAC, caveats, native expiring relationships, explicit
  consistency controls, relationship watch, debug traces, and Windows binary
- Trade-offs: supported durable deployments require an additional database;
  service lifecycle is high for a local desktop; watch is not complete decision
  audit; expiry is time-based rather than one-use consumption
- Required adapter: local service/database lifecycle plus complete Workspace
  proof/workflow/audit layer
- Disposition: retain as a mature service-pattern comparator; no selection

### Eclipse Biscuit `biscuit-auth` 6.0.0

- Category: cryptographic capability token
- Strengths: Rust-first, offline verification, public-key signing, caveated
  attenuation, explicit checks, and Wasm bindings
- Trade-offs: revocation requires external state; bearer leakage and replay
  remain concerns; no challenge, atomic consumption, effect/control split, or
  operation binding; authorizer snapshots can be content-rich
- Required adapter: keys, authoritative revocation/consumption state, typed
  proof lanes, challenge workflow, audit, and explanation
- Disposition: retain as the capability-proof category comparator; no selection

### Legacy Oso open-source library 0.27.3

- Category: embedded declarative policy library
- Strengths: Rust core, local evaluation, host-language bindings, and familiar
  actor/action/resource rules
- Trade-offs: officially deprecated legacy line, uncertain successor timeline,
  host callback trust surface, and no Workspace proof/workflow/audit lifecycle
- Required adapter: complete Workspace authority layer
- Disposition: document as maintenance-risk evidence; not selected or formally
  rejected by this research

### Internal construction

- Category: Workspace-native authority
- Strengths: exact contract fit, minimal semantic translation, and direct
  control of local state and privacy
- Trade-offs: highest responsibility for authorization correctness, policy
  evolution, cryptography, formal testing, secure storage, migration, and long-
  term maintenance
- Disposition: mandatory build-versus-integrate comparator; no selection

---

## V. Required Acceptance Criteria

Before any candidate or composition can be adopted, it must demonstrate:

1. One Permission Authority owner and no bypass path.
2. Exact binding of requester, subject, purpose, target, operation, workspace,
   extension, lifetime, and use constraints.
3. Default deny and fail-closed unknown/error handling.
4. Challenge redemption that never treats a decision event as authority.
5. Point-of-use validation and durable single-use consumption ordered against
   revocation.
6. Correct multi-effect behavior when revocation occurs between effects.
7. Cryptographic and semantic separation of effect and control authority.
8. Bounded local safety control during Authority outage with no new effect.
9. Rejection of wrong requester, subject, purpose, target, operation, scope,
   extension, expiry, revocation, replay, type, audience, and version.
10. Local operation with the network unavailable, including administration,
    revocation, audit, explanation, update, and recovery.
11. Stable, privacy-minimized explanations for every decision and failure class.
12. Content-free bounded audit with declared tamper and rollback guarantees.
13. Windows packaging, signing, standard-user, key-store, update, and lifecycle
    evidence.
14. Reproducible latency, resource, concurrency, crash, corruption, and recovery
    evidence.
15. Versioning and migration that never resurrects revoked authority or weakens
    meaning.
16. Acceptable exact-component and transitive licensing.
17. Security, maintenance, and governance risks with named controls.
18. Complexity Budget evidence for every process, runtime, policy language,
    database, key store, and adapter introduced.
19. Explicit build-versus-integrate reasoning under ADR-0002.
20. Resolution or named acceptance of the architecture unknowns that affect the
    evaluation scope.

### Recommended evaluation criteria

Mandatory gates must precede preferences. Surviving candidates should be
compared on:

- semantic contract fit
- revocation and consumption correctness
- proof separation and replay resistance
- challenge and administration adaptability
- Local First completeness
- audit privacy and integrity
- deterministic explainability
- Windows and Rust/Tauri compatibility
- measured latency and resource cost
- failure, recovery, and migration behavior
- maintenance, security response, and governance
- licence and distribution obligations
- integration and operational complexity
- replaceability and lock-in
- total lifecycle ownership cost

No aggregate score may override a failed mandatory gate.

---

## W. Decision and Review

- Decision: no technology selected
- Selected scope: none
- Decision rationale: this research establishes categories, evidence, trade-offs,
  and acceptance criteria only
- Rejected candidates: none; the deprecated Oso line is recorded as a current
  maintenance risk rather than converted into a selection decision
- Conditions before selection:
  - resolve architecture questions that materially change proof, consent, scope,
    time, isolation, and recovery requirements
  - run bounded, reproducible evaluation against mandatory acceptance criteria
  - validate the exact candidate version and transitive dependency set
- Remaining unknowns: listed in section C
- Required ADR: none for this research
- Open Source Registry action: none
- Engineering Ledger action: record completion of PA-001 research
- Review date: 2027-02-01, or earlier on a trigger below
- Event-driven re-evaluation triggers:
  - relevant architecture gap is resolved or requirements change
  - material candidate release, deprecation, security advisory, licence change,
    or platform-support change
  - a proof, policy, isolation, audit, or time assumption becomes testable
  - measured Windows evidence invalidates a finding
- Approver: pending architecture review

---

## Architectural Trade-offs

- Central state gives immediate revocation and one-use ordering but makes
  Authority availability and integrity critical.
- Offline self-contained proof verification improves availability but cannot
  provide immediate effect revocation without current state.
- Embedded evaluation reduces lifecycle cost but shares the application's
  process and tampering boundary.
- A sidecar can isolate policy execution but adds IPC, packaging, authentication,
  health, update, and shutdown complexity.
- Rich policy languages improve expressiveness but increase configuration,
  validation, predictability, and explanation burden.
- Relationship models simplify durable membership/delegation but fit
  operation-purpose proofs less directly.
- Prompting every effect maximizes freshness but creates fatigue and
  habituation; persistent grants reduce friction but enlarge stale-authority
  exposure.
- Object-scoped grants improve least authority but require stable object
  identity and stale-reference handling.
- Detailed audit improves diagnosis but can violate Privacy First; content-free
  audit reduces leakage but limits historical narrative.
- Hardware-backed keys improve extraction resistance but complicate recovery,
  migration, unsupported hardware, and rollback behavior.

---

## Desktop Permission Model Findings

### Windows

Windows combines package capability declarations, user privacy settings, object
ACL/token checks, handles, restricted capabilities, and UAC rather than one
uniform desktop permission model. Packaged app capability status distinguishes
system denial, user denial, missing declaration, prompt required, and allowed.
Classic desktop apps have important API and per-app privacy limitations.

Lesson: distinguish decision sources and check actual resource access. UAC's
secure desktop demonstrates trusted consent UI, but broad process elevation is
not a model for narrow Workspace authority. Existing Windows handles also show
that changing policy may not terminate already granted access.

### macOS

TCC remembers category-level privacy decisions and lets users revoke them in
system settings. App Sandbox and security-scoped resources provide a more
capability-like object selection and lifetime model.

Lesson: request in context, keep purpose clear, separate read from write, and
pair durable choices with explicit access lifetime. System indicators show that
current-use transparency and historical audit are different concerns.

### Linux portals and Polkit

XDG portals broker user-selected resources and define process, session, and
persistent-until-revoked modes. Restore tokens can be single-use. Polkit
separates an untrusted requester, privileged mechanism, policy authority, and
authentication agent; it also exposes temporary authorization and warns about
process-identity races.

Lesson: use stable caller identity, trusted prompt ownership, explicit session
handles, revocable temporary authority, and point-of-use enforcement.

### Browser and mobile

Browsers retain control of trusted permission UI, bind authority to origins,
support transient user activation, and increasingly offer one-time grants,
quiet prompts, and automatic expiry. Android and Apple platforms support
one-time, while-in-use, object-subset, precision-limited, and persistent grant
variants.

Lesson: grant lifetime and scope are first-class dimensions. Status is advisory;
the actual protected access must still handle revocation and policy change.
Denial should degrade only the dependent feature, and repeated prompting must
not punish refusal.

---

## Evidence Register

All external sources were accessed 2026-08-01.

### Authorization projects

- OPA releases and source: https://github.com/open-policy-agent/opa/releases
- OPA integration modes: https://openpolicyagent.org/docs/integration
- OPA decision logs and masking:
  https://www.openpolicyagent.org/docs/management-decision-logs
- OPA bundles and Wasm:
  https://openpolicyagent.org/docs/management-bundles
- Cedar releases: https://github.com/cedar-policy/cedar/releases
- Cedar authorization semantics:
  https://docs.cedarpolicy.com/auth/authorization.html
- Cedar validation: https://docs.cedarpolicy.com/policies/validation.html
- Cedar security: https://docs.cedarpolicy.com/other/security.html
- Casbin overview and model: https://casbin.org/docs/overview/
- Casbin releases: https://github.com/apache/casbin/releases
- Casbin Rust releases: https://github.com/apache/casbin-rs/releases
- OpenFGA releases: https://github.com/openfga/openfga/releases
- OpenFGA authorization concepts:
  https://openfga.dev/docs/authorization-concepts
- OpenFGA conditions: https://openfga.dev/docs/modeling/conditions
- OpenFGA consistency: https://openfga.dev/docs/interacting/consistency
- OpenFGA local datastore configuration:
  https://openfga.dev/docs/getting-started/setup-openfga/configure-openfga
- SpiceDB releases: https://github.com/authzed/spicedb/releases
- SpiceDB consistency:
  https://authzed.com/docs/spicedb/concepts/consistency
- SpiceDB expiring relationships:
  https://authzed.com/docs/spicedb/concepts/expiring-relationships
- SpiceDB datastores:
  https://authzed.com/docs/spicedb/concepts/datastores
- SpiceDB access-control audit:
  https://authzed.com/docs/spicedb/modeling/access-control-audit
- Eclipse Biscuit Rust:
  https://github.com/eclipse-biscuit/biscuit-rust
- Biscuit specification:
  https://doc.biscuitsec.org/reference/specifications
- Biscuit authorization policies:
  https://doc.biscuitsec.org/getting-started/authorization-policies
- Oso legacy deprecation:
  https://www.osohq.com/docs/oss/getting-started/deprecation.html

### Security standards and patterns

- NIST reference monitor:
  https://csrc.nist.gov/glossary/term/reference_monitor
- NIST SP 800-207 Zero Trust:
  https://csrc.nist.gov/pubs/sp/800/207/final
- NIST SP 800-162 ABAC:
  https://csrc.nist.gov/pubs/sp/800/162/upd1/final
- NIST SP 800-53 Rev. 5:
  https://csrc.nist.gov/pubs/sp/800/53/r5/final
- RFC 6750 bearer tokens: https://www.rfc-editor.org/rfc/rfc6750.html
- RFC 7009 token revocation: https://www.rfc-editor.org/rfc/rfc7009.html
- RFC 7662 token introspection: https://www.rfc-editor.org/rfc/rfc7662.html
- RFC 8725 JWT best practices: https://www.rfc-editor.org/rfc/rfc8725.html
- RFC 9449 proof of possession: https://www.rfc-editor.org/rfc/rfc9449.html
- RFC 5848 signed syslog: https://www.rfc-editor.org/rfc/rfc5848.html
- RFC 9162 Certificate Transparency:
  https://www.rfc-editor.org/rfc/rfc9162.html
- Macaroons paper:
  https://research.google/pubs/macaroons-cookies-with-contextual-caveats-for-decentralized-authorization-in-the-cloud/
- Zanzibar paper:
  https://research.google/pubs/zanzibar-googles-consistent-global-authorization-system/
- SQLite atomic commit: https://sqlite.org/atomiccommit.html

### Desktop, browser, and mobile models

- Windows app capability declarations:
  https://learn.microsoft.com/en-us/windows/apps/package-and-deploy/app-capability-declarations
- Windows AppCapability status:
  https://learn.microsoft.com/en-us/uwp/api/windows.security.authorization.appcapabilityaccess.appcapabilityaccessstatus
- WinRT desktop support limitations:
  https://learn.microsoft.com/en-us/windows/apps/desktop/modernize/winrt-api-desktop-app-support
- Windows camera privacy handling:
  https://learn.microsoft.com/en-us/windows/apps/develop/camera/camera-privacy-setting
- Windows access checks:
  https://learn.microsoft.com/en-us/windows/win32/secauthz/interaction-between-threads-and-securable-objects
- Windows UAC:
  https://learn.microsoft.com/en-us/windows/security/application-security/application-control/user-account-control/how-it-works
- Windows DPAPI:
  https://learn.microsoft.com/en-us/windows/win32/api/dpapi/nf-dpapi-cryptprotectdata
- Windows CNG key storage:
  https://learn.microsoft.com/en-us/windows/win32/seccng/key-storage-and-retrieval
- Windows high-resolution monotonic time:
  https://learn.microsoft.com/en-us/windows/win32/sysinfo/acquiring-high-resolution-time-stamps
- Apple privacy guidance:
  https://developer.apple.com/design/human-interface-guidelines/privacy/
- Apple sandboxed file access:
  https://developer.apple.com/documentation/security/accessing-files-from-the-macos-app-sandbox
- XDG portal overview:
  https://flatpak.github.io/xdg-desktop-portal/
- XDG Permission Store:
  https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.impl.portal.PermissionStore.html
- XDG ScreenCast permission lifetimes:
  https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.ScreenCast.html
- Polkit authority:
  https://polkit.pages.freedesktop.org/polkit/PolkitAuthority.html
- Polkit process identity:
  https://polkit.pages.freedesktop.org/polkit/PolkitUnixProcess.html
- W3C Permissions:
  https://www.w3.org/TR/permissions/
- Chrome one-time permissions:
  https://developer.chrome.com/blog/one-time-permissions
- Chrome permission request chip:
  https://developer.chrome.com/blog/permissions-chip
- Android permission overview:
  https://developer.android.com/guide/topics/permissions/overview
- Android runtime and one-time permissions:
  https://developer.android.com/training/permissions/requesting

---

## Validation Result

- The record follows the canonical framework sections and Permission Authority
  research profile.
- Findings are mapped to existing capability ownership, contracts, acceptance
  cases, Local First, privacy, security, explainability, Windows, licensing,
  maintenance, migration, and Complexity Budget requirements.
- Candidate facts, architectural inferences, and unresolved questions are
  distinguished.
- Current open-source approaches and desktop permission models are compared.
- No technology, library, policy language, proof format, database, process
  boundary, or implementation was selected.
- No runtime code, architecture ownership, communication path, contract meaning,
  or Open Source Registry approval changed.
