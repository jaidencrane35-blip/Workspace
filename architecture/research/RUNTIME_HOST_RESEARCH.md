# Runtime Host Capability Research

Research ID: RH-001

Status: Research complete; architecture review and candidate validation pending

Evidence review date: 2026-08-01

This record applies the canonical template in
`architecture/12_Capability_Technology_Research_Framework.md`. It is technology
research only. It does not select, recommend, approve, conditionally approve,
or reject a technology, change capability ownership, activate Extension Host,
or authorize implementation.

---

## A. Identity and Scope

- Research ID: `RH-001`
- Capability: Runtime Host
- Technology category or question: modern architectures for a Windows-first,
  local-first desktop runtime host
- Research status: Research complete; decision pending
- Research owner: Workspace engineering
- Date opened: 2026-08-01
- Last reviewed: 2026-08-01
- Catalogue entry: `RH-001` in `architecture/03_Research_Catalogue.md`
- Related contracts: `HST-REQ-001`, `HST-REQ-002`, `HST-CMD-001` through
  `HST-CMD-004`, and `HST-EVT-001` through `HST-EVT-002`
- Related interactions: `IC-001` through `IC-005`; `IC-001` and `IC-002`
  apply to Extension Host only after separate activation
- Required acceptance cases: Contract Specification invariants; offline core
  scenarios 1 and 8; lifecycle, event loss/duplication/order, timeout,
  cancellation, compatibility, recovery, privacy, and explainability cases
  where applicable
- Relevant ADRs: ADR-0001 through ADR-0008

In scope:

- application and capability lifecycle
- dependency composition and capability registration
- startup, readiness, degradation, pause, and shutdown sequencing
- in-process task supervision and out-of-process worker supervision
- fault isolation, bounded restart, crash recovery, and stale-health handling
- host-owned configuration, local service orchestration, and IPC coordination
- resource and background-task ownership
- diagnostics, structured logging, and optional local instrumentation
- first-party capability loading boundaries and the boundary with dormant
  Extension Host
- offline operation, packaging, update, recovery, and Windows lifecycle
- portability through replaceable OS adapters without weakening Windows

Out of scope:

- selecting or implementing a runtime, framework, library, protocol, installer,
  updater, logging stack, or process topology
- changing Runtime Host ownership or any capability contract
- product permission decisions, user consent, domain orchestration, UI,
  Memory content, sensing, action execution, or model inference
- designing or activating third-party extensions
- resolving the user-journey architecture gaps through technology choice
- approving or rejecting any named comparator

## B. Capability Objective

Runtime Host must boot Workspace locally, compose and register capabilities,
route lifecycle and status contracts, publish connectivity presence, keep
domain-free availability state, supervise owned runtime resources, and perform
bounded ordered shutdown. Core hosting must work without internet access.

Runtime Host must not become:

- Permission Authority or a source of product-level authorization
- Companion Orchestration or a domain workflow engine
- Experience or the owner of user-facing interaction
- Memory or a durable store for user content
- Context Sensing, Action, or Intelligence
- Extension Host or an implicit third-party plugin authority

The technology boundary must therefore support Workspace-owned lifecycle policy
without absorbing domain behavior.

## C. Research Questions

### Findings

1. **What is the stable lifecycle model?** Mature hosts distinguish at least
   registered, starting, ready, degraded, unavailable, stopping, and stopped.
   Process existence is not readiness. A capability can be alive but unable to
   serve its contract, and the host itself can remain live while Workspace is
   degraded.
2. **How should dependencies compose?** An explicit, validated dependency graph
   supports topological startup and reverse-order shutdown. Cycles, missing
   registrations, duplicate capability identities, and incompatible contract
   versions should fail registration or place the affected branch in a safe
   unavailable state, not be resolved by service-locator discovery.
3. **What should supervision mean?** Supervision is policy over owned work:
   observe termination, classify it, apply bounded backoff and restart budgets,
   and escalate crash loops. Erlang/OTP demonstrates restart strategies and
   maximum restart intensity; this is a mature pattern, not a requirement to
   adopt its runtime.
4. **When is restart safe?** Only when the capability declares recoverability
   and can reconcile owner-authoritative operations. Blind restart can
   duplicate effects, hide indeterminate outcomes, or erase evidence needed by
   Permission Authority, Memory, or Action.
5. **Which isolation model is viable?** Single-process modules minimize IPC and
   packaging cost but share crashes, memory corruption, allocator pressure, and
   privileges. Worker processes add serialization, authentication, deployment,
   update, and shutdown cost but create stronger crash and resource boundaries.
   Isolation should follow risk and recovery class, not one topology for every
   capability.
   On Windows, child processes are independent: parent exit does not terminate
   them, and dropping a Rust `Child` handle does not kill the child. A managed
   worker therefore needs explicit graceful control plus retained process
   handles and hard containment. A Job Object with kill-on-final-handle-close
   can prevent orphans, but that path is forced termination, not graceful
   shutdown or a security sandbox.
