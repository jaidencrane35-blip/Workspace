# Workspace User Journey Architecture Validation v1.0

Status: Complete
Authority: Architecture validation and gap analysis
Version: 1.0

This document validates whether the current Workspace capability architecture can support the user experience required by the Blueprint.

It does not select technologies, define implementation plans, or change capability ownership. Where a journey is not fully supported, it records the minimum architecture change required.

Authority order remains:

1. `00_Workspace_Blueprint.md`
2. Accepted ADRs in `architecture/decisions/`
3. `08_Workspace_Capability_Architecture.md`
4. `09_Capability_Interaction_Matrix.md`
5. `10_Capability_Contracts.md`
6. `11_Contract_Schema_and_Acceptance_Specification.md`
7. This validation

---

## Validation Method

Each journey is reviewed for:

- user goal
- participating capabilities
- permitted interactions and contracts
- permission checks
- explainability
- persistence
- recovery/failure
- architectural risk
- capability completeness
- contract completeness
- permission-boundary compliance
- Blueprint and ADR compliance

Outcomes:

- **Validated** — current architecture supports the journey.
- **Conditional** — the journey is substantially supported but requires explicit guidance or a bounded contract correction.
- **Gap** — a required interaction is missing, forbidden, contradictory, or cannot satisfy a Blueprint invariant.

Optional voice and Extension Host paths do not determine core Blueprint readiness, but their activation requirements are recorded.

---

## Primary Journeys Implied by the Blueprint

The Blueprint implies these primary journeys:

1. Launch Workspace and reach a safe, usable local state.
2. Organize work into workspaces and zones.
3. Ask the Companion for assistance and receive an understandable response.
4. Permit Workspace to understand current computing context.
5. Remember, recall, redact, and forget appropriately.
6. Configure and use local-first intelligence.
7. Choose optional remote enhancement with explicit data-sharing permission.
8. Perform an explainable, permissioned environment action.
9. Inspect, grant, deny, revoke, and configure permissions.
10. Inspect task status and explanations.
11. Cancel work and reduce risk.
12. Recover honestly from unavailable, timed-out, partial, or indeterminate work.
13. Shut Workspace down safely.

Voice is an optional Experience modality across these journeys. Extensions are an optional future contribution source, not a core user journey.

---

## UJ-01 — Launch and Reach a Safe Local State

**Outcome: Gap**

### User goal

Open Workspace and reliably reach either a usable local experience or an honest degraded experience without requiring internet access.

### Participating capabilities

- Runtime Host
- Permission Authority
- Workspace Management
- Memory
- Context Sensing
- Action
- Intelligence
- Companion Orchestration
- Experience
- Extension Host only if separately activated

### Capability interactions

- IC-001: Runtime Host starts active capabilities.
- IC-002: capabilities report health to Runtime Host.
- IC-003: Runtime Host publishes aggregate status to Experience.
- IC-004: Experience queries Host status.
- IC-005: Runtime Host publishes offline/online-optional mode to Intelligence.
- Startup order is defined in Capability Architecture.

### Permission checks

- Permission Authority starts default-deny.
- Context Sensing remains inactive until authorized.
- Action remains idle without authorization.
- Remote Intelligence is not selected silently.
- Extension Host remains dormant unless separately justified.

### Explainability points

- Experience presents starting, ready, degraded, offline, capability-unavailable, and shutdown states.
- Missing local Intelligence must be explained without cloud fallback.

### Persistence points

- Runtime Host loads local host configuration.
- Permission Authority loads local grants/policies.
- Workspace Management ensures an active/default scope.
- Memory loads local retained knowledge.
- Experience loads local presentation preferences.

### Recovery/failure paths

- Non-critical capability failure becomes an aggregate degraded state.
- Core local functions remain available where prerequisites exist.
- Host failure stops Workspace rather than leaving hidden autonomy.

### Architectural risks and gaps

1. Companion lists all domain capabilities as dependencies and starts when “required dependencies” are available, but the architecture does not classify which are startup-essential and which are degradable.
2. Experience starts after Companion even though Experience must expose status, permission administration, recovery, and limited mode when Companion is unavailable.
3. First-run safe behavior is distributed across capability lifecycle rules. The defaults are sufficient architecturally (sensing off, Action idle, Extension Host dormant, no silent remote fallback), but should be presented coherently by Experience.

### Validation

- Required capabilities exist: **Yes**
- Required interaction contracts exist: **Partly**
- Permission boundaries respected: **Conditional**
- Explainability satisfied: **Conditional**
- Blueprint compliance: **Not yet proven**
- ADR compliance: **ADR-0001 and ADR-0006 require clarification**

### Minimum required architecture change

- Classify capability dependencies as essential or degradable.
- Require Experience/status/permission administration to start independently of Companion readiness.
- Require Experience to present the existing safe defaults coherently on first run; this does not create a new authority or bootstrap permission.

---

## UJ-02 — Organize Work into Workspaces and Zones

**Outcome: Conditional**

### User goal

View, create, rename, select, reorganize, archive, and understand the active workspace/zone.

### Participating capabilities

- Experience
- Companion Orchestration
- Permission Authority
- Workspace Management
- Memory, Context Sensing, and Action as scope consumers

### Capability interactions

