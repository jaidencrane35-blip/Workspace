# Workspace Capability Contracts v1.0

Status: Active
Authority: Authoritative public contract definitions for Workspace capabilities
Version: 1.0

This document defines communication contracts only. It does not select transports, libraries, process boundaries, persistence engines, or vendors.

Authority order:

1. `00_Workspace_Blueprint.md`
2. Accepted ADRs in `architecture/decisions/`
3. `08_Workspace_Capability_Architecture.md` for capability ownership
4. `09_Capability_Interaction_Matrix.md` for allowed communication and trust boundaries
5. This document for public message and interaction semantics

If this document conflicts with a higher authority, the higher authority wins.

---

## Contract Model

### Message kinds

- **Request** — asks the receiver to evaluate or return information and expects one response.
- **Command** — asks the receiver to attempt a state transition. Acceptance is not completion; consequential commands produce terminal events.
- **Event** — an immutable fact already observed or committed. Events grant no command authority.
- **Response** — the receiver's direct answer to a request or command acceptance attempt.

No message kind implies a technology, transport, thread, process, or network boundary.

Runtime Host lifecycle control applies to every active capability as a common control-plane contract and is not repeated in each capability's domain public interface.

### Common envelope

Every cross-capability message carries:

- `message_id` — unique identity for deduplication and explanation
- `message_kind` — request, command, event, or response
- `contract_id` and `contract_version`
- `timestamp`
- `requester_capability`
- `task_id` — required for Companion-mediated work; absent only for explicit direct administration
- `operation_id` — assigned by the committing capability
- `authorization_id` — required when authorization applies
- `causation_id` — immediate predecessor
- `correlation_id` — groups request/response/event completion
- `extension_id` — required for extension-originated work
- `purpose` — human-understandable reason

Identifiers contain no user content and grant no authority.

### Authorization proof

An authorization proof is opaque outside Permission Authority and contains or binds:

- authorization identity
- requester capability
- subject/user
- permission scope
- purpose
- target
- operation
- optional workspace scope
- optional extension identity
- issue/expiry conditions
- replay/single-use constraints

The committing capability must call `Permission.validate` against the exact point-of-use context. A proof must never appear in an event or explanation payload. Events may carry only `authorization_id`.

### Explainability envelope

Every consequential operation and degraded/failure state exposes:

- `what` — current or completed activity
- `why` — purpose
- `responsible_capability`
- `permission_scope` and `authorization_id`, when applicable
- `target_summary` — minimized, non-secret description
- `status` — requested, waiting, active, completed, denied, cancelled, failed, degraded
- `user_action_available` — cancel, grant, deny, revoke, retry, inspect, or none
- correlation fields

Experience presents this envelope but cannot alter its domain meaning.

### Common response outcomes

- `accepted` — command accepted for processing; not terminal success
- `completed` — request or immediate operation completed
- `denied` — Permission Authority denied or proof validation failed
- `challenge_required` — explicit user decision required
- `invalid_request` — malformed or contract-invalid input
- `unavailable` — receiver or required local prerequisite unavailable
- `conflict` — requested state transition conflicts with current state
- `cancelled` — operation reached a safe cancelled state
- `failed` — receiver attempted but could not complete

### Common error contract

Every error contains:

- `error_code`
- `category` — validation, authorization, unavailable, conflict, timeout, cancellation, dependency, internal
- `safe_message`
- `responsible_capability`
- `retryable`
- `correlation_id`
- `explanation`

Errors never expose secrets, raw sensing data, model prompts/responses, unrelated permissions, or another workspace's data.

Common architectural error codes:

- `CONTRACT_INVALID`
- `CAPABILITY_UNAVAILABLE`
- `DEPENDENCY_UNAVAILABLE`
- `AUTHORIZATION_REQUIRED`
- `AUTHORIZATION_DENIED`
- `AUTHORIZATION_EXPIRED`
- `AUTHORIZATION_REVOKED`
- `AUTHORIZATION_CONTEXT_MISMATCH`
- `AUTHORIZATION_REPLAYED`
- `SCOPE_MISMATCH`
- `OPERATION_CONFLICT`
- `OPERATION_CANCELLED`
- `LOCAL_PREREQUISITE_UNAVAILABLE`
- `OFFLINE_OPTIONAL_SERVICE_UNAVAILABLE`

Capability-specific errors extend rather than redefine these meanings.

### Event rules

- Events are immutable and purpose-labelled.
- Event subscribers are restricted by `09_Capability_Interaction_Matrix.md`.
- Events contain the minimum data required by allowed subscribers.
- Consumers deduplicate by `message_id`.
- Ordering is guaranteed conceptually only within one owned aggregate when an event includes `aggregate_id` and monotonically increasing `aggregate_sequence`.
- Missing or delayed events never authorize an action.
- Terminal events use the same `correlation_id` as the accepted command.

### State exposure rules

- State is exposed as minimized snapshots or immutable events, never shared mutable references.
- The owner remains authoritative.
- A consumer cannot write exposed state except through the owner's command contract.
- Protected state requires point-of-use authorization.

---

## Runtime Host

### Public responsibilities

- Own process lifecycle, capability registry, aggregate health, runtime mode, and ordered shutdown.
- Expose lifecycle and status without exposing capability domain data.

### Public interface

