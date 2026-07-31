# Workspace Contract Schema and Acceptance Specification v1.0

Status: Active
Authority: Authoritative conceptual schema and pre-implementation acceptance specification for capability contracts
Version: 1.0

This document defines architectural meaning, required conceptual fields, compatibility rules, invariants, and acceptance cases. It does not define a wire format, serialization, programming-language type, transport, storage engine, framework, or API.

Authority order:

1. `00_Workspace_Blueprint.md`
2. Accepted ADRs in `architecture/decisions/`
3. `08_Workspace_Capability_Architecture.md` — ownership
4. `09_Capability_Interaction_Matrix.md` — permitted communication and trust
5. `10_Capability_Contracts.md` — public contract purposes and interactions
6. This document — conceptual schema, evolution, and acceptance

Higher authority wins on conflict.

---

## Scope

This specification applies to:

- requests
- responses
- commands
- events
- lifecycle control messages
- permission challenges, decisions, proofs, and validation
- errors
- cancellation
- state references
- explanations
- audit/trace metadata

Every future capability contract must satisfy this specification before implementation begins.

---

## Standard Contract Model

### Conceptual message

Every interaction is represented by one conceptual message envelope plus message-kind-specific content.

The envelope separates:

1. **Identity** — which message and contract this is.
2. **Causality** — what caused it and which activity it belongs to.
3. **Authority** — who requested it and which permission applies.
4. **Intent** — why the interaction exists.
5. **Temporal expectations** — when it was created and when waiting should stop.
6. **Content reference** — the minimized request, result, event fact, error, or state reference.
7. **Explainability** — what the user may need to understand.
8. **Evolution** — which contract version defines the meaning.

The envelope is architectural metadata, not a license to centralize capability-owned data.

### Presence classes

Conceptual fields have one of four presence classes:

- **Always** — required on every cross-capability message.
- **Conditional** — required when the stated condition exists.
- **Introduced by owner** — absent before the responsible owner creates the concept, then required on subsequent related messages.
- **Prohibited** — must not appear for that message or trust boundary.

Absence is valid only when this specification explicitly allows it. Empty placeholder values do not satisfy required presence.

---

## Identity and Correlation

### Message identity

Every message has an **Always** message identity that:

- distinguishes it from every other message
- remains stable across delivery retries
- is never reused for different semantic content
- supports deduplication and audit correlation
- contains no user content or authority

A retry of the same semantic attempt keeps the same message identity. A new attempt with changed intent, target, permission, or content receives a new identity.

### Contract identity

Every message has:

- **Always** contract identity
- **Always** breaking-version identity
- **Always** compatible-revision identity
- **Always** message kind

Contract identity names one architectural purpose from `10_Capability_Contracts.md`. It must not be shared by semantically unrelated operations.

### Correlation identity

Every request/response, command/result, and event chain has an **Always** correlation identity.

Correlation:

- groups one logical interaction
- does not grant authority
- does not transfer ownership
- does not imply all records are stored together
- must remain stable through retries and terminal outcomes

### Causation identity

Causation identity is:

- absent only for a root message
- otherwise **Conditional** and points to the immediate preceding message
- one step only; it is not a copied list of the whole trace

The complete causal path is reconstructed from separately owned records.

### Task identity

Task identity is:

- **Conditional** for Companion-mediated work
- stable for the lifetime of the user intent/task
- absent for direct user administration that has no Companion task
- prohibited from being invented by a downstream capability

Companion Orchestration owns task identity.

### Operation identity

Operation identity is:

- **Introduced by owner** when a capability accepts or begins a consequential operation
- owned by the capability committing the operation
- required on progress, status, cancellation, terminal events, and operation explanations
- absent on an initial request before the owner assigns it

An operation identity never authorizes another operation.

For a non-immediate domain command, acceptance also returns an opaque operation reference bound to the originating user/task/operation. The reference grants no authority, remains stable on redelivery, and is used only with the Permission Authority-issued operation-control proof for status or safety cancellation. It is prohibited from events, explanations, and audit content.

### Authorization identity

Authorization identity is:

- **Introduced by owner** by Permission Authority
- required after a permission decision/proof exists
- required on point-of-use validation, protected operation records, and protected explanations
- absent on the initial authorization request
- never a substitute for the opaque proof

### Extension identity

Extension identity is:

- **Conditional** and required for all extension-originated or extension-management messages
- preserved through Companion mediation and downstream authorization
- never replaced with Companion's identity

### Aggregate identity and sequence

Aggregate identity and sequence are:

- **Conditional** for ordered events within one capability-owned aggregate
- assigned only by the aggregate owner
- monotonic within that aggregate
- not a global system order

