# Workspace Architecture Guardian v1.0

Status: Proposed; pending architecture review

Authority: Proposed mandatory final architectural completion gate for Workspace
engineering tasks; subordinate to the Blueprint, accepted ADRs, and
authoritative architecture; becomes active only after architecture acceptance

Version: 1.0

---

## 1. Purpose

The Architecture Guardian prevents architectural drift by auditing completed
engineering work before that work may be declared complete.

The Guardian:

- audits only
- never engineers
- never changes code
- never changes architecture
- never edits documentation
- never selects or researches technology
- never redesigns the project
- never introduces a capability
- never proposes implementation
- never treats its own output as authority

The Guardian determines only whether the reviewed work conforms to existing
authority. A failure identifies the minimum correction required to restore
conformance or refers an unresolved conflict to the existing architecture
governance process.

---

## 2. Applicability

Upon architecture acceptance of this document, Guardian review is mandatory for
every task that changes repository state or binds an engineering decision,
including:

- architecture and governance documentation
- capability research and technology decisions
- implementation and refactoring
- contracts, schemas, migrations, packaging, configuration, and dependencies
- tests, build tooling, automation, and release work
- documentation that changes engineering meaning, readiness, or status

Pure read-only explanation, status, or discovery requests that bind no decision
and change no repository state do not require Guardian review.

The Guardian reviews the complete intended task result, including required
Ledger and Current State updates. It runs immediately before the task is
declared complete. Within the Cursor Protocol, Guardian PASS is a precondition
to step 11, `Stop`; it does not replace or reorder the Protocol's earlier
planning, implementation, validation, or documentation steps.

---

## 3. Authority Order

The Guardian evaluates conflicts in this order:

1. `00_Workspace_Blueprint.md`
2. accepted ADR bodies in `architecture/decisions/`
3. `08_Workspace_Capability_Architecture.md`
4. `09_Capability_Interaction_Matrix.md`
5. `10_Capability_Contracts.md`
6. `11_Contract_Schema_and_Acceptance_Specification.md`
7. `12_User_Journey_Architecture_Validation.md`
8. `12_Capability_Technology_Research_Framework.md` for research and selection
   governance
9. `13_Capability_Research_Roadmap.md` for research sequencing and gates
10. `04_Cursor_Protocol.md`
11. `01_Current_State.md`
12. `02_Engineering_Ledger.md`, `03_Research_Catalogue.md`,
    `06_Open_Source_Registry.md`, and relevant
    research records as historical, decision, and evidence records
13. the reviewed task objective, plan, change set, and validation evidence

`05_Architecture_Decision_Records.md` is an index. Accepted ADR bodies, not the
index summaries, are decision authority.

When two authorities at the same level conflict, the Guardian does not choose
between them. It returns FAIL and refers the conflict to architecture review.

---

## 4. Frozen Review Package

Before evaluation, the Guardian freezes one review package containing:

- review identity and timestamp
- task identity, exact objective, and task type
- declared scope and explicit exclusions
- baseline branch and HEAD
- complete files added, modified, removed, or renamed
- affected capabilities, contracts, user journeys, and runtime/package surfaces
- applicable authority document versions
- relevant accepted ADRs
- known gaps and assumptions the task touches or depends on
- task validation evidence
- final intended Ledger, Current State, Catalogue, registry, and decision
  updates
- repository status and any pre-existing unrelated changes

The Guardian reviews only this frozen package. Any material change to objective,
scope, assumptions, authorities, or change set invalidates the decision and
requires a complete rerun.

Missing evidence is a failure. It is Major by default and Critical when the
missing evidence could conceal a Critical violation.

Pre-existing unrelated working-tree changes do not fail review merely because
they exist. They must be identified, excluded from the task package, and shown
not to affect the reviewed result.

### Closed applicability enumeration

Applicability is calculated; it is not chosen by reviewer discretion.

First, classify the task as one or more of these closed task types:

