# Workspace Capability Interaction Matrix

Status: Active
Authority: Authoritative capability communication and trust-boundary constraints
Reviewed against: Workspace Capability Architecture v1.1

This document constrains communication between capabilities. It does not replace capability ownership in `08_Workspace_Capability_Architecture.md`, the accepted ADRs, or the Blueprint.

---

## Legend

- **Direct** — initiator may call the target's public contract for the stated purpose.
- **Event** — source may publish a typed event; subscribers gain no authority to command the source.
- **Lifecycle** — Runtime Host may start, stop, and query health only.
- **Intermediary** — direct communication is forbidden; the named capability is required.
- **Forbidden** — no direct call or event path is permitted.
- Contract responses are allowed on the initiating request path and do not create a reverse dependency.

Abbreviations: RH Runtime Host; PA Permission Authority; WM Workspace Management; ME Memory; CS Context Sensing; AC Action; IN Intelligence; CO Companion Orchestration; EX Experience; EH Extension Host.

---

## Direct Communication Matrix

Rows initiate; columns receive.

| From \ To | RH | PA | WM | ME | CS | AC | IN | CO | EX | EH |
|-----------|----|----|----|----|----|----|----|----|----|----|
| RH | — | Lifecycle | Lifecycle | Lifecycle | Lifecycle | Lifecycle | Lifecycle; Event: mode | Lifecycle | Lifecycle; Event: host status | Lifecycle |
| PA | Event: health only | — | Event: revoke | Event: revoke | Event: revoke | Event: revoke | Event: revoke | Event: challenge/decision/revoke | Event: challenge/decision/revoke | Event: revoke |
| WM | Event: health | Direct: validate | — | Event: scope changed | Event: scope changed | Event: scope changed | Forbidden | Event: scope/domain result | Forbidden | Forbidden |
| ME | Event: health | Direct: validate | Direct: validate scope | — | Forbidden | Forbidden | Forbidden | Event: result/policy | Forbidden | Forbidden |
| CS | Event: health | Direct: validate | Direct: scope tag | Forbidden | — | Forbidden | Forbidden | Event: context/candidate/state | Forbidden | Forbidden |
| AC | Event: health | Direct: validate proof | Direct: validate target scope | Forbidden | Forbidden | — | Forbidden | Event: execution result | Forbidden | Forbidden |
| IN | Event: health | Direct: validate provider/data sharing | Forbidden | Forbidden | Forbidden | Forbidden | — | Event: inference result/proposal/availability | Forbidden | Forbidden |
| CO | Event: health | Direct: authorize; explain task authorization | Direct | Direct | Direct | Direct: resolve plan or execute with the corresponding proof | Direct | — | Event: progress/interaction request | Direct: manage, only if activated |
| EX | Direct: status query; Event: health | Direct: authorize Workspace read; catalogue/explain user's permissions; user decision/revoke/automatic-policy update | Direct: read-only view with proof | Forbidden | Forbidden | Forbidden | Forbidden | Direct: intent/interaction result | — | Forbidden |
| EH | Event: health | Direct: validate extension scopes | Forbidden | Forbidden | Forbidden | Forbidden | Forbidden | Event: extension contribution/intent/error/state | Forbidden | — |

### Matrix interpretation rules

1. A cell permits only the stated purpose; it is not general access.
2. All meaningful mutations require Permission Authority authorization even when a direct domain call is allowed.
3. Action accepts plan resolution and execution only from Companion
   Orchestration. Plan resolution requires `action.plan.resolve`, which grants
   no effect authority; execution requires one effect proof per item bound to
   capability, purpose, subject, target, operation, item, and action type. No
   batch-wide or omnibus effect proof is permitted.