6. **How should shutdown work?** A two-phase model is mature: stop accepting new
   work, signal cancellation, drain or settle owned work to a deadline, then
   force termination only where policy permits. Windows can force session
   termination, so correctness must rely on frequent durable commit points and
   restart reconciliation rather than an unlimited exit hook.
   Windows has no universal desktop equivalent of POSIX `SIGTERM`; GUI,
   console, service, and detached workers receive different controls. The
   application-level shutdown request and acknowledgement must therefore be
   independent of the OS-specific hard-stop mechanism.
7. **How should health be represented?** Separate host liveness, lifecycle
   readiness, capability availability, and aggregate degradation. Health events
   contain capability identity, state, freshness, and safe reason codes only;
   domain payloads remain forbidden.
8. **How should configuration work?** Host configuration needs a typed,
   versioned schema; deterministic precedence; validation before activation;
   atomic replacement; last-known-good recovery; migration and rollback; and
   provenance for effective values. General configuration libraries parse and
   layer sources but do not supply Workspace migration or ownership policy.
9. **How should local IPC work?** Any worker protocol needs explicit peer
   identity, a version handshake, bounded framing, request identity,
   cancellation, backpressure, heartbeat/freshness semantics, and restart
   epochs. On Windows, named pipes can use DACL access checks; default security
   descriptors must not be assumed sufficient.
10. **How should background work be managed?** No detached, untracked work.
    Every task needs an owning capability, parent lifetime, cancellation token,
    completion handle, resource budget, and shutdown class. Tokio's
    `CancellationToken` plus `TaskTracker` is representative of the structured
    cancellation/drain pattern. Plain task handles can detach, and already
    started blocking work may not be abortable; future candidates must prove
    how blocking work is bounded and prevented from outliving shutdown.
11. **How should diagnostics work?** Structured events and spans allow local
    correlation across asynchronous work. Instrumentation and export are
    separate concerns: local diagnostics must work with no collector and no
    network; remote export, if ever present, is optional, disabled by default,
    minimized, and separately governed.
12. **How should plugin loading be bounded?** Compile-time or trusted
    first-party modules are composition choices. Native DLL loading executes
    initialization code in the host and is not a security or crash boundary.
    Processes and WebAssembly can provide stronger boundaries with material
    complexity. Third-party extension discovery and privilege attenuation
    remain dormant Extension Host concerns.
13. **How should Windows-first portability work?** Keep the lifecycle state
    machine, contracts, and policy portable; place shutdown messages, Job
    Objects, named-pipe security, power notifications, packaging, signing, and
    crash diagnostics behind explicit Windows adapters. Cross-platform support
    must add peer adapters rather than reduce Windows behavior to a lowest
    common denominator.

### Assumptions requiring architecture review

- startup classes for capabilities outside the audited Runtime Host, Permission
  Authority, Workspace Management, and Memory foundation
- which capabilities may use processes and which capability-specific restart
  classes are safe within the accepted descriptor taxonomy
- the authoritative state and recovery rule after forced shutdown
- whether host configuration needs backup beyond last-known-good local state

Capability Architecture v1.2 and Contracts v1.1 resolve the foundational
dependency classes, start Experience's minimal status/direct-administration
surface beside Host, permit direct Experience-to-Host shutdown, and require
declared restart/reconciliation semantics.

### Unacceptable outcomes for future candidates

- internet required for startup, local recovery, or core status
- hidden cloud fallback or mandatory telemetry export
- host ownership of domain data, permission decisions, or orchestration
- unbounded restart loops or detached background work
- health records containing user content, secrets, raw observations, prompts,
  targets, Memory items, or extension partition data
- shutdown that closes Experience before consequential owners report their
  final known state
- IPC with no peer restriction, versioning, bounds, or backpressure
- in-process third-party native code described as isolated
- updates that cannot coordinate worker termination or preserve rollback
- parent-exit assumptions that permit orphan workers
- lifecycle designs that treat hard process termination as graceful completion
- engineering documentation included in runtime packages

## D. Required Functional Capabilities

| ID | Requirement | Source | Class | Pattern-level support |
|---|---|---|---|---|
| RH-F-01 | Register one descriptor per active capability and reject duplicates/incompatible descriptors | Runtime Host architecture | Mandatory | Explicit registry/graph: native or adapter-required |
| RH-F-02 | Execute dependency-aware startup and publish readiness | `HST-CMD-001`, `IC-001` | Mandatory | Generic hosts and supervisor patterns support hooks; Workspace graph policy required |
| RH-F-03 | Query aggregate host status and per-capability availability | `HST-REQ-001/002`, `IC-003/004` | Mandatory | Health registries support mechanics; domain-free schema required |
| RH-F-04 | Receive capability health and detect stale/missing reports | `HST-EVT-001`, `IC-002` | Mandatory | Heartbeat/event patterns available; freshness policy required |
| RH-F-05 | Publish offline/online-optional presence to Intelligence only | `HST-EVT-002`, `IC-005` | Mandatory | OS/network observation adapter required; event grants no provider authority |
| RH-F-06 | Pause or stop a capability according to declared lifecycle support | `HST-CMD-002/003` | Mandatory | Cancellation/process-control mechanics exist; capability semantics required |
| RH-F-07 | Coordinate ordered application shutdown | `HST-CMD-004` | Mandatory | Lifecycle hooks and structured cancellation support mechanics |
| RH-F-08 | Supervise owned tasks and isolated workers | Architecture lifecycle | Mandatory where present | Task trackers, process handles, and supervisor policies are mature categories |
| RH-F-09 | Apply bounded restart/backoff and expose crash-loop degradation | Failure behavior | Mandatory | Supervisor-tree pattern supports mechanics; safe-restart declaration required |
| RH-F-10 | Load, validate, migrate, and recover host configuration | Runtime Host ownership | Mandatory | Typed configuration and atomic persistence require composition |
| RH-F-11 | Coordinate local IPC with identity, compatibility, bounds, and cancellation | Contract invariants | Mandatory for workers | Named-pipe/socket/RPC categories require Workspace envelope adapters |
| RH-F-12 | Own and release handles, tasks, processes, IPC endpoints, timers, and diagnostic sinks | Lifecycle acceptance | Mandatory | Resource scopes/RAII help; explicit ownership inventory still required |
| RH-F-13 | Produce bounded local lifecycle diagnostics | Explainability/ADR-0006 | Mandatory | Structured logging/tracing categories support mechanics |
| RH-F-14 | Coordinate updater/installer exit without treating update as core-online functionality | Roadmap output | Mandatory before distribution | Framework/OS mechanisms exist; lifecycle integration unknown |
| RH-F-15 | Load dormant Extension Host only after separate activation | Architecture | Conditional | Registration gate required; this research cannot activate it |

