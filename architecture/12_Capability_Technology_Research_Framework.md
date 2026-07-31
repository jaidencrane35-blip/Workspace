# Workspace Capability Technology Research Framework v1.0

Status: Active
Authority: Authoritative research-planning and technology-evaluation framework
Version: 1.0

This document defines how future engineering sessions research technology options for Workspace capabilities. It does not perform technology research, name or recommend libraries, select vendors, approve implementation, or alter capability ownership and contracts.

Authority order:

1. `00_Workspace_Blueprint.md`
2. Accepted ADRs in `architecture/decisions/`
3. `08_Workspace_Capability_Architecture.md` — capability ownership
4. `09_Capability_Interaction_Matrix.md` — permitted communication and trust boundaries
5. `10_Capability_Contracts.md` — public contract purposes and interactions
6. `11_Contract_Schema_and_Acceptance_Specification.md` — conceptual schema and acceptance gates
7. This document — research process and evidence requirements

Higher authority wins on conflict.

---

## Purpose

The framework makes technology research repeatable, comparable, evidence-based, and replaceable across AI and engineering sessions.

It exists to:

- satisfy ADR-0002 by researching common capabilities before implementation
- preserve Local First, Privacy First, Permission First, explainability, and human control
- compare integration against internal construction without presuming either result
- test candidates against existing capability contracts rather than allowing technology to redefine architecture
- record each investigation exactly once in the Research Catalogue
- prevent unverified popularity, familiarity, or novelty from becoming an adoption decision

---

## Scope

The framework applies to technology research for:

1. Runtime Host
2. Permission Authority
3. Workspace Management
4. Memory
5. Context Sensing
6. Action
7. Intelligence
8. Companion Orchestration
9. Experience
10. Extension Host

Extension Host research may characterize feasibility and risk, but it does not activate or justify the subsystem. Activation still requires a separate Complexity Budget decision.

---

## Research Unit and Single-Record Rule

One research unit is one capability plus one clearly bounded technology category or architectural question.

Each research unit has exactly one canonical entry in `03_Research_Catalogue.md`. The entry links to evidence and any detailed comparison artifact; evidence is referenced rather than copied into competing records.

Rules:

- Search the Research Catalogue before beginning.
- Continue an existing open investigation when scope matches.
- Create a new entry only when the capability, category, or materially different question is distinct.
- Record candidates, rejected options, and the selected direction in the same research unit.
- Do not repeat research merely because a new session begins.
- Re-evaluation updates the existing entry with dated evidence and preserves prior conclusions.
- The Engineering Ledger records adoption, architecture, or implementation consequences; it does not duplicate research evidence.
- The Open Source Registry records only approved open-source dependencies; it does not replace the Research Catalogue.
- An ADR records a durable architectural decision when required; it does not replace the comparison record.

---

## Standard Research Lifecycle

Every research unit follows the same lifecycle.

### 1. Frame

- Name the owning capability and bounded research question.
- Cite the capability responsibility, allowed interactions, relevant contracts, and applicable acceptance cases.
- Separate mandatory requirements from preferences.
- State whether the question concerns a commodity integration category or unique Workspace value.

### 2. Discover

- Identify candidate classes and candidates from primary sources.
- Include internal construction as a comparison option where credible.
- Record discovery date, candidate version or revision, supported status, and source.
- Do not eliminate candidates solely by reputation or familiarity.

### 3. Screen

Reject or quarantine candidates that cannot satisfy non-negotiable gates:

- architecture ownership or communication boundaries
- Local First core operation
- privacy and data-minimization requirements
- permission and point-of-use validation semantics
- required platform compatibility
- acceptable licensing
- required contract behavior

### 4. Compare

- Apply the same criteria, scenarios, and evidence standard to every surviving candidate.
- Distinguish verified fact, measured result, reasoned inference, and unknown.
- Record material trade-offs; do not hide weaknesses behind a total score.
- Compare integration, internal construction, and hybrid boundaries where applicable.

### 5. Validate

- Map the candidate to relevant contracts and conceptual acceptance cases.
- Use a bounded spike or prototype only when documentary evidence cannot establish feasibility.
- Keep spikes disposable, outside production adoption, and free of architecture changes.
- Capture offline, failure, privacy, security, and compatibility evidence.

### 6. Decide

The allowed outcomes are:

- adopt for a declared scope
- adopt conditionally after named evidence or controls
- retain for later evaluation
- reject
- build internally
- combine internal Workspace-owned logic with an external commodity component
- no viable option; return to architecture or requirements review

No decision may silently change capability ownership, contract meaning, permission boundaries, or Local First expectations.

### 7. Record and Review

- Complete the canonical Research Catalogue entry.
- Add approved open source to the Open Source Registry only after adoption.
- Create or update an ADR when the result changes a durable architectural constraint.
- Append the Engineering Ledger when the result authorizes or changes engineering work.
- Set a review trigger and review date.

---

## Canonical Research Template

Future sessions instantiate this template for every research unit. “Not applicable” requires a reason; blank sections are not accepted.

### A. Identity and Scope

- Research ID:
- Capability:
- Technology category or question:
- Research status:
- Research owner:
- Date opened:
- Last reviewed:
- Catalogue entry:
- Related contracts:
- Required acceptance cases:
- Relevant ADRs:
- In scope:
- Out of scope:

### B. Capability Objective

State the user and architectural outcome the capability must provide. Cite its primary responsibility and explicitly list responsibilities that must remain with other capabilities.

### C. Research Questions

List answerable questions that distinguish viable approaches. Include:

- Can the candidate satisfy the capability objective without changing ownership?
- Which required contracts and failure semantics are directly supported?
- What adaptation is required?
- Which assumptions need measured evidence?
- What would make the candidate unacceptable?
- Which parts are commodity integration and which are unique Workspace value?

### D. Required Functional Capabilities

Enumerate observable functions derived from Capability Architecture and Capability Contracts. For each function record:

- requirement identifier
- architectural source
- mandatory or optional classification
- candidate support: native, configurable, adapter-required, unsupported, or unknown
- evidence

### E. Required Non-Functional Capabilities

Define measurable or demonstrable expectations for:

- reliability and recovery
- availability and degraded behavior
- resource use
- responsiveness classes
- deterministic or bounded behavior where required
- accessibility and operability
- observability without content leakage
- testability
- portability and replaceability
- data durability or ephemerality as owned by the capability

### F. Local First Requirements

Record:

- operation with the network unavailable
- local prerequisites
- behavior when optional online services fail
- proof that no silent cloud fallback occurs
- local storage, authorization, audit, and explanation paths
- installation, update, and recovery implications for offline use
- which functionality is core, optional, degraded, or unavailable

### G. Privacy Requirements

Record:

- data classes received, produced, retained, and transmitted
- purpose limitation and minimization controls
- retention owner and deletion/redaction path
- whether data can leave the device and under which explicit permission
- logging, telemetry, crash-reporting, and training-data defaults
- raw sensing, prompt, memory, target, extension, and secret handling
- ability to disable non-essential collection
- evidence that the candidate cannot become an alternate Memory store

### H. Security Considerations

Evaluate:

- trust boundary and threat model
- authorization integration and point-of-use enforcement
- proof, credential, and secret handling
- privilege level and OS attack surface
- sandboxing or isolation where relevant
- dependency and supply-chain exposure
- vulnerability disclosure and security-update process
- secure defaults and hardening controls
- data-at-rest and data-in-transit protection where applicable
- behavior under revocation, replay, mismatch, tampering, and partial failure

Security claims require primary documentation, tests, advisories, or independently reproducible evidence.

### I. Performance Considerations

Define representative workloads and record:

- startup and readiness cost
- latency by responsiveness class
- throughput or concurrency needs
- CPU, memory, storage, GPU, battery, and background impact where relevant
- worst-case and degraded behavior
- cancellation and shutdown responsiveness
- performance under offline operation
- measurement environment and reproducible method

### J. Explainability Considerations

Verify that the candidate can expose or allow Workspace to derive:

- what is happening
- why it is happening
- responsible capability
- permission scope and authorization identity where applicable
- target summary
- local or optional-remote boundary
- current status, uncertainty, partial effects, and available user action

Opaque internal behavior is acceptable only when Workspace can still produce truthful contract-level explanations without leaking protected content.

### K. Licensing Evaluation

Record:

- exact project and component
- version/revision evaluated
- license identifier and license text source
- dependency license implications
- distribution, modification, attribution, notice, source-disclosure, patent, trademark, and model/data license obligations
- commercial-use compatibility
- unresolved legal questions
- review authority and date

License uncertainty blocks adoption until resolved.

### L. Maintenance Evaluation

Record evidence for:

- release cadence and recency
- supported versions and platform policy
- issue and security response
- maintainer concentration and continuity risk
- upgrade and migration quality
- documentation and test quality
- dependency freshness
- long-term replacement feasibility

### M. Community Maturity Evaluation

Assess:

- project age and production use evidence
- contributor and maintainer diversity
- governance transparency
- quality of issue/discussion handling
- ecosystem interoperability
- availability of operational knowledge
- evidence of sustained use rather than popularity alone

Popularity metrics are context, not adoption evidence.

### N. Platform Compatibility

Record:

- Windows support as mandatory for the product target
- compatibility with the established desktop/runtime boundary
- required architectures and hardware classes
- OS API and permission requirements
- packaging, signing, update, and installer implications
- local database, process, UI, and Rust/TypeScript integration constraints where applicable
- accessibility and locale implications
- unsupported or degraded environments

Claims must identify the exact tested version and environment.

### O. Integration Complexity

Estimate and evidence:

- adapter surface and contract mapping
- additional processes, runtimes, services, or build tools
- dependency count and transitive risk
- lifecycle, failure isolation, and shutdown integration
- permission, audit, tracing, and explanation integration
- test harness and operational burden
- upgrade burden
- effect on the Complexity Budget

Complexity is assessed by boundaries introduced, not only lines of glue code.

### P. Extensibility

Determine whether the candidate:

- supports required future variants without owning new responsibilities
- exposes stable, versionable interfaces
- permits provider or implementation replacement
- can add capability-specific behavior through adapters
- avoids proprietary or data-format lock-in
- preserves contract compatibility and migration paths

Extensibility must not become speculative subsystem growth.

### Q. Failure Modes

For each credible failure record:

- trigger
- affected contract or user outcome
- owner-authoritative state
- fail-open or fail-closed behavior
- partial-effect and outcome-unknown risk
- cancellation/recovery path
- offline/degraded behavior
- explanation shown
- detection and reproducible validation

Include crash, corruption, unavailable local prerequisite, optional online outage, timeout, duplicate/out-of-order/missing event, incompatibility, revocation race, resource exhaustion, and update failure where applicable.

### R. Migration Risk

Record:

- data, schema, configuration, contract, and API migration needs
- backward/forward compatibility
- rollback feasibility
- bounded coexistence requirements
- export and replacement strategy
- risk of semantic changes to authority, privacy, outcomes, or ownership
- cost of moving away from the candidate

Parsing compatibility is insufficient when semantics differ.

### S. Reasons to Build Internally

Use evidence to assess whether internal construction is justified because:

- the behavior is unique Workspace value
- trust, permission, privacy, or explainability semantics cannot be safely adapted
- external options introduce greater lifecycle or migration risk
- required behavior is small and bounded
- ownership and long-term maintenance are affordable

“More control,” preference, or unfamiliarity with candidates is insufficient by itself.

### T. Reasons to Integrate Externally

Use evidence to assess whether integration is justified because:

- the function is commodity and mature
- external implementation reduces security, correctness, platform, or maintenance risk
- adapters can preserve Workspace contracts and ownership
- replacement remains feasible
- license and governance are acceptable
- integration complexity is lower than responsible internal maintenance

Integration never delegates Workspace's architectural authority to the technology.

### U. Candidate Comparison

For every candidate, including credible internal construction, record:

- requirement-by-requirement result
- evidence confidence
- blocking gaps
- required mitigations/adapters
- lifecycle cost
- material trade-offs
- disposition

Use qualitative ratings only when definitions are declared. A weighted score may summarize but cannot override a failed mandatory gate.

### V. Required Acceptance Criteria

A candidate is eligible for adoption only when:

- all mandatory functional and non-functional requirements are satisfied or have approved bounded mitigations
- all applicable Contract Specification invariants and acceptance cases have mapped evidence
- capability ownership and Interaction Matrix paths remain unchanged
- protected operations remain fail-closed and point-of-use authorized
- Local First core scenarios pass with network unavailable
- privacy minimization and retention ownership are demonstrated
- explanations preserve required architectural meaning
- Windows compatibility is verified
- license obligations are understood and acceptable
- security and maintenance risks have owners and controls
- migration and replacement paths are documented
- integration complexity fits the Complexity Budget
- build-versus-integrate reasoning explicitly satisfies ADR-0002
- unknowns are either resolved or accepted by a named decision authority
- the Research Catalogue entry is complete and reviewable