4. Experience may read Workspace views directly, but all mutations are intents through Companion Orchestration.
5. Extension Host has no direct path to domain capabilities. Companion Orchestration is mandatory for all extension-originated domain effects.
6. Intelligence never calls Context Sensing, Memory, Action, Experience, or Extension Host. Companion supplies approved context and mediates proposals.
7. Context Sensing never writes Memory directly. It may emit a candidate to Companion, which decides whether to initiate a permissioned Memory proposal.
8. Runtime Host lifecycle access does not grant access to capability-owned domain data.
9. Workspace Management, Memory, Context Sensing, Intelligence, Action, and an activated Extension Host validate bound authorization at their own access or commit boundary; an earlier Companion check is insufficient.
10. For a non-immediate protected operation, Permission Authority issues a separate operation-control proof with effect authority. The owner binds it to the accepted operation and may validate it locally for minimized status or safety-reducing pause/stop/cancel when Authority is unavailable. It grants no continuation, retry, compensation, or new effect.

---

## Required Intermediaries

| Origin | Destination | Required intermediary | Reason |
|--------|-------------|-----------------------|--------|
| Experience | Any domain mutation | Companion Orchestration, then Permission Authority | Preserve one orchestration authority and permission-before-action |
| Intelligence | Action | Companion Orchestration, then Permission Authority | Model output is a proposal, never execution authority |
| Intelligence | Memory or Context Sensing | Companion Orchestration | Prevent provider/model layer from acquiring ambient user context |
| Context Sensing | Memory | Companion Orchestration, then Permission Authority | Observation is not consent to retain |
| Extension Host | Any domain capability | Companion Orchestration, then Permission Authority where meaningful | Attenuate extension authority and retain explainability |
| Any capability | User-facing rendering | Experience | Keep presentation ownership cohesive |
| Any meaningful operation | Execution/commit point | Permission Authority authorization proof | Enforce explicit, revocable, purpose-bound authority |

---

## Authority Boundaries

| Authority | Sole owner | Constraint |
|-----------|------------|------------|
| Product principles | Blueprint | No capability document may override it |
| Architecture decisions | Accepted ADRs | Capability documents implement, not redefine, decisions |
| Capability responsibilities | Capability Architecture | Interaction Matrix may constrain communication but not transfer ownership |
| Permission truth | Permission Authority | Other capabilities may request and validate; never grant |
| User intent/task sequencing | Companion Orchestration | Intelligence and extensions may propose; never sequence core work |
| Environment mutation | Action | Requires bound authorization and Companion request |
| Long-term remembered knowledge | Memory | Observation buffers and task state are not alternate memory stores |
| Workspace organization | Workspace Management | Memory may reference scope but cannot redefine it |
| Human presentation | Experience | Other capabilities emit structured explanation data only |
| Process lifecycle | Runtime Host | Lifecycle authority gives no domain authority |

---

## Trust and Privacy Boundaries

1. **Local process boundary:** Runtime Host is trusted for lifecycle and routing, not for domain data access.
2. **Authorization boundary:** Permission Authority is fail-closed. Authorization proofs are narrow, short-lived or single-use where appropriate, and invalid when revoked or context-mismatched. Every protected capability validates at point of use.
3. **Observation boundary:** Context Sensing owns ephemeral raw observations. Every event crossing the boundary is purpose-limited and minimized, including events produced under deep-sensing permission; durable retention requires a separate Memory authorization path.
4. **Execution boundary:** Action is the only environment-mutation boundary. It revalidates authorization at the point of use.
5. **Provider boundary:** Intelligence treats remote providers as an optional external trust boundary. Data may leave the device only with explicit provider and data-sharing authorization; there is no silent cloud fallback.
6. **Extension boundary:** Extensions are untrusted relative to core capabilities. Extension Host attenuates identity and scopes; Companion mediates domain requests.
7. **Presentation boundary:** Experience receives only the data needed to inform the user and collect decisions. Rendering does not imply domain authority.

---

## Permission Boundaries

- Direct user interaction is not an implicit permanent grant.
- Automatic-execution configuration is an explicit Permission Authority policy; every operation still passes authorization and point-of-use validation.
- Point-of-use validation applies to Workspace reads/mutations, Memory reads/writes/forgetting, sensor activation/deep queries, protected local or remote Intelligence, Action execution, and extension management.
- Revocation prevents new operations immediately. In-flight work follows the owning capability's declared safe cancellation policy and reports the outcome.
- Revoking/expiring effect authority does not disable the separately governed operation-control proof for minimized status and safety-reducing control. It is Permission Authority-issued, owner-bound, and cannot create effects.
- Read and write scopes remain distinct for Workspace and Memory.
- Basic and deep sensing remain distinct.
- Local and remote Intelligence use remain distinct; remote data sharing is separately declared.
- Extension permissions are bound to extension identity and are never inherited from Companion or the user session.
- No omnibus action, extension, sensing, memory, or provider permission is permitted.