- `HST-REQ-001 Host.getStatus() -> HostStatus`
- `HST-REQ-002 Host.getCapabilityAvailability(capability_id) -> CapabilityAvailability`
- `HST-CMD-001 Host.startCapability(capability_id)`
- `HST-CMD-002 Host.pauseCapability(capability_id)`
- `HST-CMD-003 Host.shutdownCapability(capability_id, reason)`
- `HST-CMD-004 Host.shutdown(reason)`
- `HST-EVT-001 HostStatusChanged`
- `HST-EVT-002 RuntimeModeChanged` (Intelligence only)

### Internal responsibilities

- Determine startup/shutdown order.
- Maintain host-only configuration and registry.
- Aggregate health without reading domain state.
- Exclude engineering artifacts from runtime packaging.

### Events consumed

- `CapabilityHealthChanged` from every capability.
- OS process/lifecycle signals.

### Events emitted

- `HostStatusChanged` to Experience.
- `RuntimeModeChanged` to Intelligence.
- Capability availability changes through host status.

### Commands accepted

- OS/user-authorized host shutdown.
- No domain commands.

### Commands produced

- Capability start, pause, and shutdown lifecycle commands.

### Requests accepted

- Host status and capability availability queries from Experience.

### Responses returned

- Domain-free host mode, readiness, degraded state, and capability availability.

### State owned

- Host configuration, capability registry, runtime mode, aggregate health, shutdown state.

### State exposed

- Minimized `HostStatus` and `CapabilityAvailability`; no domain payloads.

### Error conditions

- Invalid lifecycle transition, capability startup failure, shutdown timeout, host configuration failure.

### Permission requirements

- Status reads require no automation grant.
- User-initiated full shutdown is an explicit direct administration action.
- Host lifecycle authority never grants product permissions.

### Explainability requirements

- Explain starting, ready, degraded, offline, optional-online, capability unavailable, and shutting down.

---

## Permission Authority

### Public responsibilities

- Solely authorize, deny, challenge, validate, grant, revoke, and explain permissions.

### Public interface

- `PER-REQ-001 Permission.authorize(context) -> AuthorizationProof | Denial | ChallengeRef`
- `PER-REQ-002 Permission.validate(proof, execution_context) -> ValidationResult`
- `PER-REQ-003 Permission.getCatalogue(requester) -> PermissionCatalogue`
- `PER-REQ-004 Permission.explain(authorization_id) -> PermissionExplanation`
- `PER-CMD-001 Permission.resolveChallenge(challenge_id, user_decision)`
- `PER-CMD-002 Permission.revoke(grant_id | scope, reason)`
- `PER-CMD-003 Permission.updateAutomaticPolicy(policy_change)`
- `PER-EVT-001 PermissionChallengeRaised`
- `PER-EVT-002 PermissionDecisionRecorded`
- `PER-EVT-003 PermissionRevoked`

### Internal responsibilities

- Evaluate default-deny policy and explicit automatic-execution exceptions.
- Bind proofs to context and enforce expiry/replay constraints.
- Own permission audit and challenge lifecycle.

### Events consumed

- Capability health/lifecycle only indirectly through Host availability; no domain event dependency.
- Explicit user decisions from Experience arrive as commands.

### Events emitted

- Challenges, decisions, and revocations to Experience, Companion, and affected capability only as relevant.
- Decision events contain no proof secret or unrelated grants.

### Commands accepted

- Resolve challenge, revoke, update automatic policy.

### Commands produced

- None. Authority issues responses and immutable authority events.

### Requests accepted

- Authorize, validate, catalogue, and explain.

### Responses returned

- Bound proof, denial, challenge reference, validation result, catalogue, or minimized explanation.

### State owned

- Permission catalogue, grants, denials, policies, challenge state, proof state, audit records.

### State exposed

- Requester-relevant decisions and catalogue entries; never unrelated grants.

### Error conditions

- Unknown scope, invalid subject/target, expired challenge, revoked/expired/replayed/mismatched proof, policy unavailable.

### Permission requirements

- Permission inspection is user-authorized administration.
- Grant/revoke/policy changes require explicit user decision or pre-existing narrowly scoped administration authority.
- Permission Authority cannot authorize itself to perform domain actions.

### Explainability requirements

- Explain requested scope, requester, purpose, target, decision basis, active grant, expiry, and revocation path.

---

## Workspace Management

### Public responsibilities

- Own workspaces, zones, membership, active scope, and organizational metadata.

### Public interface

- `WSP-REQ-001 Workspace.getActive(authorization_proof) -> WorkspaceScope`
- `WSP-REQ-002 Workspace.list(filter, authorization_proof) -> WorkspaceSummary[]`
- `WSP-REQ-003 Workspace.scopeFor(capability_request, authorization_proof) -> WorkspaceScope`
- `WSP-CMD-001 Workspace.mutate(change, authorization_proof)`
- `WSP-EVT-001 WorkspaceScopeChanged`
- `WSP-EVT-002 WorkspaceChanged`

### Internal responsibilities

- Validate workspace invariants and authorization at access/commit.
- Persist organizational structure locally.
- Maintain active selection.

### Events consumed

- Permission revocation affecting in-flight protected access.
- Host lifecycle commands.

### Events emitted

- Scope changes to Companion, Memory, Context Sensing, and Action where scope validation is required.
- Domain change completion to requesting Companion task.
- Health to Runtime Host.

### Commands accepted