No pattern provides all rows unchanged. The stable comparison unit is a
composition of host policy, runtime mechanics, OS adapters, and optional worker
transport.

## E. Required Non-Functional Capabilities

- **Reliability:** deterministic registration, idempotent lifecycle commands,
  bounded retries, crash-loop suppression, and restart reconciliation.
- **Availability:** host and Experience should expose honest degraded state when
  non-essential capabilities fail; Permission Authority remains fail-closed.
- **Resource use:** measure cold/warm startup, steady memory, process count,
  handles, threads, wakeups, CPU, disk writes, battery impact, and shutdown.
- **Responsiveness:** status must remain responsive during dependency failure;
  shutdown and cancellation need per-class deadlines.
- **Observability:** domain-free structured events with monotonic timestamps,
  correlation, bounded transient buffering, redaction, and loss accounting;
  retained subsets remain conditional on architecture approval.
- **Testability:** deterministic fake clock/process/network adapters, fault
  injection, crash-loop tests, forced termination, and packaging tests.
- **Portability:** portable host state machine and replaceable Windows adapters;
  no platform claim without an exact tested environment.
- **Replaceability:** runtime, IPC codec, diagnostic sink, and configuration
  parser remain behind Workspace-owned interfaces.
- **Durability:** under accepted architecture, only host configuration persists.
  Registry, health, and diagnostics remain transient, domain-free, bounded, and
  erasable unless a later architecture decision explicitly approves a retained
  subset and its access/retention policy.

Thresholds remain unknown until representative hardware classes and capability
process boundaries are approved.

## F. Local First Requirements

Core host behavior must pass with network interfaces unavailable:

1. start from installed local artifacts
2. validate local configuration and recover last-known-good state
3. start required local capabilities in dependency order
4. publish offline mode without waiting for DNS or remote endpoints
5. present ready or degraded local status
6. shut down and recover from prior interruption locally
7. produce transient local diagnostics without mandatory export

Optional update checks, remote diagnostics, remote Intelligence, and cloud
services must not block startup or alter local authority. Offline installation
and recovery media, signature verification, update staging, and rollback must
be evaluated separately from online convenience. A product is not Local First
if it runs offline only after network installation, activation, authorization,
model acquisition, or recovery.

## G. Privacy Requirements

Runtime Host may receive:

- capability identity and declared lifecycle metadata
- ready/degraded/unavailable state and safe reason codes
- process/task/resource counters
- host configuration
- connectivity presence, not network content or history
- correlation identifiers without user content

It must not retain domain payloads. Logs, traces, crash dumps, worker stdout,
IPC errors, command lines, environment variables, and file paths are leakage
paths. Required controls include:

- allow-listed fields and reason codes rather than arbitrary error serialization
- secret and content redaction before any sink
- transient bounded buffering and user-visible clearing
- any retained diagnostic subset is conditional on later architecture approval
  of its exact fields, access, retention, deletion, and crash-dump policy
- no remote export by default
- diagnostic levels that can disable non-essential collection
- crash-dump policy that recognizes full dumps can contain sensitive memory
- no use of diagnostics as training data
- no alternate Memory store in logs, traces, caches, or recovery files

## H. Security Considerations

### Trust boundaries

- The host is privileged over lifecycle, not over product permissions.
- An in-process capability shares the host's address space and effective token.
- A worker process adds a crash boundary; privilege isolation exists only when
  the OS token, ACLs, handles, namespace, and IPC are constrained.
- Windows Job Objects manage and account for process groups but are not an
  authorization sandbox.
- AppContainer can restrict resource access but introduces packaging,
  capability, IPC, and compatibility complexity that needs testing.
- WebAssembly provides a language/runtime sandbox only to the extent that host
  imports, resource limits, interruption, and implementation security are
  correctly configured.

### Mandatory controls for future evaluation

