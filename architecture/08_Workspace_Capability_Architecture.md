# Workspace Capability Architecture v1.1

Status: Active
Authority: Authoritative product capability decomposition
Version: 1.1

This document defines the foundational capability architecture for Workspace.
It does not select technologies, libraries, or vendors.
Accepted ADRs remain binding. The Blueprint remains highest authority.
Permitted communication paths and required intermediaries are defined by `09_Capability_Interaction_Matrix.md`.

---

## Design Intent

Workspace is an intelligent operating environment composed of capabilities.
Capabilities communicate only through contracts.
Each capability has one primary responsibility.
Custom software is reserved for unique Workspace value; commodity functions are integration categories for later research (ADR-0002).

### Capability Map (summary)

| Capability | Primary responsibility |
|------------|------------------------|
| Runtime Host | Local process lifecycle and capability wiring |
| Permission Authority | Explicit, revocable, explainable permission control |
| Workspace Management | User workspaces, zones, and work organization |
| Memory | What is remembered, recalled, and forgotten |
| Context Sensing | Read-only observation of the user's computing context |
| Action | Execution of permitted environment changes |
| Intelligence | Local-first reasoning and generation |
| Companion Orchestration | Intent interpretation and multi-capability coordination |
| Experience | Human-facing interaction and presentation |
| Extension Host | Optional, dormant boundary for controlled third-party capability extensions |

---

## Runtime Host

### Purpose

Provide the local application substrate that starts, wires, persists configuration for, and shuts down capabilities so Workspace can operate offline as a coherent system.

### Owns

- Process/application lifecycle
- Capability registration and health
- Local configuration required for hosting
- Ordered startup and shutdown
- Offline/online operating mode signals (connectivity presence only)

### Does Not Own

- User permissions (Permission Authority)
- Companion decisions (Companion Orchestration)
- UI presentation (Experience)
- Memory contents (Memory)
- Desktop observation or automation (Context Sensing / Action)
- Model inference (Intelligence)

### Responsibilities

- Boot Workspace locally
- Register capabilities and expose contract routing
- Signal degraded/offline mode
- Coordinate graceful shutdown
- Isolate engineering documentation from runtime packaging (ADR-0007)

### Inputs

- OS process start/stop
- Capability registration descriptors
- Host configuration
- Health/failure reports from capabilities

### Outputs

- Capability ready / unavailable events
- Offline/online mode events
- Shutdown directives
- Host health state

### Contracts

- `Host.lifecycle` — start, ready, pause, shutdown
- `Host.mode` — offline | online-optional
- `Host.registry` — capability availability queries

### Dependencies

- None (root runtime dependency)

### Permissions

- OS-level application permissions required to run as a local desktop process.
- Does not grant product-level automation permissions to other capabilities.

### Data Ownership

- Host configuration
- Capability registry state
- Runtime health snapshots

### Lifecycle

- Creation: OS launches Workspace process
- Runtime: maintains registry and mode; restarts failed capabilities only when safe and declared
- Persistence: host configuration only
- Shutdown: ordered capability shutdown, then process exit

### Failure Behaviour

- If host fails, Workspace stops; no silent background autonomy.
- If a non-critical capability fails, host marks it unavailable and emits explainable degradation.
- Core local operation must not require internet (ADR-0001).

### Explainability

- Surfaces host-level status via Experience: starting, ready, offline mode, capability unavailable, shutting down.

### Future Research Required

- Capability isolation boundaries (process vs in-process)
- Crash recovery policy granularity
- Update/migration of host configuration

### Candidate Open Source Categories

- Application Runtime Framework
- Local Configuration Store
- Structured Logging Framework

### Acceptance Criteria

- Ordered local startup/shutdown defined and testable
- Capability availability is queryable
- Offline mode does not block core host operation
- No product permission or companion logic embedded in host

---

## Permission Authority

### Purpose

Ensure meaningful Workspace actions occur only with explicit, revocable, explainable user permission (ADR-0003).

### Owns

- Permission catalogue
- Grant, deny, revoke, and expiry
- Permission checks for other capabilities
- Permission audit trail
- User-configured automatic-execution exceptions

### Does Not Own

- Performing the action being permitted (Action / others)
- Deciding whether an action is desirable (Companion Orchestration)
- Rendering consent UI chrome beyond permission prompts contracts (Experience renders; Authority defines prompt content)

### Responsibilities

- Define permission scopes
- Evaluate allow/deny for requested operations
- Issue purpose-, capability-, subject-, target-, and operation-bound authorization proofs for approved meaningful operations
- Record why a permission was used
- Support revocation at any time
- Enforce “permission before automation” defaults

### Inputs

- Authorization requests from Companion Orchestration and Experience; point-of-use validation requests from protected capabilities
- User grant/deny/revoke decisions
- Automatic-execution policy updates from the user

### Outputs

- Allow / deny decisions and bound authorization proofs
- Permission-required challenges
- Revocation events
- Audit records (what permission, which capability, why)

### Contracts

- `Permission.authorize(scope, capability, purpose, subject, target, operation) -> proof | deny | challenge`
- `Permission.validateForUse(proof, exact_effect_context) -> authorized_use | invalid`
- `Permission.grant` / `Permission.revoke`
- `Permission.explain(decision_id)`
- `Permission.catalogue`