- Create, rename, reorganize, select, and archive workspace/zone changes from Companion with proof.

### Commands produced

- None outside its owned aggregate.

### Requests accepted

- Active scope, list, and scope validation from Experience (read-only) and authorized capabilities.

### Responses returned

- Minimized summaries, active scope, accepted/rejected mutation, and conflict details.

### State owned

- Workspace/zone records, membership, active scope, organizational metadata.

### State exposed

- Authorized summaries and immutable scope snapshots.

### Error conditions

- Workspace missing, invalid hierarchy, archived target, scope mismatch, conflicting mutation, authorization invalid.

### Permission requirements

- `workspace.read` for all reads.
- `workspace.write` for mutations.
- Validate proof at read or mutation commit point.

### Explainability requirements

- Explain active scope, requested structural change, responsible requester, permission used, result, and conflicts.

---

## Memory

### Public responsibilities

- Solely own durable remembered knowledge, provenance, retrieval, retention, redaction, and forgetting.

### Public interface

- `MEM-CMD-001 Memory.proposeWrite(candidate, authorization_proof)`
- `MEM-REQ-001 Memory.retrieve(query, scope, purpose, authorization_proof) -> MemoryResult`
- `MEM-CMD-002 Memory.forget(item_id, authorization_proof)`
- `MEM-CMD-003 Memory.redact(item_id, redaction, authorization_proof)`
- `MEM-REQ-002 Memory.explain(item_id, authorization_proof) -> MemoryExplanation`
- `MEM-EVT-001 MemoryWriteResolved`
- `MEM-EVT-002 MemoryForgetResolved`
- `MEM-EVT-003 MemoryPolicyChanged`

### Internal responsibilities

- Validate provenance, scope, retention, redaction, and authorization.
- Keep retrieval purpose-limited and minimized.
- Prevent observations, prompts, logs, and task history becoming alternate memory stores.

### Events consumed

- Workspace scope change where relevant.
- Permission revocation.
- Host lifecycle.

### Events emitted

- Write/forget/policy outcomes to requesting Companion task.
- Health to Runtime Host.

### Commands accepted

- Propose write, forget, and redact from Companion only, with proof.

### Commands produced

- None.

### Requests accepted

- Retrieve and explain from Companion only, with `memory.read`.

### Responses returned

- Provenance-bearing minimized memory result, explanation, command acceptance, or denial.

### State owned

- Memory items, collections, provenance, retention/forget policies, sensitive-access audit metadata.

### State exposed

- Purpose- and scope-limited copies; never mutable storage references.

### Error conditions

- Item missing, scope mismatch, retention conflict, invalid provenance, authorization invalid, local store unavailable.

### Permission requirements

- `memory.read`, `memory.write`, and `memory.forget` are distinct.
- Explanations that reveal content/provenance require `memory.read`.
- Validate at each access/commit point.

### Explainability requirements

- Explain what was remembered, why, source, scope, retention, permission, and how to redact/forget.

---

## Context Sensing

### Public responsibilities

- Own permissioned read-only sensing sessions, ephemeral raw observations, minimization, and context events.

### Public interface

- `SEN-CMD-001 Sense.start(sensor_class, purpose, authorization_proof)`
- `SEN-CMD-002 Sense.stop(session_id)`
- `SEN-CMD-003 Sense.pause(session_id)`
- `SEN-CMD-004 Sense.resume(session_id, authorization_proof)`
- `SEN-REQ-001 Sense.queryCurrent(purpose, authorization_proof) -> ContextSnapshot`
- `SEN-REQ-002 Sense.explain(sample_id, authorization_proof) -> SensingExplanation`
- `SEN-REQ-003 Sense.subscribe(filter, authorization_proof) -> SubscriptionRef`
- `SEN-EVT-001 ContextObserved`
- `SEN-EVT-002 MemoryCandidateObserved`
- `SEN-EVT-003 SensingStateChanged`

### Internal responsibilities

- Keep raw observations inside Context Sensing.
- Minimize every emitted payload, including deep-sensing output.
- Enforce session authorization continuously and clear ephemeral buffers on stop/revoke/shutdown.

### Events consumed

- Workspace scope change for tagging/invalidating scope.
- Permission revocation.
- Host lifecycle.

### Events emitted

- Minimized context, optional memory candidate, and sensing state to Companion only.
- Health to Runtime Host.

### Commands accepted

- Start, stop, pause, and resume from Companion.

### Commands produced

- None; Context Sensing never commands Action or Memory.

### Requests accepted

- Current context, explanation, and subscription from Companion.

### Responses returned

- Minimized snapshot, subscription reference, sensing explanation, acceptance/denial.

### State owned

- Sensing sessions, ephemeral buffers, rate limits, pause state, minimization/redaction rules.

### State exposed

- Purpose-limited minimized snapshots/events; never raw buffers.

### Error conditions

- Sensor unavailable, sensor class denied, invalid/expired session, revoked proof, scope mismatch, minimization failure.

### Permission requirements

- `sense.observe.basic` and `sense.observe.deep` are distinct.
- Deep permission never waives minimization.
- Start/resume require a valid sensing proof. Pause/stop require the owning task/session identity but no fresh sensing grant because they only reduce observation.
- Explanations capable of revealing observed data require the same sensing scope as the sample.

### Explainability requirements

- Explain sensor class, active/paused state, purpose, requester, permission, data class observed, minimization, and stop/revoke action.