- IC-007: Experience obtains purpose-bound Workspace-read authorization.
- IC-013: Experience reads navigation summaries directly.
- IC-032: Experience submits mutation intent to Companion.
- IC-006: Companion requests `workspace.write`.
- IC-014: Companion reads/mutates Workspace and reconciles outcomes.
- IC-018: Workspace publishes scope changes/results.
- IC-015–017: Memory, Context Sensing, and Action validate scope.

### Permission checks

- `workspace.read` for navigation and scope views.
- `workspace.write` for structural mutations.
- Workspace validates authorization at access/commit.
- Experience cannot mutate Workspace directly.

### Explainability points

- Active workspace/zone is visible.
- Structural change, requester, permission, result, and conflict are explained.
- Unscoped/degraded behavior is explicit.

### Persistence points

- Workspace/zone records and active scope persist locally.
- Dependent capabilities store only their own scoped state.

### Recovery/failure paths

- Missing Workspace capability produces constrained unscoped behavior.
- Mutation conflicts/failures do not imply success.
- Lost terminal results use operation status/tombstone reconciliation.

### Architectural risks and gaps

Archiving an active workspace has no explicit active-scope fallback or rule for whether associated scoped Memory remains retrievable.

### Validation

- Required capabilities exist: **Yes**
- Required interaction contracts exist: **Yes for normal read/mutate**
- Permission boundaries respected: **Yes**
- Explainability satisfied: **Yes for individual operations**
- Blueprint compliance: **Conditional on predictable archive/Memory behavior**
- ADR compliance: **No accepted ADR violation; lifecycle guidance is incomplete**

### Minimum required architecture change

Define workspace archive lifecycle semantics:

- active-scope fallback
- treatment of in-flight scoped work
- visibility of Memory that remains scoped to an archived workspace

Deletion and bulk Memory disposition are product decisions, not current Blueprint requirements.

---

## UJ-03 — Ask the Companion for Assistance

**Outcome: Gap**

### User goal

Express an intent and receive calm, useful, understandable assistance without hidden actions.

### Participating capabilities

- Experience
- Companion Orchestration
- Workspace Management
- Memory
- Context Sensing
- Intelligence
- Permission Authority
- Action only when the intent requires effects

### Capability interactions

- IC-032: Experience submits intent/interaction result/cancel.
- IC-014: Companion resolves Workspace scope.
- IC-019: Companion retrieves approved Memory.
- IC-024/025: Companion queries/subscribes to minimized context.
- IC-026–028: Companion requests reasoning and evaluates non-executing proposals.
- IC-034: Companion publishes progress, questions, explanations, and terminal results.

### Permission checks

- User intent is not ambient permission.
- Each protected downstream operation receives task-purpose-bound authorization.
- Intelligence proposals grant no Action authority.

### Explainability points

- Current step, purpose, responsible capability, permission state, waiting condition, uncertainty, and result are published.
- Experience preserves domain meaning.

### Persistence points

- Active task/plan exists in Companion.
- Task content is in-flight only.
- Durable user knowledge goes through Memory.
- Companion persists metadata-only outcomes/correlation.

### Recovery/failure paths

- Missing dependency narrows assistance.
- Missing Intelligence permits deterministic fallback or a user question.
- Permission denial stops only the affected path.

### Architectural risks and gaps

1. The Interaction Matrix permits Experience → Companion only for intent/interaction result. Capability Architecture and Capability Contracts also require cancellation, status, and explanation. Because matrix cells are purpose-limited, the architecture authorities contradict each other and must be reconciled.
2. “Continuous narration” has no attention budget. It can conflict with calmness and reduced cognitive load.
3. No interaction metadata defines urgency, deferrability, expiry, suppression, safe default, or bounded re-prompting.

### Validation

- Required capabilities exist: **Yes**
- Required interaction contracts exist: **Contradictory**
- Permission boundaries respected: **Yes**
- Explainability satisfied: **Informationally yes; attention behavior no**
- Blueprint compliance: **No, until calm/non-manipulative interaction is enforceable**
- ADR compliance: **ADR-0004, ADR-0005, and ADR-0006 require matrix reconciliation**

### Minimum required architecture change

- Expand Experience → Companion matrix purpose to include intent, interaction result, cancel, task status, and task explanation.
- Define behavioral acceptance invariants for Companion → Experience attention:
  - respect user-controlled quiet/focus state
  - interrupt only for a declared, truthful reason
  - defer safely where possible
  - use bounded re-prompting
  - define a safe no-response outcome
- Let later Experience research determine the smallest contract metadata needed to enforce those invariants.
- Replace “continuous narration” with concise state-change presentation plus inspectable detail.

---

## UJ-04 — Permit Context-Aware Assistance

**Outcome: Gap**

### User goal

Allow Workspace to observe a clearly described class of local context, understand what is observed, and pause/stop it immediately.

### Participating capabilities

- Experience
- Companion Orchestration
- Permission Authority
- Context Sensing
- Workspace Management for optional scope tags
- Memory only through a separate Companion-mediated retention path

### Capability interactions

- IC-006: Companion requests basic/deep sensing authority and operation control.
- IC-009: Context Sensing validates protected use.
- IC-023: Companion starts/resumes/pauses/stops sensing.
- IC-024: Companion queries, subscribes, explains, and reconciles sensing.
- IC-025: Context Sensing emits minimized context/candidate/state events.
- IC-016: Context Sensing validates Workspace scope tags.