### Dependencies

- Runtime Host (persistence and availability)

### Permissions

- Meta-permission to manage the permission store (user-controlled).
- Cannot be bypassed by Extension Host or Action.

### Data Ownership

- Permission grants and policies
- Permission audit log
- Automatic-execution exception list

### Lifecycle

- Creation: default-deny catalogue seeded at first run
- Runtime: authorize every meaningful operation; order each proof use/consumption against revocation at the point of access or each independently meaningful effect
- Persistence: grants/policies/audit retained locally
- Shutdown: flush audit; deny in-flight challenges

### Failure Behaviour

- Fail closed: if Permission Authority is unavailable, new meaningful actions are denied. An owner may still validate a previously Permission Authority-issued, operation-bound control proof for minimized status or safety-reducing pause/stop/cancel; it cannot create effects.
- Challenges that cannot be presented remain denied.
- Revocation invalidates unused proofs and prevents subsequent operations; in-flight cancellation follows the owning capability's declared safety policy.

### Explainability

- Every decision can answer: what was requested, why, which capability asked, which grant applied, and how to revoke.

### Future Research Required

- Permission scope taxonomy granularity
- Time-bounded and context-bounded grants
- Safe defaults for first-run experience

### Candidate Open Source Categories

- Policy Evaluation Engine
- Local Audit Log Store
- Secure Local Credential Store (if secrets attach to grants)

### Acceptance Criteria

- Default-deny for meaningful actions
- Explicit grant/revoke paths exist
- Decisions are explainable and auditable
- No capability can self-authorize around this boundary

---

## Workspace Management

### Purpose

Organize the user’s work into durable workspaces and zones so Companion assistance stays scoped and cognitively light.

### Owns

- Workspace and zone definitions
- Membership of work items at the organizational level
- Active workspace/zone selection
- Workspace-scoped labels and structure

### Does Not Own

- Episodic/semantic memory contents (Memory)
- Desktop window geometry control (Action / OS)
- Companion task planning (Companion Orchestration)
- File contents indexing strategy (may request Context Sensing / Memory later)

### Responsibilities

- Create, update, archive workspaces and zones
- Track the active workspace context
- Provide scope boundaries for other capabilities
- Preserve user organizational intent

### Inputs

- User workspace/zone commands mediated by Companion Orchestration
- Companion requests for current scope
- Import/structure hints (non-authoritative)

### Outputs

- Workspace/zone state changes
- Active-scope events
- Scope query responses

### Contracts

- `Workspace.getActive(authorization_proof)`
- `Workspace.list(authorization_proof)`
- `Workspace.mutate(change, authorization_proof)` — validates the proof at the mutation commit point
- `Workspace.scopeFor(capabilityRequest, authorization_proof)`

### Dependencies

- Runtime Host
- Permission Authority (all protected reads and mutations)

### Permissions

- `workspace.read` — observe structure
- `workspace.write` — create/modify/archive structure

### Data Ownership

- Workspace and zone records
- Active workspace pointer
- Organizational metadata

### Lifecycle

- Creation: default personal workspace on first run
- Runtime: serves scope to dependents
- Persistence: durable local structure
- Shutdown: persist active selection

### Failure Behaviour

- If unavailable, dependents operate in a constrained “unscoped” mode with reduced automation and clear explanation.
- Mutations fail closed without Permission Authority.
- Reads and mutations reject expired, revoked, replayed, or context-mismatched authorization proofs at the access/commit point.

### Explainability

- States which workspace/zone is active and when scope changes.

### Future Research Required

- Zone model depth
- Multi-workspace simultaneous presence
- Relationship between OS folders and Workspace zones

### Candidate Open Source Categories

- Local Document Database
- Hierarchical Data Model Utilities

### Acceptance Criteria

- Active scope always defined or explicitly unscoped
- Other capabilities consume scope only via contract
- No memory or action semantics embedded here

---

## Memory

### Purpose

Remember appropriately: store, recall, and forget user-relevant knowledge under permission and scope, reducing repeated cognitive load without becoming surveillance.

### Owns

- Memory items and collections
- Retention, forgetting, and redaction policies
- Retrieval queries for companion context
- Memory provenance (what source produced an item)

### Does Not Own

- Embedding engine implementation (integration category)
- Raw continuous desktop capture archives (Context Sensing may emit candidates; Memory accepts only approved items)
- Model weights (Intelligence)
- Permission grants (Permission Authority)

### Responsibilities

- Accept memory write proposals (user or companion-mediated)
- Retrieve relevant memories for a scoped request
- Forget/redact on user request or policy
- Enforce workspace scope on memory access

### Inputs

- Memory write proposals
- Retrieval requests from Companion Orchestration
- Forget/redact commands
- Workspace scope

### Outputs

- Retrieved memory sets with provenance
- Write accepted/rejected
- Forget confirmations
- Memory policy events

### Contracts

- `Memory.proposeWrite(candidate, authorization_proof)`
- `Memory.retrieve(query, scope, purpose, authorization_proof)`
- `Memory.forget(item, authorization_proof)` / `Memory.redact(item, authorization_proof)`
- `Memory.explain(item_id, authorization_proof)`