---

## Action

### Public responsibilities

- Solely execute permitted environment mutations and report exact outcomes.

### Public interface

- `ACT-CMD-001 Action.execute(action_request, authorization_proof)`
- `ACT-CMD-002 Action.cancel(execution_id, authorization_proof)`
- `ACT-REQ-001 Action.describe(action_type, authorization_proof) -> ActionDescription`
- `ACT-REQ-002 Action.getStatus(execution_id, authorization_proof) -> ExecutionStatus`
- `ACT-EVT-001 ActionStarted`
- `ACT-EVT-002 ActionProgressed`
- `ACT-EVT-003 ActionCompleted`
- `ACT-EVT-004 ActionFailed`
- `ACT-EVT-005 ActionCancelled`

### Internal responsibilities

- Validate proof immediately before execution.
- Enforce declared action type and exact target/scope.
- Track in-flight state, cancellation, partial effects, and local audit.

### Events consumed

- Workspace scope change where target scope matters.
- Permission revocation.
- Host lifecycle.

### Events emitted

- Progress and terminal outcome to requesting Companion task.
- Health only to Runtime Host.

### Commands accepted

- Execute and cancel from Companion only.

### Commands produced

- Environment mutation operations internal to Action; never capability-domain commands.

### Requests accepted

- Describe action type and query execution status from Companion.

### Responses returned

- Acceptance/rejection, action description, and execution status. Completion arrives as terminal event for non-immediate work.

### State owned

- Action catalogue, in-flight execution state, metadata-only execution audit, and minimized effect summary; never copied user content.

### State exposed

- Minimized status/effect summary; no reusable execution authority.

### Error conditions

- Unsupported action, invalid target, proof invalid, target unavailable, partial failure, cancellation unsafe/too late, scope changed.

### Permission requirements

- Per-action-class permission; no omnibus grant.
- Proof bound to requester, purpose, subject, target, operation, scope, and optional extension.
- Describe/status/cancel require a proof bound to the task/execution. Revoking execution authority never blocks a safe cancellation request authorized for that same task.
- Revocation blocks new effects; in-flight safe cancellation is reported.

### Explainability requirements

- Before/during/after explain action, purpose, target, responsible capability, permission, effects, partial effects, result, cancellation/undo availability.

---

## Intelligence

### Public responsibilities

- Provide local-first reasoning/generation and non-executing proposals.

### Public interface

- `INT-CMD-001 Intelligence.reason(request, authorization_proof)`
- `INT-CMD-002 Intelligence.generate(request, authorization_proof)`
- `INT-REQ-001 Intelligence.listProviders(authorization_proof) -> ProviderSummary[]`
- `INT-REQ-002 Intelligence.explain(transaction_id, authorization_proof) -> InferenceExplanation`
- `INT-EVT-001 InferenceCompleted`
- `INT-EVT-002 InferenceFailed`
- `INT-EVT-003 ProposalProduced`
- `INT-EVT-004 ProviderAvailabilityChanged`

### Internal responsibilities

- Select only permitted local/optional-remote provider class.
- Enforce local resource limits and offline mode.
- Keep persisted records metadata-only.
- Mark proposals as untrusted, non-executing suggestions.

### Events consumed

- Runtime mode from Runtime Host.
- Permission revocation.
- Host lifecycle.

### Events emitted

- Inference outcomes/proposals to requesting Companion task only.
- Provider availability changes to Companion without provider secrets.
- Health to Runtime Host.

### Commands accepted

- Reason and generate from Companion only.

### Commands produced

- None. Proposals are events/data, never Action commands.

### Requests accepted

- List providers and explain transaction from Companion.

### Responses returned

- Command acceptance/denial, provider summaries, metadata-only explanation. Results arrive as events for non-immediate work.

### State owned

- Provider configuration/references, local resource limits, metadata-only inference transaction records.

### State exposed

- Provider class/availability, result payload to requesting task, uncertainty, metadata-only explanation.

### Error conditions

- No local provider, offline optional remote, remote/data-sharing denied, resource limit, invalid structured result, timeout/cancel.

### Permission requirements

- `intelligence.local.run` for protected local inference.
- `intelligence.remote.run` plus explicit data-sharing scope for remote use.
- No silent cloud fallback.
- Explanations/results restricted to the requesting task or authorized administrator.

### Explainability requirements

- Explain local/remote provider class, requester, purpose, permission/data boundary, waiting/resource state, uncertainty, and outcome.

---

## Companion Orchestration

### Public responsibilities

- Solely interpret user intent, own task sequencing, coordinate capabilities, request authorization, and publish progress.

### Public interface

- `COM-CMD-001 Companion.handleIntent(intent)`
- `COM-CMD-002 Companion.submitInteractionResult(task_id, result)`
- `COM-CMD-003 Companion.cancel(task_id)`
- `COM-REQ-001 Companion.explain(task_id) -> TaskExplanation`
- `COM-REQ-002 Companion.getTaskStatus(task_id) -> TaskStatus`
- `COM-EVT-001 TaskProgressed`
- `COM-EVT-002 UserInteractionRequested`
- `COM-EVT-003 TaskCompleted`
- `COM-EVT-004 TaskFailed`
- `COM-EVT-005 TaskCancelled`

### Internal responsibilities