### Permission checks

- Basic and deep sensing are distinct.
- Start/resume require effect authority.
- Pause/stop use owner-bound operation control and cannot expand access.
- Revocation stops sensing and clears buffers.
- Observation permission does not authorize Memory retention.

### Explainability points

- Sensor class, purpose, requester, active/paused state, data class, minimization, and stop/revoke action are shown.

### Persistence points

- Raw observations remain ephemeral inside Context Sensing.
- Durable retention requires a separately authorized Memory proposal.

### Recovery/failure paths

- Sensor failure falls back to user-provided context.
- Missing/revoked authority stops sensing.
- Operation status/indeterminate recovery is defined.

### Architectural risks and gaps

1. `Sense.subscribe` is a request that creates ongoing delivery/state, conflicting with the schema rule that requests are side-effect-free.
2. No unsubscribe contract exists.
3. Start acceptance does not explicitly return a sensing `session_reference`; pause/stop use `session_id` while the standard model requires owner-issued reference plus control proof.
4. Task cancellation fan-out to sensing is stated at orchestration level but not defined as a complete cancellation/reconciliation invariant.

### Validation

- Required capabilities exist: **Yes**
- Required interaction contracts exist: **Incomplete**
- Permission boundaries respected: **Yes in defined paths**
- Explainability satisfied: **Yes**
- Blueprint compliance: **Conditional**
- ADR compliance: **ADR-0003 and ADR-0006 require lifecycle completion**

### Minimum required architecture change

- Model subscription creation/removal as explicit state-changing commands with terminal state, or define a side-effect-free bounded stream contract that owns no durable subscription state.
- Add unsubscribe.
- Define `session_id` as the owner operation identity. Permission Authority issues the control proof before start; Context Sensing returns the opaque session reference and binds that proof at acceptance.
- Require task cancellation to pause/stop all active sensing sessions and reconcile their terminal states.

---

## UJ-05 — Remember, Recall, Redact, and Forget

**Outcome: Conditional**

### User goal

Allow Workspace to remember useful knowledge deliberately, retrieve it in scope, understand its provenance, and remove it.

### Participating capabilities

- Experience
- Companion Orchestration
- Permission Authority
- Memory
- Workspace Management
- Context Sensing only as a candidate source through Companion

### Capability interactions

- IC-019: retrieve/explain/reconcile Memory.
- IC-020: propose authorized durable retention.
- IC-021: forget/redact.
- IC-022: Memory returns terminal/policy outcomes.
- IC-015: Memory validates Workspace scope.
- IC-025 → Companion → IC-020: sensing candidate retention path.

### Permission checks

- `memory.read`, `memory.write`, and `memory.forget` are distinct.
- Provenance/content explanations require `memory.read`.
- Observation permission never implies retention.
- Point-of-use validation applies to read/write/forget/redact.

### Explainability points

- What was remembered, why, source, scope, retention, permission, and removal path are available.

### Persistence points

- Memory is the sole durable user-knowledge owner.
- Provenance and retention policy persist locally.
- Other capabilities persist metadata only.

### Recovery/failure paths

- Retrieval failure produces reduced-context assistance.
- Write failure rejects without hidden permission-bypassing retry.
- Forget/redact failure leaves the item unchanged and is explained.

### Architectural risks and gaps

1. Visibility of Memory scoped to an archived workspace is undefined.
2. Concrete retention durations remain a future policy decision already constrained by the contract specification.

### Validation

- Required capabilities exist: **Yes**
- Required interaction contracts exist: **Yes**
- Permission boundaries respected: **Yes**
- Explainability satisfied: **Yes for retained items**
- Blueprint compliance: **Conditional only on archived-scope visibility**
- ADR compliance: **No accepted ADR is directly violated; guidance is incomplete**

### Minimum required architecture change

Define archived-workspace Memory visibility as described in UJ-02.

User-editable retention policy, deletion, and bulk forgetting may be considered later as product choices; they are not required by the current Blueprint.

---

## UJ-06 — Receive Local-First Intelligence

**Outcome: Conditional**

### User goal

Receive useful reasoning/generation locally when configured, with honest degradation when unavailable.

### Participating capabilities

- Experience
- Companion Orchestration
- Permission Authority
- Intelligence
- Runtime Host
- Workspace/Memory/Context only through Companion-supplied minimized context

### Capability interactions

- IC-005: Runtime Host sends mode to Intelligence.
- IC-006: Companion obtains local inference authority/control.
- IC-009: Intelligence validates protected use.
- IC-026: Companion submits minimized reasoning/generation request.
- IC-027: Companion reads provider/status/explanation.
- IC-028: Intelligence returns result/proposal/availability/indeterminate event.
- IC-034: Companion explains the result to Experience.

### Permission checks

- Protected local inference uses `intelligence.local.run`.
- Intelligence receives only caller-supplied minimized context.
- Proposals are not execution authority.

### Explainability points

- Local provider class, requester, purpose, permission, waiting/resource state, uncertainty, and outcome are shown.

### Persistence points

- Provider configuration and metadata-only transaction records persist.
- Prompts/responses and Memory content do not persist in Intelligence.

### Recovery/failure paths

- No local provider produces honest degraded assistance.
- Companion may use deterministic behavior or ask the user.
- Timeout/indeterminate outcomes do not imply success or automatic retry.