1. architecture or governance
2. capability research or technology decision
3. product or runtime implementation/refactoring
4. contract, schema, persistence, migration, or recovery
5. build, packaging, dependency, automation, test, or release
6. documentation, status, catalogue, registry, or decision recording

If the objective cannot be classified, the review package is incomplete and
fails Major.

The affected set is the union of:

- every file in the frozen change set
- every capability, contract ID, interaction ID, ADR ID, research ID, journey
  ID, permission scope, owned-state term, runtime/package surface, and authority
  document named by the objective, plan, changed lines, or validation evidence
- the authoritative definition of every identified item
- the owner, permitted initiator/receiver/subscriber, required intermediary, and
  user journey attached to every identified interaction
- every active known gap or unknown whose text names an identified item
- every document directly referenced by a changed active architecture or
  governance document

Exact canonical identifiers are used first. For unnumbered concepts, the
canonical capability and responsibility names in Capability Architecture are
used. An ambiguous identifier fails Naming Consistency and evidence
completeness; the Guardian does not guess.

Authority applicability is then fixed:

- Blueprint and all accepted ADRs are considered for every task.
- Capability Architecture, Interaction Matrix, Capability Contracts, Contract
  Schema, and User Journey Validation are inspected whenever the affected set
  contains a capability, contract, interaction, permission, owned state,
  lifecycle, recovery, runtime, or user-visible behavior.
- Research Framework, Roadmap, Research Catalogue, relevant research records,
  and Open Source Registry are inspected for task type 2 or any task relying on
  a research/selection/dependency claim.
- Cursor Protocol, Current State, Engineering Ledger, Documentation
  Consistency, Repository Integrity, and Naming Consistency are inspected for
  every repository-changing or decision-binding task.

Evidence is sufficient only when each pass claim cites the governing repository
section and either the changed artifact that conforms or a validation result
that directly tests the criterion. General assertions, unstated assumptions,
absence of observed failure, or reviewer confidence are not evidence.

---

## 5. Deterministic Evaluation Procedure

The Guardian executes these steps in order:

1. Freeze and identify the review package.
2. Verify every required authority is present and readable.
3. Resolve applicable authority versions and accepted ADR status.
4. Identify affected capabilities, interactions, contracts, data, journeys,
   runtime surfaces, and known gaps.
5. Evaluate review categories 1 through 18 in order.
6. Record PASS or FAIL for every category. There is no `N/A`.
7. For a category not triggered by the task, record PASS only after evidence
   shows why the task cannot affect that category.
8. Determine the highest failure severity.
9. Produce exactly one final decision: PASS or FAIL.

Existing known gaps fail the task only when the task:

- touches or depends on the gap
- worsens the gap
- routes around the gap
- claims the gap is resolved
- claims readiness that the gap still blocks

An unrelated known gap is recorded as considered and does not freeze unrelated
work.

---

## 6. Severity and Stop Rules

### Critical

A Critical failure violates the Blueprint, an accepted ADR, sole authority,
permission or privacy boundaries, runtime/repository authority, or could permit
uncontrolled effects, disclosure, or fabricated outcomes.

Engineering stop: Immediate. Stop the task and dependent work. Only rollback,
containment, evidence preservation, or the minimum correction may proceed.

### Major

A Major failure violates capability ownership, a permitted dependency,
contract/schema meaning, lifecycle/recovery semantics, a relevant journey gate,
or required architectural evidence.

Engineering stop: Yes. Stop the task. Only the minimum correction and required
re-validation may proceed.

### Minor

A Minor failure is non-behavioural but prevents deterministic architectural
acceptance, such as stale naming, traceability, status, or documentation that
does not alter runtime meaning.

Engineering stop: Stop completion, not unrelated engineering. Only the minimum
documentation or naming correction and re-review may proceed.

A passing category has severity `None`. Any unresolved Critical, Major, or Minor
failure makes the final decision FAIL.

---

## 7. Review Categories

### 1. Blueprint Compliance