---

## Event Flow Boundaries

| Event class | Publisher | Allowed subscribers | Prohibited content |
|-------------|-----------|---------------------|--------------------|
| Capability health | All capabilities | Runtime Host only | Domain payloads, secrets, raw observations |
| Aggregated host status | Runtime Host | Experience | Domain payloads, secrets, raw observations |
| Runtime mode | Runtime Host | Intelligence | Domain payloads, connectivity history, network content |
| Permission challenge/decision/revocation | Permission Authority | Experience; affected capability; Companion | Unrelated grants or audit history |
| Workspace scope/domain result | Workspace Management | Companion for scope/results; Memory/Context/Action for scope invalidation only | Workspace content not required by subscriber |
| Context/state event | Context Sensing | Companion only | Raw or unminimized capture; deep-sensing permission never waives minimization |
| Memory result/policy | Memory | Companion only | Unrequested memory collections |
| Inference result/proposal/availability | Intelligence | Companion (result/proposal limited to requesting task) | Execution authority, prompts, unrelated task content |
| Action progress/result | Action | Requesting Companion task; Runtime Host for health only | New action requests or expanded scope |
| Companion progress/interaction request | Companion | Experience | Hidden action execution or permission grants |
| Extension contribution/error/state | Extension Host | Companion; Runtime Host for health; Experience only through Companion explanation | Direct domain commands, extension partition content |

Events are typed, purpose-labelled, correlated to a task or operation where applicable, and carry the minimum data required. Event delivery never grants command authority.

---

## Correlation Boundaries

Every cross-capability message uses the correlation envelope defined by the Capability Architecture:

- `task_id` correlates work for a user intent and is owned by Companion.
- `operation_id` identifies a committing capability's operation and is owned by that capability.
- `authorization_id` identifies the Permission Authority decision/proof.
- `causation_id` links only to the immediate predecessor.
- `requester_capability` and optional `extension_id` preserve accountability across intermediaries.

Correlation identifiers contain no user content and grant no authority. Capabilities retain separate records and expose only the minimum correlated metadata needed for explainability.

---

## Formal Architectural Findings

### Critical

No critical findings.

### Major

#### FINDING-MAJ-001 — Circular Experience/Companion dependency

- **Description:** v1.0 listed Experience as a Companion dependency while Experience also depended on Companion, creating a hard cycle that the validation text dismissed as a UI cycle.
- **Why it matters:** Cycles obstruct independent startup, failure isolation, testing, and replacement; dismissing the cycle contradicted the document's acyclic claim.
- **Recommendation:** Make dependency one-way: Experience calls Companion; Companion publishes presentation-neutral progress and interaction-request events. Responses return through Companion's contract.
- **ADRs affected:** ADR-0004, ADR-0005, ADR-0006.
- **Resolution:** Applied in Capability Architecture v1.1.

#### FINDING-MAJ-002 — Authorization susceptible to scope drift and time-of-check/time-of-use ambiguity

- **Description:** v1.0 passed a generic `permission_ref` or check result to Action without defining binding to requester, purpose, target, operation, expiry, or revocation.
- **Why it matters:** A broad or stale approval could be replayed or used for a different target, undermining explicit and revocable permission.
- **Recommendation:** Permission Authority issues bound authorization proofs; Action validates the exact execution context immediately before execution and rejects revoked, expired, replayed, or mismatched proofs.
- **ADRs affected:** ADR-0003, ADR-0006.
- **Resolution:** Applied in Capability Architecture v1.1 and formalized in this matrix.

#### FINDING-MAJ-003 — Extension Host had an unrestricted core-capability path