### Architectural risks and gaps

1. Intelligence has no explicit cancellation command even though inference is non-immediate and task cancellation must stop feasible child work.
2. Companion crash/restart custody of operation reference/control proof is not defined.

### Validation

- Required capabilities exist: **Yes**
- Required interaction contracts exist: **Mostly**
- Permission boundaries respected: **Yes**
- Explainability satisfied: **Yes**
- Blueprint compliance: **Conditional on cancellation/recovery**
- ADR compliance: **ADR-0001 satisfied; Human First control incomplete**

### Minimum required architecture change

- Declare inference cancellation class and add status/safety cancellation control where feasible.
- Include inference in Companion cancellation fan-out and terminal reconciliation.
- Define secure, bounded custody/recovery of in-flight operation references/control proofs.

---

## UJ-06A — Configure a Local Intelligence Provider

**Outcome: Gap**

### User goal

Inspect whether local Intelligence is available, configure or remove a local provider, understand local resource limits, and remain usable when no provider is configured.

### Participating capabilities

- Experience
- Companion Orchestration
- Permission Authority
- Intelligence
- Runtime Host

### Capability interactions

- IC-005: Runtime Host supplies local/offline mode.
- IC-027: Companion can list provider availability and explanations.
- IC-034: Companion presents availability/degradation through Experience.

### Permission checks

- Provider configuration is local administration.
- Configuration authority must not imply inference, remote use, data sharing, or Action authority.
- Secrets, if any, remain references rather than explanation/event payloads.

### Explainability points

- Configured/unset state, provider class, local resource implications, responsible capability, and effect of removal.

### Persistence points

- Intelligence already owns provider configuration/references and local resource-cap settings.

### Recovery/failure paths

- Unset/unavailable local provider leaves deterministic core flows available.
- Invalid configuration must not trigger remote fallback.

### Architectural risks and gaps

Intelligence owns provider configuration and can list providers, but no permitted public command allows the user to configure, update, remove, or set local resource limits.

### Validation

- Required capabilities exist: **Yes**
- Required interaction contracts exist: **No**
- Permission boundaries respected: **Not yet defined for administration**
- Explainability satisfied: **Availability only**
- Blueprint compliance: **Local First is incomplete as a user-configurable journey**
- ADR compliance: **ADR-0001 requires a first-class local path; ADR-0003 applies to meaningful configuration**

### Minimum required architecture change

Add Companion-mediated Intelligence administration contracts for:

- configure/update/remove local provider
- set local resource caps
- report validation/availability outcome

Define separate, narrowly scoped administration authority. Do not combine provider configuration with inference or remote data-sharing permission.

---

## UJ-07 — Use Optional Remote Intelligence

**Outcome: Gap (optional journey)**

### User goal

Choose an optional remote provider with full awareness that selected data may leave the device.

### Participating capabilities

- Experience
- Companion Orchestration
- Permission Authority
- Intelligence
- Runtime Host

### Capability interactions

- IC-005: Host indicates offline/online-optional mode.
- IC-006: Companion requests remote/provider/data-sharing authorization.
- IC-009: Intelligence validates before transfer.
- IC-026–028: request, result, proposal, availability, and explanation.
- IC-034: Experience presents the boundary and outcome.

### Permission checks

- `intelligence.remote.run` is distinct from local inference.
- Explicit data-sharing scope is also required.
- No silent cloud fallback.

### Explainability points

- Remote provider class and data boundary must be identified before transfer.
- Denial/offline state must remain understandable.

### Persistence points

- Provider configuration persists locally.
- Remote prompt/response content does not become Intelligence history.

### Recovery/failure paths

- Offline/denied remote use falls back only to an authorized local path or refusal.
- Failure never broadens data sharing.

### Architectural risks and gaps

Authorization proof semantics bind one permission scope, while remote inference requires both remote-run and data-sharing scopes. The contracts do not define whether this is one compound proof, two independently validated proofs, or one authorization set.

### Validation

- Required capabilities exist: **Yes**
- Required interaction contracts exist: **Ambiguous**
- Permission boundaries respected: **Not provable**
- Explainability satisfied: **Yes if authorization is resolved**
- Blueprint compliance: **Core unaffected; optional journey not ready**
- ADR compliance: **ADR-0001 and ADR-0003 require explicit multi-scope semantics**

### Minimum required architecture change

Define independently revocable compound authorization semantics:

- all required scopes named
- each scope independently explainable/revocable
- point-of-use validation proves the complete required set
- data transfer cannot begin if any required scope is absent

No new capability is required.

---

## UJ-08 — Perform Explainable Automation

**Outcome: Conditional**

### User goal

Ask Workspace to change the environment, approve the exact meaningful action, observe progress, and receive an honest result.

### Participating capabilities

- Experience
- Companion Orchestration
- Permission Authority
- Action
- Workspace Management for target scope validation
- Intelligence only as a non-authoritative proposal source

### Capability interactions

- IC-032: Experience submits intent.
- IC-006: Companion requests effect/control authority.
- IC-009: Action validates each independently meaningful effect.
- IC-017: Action validates Workspace target scope.
- IC-029: Companion requests execution.
- IC-030: Companion describes/statuses/cancels/reconciles.
- IC-031: Action emits progress and terminal result.
- IC-034: Companion publishes explanations/outcome.