- **What is inspected:** Mission, vision, all Product Principles, Companion
  Principles, capability composition, contract communication, and prohibition
  on duplicate responsibility.
- **Pass criteria:** Every changed decision, behavior, dependency, data flow,
  package, and claim is consistent with every applicable Blueprint principle;
  no principle is weakened, silently reinterpreted, or traded away.
- **Failure criteria:** Any contradiction with the Blueprint; required internet
  for core behavior; hidden or unexplained effects; privacy or human control
  weakened; duplicated capability responsibility; engineering documentation
  placed in runtime.
- **Severity:** Critical.
- **Required remediation:** Remove the contradiction or refer the task to a
  separate architecture review when the objective cannot conform. Name only the
  violated Blueprint clause and required conforming outcome.
- **Whether engineering must stop:** Yes, immediately.

### 2. ADR Compliance

- **What is inspected:** ADR index parity, accepted ADR bodies, status,
  decision, constraints, consequences, and task evidence for every applicable
  ADR.
- **Pass criteria:** Every accepted ADR is considered, every triggered ADR is
  identified and satisfied, and every untriggered ADR has evidence that its
  decision scope and consequences do not intersect the calculated affected set;
  no superseded or proposed text is treated as accepted authority; the index is
  used only for navigation.
- **Failure criteria:** Conflict with an accepted decision; omitted applicable
  ADR; unrecorded reversal; consequence bypass; stale index/body mismatch that
  makes accepted authority ambiguous.
- **Severity:** Critical for decision conflict or bypass; Major for incomplete
  applicability evidence; Minor for a non-semantic index reference error.
- **Required remediation:** Align the task with the accepted ADR or stop for the
  existing ADR governance process. The Guardian does not draft or amend an ADR.
- **Whether engineering must stop:** Yes. Critical stops immediately; Major
  stops the task; Minor blocks completion.

### 3. Capability Ownership

- **What is inspected:** Capability responsibilities, owned state, authoritative
  decisions/results, persistence, lifecycle authority, and every state
  transition introduced or affected.
- **Pass criteria:** Exactly one capability owns each responsibility, mutable
  state, transition, and authoritative result; consumers use copies, immutable
  references, or contracts; no capability acquires shadow authority.
- **Failure criteria:** Shared mutable ownership; responsibility transfer;
  alternate domain store; non-owner result fabrication; infrastructure,
  presentation, intelligence, or orchestration acquiring domain authority.
- **Severity:** Critical for authority or protected-data takeover; Major for
  responsibility or persistence overlap.
- **Required remediation:** Return the responsibility, state, or result to the
  owner named by Capability Architecture, or refer an unavoidable ownership
  conflict to architecture review.
- **Whether engineering must stop:** Yes.

### 4. Contract Compliance

- **What is inspected:** Every cross-capability request, command, event,
  response, lifecycle message, contract identity/version, caller, receiver,
  subscriber, conceptual field, and acceptance obligation.
- **Pass criteria:** Every interaction has an admitted purpose, permitted path,
  correct message kind, compatible version, complete schema, owner-authoritative
  result, and required permission, failure, recovery, privacy, and explanation
  semantics.
- **Failure criteria:** Undocumented interaction; missing contract; wrong
  message kind; incompatible meaning; missing required field; event used as
  authority; acceptance treated as completion; implementation inventing absent
  architecture.
- **Severity:** Critical when authority, privacy, or effect semantics can be
  bypassed; Major otherwise.
- **Required remediation:** Remove the uncontracted interaction or complete the
  existing contract-acceptance governance before the task resumes. The Guardian
  does not design the contract.
- **Whether engineering must stop:** Yes.

### 5. Responsibility Duplication

- **What is inspected:** Duplicate workflows, policy, state, persistence,
  validation, lifecycle control, explanations, audit content, caches, logs,
  indexes, and fallback implementations.
- **Pass criteria:** Repeated mechanisms remain non-authoritative adapters or
  projections behind one owner; no duplicate responsibility or alternate source
  of truth exists.