### W. Decision and Review

- Decision:
- Selected scope:
- Decision rationale:
- Rejected candidates and reasons:
- Conditions or controls:
- Remaining unknowns:
- Required ADR:
- Open Source Registry action:
- Engineering Ledger action:
- Review date:
- Event-driven re-evaluation triggers:
- Approver:

---

## Capability Research Profiles

The canonical template is used unchanged for each capability. The profiles below supply capability-specific objectives, research questions, required capabilities, risks, and acceptance emphasis; they do not create separate research processes.

### Runtime Host

**Capability objective**

Start, wire, monitor, degrade, and shut down local capabilities predictably without acquiring domain authority.

**Research questions**

- What isolation boundary provides sufficient crash containment without unnecessary subsystem cost?
- How are ordered lifecycle, availability, and domain-free health represented?
- How does configuration migrate and recover locally?
- Can engineering documentation be reliably excluded from runtime packaging?

**Required functional capabilities**

- ordered startup, pause, and shutdown
- capability registration and availability query
- aggregate domain-free health
- offline/online-optional mode signaling
- safe failure isolation and explicit degradation

**Required non-functional emphasis**

- deterministic lifecycle ordering
- low startup/background overhead
- local availability without internet
- recoverable local configuration
- process and packaging compatibility on Windows

**Capability-specific security, privacy, and failure focus**

- lifecycle authority must not expose domain data or product permission
- health and logs remain domain-free
- host failure stops Workspace; no silent background autonomy
- shutdown timeout and capability crash remain explainable

**Acceptance emphasis**

- offline architecture scenarios 1 and 8
- IC-001 through IC-005
- lifecycle, timeout, health-loss, and shutdown acceptance evidence

### Permission Authority

**Capability objective**

Remain the sole local, explicit, revocable, explainable source of permission truth and bound effect/control authority.

**Research questions**

- Can exact purpose, subject, requester, target, operation, scope, extension, expiry, replay, and consumption bindings be enforced?
- Can revocation be ordered against each meaningful effect?
- Can effect proofs and independently governed operation-control proofs remain opaque and locally verifiable as specified?
- Can audit be content-free, bounded, and explainable without becoming Memory?
- What secure local storage and time assumptions are required?

**Required functional capabilities**

- authorize, deny, challenge, grant, revoke, validate, catalogue, and explain
- point-of-use use/consumption
- replay and mismatch rejection
- challenge redemption
- bounded offline safety-control lease
- minimized permission audit

**Required non-functional emphasis**

- fail-closed consistency
- tamper resistance
- low interactive validation latency
- durable local authority without network
- auditable but privacy-minimized behavior

**Capability-specific security, privacy, and failure focus**

- proof and secret non-disclosure
- secure persistence and key lifecycle
- clock rollback, corruption, replay, outage, and revocation-race behavior
- no self-authorization or domain execution
- safe local status/cancellation control during temporary Authority outage

**Acceptance emphasis**

- required permission cases 5–11 and 40–42
- IC-006 through IC-012
- exact point-of-use, multi-effect revocation, challenge, offline-control, and audit evidence

### Workspace Management

**Capability objective**

Own durable local workspaces, zones, membership, and active scope without becoming Memory or orchestration.

**Research questions**

- Which local data model supports hierarchy, scope validation, archive, and evolution?
- How are atomic mutations, conflicts, and owner-authoritative outcomes represented?
- How are immutable scope snapshots exposed without shared mutable state?
- What import/export path preserves replacement feasibility?

**Required functional capabilities**

- create, read, change, select, organize, and archive workspaces/zones
- active-scope and scope-validation queries
- protected read/write paths
- scope-change and terminal-result events
- operation status and terminal tombstone lookup

**Required non-functional emphasis**

- durable local consistency
- fast interactive reads
- atomic or honestly partial mutations
- schema migration and backup/recovery
- offline operation

**Capability-specific security, privacy, and failure focus**

- workspace metadata minimization and access separation
- stale scope, hierarchy corruption, concurrent mutation, and archived-target handling
- no memory, action, or companion semantics

**Acceptance emphasis**

- offline scenario 3
- IC-013 through IC-018
- protected reads, mutation deduplication, scope invalidation, conflict, migration, and recovery evidence

### Memory

**Capability objective**

Solely retain, retrieve, explain, redact, and forget durable user knowledge with provenance, scope, and policy.

**Research questions**