### Dependencies

- Runtime Host
- Permission Authority
- Workspace Management (scope)

### Permissions

- `memory.read`
- `memory.write`
- `memory.forget`

### Data Ownership

- Memory items and provenance
- Retention/forget policies
- Retrieval audit of sensitive access (high-level)

### Lifecycle

- Creation: empty store with default retention policy
- Runtime: write/retrieve/forget under permission
- Persistence: local durable memory
- Shutdown: complete in-flight writes; no cloud flush required

### Failure Behaviour

- Retrieve failure → Companion continues with reduced context and explains memory unavailability.
- Write failure → proposal rejected; no silent retry loops that bypass permission.
- Fail closed on permission uncertainty.
- Reject expired, revoked, replayed, or context-mismatched authorization proofs at read/write/forget commit points.

### Explainability

- Can explain what was remembered, why, from what source, which permission applied, and how to forget.

### Future Research Required

- Memory taxonomy (episodic, preference, procedural)
- Relevance/retrieval quality approach
- Forgetting UX and retention defaults

### Candidate Open Source Categories

- Embeddings
- Vector Index
- Local Document Database
- Full-Text Search

### Acceptance Criteria

- Scoped, permissioned read/write/forget
- Provenance on every item
- No direct desktop capture pipeline owned here
- Offline recall of locally stored memory

---

## Context Sensing

### Purpose

Observe the user’s computing context in a read-only, permissioned way so Workspace can understand work without acting on it.

### Owns

- Observation sessions
- Context snapshots and derived signals (active application, focus heuristics, user-approved sensors)
- Sensing rate limits and pause state

### Does Not Own

- Changing OS/application state (Action)
- Deciding what to do about context (Companion Orchestration)
- Long-term memory retention (Memory; sensing may propose)
- UI shell (Experience)

### Responsibilities

- Start/stop observation under permission
- Emit purpose-limited, privacy-minimized context events; deep-sensing permission changes the permitted source class, never the minimization requirement
- Support on-demand context queries
- Never mutate the observed environment

### Inputs

- Start/stop observe requests
- Permission decisions
- On-demand context query from Companion
- User pause/resume sensing

### Outputs

- Context events/snapshots
- Sensing paused/denied/unavailable
- Memory write candidate events to Companion Orchestration (optional; never sent directly to Memory or persisted without an explicit orchestration and permission path)

### Contracts

- `Sense.start(sensor_class, purpose, effect_proof, operation_control_proof)` / `Sense.stop(session_id, operation_control_proof)` / `Sense.pause(session_id, operation_control_proof)`
- `Sense.queryCurrent(purpose, authorization_proof)`
- `Sense.events(authorization_proof)` (subscribe while authorization remains valid)
- `Sense.explain(sample_id, authorization_proof)`

### Dependencies

- Runtime Host
- Permission Authority
- Workspace Management (optional scope tags on events)

### Permissions

- `sense.observe.basic` (e.g. active app metadata)
- `sense.observe.deep` (e.g. screen content / OCR pathways) — separately grantable
- Default deny for deep observation

### Data Ownership

- Ephemeral observation buffers
- Sensing session state
- Minimization/redaction rules for emitted events

### Lifecycle

- Creation: sensing disabled until granted
- Runtime: emit only while authorization remains valid and sensing is not paused
- Persistence: prefer ephemeral; durable history only via Memory proposals
- Shutdown: stop sensors immediately; clear ephemeral buffers

### Failure Behaviour

- Sensor failure → unavailable signal; Companion degrades to user-provided context.
- Permission revoked → immediate stop and buffer clear.
- Reject expired, revoked, replayed, or context-mismatched authorization proofs before sensor activation and each on-demand deep query.

### Explainability

- States: observing, paused, permission required, what class of data is being observed, which capability requested it.

### Future Research Required

- Desktop observation stack options
- Minimum viable signals for useful assistance
- OCR/screen content privacy controls

### Candidate Open Source Categories

- Window Management (read APIs)
- Accessibility APIs
- OCR
- Screen Capture Framework
- Activity Signal Collectors

### Acceptance Criteria

- Strict read-only boundary (no action APIs)
- Deep vs basic observation separately permissioned
- Immediate stop on revoke
- Offline operation for local sensors

---

## Action

### Purpose

Execute permitted changes in the user’s environment—automation with a hard permission gate and clear accountability.

### Owns

- Action plans at the execution layer (atomic/controlled operations)
- Execution results and failure details
- Action allow-list binding to permission scopes

### Does Not Own

- Choosing goals or strategies (Companion Orchestration)
- Observation (Context Sensing)
- Permission decisions (Permission Authority)
- User-facing narration beyond action status events (Experience presents)

### Responsibilities

- Validate bound authorization proofs against the exact execution context immediately before execution
- Execute only declared action types
- Report success, partial success, or failure
- Support cancellation where feasible
- Remain inert without grants

### Inputs

- Action requests from Companion Orchestration (or user-direct via Experience → Companion)
- Purpose-, capability-, subject-, target-, and operation-bound authorization proofs
- Cancel requests