- **Failure criteria:** Two owners for one responsibility; a cache/log/history
  becoming authoritative; duplicated permission, Memory, orchestration,
  lifecycle, sensing, or Action policy; fallback bypassing the owner.
- **Severity:** Critical when duplication creates authority, privacy, or
  execution bypass; Major otherwise.
- **Required remediation:** Remove the duplicate authority or reduce it to a
  replaceable non-authoritative mechanism under the existing owner.
- **Whether engineering must stop:** Yes.

### 6. Dependency Direction

- **What is inspected:** Direct calls, responses, events, lifecycle paths,
  required intermediaries, provider boundaries, extension paths, and hard/event
  dependency cycles.
- **Pass criteria:** Every initiator/receiver/purpose matches the Interaction
  Matrix; responses remain on the request path; events do not create reverse
  command dependencies; hard dependencies remain acyclic; required
  intermediaries are preserved.
- **Failure criteria:** Forbidden path; undocumented receiver/subscriber;
  intermediary bypass; event-command confusion; Intelligence-to-Action or
  extension-to-domain bypass; new hard cycle.
- **Severity:** Critical for trust, permission, extension, provider, or execution
  bypass; Major for any other forbidden direction or cycle.
- **Required remediation:** Remove the path or restore the exact permitted
  Matrix route and intermediary. Any required new path goes to separate
  architecture review.
- **Whether engineering must stop:** Yes.

### 7. Explainability Preservation

- **What is inspected:** Explanations for requested, waiting, active,
  challenge-required, denied, degraded, timed-out, partial, outcome-unknown,
  indeterminate, cancelled, failed, and completed states.
- **Pass criteria:** Every applicable state preserves what, why, responsible
  capability, requester, target summary, permission scope/authorization
  identity, uncertainty, local/remote boundary, available user action, and
  correlation without exposing protected content.
- **Failure criteria:** Hidden action/state; acceptance shown as completion;
  failure shown as success; owner meaning rewritten by Experience; missing
  uncertainty or responsible capability; proof, secret, or unrelated data in an
  explanation.
- **Severity:** Critical for fabricated success or concealed consequential
  effect; Major for missing or misleading required explanation; Minor for
  non-semantic trace/reference drift.
- **Required remediation:** Restore the existing explainability envelope and
  owner-authored status meaning; remove protected excess.
- **Whether engineering must stop:** Yes; Minor blocks completion.

### 8. Permission Boundary Preservation

- **What is inspected:** Default deny, authorization request context, proof
  binding, point-of-use validation/consumption, revocation ordering, replay,
  challenge redemption, operation-control authority, automatic policy, and
  caller identity.
- **Pass criteria:** Permission Authority remains sole grant/revoke/validation
  authority; every protected owner validates exact context at the use/commit
  point; proofs remain opaque and excluded from events/logs/explanations;
  control authority cannot create effects.
- **Failure criteria:** Ambient or implicit grant; cached decision used as
  authority; missing point-of-use validation; proof replay/context drift;
  revocation bypass; event/reference treated as proof; self-authorization;
  control proof creating or continuing effects.
- **Severity:** Critical.
- **Required remediation:** Remove the bypass and restore the exact existing
  authorization and validation path. Any missing authority model requires
  separate architecture review.
- **Whether engineering must stop:** Yes, immediately.

### 9. Local-First Compliance

- **What is inspected:** Core offline journeys, installation/runtime
  prerequisites, startup, authorization, Workspace, Memory, local sensing,
  local Action, local Intelligence when configured, recovery, repair, migration,
  and optional remote behavior.
- **Pass criteria:** Core local paths work without internet when local
  prerequisites exist; remote behavior is optional, explicit, separately
  authorized, and never a silent fallback; offline failure remains honest and
  bounded.
- **Failure criteria:** Internet, activation, hosted control plane, telemetry,
  remote authorization, or cloud recovery required for core operation; silent
  remote fallback; optional cloud failure disables unrelated local capability.
- **Severity:** Critical for required/silent remote transfer or authority;
  Major for loss of a required offline core path.