### Permission checks

- Exact action class, purpose, subject, target, operation, and scope are bound.
- No omnibus action permission.
- Multi-effect operations revalidate each meaningful effect unless atomic.
- Intelligence proposals never authorize execution.

### Explainability points

- Before/during/after: action, why, target, responsible capability, permission, effects, partial effects, cancellation/undo availability, and result.

### Persistence points

- Action keeps metadata-only audit and minimized effect summaries.
- User content is not copied into Action history.

### Recovery/failure paths

- Cancellation, partial completion, indeterminate outcomes, lost events, and terminal lookup are modeled.
- No blind retry after unknown/partial effects.

### Architectural risks and gaps

1. Every consequential command must declare a cancellation class, but concrete Action classes have no accepted commit points/cancellation/compensation semantics yet.
2. Task cancellation fan-out is incomplete.
3. Consent presentation has information requirements but no fairness/non-manipulation invariants.

### Validation

- Required capabilities exist: **Yes**
- Required interaction contracts exist: **Yes at generic level**
- Permission boundaries respected: **Yes**
- Explainability satisfied: **Informationally yes**
- Blueprint compliance: **Conditional on Human First cancellation/consent**
- ADR compliance: **ADR-0003 and ADR-0006 are structurally supported**

### Minimum required architecture change

- Require each Action class to declare irreversible commit point, cancellation class, partial-effect boundary, and compensation availability before technology selection/implementation for that class.
- Complete Companion cancellation fan-out.
- Add neutral consent invariants in UJ-09.

---

## UJ-09 — Manage Permissions and Consent

**Outcome: Conditional**

### User goal

Inspect permissions, understand requests, grant or deny neutrally, revoke later, and configure narrowly scoped automatic execution.

### Participating capabilities

- Experience
- Permission Authority
- Companion Orchestration for task-bound requests
- affected protected capability

### Capability interactions

- IC-007: Experience reads catalogue/explanation and requests Workspace-read authority.
- IC-008: Experience resolves challenge/revokes/updates automatic policy.
- IC-010: Permission Authority publishes visible challenge/revoke.
- IC-011: Companion receives task decision/revocation.
- IC-012: affected capability receives revocation.

### Permission checks

- Permission Authority alone grants/revokes.
- Challenges are not authority.
- Proofs are purpose/target/operation bound.
- Automatic policy still passes authorize/validate.

### Explainability points

- Scope, requester, purpose, target, decision basis, grant, expiry, and revocation path.

### Persistence points

- Permission catalogue, grants, policies, proof state, and content-free audit remain local.

### Recovery/failure paths

- Authority unavailable means new meaningful actions fail closed.
- Unpresented challenges remain denied.
- Revocation blocks new effects and invokes owner safety policy.

### Architectural risks and gaps

1. Experience’s declared produced commands omit automatic-policy update even though IC-008 permits it.
2. Consent can satisfy information fields while still using repeated prompts, urgency inflation, asymmetric controls, preselection, or punishment after denial.

### Validation

- Required capabilities exist: **Yes**
- Required interaction contracts exist: **Minor inconsistency**
- Permission boundaries respected: **Yes**
- Explainability satisfied: **Information yes; fairness not enforceable**
- Blueprint compliance: **No, until “never manipulative” has acceptance invariants**
- ADR compliance: **ADR-0003 semantics are strong but presentation acceptance is incomplete**

### Minimum required architecture change

Add consent acceptance invariants:

- neutral wording
- no preselected grant
- equivalent grant/deny accessibility
- no fabricated urgency
- no degraded unrelated service as punishment for refusal
- bounded re-prompting
- denial respected until purpose/context materially changes or the user reopens the choice

Align Experience’s produced commands with IC-008.

---

## UJ-10 — Inspect Status and Explanations

**Outcome: Gap**

### User goal

Ask what Workspace is doing or did, why, which permission applied, which capability is responsible, and what action remains available.

### Participating capabilities

- Experience
- Companion Orchestration
- Permission Authority
- relevant domain owner

### Capability interactions

- IC-033: Experience requests task status/explanation.
- IC-014/019/024/027/030/035: Companion requests owner status/explanation/recovery.
- IC-034: Companion publishes explanation to Experience.
- IC-007: Experience inspects permission catalogue/decision explanation.

### Permission checks

- Protected domain explanations require appropriate task/read/admin authority.
- Proof content and unrelated data are excluded.
- Historical terminal lookup requires user administration authority.

### Explainability points

- What, why, responsible/requester capability, target summary, permission, local/remote boundary, extension identity, uncertainty/partial effects, and available action.

### Persistence points

- In-flight detailed explanation context is bounded.
- Durable audit uses content-free classes.
- Detailed long-term rationale exists only if explicitly retained through Memory.

### Recovery/failure paths

- Missing event becomes outcome-unknown.
- Owner status/tombstone reconciliation prevents fabricated success.
- Expired history is disclosed honestly.

### Architectural risks and gaps

1. IC-033 is not permitted by the narrower Experience → Companion matrix cell.
2. Historical explanation is intentionally privacy-bounded: detailed rationale requires explicit Memory retention, while content-free terminal/permission history remains available for the declared retention horizon.

### Validation

