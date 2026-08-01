# Current Mission

Proving that Workspace can help an interruption-heavy Windows professional
return to meaningful work materially faster than existing tools, without
privacy surprise.

---

# Current Milestone

Product Proof — Trusted Interruption Recovery

---

# Completed

✓ Blueprint established

✓ ADR framework established

✓ Engineering Ledger created

✓ Research Catalogue created

✓ Cursor Protocol created

✓ Open Source Registry created

✓ Prompt Pattern Library created

✓ ADR index aligned (`05_Architecture_Decision_Records.md` indexes ADR-0001–0008)

✓ Documentation consistency pass recorded (LEDGER-0001)

✓ Capability Architecture complete (`08_Workspace_Capability_Architecture.md`, LEDGER-0002)

✓ Formal Capability Architecture review complete (`08` v1.1, `09_Capability_Interaction_Matrix.md`, LEDGER-0003)

✓ Capability Contracts complete (`10_Capability_Contracts.md`, LEDGER-0004)

✓ Contract Schema complete (`11_Contract_Schema_and_Acceptance_Specification.md`, LEDGER-0005)

✓ Architectural Acceptance Specification complete (42 conceptual acceptance cases, LEDGER-0005)

✓ Blueprint user-journey architecture validation complete (`12_User_Journey_Architecture_Validation.md`, LEDGER-0006)

✓ Engine/companion experience principle made explicit in Blueprint v1.1 and User Journey Architecture Validation v1.1 (LEDGER-0011)

✓ Capability Technology Research Framework established (`12_Capability_Technology_Research_Framework.md`, LEDGER-0007)

✓ Repeatable evidence, comparison, decision, rejection, and re-evaluation governance defined for all ten capabilities

✓ Permission Authority capability research complete (`research/PERMISSION_AUTHORITY_RESEARCH.md`, `PA-001`, LEDGER-0008)

✓ Permission Authority patterns, open-source comparators, desktop permission models, trade-offs, unknowns, and mandatory evaluation criteria recorded without technology selection

✓ Memory capability research complete (`research/MEMORY_RESEARCH.md`, `MEM-001`, LEDGER-0009)

✓ Local-first Memory patterns, open-source comparators, retrieval architectures, privacy/encryption/synchronization trade-offs, unknowns, and mandatory evaluation criteria recorded without technology selection or rejection

✓ Capability Research Roadmap established (`13_Capability_Research_Roadmap.md`, `ROADMAP-001`, LEDGER-0010)

✓ Remaining research sequence ordered by architectural dependency, risk, uncertainty, implementation impact, and later selection influence

✓ RH-001 repository audit complete: no accepted Runtime Host research artifact,
Catalogue entry, branch, commit, supersession record, or alternate filename
exists; repository references consistently describe the research as future work
(LEDGER-0012)

✓ Final Product Proof Strategy review complete: trusted interruption recovery is
the strongest product-wedge hypothesis, but it remains unproven until measured
against existing workflows and substitutes (LEDGER-0013)

✓ `PP-B01` complete: truthful green kernel baseline restored; `resilience_validation.rs`
repaired against the domain API that exists (LEDGER-0014, commit `2d88331`)

✓ `PP-B04` complete: resilience tests rewritten to validate the selection
immutability invariant against the current public API (LEDGER-0015, commit `da6a85e`)

✓ `PP-B02` complete: zero ambient capture; no observation, enumeration,
persistence, or scheduled sensing occurs before an explicit user-initiated
capture (LEDGER-0016, commit `12573da`)

✓ `PP-B03` complete: production Content Security Policy enforced in the WebView
and pinned by an automated verifier (LEDGER-0017, commit `f77200e`)

✓ `PP-M1-01` complete: a user can name a workspace context, read the
kernel-served scope of what would be captured, confirm or cancel, and receive a
durable bounded record with a summary of exactly what was kept; a refused save
observes nothing (LEDGER-0018, commit `868ce12`)

---

# Active Task

Establish product evidence for the trusted interruption-recovery hypothesis
before sustained capability research or large-scale feature implementation.

The four Product Proof engineering blockers are closed. Product Proof
Milestone 1 is under way: `PP-M1-01` is complete and awaits architectural
review before `PP-M1-02` begins.

---

# Next Task

Validate one primary workflow with interruption-heavy Windows professionals:
explicitly save a bounded work context and intended next action, preview a
supported restore, return to the intended artifact, and inspect or delete what
was retained.

Measure return-to-work improvement against each user's baseline and credible
substitutes, including PowerToys plus manual notes. Treat privacy surprise,
unpreviewed capture, or unpreviewed action as proof invalidation.