No consumer may infer order between unrelated aggregates.

---

## Intent and Purpose

Every message has an **Always** purpose statement.

Purpose must:

- be meaningful to a user, not merely an internal operation label
- be specific enough to evaluate permission and minimization
- remain stable across retries
- identify whether it is user-initiated, policy-initiated, lifecycle, or recovery work
- be narrowed or re-authorized if the operation expands

“System,” “automation,” “processing,” or equivalent generic wording is insufficient for a consequential operation.

Lifecycle and health messages use a fixed, domain-free purpose.

---

## Request and Response Semantics

### Request

A request:

- asks one receiver to evaluate or return information
- expects exactly one conceptual response
- does not itself assert that state changed
- identifies the requested view/purpose/scope
- includes permission context when protected data is requested
- declares a conceptual waiting limit

A request must be side-effect-free unless the corresponding contract explicitly identifies an immediate, idempotent administrative effect. Otherwise it is a command.

### Response

A response:

- carries the same correlation identity as the request
- causally references the request
- states completed, denied, challenge-required, invalid, unavailable, conflict, or failed
- identifies the receiver as responsible for the answer
- contains only the minimized data authorized for that requester
- cannot create a reverse dependency

A response received after the requester has stopped waiting is still not an event unless the contract defines an event separately.

### Request timeout

When a request waiting limit expires:

- the requester marks its wait as timed out/unknown
- timeout does not prove receiver failure
- timeout does not prove cancellation
- timeout does not authorize fallback to a broader scope or cloud provider
- any retry follows the retry and idempotency rules
- the user receives an explanation when the timeout affects visible work

---

## Command Semantics

A command:

- asks the owning capability to attempt a state transition
- names one declared operation
- identifies expected target and scope
- carries required authorization proof/context
- declares cancellation support and conceptual waiting limit
- receives an acceptance/rejection response
- uses terminal events for non-immediate completion

Acceptance means only that the owner took responsibility for attempting the operation.

Acceptance must never be presented as completion.

### Command lifecycle

Conceptual command states:

1. submitted
2. accepted or rejected
3. active (when execution begins)
4. optionally cancellation-requested
5. exactly one terminal state:
   - completed
   - failed
   - cancelled
   - partially-completed
   - indeterminate

`outcome-unknown` is a requester-observed uncertainty state, not an owner terminal state. It is required when the requester lacks authoritative terminal evidence. It blocks dependent effects and blind retry until reconciled. The owner later provides its single authoritative terminal state, including `indeterminate` if owner-side recovery cannot establish what happened.

`indeterminate` is an owner-authoritative terminal admission that evidence is irrecoverably insufficient after declared recovery is exhausted. It records all known partial effects, never becomes success, and prohibits automatic retry. Later discovery is a new reconciliation fact/operation and does not rewrite the original terminal claim.

### Command producer constraints

- Experience may produce only user-intent, user-decision, cancellation, and read/admin requests defined in the matrix.
- Companion may coordinate defined capability commands but owns none of their domain effects.
- Intelligence produces proposals, never commands to Action.
- Context Sensing produces no commands to Memory or Action.
- Extension Host produces no domain commands.

---

## Event Semantics

An event:

- is an immutable fact already observed or committed
- has one owning publisher
- names the fact in past/present-state language, not imperative language
- cannot request work
- cannot grant permission
- cannot contain an authorization proof
- is delivered only to subscribers permitted by the Interaction Matrix
- contains minimum necessary data for each allowed subscriber

### Event completion

Terminal events for a command:

- share correlation identity with the command
- contain the owner-assigned operation identity
- causally reference the latest known command/progress message
- include terminal status and minimized effect/outcome
- include authorization identity when protected
- never claim success without owned evidence

### Event duplication and ordering

- Duplicate delivery is expected and must not duplicate effects.
- Consumers deduplicate by message identity.
- Sequence gaps produce stale/unknown state, not fabricated state.
- Out-of-order events cannot roll an aggregate back to an earlier sequence.
- Cross-aggregate arrival order has no business meaning.
- Event replay never reauthorizes a command.

### Event loss

When a required terminal event may be missing:

- the requester marks outcome unknown after its waiting limit
- the owner remains authoritative for operation status
- every owner of a non-immediate domain command exposes an operation-status request requiring the stable operation reference and bound operation-control proof
- after control-proof expiry, delayed reconciliation uses a fresh user-administration proof to request a content-free terminal tombstone from the owner
- recovery queries that owner before retry or dependent effects
- a new side-effecting command is not issued merely because an event was lost