### Outputs

- Action completed / partially completed / indeterminate / failed / cancelled events
- Environment change confirmations
- Execution explain payloads

### Contracts

- `Action.execute(request, effect_proof, operation_control_proof)`
- `Action.cancel(operation_id, operation_reference, operation_control_proof)` — reference grants no authority; control proof is Permission Authority-issued and permits only status/safety reduction
- `Action.describe(action_type, authorization_proof)`
- `Action.events`

### Dependencies

- Runtime Host
- Permission Authority (mandatory)
- Workspace Management (optional targeting/scope validation)

### Permissions

- Per action-class permissions (examples of classes, not tech): `action.window`, `action.files`, `action.apps`, `action.input.simulate`
- No omnibus “do anything” grant in the architecture

### Data Ownership

- Metadata-only execution audit and minimized effect summaries (local); never copied user content
- Action type catalogue
- In-flight execution state

### Lifecycle

- Creation: catalogue loaded; executor idle
- Runtime: execute only with an authorization proof validated against the exact execution context
- Persistence: execution audit retained locally per policy
- Shutdown: cancel or safely complete in-flight actions; no new accepts

### Failure Behaviour

- Fail closed without permission.
- Reject expired, revoked, replayed, or context-mismatched authorization proofs.
- Partial failures reported honestly; no hidden retries that expand scope.
- Target app missing → failed with explanation.

### Explainability

- Before/during/after: what action, why (purpose from requester), which permission, target, result.

### Future Research Required

- Safe action taxonomy for desktops
- Confirmation thresholds by risk class
- Undo/compensation patterns

### Candidate Open Source Categories

- Window Management (write APIs)
- UI Automation Framework
- Process Launch Control
- Notification Framework

### Acceptance Criteria

- No execution path bypasses Permission Authority
- Action classes are enumerable and explainable
- Cancellation policy defined
- Separated from sensing APIs

---

## Intelligence

### Purpose

Provide local-first reasoning and generation services so Companion Orchestration can understand, plan, and communicate—without owning product control flow.

### Owns

- Model/provider invocation contracts (local primary; optional remote under permission)
- Prompt/response transactions for callers
- Inference safety limits (length, timeout, local resource caps)
- Tool-calling proposals emitted as structured suggestions (not direct actions)

### Does Not Own

- Multi-capability workflow control (Companion Orchestration)
- Memory storage (Memory)
- Desktop sensing/acting (Context Sensing / Action)
- Final user-visible chrome (Experience)

### Responsibilities

- Accept reasoning/generation requests with purpose and scope
- Use only permitted providers
- Return structured results and uncertainty
- Emit tool/action proposals for Orchestration to evaluate—never self-execute environment actions
- Prefer local providers; degrade when unavailable

### Inputs

- Reasoning/generation requests from Companion Orchestration
- Optional memory snippets supplied by caller (caller retrieves from Memory)
- Provider permission state
- Offline mode from Runtime Host

### Outputs

- Completions / structured plans / classifications
- Tool or action proposals (non-executing)
- Provider unavailable / degraded mode
- Token/resource usage summaries (for explanation)

### Contracts

- `Intelligence.reason(request, authorization_proof)`
- `Intelligence.generate(request, authorization_proof)`
- `Intelligence.listProviders(authorization_proof)`
- `Intelligence.explain(transaction_id, authorization_proof)`

### Dependencies

- Runtime Host
- Permission Authority
- Does not depend on Action or Context Sensing directly (caller supplies context)

### Permissions

- `intelligence.local.run`
- `intelligence.remote.run` (optional cloud enhancement; default deny)
- Data-sharing permission when request payloads may leave the machine

### Data Ownership

- Inference transaction metadata only (provider class, requester, purpose, timing, resource use, correlation identifiers, outcome); never prompt, response, or memory content
- Provider configuration (non-secret and secret refs)
- Local resource cap settings

### Lifecycle

- Creation: local provider configuration discovered or marked unset
- Runtime: serve requests; refuse remote without permission
- Persistence: provider config and metadata-only transaction records; retained prompt/response content must be proposed to Memory through Companion
- Shutdown: abort in-flight inference; no remote flush required

### Failure Behaviour

- Local provider missing → degraded companion reasoning; explain gap; no silent cloud failover.
- Remote denied/offline → local-only or refuse with explanation.
- Never converts a proposal into an Action call itself.
- Reject expired, revoked, replayed, or context-mismatched authorization proofs before remote use or protected local inference.

### Explainability

- States: reasoning, waiting for model, using local/remote provider, permission required for remote, which capability requested inference.

### Future Research Required

- AI orchestration patterns
- Local model viability classes
- Structured tool-proposal schema
- Evaluation of quality vs resource use

### Candidate Open Source Categories

- LLM Orchestration
- Local Inference Runtime
- AI SDKs
- Prompt Templating
- Structured Output Validators

### Acceptance Criteria

- Local-first invocation path exists in architecture
- No direct Action execution from Intelligence
- Remote path is optional and permissioned
- Callers receive explainable degradation

---

## Companion Orchestration

### Purpose

Interpret user intent and coordinate capabilities to assist with work—calmly, permission-first, and without absorbing their responsibilities (unique Workspace value).