- standard-user operation and explicit elevation boundaries
- least-privilege process tokens and inherited-handle review
- absolute-path executable resolution and signed-artifact verification
- restrictive IPC ACLs, local-only endpoints, peer identity, and anti-squatting
- bounded messages, queues, memory, CPU, restart rate, and child process trees
- authenticated protocol handshake and semantic version compatibility
- supply-chain inventory, advisories, reproducible packaging, and rollback
- secret-free command lines and environment
- fail-closed behavior when Permission Authority is unavailable

## I. Performance Considerations

Future prototypes must measure on declared Windows x64 and Arm64 hardware:

- cold and warm launch to host event loop, Permission Authority ready,
  Experience visible, required local capability ready, and aggregate ready
- per-capability registration and health propagation latency
- steady and peak CPU, private working set, handles, threads, process count,
  disk writes, wakeups, and battery impact
- in-process call versus worker IPC latency and throughput for representative
  small control messages
- bounded-queue behavior under a stalled consumer
- crash detection, restart delay, and crash-loop escalation
- suspend/resume reconciliation
- cooperative and forced shutdown duration
- diagnostic overhead at default and diagnostic levels
- offline startup with network calls blocked and DNS unavailable

Average startup is insufficient; percentiles, worst-case deadlines, and
resource exhaustion behavior are required.

## J. Explainability Considerations

Runtime Host must enable Experience to truthfully present:

- starting, ready, degraded, offline, and shutting-down states
- which capability is unavailable
- a safe reason class such as dependency unavailable, incompatible, crashed,
  restart suppressed, configuration invalid, or shutdown timed out
- whether recovery is automatic, waiting, unavailable, or requires user action
- last health freshness and uncertainty
- no claim that a process restart means a domain operation succeeded or failed

The explanation record must remain content-free. Host status does not expose
another capability's domain state or permission evidence.

## K. Licensing Evaluation

Representative comparators are evidence sources, not approved dependencies:

| Comparator | Evaluated revision | Licence snapshot | Legal note |
|---|---|---|---|
| Tauri | 2.11.5 | MIT or Apache-2.0 | Exact plugin, WebView2, installer, updater, build-tool, and transitive obligations remain separate |
| Electron | 43.2.0 stable | MIT | Bundled Chromium/Node and codecs carry additional notices and update obligations |
| .NET Generic Host | .NET 10.0.10 | MIT for relevant open-source runtime components | Runtime redistribution and transitive notices require exact-scope review |
| Tokio | 1.53.1 | MIT | Exact feature and transitive dependency review required |
| tracing ecosystem | current repository evidence; tracing-appender 0.2.5 release observed | MIT | Sink and exporter dependencies require separate review |
| Erlang/OTP supervision | 29.0.4 | Apache-2.0 | Used here as a pattern comparator, not a proposed runtime |
| Wasmtime | 47.0.2 | Apache-2.0 WITH LLVM-exception | WASI/component dependencies and notices require exact review |
| libloading | 0.8 line | ISC | Native libraries loaded through it retain their own licences |
| OpenTelemetry Rust SDK | 0.32.1 | Apache-2.0 | Exporters, protocol generators, and collector deployments add obligations |
| Windows APIs, MSIX, WebView2, WER, and ETW | platform services/docs | Microsoft terms | Distribution and build-tool terms are not open-source dependency approval |

No licence is approved. Exact selected versions, features, transitive trees,
notices, patents, trademarks, commercial distribution, code-signing services,
and build-tool terms require legal review before adoption.

## L. Maintenance Evaluation

- Tauri 2.11.5 was released 2026-07-01 and exposes current lifecycle, plugin,
  sidecar, updater, and Windows support documentation.
- Electron 43.2.0 was stable on 2026-07-21; its published schedule shows a
  rapid major cadence and finite support windows, creating recurring Chromium
  security-update and migration obligations.
- .NET 10.0.10 is an active LTS release supported through 2028-11-14, subject
  to current-patch requirements.
- Tokio 1.53.1 was released 2026-07-20, including a Windows signal-handler fix.
- Erlang/OTP 29.0.4 documents long-lived supervision principles.
- Wasmtime 47.0.2 was released 2026-07-21; rapid major evolution makes API,
  security, MSRV, and component-model migration evidence mandatory.
- OpenTelemetry Rust was active in 2026, but signal stability and exporter
  dependency churn need exact-version review.

Recency is not fitness. Future evaluation must inspect support policy,
advisories, Windows regressions, maintainer continuity, migration notes,
dependency freshness, and replacement cost.

## M. Community Maturity Evaluation

The comparator set spans mature but different communities:

- Windows lifecycle/process/diagnostic primitives are durable platform APIs.
- OTP supervision and .NET Generic Host provide long-lived operational models.
- Electron and Tauri provide production desktop event-loop and packaging models
  with different process/resource footprints.
- Tokio and tracing are established Rust runtime/diagnostic ecosystems.
- Wasmtime and the component model have strong governance and active releases
  but evolve more rapidly than basic process boundaries.
- OpenTelemetry provides interoperable semantics, but a local desktop host has
  different privacy and operational needs than a server fleet.

Production use and popularity are context only. Reproducible Workspace evidence
remains mandatory.