Owners retain the content-free terminal/deduplication tombstone for the declared retry/replay horizon. If that tombstone has expired, the original outcome remains unrecoverable and no automatic retry occurs; any new user-authorized attempt presents an explicit indeterminate-history warning.

Equivalent control-plane recovery is explicit:

- Runtime Host lifecycle state is reconciled through Host registry and aggregate health.
- Permission resolve/revoke/policy commands are immediate authoritative state transitions; decision events are notifications.
- Companion tasks are reconciled through `Companion.getTaskStatus`.

If an acceptance response is lost, redelivery of the same command identity returns the same operation identity/reference and does not create another operation.

---

## Permission Context

### Authorization request context

Every authorization request includes:

- requester capability
- subject/user
- permission scope
- purpose
- target
- operation
- optional workspace scope
- optional extension identity
- task/correlation identity where applicable
- requested lifetime/usage constraint

The initial request has no authorization identity until Permission Authority creates one.

### Authorization proof

The proof is opaque outside Permission Authority.

For every protected non-immediate operation, Permission Authority issues two independently governed authorities:

- effect authority for the exact protected access/effect
- operation-control proof for minimized status and supported safety-reducing pause/stop/cancel only

The operation-control proof is initially bound to subject, requester, task, owner capability, command identity, and purpose. The owner binds it to the operation identity/reference at acceptance. It cannot continue, repeat, compensate, or create an effect.

It has an independently user-revocable, bounded offline-control lease:

- effect-authority expiry/revocation does not revoke safety control
- local owner validation during Permission Authority outage is allowed only inside the lease
- control-proof revocation causes safe pause/cancel where possible before invalidation; an already irreversible effect may complete, but no subsequent effect begins
- delayed revocation delivery permits only bounded stale status/safety control and no new effect
- lease expiry without refreshed authority causes safe pause/cancel where possible; otherwise later effects stop and too-late/unsafe status is reported
- ordinary control expires at terminal state plus a bounded reconciliation period

It never appears in events, explanations, or audit content.

When authorization initially returns a challenge:

1. the requester retains the original authorization context and challenge reference
2. Experience submits the user's decision
3. Permission Authority emits a decision event containing no proof
4. the original requester repeats `authorize` with the same context and challenge reference
5. Permission Authority returns a proof only if the decision, context, lifetime, and current policy still permit it

A challenge decision event is never authority.

Every protected committing capability must:

1. receive the proof through a permitted caller
2. construct exact point-of-use context
3. validate with Permission Authority immediately before protected access/effect
4. reject expiry, revocation, replay, requester mismatch, purpose mismatch, target mismatch, operation mismatch, scope mismatch, or extension mismatch
5. record authorization identity, never proof content, in audit/explanation metadata

### Authorization consumption and revocation ordering

Permission Authority defines one conceptual ordering point for proof use versus revocation.

- A protected effect requires an authorized use/consumption bound to that exact effect.
- Revocation prevents every use whose ordering point occurs after revocation.
- A use ordered before revocation may complete only the already bounded effect; it grants no later effect.
- If an operation has multiple independently meaningful effects, each effect requires a separate use/consumption immediately before that effect unless the owner can commit all effects atomically.
- Revocation events are notification; they do not replace authoritative validation/consumption.
- Partial effects caused by revocation during a multi-effect operation are recorded and explained.

### Revocation

- Revocation blocks new protected effects immediately.
- Unused proofs become invalid.
- Reads/subscriptions stop when ongoing authorization is no longer valid.
- In-flight work follows the owning capability's safe cancellation policy.
- Revocation does not erase already committed effects; those effects remain explainable.
- Revocation cannot be ignored because a command was previously accepted.

### Safety-reducing operations

Pause, stop, and safe cancellation:

- require owning task/session/operation identity
- use the owner-issued operation/session reference, which grants no authority
- require the independently governed Permission Authority-issued operation-control proof bound to the originating user/task/operation
- remain available for safety reduction after effect permission is revoked or Permission Authority is temporarily unavailable
- are denied on user/task/operation identity mismatch, but not merely because effect authority was revoked
- safely pause/cancel active work where possible when the operation-control proof itself is revoked or its offline lease expires; otherwise prevent subsequent effects and report too-late/unsafe
- must not require permission to continue the risky operation
- may not expand scope or create new effects
- must be accepted when safely possible
- must report when too late or unsafe

### Automatic policy

An automatic-execution policy:

- is explicit, user-controlled, revocable, and narrowly scoped
- changes authorization decision behavior only
- does not bypass authorize/validate
- does not permit proof replay or context drift

---

## Explainability Metadata

Every consequential, protected, denied, degraded, timed-out, cancelled, partial, or failed interaction includes an explanation reference or envelope with:

- what is happening or happened
- why
- responsible capability
- requester capability
- user-visible target summary
- current status
- task and operation identity where applicable
- permission scope and authorization identity where applicable
- whether data may leave the device
- whether an extension is involved
- uncertainty/partial-effect statement
- available user action

Explainability must:

- never expose proof content, secrets, raw sensing data, hidden prompts, unrelated memory, or unrelated grants
- distinguish waiting from active execution
- distinguish acceptance from completion
- distinguish failure from unknown outcome
- name optional cloud use before data leaves the device
- preserve domain meaning when Experience presents it

---

## State References

A state reference is a conceptual, immutable pointer to capability-owned state. It contains:

- owning capability
- aggregate/item identity
- owner-observed revision
- workspace scope when applicable
- minimum classification needed to enforce access

A state reference:

- is not a shared mutable object
- is not authority
- does not embed protected content
- must be resolved through the owner's public contract
- requires fresh permission validation when protected
- becomes stale when owner revision changes
- cannot be used to write state directly

Raw sensing buffers, authorization proofs, provider secrets, and extension partition contents are prohibited state references.

---

## Error Categories

Every error has one primary category:

### Validation

Input violates contract meaning, required metadata, invariant, or supported operation.

### Authorization

Permission required, denied, expired, revoked, replayed, or mismatched.

### Scope

Workspace, target, task, session, operation, or extension identity does not match.

### Conflict

Requested transition is valid in general but conflicts with current owned state.

### Unavailable

Capability or required local prerequisite is unavailable.

### Offline optional service

Only an optional internet enhancement is unavailable. This must not be reported as loss of local core capability.

### Timeout

Requester stopped waiting without evidence of receiver cancellation or failure.

### Cancellation

Cancellation accepted, too late, unsafe, or completed.

### Partial effect

Some effects committed and others did not. Exact known effects are minimized and explained.

### Outcome unknown

Evidence cannot determine terminal outcome. No automatic side-effect retry.

### Indeterminate

The owner exhausted declared recovery and cannot determine its own terminal effect. Known partial evidence is preserved; dependent effects and automatic retry remain blocked.

### Dependency

A permitted dependency failed or was unavailable; no routing around trust/permission boundaries.

### Internal

The owner failed without a safer public category. The user receives a safe explanation and correlation reference.

Errors always state responsible capability, safe message, retryability, correlation, and explanation. Error detail obeys privacy minimization.

---

## Cancellation Semantics

Every consequential command declares one cancellation class:

- **Not started — fully cancellable**
- **Active — safely cancellable**
- **Active — cancellation requested, completion race possible**
- **Irreversible — not cancellable after stated commit point**

Cancellation:

- is a distinct command correlated to the operation
- is idempotent
- never implies rollback
- does not erase partial effects
- produces a terminal cancellation or too-late/failed result
- preserves authorization and effect traceability
- is always presented honestly

Where compensation/undo exists, it is a new explicitly authorized command, not cancellation.

---

## Timeouts

Every request and non-immediate command declares a conceptual waiting policy:

- expected responsiveness class (immediate, interactive, background, lifecycle)
- requester waiting limit
- owner operation limit where one exists
- status/recovery path after waiting expires
- user explanation threshold

Timeout values are implementation decisions; their architectural semantics are fixed here.

Timeout:

- never equals cancellation
- never equals permission to retry
- never triggers silent cloud fallback
- never changes owner authority
- results in unknown/degraded state until authoritative evidence arrives

---

## Retry Philosophy

Retries are conservative, bounded, visible when consequential, and never broaden authority.

### Requests

Read-only requests may retry when:

- purpose, scope, and authorization remain valid
- the same correlation/attempt lineage is preserved
- retry does not disclose data to a new receiver

### Commands

A command may retry automatically only when:

- the contract declares it idempotent/deduplicated
- semantic content is unchanged
- the same message identity is used for delivery retry
- authorization remains valid
- owner can return prior acceptance/outcome

Otherwise retry requires:

- authoritative status/reconciliation first
- a new user-visible attempt
- a new message identity
- fresh authorization where context/lifetime changed

### Never retry automatically

- unknown-outcome environment mutations
- partial effects
- denied/revoked operations
- deep sensing after permission loss
- remote inference after offline/denied response
- extension effects after extension failure/revocation

---

## Idempotency Expectations

Every request/command declares one idempotency expectation:

- **Read-only repeatable** — repeated evaluation creates no effect.
- **Deduplicated effect** — same message identity yields at most one owned effect.
- **State-setting** — repeated identical desired state has no additional effect.
- **Non-repeatable** — retry forbidden until reconciliation/new authorization.