- Convert intent into a bounded task/plan.
- Sequence public capability contracts only.
- Obtain authorization before protected operations and pass proofs to committing capability.
- Evaluate Intelligence proposals without treating them as authority.
- Keep task content in-flight only; persist metadata-only outcomes/correlation.

### Events consumed

- Permission decisions/revocations.
- Permission challenges.
- Workspace scope changes.
- Workspace terminal domain results for requested mutations.
- Memory outcomes.
- Minimized context/memory candidates.
- Sensing state changes.
- Intelligence results/proposals.
- Intelligence provider availability changes.
- Action outcomes.
- Extension contributions/errors.
- Extension state changes.
- Host lifecycle.

### Events emitted

- Presentation-neutral progress, interaction requests, explanations, and terminal task outcomes to Experience.
- Health to Runtime Host.

### Commands accepted

- Handle intent, submit interaction result, cancel task from Experience.

### Commands produced

- Permission challenge/authorization requests.
- Workspace mutations, Memory writes/forgetting, Sense session changes, Intelligence inference, Action execution/cancellation, and optional Extension management through their public contracts.

### Requests accepted

- Task status/explanation from Experience.

### Responses returned

- Intent/task acceptance, task identity, current status, and explanation. Consequential completion uses events.

### State owned

- Active task/plan state, minimized in-flight intent history, orchestration policy, metadata-only outcomes/correlation.

### State exposed

- Task status and explanation; no raw provider, sensing, or memory internals.

### Error conditions

- Intent invalid, dependency unavailable, permission denied, user response expired, conflicting task, cancellation, proposal invalid.

### Permission requirements

- No ambient superuser authority.
- Every protected downstream call uses task-purpose-bound authorization.
- Extension-originated identity remains bound through all downstream requests.

### Explainability requirements

- Continuously identify current step, why, responsible capability, permission state, waiting condition, user decision needed, and final/partial result.

---

## Experience

### Public responsibilities

- Solely capture user interaction and present status, challenges, explanations, and domain results.

### Public interface

- `EXP-EVT-001 Experience.consumeHostStatus(event)`
- `EXP-EVT-002 Experience.consumePermissionEvent(event)`
- `EXP-EVT-003 Experience.consumeCompanionEvent(event)`
- User outputs are sent through receiver-owned contracts: Companion intents/results, Permission decisions, Host status queries, Workspace read queries.

### Internal responsibilities

- Render without changing domain meaning.
- Manage local presentation state, accessibility, and ephemeral input buffers.
- Keep voice a modality, not an orchestration capability.

### Events consumed

- Aggregated host status.
- Permission challenges/revocations.
- Permission decisions.
- Companion progress/interaction/terminal events.

### Events emitted

- Health to Runtime Host only.
- User input is a command/request, not a fabricated domain event.

### Commands accepted

- None from capabilities. Host, Permission, and Companion information arrives as immutable events.

### Commands produced

- Companion intent/interaction/cancel commands.
- Permission challenge decision/revoke commands.

### Requests accepted

- None from capabilities.

### Responses returned

- Event-consumption acknowledgement only; never domain success.

### State owned

- UI preferences/layout, theme, accessibility settings, ephemeral interaction buffers.

### State exposed

- Presentation state only.

### Error conditions

- Surface unavailable, invalid presentation payload, accessibility fallback, voice modality unavailable.

### Permission requirements

- UI presentation requires no automation grant.
- Workspace reads require purpose-bound `workspace.read`.
- Microphone/overlay modalities require their explicit scopes.
- Experience cannot grant permission; it conveys the user's decision to Authority.

### Explainability requirements

- Never hide or rewrite responsible capability, purpose, permission, target summary, uncertainty, partial result, or failure.

---

## Extension Host

### Public responsibilities

- If separately activated, isolate extensions, attenuate their identity/scopes, manage lifecycle, and route contributions to Companion.

### Public interface

- `EXT-CMD-001 Extension.install(manifest, authorization_proof)`
- `EXT-CMD-002 Extension.unload(extension_id, authorization_proof)`
- `EXT-CMD-003 Extension.invoke(extension_id, request, authorization_proof)`
- `EXT-REQ-001 Extension.list(authorization_proof) -> ExtensionSummary[]`
- `EXT-REQ-002 Extension.explain(extension_id, authorization_proof) -> ExtensionExplanation`
- `EXT-EVT-001 ExtensionContributionProduced`
- `EXT-EVT-002 ExtensionErrorOccurred`
- `EXT-EVT-003 ExtensionStateChanged`

### Internal responsibilities

- Validate manifests, isolate data, bind extension identity, and enforce declared scopes.
- Keep the capability dormant until separately justified.
- Never call domain capabilities directly.

### Events consumed

- Permission revocation for extension/manage scopes.
- Host lifecycle.

### Events emitted

- Contributions/intents, errors, and state changes to Companion only.
- Health to Runtime Host.

### Commands accepted

- Install, unload, and invoke from Companion only, after activation and with proof.

### Commands produced

- None to domain capabilities. Extension-originated requests are contribution events to Companion.

### Requests accepted

- List and explain from Companion only, with `extension.manage`.

### Responses returned

- Acceptance/denial, minimized summaries/explanations; invocation output is an event to Companion.

### State owned

- Manifests, enablement, isolated extension configuration, disposable cache, metadata-only extension audit.

### State exposed

- Minimized manifest/permission/state summary; never another extension's partition.

### Error conditions