## N. Platform Compatibility

### Windows-first requirements

- handle `WM_QUERYENDSESSION` and `WM_ENDSESSION` semantics or prove the
  framework maps them correctly; ordinary framework quit hooks may not run
  during Windows shutdown or logout
- register for suspend/resume or power-setting notifications where behavior
  depends on them; callbacks must be fast and recovery must tolerate missed or
  delayed notifications
- use Job Objects or equivalent owned-handle tracking for worker trees when
  process grouping is required; test final-handle kill behavior, nested jobs,
  breakaway policy, and launch inside an existing restrictive job
- coordinate Restart Manager/updater shutdown and prevent orphan workers
- test single-instance activation, deep links, secondary-instance handoff, and
  stale primary-instance recovery; explicitly define per-user, per-session,
  per-profile, or machine-wide scope under Fast User Switching and Remote
  Desktop
- use standard-user writable locations and preserve ACLs
- test packaged and unpackaged behavior, signing, installer upgrade, repair,
  rollback, offline deployment, x64, and Arm64
- test WebView/runtime prerequisites when a desktop framework depends on them
- validate ETW/WER/local-dump behavior and privacy under real crashes
- distinguish a user-session host from a Windows service: a service runs in
  non-interactive Session 0, introduces installation/elevation and multi-user
  routing obligations, and cannot directly own normal Experience surfaces
- prove offline repair/update dependencies, package identity continuity,
  signing certificate rotation, and update-key loss/compromise recovery

### Portability

Portable concepts: registry descriptors, dependency graph, lifecycle states,
restart policy, task ownership, health schema, diagnostics schema, and IPC
envelope.

OS-specific adapters: session shutdown, signals, power events, process groups,
sandboxing, single-instance locks, local IPC security, paths, credential
stores, crash capture, packaging, signing, and updates.

## O. Integration Complexity

| Pattern | Added boundaries | Principal complexity |
|---|---|---|
| Explicit in-process modular host | one process, internal registry | shared failure/privilege domain; task and panic containment |
| Desktop framework event loop plus Workspace lifecycle coordinator | framework event loop and plugin hooks | mapping OS exits, updater, windows, and lifecycle without framework ownership drift |
| Generic-host/service-style composition | DI/config/logging/hosted-service abstraction | adapting server assumptions to user-session desktop and existing language/runtime |
| Supervisor-tree-inspired host | restart tree and escalation policy | safe recovery declarations and domain reconciliation |
| Hybrid host with local workers | process manager, IPC, packaging, protocol | identity, serialization, versioning, backpressure, orphan cleanup, update |
| OS-assisted constrained workers | Job Objects, tokens/AppContainer, ACLs | Windows-specific security and compatibility engineering |
| Sandboxed component workers | Wasm runtime, WIT/import boundary | runtime size, host-call design, interruption, resource limits, ecosystem churn |

Additional processes and runtimes spend more Complexity Budget than adapter
line count suggests. A hybrid topology is not justified until measured risk
classes require its boundaries.

## P. Extensibility

A viable host should allow:

- new first-party capability descriptors without changing core policy
- versioned lifecycle and health contracts
- replacement of task runtime, process adapter, IPC codec, configuration
  provider, and diagnostic sink
- Windows-specific behavior behind narrow interfaces
- capability-specific restart and shutdown declarations

It must not:

- provide arbitrary service-location across capability contracts
- create a generic plugin marketplace
- let first-party registration bypass the Interaction Matrix
- pre-activate Extension Host
- make host configuration a general domain settings store

## Q. Failure Modes

| Failure | Owner-authoritative host response | Required recovery/explanation |
|---|---|---|
| Invalid/duplicate descriptor or cycle | reject affected registration | identify safe registration reason; do not infer domain state |
| Required capability fails startup | mark unavailable; aggregate startup degraded or failed by approved class | keep status path available where architecture permits |
| Optional capability fails startup | continue degraded | identify unavailable capability and user impact class |
| Health event missing/stale | mark unknown/unavailable after threshold | expose freshness and avoid fabricated health |
| In-process panic/fatal crash | host may terminate; no claimed isolation | recover from durable owner state on next launch |
| Worker exits | capture exit class; bounded restart only if declared safe | preserve restart epoch and reconcile operations |
| Host exits while workers remain | Job/owned-handle containment applies bounded hard stop after graceful path is unavailable | no orphan process tree; forced stop is never reported as graceful domain completion |
| Crash loop | stop restart after budget | show restart suppressed and user action |
| IPC disconnect or incompatible version | reject new work; availability unknown/unavailable | reconnect only after authenticated handshake |
| Queue/resource exhaustion | apply bounds/backpressure; degrade or reject | no unbounded memory growth or hidden drops |
| Configuration parse/migration failure | do not activate invalid config | recover last-known-good or enter explicit recovery |
| Disk full/read-only | preserve current safe state where possible | report persistence limitation; no false flush claim |
| Permission Authority unavailable | no new meaningful effects | operation-control behavior remains separately bounded |
| Network unavailable | publish offline; no restart storm | core local host continues |
| Suspend/resume | quiesce where notified; reconcile clocks, IPC, and health on resume | stale state becomes unknown until refreshed |
| Forced OS shutdown | best-effort bounded checkpoint; OS may terminate | next launch reconciles interruption honestly |
| Update/installer failure | retain current or rollback-safe version | no mixed binary/protocol set or orphan worker |
| Diagnostic sink failure | drop/degrade with counters; never block core indefinitely | expose local diagnostic degradation safely |
| Host configuration corruption | preserve evidence and use verified recovery path | do not silently reset security-relevant host state |