### Owns

- Intent interpretation outcomes
- Task/plan state for multi-step assistance
- Capability call sequencing
- When to ask the user vs proceed under existing grants
- Companion behavioural policy aligned to Blueprint tone

### Does Not Own

- UI rendering (Experience)
- Permission truth (Permission Authority)
- Memory storage (Memory)
- Raw sensing (Context Sensing)
- Environment mutation (Action)
- Model inference engines (Intelligence)
- Workspace structure (Workspace Management)

### Responsibilities

- Turn user intents into capability plans
- Request checks from Permission Authority before meaningful steps
- Pull context via Sense/Memory/Workspace contracts
- Ask Intelligence for reasoning; evaluate proposals
- Dispatch Action only after permission
- Publish explainable progress for Experience
- Stop when permission denied or user cancels

### Inputs

- User intents from Experience
- Context events (subscribed, minimized)
- Intelligence results and proposals
- Permission decisions
- Action results
- Memory retrievals
- Extension-contributed intents (via Extension Host, still permissioned)

### Outputs

- Capability requests (Sense, Memory, Intelligence, Action, Workspace)
- Optional Extension Host management requests after that capability has been separately activated
- Permission authorization requests to Authority
- Progress/explanation events to Experience
- User-interaction request events; responses return through the Companion contract
- Task completion / cancellation states

### Contracts

- `Companion.handleIntent(intent)`
- `Companion.submitInteractionResult(task_id, result)`
- `Companion.cancel(task_id)`
- `Companion.explain(task_id)`
- `Companion.events` (progress, questions, completions)

### Dependencies

- Runtime Host
- Permission Authority
- Workspace Management
- Memory
- Context Sensing
- Intelligence
- Action
- Extension Host (optional; only when separately activated, for extension management)

### Permissions

- Does not hold ambient superuser rights.
- Operates by requesting capability-scoped permissions on behalf of a task purpose.
- May use only grants the user has issued.

### Data Ownership

- Active task/plan state
- Intent history needed for in-flight work (minimized)
- Orchestration policy configuration

### Lifecycle

- Creation: idle companion ready after dependencies available
- Runtime: task-driven coordination
- Persistence: orchestration policy and metadata-only task outcome/correlation records; no intent, plan, response, or user-content history persists after task completion unless separately proposed to Memory
- Shutdown: cancel tasks; discard task content; persist only metadata-only outcome/correlation records

### Failure Behaviour

- Dependency unavailable → narrow assistance and explain which capability is missing.
- Permission denied → stop that path; never route around Authority.
- Intelligence degraded → simpler deterministic assistance or ask user.

### Explainability

- Continuous narration of: current step, why, permissions in use, responsible capability (ADR-0006 examples: observing desktop, opening app, waiting, permission required).

### Future Research Required

- Planning depth vs predictability trade-offs
- Interruption and turn-taking policy
- How much autonomy default profiles should allow

### Candidate Open Source Categories

- Workflow Orchestration (lightweight)
- State Machine Framework
- Event Bus

### Acceptance Criteria

- No ownership overlap with Sense/Action/Memory/Intelligence
- Every meaningful step permission-checked
- User can cancel in-flight tasks
- Explanations always identify responsible capability

---

## Experience

### Purpose

Present Workspace as a calm, honest, predictable human interface—capturing intent and showing explanations without owning orchestration logic.

### Owns

- User interaction surfaces (visual and, when enabled, voice as an interaction modality)
- Presentation of explanations, permission challenges, and progress
- Interaction accessibility and cognitive-load-reducing UX patterns
- Local UI state

### Does Not Own

- Task planning (Companion Orchestration)
- Permission truth (Permission Authority)
- Memory/action/sensing backends
- Model inference

### Responsibilities

- Capture user intents and preferences
- Render companion progress and questions
- Present permission challenges and revocation controls
- Provide workspace navigation chrome
- Optional voice input/output as UX modalities (engines are integration categories)

### Inputs

- User pointer/keyboard/voice interactions
- Companion progress/explanation events
- Permission challenges
- Host mode/health events

### Outputs

- Intent messages and interaction responses to Companion
- Purpose-bound Workspace read authorization requests to Permission Authority
- User permission decisions to Permission Authority
- Read-only Workspace view queries to Workspace Management; mutations are submitted as intents to Companion
- Presentation-only state changes

### Contracts

- `Experience.submitIntent`
- `Experience.showExplanation`
- `Experience.presentPermissionChallenge`
- `Experience.setPresentationState`

### Dependencies

- Runtime Host
- Companion Orchestration
- Permission Authority (Workspace read authorization, challenge completion, and user grant/deny/revoke decisions)
- Workspace Management (navigation)

### Permissions

- UI itself requires no automation permission.
- Voice listening / microphone: `experience.microphone` when voice enabled.
- Screen overlay / always-on surfaces: explicit UI permissions as needed.

### Data Ownership

- UI preferences and layout state
- Presentation theme/settings
- Ephemeral interaction buffers

### Lifecycle

- Creation: show startup/status
- Runtime: primary human loop
- Persistence: UI preferences locally
- Shutdown: close surfaces; discard ephemeral buffers