Do not activate new capability layers, cognition projections, Extension Host,
voice, cloud, phone, audio, broad automation, or additional governance while
the product hypothesis remains unproven. `ROADMAP-001` remains the valid
dependency order if the proof later justifies resumed capability research; it
is not the active milestone.

Recommended next engineering milestone: **Product Proof**.

---

# Known Unknowns

- Whether interruption and project switching cost the target user enough time
  and attention to drive active adoption
- Whether Workspace can reduce return-to-work time materially beyond Windows,
  PowerToys Workspaces, app-native restoration, and manual notes
- Whether users will maintain bounded saved contexts and a short handoff note
- Whether window and resource metadata are sufficient to recover meaningful
  work without misleading claims
- Whether explicit local capture is acceptable to the target user
- Whether users will repeatedly choose Workspace and pay for the result
- Whether companion value remains after removing ambient observation and
  general-purpose AI from the critical path
- Voice stack (Experience modality)
- Memory taxonomy and the boundary between ephemeral working context and durable retained knowledge
- Memory correction, contradiction, supersession, valid-time, and recorded-time semantics
- Memory retention defaults, pruning authority, consolidation policy, and user visibility
- Exact Memory redaction/forget guarantees across canonical records, derived indexes, summaries, graphs, caches, keys, backups, and replicas
- Memory encryption attacker model, key custody, recovery, rotation, and cryptographic-erasure policy
- Whether Memory synchronization is required; if so, device trust, E2EE, conflict, tombstone, offline-duration, retired-device, and backup-expiry semantics
- Representative Memory scale, Windows hardware classes, retrieval-quality thresholds, and local model viability
- Memory write/redact/forget/reindex/migration commit points, cancellation classes, and crash recovery
- AI orchestration (Intelligence)
- Desktop observation stack (Context Sensing)
- Plugin architecture (Extension Host)
- Action safety taxonomy
- Permission scope granularity
- Permission proof representation and the boundary between opacity and designated local control-proof verification
- Permission Authority process/isolation boundary and same-process bypass resistance
- Durable revocation/use ordering, storage rollback, clock rollback, restart, and offline-control lease semantics
- Permission audit attacker model, tamper-evidence assurance, retention, and checkpoint recovery
- Grant lifetime taxonomy, batch/multi-effect approval boundaries, and independently revocable compound scopes
- Trusted, neutral, bounded consent presentation and requester-controlled text handling
- Local inference viability classes
- Concrete contract representation after technology research (conceptual schema is complete)
- Executable tests for offline/degraded core-function acceptance scenarios
- Executable event loss/duplication/order/recovery and compatibility evidence
- Per-action-class commit points, cancellation, compensation, and partial-effect evidence
- Operation-control lease duration and terminal tombstone retention values
- Extension Host activation justification (Complexity Budget checkpoint)
- Risk that candidate convenience pressures capability ownership or forbidden communication paths
- Risk that logs, telemetry, caches, indexes, or provider histories become alternate Memory stores
- Risk that claimed offline support depends on network installation, activation, authorization, model acquisition, or recovery
- Risk that Windows support omits packaging, accessibility, permissions, signing, or lifecycle evidence
- Risk that aggregate comparison scores hide mandatory privacy, security, contract, or licence failures
- Risk that technology feasibility research is mistaken for Extension Host activation authority
- User-journey contract/guidance gaps in task control, shutdown, sensing lifecycle, cancellation recovery, provider administration, compound remote authorization, degraded startup, attention/consent, and archived-workspace Memory visibility
- Ordered remaining research dependencies: Runtime Host → Workspace Management → parallel Context Sensing / Action / Intelligence → Companion Orchestration → Experience → conditional Extension Host

---

# Architecture Health

Capability decomposition stable; product proof absent; sustained capability
expansion and large-scale implementation are not yet justified

Build integrity restored: `cargo test --workspace` and `pnpm test` are green,
the workspace packages as an MSI and NSIS installer, no observation occurs
before explicit user capture, and the WebView runs under an enforced Content
Security Policy.

The first Product Proof workflow now exists end to end, so the proof hypothesis
has a surface to be tested against rather than only a strategy.

---

# Drift Assessment

The four Product Proof blockers changed behaviour and configuration, not
architecture. Capability ownership, contracts, interactions, and the Blueprint
are unchanged. `PP-B02` enforces an existing Blueprint principle rather than
introducing a new one, and retains the ambient observation implementations in a
dormant, still-tested state instead of deleting them. `PP-B03` removed the only
Content Security Policy exception in the repository, the disabled policy itself,
and required no compensating exception.

