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

✓ Engineering Session Protocol created (`17_Engineering_Session_Protocol.md`,
LEDGER-0024); mandatory after Cursor Protocol and before Current State

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

✓ Final `PP-M1-02` architecture dependencies closed: Action plan interaction,
saved-context restore identity, and Action-owned matching threshold
(LEDGER-0021)

✓ `PP-M1-02` complete: a user can select a previously saved Workspace Context,
review the exact restore plan, approve it, and receive honest per-item outcomes
from deterministic same-session `window.place` / `window.focus` execution;
nothing mutates before approval (LEDGER-0022)

✓ `PP-P01` Pilot Package implementation complete (`PP-P01A`–`PP-P01E`,
LEDGER-0026–0030)

✓ Post-PP-P01 Product Proof Review complete (`18_Product_Proof_Review_PP_P01.md`,
LEDGER-0031): recruitment-readiness gate satisfied; hypothesis unproven but
testable; next objective is LEDGER-0013 pilot recruitment and execution

✓ Participant #1 local installable build defect fixed: SQL migrations are
compile-time embedded so installed binaries no longer depend on a developer
checkout path (LEDGER-0032). Pilot build notes in
`19_Pilot_Participant_1_Local_Build.md`.

✓ Participant #1 Windows install completed (NSIS → Local\Workspace; Desktop and
Start Menu shortcuts launch production UI).

✓ Active-workspace persistence defect fixed (LEDGER-0033).

✓ Experience Fidelity Review and convergence pass completed (LEDGER-0034,
`20_Experience_Fidelity_Review.md`): companion-first Home hub and product
navigation; engineering-form chrome reduced without changing Product Proof
behaviour.

✓ Experience Roadmap accepted (`21_Experience_Roadmap.md`, LEDGER-0035):
Phase 1 companion identity largely achieved; next Experience milestone is
Phase 2 dashboard-first Home — not open-ended UI tweaks or capability growth.

✓ Experience Component Library research complete (EXP-001, LEDGER-0036):
catalogue, licensing matrix, gap analysis, patterns, tokens, OSS candidates,
build plan, and Phase 2 implementation plan under
`architecture/research/experience/`. No dependency approved; no runtime change.

✓ Experience Phase 2 dashboard-first Home convergence complete (LEDGER-0037):
shared moment cards, empty structure, design tokens, Lucide icons; Continue/
Save/Guide/Check-in presentation refined without Product Proof behaviour change.

✓ Experience Recovery Sprint complete (LEDGER-0038): Participant #1 rejection of
page/admin feel accepted as evidence; shell rebuilt as one spatial environment
(atmosphere + glass dock + place/layer modes) without capability or Product
Proof behaviour change. Prior Phase 2 self-score treated as invalid.

✓ Persistent Workspace Shell sprint complete (LEDGER-0039): shell stays mounted;
Home/Save/Continue/Check-in/Guide transition as content with spring motion;
ElevatedCard primitive + unified spatial system; `motion` (MIT) adopted.

✓ Adaptive spatial composition sprint complete (LEDGER-0040): Focus/Balanced/
Flow density, layered depth, living atmosphere, glass surfaces, destination
refinements, premium dock interactions.

✓ Workspace visual language sprint complete (LEDGER-0041): design-system tokens,
ambient lighting, WorkspaceSurface materials, moment objects, canvas Home,
writing Save, cinematic Continue, metric orbs, Guide walkthrough, dock polish.

---

# Active Task

Establish product evidence for the trusted interruption-recovery hypothesis
before sustained capability research or large-scale feature implementation.

The four Product Proof engineering blockers are closed. Product Proof
Milestone 1 is under way: `PP-M1-01` and `PP-M1-02` are complete.

`PP-M1-02` delivered deterministic Resume against the approved contracts
(LEDGER-0021 / LEDGER-0022):

1. Save scope is now `saved-context-scope-v2` and captures restore identity only
   after that scope is confirmed. Legacy v1 contexts remain browseable and are
   never backfilled; preview reports them as identity-unavailable.
2. Companion-side Resume commands load a saved context, copy declared targets
   into Action without a saved-context identifier, resolve an expiring plan, and
   execute only after the user approves that plan digest.
3. Action owns exact-session matching and the fixed confidence floor; Permission
   Authority still alone owns effect authorization through per-item
   `action.window.place` / `action.window.focus` proofs at point of use.