Every row needs reproducible injection tests before selection.

## R. Migration Risk

Host migration can affect:

- configuration schema, source precedence, paths, and ACLs
- capability descriptor and lifecycle contract versions
- process topology and worker protocol
- single-instance identity and activation handoff
- packaging identity, signing certificate, updater metadata, and rollback
- diagnostic schemas and retention
- restart reconciliation and operation epochs

Required protections are versioned config with reversible migrations,
last-known-good snapshots, protocol negotiation, mixed-version rejection or
explicit bounded coexistence, process-drain coordination, exportable
domain-free host records, and package rollback tests. Replacing a framework is
expensive if lifecycle policy is embedded in framework plugins or UI code.

## S. Reasons to Build Internally

Workspace-owned code is justified for:

- the capability descriptor and dependency model
- essential/degradable classification and startup policy
- safe-restart declaration, crash-loop escalation, and recovery semantics
- domain-free health aggregation and explanation mapping
- lifecycle command mapping to `HST-*` and `IC-001` through `IC-005`
- resource ownership inventory
- privacy-safe diagnostic schema and retention policy
- architecture gates around Extension Host and documentation packaging

These are unique because they encode Workspace authority and accepted
contracts. They should remain bounded and use commodity mechanisms underneath.

## T. Reasons to Integrate Externally

Credible commodity integration categories include:

- desktop application event loop and packaging framework
- asynchronous task runtime and structured cancellation
- process creation, observation, and OS-native job control bindings
- dependency-injection or composition primitives
- typed configuration parsing
- structured logging and tracing
- local IPC transport, framing, and serialization
- crash diagnostics and OS event integration
- updater/installer primitives
- optional sandbox runtime for a separately justified boundary

These functions contain mature platform and correctness work. Integration is
acceptable only through replaceable adapters that preserve Workspace policy.

## U. Candidate Comparison

The categories below remain open; no disposition is made.

| Category | Strength | Material trade-off | Evidence still required |
|---|---|---|---|
| In-process modular host | lowest IPC/packaging cost; direct types | shared crash, memory, resource, and privilege domain | panic/fatal failure, task leaks, startup/shutdown bounds |
| Desktop framework lifecycle host | native window/event/update integration | framework quit semantics may differ by OS; plugin ownership pressure | Windows shutdown, updater, sidecar, single-instance tests |
| Generic host/hosted-service pattern | mature DI/config/logging/lifetime composition | extra runtime or conceptual mismatch with desktop | startup ordering, UI availability, footprint, packaging |
| Supervisor-tree pattern | explicit restart classes and intensity | does not solve effect reconciliation by itself | Workspace-safe restart taxonomy and crash-loop tests |
| Hybrid in-process plus workers | risk-based isolation and native integration | IPC, protocol, packaging, update, resource complexity | per-capability risk model and worker lifecycle evidence |
| Windows Job Object-managed workers | process-tree control and accounting | Windows-specific and not a security sandbox | nested-job compatibility, breakaway, forced cleanup |
| AppContainer/low-privilege workers | stronger Windows resource isolation | capability/IPC/packaging compatibility burden | standard-user, packaged/unpackaged, required OS API tests |
| WebAssembly component workers | typed imports, interruption, resource limits | host-call design, runtime footprint, evolving ecosystem | Windows performance, sandbox threat model, migration |
| Native dynamic libraries | small deployment boundary and native performance | arbitrary initialization, ABI risk, no crash/security isolation | ABI/version/unload safety; only trusted first-party scope |
| Named-pipe/local RPC workers | Windows ACL integration and local transport | peer identity, framing, cancellation, backpressure still external | restrictive DACL, anti-squatting, load and fault tests |
| Structured local tracing with optional export | correlation and sink replaceability | privacy leakage and retention risk | redaction, bounded storage, crash behavior, export-off tests |

Representative named projects establish that these mechanisms exist; they do
not establish Workspace fitness.

## V. Required Acceptance Criteria

Before any candidate can be adopted:

1. `HST-*` contracts and `IC-001` through `IC-005` map without ownership change.
2. The approved essential/degradable dependency graph is explicit and acyclic.
3. Startup, readiness, degraded operation, pause, and reverse shutdown are
   deterministic and tested.
4. Experience can present honest host status during applicable degradation and
   ordered shutdown.
5. Every background task, process, endpoint, handle, and sink has one owner and
   a bounded release path.
6. Restarts are capability-declared, bounded, backoff-controlled, and cannot
   fabricate domain outcomes or duplicate meaningful effects.
7. Health is domain-free, freshness-aware, queryable, and loss tolerant.
8. Configuration is typed, versioned, atomic, recoverable, and rollback-tested.
9. Worker IPC, if any, restricts peers and implements versioning, framing,
   bounds, cancellation, backpressure, and restart epochs.