- **Required remediation:** Remove the remote prerequisite/fallback or keep the
  affected function explicitly optional and unavailable under existing
  authority.
- **Whether engineering must stop:** Yes.

### 10. Privacy Compliance

- **What is inspected:** Purpose limitation, minimization, raw sensing, prompts,
  responses, Memory, audit, logs, diagnostics, caches, indexes, temporary files,
  exports, backups, remote transfer, retention, deletion, and redaction
  ownership.
- **Pass criteria:** Only minimum purpose-bound data crosses each boundary;
  sensitive raw/content data remains with its owner; retention has an owner and
  policy; no alternate Memory exists; remote transfer is explicit and
  authorized.
- **Failure criteria:** Excess collection/disclosure; raw observation leakage;
  prompts or user knowledge in non-Memory persistence; proof/secret logging;
  unowned retention; deletion/redaction bypass; silent remote transfer.
- **Severity:** Critical.
- **Required remediation:** Remove excess collection, transfer, or retention and
  restore the existing owner/minimization/deletion boundary. Unresolved policy
  returns to architecture review.
- **Whether engineering must stop:** Yes, immediately.

### 11. Complexity Budget

- **What is inspected:** New capabilities, subsystems, processes, services,
  abstractions, dependencies, persistence layers, extension activation,
  duplicated mechanisms, and custom commodity implementations.
- **Pass criteria:** Existing capabilities and contracts are used; each new
  mechanism is necessary for the approved objective, has evidence, has one
  owner, and is the smallest change that satisfies authority; dormant Extension
  Host remains dormant without separate activation.
- **Failure criteria:** Unjustified subsystem/capability; premature abstraction;
  commodity reinvention without required research; dependency or process added
  without lifecycle/security burden; optional complexity made foundational.
- **Severity:** Major; Critical when complexity creates a trust/authority bypass
  or activates an unapproved capability.
- **Required remediation:** Remove the unjustified expansion or stop for the
  existing architecture/research/activation governance. The Guardian proposes
  no replacement.
- **Whether engineering must stop:** Yes.

### 12. Runtime Boundary Violations

- **What is inspected:** Host/domain separation, process and worker authority,
  IPC, lifecycle versus domain commands, runtime configuration, packaged files,
  generated artifacts, diagnostics, and engineering documentation.
- **Pass criteria:** Runtime Host retains lifecycle/routing/health authority
  only; process or IPC boundaries do not broaden authority; domain data remains
  owner-held; engineering documentation is excluded from runtime packaging.
- **Failure criteria:** Host reads or persists domain data; lifecycle fabricates
  domain results; worker/process identity grants product permission; hidden IPC
  path; architecture/research/prompt documents included in runtime; diagnostics
  become durable user knowledge.
- **Severity:** Critical for domain/permission/data authority leakage; Major for
  lifecycle, IPC, packaging, or documentation-boundary violations.
- **Required remediation:** Remove the leaked authority/data/artifact and
  restore the documented runtime/domain/package boundary.
- **Whether engineering must stop:** Yes.

### 13. Documentation Consistency

- **What is inspected:** Blueprint/ADR references, versions, statuses, ownership
  claims, contract IDs/signatures, Matrix paths, acceptance counts, known gaps,
  roadmap state, Catalogue/registry decisions, Ledger chronology, and Current
  State.
- **Pass criteria:** Every changed claim agrees with higher authority and all
  affected records; subordinate summaries are clearly subordinate; historical
  records remain historical; required updates are present once.
- **Failure criteria:** Contradictory active documents; stale current state;
  duplicated authority; research presented as architecture; implementation
  claim without evidence; missing or duplicated Ledger/Catalogue/registry
  record.
- **Severity:** Major for semantic or authority conflict; Minor for
  non-behavioural stale reference, count, title, or status.
- **Required remediation:** Correct only the inconsistent record to match the
  governing authority, preserving history and single-source ownership.
