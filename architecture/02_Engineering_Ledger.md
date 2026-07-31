# Workspace Engineering Ledger

Every engineering change is appended.

Never rewrite history.

---

## Entry schema

Entry ID

Capability

Research

Decision

Implementation

Validation

Knowledge Gained

Unlocks

Supersedes

Status

---

## Entries

### LEDGER-0001

Entry ID: LEDGER-0001

Capability: Architecture documentation (engineering process)

Research: N/A — documentation consistency review only; no technology research performed.

Decision: Convert `05_Architecture_Decision_Records.md` from a partial ADR restatement into an index of all accepted ADRs. Leave accepted ADR files in `architecture/decisions/` unchanged as the source of truth. Do not alter product architecture or Blueprint meaning.

Implementation:
- Replaced incomplete ADR-0001/0002 body copy in `architecture/05_Architecture_Decision_Records.md` with a full accepted-ADR index (ADR-0001 through ADR-0008), one-sentence summaries, and links to `architecture/decisions/`.
- Clarified that `05` is index-only authority.
- Updated Current State to record this documentation refinement.

Validation:
- All eight accepted ADR files remain byte-content authorities; none rewritten.
- Index introduces no new architectural decisions.
- Blueprint, Current State active task (Capability Map), and product principles unchanged.

Knowledge Gained:
- Restating ADR bodies in `05` created duplicate authority and fell out of date when ADR-0003–0008 were added.
- An index document maintains discoverability without competing with `architecture/decisions/`.
- Remaining soft inconsistencies: ADR short titles vs Blueprint principle phrasing (e.g. Permission First vs Permission Before Automation; Explainability vs Explain Every Action; Integrate Before Reinvent vs Integrate Before Reinventing) — naming alignment only, not conflicting decisions.

Unlocks: Clearer onboarding path for new AI/engineering sessions reading the architecture pack.

Supersedes: Prior content of `05_Architecture_Decision_Records.md` as a decision-detail document (role change to index only).

Status: Complete

### LEDGER-0002

Entry ID: LEDGER-0002

Capability: Workspace Capability Architecture (system-wide)

Research: N/A — architecture decomposition only; no technology research or vendor selection performed.

Decision: Adopt `architecture/08_Workspace_Capability_Architecture.md` as the authoritative product capability decomposition. Ten capabilities with single responsibilities, contract-based interaction, fail-closed permissions, local-first operation, and a strict sense/act split. Intelligence may propose but never execute environment actions. Voice is an Experience modality, not a separate capability. Optional cloud is a permissioned Intelligence provider class, not a separate capability.

Implementation:
- Created `architecture/08_Workspace_Capability_Architecture.md` defining Runtime Host, Permission Authority, Workspace Management, Memory, Context Sensing, Action, Intelligence, Companion Orchestration, Experience, and Extension Host.
- Included per-capability contracts, permissions, data ownership, lifecycle, failure behaviour, explainability, research unknowns, and open-source categories (categories only).
- Included system dependency graph, interaction summary, data ownership map, permission boundary map, user interaction flow, startup/shutdown sequences, lifecycle overview, and architectural validation.
- Updated Current State to mark Capability Architecture complete and set next milestone to per-capability technology research via the Research Catalogue.

Validation:
- Reviewed for duplicate responsibilities, circular hard dependencies, ownership conflicts, unnecessary complexity, Blueprint violations, and ADR violations; residual risks resolved via sense/act split and intelligence non-execution rule.
- No runtime code or behaviour modified.
- No ADR bodies rewritten; no technology selections made.

Knowledge Gained:
- Separating Context Sensing from Action is required for permission clarity and explainability.
- Companion Orchestration must remain the sole core coordinator; Intelligence and Extension Host must not become shadow orchestrators with execution power.
- Collapsing voice/cloud/privacy into existing capabilities reduces complexity without losing ADR coverage.

Unlocks: Per-capability technology research entries in the Research Catalogue; subsequent implementation planning against stable ownership boundaries.

Supersedes: None (first capability architecture authority).

Status: Complete

### LEDGER-0003

Entry ID: LEDGER-0003

Capability: Formal review of Workspace Capability Architecture and capability interactions

Research: N/A — architectural review only; no technology research or vendor selection performed.

Decision: Revise the Capability Architecture to v1.1 and adopt `architecture/09_Capability_Interaction_Matrix.md` as the authoritative communication, intermediary, authority, trust, permission, and event-flow constraint document subordinate to the Blueprint and accepted ADRs.

Implementation:
- Removed the hard Experience/Companion dependency cycle by making Experience the caller/subscriber and Companion presentation-independent.
- Replaced generic permission references with purpose-, capability-, subject-, target-, and operation-bound authorization proofs validated at the point of use.
- Removed direct Context Sensing-to-Memory persistence proposals; observation candidates now flow through Companion and a separate Memory authorization path.
- Removed Extension Host access to arbitrary domain capabilities; all extension-originated domain effects flow through Companion.
- Removed direct Intelligence-to-Memory access and direct Experience-to-Workspace mutations.
- Revised shutdown ordering so Experience remains available until consequential operations report cancellation/completion.
- Replaced the ambiguous dependency diagram and created the formal interaction matrix with required intermediaries and forbidden paths.
- Required purpose-limited minimization for every Context Sensing event, including deep-sensing paths.
- Extended bound, point-of-use authorization validation to Workspace Management, Memory, Context Sensing, Intelligence, and activated Extension Host operations.
- Restricted Intelligence persistence to metadata-only transaction records so it cannot become an alternate Memory store.
- Added an authorized Companion-to-Extension-Host management path while keeping extension contributions event-only.
- Routed capability health only to Runtime Host and aggregate host status to Experience.
- Defined cross-capability correlation semantics and offline core-functionality acceptance scenarios.
- Reconciled matrix validation, scope-event, revocation, and health-event permissions with the capability contracts.
- Added an explicit Experience path for authorized read-only Workspace queries and protected Memory item explanations with `memory.read`.
- Restricted persisted Companion records to metadata-only outcomes/correlation; retained user content must flow through Memory.
- Completed the Experience Workspace-read authorization path across outputs, dependencies, graph, and interaction summary.
- Corrected Runtime Host mode delivery to Intelligence, declared Permission Authority for protected Workspace reads, and separated Extension Host events from call dependencies.
- Separated domain-call graph notation from lifecycle/event paths, added the Runtime mode event policy, and completed Experience's Permission Authority dependency description.

Validation:
- Reviewed every capability for responsibility, ownership, boundary, cohesion, coupling, dependency direction, permissions, explainability, data/lifecycle ownership, failure isolation, Local First, Privacy First, Complexity Budget, and Blueprint alignment.
- Re-ran the review after revisions: no ownership conflicts, circular hard dependencies, Blueprint violations, or accepted ADR violations remain.
- No Critical findings and no unresolved Major findings remain.
- No runtime code or behaviour changed; no technologies selected.

Knowledge Gained:
- Public contracts are insufficient as trust boundaries without explicit caller and intermediary restrictions.
- Observation permission must remain separate from retention permission.
- Authorization must be bound and revalidated at the execution boundary to preserve revocability.
- Event return paths must not be represented as reverse hard dependencies.
- Deep-sensing authorization does not waive data minimization.
- Explainability needs shared correlation semantics without shared data ownership.

Unlocks: Capability contract specification and offline/degraded acceptance scenarios; technology research can proceed against explicit communication boundaries.

Supersedes: Capability Architecture v1.0 boundary definitions where revised by v1.1 and the Interaction Matrix.

Status: Complete