- **Description:** v1.0 allowed Extension Host to call “any capability” through public contracts.
- **Why it matters:** Public contracts do not by themselves establish trust. This path enabled extensions to bypass the sole orchestrator, increased coupling, and risked inconsistent permissions and explanations.
- **Recommendation:** Extensions communicate only with Extension Host; Extension Host communicates with Companion and Permission Authority. Companion mediates all domain effects.
- **ADRs affected:** ADR-0003, ADR-0004, ADR-0005, ADR-0006.
- **Resolution:** Applied in Capability Architecture v1.1.

#### FINDING-MAJ-004 — Observation could flow directly into durable Memory

- **Description:** v1.0 permitted Context Sensing to send Memory write proposals directly.
- **Why it matters:** Permission to observe is not permission to retain. Direct coupling also bypassed Companion's task purpose and created a privacy-sensitive persistence path.
- **Recommendation:** Context Sensing emits a minimized candidate event only to Companion; Companion initiates a separately authorized Memory proposal when justified.
- **ADRs affected:** ADR-0001, ADR-0003, ADR-0005, ADR-0006.
- **Resolution:** Applied in Capability Architecture v1.1.

#### FINDING-MAJ-005 — Deep-sensing permission could waive minimization

- **Description:** The first v1.1 draft required minimized sensing events generally but allowed unminimized capture to cross the event boundary when deep-sensing authorization existed.
- **Why it matters:** Authorization to inspect a sensitive source does not authorize unnecessary disclosure to another capability; this would weaken Privacy First.
- **Recommendation:** Raw observations remain inside Context Sensing. Every emitted event is purpose-limited and minimized, including deep-sensing events.
- **ADRs affected:** ADR-0001, ADR-0003, ADR-0005, ADR-0006.
- **Resolution:** Applied in Capability Architecture v1.1 and this matrix.

#### FINDING-MAJ-006 — Point-of-use authorization was specified only for Action

- **Description:** The first v1.1 draft claimed every meaningful operation used point-of-use validation, but Workspace, Memory, Context Sensing, Intelligence, and Extension Host contracts did not accept or validate proofs.
- **Why it matters:** An earlier authorization check can become stale, be replayed, or drift from the operation actually committed.
- **Recommendation:** Every protected capability accepts a bound proof and validates it against the exact access or commit context.
- **ADRs affected:** ADR-0003, ADR-0005, ADR-0006.
- **Resolution:** Applied across the v1.1 capability contracts and this matrix.

#### FINDING-MAJ-007 — Matrix prohibited required point-of-use validation calls

- **Description:** The second review found that protected capability-to-Permission-Authority cells permitted authorization but, except for Action, did not permit validation.
- **Why it matters:** Because matrix cells allow only their stated purpose, the matrix prohibited the validation required by the capability contracts.
- **Recommendation:** Permit both authorization and point-of-use validation for Workspace, Memory, Context Sensing, Intelligence, and activated Extension Host.
- **ADRs affected:** ADR-0003, ADR-0005, ADR-0006.
- **Resolution:** Applied in this matrix.

#### FINDING-MAJ-008 — Experience lacked a Workspace-read authorization path

- **Description:** Experience could directly query permissioned Workspace read contracts but could not request a read authorization proof.
- **Why it matters:** The only viable outcomes were an authorization bypass or an unusable read path.
- **Recommendation:** Permit Experience to request purpose-bound `workspace.read` authorization and pass the proof to read-only Workspace contracts; mutations remain Companion-mediated.
- **ADRs affected:** ADR-0003, ADR-0005, ADR-0006.
- **Resolution:** Applied in this matrix.

#### FINDING-MAJ-009 — Memory explanation bypassed read authorization

- **Description:** `Memory.explain(item_id)` could reveal remembered content or provenance without a proof.
- **Why it matters:** Explanation must not become an alternate unpermissioned read API.
- **Recommendation:** Require and validate a bound `memory.read` authorization proof for item explanations.
- **ADRs affected:** ADR-0003, ADR-0005, ADR-0006.
- **Resolution:** Applied in Capability Architecture v1.1.

#### FINDING-MAJ-010 — Experience Workspace-read authorization path remained incomplete