- How are retrieval quality, provenance, retention, and deletion demonstrated independently?
- Can all durable user knowledge remain locally owned and exportable?
- Can exact forgetting/redaction be verified across indexes, caches, backups, and derived data?
- How can multiple retrieval methods remain replaceable behind Memory contracts?

**Required functional capabilities**

- permissioned propose-write, retrieve, explain, redact, and forget
- provenance and workspace scope
- retention policy
- content and index deletion
- operation recovery and content-free terminal history

**Required non-functional emphasis**

- local durability and recoverability
- retrieval quality with reproducible evaluation
- bounded latency and storage growth
- deterministic deletion semantics
- replaceable indexes and data portability

**Capability-specific security, privacy, and failure focus**

- encryption/access controls for sensitive local knowledge
- no raw continuous capture archive
- no alternate retention in logs, caches, inference, or orchestration
- corruption, index drift, stale retrieval, incomplete deletion, and backup leakage

**Acceptance emphasis**

- offline scenario 4
- IC-019 through IC-022
- privacy cases 25–28 as applicable
- provenance, scope, retrieval, redaction/forgetting, recovery, and migration evidence

### Context Sensing

**Capability objective**

Observe local computing context read-only under permission while keeping raw observations ephemeral and emitting only purpose-limited minimized data.

**Research questions**

- Which minimum local signals provide value without deep observation?
- Can read APIs be isolated from all mutation APIs?
- Where do raw capture, minimization, and redaction occur?
- How quickly and reliably can sensors stop, clear buffers, and react to revocation?
- Which OS permissions and accessibility implications apply?

**Required functional capabilities**

- separately permissioned basic and deep sensing
- start, pause, stop, resume, query, explain, and subscribe
- continuous authorization enforcement
- purpose-based minimization
- ephemeral buffer clearing
- local sensor operation

**Required non-functional emphasis**

- low CPU, memory, battery, and storage impact
- bounded observation latency/rate
- immediate safety-reducing stop
- robust Windows API compatibility
- accessibility-aware behavior

**Capability-specific security, privacy, and failure focus**

- raw observations never cross the capability boundary
- deep permission never waives minimization
- no direct Memory or Action path
- capture leakage, minimization failure, stuck sensor, revoked session, and unavailable sensor behavior

**Acceptance emphasis**

- offline scenario 5
- IC-023 through IC-025
- permission cases for start/resume and safety stop
- privacy cases 25–27
- read-only boundary and buffer-clearing evidence

### Action

**Capability objective**

Solely execute declared, permitted environment mutations with exact authorization, accountable outcomes, and safe cancellation where feasible.

**Research questions**

- Which action classes and irreversible commit points can be made explicit?
- Can exact target/effect context be validated immediately before each meaningful effect?
- How are partial, indeterminate, timeout, cancellation, compensation, and recovery represented?
- Can sensing and mutation APIs remain structurally separated?
- Which OS privileges create unacceptable attack surface?

**Required functional capabilities**

- enumerable action catalogue and descriptions
- exact point-of-use execution
- operation identity, status, progress, terminal outcome, and tombstone lookup
- cancellation classes and supported safety control
- metadata-only execution audit and minimized effect summaries

**Required non-functional emphasis**

- predictable execution and bounded privileges
- low-latency cancellation/status
- deduplication and owner-authoritative recovery
- failure isolation by action class
- local operation without internet

**Capability-specific security, privacy, and failure focus**

- no Permission Authority bypass or omnibus grant
- no copied user content in audit
- target substitution, privilege escalation, replay, revocation race, partial effect, unsafe cancellation, and unknown outcome
- compensation is a new authorized action, never hidden rollback

**Acceptance emphasis**

- offline scenario 5
- IC-029 through IC-031
- command/outcome cases 12–20
- per-action-class commit, cancellation, partial-effect, idempotency, and recovery evidence

### Intelligence

**Capability objective**

Provide local-first reasoning and generation with uncertainty and non-executing proposals, while optional remote use remains explicit and permissioned.

**Research questions**

- Which local viability classes meet quality, latency, memory, hardware, and power constraints?
- Can structured outputs and uncertainty be validated without granting execution authority?
- Can provider adapters prevent prompt/response persistence and silent remote fallback?
- How are cancellation, resource limits, and provider failure exposed?
- Can providers be replaced without changing contract semantics?

**Required functional capabilities**