- Required capabilities exist: **Yes**
- Required interaction contracts exist: **Contradictory: the current Matrix forbids inspection that Capability Architecture/Contracts require**
- Permission boundaries respected: **Yes**
- Explainability satisfied: **Yes once the user inspection path is permitted**
- Blueprint compliance: **No until inspection path is permitted**
- ADR compliance: **ADR-0006 is not end-to-end enforceable**

### Minimum required architecture change

- Permit task status/explanation in the Experience → Companion matrix cell.

Concrete retention durations and UX wording remain future policy/Experience detail constrained by `10` and `11`, not a missing architecture contract.

---

## UJ-11 — Cancel Work and Recover Control

**Outcome: Gap**

### User goal

Cancel a Companion task and stop every feasible child operation without granting new effects.

### Participating capabilities

- Experience
- Companion Orchestration
- Context Sensing
- Intelligence
- Action
- Workspace Management
- Memory
- Extension Host if active
- Permission Authority for originally issued operation control

### Capability interactions

- IC-032: Experience requests task cancellation.
- IC-023: Companion pauses/stops sensing.
- IC-030: Companion cancels Action.
- Common owner status/terminal lookup applies to accepted domain operations.
- Permission revocation events stop new protected effects.

### Permission checks

- Cancellation uses operation reference/control proof.
- It cannot create effects, retry, compensate, or expand scope.
- Effect-authority revocation does not disable safety reduction.

### Explainability points

- Cancellation requested, accepted, too late, unsafe, partial, cancelled, completed race, or indeterminate.

### Persistence points

- Companion owns in-flight task/operation correlation.
- Owners retain bounded content-free terminal tombstones.

### Recovery/failure paths

- Control-proof lease expiry safely pauses/cancels where possible.
- Irreversible effects may complete but no later effect begins.
- Missing terminal evidence is reconciled before retry.

### Architectural risks and gaps

1. Experience → Companion cancellation is not permitted by the current matrix wording.
2. Companion cancellation does not define an invariant that enumerates every accepted child operation, issues safety control to each cancellable owner, waits/reconciles terminal states, and reports residual irreversible work.
3. Intelligence exposes no cancel command.
4. Workspace/Memory/Extension commands do not declare cancellability/commit points.
5. Companion crash recovery does not define durable, protected custody of in-flight operation identity/reference/control proof. These values are excluded from audit, and terminal lookup still needs the operation identity.

### Validation

- Required capabilities exist: **Yes**
- Required interaction contracts exist: **Incomplete/forbidden**
- Permission boundaries respected: **Yes in defined owner-control paths**
- Explainability satisfied: **Yes for owner outcomes**
- Blueprint compliance: **No; human control is not end-to-end**
- ADR compliance: **ADR-0003 and ADR-0006 require completion**

### Minimum required architecture change

Define Companion task-cancellation propagation:

1. stop accepting new child work
2. enumerate accepted child operations
3. issue pause/stop/cancel where supported
4. mark irreversible children and prevent later effects
5. reconcile each owner terminal state
6. publish one aggregate cancellation result with residual effects

Define bounded, protected custody/recovery of in-flight operation references/control proofs. This is task-control state, not audit or durable user knowledge.

---

## UJ-12 — Recover from Degradation, Timeout, Partial, or Indeterminate Work

**Outcome: Conditional**

### User goal

Remain informed and in control when a capability is unavailable or an operation cannot prove success.

### Participating capabilities

- Runtime Host
- Experience
- Companion Orchestration
- affected domain owner
- Permission Authority

### Capability interactions

- IC-002/003/004: health aggregation and status.
- IC-014/019/024/027/030/035: owner status and delayed terminal lookup.
- IC-018/022/025/028/031/038: domain outcome events.
- IC-033/034: task inspection and presentation.

### Permission checks

- Recovery identifiers grant no authority.
- Live status uses operation control.
- Delayed content-free lookup uses user administration authority.
- No blind retry after unknown/partial/indeterminate effects.

### Explainability points

- Unavailable, degraded, timeout, outcome-unknown, partial, indeterminate, retry eligibility, and user action.

### Persistence points

- Owners keep bounded content-free terminal/deduplication tombstones.
- Runtime Host keeps domain-free health.
- Companion keeps metadata-only task outcomes/correlation.

### Recovery/failure paths

- Event loss causes owner reconciliation.
- Missing local prerequisite is distinguished from internet dependency.
- Expired history is honest and never treated as success.

### Architectural risks and gaps

1. User-requested task status is forbidden by matrix wording.
2. Companion crash can lose the operation identity needed for recovery.
3. Experience availability during degraded startup is ambiguous.

### Validation

- Required capabilities exist: **Yes**
- Required interaction contracts exist: **Strong owner model; weak user/control path**
- Permission boundaries respected: **Yes**
- Explainability satisfied: **Conditional**
- Blueprint compliance: **Conditional**
- ADR compliance: **Conditional on ADR-0001 and ADR-0006 end-to-end access**

### Minimum required architecture change

Resolve UJ-01, UJ-10, and UJ-11. No new recovery capability is justified.

---

## UJ-13 — Shut Workspace Down Safely

**Outcome: Gap**

### User goal

Request shutdown, see consequential work settle or report residual effects, and know when Workspace has stopped.

### Participating capabilities

- Experience
- Runtime Host
- Companion Orchestration
- Action
- Context Sensing
- Intelligence
- Extension Host if active
- Memory
- Workspace Management
- Permission Authority