- **Description:** The matrix allowed Experience to request a Workspace-read proof, while the Experience outputs and dependency graph described Authority communication as user decisions only.
- **Why it matters:** The same path was simultaneously required and omitted, leaving an implementation to bypass proof requirements or fail.
- **Recommendation:** Declare Experience's purpose-bound Workspace read authorization request in its outputs, dependency graph, and interaction summary.
- **ADRs affected:** ADR-0003, ADR-0005, ADR-0006.
- **Resolution:** Applied in Capability Architecture v1.1.

### Minor

#### FINDING-MIN-001 — Intelligence/Memory contract contradiction

- **Description:** Memory accepted direct retrieval requests from Intelligence, while Intelligence explicitly had no Memory dependency and expected caller-supplied snippets.
- **Why it matters:** Contradictory contracts create accidental coupling and risk ambient model access to memory.
- **Recommendation:** Permit Memory retrieval only through Companion; Companion supplies minimized context to Intelligence.
- **ADRs affected:** ADR-0003, ADR-0005.
- **Resolution:** Applied in Capability Architecture v1.1.

#### FINDING-MIN-002 — Experience could mutate Workspace outside orchestration

- **Description:** v1.0 allowed direct Experience-to-Workspace mutation, despite Companion being the sole coordinator of meaningful work.
- **Why it matters:** Two mutation paths complicate permission consistency and explanations.
- **Recommendation:** Keep direct Experience-to-Workspace access read-only; submit mutations as Companion intents.
- **ADRs affected:** ADR-0003, ADR-0005, ADR-0006.
- **Resolution:** Applied in Capability Architecture v1.1.

#### FINDING-MIN-003 — Shutdown hid outcomes before side effects settled

- **Description:** v1.0 closed Experience before Action, sensing, and inference shutdown completed.
- **Why it matters:** The user could lose visibility while consequential operations were still cancelling or completing.
- **Recommendation:** Keep Experience available until cancellation outcomes and final shutdown state are presented.
- **ADRs affected:** ADR-0003, ADR-0006.
- **Resolution:** Applied in Capability Architecture v1.1.

#### FINDING-MIN-004 — Dependency graph notation was ambiguous

- **Description:** The original vertical graph mixed dependency direction with event return paths and did not show several declared dependencies.
- **Why it matters:** Ambiguity makes circularity and forbidden paths hard to detect.
- **Recommendation:** Define arrow semantics explicitly and pair the graph with an authoritative interaction matrix.
- **ADRs affected:** ADR-0004, ADR-0005, ADR-0008.
- **Resolution:** Applied in Capability Architecture v1.1 and this document.

#### FINDING-MIN-005 — Intelligence logs could become alternate Memory

- **Description:** The first v1.1 draft allowed persisted inference transaction logs without excluding prompts, responses, or retrieved memories.
- **Why it matters:** Content-bearing logs would bypass Memory's retention, redaction, provenance, and forgetting authority.
- **Recommendation:** Intelligence persists metadata only. Any retained prompt/response content must be proposed to Memory through Companion.
- **ADRs affected:** ADR-0001, ADR-0003, ADR-0005.
- **Resolution:** Applied in Capability Architecture v1.1.

#### FINDING-MIN-006 — Activated Extension Host had no authorized management caller

- **Description:** The first matrix prohibited both Experience and Companion from calling Extension Host, leaving install/unload/list contracts unreachable.
- **Why it matters:** A dormant boundary that cannot be safely administered would force an undocumented bypass when activated.
- **Recommendation:** Allow Companion to call activated Extension Host management contracts with bound authorization; Extension Host publishes contributions back as events.
- **ADRs affected:** ADR-0003, ADR-0004, ADR-0005, ADR-0006.
- **Resolution:** Applied in Capability Architecture v1.1 and this matrix.

#### FINDING-MIN-007 — Lifecycle event table contradicted the direct matrix