- local reasoning and generation
- optional separately authorized remote provider path
- structured results/proposals
- provider availability and metadata-only explanation
- resource limits, cancellation, operation status, and recovery

**Required non-functional emphasis**

- measured quality and reliability
- bounded local CPU/GPU/memory/storage use
- interactive/background responsiveness classes
- deterministic offline selection
- provider portability

**Capability-specific security, privacy, and failure focus**

- prompts, responses, and supplied Memory content do not persist outside authorized Memory
- remote data sharing is explicit before transfer
- proposals never become Action commands
- model file integrity, prompt injection boundaries, malformed output, resource exhaustion, timeout, and provider outage

**Acceptance emphasis**

- offline scenarios 6 and 7
- IC-005 and IC-026 through IC-028
- Local First cases 29–31
- Intelligence boundary case 32
- no-persistence, structured-output, uncertainty, and resource evidence

### Companion Orchestration

**Capability objective**

Own intent interpretation and predictable multi-capability task sequencing without absorbing domain responsibilities or ambient authority.

**Research questions**

- What is the smallest orchestration/state mechanism that supports interruption, waiting, recovery, and explanation?
- Can plans remain bounded and task content remain in-flight only?
- How are Intelligence proposals treated as untrusted input?
- Can every downstream call preserve purpose, correlation, extension identity, and permission context?
- Which behavior is unique Workspace value and therefore unsuitable for wholesale delegation?

**Required functional capabilities**

- intent-to-task conversion
- explicit task state and status recovery
- capability sequencing through public contracts
- permission challenge/wait/resume/stop behavior
- cancellation and presentation-neutral progress/explanations
- deterministic degraded paths

**Required non-functional emphasis**

- predictability and inspectable state
- bounded plan depth and resource use
- resilience to duplicate, missing, late, and incompatible results
- low cognitive-load interaction
- testable deterministic policies

**Capability-specific security, privacy, and failure focus**

- no ambient superuser authority
- no durable content-bearing task history
- no forbidden direct path or shadow ownership
- dependency failure, stale result, proposal injection, task conflict, user-response expiry, and cancellation race

**Acceptance emphasis**

- offline scenario 7
- IC-006, IC-014, IC-019–035 as initiating or receiving paths
- boundary cases 32–34
- task recovery, identity propagation, permission sequencing, and explanation evidence

### Experience

**Capability objective**

Provide a calm, accessible, honest local interface that captures user intent and presents authoritative status, permissions, and explanations without owning domain behavior.

**Research questions**

- Can all required states and distinctions be presented accessibly and with low cognitive load?
- How do optional modalities fail back without affecting core UI?
- Can presentation preserve domain meaning and responsible capability identity?
- What Windows packaging, accessibility, notification, microphone, and overlay constraints apply?
- Can UI state remain local and content buffers ephemeral?

**Required functional capabilities**

- intent, interaction-result, cancellation, and permission-decision capture
- workspace read-only navigation
- status, progress, challenge, explanation, uncertainty, and terminal-state presentation
- local settings and accessibility
- optional voice and overlay modalities with explicit permissions

**Required non-functional emphasis**

- responsive, accessible, predictable interaction
- offline core usability
- graceful modality degradation
- minimal background/resource impact
- stable Windows desktop behavior

**Capability-specific security, privacy, and failure focus**

- no domain mutation, orchestration, grant issuance, or fabricated success
- ephemeral input handling
- safe rendering of untrusted model/extension text
- microphone/overlay permission, notification leakage, stale status, and surface failure

**Acceptance emphasis**

- offline scenario 8
- IC-003, IC-004, IC-007, IC-008, IC-010, IC-013, and IC-032 through IC-034
- all required explainability states
- Experience boundary case 33
- accessibility and fallback evidence

### Extension Host

**Capability objective**

If separately activated, isolate untrusted extensions, attenuate identity and scopes, and route contributions through Companion without creating direct domain authority.

**Research questions**

- Is activation justified by demonstrated user value and the Complexity Budget?
- What sandbox strength, signing, trust, update, and revocation model is required?
- Can extension identity survive every mediated interaction?
- Can extension partitions prevent durable user knowledge, observation, task, prompt, and response retention?
- Can crashes and revocation unload extensions without affecting core operation?

**Required functional capabilities**

- manifest validation
- install, enable, invoke, disable, and unload lifecycle
- sandbox/privilege attenuation
- identity- and scope-bound contribution events
- isolated disposable storage
- status, explanation, recovery, and audit