Product Proof pilot package engineering (LEDGER-0023 / LEDGER-0025): the five
PP-P01 blockers are implemented (`PP-P01A`–`PP-P01E`, LEDGER-0030). The
post-PP-P01 Product Proof Review (`18_Product_Proof_Review_PP_P01.md`,
LEDGER-0031) finds the recruitment-readiness gate **satisfied** and the
hypothesis **still unproven but now testable**. Next programme objective is
LEDGER-0013 cohort recruitment and four-week pilot execution — not automatic
implementation of `PP-M1-03` or capability research. `PP-M1-03` (Undo) remains
valuable but stays sequenced after pilot evidence (or a repository-authorised
defect) justifies engineering.

---

# Next Task

The **PP-P01 Pilot Package** implementation slices are complete (LEDGER-0030).
The Product Proof Review after PP-P01 is complete (LEDGER-0031). Completing the
package and the review does **not** prove the Product Proof hypothesis.

Authorised implementation order:

| Slice | Name | Status |
|---|---|---|
| `PP-P01A` | User-authored handoff / intended next action on Save and Resume | **Complete** (LEDGER-0026) |
| `PP-P01B` | Explicit restore-limits copy (same session; still-open windows; no silent relaunch) | **Complete** (LEDGER-0027) |
| `PP-P01C` | Inspect and delete retained saved contexts in the product UI | **Complete** (LEDGER-0028) |
| `PP-P01D` | Pilot-safe primary chrome (Save / Resume + minimal help; engine tabs out of default pilot surface) | **Complete** (LEDGER-0029) |
| `PP-P01E` | Consented measurement and interview kit (baseline, leave→resume time, correction, week-four habit; no ambient observation) | **Complete** (LEDGER-0030) |

**PP-P01 Pilot Package implementation slices: complete** (LEDGER-0030).
**Product Proof Review after PP-P01: complete** (LEDGER-0031).

There is no active implementation slice. Recruitment-readiness gate: **satisfied**.
Product Proof hypothesis: **unproven**. Do not begin implementation from habit.

`PP-P01A` is complete: Save requires an explicit user-authored handoff note;
the note is reviewed before capture, persisted with the saved context, and
shown unchanged on Resume browse, preview, and outcomes. Workspace does not
infer, generate, or rewrite it. Deterministic window restore is unchanged.

`PP-P01B` is complete: Save review/confirmation and Resume browse/preview/
outcomes present shared restore-limits copy — same continuing Windows session,
still-open windows only, no silent relaunch, no file/link opening, no invented
next action. Presentation only; Action restore behaviour is unchanged.

`PP-P01C` is complete: Resume browse offers Inspect and Preview; Inspect shows
retained handoff, windows, and monitors truthfully; Delete requires an explicit
confirmation step and removes the saved context (and restore identities) via
Workspace Management. Deleted contexts cannot be listed, inspected, or resumed.
No background cleanup or retention policy.

`PP-P01D` is complete: Default primary chrome excludes Canvas, Work, Assistant,
and Diagnostic (components retained in the codebase). Workspace creation is
available from Save without Canvas. Help restates the Product Proof loop and
restore limits. Presentation only; restore and persistence behaviour unchanged.

`PP-P01E` is complete: Pilot tab offers consented local measurement — baseline
minutes, participant-entered leave→resume times and corrections, distinct-day
habit summary, and baseline/week-four interview notes. Scope consent is
required; records refuse without active consent; withdraw can clear evaluation
data. No ambient observation, no network upload. Pilot records are evaluation
data, distinct from saved contexts.

Do not activate new capability layers, cognition projections, Extension Host,
voice, cloud, phone, audio, broad automation, fuzzy matching, or additional
governance while the product hypothesis remains unproven. Defer `PP-M1-03`
and declared application launch/reuse until the pilot package can falsify or
confirm the wedge. `ROADMAP-001` remains the valid dependency order if the
proof later justifies resumed capability research; it is not the active
milestone.