- **Description:** The first matrix allowed all capability health events to Experience while marking those paths forbidden.
- **Why it matters:** Conflicting allow/deny rules make the trust boundary unenforceable.
- **Recommendation:** Capabilities report health only to Runtime Host; Runtime Host publishes an aggregate, domain-free host status to Experience.
- **ADRs affected:** ADR-0005, ADR-0006.
- **Resolution:** Applied in this matrix.

#### FINDING-MIN-008 — Permission domain events leaked into Runtime Host

- **Description:** The first matrix sent Permission Authority revocation events to Runtime Host despite Host's lifecycle-only authority.
- **Why it matters:** It exposes domain data to infrastructure and contradicts the stated trust boundary.
- **Recommendation:** Runtime Host receives health only; revocation goes to affected capabilities, Companion, and Experience where relevant.
- **ADRs affected:** ADR-0003, ADR-0005.
- **Resolution:** Applied in this matrix.

#### FINDING-MIN-009 — Workspace scope events contradicted forbidden matrix cells

- **Description:** The event table allowed Memory, Context Sensing, and Action to receive scope changes while the matrix forbade those Workspace-to-capability event paths.
- **Why it matters:** Conflicting constraints make scope invalidation unreliable and the matrix non-authoritative.
- **Recommendation:** Permit purpose-limited scope-change events to capabilities that validate scoped access.
- **ADRs affected:** ADR-0005, ADR-0006.
- **Resolution:** Applied in this matrix.

#### FINDING-MIN-010 — Permission revocation delivery to Experience was contradictory

- **Description:** The event table allowed Experience to receive revocation updates while its matrix cell allowed challenges only.
- **Why it matters:** Experience must accurately show current permission state, and authoritative rules cannot disagree.
- **Recommendation:** Permit both challenge and revocation events to Experience.
- **ADRs affected:** ADR-0003, ADR-0006.
- **Resolution:** Applied in this matrix.

#### FINDING-MIN-011 — Capability health mixed direct-call and event semantics

- **Description:** Capability-to-Host health was represented as direct calls in the matrix and events in the event table.
- **Why it matters:** Direct calls and events create different dependency semantics.
- **Recommendation:** Standardize capability health as events to Runtime Host; retain separate Experience-to-Host status queries.
- **ADRs affected:** ADR-0004, ADR-0005.
- **Resolution:** Applied in this matrix.

#### FINDING-MIN-012 — Persisted Companion history could become alternate Memory

- **Description:** Companion permitted unspecified persisted task history despite Memory being the sole owner of retained user knowledge.
- **Why it matters:** Task history may contain intents, plans, responses, and user context that bypass Memory retention, redaction, and forgetting.
- **Recommendation:** Persist only orchestration policy and metadata-only outcome/correlation records; route retained user content through a separately authorized Memory proposal.
- **ADRs affected:** ADR-0001, ADR-0003, ADR-0005.
- **Resolution:** Applied in Capability Architecture v1.1.

#### FINDING-MIN-013 — Runtime mode delivery to Intelligence was reversed

- **Description:** Intelligence consumes Host-owned offline/online mode, but the matrix placed `mode` in the Intelligence-to-Host direction.
- **Why it matters:** Local First fallback cannot be deterministic if the consumer has no permitted mode-delivery path.
- **Recommendation:** Runtime Host publishes mode to Intelligence; Intelligence reports health to Host.
- **ADRs affected:** ADR-0001, ADR-0005, ADR-0006.
- **Resolution:** Applied in this matrix and the Capability Architecture graph.

#### FINDING-MIN-014 — Workspace understated its Permission Authority dependency

- **Description:** Workspace declared Permission Authority only for mutations even though read contracts also require proof validation.
- **Why it matters:** Startup and degraded-mode decisions could incorrectly treat protected reads as available without Authority.
- **Recommendation:** Declare Permission Authority as a dependency for all protected reads and mutations.
- **ADRs affected:** ADR-0003, ADR-0005.
- **Resolution:** Applied in Capability Architecture v1.1.

#### FINDING-MIN-015 — Extension event was drawn as a hard call cycle