**Required non-functional emphasis**

- strong isolation and bounded resource use
- independent failure containment
- deterministic unload/revoke
- secure update and compatibility policy
- zero-extension core operation

**Capability-specific security, privacy, and failure focus**

- extensions are untrusted
- no direct domain-capability path
- no inherited Companion/user-session authority
- supply-chain compromise, sandbox escape, confused deputy, identity loss, partition leakage, malicious update, and crash loop

**Acceptance emphasis**

- no implementation research may be treated as activation
- IC-035 through IC-038 only after activation
- extension boundary case 34
- permission identity mismatch, isolation, revocation/unload, privacy, and core-independence evidence

---

## Required Evidence Before Adoption

Adoption requires an evidence package proportionate to risk and containing:

1. **Primary-source evidence**
   - official documentation
   - exact version/revision
   - license text
   - support/platform policy
   - security and release policy

2. **Architecture mapping**
   - owned responsibility preserved
   - allowed initiators, receivers, and subscribers
   - contract and schema mapping
   - permission, privacy, Local First, and explainability mapping

3. **Reproducible validation**
   - environment and procedure
   - representative inputs/workloads
   - expected and observed results
   - offline and failure scenarios
   - applicable conceptual acceptance cases

4. **Operational evidence**
   - lifecycle and shutdown behavior
   - resource profile
   - recovery and migration
   - update and replacement strategy

5. **Risk evidence**
   - security advisories and threat assessment
   - privacy/data-flow assessment
   - maintenance and governance assessment
   - known gaps with owners and controls

6. **Decision evidence**
   - comparable alternatives
   - internal-build comparison
   - rejection reasons
   - selected scope and review triggers

Claims without a source or reproducible observation are recorded as unknown, not fact. Mandatory unknowns block adoption.

---

## Required Comparison Criteria

Every candidate is compared using the same declared criteria:

- functional fit
- contract and acceptance fit
- Local First fit
- privacy fit
- security fit
- permission and revocation fit
- explainability fit
- performance and resource fit
- Windows/platform fit
- licensing
- maintenance
- community maturity and governance
- integration complexity
- operational complexity
- extensibility and replaceability
- failure and recovery behavior
- migration and lock-in risk
- total lifecycle ownership cost
- unique-value versus commodity alignment
- Complexity Budget impact

Mandatory gates are evaluated before preferences. Weighting, if used, is declared before scoring and never converts a failed mandatory gate into a pass.

---

## Documentation Standards

Every research record must:

- use the canonical template
- use stable requirement, candidate, evidence, risk, and decision identifiers
- cite architecture sources by file and section
- cite primary external evidence with source, version, access date, and relevant claim
- label facts, measurements, inferences, and unknowns
- make test procedures reproducible
- record the exact environment for measured results
- separate candidate claims from Workspace validation
- preserve rejected options and reasons
- avoid copying protected user content into evidence
- include a concise decision summary and unresolved risks
- include review date and event-driven triggers

Documentation is an engineering artifact and must not become runtime configuration or logic.

---

## Decision Recording

The canonical Research Catalogue entry records:

- capability and research-unit scope
- status
- candidates
- decision
- reason
- integration notes
- license
- security review
- maintenance assessment
- evidence links
- rejected options
- unresolved risks
- review date and triggers

After a decision:

- append the Engineering Ledger when work or architecture is affected
- add an adopted open-source dependency to the Open Source Registry
- create an ADR only when the decision establishes or changes a durable architectural rule
- update Current State when milestone, active task, next task, known unknowns, or architecture health changes

No chat transcript is a decision authority.

---

## Rejected Technology Documentation

Rejected candidates remain in the same canonical research record and include:

- candidate and exact version/revision
- date evaluated
- evidence considered
- failed mandatory gates
- material trade-offs
- whether rejection is permanent, current-scope-only, or evidence-dependent
- conditions that could justify re-evaluation
- replacement or selected direction, if any

Rejection language must distinguish “does not meet requirements” from “not enough evidence.”

---

## Future Re-evaluation

Re-evaluation occurs on the scheduled review date or when a trigger occurs:

- material security advisory or supply-chain incident
- license, ownership, governance, or funding change
- end-of-life or platform-support change
- major version or breaking contract change
- new Windows/runtime incompatibility
- unacceptable performance or reliability evidence
- changed Workspace contract or capability requirement
- migration difficulty or maintainer concentration exceeding accepted risk
- a previously blocking unknown becoming testable
- a materially improved alternative