### Failure Behaviour

- If Companion unavailable, Experience shows honest limited mode (settings, permissions, status) without pretending autonomy.
- Voice stack failure falls back to non-voice UI.

### Explainability

- Primary channel for all capability explanations; never hides permission or capability identity.

### Future Research Required

- Voice stack
- Optimal companion presence patterns (non-manipulative)
- Notification urgency rules

### Candidate Open Source Categories

- UI Framework
- Speech Recognition
- Speech Synthesis
- Notification Framework
- Accessibility Toolkit

### Acceptance Criteria

- All permission challenges user-resolvable here
- Explanations show what/why/permission/capability
- No orchestration or action APIs embedded in Experience
- Usable in local offline mode for core flows

---

## Extension Host

### Purpose

Define the optional boundary for controlled extension of Workspace without breaking permission, privacy, or single-responsibility boundaries. This capability remains dormant and need not exist at runtime until a future architecture milestone justifies activation.

### Owns

- Extension registration, load/unload
- Extension sandboxes / privilege attenuation
- Extension manifests and declared permission needs
- Routing extension contributions and requests into the Companion Orchestration contract

### Does Not Own

- Core companion policy (Companion Orchestration)
- Permission granting (user via Permission Authority)
- Direct OS automation APIs (must use Action)
- Unscoped memory access
- Direct calls from extensions or Extension Host to core domain capabilities other than Permission Authority and Companion Orchestration

### Responsibilities

- Discover and validate extensions
- Declare required extension scopes to Companion Orchestration; Companion requests authorization through Permission Authority
- Route all extension requests and side effects through Companion Orchestration
- Unload extensions on revoke/failure
- Prevent extensions from becoming a second orchestrator with hidden powers

### Inputs

- Install/enable/disable extension commands
- Extension invocations
- Permission grants for extension scopes

### Outputs

- Extension-provided intents/contributions to Companion (optional)
- Extension contribution and intent requests to Companion Orchestration
- Extension error/security events

### Contracts

- `Extension.install(manifest, authorization_proof)` / `Extension.unload(extension_id, authorization_proof)`
- `Extension.list(authorization_proof)`
- `Extension.invoke(request, authorization_proof)` (mediated through Companion)
- `Extension.explain(extension_id, authorization_proof)`

### Dependencies

- Runtime Host
- Permission Authority
- Publishes extension contributions to Companion Orchestration without requiring a reverse Companion implementation dependency
- No direct dependency on Workspace Management, Memory, Context Sensing, Action, Intelligence, or Experience

### Permissions

- `extension.manage`
- Each extension holds only user-granted scopes listed in its manifest
- Extensions never inherit ambient companion authority

### Data Ownership

- Extension manifests and enablement state
- Extension-local configuration and disposable cache partitions (isolated); never durable user knowledge, observations, task content, or prompt/response content
- Extension audit events

### Lifecycle

- Creation: capability is not activated until an approved extension milestone; when activated, no extensions are loaded by default
- Runtime: load only enabled+granted extensions
- Persistence: enablement and isolated data locally
- Shutdown: unload all extensions first

### Failure Behaviour

- Extension crash → unload; core Workspace continues.
- Permission revoke → immediate loss of mediated powers.
- Manifest invalid → refuse load.
- Reject expired, revoked, replayed, or extension-identity/context-mismatched authorization proofs at management commit points.

### Explainability

- Identify when an extension is responsible for a step; show required permissions.

### Future Research Required

- Plugin architecture model
- Sandbox strength vs capability
- Trust/signing model

### Candidate Open Source Categories

- Plugin Runtime
- WebAssembly Sandbox
- Extension Manifest Schemas
- Capability-Based Security Frameworks

### Acceptance Criteria

- Extensions cannot bypass Permission Authority
- Extensions and Extension Host cannot call Action or other domain capabilities directly; all requests flow through Companion Orchestration
- Core system runs with zero extensions
- Responsibility for each extension action is explainable

---

## System Architecture

### High-level capability dependency graph

This graph shows domain contract calls only. An arrow means “may initiate a call to the public contract of.” Runtime lifecycle calls and all events are excluded and defined separately in the Interaction Matrix:

```text
Experience ───────────────→ Companion Orchestration
Experience ───────────────→ Permission Authority (user decisions and Workspace read authorization)
Experience ───────────────→ Workspace Management (read-only views)

Extension Host ───────────→ Permission Authority

Companion Orchestration ──→ Permission Authority
Companion Orchestration ──→ Workspace Management
Companion Orchestration ──→ Memory
Companion Orchestration ──→ Context Sensing
Companion Orchestration ──→ Intelligence
Companion Orchestration ──→ Action
Companion Orchestration ──→ Extension Host (optional management, only after activation)
```

Notes:
- Runtime Host starts/stops capabilities and receives their health events as defined by the Interaction Matrix; these lifecycle/event paths are not domain call dependencies.
- Runtime Host publishes offline/online mode to Intelligence as an event, not a call dependency.
- Extension Host publishes contributions to Companion as events, not calls.
- Intelligence does not depend on Context Sensing or Action; Companion supplies context and dispatches actions.
- Extension Host cannot reach domain capabilities directly; Companion Orchestration is its required intermediary.
- Companion does not depend on Experience. It publishes progress and interaction-request events; Experience subscribes and returns responses through the Companion contract.
- Permission Authority is the sole authorization issuer; Action revalidates bound authorization at the execution boundary.