### Capability interactions

- Runtime Host owns ordered lifecycle shutdown through IC-001.
- Capabilities report health through IC-002.
- Runtime Host reports aggregate/final state through IC-003.
- Capability Architecture defines cancellation, buffer clearing, persistence, proof invalidation, final presentation, and process exit order.

### Permission checks

- User-initiated shutdown is explicit direct administration.
- Shutdown authority grants no domain permission.
- No new protected work is accepted after shutdown begins.

### Explainability points

- Shutting down, cancellation/settlement outcomes, residual/irreversible effects, and final stopped state.

### Persistence points

- Safe Memory writes flush.
- active Workspace scope persists.
- permission audit flushes and unused proofs invalidate.
- ephemeral Context/Experience buffers clear.

### Recovery/failure paths

- Timeout or unsafe cancellation is reported before Experience closes.
- Host exits only after ordered release.

### Architectural risks and gaps

The architecture says shutdown may be requested through Experience and Host exposes `Host.shutdown`, but Experience → Runtime Host is permitted only for status queries. Companion also has no Host shutdown path. The user-initiated shutdown contract therefore has no permitted capability caller.

### Validation

- Required capabilities exist: **Yes**
- Required interaction contracts exist: **No permitted user initiator**
- Permission boundaries respected: **Cannot be evaluated end-to-end**
- Explainability satisfied: **Sequence yes; invocation no**
- Blueprint compliance: **No; direct human control is incomplete**
- ADR compliance: **ADR-0003 and ADR-0006 require an explicit path**

### Minimum required architecture change

Permit Experience → Runtime Host explicit shutdown administration:

- request and acknowledgement
- no domain authority
- visible transition to shutting down
- bounded wait with cancellation/residual outcomes
- final state presented before Experience closes

This path must remain available when Companion is unavailable.

---

## Optional Modality Validation — Voice

**Outcome: Not ready for activation; not a core Blueprint blocker**

Voice correctly remains an Experience modality rather than a capability.

Existing architecture covers:

- explicit microphone scope
- fallback to non-voice UI
- ephemeral interaction buffers

Missing before voice activation:

- Experience → Permission Authority authorization purpose for microphone/overlay modality scopes
- visible listening state
- start/stop/listening control
- accidental activation handling
- turn-taking and barge-in
- output cancellation
- buffer clearing and retention statement
- attention/fairness invariants

Voice technology research should not select a runtime until these modality contracts are accepted.

---

## Optional Extension Validation

**Outcome: Correctly dormant**

Extension Host is not required for any core Blueprint journey.

The boundary is justified as a future trust boundary, but implementation remains blocked by ADR-0004 until user value is established.

Before activation, extensions must inherit:

- attention budget and consent fairness rules
- cancellation propagation
- operation recovery
- identity-preserving permission mediation
- no direct domain access

No new core capability is required.

---

## Cross-Journey Validation

### Capability completeness

No missing core capability was identified.

The ten-capability decomposition covers:

- lifecycle
- permission
- organization
- memory
- sensing
- action
- intelligence
- orchestration
- presentation
- optional extension isolation

Adding a separate Shutdown, Privacy, Attention, Recovery, Voice, Cloud, or Onboarding capability would duplicate existing ownership and violate the Complexity Budget/Single Responsibility principles.

### Contract completeness

Contract coverage is not complete.

Required corrections:

- Experience → Companion cancel/status/explain purposes
- Experience → Runtime Host shutdown administration
- sensing subscribe/unsubscribe/session-reference lifecycle
- Companion child-operation cancellation fan-out
- Intelligence cancellation
- local provider configuration/resource-cap administration
- multi-scope remote authorization
- protected in-flight control-state recovery after Companion crash

### Permission-boundary validation

Strengths:

- Permission Authority is sole authority.
- point-of-use effect validation is explicit.
- observation and retention are separate.
- effect and operation-control authority are separate.
- remote use is optional and cannot silently replace local behavior.
- extension identity remains attenuated.

Gaps:

- local provider administration authority is unspecified.
- remote run plus data-sharing scope composition is ambiguous.
- optional Experience modality authorization path is absent.

### Explainability validation

Strengths:

- what, why, permission, capability, target, state, uncertainty, partial effects, and user action are modeled.
- acceptance is distinct from completion.
- unknown and indeterminate outcomes are honest.
- Experience cannot rewrite domain meaning.

Gaps:

- user-requested task inspection is forbidden by matrix wording.
- attention/interruptibility semantics are absent.
- consent fairness/non-manipulation is not tested.

### Blueprint validation

**Local First:** structurally strong; degraded startup classification remains ambiguous.

**Human First:** not fully satisfied until cancellation, shutdown, degraded access, and interaction fairness are end-to-end.

**Privacy First:** strong; archived-workspace Memory visibility needs clarification.

**Permission Before Automation:** strong for normal effects; provider administration and remote multi-scope proofing need clarification.

**Explain Every Action:** strong metadata and privacy-bounded history; user inspection path needs matrix correction.

**Build Only Where We Create Value / Integrate Before Reinventing:** preserved; no technology chosen.

**Complexity Must Justify Itself:** preserved if gaps extend existing capabilities rather than add subsystems.

**Documentation Never Lives In Runtime:** preserved.

**Reduce Cognitive Load:** not enforceable without attention/notification/consent rules.