Events are deduplicated by message identity and never produce domain effects without a separately authorized command.

Permission validation is repeatable for the same context but may change from valid to invalid due to expiry/revocation. A prior valid response is not cacheable authority beyond its declared conditions.

---

## Auditability

Every consequential interaction must leave enough separately owned metadata to establish:

- who/requesting capability initiated it
- a content-free purpose class and correlation to the in-flight explanation
- contract/version used
- permission scope and authorization identity
- responsible capability
- operation identity
- accepted/rejected/terminal status
- known effects or outcome unknown
- timing and causal/correlation identities
- extension/provider class when applicable

Auditability must not create an alternate Memory store:

- Runtime Host keeps domain-free lifecycle/health metadata.
- Permission Authority keeps permission decisions.
- Domain owners keep metadata-only operation records and minimized effect summaries.
- Memory alone retains durable user knowledge.
- Intelligence stores no prompt/response content.
- Extension Host stores no user knowledge/task/prompt/observation content.

Audit records:

- use controlled purpose/effect classes rather than free-text user intent, target names, document content, prompts, observations, or responses
- retain only minimized identifiers and classifications needed for accountability
- apply explicit retention bounds and access controls
- support redaction/deletion of metadata where policy permits
- require a separately authorized Memory proposal if a user wants durable human-readable task history or detailed rationale

Historical traceability may establish that an authorized purpose class occurred without retaining the original user content. Detailed long-term “why” is available only when retained through Memory under its policies.

Audit retention and access require explicit future policy; indefinite retention is not implied.

---

## Traceability

For a consequential user task, architecture must be able to trace:

1. user intent or explicit administration action
2. Companion task (when mediated)
3. authorization request/challenge/decision
4. authorization identity
5. protected capability request/command
6. point-of-use validation
7. owner operation identity
8. progress/terminal event or authoritative status
9. explanation presented by Experience

Traceability is reconstructed from correlation/causation identities across separately owned metadata. It does not require a central content log.

---

## Versioning Strategy

### Version dimensions

Every contract declares:

- **Breaking version** — changes only when an existing valid participant could misinterpret meaning or violate an invariant.
- **Compatible revision** — changes when meaning is preserved and existing participants can safely continue.

Every message declares both.

### Breaking changes

These require a new breaking version:

- changing ownership or authority
- changing allowed initiator/receiver
- changing command/request/event kind
- adding a required permission or removing one without an explicit migration
- changing required field meaning
- removing or renaming required conceptual information
- changing terminal-state meaning
- weakening privacy/minimization
- changing idempotency/cancellation semantics
- making optional cloud behavior required

### Compatible changes

These may use a compatible revision only when safe:

- adding optional minimized metadata
- adding a new error detail under an existing category
- adding a new terminal explanation that preserves status meaning
- tightening documentation without changing valid behavior
- adding a new separately identified operation without changing existing operations

### Unknown information

- Unknown optional metadata is ignored only if ignoring it cannot affect permission, ownership, privacy, idempotency, cancellation, or outcome meaning.
- Unknown permission, message kind, terminal status, or required invariant fails closed.
- Unknown event facts are not converted into commands.
- An unknown terminal event is quarantined from state transition, moves the requester to outcome-unknown, blocks dependent effects/retry, triggers an authorized owner-status reconciliation, and produces a compatibility explanation.

### Compatibility window

During migration:

- producer and consumer declare supported breaking versions
- only a mutually understood version is used
- old and new versions may coexist for a bounded transition
- one message is interpreted under exactly one version
- translation must not broaden permission, subscribers, data, or effects
- unsupported protected interactions fail closed with explanation

### Deprecation

Deprecation records:

- replacement contract/version
- reason
- affected capabilities
- compatibility period
- removal condition
- migration validation evidence

No contract is removed while an active capability depends on it without an approved architecture update.

---

## Compatibility Expectations

Every contract evolution must preserve:

- capability ownership
- permitted communication paths
- permission binding and point-of-use validation
- Privacy First minimization
- Local First core behavior
- event non-authority
- Intelligence non-execution
- Experience presentation-only behavior
- Memory's sole durable user-knowledge ownership
- explainability status meanings

Compatibility is not merely successful parsing. A participant is incompatible if it could parse a message but misapply authority, scope, state, outcome, or privacy meaning.

---

## Architectural Acceptance Specification

### Admission rule

A future capability contract is architecturally accepted only when:

1. its owner and primary responsibility are named in Capability Architecture
2. initiator/receiver/event subscribers are permitted by Interaction Matrix
3. purpose exists in Capability Contracts or is added through an architecture update
4. conceptual schema fields and presence classes are documented
5. permission, privacy, failure, cancellation, timeout, retry, idempotency, audit, trace, and version semantics are specified
6. acceptance cases below pass conceptually
7. Current State and Engineering Ledger record the result

No implementation may define missing architecture by accident.

---

## Required Invariants

Every contract must prove:

### Ownership

- Exactly one capability owns each state transition and authoritative result.
- Responses/events do not transfer ownership.
- Shared mutable state is prohibited.
- Durable user knowledge is owned only by Memory.

### Communication

- Initiator, receiver, and subscribers are explicit.
- Every interaction uses a defined contract identity/version.
- Unlisted direct communication is forbidden.
- Required intermediary paths cannot be bypassed.

### Authority

- Permission Authority alone grants/revokes/validates permission truth.
- Protected owner validates at point of use.
- Events, correlation ids, state references, and explanations grant no authority.
- Intelligence proposals and Extension contributions never execute directly.

### State

- State references are immutable and owner-resolved.
- Stale revisions cannot silently overwrite current state.
- Each terminal result is owned and evidenced.

### Lifecycle

- Command acceptance and completion are distinct.
- Non-immediate command has one owner-authoritative terminal state.
- Requester-observed outcome unknown remains uncertainty until reconciled with the owner; it is never treated as a second terminal state.
- Shutdown/revocation prevents new protected effects.

---

## Required Metadata

Before acceptance, every message kind documents presence for:

- message identity
- contract identity
- breaking version and compatible revision
- message kind
- creation time
- requester capability
- receiver capability or event subscriber class
- purpose
- correlation identity
- causation identity condition
- task identity condition
- operation identity introduction/condition
- operation reference introduction/condition for non-immediate domain commands
- authorization identity condition
- extension identity condition
- workspace scope condition
- aggregate identity/sequence condition
- waiting/deadline policy where applicable
- idempotency expectation
- cancellation class for consequential commands
- privacy classification/minimization statement
- explanation reference/envelope condition

---

## Required Permission Validation

Acceptance requires cases proving:

- no-proof protected operation is denied
- wrong requester is denied
- wrong subject is denied
- wrong purpose is denied
- wrong target/operation is denied
- wrong workspace scope is denied
- wrong extension identity is denied
- expired proof is denied
- revoked proof is denied
- replay contrary to proof conditions is denied
- proof valid at orchestration but invalid at commit is denied
- automatic policy still authorizes and validates
- pause/stop/cancel can reduce risk without permission to continue the risky action
- event/state reference cannot substitute for proof

---

## Required Explainability Information

Acceptance requires user-visible evidence for:

- requested
- challenge required
- denied
- waiting
- active
- timed out
- degraded/unavailable
- partially completed
- outcome unknown
- indeterminate
- cancelled
- failed
- completed

Each explanation identifies:

- what
- why
- responsible capability
- permission scope/authorization identity when applicable
- target summary
- local/optional-remote boundary
- extension identity when applicable
- known partial effects/uncertainty
- available user action

---

## Failure Expectations

Every contract defines:

- fail-open or fail-closed decision (protected operations are fail-closed)
- dependency-unavailable behavior
- local-prerequisite-unavailable behavior
- optional-online-unavailable behavior
- timeout behavior
- cancellation behavior
- partial-effect behavior
- outcome-unknown behavior
- owner-indeterminate behavior and evidence/recovery threshold
- duplicate/out-of-order/missing event behavior
- retry eligibility
- user explanation threshold

No failure may silently:

- broaden permission
- switch to cloud
- retain additional data
- call a forbidden capability
- assume success
- repeat an unknown side effect

---

## Privacy Expectations

Acceptance requires:

- purpose limitation
- minimum necessary payload
- raw sensing confined to Context Sensing
- proof/secret exclusion from events/explanations
- prompt/response content excluded from Intelligence persistence
- user/task content excluded from Companion metadata
- user knowledge excluded from Action/Extension/runtime logs
- unrelated workspace/memory/grant data excluded
- retention owner and deletion/redaction path identified
- remote data-sharing permission explicit before transfer

Deep sensing authorization never waives minimization.

---

## Local First Expectations

Every core contract must:

- operate without internet when local prerequisites exist
- define local unavailable/degraded outcomes
- avoid remote dependencies for authorization, workspace, memory, local sensing, local action, host, and core Experience
- keep remote Intelligence optional
- prohibit silent remote fallback
- preserve local permission/audit/explanation paths
- remain usable after optional cloud failure

Acceptance must cover all offline core scenarios in Capability Architecture.

---

## Blueprint Compliance Gate

Every contract review records how it:

- supports Local First
- supports Human First
- supports Privacy First
- enforces Permission Before Automation
- explains every consequential action/state
- builds only unique Workspace value
- leaves commodity implementation for research/integration
- stays within the Complexity Budget
- keeps engineering documentation outside runtime
- reduces rather than increases user cognitive load

A contract that cannot demonstrate these is rejected or simplified.

---

## ADR Compliance Gate

Every contract review explicitly records:

- **ADR-0001:** offline behavior and optional cloud boundary
- **ADR-0002:** no unresearched commodity implementation decision
- **ADR-0003:** explicit, revocable, explainable authorization
- **ADR-0004:** contract/subsystem complexity justification
- **ADR-0005:** one owner, no responsibility overlap
- **ADR-0006:** what/why/permission/capability explanation
- **ADR-0007:** architecture specification does not become runtime logic
- **ADR-0008:** artifact contains enough authority/context for a new AI session

---

## Required Acceptance Cases

Each future contract supplies conceptual Given/When/Then cases. At minimum:

### Identity and correlation

1. Given a request, when its response occurs, then correlation is stable, causation references the request, and no terminal event is required.
2. Given a non-immediate command, when acceptance and its owner terminal event occur, then correlation is stable and causation reconstructs the path.
3. Given a delivery retry, when the same message is redelivered, then message identity is unchanged and no duplicate effect occurs.
4. Given changed purpose/target/content, when retried, then it is a new attempt with new identity and authorization evaluation.

### Permission

5. Given no/invalid proof, when protected access/effect is attempted, then owner denies at point of use.
6. Given a proof valid earlier but revoked before its use/consumption ordering point, when commit is attempted, then no new effect occurs.
7. Given a bounded proof use ordered before revocation, when revocation follows, then at most that already bounded effect may complete and no later effect begins.
8. Given a multi-effect operation, when revocation occurs between effects, then later effects are blocked and committed partial effects are explained.
9. Given a challenge decision event, when the requester needs authority, then it repeats authorization with original context/challenge and never treats the event as proof.
10. Given a valid proof for one target, when used for another, then validation fails.
11. Given automatic policy, when operation begins, then authorize/validate still occurs and remains explainable.

### Commands and outcomes

12. Given command acceptance, when no terminal evidence exists, then UI never shows completed.
13. Given partial effects, when operation ends, then known effects and uncertainty are explained and no blind retry occurs.
14. Given lost terminal event, when waiting expires, then requester state becomes outcome-unknown, owner status reconciliation occurs, and a later owner terminal result (including indeterminate after exhausted recovery) replaces uncertainty rather than creating a second terminal outcome.
15. Given a lost acceptance response, when the same command identity is redelivered, then the owner returns the same operation identity/reference and creates no duplicate operation.
16. Given effect authority expired/revoked or Permission Authority is temporarily unavailable, when the originating task presents its bound operation reference/control proof, then the owner returns minimized authoritative status without authorizing new effects.

### Cancellation

17. Given an operation not started, when the owning task presents its operation reference and operation-control proof, then no effect begins and terminal cancellation is emitted.
18. Given effect permission was revoked or Permission Authority is temporarily unavailable, when the originating task presents its bound operation-control proof, then safety cancellation remains available without authorizing new effects.
19. Given an active cancellable operation, when cancellation races completion, then exactly one authoritative terminal outcome is exposed.
20. Given irreversible commit, when cancellation arrives too late, then exact known effect and too-late status are explained.

### Events

21. Given duplicate event, when consumer receives it, then no duplicate state/effect occurs.
22. Given out-of-order aggregate events, when an older sequence arrives, then current state is not rolled back.
23. Given a sequence gap, when current state cannot be proven, then state is stale/unknown and queried from owner.
24. Given an unknown terminal event version/status, when received, then it is quarantined, dependent effects stop, owner status is queried, and incompatibility is explained.

### Privacy

25. Given deep sensing, when context crosses boundary, then raw/unminimized data remains inside Context Sensing.
26. Given an explanation/error, when presented, then no proof, secret, prompt, raw sensing, unrelated memory/grant, or extension partition data appears.
27. Given content proposed for durable retention, when owner is not Memory, then retention is rejected or routed through authorized Memory proposal.
28. Given an audit record, when inspected, then purpose/effect are content-free classes, retention is bounded, and no user intent/target/content can be reconstructed outside Memory.

### Local First

29. Given no internet, when a core local contract is invoked with local prerequisites, then it remains available.
30. Given optional remote Intelligence offline, when requested, then local-only/degraded outcome occurs with no silent remote fallback.
31. Given cloud failure, when permission/workspace/memory/local action/sensing/status is used, then those local paths remain available.