### Capability interaction summary

| From → To | Interaction |
|-----------|-------------|
| Experience → Companion | User intents, cancellations |
| Experience → Permission Authority | Workspace read authorization requests; grant/deny/revoke decisions |
| Experience → Workspace Management | Read-only view/navigation queries |
| Companion → Permission Authority | Authorize/challenge before meaningful steps |
| Companion → Workspace Management | Read/adapt to active scope |
| Companion → Memory | Retrieve/propose writes |
| Companion → Context Sensing | Query/subscribe to context |
| Companion → Intelligence | Reason/generate; receive proposals |
| Companion → Action | Execute permitted operations |
| Companion → Experience | Progress, questions, explanations |
| Context Sensing → Companion | Optional memory-write candidate event; Companion decides whether to initiate a permissioned Memory proposal |
| Intelligence → Action | Forbidden (proposals only, via Companion) |
| Extension Host → Companion | Extension contributions and intents; all domain effects require Companion mediation |
| Extension Host → domain capabilities | Forbidden |

### Data ownership map

| Data | Owner |
|------|-------|
| Host config, registry, mode | Runtime Host |
| Grants, policies, permission audit | Permission Authority |
| Workspaces, zones, active scope | Workspace Management |
| Memory items, provenance, retention | Memory |
| Ephemeral observations, sensing sessions | Context Sensing |
| Metadata-only execution audit, minimized effect summaries, action catalogue | Action |
| Provider config, inference transactions (minimized) | Intelligence |
| Task/plan state, orchestration policy | Companion Orchestration |
| UI preferences, presentation state | Experience |
| Extension manifests, isolated extension data | Extension Host |

No shared mutable ownership. Cross-capability data moves only through contracts as copies or immutable references with purpose.

### Cross-capability correlation envelope

Every request, event, authorization decision, and consequential state change carries:

- `task_id` — owned by Companion Orchestration for a user intent; absent only for explicit direct user administration
- `operation_id` — owned by the capability committing the operation
- `authorization_id` — owned by Permission Authority when authorization is required
- `causation_id` — identifies the immediately preceding request or event
- `requester_capability` and, where applicable, `extension_id`

Identifiers correlate separately owned records; they do not centralize data ownership or grant authority. Explainability joins only the minimum metadata required to answer what, why, permission, and responsible capability.

### Permission boundary map

| Boundary | Rule |
|----------|------|
| Default | Deny meaningful automation and deep observation |
| Permission Authority | Sole granter/revoker; fail closed |
| Context Sensing | `sense.observe.basic` vs `sense.observe.deep` separated |
| Action | Per action-class permissions; no omnibus grant |
| Intelligence | Local vs remote separately permissioned |
| Memory | read/write/forget separated |
| Extension Host | Per-extension attenuated scopes |
| Companion | No ambient superuser; purpose-bound checks |
| Experience | Microphone/overlay permissions only as needed for modalities |

### User interaction flow

1. User expresses intent in Experience (typed, clicked, or optional voice).
2. Experience submits intent to Companion Orchestration.
3. Companion reads Workspace scope; may query Memory and Context Sensing (permissioned).
4. Companion requests Intelligence for understanding/planning when needed.
5. Before meaningful side effects, Companion calls Permission Authority (`authorize` / `challenge`) with purpose, capability, subject, target, and operation.
6. Experience presents challenges; user grant/deny returns to Permission Authority.
7. On authorization, Companion dispatches Action with the bound proof (and/or initiates separately authorized Memory writes or Workspace mutations).
8. Companion emits explanations throughout; Experience presents what/why/permission/capability.
9. User may cancel via Experience → Companion at any time; revoke via Experience → Permission Authority stops further use.

### Offline core-functionality acceptance scenarios

Workspace satisfies Local First only when all of these scenarios work without internet connectivity:

1. Runtime Host launches, reports local/offline mode, starts required capabilities, and shuts down safely.
2. The user can inspect, grant, deny, and revoke locally stored permissions.
3. The user can create, read, change, and archive local workspaces/zones.
4. The user can write, retrieve, redact, and forget locally stored Memory.
5. Context Sensing and Action functions that target local OS resources remain available when their local prerequisites and permissions exist.
6. Configured local Intelligence performs inference without network access; lack of a configured local provider is explained and never triggers silent cloud fallback.
7. Companion supports deterministic permission/workspace/memory flows offline and intelligence-assisted flows when local Intelligence is available.
8. Experience remains usable offline and explains unavailable optional cloud functions.

These are architecture acceptance scenarios, not technology selections. Capability-level tests must distinguish unavailable local prerequisites from internet dependency.

### Startup sequence