Re-evaluation:

1. reopens the existing Research Catalogue entry
2. preserves the prior evidence and decision
3. adds dated new evidence
4. reruns affected mandatory gates and acceptance cases
5. records whether the decision remains, changes, or requires migration
6. updates dependent Registry, ADR, Ledger, and Current State records as needed

---

## Architectural Risks Identified

The framework exposes these research-stage risks:

1. Technology convenience may pressure capability ownership or forbidden communication paths.
2. Broad framework defaults may weaken purpose binding, point-of-use authorization, or fail-closed behavior.
3. Logging, telemetry, caches, indexes, and provider histories may become alternate Memory stores.
4. “Offline capable” claims may still depend on network installation, activation, authorization, models, or recovery.
5. Windows support claims may omit packaging, accessibility, OS permission, signing, or long-running lifecycle behavior.
6. Aggregate scores may hide mandatory privacy, security, contract, or license failures.
7. Prototype success may omit event loss, duplicate effects, revocation races, partial effects, and migration.
8. Internal construction may be justified by preference rather than unique Workspace value.
9. External integration may import hidden runtimes, services, privileges, or lifecycle ownership.
10. Community popularity may be mistaken for governance, security response, or maintenance continuity.
11. A selected data format or engine may create lock-in that undermines forgetting, export, or replacement.
12. Extension Host feasibility research may be mistaken for authorization to activate it.
13. Research evidence may be duplicated across Catalogue, Ledger, ADRs, and Registry and then drift.
14. Contract acceptance may be claimed from successful parsing or happy paths while semantic authority and failure behavior remain unproved.

---

## Recommended Research Order

The first capability to research should be **Permission Authority**.

Reason:

- it is the sole trust authority used by every protected capability
- point-of-use validation, revocation ordering, challenge redemption, replay resistance, and operation-control semantics constrain all later integrations
- a weak technology fit here would invalidate otherwise viable capability choices
- it carries high security and privacy risk while remaining fully local-first
- researching it first does not select or implement a technology

Suggested subsequent order:

1. Runtime Host
2. Workspace Management
3. Memory
4. Context Sensing
5. Action
6. Intelligence
7. Companion Orchestration
8. Experience
9. Extension Host only after separate activation justification

This order is a planning recommendation, not an implementation dependency or architecture decision.

---

## Internal Consistency Review

### ADR-0002

- Every common capability is researched before implementation.
- Internal construction is a candidate that requires evidence of unique Workspace value.
- External integration is evaluated for lifecycle cost, risk, and replaceability rather than presumed.
- Rejected and adopted candidates share one canonical comparison record.

Result: consistent with Integrate Before Reinvent.

### Blueprint

- Local First, Human First, Privacy First, Permission Before Automation, and Explain Every Action are mandatory gates.
- Build Only Where We Create Value and Integrate Before Reinventing are explicit build-versus-integrate criteria.
- Complexity Must Justify Itself is evaluated through boundary and lifecycle cost.
- Documentation remains outside runtime.
- Capability ownership remains single and contract-based.
- Cognitive load is included in Experience and explanation evaluation.

Result: consistent with the Blueprint.

### Contract Specification

- Research maps candidates to contract identities, semantics, invariants, metadata, failure behavior, and applicable acceptance cases.
- Successful parsing or happy-path operation is not treated as semantic compatibility.
- Local First, privacy, permission, cancellation, retry, idempotency, recovery, audit, traceability, explainability, and versioning are required evidence dimensions.
- Research cannot redefine missing architecture through technology adoption.

Result: consistent with the Contract Schema and Acceptance Specification.

### Duplicate Process Review

- This document owns the research method and template.
- The Research Catalogue owns each investigation record.
- The Open Source Registry owns approved dependency inventory.
- ADRs own durable architectural decisions.
- The Engineering Ledger owns append-only engineering consequences.
- Current State owns present milestone and next work.

No second discovery, evaluation, decision, or re-evaluation process is introduced.

### Validation Result

- All ten capabilities have a research profile.
- Every required research dimension appears once in the canonical template.
- Governance defines evidence, comparison, documentation, decision, rejection, and re-evaluation.
- No technology research, library recommendation, vendor selection, or code implementation is present.
- No ownership, authority, interaction, or contract semantics are changed.
- No unresolved internal inconsistency was found.