- **Whether engineering must stop:** Major stops the task; Minor blocks
  completion.

### 14. Repository Integrity

- **What is inspected:** Repository root, branch/HEAD evidence, expected
  directories, authoritative architecture/ADR/research packs, file placement,
  links, merge markers, unexpected deletion/rename, generated artifacts,
  package inclusion, secrets, and task-scope change set.
- **Pass criteria:** The reviewed state is reproducible; required authorities
  exist once; links resolve; no conflict markers, secret material, accidental
  artifacts, or unexplained in-scope/out-of-scope changes exist; packaging
  excludes engineering documents.
- **Failure criteria:** Missing/corrupt authority; duplicate authority; broken
  required reference; unresolved merge content; secret or generated noise;
  unexplained deletion/rename; reviewed files differ from the frozen package.
- **Severity:** Critical for missing/ambiguous highest authority, secret
  exposure, or review-package substitution; Major for integrity/scope failure;
  Minor for a non-authoritative broken reference.
- **Required remediation:** Restore repository integrity, remove accidental
  artifacts/exposure, or refreeze the exact intended package and rerun the full
  review.
- **Whether engineering must stop:** Yes; Minor blocks completion.

### 15. Naming Consistency

- **What is inspected:** Canonical capability names, responsibility terms,
  contract/interaction/ADR/research/Ledger IDs, message kinds, status/outcome
  names, versions, paths, and cross-document references.
- **Pass criteria:** One canonical term and identifier is used for each concept;
  aliases are explicitly historical or mapped without semantic change; IDs are
  unique and stable.
- **Failure criteria:** Collision; renamed identifier without governed
  migration; inconsistent status/outcome meaning; ambiguous capability alias;
  incorrect ID or version reference.
- **Severity:** Major when ambiguity can change ownership, authority, routing,
  compatibility, or outcome meaning; Minor otherwise.
- **Required remediation:** Restore the canonical identifier/term or complete
  the existing compatibility/deprecation governance.
- **Whether engineering must stop:** Major stops the task; Minor blocks
  completion.

### 16. Recovery Behaviour

- **What is inspected:** Owner reconciliation, operation identity/reference,
  idempotency, terminal events/status/tombstones, lifecycle epochs, restart
  classes, persisted control custody, migration/restore recovery, and retry
  gates.
- **Pass criteria:** The owner remains authoritative after timeout, event loss,
  crash, restart, or restore; unknown remains unknown until reconciled; restart
  waits for required owner reconciliation; retry never duplicates or broadens
  authority.
- **Failure criteria:** Blind retry; non-owner recovery claim; lost operation
  identity/control; restart presented ready before reconciliation; restore
  resurrects authority or forgotten content; expired history treated as
  success.
- **Severity:** Critical when recovery can repeat effects, resurrect authority
  or private data, or fabricate success; Major otherwise.
- **Required remediation:** Restore the existing owner-authoritative
  reconciliation and retry gate, or stop for missing recovery architecture.
- **Whether engineering must stop:** Yes.

### 17. Failure-Mode Consistency

- **What is inspected:** Invalid, denied, unavailable, dependency-unavailable,
  offline-optional, timeout, cancellation, partial-effect, outcome-unknown,
  indeterminate, duplicate, out-of-order, sequence-gap, revocation, and forced
  shutdown behavior.
- **Pass criteria:** Failure states keep their defined meaning; protected
  operations fail closed; no failure broadens permission, scope, retention, or
  receiver; partial/unknown outcomes block dependent effects and blind retry;
  the user receives the required explanation.
- **Failure criteria:** Failure assumed to be success; timeout treated as
  cancellation; unknown treated as terminal; partial effect hidden; stale event
  rolls state back; dependency failure routes around trust; failure triggers
  silent cloud or extra retention.
- **Severity:** Critical for fabricated success, repeated effect, authority/data
  broadening, or trust-boundary bypass; Major for other semantic inconsistency.
- **Required remediation:** Restore the existing failure category, owner state,
  stop/retry rule, and explanation without inventing fallback behavior.