10. Core host startup, status, recovery, and shutdown pass with all network
    access blocked.
11. Windows session shutdown, power events, single-instance activation, worker
    cleanup, installer/update, signing, standard-user, x64, and Arm64 behavior
    have reproducible evidence.
12. Parent exit, dropped process handles, nested/existing jobs, breakaway,
    detached workers, blocking tasks, and hard-stop deadlines cannot leave
    unowned work or fabricate graceful completion.
13. Diagnostics remain local by default, content-minimized, bounded, erasable,
    and non-blocking; crash-dump privacy is explicit.
14. Disabled external diagnostics produce no DNS, listener, retry queue,
    client identifier, or network attempt.
15. Engineering documentation is absent from runtime packages.
16. Extension Host remains absent unless separately activated.
17. Exact licences, transitive dependencies, advisories, maintenance policy,
    migration, replacement, and Complexity Budget are accepted by named
    authorities.
18. Required Contract Specification acceptance cases pass under crash,
    duplicate, missing, out-of-order, timeout, incompatibility, disk failure,
    forced termination, and update failure.

### Remaining unknowns

- essential, independently available, and degradable capability classes
- Experience startup/shutdown placement and direct Host invocation contracts
- capability-specific process and restart-risk classes
- host configuration schema, backup, restore, and reset authority
- health heartbeat, staleness, startup, drain, and force-termination thresholds
- operation reconciliation after host or worker crash
- supported Windows versions, hardware classes, x64/Arm64 baseline, and
  packaging channels
- packaged versus unpackaged identity and AppContainer feasibility
- update source, offline update, certificate rotation, rollback, and
  mixed-version rules
- single-instance/deep-link trust and stale-instance recovery
- single-instance scope across users, sessions, profiles, Fast User Switching,
  and Remote Desktop
- sleep, Modern Standby, clock discontinuity, and long-offline resume behavior
- local IPC transport and protocol schema
- diagnostic retention, user controls, crash-dump attacker model, and whether
  any optional export will exist
- resource budgets and representative later-capability workloads
- cross-platform target order and minimum parity
- whether any host-owned work must survive logout or run before user sign-in
- behavior when the host is launched inside another Job Object and whether any
  worker may legitimately break away
- offline repair sources, package-identity continuity, certificate rollover,
  update-key escrow, and recovery after key loss or compromise

## W. Decision and Review

- Decision: No technology selected, recommended, approved, conditionally
  approved, or rejected. Research findings and comparator categories recorded.
- Selected scope: None.
- Decision rationale: Architecture assumptions and reproducible Windows,
  offline, failure, recovery, packaging, privacy, and contract evidence remain
  unresolved.
- Rejected candidates and reasons: None; project approval/rejection was
  explicitly out of scope.
- Conditions or controls: All criteria in section V are mandatory gates unless
  a named architecture authority explicitly approves a bounded exception.
- Remaining unknowns: Section V.
- Required ADR: None at research completion. A future process/isolation,
  lifecycle, packaging, or update decision may require an ADR if durable.
- Open Source Registry action: None.
- Engineering Ledger action: Record completion of `RH-001`.
- Review date: 2027-02-01, or earlier on architecture correction, process-model
  decision, framework/runtime major release, advisory, Windows support change,
  packaging decision, or new reproducible evidence.
- Approver: Pending.

---

## Architectural Trade-offs

1. **Simplicity versus isolation:** one process minimizes moving parts; workers
   narrow some failure/resource boundaries but add protocol and deployment risk.
2. **Automatic recovery versus truth:** restart improves availability only when
   owner state can be reconciled; otherwise it can hide unknown outcomes.
3. **Framework convenience versus ownership:** framework hooks reduce platform
   work but must not become the source of Workspace lifecycle policy.
4. **Static composition versus runtime flexibility:** static registration gives
   compile-time assurance; dynamic loading increases replacement flexibility
   while expanding ABI, trust, and failure surfaces.
5. **Portable abstraction versus Windows quality:** portable policy is useful;
   hiding Windows lifecycle semantics behind a lowest-common-denominator API is
   not.
6. **Diagnostic depth versus privacy:** richer traces improve recovery evidence
   but can capture domain content and become an alternate Memory store.
7. **Graceful drain versus user/OS deadlines:** drain protects work, but the host
   must remain bounded and tolerate forced termination.
8. **Typed RPC versus integration cost:** generated protocols improve
   compatibility discipline but do not solve peer trust, backpressure, or
   semantic ownership.
9. **Aggressive health probing versus background cost:** frequent probes reduce
   stale windows but consume CPU, battery, handles, and disk/network activity.
10. **Updater automation versus offline control:** automated updates improve
    patch velocity but add network, signing, restart, rollback, and worker
    coordination obligations.

## Candidate Solution Categories

- Workspace-owned lifecycle coordinator over an in-process modular host
- desktop framework event loop with explicit Workspace host policy
- generic-host/hosted-service composition adapted to a user-session desktop app
- supervisor-tree-inspired task and capability policy
- hybrid modular host with selected isolated local workers
- Windows Job Object-managed worker topology
- constrained Windows workers using AppContainer or lower-privilege tokens
- sandboxed WebAssembly component boundary for separately justified use
- typed local RPC over named pipes or another OS-local transport
- structured local logging/tracing with optional, separately governed export
- versioned typed configuration with atomic local persistence