- Capability dormant, manifest invalid, extension untrusted/disabled/crashed, scope undeclared, identity mismatch, proof invalid.

### Permission requirements

- `extension.manage` plus extension-identity-bound scopes.
- Extension never inherits Companion/user-session authority.
- All domain effects are re-authorized after Companion mediation.
- Extension partitions may not retain user knowledge, observations, task content, or prompt/response content. Durable user knowledge must follow a Companion-mediated, separately authorized Memory proposal.

### Explainability requirements

- Identify extension, contribution, declared/requested scopes, mediation step, responsible core capability, failure/unload/revoke state.

---

## Inter-capability Interaction Contracts

Each row is exhaustive for allowed domain, lifecycle, authorization, and event interactions in `09_Capability_Interaction_Matrix.md`. A direct response is allowed only on the initiating path.

### Runtime and health interactions

| ID | Initiator → Receiver | Purpose / trigger | Expected response | Failure behaviour | Permission boundary | Data exchanged | Mode |
|----|----------------------|-------------------|-------------------|-------------------|---------------------|----------------|------|
| IC-001 | Runtime Host → every active capability | Start/pause/shutdown after host lifecycle transition | Lifecycle acknowledgement; eventual health event | Mark unavailable; no domain fallback | Host lifecycle only; no domain authority | Capability id, lifecycle state, reason, correlation | Command + async event |
| IC-002 | Every capability → Runtime Host | Health changed | No domain response; Host aggregates status | Missing health becomes unavailable/unknown | No product permission; no domain payload | Capability id, ready/degraded/unavailable, safe reason | Async event |
| IC-003 | Runtime Host → Experience | Aggregate host status changed | Presentation acknowledgement | Experience shows last-known/degraded status | No automation grant | Mode, readiness, capability availability, safe reason | Async event |
| IC-004 | Experience → Runtime Host | User opens system status | `HostStatus` | Honest unavailable status; no fabricated health | No automation grant | Status query; domain-free response | Sync request |
| IC-005 | Runtime Host → Intelligence | Offline/online-optional mode changed | No command response | Intelligence defaults local-only if mode unknown | Event grants no provider authority | Mode and timestamp only | Async event |

### Permission interactions

| ID | Initiator → Receiver | Purpose / trigger | Expected response | Failure behaviour | Permission boundary | Data exchanged | Mode |
|----|----------------------|-------------------|-------------------|-------------------|---------------------|----------------|------|
| IC-006 | Companion → Permission Authority | Protected task step is ready, or task authorization needs explanation | Proof, denial, challenge ref, or task-scoped permission explanation | Stop/wait; never bypass | Purpose/task/requester/target/operation bound; explanation limited to task authorization | Authorization context or authorization id; no protected payload beyond target summary | Sync request |
| IC-007 | Experience → Permission Authority | Authorized Workspace read needed, or user opens permission catalogue/explanation | Proof, denial, challenge ref, catalogue, or explanation | Workspace/permission view remains unavailable | `workspace.read` for Workspace proof; user administration authority for catalogue/explanation | User, purpose, requested view/authorization id; only user's relevant permissions returned | Sync request |
| IC-008 | Experience → Permission Authority | User answers challenge, revokes, or explicitly updates automatic policy | Decision acknowledgement; decision/revoke event | Fail closed and show unresolved state | Only explicit user administration changes grant/policy state | Challenge/grant/policy id, decision/change, reason | Command + async event |
| IC-009 | Workspace/Memory/Context Sensing/Action/Intelligence/activated Extension Host → Permission Authority | Point-of-use validation before access/commit | Valid/invalid with reason code | Reject operation; emit explanation | Exact execution context must match proof; protected capabilities do not issue grants | Opaque proof + execution context; response contains no grant secret | Sync request |
| IC-010 | Permission Authority → Experience | Challenge or visible revoke occurred | Presentation acknowledgement/user response later | Deny while unpresented/unresolved | Event is not a grant | Minimized challenge/revoke explanation | Async event |
| IC-011 | Permission Authority → Companion | Task authorization challenge raised, decided, or revoked | Companion waits, resumes, or stops affected path | Stop/wait path; cancel safely where possible | No new authority in event; only a returned proof authorizes a later protected call | Challenge/authorization id, decision/state, scope, correlation | Async event |
| IC-012 | Permission Authority → affected capability | Active proof/grant revoked | Capability blocks new protected effects | Safe cancel/invalidate according to owner policy | Revocation overrides unused authority | Authorization id, scope, effective time | Async event |

### Workspace interactions