`PP-M1-01` introduced no capability, contract, or ownership change, but it did
add a persisted domain concept: a saved context, owned by Workspace Management,
built from a Context Sensing capture the user explicitly authorised. It is
deliberately distinct from `WorkspaceContext` and `WorkspaceSnapshot`, which are
ephemeral compositions, and from `MemoryEntry`, which is Memory's. Two decisions
warrant review: the saved context duplicates window and monitor rows rather than
referencing an observation pass, accepted because passes are purged by
retention; and `SaveWorkspaceContext` enforces `desktop.read` inside the command
rather than through the pipeline, because the pipeline authorises only a single
declared capability. If commands with two effects become common, that check
belongs in the pipeline rather than repeated per command.

Saving is now the view the application opens on, displacing Canvas. This is a
product-sequencing decision made to put the Product Proof workflow first, not an
architectural one, and it is a single line to reverse.

The Content Security Policy is verified by the audit in `pnpm test`, by the
policy string embedded in the packaged `workspace-app.exe`, and by loading the
production bundle under the identical policy in headless Chromium with zero
violations. It has not been observed in a running WebView2 window; that check
belongs to the first pilot build.

The Product Proof Strategy changes readiness and sequencing, not architecture.
The Blueprint, ADRs, capability ownership, interactions, contracts, research
records, and `ROADMAP-001` dependency order remain unchanged. Product proof is
now the active gate before that research roadmap resumes.

No capability ownership drift. The engine exists to serve the Companion
experience; internal capability and operational surfaces are not the primary
user interface. User-journey validation identified bounded contract and
guidance gaps within existing capabilities.

Repository audit confirms that `RH-001` is missing. No repository reference
incorrectly records it as complete: Current State, the Research Catalogue, the
Engineering Ledger, and the Capability Research Roadmap all describe Runtime
Host research as future or prerequisite work. Any prior session report that
called `RH-001` complete is unsupported by the available repository history.
Do not begin an evaluation that requires `RH-001` until an accepted canonical
research artifact and Catalogue entry exist.

Blueprint v1.1 remains the highest authority. Capability Architecture v1.1 remains the authoritative decomposition. The Interaction Matrix is authoritative for communication and trust constraints. Capability Contracts v1.0 is authoritative for public message and interaction semantics. Contract Schema and Acceptance Specification v1.0 is authoritative for conceptual fields, evolution, invariants, and pre-implementation acceptance. User Journey Architecture Validation v1.1 identifies current end-to-end gaps and validates engine/companion experience separation without changing ownership. Capability Technology Research Framework v1.0 is authoritative for research planning, comparison evidence, and technology-evaluation governance. Capability Research Roadmap v1.0 is authoritative for the remaining research sequence and dependency gates.

Technology choices remain open. Permission Authority research unit `PA-001` is
complete and records six candidate solution categories without adoption,
conditional adoption, or formal rejection. It confirms that policy evaluation,
relationship evaluation, proof authenticity, revocation, one-use consumption,
challenge workflow, audit, and explanation are distinct mechanisms and that no
examined candidate supplies the full Workspace contract.

Architecture review must resolve the recorded proof, isolation, time/rollback,
audit, compound-scope, grant-lifetime, recovery, and consent assumptions before
bounded candidate validation or selection. Final selection or implementation
in affected areas still waits for relevant user-journey corrections and
acceptance. Concrete representations, executable validation artifacts,
per-action safety detail, control/tombstone durations, and Extension Host
activation justification remain pre-implementation risks.

Memory research unit `MEM-001` is complete and records eight candidate solution
categories plus representative storage, lexical/vector, graph, framework,
encryption, and synchronization comparators without adoption, conditional
adoption, or rejection. It confirms that ephemeral working context, canonical
durable records, derived retrieval projections, compression, encryption,
deletion, backup, and synchronization require distinct ownership and guarantees.

Architecture review must resolve the recorded taxonomy, archived-scope,
correction, retention, deletion, encryption, synchronization, scale, and
operation-lifecycle assumptions before bounded candidate validation or
selection. Retrieval quality, Windows/resource performance, exact forgetting,
offline model completeness, migration, and failure recovery remain mandatory
reproducible evidence gaps.

Capability Research Roadmap unit `ROADMAP-001` is complete and records the
dependency-driven sequence for the eight remaining research areas without
technology research, recommendation, selection, or rejection. Runtime Host is
first because lifecycle and packaging assumptions fan out to every later
integration. Workspace Management follows because authoritative scope constrains
multiple domain capabilities. Context Sensing, Action, and Intelligence may
proceed as a bounded parallel wave after shared prerequisites. Companion
Orchestration then Experience follow their domain and coordination
dependencies. Extension Host remains last and conditional on separate
activation justification.