### Companion-principle validation

- Calm: **Not enforceable**
- Helpful: **Supported**
- Honest: **Supported**
- Predictable: **Conditional**
- Transparent: **Supported informationally**
- Respectful: **Conditional**
- Never manipulative: **Not enforceable**

---

## Gap Analysis

### Missing capabilities

None.

### Missing or contradictory contracts

#### GAP-01 — Experience task control and inspection

The matrix does not permit Experience → Companion cancel/status/explanation even though Capability Contracts require them.

Required change: widen only that matrix purpose and keep all domain effects Companion-mediated.

#### GAP-02 — User-initiated shutdown

Host exposes shutdown, but no capability may invoke it for the user.

Required change: direct Experience → Host shutdown administration.

#### GAP-03 — Sensing subscription and session control

Subscription mutates state as a request, no unsubscribe exists, and session reference semantics are incomplete.

Required change: explicit bounded subscription/session lifecycle.

#### GAP-04 — Task cancellation propagation and crash recovery

Companion cancel does not guarantee child fan-out/reconciliation; in-flight control metadata custody after crash is undefined.

Required change: task-control invariant and protected bounded recovery state.

#### GAP-05 — Local provider administration

Intelligence owns local provider configuration/resource caps but exposes no configure/update/remove contract.

Required change: Companion-mediated local provider administration with separate narrow authority.

#### GAP-06 — Optional remote multi-scope authorization

Remote run and data sharing are independently required but proof composition is undefined.

Required change: explicit compound authorization-set semantics.

### Missing architectural guidance

#### GAP-07 — Startup dependency classes

Required change: essential/degradable dependency classes and Experience-first degraded access.

#### GAP-08 — Attention and non-manipulative consent

Required change: behavioral acceptance invariants for quiet/focus control, bounded prompting, neutral consent, and no fabricated urgency. Later research determines minimal metadata.

#### GAP-09 — Workspace/Memory lifecycle

Required change: archive-time active-scope fallback and archived-workspace Memory visibility.

#### GAP-10 — Per-operation cancellation declarations

Required change: every non-immediate command declares cancellability, commit point, partial boundary, and recovery; Action classes require this before implementation.

### Unnecessary complexity

No unjustified active capability was identified.

Documentation-level complexity remains:

- Capability Architecture repeats contract signatures that are more authoritative and more detailed in Capability Contracts. Some high-level signatures no longer show effect/control proof semantics.
- Interaction Matrix formal findings still describe some schema work as future even though Contract Schema is complete.

### Opportunities to simplify

1. Keep `08` authoritative for ownership/lifecycle and reference `10` for exact public contract signatures instead of duplicating them.
2. Keep `09` authoritative for permitted pairs/purposes and mechanically reconcile it with IC-001–038 after every contract change.
3. Express attention, cancellation, startup, and archive behavior as shared invariants over existing capabilities, not new subsystems.
4. Keep Extension Host dormant.
5. Treat voice as an Experience variant, not a new capability.

---

## Necessary Final Architecture Changes

Before final candidate selection for affected capabilities or implementation:

1. Reconcile Experience → Companion cancel/status/explanation in the Interaction Matrix.
2. Add direct user shutdown administration from Experience to Runtime Host.
3. Define essential versus degradable startup dependencies and guarantee Experience/status/permission access in degraded startup.
4. Complete sensing subscription/session control.
5. Define local provider configuration/resource-cap administration.
6. Define Companion cancellation fan-out, per-owner cancellation declarations, and crash-recoverable in-flight control custody.
7. Define attention/notification and non-manipulative consent acceptance invariants.
8. Define compound authorization for remote run plus data sharing before optional remote activation.
9. Define archived-workspace active-scope fallback and Memory visibility.

These changes extend existing contracts and guidance. No new capability is recommended.

---

## Readiness for Capability Technology Research

**Readiness: Ready for bounded Capability Technology Research; not ready for final selection in affected areas or implementation.**

The capability decomposition is complete and stable, but the user journey layer exposes interaction and lifecycle gaps that could materially alter:

- Runtime Host startup/shutdown requirements
- Companion task/cancellation requirements
- Context Sensing session/subscription requirements
- Permission proof composition
- Experience event/attention requirements
- local provider administration requirements
- archived-workspace Memory visibility
- Intelligence cancellation/control requirements

Candidate discovery/comparison may help test feasibility and close the open requirements, as required by ADR-0002. Research must record assumptions and must not silently decide unresolved architecture.

Final selection or implementation in an affected area remains blocked until its relevant architecture acceptance gates pass.

Extension Host and voice remain separately blocked until their optional journey requirements are accepted.

---

## Final Validation Result

### User journeys validated

- 13 core journeys were traced.
- No missing core capability was found.
- Local operation, permission isolation, data ownership, effect authorization, honest terminal outcomes, and extension isolation are architecturally strong.

### Architectural gaps

- 6 missing/contradictory contract groups.
- 4 missing guidance groups.
- no required new capability.

### Recommended final architecture changes

Ten gap groups are consolidated into nine bounded change packages above. All belong within existing capability ownership.

### Readiness

Workspace is **capability-complete but not user-journey-complete**.

Bounded Capability Technology Research may begin now. Final selection or implementation in an affected area waits for its relevant change packages to be resolved and revalidated.
