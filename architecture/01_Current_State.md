# Current Mission

Building the Workspace engineering foundation.

---

# Current Milestone

Capability Technology Research

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

✓ Capability Technology Research Framework established (`12_Capability_Technology_Research_Framework.md`, LEDGER-0006)

✓ Repeatable evidence, comparison, decision, rejection, and re-evaluation governance defined for all ten capabilities

✓ Permission Authority capability research complete (`research/PERMISSION_AUTHORITY_RESEARCH.md`, `PA-001`, LEDGER-0007)

✓ Permission Authority patterns, open-source comparators, desktop permission models, trade-offs, unknowns, and mandatory evaluation criteria recorded without technology selection

✓ Memory capability research complete (`research/MEMORY_RESEARCH.md`, `MEM-001`, LEDGER-0008)

✓ Local-first Memory patterns, open-source comparators, retrieval architectures, privacy/encryption/synchronization trade-offs, unknowns, and mandatory evaluation criteria recorded without technology selection or rejection

---

# Active Task

None (Memory research complete; awaiting architecture review).

---

# Next Task

Review `MEM-001` and resolve the Memory architecture assumptions that materially
affect taxonomy, archived-workspace visibility, correction/contradiction,
retention/pruning, deletion guarantees, encryption/key recovery,
synchronization, and destructive-operation lifecycle.

After architecture acceptance, define representative Windows hardware, data,
query, and failure workloads, then perform bounded reproducible candidate
validation against the mandatory gates in `research/MEMORY_RESEARCH.md`. Do not
select a technology from documentary comparison alone.

Recommended next engineering milestone: **Capability Technology Research**.

---

# Known Unknowns

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

---

# Architecture Health

Stable

---

# Drift Assessment

No capability ownership drift. Permission Authority research identified unresolved assumptions without changing architecture ownership.

Capability Architecture v1.1 is the authoritative decomposition. The Interaction Matrix is authoritative for communication and trust constraints. Capability Contracts v1.0 is authoritative for public message and interaction semantics. Contract Schema and Acceptance Specification v1.0 is authoritative for conceptual fields, evolution, invariants, and pre-implementation acceptance. Capability Technology Research Framework v1.0 is authoritative for research planning, comparison evidence, and technology-evaluation governance. Repeated validation found no remaining ownership conflicts, circular hard dependencies, duplicate authority, permission bypasses, forbidden direct paths, Blueprint violations, accepted ADR violations, or actionable contract-architecture findings.

Technology choices remain open. Permission Authority research unit `PA-001` is
complete and records six candidate solution categories without adoption,
conditional adoption, or formal rejection. It confirms that policy evaluation,
relationship evaluation, proof authenticity, revocation, one-use consumption,
challenge workflow, audit, and explanation are distinct mechanisms and that no
examined candidate supplies the full Workspace contract.

Architecture review must resolve the recorded proof, isolation, time/rollback,
audit, compound-scope, grant-lifetime, recovery, and consent assumptions before
bounded candidate validation or selection. Concrete representations, executable
validation artifacts, per-action safety detail, control/tombstone durations,
and Extension Host activation justification remain pre-implementation risks.

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