## Evidence Register

Primary and official sources accessed 2026-08-01:

1. Tauri `RunEvent` lifecycle:
   https://docs.rs/tauri/latest/tauri/enum.RunEvent.html
2. Tauri plugin lifecycle:
   https://v2.tauri.app/develop/plugins/
3. Tauri sidecars:
   https://v2.tauri.app/develop/sidecar/
4. Tauri updater and Windows exit behavior:
   https://v2.tauri.app/plugin/updater/
5. Tauri releases, including 2.11.5:
   https://github.com/tauri-apps/tauri/releases
6. Tauri licences:
   https://github.com/tauri-apps/tauri
7. Electron process model:
   https://github.com/electron/electron/blob/main/docs/tutorial/process-model.md
8. Electron application lifecycle and Windows shutdown caveat:
   https://electronjs.org/docs/latest/api/app
9. Electron release schedule and 43.2.0:
   https://releases.electronjs.org/
10. .NET Generic Host:
    https://learn.microsoft.com/dotnet/core/extensions/generic-host
11. .NET health models:
    https://learn.microsoft.com/dotnet/core/diagnostics/diagnostic-health-checks
12. .NET support policy:
    https://dotnet.microsoft.com/platform/support/policy
13. Erlang/OTP supervision principles:
    https://www.erlang.org/docs/29/system/sup_princ.html
14. Erlang/OTP 29.0.4 and licence:
    https://www.erlang.org/downloads
15. Tokio graceful shutdown:
    https://tokio.rs/tokio/topics/shutdown
16. Tokio releases, including 1.53.1:
    https://github.com/tokio-rs/tokio/releases
17. Rust `tracing` structured diagnostics:
    https://docs.rs/tracing/latest/tracing/
18. Windows Job Objects:
    https://learn.microsoft.com/windows/win32/procthread/job-objects
19. Windows nested jobs:
    https://learn.microsoft.com/windows/win32/procthread/nested-jobs
20. Windows process termination:
    https://learn.microsoft.com/windows/win32/procthread/terminating-a-process
21. Rust child-process ownership:
    https://doc.rust-lang.org/stable/std/process/struct.Child.html
22. Windows application shutdown:
    https://learn.microsoft.com/windows/win32/shutdown/shutting-down
23. Windows Restart Manager application guidance:
    https://learn.microsoft.com/windows/win32/rstmgr/guidelines-for-applications
24. Windows Application Recovery and Restart:
    https://learn.microsoft.com/windows/win32/api/_recovery/
25. Windows suspend/resume notifications:
    https://learn.microsoft.com/windows/win32/api/winuser/nf-winuser-registersuspendresumenotification
26. Windows named-pipe security:
    https://learn.microsoft.com/windows/win32/ipc/named-pipe-security-and-access-rights
27. Windows AppContainer isolation:
    https://learn.microsoft.com/windows/win32/secauthz/appcontainer-isolation
28. Event Tracing for Windows:
    https://learn.microsoft.com/windows/win32/etw/about-event-tracing
29. Windows Error Reporting local dumps:
    https://learn.microsoft.com/windows/win32/wer/collecting-user-mode-dumps
30. Windows app instancing:
    https://learn.microsoft.com/windows/apps/windows-app-sdk/applifecycle/applifecycle-instancing
31. Windows service/session isolation:
    https://learn.microsoft.com/windows/win32/services/service-changes-for-windows-vista
32. Windows packaging-model comparison:
    https://learn.microsoft.com/windows/apps/package-and-deploy/choose-packaging-model
33. MSIX signing:
    https://learn.microsoft.com/windows/msix/package/sign-msix-package-guide
34. Wasmtime interruption:
    https://docs.wasmtime.dev/examples-interrupting-wasm.html
35. Wasmtime resource limiting:
    https://docs.rs/wasmtime/latest/wasmtime/struct.Store.html
36. Wasmtime releases and licence:
    https://github.com/bytecodealliance/wasmtime/releases
37. `libloading` dynamic-library safety:
    https://docs.rs/libloading/latest/libloading/
38. OpenTelemetry Rust SDK and licence:
    https://github.com/open-telemetry/opentelemetry-rust
39. OpenTelemetry SDK exporter configuration:
    https://opentelemetry.io/docs/specs/otel/configuration/sdk-environment-variables/
40. Windows file flush and atomic replacement primitives:
    https://learn.microsoft.com/windows/win32/api/fileapi/nf-fileapi-flushfilebuffers
    and https://learn.microsoft.com/windows/win32/api/winbase/nf-winbase-replacefilea
41. Figment layered configuration and provenance:
    https://docs.rs/figment/latest/figment/

### Evidence limitations

- Documentation and release metadata establish available mechanisms, not
  Workspace acceptance.
- No benchmark, prototype, packaging test, fault-injection test, security
  review, or licence approval was performed.
- Exact platform claims remain unknown until reproduced on declared Windows
  versions and hardware.
- Framework issue reports informed risk discovery but are not treated as proof
  that current releases pass or fail Workspace requirements.