### Boundaries

32. Given Intelligence proposal, when an action is desired, then Companion mediation and fresh authorization occur before Action.
33. Given Experience input, when a domain mutation is desired, then Experience does not mutate or report domain success itself.
34. Given extension contribution, when a domain effect is desired, then extension identity survives Companion mediation and fresh authorization.

### Compatibility

35. Given compatible optional metadata, when an older participant ignores it safely, then authority/privacy/outcome meaning remains unchanged.
36. Given unknown required permission/status/invariant, when received, then interaction fails closed.
37. Given breaking version mismatch, when no mutually supported version exists, then protected interaction is refused with explanation.

### Audit and trace

38. Given a completed consequential task, when audited, then purpose class → authorization → validation/consumption → operation → terminal result → explanation presentation is traceable without centralizing user content.
39. Given audit deletion/retention policy, when metadata expires, then Memory content and domain ownership are unaffected.

### Control-proof revocation and delayed recovery

40. Given user revocation of an operation-control proof, when the owner receives it, then active work safely pauses/cancels where possible before control invalidation; an irreversible effect may complete, but no subsequent effect begins and too-late/unsafe status is explained.
41. Given control revocation delivery is missed during an Authority outage, when the bounded offline lease remains active, then only minimized status/safety control is possible; at lease expiry nonterminal work safely pauses/cancels where possible, otherwise later effects stop and too-late/unsafe is reported.
42. Given reconciliation begins after control-proof expiry, when a fresh user-administration proof is presented, then the owner returns its content-free terminal tombstone or an honest expired-history result; neither permits automatic retry.

---

## Contract Acceptance Record

Each contract's architecture acceptance record contains:

- contract identity and version
- owner
- allowed initiators/subscribers
- purpose
- message kinds
- conceptual field/presence catalogue
- permission scopes and validation point
- state owner/references
- failure/cancellation/timeout/retry/idempotency rules
- privacy/retention classification
- Local First behavior
- explainability mapping
- compatibility/deprecation assessment
- required acceptance cases and outcomes
- Blueprint/ADR compliance
- unresolved risks
- approval status and date
- Engineering Ledger reference

This is documentation evidence, not runtime configuration.

---

## Internal Consistency Review

### Ambiguity reviewed

- **Operation/authorization identities on initial messages:** clarified as owner-introduced, not empty required placeholders.
- **Timeout versus cancellation:** explicitly separate.
- **Acceptance versus completion:** explicitly separate.
- **Retry versus redelivery:** redelivery keeps identity; changed/new attempt gets new identity.
- **Cancellation after revocation:** safety-reducing cancellation does not require authority to continue the original effect.
- **Compatibility versus parsing:** semantic authority/privacy compatibility is required, not syntactic readability.

### Conflicts reviewed

- Contract Schema does not change capability ownership or matrix paths.
- Events never issue commands or permission.
- Permission proof remains opaque and excluded from events/explanations.
- State references do not create shared ownership.
- Auditability does not create central content storage or alternate Memory.
- Optional remote behavior cannot become core dependency.

### Complexity reviewed

- One shared conceptual envelope replaces per-capability metadata reinvention.
- Presence classes avoid meaningless placeholder fields.
- One error taxonomy permits capability-specific extension without duplicate semantics.
- One acceptance suite applies to all contracts; capability-specific cases add only distinct risk.
- No transport, encoding, framework, or programming-language model is introduced.

### Missing rules reviewed

Rules now cover identity, correlation, causation, commands, events, requests/responses, errors, permission, explanation, state references, versions, compatibility, cancellation, timeouts, retries, idempotency, auditability, traceability, privacy, Local First, and Blueprint/ADR gates.

### Validation result

- No ambiguous required metadata remains at architecture level.
- No conflicting command/event/request semantics remain.
- No ownership or authority transfer is introduced.
- No Permission Authority bypass is introduced.
- No unnecessary subsystem is introduced.
- Specification is internally consistent with Blueprint, accepted ADRs, Capability Architecture, Interaction Matrix, and Capability Contracts.

---

## Remaining Pre-implementation Risks

- Permission scope taxonomy still requires capability-by-capability refinement.
- Concrete waiting limits remain implementation decisions constrained by this specification.
- Each action class still needs specific irreversible commit points and compensation rules.
- Event recovery and compatibility cases require executable validation artifacts.
- Extension Host remains dormant until separately justified under ADR-0004.

Recommended next milestone: **Capability Technology Research** — investigate candidate categories for each capability, record each investigation once in the Research Catalogue, and make no implementation choice without architecture/contract acceptance evidence.