- **Description:** The call-dependency graph showed Extension Host calling Companion while Companion could call Extension Host management, despite the matrix defining the return path as an event.
- **Why it matters:** The graph appeared to reintroduce the hard cycle that the architecture claimed to remove.
- **Recommendation:** Remove Extension Host-to-Companion from the call graph and label it explicitly as an event-only path.
- **ADRs affected:** ADR-0004, ADR-0005.
- **Resolution:** Applied in Capability Architecture v1.1.

#### FINDING-MIN-016 — Graph mixed domain calls, lifecycle, and events

- **Description:** The graph defined plain arrows as contract calls but used the same notation for capability health and Host mode events, and described capability-to-Host health as lifecycle.
- **Why it matters:** Mixed semantics obscure dependency direction and can falsely introduce cycles.
- **Recommendation:** Limit the graph to domain calls; define lifecycle and event paths only in the Interaction Matrix and explanatory notes.
- **ADRs affected:** ADR-0004, ADR-0005, ADR-0008.
- **Resolution:** Applied in Capability Architecture v1.1.

#### FINDING-MIN-017 — Host mode event was absent from the event-flow table

- **Description:** The matrix allowed Runtime Host to publish mode to Intelligence, but the event-flow table omitted the event class.
- **Why it matters:** An authoritative event policy must define allowed subscribers and prohibited content for Local First mode delivery.
- **Recommendation:** Add a Runtime mode event restricted to Intelligence and prohibit domain/network-content payloads.
- **ADRs affected:** ADR-0001, ADR-0005, ADR-0006.
- **Resolution:** Applied in this matrix.

#### FINDING-MIN-018 — Experience Permission dependency annotation was incomplete

- **Description:** Experience's outputs included Workspace-read authorization, but its dependency text mentioned challenge completion only.
- **Why it matters:** Incomplete dependency declarations undermine readiness and failure-mode reasoning.
- **Recommendation:** Include Workspace read authorization and user permission decisions in the Experience-to-Authority dependency description.
- **ADRs affected:** ADR-0003, ADR-0005.
- **Resolution:** Applied in Capability Architecture v1.1.

### Observation

#### FINDING-OBS-001 — Extension Host remains a complexity-budget checkpoint

- **Description:** Plugin architecture is still a known unknown, yet Extension Host is represented in the foundational decomposition.
- **Why it matters:** Implementing a sandbox and extension lifecycle prematurely would create substantial complexity.
- **Recommendation:** Treat the boundary as authoritative but the runtime subsystem as dormant; do not implement it until plugin value and requirements are established through the normal architecture/research process.
- **ADRs affected:** ADR-0002, ADR-0004.
- **Resolution:** Remaining risk; no implementation authorized.

#### FINDING-OBS-002 — Distributed audit records require correlation semantics

- **Description:** Permission, Action, Intelligence, and Extension Host each own distinct audit or transaction records.
- **Why it matters:** Without shared correlation identifiers, complete explanations may be difficult; centralized ownership would instead create an oversized subsystem.
- **Recommendation:** Define common task, authorization, and operation correlation identifiers at contract-design time while preserving separate data ownership.
- **ADRs affected:** ADR-0004, ADR-0005, ADR-0006.
- **Resolution:** Resolved architecturally with the correlation envelope in Capability Architecture v1.1 and this matrix; field representation remains future contract-definition work.

#### FINDING-OBS-003 — Degraded-mode policy is intentionally incomplete

- **Description:** Capabilities state local-first degradation principles but do not yet define which user outcomes constitute “core functionality.”
- **Why it matters:** Local First validation needs explicit capability-level service expectations.
- **Recommendation:** Define offline/degraded acceptance scenarios before implementation planning, without selecting technologies.
- **ADRs affected:** ADR-0001, ADR-0006.
- **Resolution:** Resolved at architecture level by the offline core-functionality acceptance scenarios in Capability Architecture v1.1; detailed test cases remain future validation work.

---

## Re-review Result

After the v1.1 revisions:

- No ownership conflicts remain.
- No circular hard dependencies remain.
- No Blueprint violations remain.
- No accepted ADR violations remain.
- No Critical or unresolved Major findings remain.
- Remaining items are pre-implementation contract and acceptance risks, not violations of the capability decomposition.