| ID | Initiator → Receiver | Purpose / trigger | Expected response | Failure behaviour | Permission boundary | Data exchanged | Mode |
|----|----------------------|-------------------|-------------------|-------------------|---------------------|----------------|------|
| IC-013 | Experience → Workspace Management | Render read-only navigation view | Authorized summaries/scope | Show unavailable/denied state | `workspace.read` proof validated at access | Filter, proof; minimized summaries | Sync request |
| IC-014 | Companion → Workspace Management | Read task scope or mutate organization | Scope/summary or accepted mutation | Task degrades unscoped; mutation fails closed | Read/write proof as appropriate | Task purpose, query/change, proof | Sync request or command |
| IC-015 | Memory → Workspace Management | Validate retrieval/write scope | Scope validation result | Reject scoped memory operation | Caller-bound `workspace.read`/scope proof | Workspace/zone ids and authorization context | Sync request |
| IC-016 | Context Sensing → Workspace Management | Tag/validate observation scope | Scope snapshot | Emit unscoped/minimized or stop as policy requires | Authorized scope only | Workspace id request; no observation content | Sync request |
| IC-017 | Action → Workspace Management | Validate target belongs to authorized scope | Scope validation result | Reject execution | Action proof must bind same scope | Target scope identity; no effect payload | Sync request |
| IC-018 | Workspace Management → Companion/Memory/Context Sensing/Action | Active/relevant scope changed, or an accepted Workspace mutation reached a terminal domain result | Companion updates task/result; scoped subscribers invalidate/update scope | Subscriber treats stale scope as invalid; Companion treats missing terminal result as unknown, never success | Event grants no read/write authority; domain results go only to requesting Companion task | Scope ids/version for subscribers; minimized mutation outcome/correlation for Companion; no unrelated workspace content | Async event |

### Memory interactions

| ID | Initiator → Receiver | Purpose / trigger | Expected response | Failure behaviour | Permission boundary | Data exchanged | Mode |
|----|----------------------|-------------------|-------------------|-------------------|---------------------|----------------|------|
| IC-019 | Companion → Memory | Retrieve relevant memory or explain a memory item for task | Provenance-bearing minimized result/explanation | Continue reduced-context and explain | `memory.read` + workspace scope | Query/item id, scope, purpose, proof; result copies | Sync request |
| IC-020 | Companion → Memory | Retain approved candidate/user knowledge | Accepted then write-resolved event | Reject; no silent retry/bypass | `memory.write`; observation permission is insufficient | Candidate, provenance, retention intent, proof | Command + async event |
| IC-021 | Companion → Memory | Forget/redact on user intent/policy | Accepted then terminal event | Keep item unchanged and explain failure | `memory.forget` | Item id/redaction, purpose, proof | Command + async event |
| IC-022 | Memory → Companion | Write/forget/policy outcome | Task updates/explanation | Companion marks result unknown if missing | Event contains authorization id, not proof | Item id or minimized summary, outcome, provenance/policy refs | Async event |

### Context Sensing interactions

| ID | Initiator → Receiver | Purpose / trigger | Expected response | Failure behaviour | Permission boundary | Data exchanged | Mode |
|----|----------------------|-------------------|-------------------|-------------------|---------------------|----------------|------|
| IC-023 | Companion → Context Sensing | Start/resume or safety-reducing pause/stop for a task sensing session | Acceptance/denial; state event | Ask user for context or degrade; stop on invalid session ownership | Start/resume require basic/deep proof with continuous validity; pause/stop require owning task/session identity and cannot expand access | Sensor class/purpose/scope/proof for start/resume; session/task id for pause/stop; no unrelated context | Command + async event |
| IC-024 | Companion → Context Sensing | Query/subscribe to current context or explain a sample | Minimized snapshot/subscription/explanation | User-provided context fallback | Sensing proof validated at query/subscription/explanation | Purpose, filter/sample id, scope, proof | Sync request |
| IC-025 | Context Sensing → Companion | Authorized context or memory candidate observed | No command response; Companion may use/propose retention | Drop if task/subscription invalid | Event is minimized and grants no retention/action authority | Derived/minimized context, provenance class, scope, correlation | Async event |

### Intelligence interactions

| ID | Initiator → Receiver | Purpose / trigger | Expected response | Failure behaviour | Permission boundary | Data exchanged | Mode |
|----|----------------------|-------------------|-------------------|-------------------|---------------------|----------------|------|
| IC-026 | Companion → Intelligence | Reason/generate for task | Acceptance; result/proposal event | Deterministic fallback or ask user | Local proof or remote + data-sharing proof | Purpose, minimized supplied context, constraints, proof | Command + async event |
| IC-027 | Companion → Intelligence | Query provider availability/explanation | Minimized provider/transaction metadata | Explain unavailable | Requester/task authorization; no prompt-history read | Provider filter or transaction id, proof | Sync request |
| IC-028 | Intelligence → Companion | Inference completed/failed, proposal produced, or provider availability changed | Companion evaluates; never auto-executes | Reject malformed/uncertain proposal; mark unavailable provider | Event has no Action authority | Result/proposal or provider availability, uncertainty, provider class, correlation | Async event |

### Action interactions

| ID | Initiator → Receiver | Purpose / trigger | Expected response | Failure behaviour | Permission boundary | Data exchanged | Mode |
|----|----------------------|-------------------|-------------------|-------------------|---------------------|----------------|------|
| IC-029 | Companion → Action | Execute approved environment mutation | Acceptance/denial; progress/terminal events | Stop; report partial effects honestly | Exact action proof validated immediately before effects | Declared action, target, constraints, proof | Command + async event |
| IC-030 | Companion → Action | Cancel/query/describe execution | Acceptance/status/description | Report invalid proof or unsafe/too-late cancellation | Task/execution-bound proof required; cancellation cannot expand authority | Execution/action type id, proof, correlation | Sync request or command |
| IC-031 | Action → Companion | Execution progress or terminal outcome | Task updates/explanation | Missing terminal event becomes unknown/degraded, never assumed success | Event grants no further action | Effect summary, partial effects, status, authorization id | Async event |

### Experience and Companion interactions