1. OS starts Runtime Host.
2. Runtime Host loads host configuration; sets offline/online-optional mode.
3. Permission Authority starts (fail-closed ready).
4. Workspace Management starts; ensures default workspace/scope.
5. Memory, Context Sensing, Intelligence, and Action start (sensing remains inactive until granted). Extension Host starts only if a separately approved extension milestone has activated it.
6. Companion Orchestration starts when required dependencies are available.
7. Experience starts; shows ready/degraded status honestly.
8. Host emits `ready`.

### Shutdown sequence

1. User or OS requests shutdown via Experience or Host.
2. Companion cancels in-flight tasks.
3. Action stops accepting work; cancels or safely completes in-flight executions and reports the outcome.
4. Context Sensing stops and clears ephemeral buffers.
5. Intelligence aborts in-flight inference.
6. Extension Host unloads extensions.
7. Memory flushes safe writes.
8. Workspace Management persists active scope.
9. Permission Authority invalidates unused proofs and flushes audit.
10. Experience presents final shutdown state, then closes interactive surfaces.
11. Runtime Host exits process.

### Capability lifecycle overview

| Phase | Behaviour |
|-------|-----------|
| Create | Host registers capability; default-safe state (sensing off, actions idle, extensions unloaded) |
| Ready | Contracts available; Permission Authority fail-closed |
| Active | Serve requests; emit explanations |
| Degraded | Missing dependency or provider → reduced function with explicit explanation |
| Suspended | User pause (especially sensing) or permission revoke |
| Shutdown | Ordered release; ephemeral sensitive buffers cleared |

---

## Architectural Validation

### Duplicate responsibilities

| Risk | Resolution |
|------|------------|
| Sensing vs Action both “desktop” | Split read (Context Sensing) vs write (Action) |
| Intelligence vs Companion both “decide” | Intelligence proposes/reasons; Companion decides and sequences |
| Memory vs Workspace both “structure” | Workspace = organization; Memory = retained knowledge |
| Experience vs Companion both “conversation” | Experience presents/captures; Companion orchestrates |
| Extension Host vs Companion both “coordinate” | Extensions may contribute intents; Companion remains sole core orchestrator |

### Circular dependencies

- Graph is layered and acyclic for hard dependencies.
- Experience calls Companion; Companion publishes events without depending on an Experience implementation. Interaction responses return through the Companion contract, so no hard dependency cycle exists.
- Intelligence has no dependency on Action/Context Sensing.

### Ownership conflicts

- Permission truth solely in Permission Authority.
- Environment mutation solely in Action.
- Long-term retention solely in Memory.
- Extension-originated effects require Companion mediation; Extension Host has no direct domain-capability path.
- No dual owners identified after revision.

### Unnecessary complexity

- Voice is a modality under Experience (not a separate capability).
- Privacy enforcement is distributed as ownership + Permission Authority rather than a redundant Privacy capability.
- Optional cloud is a permissioned provider class under Intelligence, not a separate Cloud capability.
- Nine active foundational capabilities are justified by ADR-0002 and the explicit sense/act split. Extension Host defines a dormant trust boundary only and requires separate Complexity Budget justification before activation.

### Blueprint compliance

- Local First: host and core paths offline-capable; remote intelligence optional.
- Human First / Privacy First: default-deny, minimized sensing, forget/redact in Memory.
- Permission Before Automation: enforced at Authority + Action.
- Explain Every Action: required outputs on each capability; Experience presents.
- Integrate Before Reinventing: candidate categories listed; no vendor selection.
- Complexity Must Justify Itself: sense/act split and extension boundary justified by trust and evolvability.
- Documentation Never Lives In Runtime: this architecture pack remains engineering artifact (ADR-0007).
- Reduce Cognitive Load: workspace scoping, calm Experience, explicit explanations.

### ADR compliance

| ADR | Compliance |
|-----|------------|
| 0001 Local First | Core capabilities operate without internet; remote intelligence optional |
| 0002 Integrate Before Reinvent | Build orchestration/permissions/memory/experience/workspace; integrate commodity categories later |
| 0003 Permission First | Permission Authority fail-closed; challenges explainable/revocable |
| 0004 Complexity Budget | No extra privacy/cloud/voice capabilities without distinct ownership need |
| 0005 Single Responsibility | Each capability has one primary responsibility; overlaps explicitly rejected |
| 0006 Explainability | What/why/permission/capability required across capabilities |
| 0007 Docs Outside Runtime | Document is architecture pack, not runtime payload |
| 0008 Sessions Replaceable | Architecture recorded in artifacts for future sessions |

### Validation outcome

No blocking duplicate responsibilities, circular hard dependencies, ownership conflicts, or Blueprint/ADR violations remain after the sense/act split, intelligence non-execution rule, one-way Experience dependency, bound execution authorization, and extension mediation rule.

---

## Authority and Evolution

- Product principles: Blueprint
- Decisions: `architecture/decisions/`
- Capability decomposition: this document
- Communication and trust constraints: `architecture/09_Capability_Interaction_Matrix.md`
- Public capability contracts: `architecture/10_Capability_Contracts.md`
- Contract schema and acceptance gates: `architecture/11_Contract_Schema_and_Acceptance_Specification.md`
- Technology choices: future Research Catalogue entries per capability (not made here)

Next engineering milestone (recommended): research candidate technologies per capability category, recording results once in the Research Catalogue before implementation.