Recommended next programme objective: **Recruit and execute the LEDGER-0013
Product Proof pilot** (~15 target users; four weeks; baseline, leave→resume,
correction, week-four habit, trust invalidation watch; local measurement only).
See `18_Product_Proof_Review_PP_P01.md` and LEDGER-0031. Participant #1 can
install/launch with persisted active workspace (LEDGER-0032/0033). Experience
Phase 1 recorded in LEDGER-0034; sequencing continues under
`21_Experience_Roadmap.md` (LEDGER-0035). Next programme objective remains
LEDGER-0013 pilot execution.
Recommended next Experience milestone (when authorised): **Experience Phase 3
— rich visual Continue library** per `21_Experience_Roadmap.md`. Phase 2 Home
dashboard is complete (LEDGER-0037). Capability expansion: **None**.

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
- Action safety taxonomy beyond the two types declared in
  `15_Action_Desktop_Mutation_Contract.md`; every reserved action class still
  needs commit points, compensation rules, and a permission scope before it can
  be declared
- Whether exact same-session matching succeeds often enough to make Resume
  useful. Cross-session and recreated-window restore remain explicitly
  unsupported; this is a Product Proof question, not an architecture gap
- Who owns the pre-effect state that Placement Undo will require: Workspace
  Management's saved-context record, or Action. Deferred to `PP-M1-03` and named
  so no implementation settles it by writing an undo buffer into Action
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

Capability decomposition stable; product proof still absent as *evidence*;
sustained capability expansion and large-scale implementation are not yet
justified

Build integrity restored: `cargo test --workspace` and `pnpm test` are green,
the workspace packages as an MSI and NSIS installer, no observation occurs
before explicit user capture, and the WebView runs under an enforced Content
Security Policy.

The Product Proof workflow and consented local measurement kit exist end to
end. The hypothesis is testable with a LEDGER-0013 cohort; it is not proven.

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

Saving is the view the application opens on. Under `PP-P01D`, Canvas, Work,
Assistant, and Diagnostic are no longer primary pilot tabs; that is a
presentation decision for the Product Proof cohort, not an architectural one.
Engine component modules remain in the repository for later product surfaces.

One instance of capability drift predates Product Proof and is now material.
`LaunchApplication` mutates the environment by spawning a real process, but does
so as an ordinary kernel mutation command guarded by `application.launch`. It
does not pass through `ACT-CMD-001`, emits no `ACT-EVT-*` outcome events, and
belongs to no action catalogue, so environment mutation already occurs outside
the capability that alone owns it. `PP-M1-02` restored through Action rather
than extending that pattern: placement and focus go through declared Action
types, plan resolution, per-item proofs, and honest outcomes.
`LaunchApplication` drift remains open and must not be used as precedent for
further non-Action environment mutation.

The Content Security Policy is verified by the audit in `pnpm test`, by the
policy string embedded in the packaged `workspace-app.exe`, and by loading the
production bundle under the identical policy in headless Chromium with zero
violations. Participant #1 should confirm CSP behaviour in the installed
WebView2 window during daily dogfood (LEDGER-0032 build).

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

`15_Action_Desktop_Mutation_Contract.md` v1.1 is authoritative for Action's
declared action types and their permission, safety, cancellation, idempotency,
item-outcome, and explainability bindings. It introduces no new capability and
redefines nothing: roughly four fifths of what a desktop mutation contract needs
was already specified generically in `10` and `11`, and the contract binds to
those definitions rather than restating them. It adds one request,
`ACT-REQ-004`, because preview integrity is an execution-side property that a
caller-assembled preview cannot guarantee.

The former interaction contradiction is resolved. The Interaction Matrix now
permits Companion to request a non-mutating Action plan with plan-only
authorization, and `IC-030` defines the same purpose and minimized data.

`16_Saved_Context_Restore_Identity_Specification.md` is authoritative for
saved-window identity used by Product Proof restore. It makes the current limit
explicit: `stable_window_id` is local correlation, not a portable OS identity,
and `PP-M1-02` may act only on an exact same-session handle/process/title match.
This prevents implementation from turning a descriptive identifier into a
cross-session restoration promise.

Blueprint v1.1 remains the highest authority. Capability Architecture v1.1 remains the authoritative decomposition. The Interaction Matrix is authoritative for communication and trust constraints. Capability Contracts v1.1 is authoritative for public message and interaction semantics. Contract Schema and Acceptance Specification v1.0 is authoritative for conceptual fields, evolution, invariants, and pre-implementation acceptance. User Journey Architecture Validation v1.1 identifies current end-to-end gaps and validates engine/companion experience separation without changing ownership. Capability Technology Research Framework v1.0 is authoritative for research planning, comparison evidence, and technology-evaluation governance. Capability Research Roadmap v1.0 is authoritative for the remaining research sequence and dependency gates.

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