| ID | Initiator → Receiver | Purpose / trigger | Expected response | Failure behaviour | Permission boundary | Data exchanged | Mode |
|----|----------------------|-------------------|-------------------|-------------------|---------------------|----------------|------|
| IC-032 | Experience → Companion | User submits intent/interaction result/cancel | Task/command acknowledgement | Show unavailable/expired/conflict | User intent is not an ambient grant | Intent or response, user-origin marker, task/correlation | Command |
| IC-033 | Experience → Companion | User requests task status/explanation | Task status/explanation | Show unavailable without inventing result | Protected details minimized to current user/task | Task id | Sync request |
| IC-034 | Companion → Experience | Progress, question, explanation, or terminal result | Presentation acknowledgement/user response later | Task may wait/cancel; domain state unchanged by rendering failure | Event cannot grant permission or execute action | Explainability envelope, interaction options, minimized result | Async event |

### Extension interactions (only after separate activation)

| ID | Initiator → Receiver | Purpose / trigger | Expected response | Failure behaviour | Permission boundary | Data exchanged | Mode |
|----|----------------------|-------------------|-------------------|-------------------|---------------------|----------------|------|
| IC-035 | Companion → Extension Host | Install/unload/list/invoke/explain approved extension | Acceptance/summary/explanation; contribution/error event | Core continues; extension remains unloaded/disabled | `extension.manage`, extension identity, declared scopes | Manifest/request summary, proof, extension id | Sync request or command |
| IC-036 | Extension Host → Permission Authority | Validate an already Companion-obtained extension management/scope proof at point of use | Valid/invalid result | Fail closed/unload affected extension | Extension Host cannot request or grant authorization; proof remains bound to extension identity | Opaque proof, extension id, exact execution context | Sync request |
| IC-037 | Permission Authority → Extension Host | Extension permission revoked | Invalidate/unload/stop affected invocation | Core remains available | Revocation blocks new mediated effects | Authorization/grant id, extension id, effective time | Async event |
| IC-038 | Extension Host → Companion | Extension contribution/intent produced, extension failed, or extension lifecycle state changed | Companion evaluates contribution or updates task/extension status; no direct execution | Drop malformed contribution, isolate crash, or mark extension state unknown if event is missing | Event carries extension identity but no domain authority | Minimized contribution, safe error, or enabled/disabled/unloaded/crashed state; declared scope and correlation | Async event |

### Host health from dormant Extension Host

- IC-001, IC-002, and IC-012 apply to Extension Host only after it is separately activated.
- No contract in this document activates Extension Host or justifies its implementation.

---

## Contract-level Validation

### Ownership and authority

- Runtime Host owns lifecycle only.
- Permission Authority alone grants/revokes and validates authorization truth.
- Workspace Management alone mutates workspace organization.
- Memory alone retains remembered user knowledge.
- Context Sensing alone holds raw observations and never persists them as Memory.
- Action alone mutates the environment.
- Intelligence only reasons/generates/proposes and produces no Action command.
- Companion alone sequences multi-capability tasks.
- Experience presents/captures only and returns no domain success.
- Extension Host, if activated, mediates extensions but has no domain-capability call path.

Result: no circular ownership or duplicate authority.

### Dependency direction

- Commands flow from Experience to Companion, then from Companion to domain capabilities.
- Domain results return as responses/events and do not reverse ownership.
- Experience and Companion do not form a hard dependency cycle.
- Extension contributions are events; optional Companion management calls do not create reverse call ownership.

Result: no circular hard dependency.

### Permission enforcement

- All protected capabilities accept bound proofs and validate at point of use.
- No event carries a proof or grants authority.
- Permission revocation stops new protected effects.
- Automatic policy still passes through authorize/validate.
- Experience conveys decisions but cannot grant.
- Intelligence proposals and extension contributions require fresh Companion mediation and authorization.

Result: no Permission Authority bypass.

### Privacy and Local First

- Raw sensing never leaves Context Sensing.
- Memory is the only durable user-knowledge owner.
- Intelligence and Companion persist metadata only.
- Remote Intelligence is optional, explicitly authorized, and never a silent fallback.
- Core contracts have local failure/degraded outcomes and no required internet interaction.

Result: Privacy First and Local First preserved.

### Explainability

- Every consequential command has a purpose, responsible capability, permission reference, and terminal outcome.
- Partial, denied, cancelled, unavailable, and degraded states are first-class.
- Experience cannot rewrite domain explanations.

Result: ADR-0006 satisfied.

### Completeness

- Every allowed pair/purpose in `09_Capability_Interaction_Matrix.md` has a contract above.
- Every unlisted direct interaction remains forbidden.
- Contract responses do not create reverse dependencies.

Result: every capability communicates only through defined contracts.

---

## Risks and Deferred Detail

These are architecture risks, not technology questions:

1. Exact field schemas and compatibility/change rules require a later contract-specification milestone.
2. Permission scope taxonomy and proof lifetime/replay policy require refinement without weakening the bindings defined here.
3. Cancellation/partial-effect semantics need per-action-class acceptance scenarios.
4. Event loss, duplication, ordering, and recovery need contract tests while preserving the conceptual rules here.
5. Offline/degraded core scenarios need executable acceptance tests.
6. Extension Host remains dormant until separately justified under ADR-0004.

Recommended next milestone: **Contract Schema and Acceptance Specification** — define versioned field schemas, invariants, and executable architectural acceptance cases before technology research or implementation.