- **Whether engineering must stop:** Yes.

### 18. User-Journey Consistency

- **What is inspected:** Every journey affected by the task from user intent or
  direct administration through capabilities, contracts, permission,
  persistence, recovery, explanation, and terminal user-visible outcome.
- **Pass criteria:** The complete journey uses permitted existing capabilities
  and contracts; applicable recorded gaps are respected; the task does not
  claim unsupported readiness; Blueprint and ADR outcomes remain end-to-end.
- **Failure criteria:** Missing caller/receiver/control; forbidden shortcut;
  permission or explanation gap; no recovery path; presentation closes before
  consequential outcome; task depends on an unresolved relevant journey gate.
- **Severity:** Critical when the journey violates permission, privacy, human
  control, or can produce uncontrolled effects; Major otherwise.
- **Required remediation:** Remove the unsupported readiness/flow or refer the
  exact existing journey gap to architecture review. The Guardian does not
  design a new journey or capability.
- **Whether engineering must stop:** Yes.

---

## 8. Final Decision Algorithm

Each category produces exactly one category result:

- PASS
- FAIL

Overall decision:

- **PASS** only when all 18 categories PASS.
- **FAIL** when any category FAILS.

Severity does not permit waivers. It controls stop scope and remediation
urgency, not whether a failure may be ignored.

No person, automation, test result, popularity score, delivery deadline, or
lower authority may waive a Guardian FAIL for the unchanged frozen package. A
higher-authority governance decision may change the governing package; that
invalidates the prior review and requires a complete Guardian rerun. The
Guardian neither proposes nor performs that governance change.

---

## 9. Review Evidence Record

The Guardian uses a transient, read-only review worksheet containing:

- review ID and timestamp
- task ID and frozen scope/change-set identity
- baseline branch and HEAD
- authority document versions
- applicable known gaps and accepted ADRs
- for each category: result, severity, evidence references, violated invariant
  when failed, required remediation, and stop scope
- highest severity
- overall decision
- invalidation conditions

Evidence references identify repository artifacts, exact sections, changed
files, validation results, or task-package facts. Assertions without evidence
do not pass.

The worksheet exists only in the active review context or tool transcript. The
Guardian does not create, edit, or persist a repository file, does not modify
the frozen package, and does not emit the worksheet as part of the final output.
The worksheet does not become runtime configuration, architecture authority, a
new audit capability, or a substitute for Engineering Ledger and Current State
updates.

---

## 10. Final Output Contract

The Guardian emits exactly one outcome.

For a successful review, the complete final output is:

```text
PASS
```

For a failed review, the final output is:

```text
FAIL

Minimum architectural corrections required:
- [category and severity] [violated authority/invariant]: [minimum conforming outcome]
```

On FAIL:

- include every independently actionable failure
- merge duplicate findings that have one root correction
- order corrections by Critical, Major, then Minor, preserving category order
- list only minimum architectural corrections
- cite the violated authority or missing evidence
- do not redesign
- do not introduce capabilities
- do not select or research technology
- do not propose code, implementation steps, libraries, schemas, or vendors
- do not edit any file

In this document, “architectural correction” means the minimum change to make
the reviewed work conform to existing architecture, or referral of an
unresolvable conflict to the existing architecture governance process. It does
not authorize the Guardian to change architecture.

---

## 11. Re-review and Completion

After FAIL:

1. Engineering remains stopped according to the highest severity.
2. A separate authorized actor performs either the minimum conforming
   correction or an explicitly authorized higher-authority governance change.
   The Guardian performs neither.
3. Normal task validation is rerun where affected.
4. Ledger and Current State are updated where required.
5. The review package is refrozen.
6. All 18 Guardian categories are rerun; prior PASS results are not reused.

A task is not complete until the latest frozen package receives PASS.

Guardian PASS proves conformance only for the reviewed package and available
evidence. It does not approve technology, guarantee defect absence, supersede
testing, or modify any architecture decision.
