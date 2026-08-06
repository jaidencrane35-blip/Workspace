# Memory Capability Research

Status: Research complete; decision pending
Research ID: MEM-001
Version: 1.0
Date opened: 2026-08-01
Last reviewed: 2026-08-01
External evidence accessed: 2026-08-01

This record follows
`architecture/12_Capability_Technology_Research_Framework.md`. It compares
local-first AI-memory patterns and mature open-source approaches without
selecting or rejecting a technology, approving implementation, or changing
architecture.

---

## A. Identity and Scope

- Capability: Memory
- Technology category or question: local canonical knowledge storage,
  short-term/long-term memory boundaries, structured and semantic retrieval,
  derived indexes, compression, retention, versioning, privacy, encryption,
  optional synchronization, and explainability
- Research status: complete for discovery and comparative analysis; no adoption
  decision
- Research owner: Workspace engineering research
- Catalogue entry: `MEM-001` in `architecture/03_Research_Catalogue.md`
- Related contracts:
  - `MEM-CMD-001` through `MEM-CMD-003`
  - `MEM-REQ-001` through `MEM-REQ-004`
  - `MEM-EVT-001` through `MEM-EVT-005`
  - `IC-015` and `IC-019` through `IC-022`
- Required acceptance cases:
  - Identity and correlation cases 1 through 4
  - Permission cases 5 through 11
  - Command, outcome, and cancellation cases 12 through 20
  - Event cases 21 through 24
  - Privacy cases 25 through 28
  - Local First cases 29 through 31
  - Compatibility cases 35 through 37
  - Audit and trace cases 38 and 39
  - Control and delayed-recovery cases 40 through 42 where destructive
    operations are non-immediate
- Relevant ADRs: ADR-0001 through ADR-0008, with primary emphasis on ADR-0001,
  ADR-0002, ADR-0003, ADR-0004, ADR-0005, and ADR-0006

### In scope

- Boundary between ephemeral working context and durable Memory
- Long-term semantic, episodic-source, structured, and relationship memory
- Canonical local storage and rebuildable derived projections
- Dense vector, sparse/keyword, full-text, metadata/time, graph, and hybrid
  retrieval
- Query transformation, fusion, reranking, context selection, and compression
- Consolidation, deduplication, contradiction, pruning, retention, and versioning
- Provenance and explainability of writes, retrievals, corrections, redactions,
  and forgetting
- Local encryption, key custody, backup leakage, and deletion limitations
- Optional local-first multi-device synchronization patterns
- Windows, Rust, Tauri, embedded-library, native-library, and sidecar implications
- Mature open-source framework and storage comparators
- Build, integrate, and hybrid solution categories

### Out of scope

- Technology adoption, rejection, recommendation, or ranking
- Runtime code, prototype, benchmark, schema, embedding model, or storage format
- Resolution of archived-workspace Memory visibility
- Final memory taxonomy, retention durations, pruning policy, or synchronization
  product requirement
- Local model or embedding-engine selection
- Intelligence prompt construction or Companion task-state implementation
- Context Sensing capture technology
- Remote service adoption or Extension Host activation

---

## B. Capability Objective

Memory must remain the sole owner of durable remembered user knowledge. It must
accept authorized write proposals, retain source and derivation provenance,
retrieve purpose- and workspace-scoped knowledge, explain why a memory was
stored or retrieved, and verifiably redact or forget it.

The architecture distinguishes durable Memory from working context:

- Companion Orchestration owns minimized in-flight task and plan state.
- Intelligence receives caller-supplied, task-bounded context and persists no
  prompt, response, or retrieved Memory content.
- Context Sensing owns ephemeral raw observations and may only emit minimized
  candidates to Companion.
- Memory owns durable knowledge and its retention, provenance, indexes, and
  forgetting lifecycle.

Therefore, an AI framework's combined “working memory” and “long-term memory”
abstraction cannot be adopted as an authority boundary unchanged. Short-term
working context may use Memory retrievals, but it remains ephemeral unless
Companion separately proposes an authorized Memory write.

---

## C. Research Questions

### Answered findings

1. Modern AI-memory systems converge on a tiered model: bounded working context,
   canonical long-term records, and one or more derived retrieval projections.
   The tiers have different ownership, retention, and failure semantics.
2. Dense retrieval, lexical retrieval, metadata/time filtering, graph traversal,
   reranking, and compression solve different problems. No single method is
   consistently sufficient for identifiers, paraphrases, temporal questions,
   relationships, and corrections.
3. A canonical record plus rebuildable indexes is the clearest pattern for
   provenance, migration, corruption recovery, and exact deletion. A search
   index alone is not a durable memory authority.
4. Sparse/BM25 retrieval remains strong for exact names, phrases, code symbols,
   and rare terms. Dense embeddings improve conceptual and paraphrase recall but
   are model-dependent and difficult to explain directly.
5. Hybrid retrieval commonly combines lexical and dense candidates, then uses
   deterministic fusion such as reciprocal rank fusion or score normalization.
   Graph and time signals can supply additional candidate sets or reranking
   features.
6. Graph memory is useful for entities, relationships, multi-hop association,
   and temporal validity. Automated extraction can also amplify incorrect
   entities, duplicate claims, and stale relationships.
7. Context compression reduces model input but is a lossy transformation.
   Extractive selection preserves source fidelity better; abstractive summaries
   are denser but can introduce unsupported claims or erase qualifiers.
8. Consolidation and reflection can create useful higher-level memories only
   when the derived item remains linked to its supporting records and can be
   invalidated, corrected, or deleted with them.
9. Decay, recency, access frequency, utility, confidence, duplication, and
   sensitivity are useful pruning signals. They are not substitutes for
   explicit user forgetting or retention policy.
10. Memory versioning benefits from separating valid time from system/recorded
    time. Overwriting one current fact loses the ability to explain what was
    believed, when, and from which evidence.
11. Dense similarity is not itself a user explanation. A defensible explanation
    records scope and time filters, source/revision identity, candidate ranks,
    fusion contribution, graph path, reranking, and cited source spans.
12. Whole-database encryption, OS key protection, envelope encryption, E2EE,
    logical deletion, physical sanitization, backup expiry, and synchronization
    are separate controls.
13. Search invisibility does not prove deletion. Soft-deleted vectors, old
    segments, summaries, graph edges, WAL/journal files, snapshots, backups,
    caches, and offline replicas can retain derived or historical content.
14. CRDT convergence does not provide authorization, confidentiality, semantic
    correctness, or complete forgetting. Concurrent semantic claims often need
    provenance-aware resolution rather than last-writer-wins.
15. Mature AI-memory frameworks provide reusable extraction, retrieval,
    checkpoint, pipeline, and storage adapters, but their native mutation and
    retention models do not supply the complete Workspace Memory contract.
16. Mature embedded and service retrieval engines offer different combinations
    of structured storage, FTS, vector, graph, versioning, snapshots, and
    filtering. Combining engines increases consistency, deletion, backup,
    lifecycle, and migration obligations.

### Unknowns requiring further investigation

- What canonical memory taxonomy is required: source episode, semantic claim,
  preference, procedure, relationship, summary, attachment, or another set?
- Which short-lived retained items belong in Memory rather than Companion's
  in-flight task state?
- Does a correction create a new revision, a superseding claim, a contradiction,
  or an explicit user adjudication record?
- Which source classes may be consolidated automatically, and which require
  confirmation before becoming durable semantic knowledge?
- What does `forget` guarantee: immediate non-retrievability, logical erasure,
  cryptographic erasure, physical sanitization, backup expiry, replica
  acknowledgement, or a declared combination?
- Which non-content identifiers and provenance classes may survive forgetting?
- How must redaction propagate into summaries, embeddings, lexical terms,
  relationships, cached retrievals, operation records, and backups?
- What merge-specific Memory identity and visibility rules apply when
  Workspaces are merged?
- What retention defaults are calm and privacy-preserving without surprising the
  user or causing repeated cognitive work?
- Which pruning decisions may be automatic, and which require user visibility or
  permission?
- What storage scale, memory count, document size, query rate, and hardware
  classes define representative Windows workloads?
- Which local embedding, reranking, extraction, and summarization viability
  classes are acceptable on supported hardware?
- What retrieval-quality thresholds and benchmark slices represent Workspace
  users rather than generic question-answering?
- Is graph traversal a core requirement, an optional derived projection, or a
  later capability of the same Memory contract?
- Is multi-device synchronization a product requirement? If so, what devices,
  topology, trust model, offline duration, and recovery guarantees apply?
- How long must deletion tombstones and retired-device barriers persist?
- Are old encrypted backups allowed to retain forgotten ciphertext until a
  declared expiry, and how is residual uncertainty explained?
- What local attacker is in scope for encryption: offline disk theft, another
  same-user process, local administrator, malware, or compromised endpoint?
- Which encryption-key recovery trade-off is acceptable between user ownership,
  device loss recovery, and cryptographic erasure?
- What operation commit points and cancellation classes apply to write,
  consolidation, redaction, forget, reindex, and migration?

Capability Architecture v1.2 resolves ordinary archive/restore visibility:
archived-scope Memory remains retained but is excluded from active retrieval;
explicit administration and restored visibility require fresh authorization
and current Workspace scope validation.

---

## D. Required Functional Capabilities

- `MEM-FR-001` — Accept a Companion-mediated write proposal without treating
  extraction or a candidate event as authority. Mandatory.
- `MEM-FR-002` — Validate `memory.write` at the commit point and record the
  proposal, source, purpose, scope, retention intent, and authorization identity.
  Mandatory.
- `MEM-FR-003` — Maintain stable canonical memory and revision identities.
  Mandatory.
- `MEM-FR-004` — Record provenance from source record through extraction,
  chunking, embedding, lexical row, relationship, summary, and later
  consolidation. Mandatory.
- `MEM-FR-005` — Retrieve only purpose-, permission-, and workspace-scoped
  results, applying scope before candidate content crosses the boundary.
  Mandatory.
- `MEM-FR-006` — Evaluate lexical, semantic, metadata/time, graph, and hybrid
  retrieval as replaceable strategies where evidence justifies them.
  Conditional internal strategy requirement; no public strategy contract or
  technology is selected.
- `MEM-FR-007` — Return minimized results with provenance, revision, confidence,
  temporal validity, and retrieval explanation metadata. Mandatory.
- `MEM-FR-008` — Explain what was remembered, why, source, scope, retention,
  permission, revisions, retrieval reason, and removal path. Mandatory.
- `MEM-FR-009` — Redact a whole item, field, or span and regenerate or remove
  every affected projection. Mandatory.
- `MEM-FR-010` — Forget canonical content and all derived artifacts according to
  a declared deletion guarantee. Mandatory.
- `MEM-FR-011` — Maintain a reverse derivation manifest sufficient to enumerate
  affected lexical rows, vectors, graph facts, summaries, caches, attachments,
  and sync operations. Mandatory.
- `MEM-FR-012` — Detect and represent correction, contradiction, supersession,
  temporal invalidation, and duplicate evidence without silently rewriting
  history. Mandatory.
- `MEM-FR-013` — Apply explicit retention and pruning policies independently
  from retrieval ranking. Mandatory.
- `MEM-FR-014` — Publish write, forget, redact, policy, and indeterminate terminal
  outcomes only to the requesting Companion task. Mandatory.
- `MEM-FR-015` — Reconcile non-immediate operations by owner status and
  content-free terminal tombstone. Mandatory.
- `MEM-FR-016` — Rebuild every derived index from canonical records and declared
  model/configuration versions. Mandatory.
- `MEM-FR-017` — Evaluate export of canonical records, provenance, policies,
  and required history in a technology-neutral logical form. Conditional until
  an owner-authoritative administration contract and policy are accepted.
- `MEM-FR-018` — Keep working context bounded and ephemeral unless separately
  proposed for durable retention. Mandatory boundary requirement.
- `MEM-FR-019` — If synchronization is later enabled, preserve authorization,
  provenance, scope, version, redaction, and forget barriers across replicas.
  Conditional.

---

## E. Required Non-Functional Capabilities

- `MEM-NFR-001` — Durable local canonical knowledge survives expected crash and
  restart without requiring internet.
- `MEM-NFR-002` — Retrieval failure degrades to reduced context; it never
  fabricates memory or broadens scope.
- `MEM-NFR-003` — Retrieval quality is reproducibly measured by query class,
  memory class, time, scope, and strategy rather than one aggregate score.
- `MEM-NFR-004` — Interactive retrieval has bounded latency on declared Windows
  hardware classes; numerical thresholds require measurement.
- `MEM-NFR-005` — Storage, index, model, CPU, RAM, GPU, disk, battery, and
  background growth are bounded and observable without content leakage.
- `MEM-NFR-006` — Destructive operations have deterministic, crash-recoverable
  progress and post-operation verification.
- `MEM-NFR-007` — Canonical data remains readable and recoverable by Memory when
  any derived index is missing, corrupt, incompatible, or rebuilding. External
  export remains conditional on an accepted administration contract.
- `MEM-NFR-008` — Indexes, caches, backups, and model-provider histories cannot
  become alternate Memory stores.
- `MEM-NFR-009` — Storage, lexical, vector, graph, embedding, reranking, and
  compression components remain replaceable behind Memory contracts.
- `MEM-NFR-010` — Unknown schema, model, index, scope, permission, or terminal
  semantics fail closed.
- `MEM-NFR-011` — Audit and observability use controlled classes and identifiers,
  never copied memory content or free-text user intent.
- `MEM-NFR-012` — Every write, retrieval, redaction, forgetting, rebuild, and
  migration path is testable with network disabled.
- `MEM-NFR-013` — Corruption, incompatible index, lost event, duplicate command,
  and partial deletion produce honest owner-authoritative states.
- `MEM-NFR-014` — Windows packaging, update, signing, standard-user operation,
  and shutdown are reproducible for every introduced native binary or process.

---

## F. Local First Requirements

Core write, retrieval, explanation, redaction, forgetting, policy, rebuild, and
recovery must work with all network interfaces unavailable when local
prerequisites exist.

If Memory export or backup/restore is later accepted through architecture
contracts, those administration paths must also be fully offline. Research and
candidate comparison may test them now, but may not treat them as approved
product contracts.

Candidate implications:

- An embedded database or search library can avoid a service lifecycle but still
  introduces native binaries, index formats, migrations, and in-process attack
  surface.
- A local sidecar can operate offline but adds startup, readiness, local
  authentication, IPC, ports, logs, crash recovery, packaging, updates, and
  ordered shutdown.
- A framework configured with remote embeddings, extraction, reranking, or
  summarization is not locally complete merely because its store is local.
- Model acquisition, first-run activation, migration, backup restoration, key
  recovery, and index rebuild must also have a declared offline path.
- Optional synchronization or remote backup must never block local reads,
  writes, redaction, forgetting, or permission administration.
- A disconnected replica cannot silently resurrect forgotten content.

Required future evidence:

- Cold start, write, retrieve, explain, redact, forget, rebuild, export, backup,
  restore, and migration with network disabled
- Local-provider absence and resource exhaustion behavior
- Sidecar and native-library failure during startup and shutdown
- Proof that telemetry, hosted control planes, model APIs, and cloud sync are
  disabled by default
- Recovery after interrupted storage, encryption, reindex, and deletion work

---

## G. Privacy Requirements

Memory contains the most privacy-sensitive durable user data in Workspace.
Content, embeddings, keywords, entity names, graph topology, temporal patterns,
access frequency, and retrieval traces can all disclose user knowledge.

Required controls:

- Store only authorized, purpose-justified content with source and retention
  classification.
- Apply workspace and permission filters before exposing candidate content to a
  retriever, model, sidecar, or caller.
- Treat embeddings and graph structure as sensitive derived data, not anonymous
  metadata.
- Disable content-bearing logs, traces, telemetry, crash reporting, training,
  and provider history by default.
- Keep temporary plaintext, exports, migration files, index builds, model
  prompts, and diagnostic bundles inside the same declared protection boundary.
- Record every derived artifact's source identity so redaction and forgetting can
  propagate deterministically.
- Keep retrieval audit high-level and bounded; detailed long-term rationale
  remains Memory only when separately authorized.
- Exclude Memory content from Runtime Host, Action, Intelligence, Companion, and
  Extension Host persistence.
- Make optional remote transfer explicit, separately permissioned, minimized,
  and visible before data leaves the device.
- Document backup retention and the limits of deleting content from user-created
  or disconnected copies.

Whole-store encryption does not remove these obligations. A process with the key
can read plaintext, and derived data may leak through another file, process, or
provider.

---

## H. Security Considerations

### Threats

- Unauthorized cross-workspace or cross-user retrieval
- Memory poisoning through untrusted documents, model output, extensions, or
  spoofed provenance
- Prompt injection retained as trusted semantic or procedural memory
- Query/filter injection into SQL, search, or graph adapters
- Malicious/corrupt index files causing unsafe deserialization or resource use
- Stale derived indexes returning redacted, superseded, or revoked content
- Sidecar APIs exposed without authentication or bound beyond loopback
- Key theft, plaintext crash dumps, swap/hibernation, backup leakage, and
  temporary-file leakage
- Rollback to an older database or snapshot that predates correction or
  forgetting
- Offline replica resurrection of forgotten content
- Dependency or model supply-chain compromise
- Resource exhaustion through large documents, high-dimensional vectors,
  pathological queries, graph expansion, or rebuild storms

### Encryption patterns

- Whole-database encryption preserves SQL/search capability and simplifies broad
  at-rest protection but creates a large key blast radius.
- User-scoped Windows DPAPI or CNG/TPM-backed keys can protect local key
  material. Same-user malware, device loss, recovery, TPM reset, and portability
  remain concerns.
- Application-layer envelope encryption can provide per-item or per-epoch keys,
  narrower compromise, E2EE, and possible cryptographic erasure. It complicates
  filtering, FTS, vectors, graph relations, deduplication, key rotation, and
  recovery.
- Full-volume BitLocker protects offline storage, paging, hibernation, and crash
  dumps on the protected volume but not data from an authorized running process.

### Deletion limits

Logical deletion, index cleanup, database compaction, cryptographic key
destruction, backup expiry, replica acknowledgement, and physical-media
sanitization are distinct guarantees.

Application overwrite cannot prove per-record physical erasure from SSD
wear-levelled cells, remapped blocks, volume snapshots, old files, or
disconnected replicas. Workspace must state the guarantee precisely instead of
claiming universal physical deletion.

### Required security evidence

- Point-of-use authorization and scope isolation before content exposure
- Parameterized/validated SQL, filter, graph, and query construction
- Malicious and corrupt index handling
- Local API authentication and loopback isolation for sidecars
- Key generation, storage, use, rotation, backup, recovery, and destruction
- Crash-dump, temp-file, WAL/journal, model cache, export, and log controls
- Memory-poisoning and provenance spoofing tests
- Dependency advisories, signed artifacts, reproducible builds where available,
  and update response
- Rollback, stale-index, stale-replica, and deleted-content resurrection tests

---

## I. Performance Considerations

No candidate benchmark was performed. Performance remains an evidence gap.

Representative workloads must vary:

- Canonical item count, revision count, text bytes, attachment size, and
  relationship density
- Embedding dimensions and number of embedding-model generations
- Exact-name, paraphrase, temporal, multi-hop, contradictory, and scoped queries
- Cold/warm caches, concurrent reads/writes, and background consolidation
- Redaction/forget fan-out across lexical, vector, graph, summary, backup, and
  sync projections
- Full and incremental rebuild, backup, restore, export, and migration

Measure:

- Cold/warm readiness
- Write proposal acceptance and terminal commit latency
- Retrieval p50/p95/p99 by stage: filtering, lexical, dense, graph, fusion,
  reranking, and compression
- Recall, precision, MRR, nDCG, citation accuracy, scope leakage, and abstention
- CPU, RAM, GPU/VRAM, disk, battery, binary size, and background utilization
- Storage amplification from revisions, vectors, graphs, summaries, WALs, and
  tombstones
- Redact/forget completion and verification time
- Index rebuild and model migration time
- Sidecar IPC and startup/shutdown overhead
- Worst-case behavior under corruption, incompatible index, resource pressure,
  and network-disabled operation

---

## J. Explainability Considerations

A retrieval explanation should allow Workspace to state:

- what query purpose and workspace scope were used
- which permission and authorization identity applied
- which canonical items and revisions were considered and returned
- source class, provenance, valid time, recorded time, and retention class
- which lexical, dense, metadata/time, graph, and other retrieval stages
  contributed
- original per-stage rank or score and fusion contribution
- graph path or relationship evidence where used
- reranker movement and compression/summary transformation
- cited source spans supporting the returned memory
- uncertainty, contradiction, stale-index, degraded, or omitted-result status
- how the item can be corrected, redacted, or forgotten

Explanations must not reveal:

- unauthorized candidates or filtered content
- raw embeddings as if they were meaningful reasons
- hidden prompts, model chain-of-thought, encryption keys, or provider secrets
- another workspace's item count, terms, relationships, or access pattern

Dense similarity is a retrieval signal, not a causal explanation. Abstractive
compression must cite its inputs and disclose that it is a derived summary.

---

## K. Licensing Evaluation

Licence statements are research snapshots. Exact components and transitive
dependencies require legal review before any future adoption.

- SQLite 3.53.4: public domain.
- SQLCipher 4.17.0 Community Edition: BSD-style/BSD-3-Clause; commercial and
  FIPS packages have separate terms.
- Tantivy 0.26.1: MIT.
- FAISS 1.14.3: MIT.
- USearch 2.26.0: Apache-2.0.
- sqlite-vec 0.1.9: MIT OR Apache-2.0.
- LanceDB Rust/Node 0.33.0: Apache-2.0.
- Qdrant 1.18.3: Apache-2.0.
- Meilisearch 1.51.0 Community Edition: MIT; enterprise portions use separate
  terms including BSL/commercial licensing.
- Chroma 1.5.9: Apache-2.0.
- LadybugDB 0.18.3: MIT.
- Mem0 Python 2.0.14: Apache-2.0.
- LangGraph 1.2.10: MIT.
- Graphiti Core 0.29.3: Apache-2.0.
- LlamaIndex Core 0.14.23: MIT.
- Haystack 3.0.0: Apache-2.0.
- Automerge JavaScript 3.3.2 / Rust 0.10.0: MIT.
- Yjs 13.6.31: MIT.
- libSQL: MIT for the evaluated open-source repository line.
- Syncthing 2.1.2: MPL-2.0.
- libsodium 1.0.22: ISC.

No licence is approved. Binary redistribution, notices, patents, trademarks,
model/data licences, optional hosted services, enterprise features, native
dependencies, and copyleft implications remain exact-scope review items.

---

## L. Maintenance Evaluation

- SQLite has long-lived maintenance, current 2026 releases, extensive
  documentation, integrity tools, and Windows artifacts.
- Tantivy has current 2026 releases and a substantial Rust search ecosystem.
- FAISS, USearch, LanceDB, Qdrant, and Meilisearch have current 2026 release
  activity. Their release cadence and migration surfaces differ materially.
- sqlite-vec resumed active releases in 2026; stable 0.1.9 remains pre-1.0 and
  brute-force, while ANN work is prerelease.
- LadybugDB is the active successor to archived Kuzu and has current releases,
  Windows artifacts, and Rust bindings; its successor age and binding/core
  version alignment require evidence.
- Mem0, LangGraph, Graphiti, LlamaIndex, and Haystack have current 2026 releases.
  Rapid evolution and broad integration matrices increase migration and
  dependency-freshness review needs.
- Letta's former server repository is identified as legacy while active work
  moved to Letta Code, making product-line and migration scope material.
- Zep Community Edition is deprecated while Graphiti remains active; these are
  separate maintenance facts and do not constitute a Workspace rejection.
- CRDT and synchronization projects have different protocol stability,
  implementation-language, server, and key-management burdens.

Before selection, exact versions require security-policy, advisory, maintainer
concentration, issue response, signed-artifact, dependency freshness, Windows
CI, migration, and long-term replacement review.

---

## M. Community Maturity Evaluation

The candidate set spans:

- long-lived embedded storage and retrieval libraries
- active vector and search engines
- newer embedded multimodel stores
- service-oriented retrieval infrastructure
- agent-memory and RAG frameworks
- temporal graph-memory frameworks
- mature collaborative CRDTs and file synchronization

Community maturity must be evaluated by sustained maintenance, governance,
operational knowledge, contributor diversity, security response, release
quality, and migration support. Stars, downloads, funding, foundation
membership, or vendor adoption alone do not establish Workspace fit.

Broad framework ecosystems improve adapter availability but increase dependency
and semantic variability. Smaller libraries reduce surface but may concentrate
maintenance and platform knowledge.

---

## N. Platform Compatibility

Windows is mandatory.

- SQLite and FTS5 are embedded and widely supported on Windows.
- Tantivy is Rust in-process and avoids a separate service.
- sqlite-vec publishes Windows x64 extension artifacts but introduces a native
  SQLite extension and pre-1.0 compatibility surface.
- FAISS is C++ with Windows support; native Rust/Tauri use requires FFI, toolchain,
  and numerical-library packaging.
- USearch publishes Windows artifacts and Rust bindings through a C++ boundary.
- LanceDB embeds through Rust/Node and has Windows CI; evaluated 0.33.0 metadata
  requires a newer Rust toolchain than Workspace's current minimum and must be
  validated rather than assumed compatible.
- Qdrant publishes a Windows binary but introduces a sidecar/service lifecycle
  and documented Windows file-handling concerns that require exact-version
  testing.
- Meilisearch publishes a large Windows executable and introduces a local
  service/API lifecycle.
- Chroma's Rust client targets a running server; its embedded local path is
  Python-oriented and adds packaging/runtime implications.
- LadybugDB publishes Windows x64/ARM64 libraries and Rust bindings; static
  artifacts are materially larger than dynamic libraries.
- Python-first AI-memory frameworks require a Python runtime or service boundary
  unless only algorithms/contracts are adapted.
- JavaScript CRDTs fit the frontend runtime but cannot own durable Memory or
  bypass the Rust backend boundary.

Every candidate needs reproducible Windows x64/ARM64, standard-user, installer,
signing, update, rollback, antivirus, filesystem, locale, long-path, crash,
sleep/resume, and shutdown evidence for the exact packaged form.

---

## O. Integration Complexity

### Canonical store plus embedded indexes

Keeps process count low and may share transactions when indexes live in the same
engine. Separate embedded indexes still need outbox/reconciliation, rebuild,
backup, deletion, and migration orchestration.

### Single embedded multimodel store

Can unify structured records, vectors, full-text, and graph operations. It also
couples canonical data to a broader engine, index format, migration model, and
release lifecycle.

### Canonical store plus sidecar retrieval services

Adds process supervision, local authentication, IPC, ports, readiness, health,
snapshots, logging, update, schema, crash recovery, and shutdown. The sidecar
must remain a rebuildable projection rather than a second authority.

### AI-memory or retrieval framework

Provides extraction, deduplication, retrieval pipelines, reranking, memory
blocks, checkpointing, and adapters. Workspace must still intercept native
mutations, own authorization/provenance/retention/deletion, prevent cloud
defaults, and map operation outcomes.

### Graph and synchronization layers

Graph extraction adds model calls, schema evolution, attribution, temporal
invalidation, and cascade deletion. Synchronization adds device identity,
authorization, E2EE, conflict resolution, anti-entropy, tombstones, compaction,
retired-device policy, and recovery.

Complexity is measured by introduced trust, process, storage, model, migration,
and lifecycle boundaries, not only adapter code.

---

## P. Extensibility

A viable future composition must permit:

- adding retrieval strategies without changing Memory ownership
- independent embedding, tokenizer, reranker, graph, and compression versions
- rebuilding projections from canonical revisions
- versioned provenance and derivation manifests
- additional memory types without untyped free-form authority
- logical export and replacement of every engine
- bounded coexistence during storage/index/model migration
- future optional synchronization without making remote availability core
- conflict and temporal semantics that remain explainable
- unknown required fields or semantics to fail closed

Extensibility does not justify implementing every retrieval mode, a generic
knowledge platform, a distributed database, or synchronization before
demonstrated requirements.

---

## Q. Failure Modes

### Canonical store unavailable or corrupt

Writes, reads, redaction, and forgetting fail closed. Companion continues with
reduced context and explains Memory unavailability. No index is promoted to
authority.

### Derived index missing, corrupt, or stale

The affected retrieval strategy is unavailable or results are marked stale.
Canonical records remain authoritative. Rebuild occurs from declared revisions;
stale results do not cross the boundary.

### Partial write or projection update

The owner records the canonical transaction and projection progress. Recovery is
idempotent. Acceptance is never presented as complete before required
projections and verification reach the declared terminal state.

### Retrieval timeout or model failure

The request becomes timed out/degraded, not empty-memory success. Optional stages
may be skipped only when the explanation and quality policy permit.

### Scope or permission mismatch

Fail closed before candidate content is exposed. Caches and sidecars cannot
reuse a result across purpose, authorization, user, or workspace boundaries.

### Memory poisoning or provenance failure

The candidate memory write is quarantined or refused; untrusted text is not
promoted to trusted procedural instruction. Existing canonical records remain
unchanged.

### Embedding, tokenizer, or graph-schema change

Old and new projections are versioned and may coexist for a bounded migration.
Queries use one declared interpretation; incompatible results are not silently
fused.

### Redaction or forgetting interruption

The item becomes unavailable immediately under a deletion barrier. The owner
resumes deterministic per-projection purge and verification, reports partial or
indeterminate state honestly, and never resurrects content from a stale index.

### Backup or replica retains content

The user-visible outcome distinguishes local non-retrievability, key revocation,
projection purge, replica acknowledgement, backup expiry, and residual
uncertainty.

### Sidecar crash or exposed endpoint

Retrieval degrades; local API access fails closed. The sidecar restarts only
under Host lifecycle and rebuilds/reconciles from canonical state.

### Resource exhaustion

Bounded query, graph expansion, context, model, memory, disk, and rebuild limits
apply. Core Memory remains available in a reduced deterministic mode where safe.

### Duplicate, missing, or out-of-order messages

Commands deduplicate by message identity. Missing terminal events trigger owner
status reconciliation. Events never authorize writes, redactions, or forgetting.

### Migration or update failure

The previous compatible canonical form remains recoverable. Rollback cannot
restore forgotten content or discard a redaction barrier. Unknown required
semantics fail closed.

---

## R. Migration Risk

Material migration surfaces include:

- canonical memory type and revision schema
- provenance and derivation identifiers
- workspace scope and archived-scope semantics
- tokenizer, language analyzer, stop words, and FTS index
- embedding model, dimensions, normalization, distance metric, and vector index
- graph entity/relationship schema, extraction prompts, and temporal rules
- deduplication, contradiction, consolidation, and pruning policy
- encryption format, KDF parameters, keys, device membership, and recovery
- storage engine, sidecar protocol, snapshot, and backup format
- synchronization operations, causal history, tombstones, and compaction
- explanation reason classes and retrieval trace

Syntactic export is insufficient if the replacement changes source authority,
scope filtering, contradiction meaning, temporal validity, deletion guarantees,
or retrieval explanation.

A migration must:

- preserve stable canonical and source identities
- keep old/new versions in bounded coexistence
- prevent stale indexes or replicas from exposing redacted/forgotten content
- prove retrieval and deletion equivalence
- support rollback without resurrecting removed knowledge
- retain an engine-neutral logical export
- permit complete projection rebuild
- explain quality or semantic changes to the user when material

---

## S. Reasons to Build Internally

Evidence supports keeping these responsibilities Workspace-owned:

- canonical memory identity, revisions, and authority
- permissioned propose-write lifecycle
- scope, retention, correction, contradiction, redaction, and forget semantics
- provenance and complete derivation lineage
- deletion barriers, per-projection purge, and verification
- retrieval strategy orchestration and explanation schema
- operation state, recovery, terminal history, and audit minimization
- working-context versus durable-Memory boundary
- local/optional-remote and future synchronization policy

These implement unique Workspace trust, privacy, ownership, and explainability
semantics. Internal ownership does not imply implementing databases, FTS,
vector algorithms, graph traversal, cryptography, CRDTs, or model runtimes from
first principles.

---

## T. Reasons to Integrate Externally

Commodity components may reduce correctness and maintenance risk for:

- local ACID storage, backup, integrity checks, and transactions
- lexical tokenization, inverted indexes, and BM25
- vector distance, ANN indexes, filtering, and compression
- graph storage and traversal
- embedding, reranking, extraction, and summarization model execution
- deterministic fusion and retrieval-pipeline utilities
- authenticated encryption, KDFs, and Windows key protection
- CRDT merge primitives and transport-independent change encoding
- retrieval evaluation metrics and benchmark harnesses

Integration is justified only when adapters preserve Workspace ownership,
permission, scope, Local First, privacy, deletion, explanation, export,
replacement, and Complexity Budget constraints. A component's native memory API
does not become Workspace's public or authoritative Memory contract.

---

## U. Candidate Comparison

### Candidate solution categories

1. Workspace-owned canonical local store with same-engine lexical/vector
   projections
2. Workspace-owned canonical local store with separate embedded lexical, vector,
   and optional graph libraries
3. Embedded multimodel retrieval store behind Workspace Memory contracts
4. Canonical local store with one or more local sidecar retrieval services
5. AI-memory/retrieval framework adapted as non-authoritative pipeline components
6. Revisioned source/claim store with temporal graph and hybrid projections
7. Optional synchronization layer over canonical operations and deletion barriers
8. Internal Memory orchestration using external commodity storage, retrieval,
   cryptography, and model primitives

### SQLite 3.53.4 with FTS5 and structured data

- Category: embedded canonical structured store and lexical index
- Evidence: ACID transactions, JSON, FTS5/BM25, backup API, integrity checks,
  secure-delete controls, `VACUUM`, and Windows support
- Trade-offs: no native semantic vector, graph, E2EE, or retrieval fusion;
  external-content FTS requires strict synchronization
- Deletion implication: logical/FTS secure-delete and compaction are available,
  but backups, WALs, SSD remapping, and external projections remain separate
- Disposition: retained for comparative evaluation; no selection or rejection

### Tantivy 0.26.1

- Category: embedded Rust lexical/full-text index
- Evidence: schema, segment indexing, atomic metadata, commit/delete/update, and
  active Rust maintenance
- Trade-offs: separate canonical store and dual-write/rebuild lifecycle; no
  vector, graph, encryption, or portable logical export
- Disposition: retained as an embedded lexical comparator; no selection or
  rejection

### sqlite-vec 0.1.9

- Category: embedded SQLite vector extension
- Evidence: stable Windows artifacts, SQL metadata/partition filtering, ordinary
  deletion, and same-database integration
- Trade-offs: stable line is pre-1.0 and brute-force; ANN work is prerelease;
  extension compatibility and exact deletion require validation
- Disposition: retained as a same-engine vector comparator; no selection or
  rejection

### FAISS 1.14.3 and USearch 2.26.0

- Category: embedded vector-index libraries
- Evidence: multiple vector index families and serialization; USearch has Rust
  bindings and Windows artifacts; FAISS has broad ANN/compression/GPU support
- Trade-offs: native C/C++ boundaries, engine-specific files, separate metadata
  and canonical records, removal/compaction differences, and no transactional
  cross-store commit
- Security note: FAISS documentation warns that untrusted index files are not
  validated and may cause unsafe resource use
- Disposition: retained as vector-library comparators; no selection or rejection

### LanceDB Rust/Node 0.33.0

- Category: embedded multimodal retrieval store
- Evidence: dense vector, scalar, FTS, hybrid search, filtering, Arrow/Lance
  interoperability, and table versioning
- Trade-offs: soft deletion and historical versions require cleanup; local
  encryption and logical backup guarantees need evidence; toolchain/dependency
  footprint and version coupling are material
- Disposition: retained as an embedded retrieval-store comparator; no selection
  or rejection

### Qdrant 1.18.3

- Category: local vector/sparse/hybrid sidecar service
- Evidence: dense/sparse vectors, payload filters, multi-stage queries, WAL,
  snapshots, Windows binary, API-key/JWT/TLS options
- Trade-offs: process/API lifecycle, default self-hosted hardening, sidecar
  authentication, snapshots retaining history, and exact Windows filesystem
  behavior
- Disposition: retained as a local vector-service comparator; no selection or
  rejection

### Meilisearch 1.51.0

- Category: local lexical/semantic/hybrid search service
- Evidence: Windows binary, full-text and hybrid search, filters, asynchronous
  tasks, same-version snapshots, and forward-portable dumps
- Trade-offs: large sidecar, local endpoint and task lifecycle, separate
  canonical authority, and no established local file-encryption or forensic
  deletion guarantee
- Disposition: retained as a search-service comparator; no selection or rejection

### Chroma 1.5.9

- Category: AI-oriented vector/search service and Python-embedded store
- Evidence: records, metadata filtering, deletion, local persistence, collection
  copy tooling, and broad framework integration
- Trade-offs: Rust client targets a server, Windows/runtime packaging is
  material, backup/physical-deletion evidence is limited, and the evaluated
  stable line had an unresolved critical server advisory on the research date
- Disposition: retained as an AI-search infrastructure comparator with current
  security evidence recorded; no selection or rejection

### LadybugDB 0.18.3

- Category: embedded property graph with vector and full-text indexes
- Evidence: Cypher, ACID transactions, native vector/FTS indexes, Rust binding,
  Windows x64/ARM64 artifacts, and active successor development after Kuzu
- Trade-offs: newer successor history, core/binding version alignment, native
  binary size, and unverified backup/encryption/forensic-deletion behavior
- Disposition: retained as an embedded graph/multimodel comparator; no selection
  or rejection

### Mem0 Python 2.0.14

- Category: AI-memory framework
- Evidence: extraction, deduplication, metadata scopes, history, deletion,
  configurable vector/entity stores, reranking, and local-provider configuration
- Trade-offs: cloud defaults require explicit removal; native `add` mutates
  framework state rather than Workspace `proposeWrite`; canonical source-of-truth
  documentation varies by version/configuration; cascade erasure is not the
  Workspace contract
- Disposition: retained as an AI-memory framework comparator; no selection or
  rejection

### LangGraph 1.2.10, LlamaIndex Core 0.14.23, and Haystack 3.0.0

- Category: persistence, indexing, retrieval, and pipeline frameworks
- Evidence: thread checkpoints/cross-thread stores; token-bounded memory blocks;
  retrievers, fusion, rankers, document stores, evaluation, and explicit
  pipelines
- Trade-offs: generic stores/pipelines do not define Workspace provenance,
  permission, retention, redaction, exact deletion, or sole ownership; Python
  runtime and integration-package breadth affect packaging and supply chain
- Disposition: retained as framework and algorithm comparators; no selection or
  rejection

### Graphiti Core 0.29.3

- Category: temporal graph memory framework
- Evidence: source episodes, entities, relationships, valid/invalid time,
  provenance, and hybrid semantic/keyword/graph retrieval
- Trade-offs: graph backend and extraction models add lifecycle; shared entities
  and summaries complicate exact deletion; local configuration does not prove
  single-process or fully offline operation
- Disposition: retained as a temporal graph-memory comparator; no selection or
  rejection

### SQLCipher 4.17.0, DPAPI/CNG, and libsodium 1.0.22

- Category: encryption and key-custody primitives
- Evidence: page/journal encryption and integrity; Windows user/device key
  protection; authenticated encryption, KDF, and streaming primitives
- Trade-offs: whole-store key blast radius versus per-item complexity; plaintext
  process memory, temporary files, backups, searchability, rotation, recovery,
  and device loss remain system obligations
- Disposition: retained as security-control comparators; no selection or rejection

### Automerge, Yjs, SQLite changesets, libSQL, and Syncthing

- Category: optional local-first synchronization patterns
- Evidence: CRDT convergence and conflict retention; database change capture;
  embedded-replica/primary patterns; mature encrypted file replication
- Trade-offs: none supplies Workspace semantic conflict, authorization, E2EE,
  provenance, tombstone, or forget-completion semantics unchanged; file sync is
  not active-database semantic replication
- Disposition: retained as synchronization-pattern comparators; no selection or
  rejection

### Internal Memory orchestration

- Category: Workspace-owned authority and lifecycle
- Strengths: exact ownership, contract, provenance, deletion, explanation, and
  working/durable boundary fit
- Trade-offs: highest responsibility for semantic policy, adapters, recovery,
  evaluation, migration, poisoning defense, and long-term maintenance
- Disposition: mandatory build-versus-integrate comparator; no selection or
  rejection

---

## V. Required Acceptance Criteria

Before any candidate or composition can be adopted, it must demonstrate:

1. Memory remains the sole durable user-knowledge owner.
2. Working context remains ephemeral unless separately proposed and authorized.
3. `proposeWrite`, retrieve, explain, redact, and forget map to existing
   contracts without native-API bypass.
4. Exact point-of-use `memory.read`, `memory.write`, and `memory.forget`
   authorization and workspace scope validation.
5. Stable canonical item/revision identity and complete source/derivation
   provenance.
6. Purpose/scope filtering before candidate content crosses a boundary.
7. Reproducible lexical, dense, temporal, graph, hybrid, and reranking quality by
   declared query class.
8. Truthful retrieval explanation with sources, filters, ranks, fusion, graph
   paths, transformations, and uncertainty.
9. Correction, contradiction, supersession, valid time, and recorded time remain
   distinguishable.
10. Compression and consolidation preserve citations, disclose lossiness, and
    cannot silently replace source evidence.
11. Retention/pruning policy remains separate from ranking and explicit
    forgetting.
12. A reverse derivation manifest enumerates every affected projection.
13. Redaction and forgetting immediately block retrieval and deterministically
    purge/regenerate indexes, summaries, graphs, caches, and attachments.
14. Completion is verified after crash/restart and stale-index scenarios.
15. The deletion claim distinguishes logical, cryptographic, replica, backup,
    and physical-media guarantees.
16. Canonical state remains available when derived indexes fail or rebuild.
17. All core paths work with the network unavailable and no silent cloud/model
    fallback.
18. Encryption, key custody, backup, recovery, temporary-file, crash-dump, and
    key-destruction behavior are demonstrated.
19. Cross-user/workspace isolation, poisoning, injection, malicious index,
    corruption, rollback, and resource-exhaustion tests pass.
20. Windows packaging, signing, standard-user, sleep/resume, update, rollback,
    antivirus, filesystem, and shutdown evidence exists.
21. Performance, resource, storage growth, rebuild, deletion, and migration are
    measured on declared hardware and workloads.
22. Complete derived-index rebuild is proven. If Memory export administration
    is accepted, technology-neutral export is also proven.
23. Storage, index, model, encryption, and schema migrations preserve authority,
    provenance, deletion barriers, and explanation meaning.
24. Optional synchronization, if in scope, preserves E2EE, authorization,
    provenance, semantic conflicts, forget barriers, retired-device policy, and
    honest residual uncertainty.
25. Exact-component and transitive licensing is acceptable.
26. Security, maintenance, governance, and advisory risks have owners and
    controls.
27. Every native library, process, model, graph, and synchronization boundary
    fits the Complexity Budget.
28. Build-versus-integrate reasoning explicitly satisfies ADR-0002.
29. Relevant architecture unknowns, especially merge/reorganization visibility
    and deletion semantics, are resolved or accepted by a named authority.

### Evaluation criteria for future technology selection

Mandatory gates precede preferences. Surviving candidates should be compared on:

- contract and ownership fit
- provenance and derivation completeness
- permission and scope isolation
- retrieval quality by query and memory class
- temporal, contradiction, and correction fidelity
- explainability and source faithfulness
- exact redaction/forget coverage and verification
- canonical durability and index rebuildability
- Local First completeness
- privacy, encryption, key, backup, and sync behavior
- Windows, Rust, Tauri, and packaging fit
- measured latency, resources, storage amplification, and background impact
- crash, corruption, rollback, stale-index, and recovery behavior
- migration, export, replaceability, and lock-in
- maintenance, security response, and governance
- licence and distribution obligations
- process, native, model, and operational complexity
- total lifecycle ownership cost

No aggregate score may override a failed mandatory gate.

---

## W. Decision and Review

- Decision: no technology selected or rejected
- Selected scope: none
- Decision rationale: this research establishes findings, categories, current
  comparators, trade-offs, unknowns, and acceptance criteria only
- Rejected candidates: none
- Conditions before selection:
  - resolve architecture questions that materially change taxonomy,
    merge/reorganization scope, retention, deletion, synchronization, and
    operation semantics
  - define representative Windows hardware, data, query, and failure workloads
  - run bounded reproducible evaluation against mandatory acceptance criteria
  - validate exact versions, models, binary packaging, and transitive
    dependencies
- Remaining unknowns: listed in section C
- Required ADR: none for this research
- Open Source Registry action: none
- Engineering Ledger action: record completion of MEM-001 research
- Review date: 2027-02-01, or earlier on a trigger below
- Event-driven re-evaluation triggers:
  - Memory architecture gaps or retention/deletion/sync requirements are resolved
  - material candidate release, deprecation, advisory, licence, or governance
    change
  - a local model, retrieval, deletion, encryption, or sync assumption becomes
    reproducibly testable
  - measured Windows evidence invalidates a finding
  - scale or hardware requirements materially change
- Approver: pending architecture review

---

## Architectural Trade-offs

- A single embedded engine simplifies transactions, backup, and lifecycle but
  increases engine and format coupling.
- Separate best-of-breed indexes improve specialization and replacement but add
  dual-write, reconciliation, deletion, backup, and migration complexity.
- Sidecars improve process isolation and independent scaling but add local API,
  authentication, packaging, health, logs, and shutdown obligations.
- Sparse retrieval is transparent and exact-term strong; dense retrieval handles
  paraphrase but is model-dependent and less interpretable.
- Hybrid retrieval improves coverage but adds tuning, latency, index lifecycle,
  and explanation surface.
- Graph relationships support multi-hop and temporal reasoning but magnify
  extraction errors and cascade-deletion complexity.
- Abstractive compression saves context but risks hallucination and semantic
  drift; extractive compression uses more tokens but preserves evidence.
- Append-only/revisioned history improves provenance and correction but conflicts
  with simple hard-deletion claims and increases storage.
- Aggressive pruning reduces growth but can remove rare, sensitive, or
  infrequently accessed knowledge that is still important.
- Whole-database encryption preserves query capability but creates broad key
  exposure; envelope encryption narrows exposure but complicates search,
  rotation, and recovery.
- CRDT synchronization improves offline convergence but does not resolve
  semantic truth, authorization, or complete forgetting.
- Long tombstone retention prevents stale resurrection but retains metadata and
  increases state; short retention weakens offline-device recovery.
- Detailed retrieval traces improve diagnosis but can expose sensitive candidate
  information; minimized reason records protect privacy but limit forensic depth.
- Rich AI-memory frameworks accelerate experimentation but can absorb authority,
  persistence, and orchestration unless strictly adapted behind contracts.

---

## Evidence Register

All external sources were accessed 2026-08-01.

### Storage, full-text, vector, and graph

- SQLite 3.53.4 release: https://sqlite.org/releaselog/3_53_4.html
- SQLite FTS5: https://sqlite.org/fts5.html
- SQLite backup API: https://sqlite.org/backup.html
- SQLite corruption guidance: https://sqlite.org/howtocorrupt.html
- SQLite `secure_delete`: https://sqlite.org/pragma.html#pragma_secure_delete
- SQLite `VACUUM`: https://sqlite.org/lang_vacuum.html
- Tantivy releases: https://github.com/quickwit-oss/tantivy/releases
- Tantivy update/delete example:
  https://tantivy-search.github.io/examples/deleting_updating_documents.html
- sqlite-vec releases: https://github.com/asg017/sqlite-vec/releases
- sqlite-vec KNN: https://alexgarcia.xyz/sqlite-vec/features/knn.html
- FAISS releases: https://github.com/facebookresearch/faiss/releases
- FAISS index I/O:
  https://github.com/facebookresearch/faiss/wiki/Index-IO,-cloning-and-hyper-parameter-tuning
- FAISS removal:
  https://github.com/facebookresearch/faiss/wiki/Special-operations-on-indexes
- USearch releases: https://github.com/unum-cloud/USearch/releases
- USearch documentation: https://unum-cloud.github.io/USearch/
- LanceDB releases: https://github.com/lancedb/lancedb/releases
- LanceDB tables and versioning: https://docs.lancedb.com/tables
- LanceDB updates/deletion: https://docs.lancedb.com/tables/update
- Qdrant releases: https://github.com/qdrant/qdrant/releases
- Qdrant storage: https://qdrant.tech/documentation/manage-data/storage/
- Qdrant snapshots: https://qdrant.tech/documentation/snapshots/
- Qdrant security: https://qdrant.tech/documentation/security/
- Meilisearch releases: https://github.com/meilisearch/meilisearch/releases
- Meilisearch backups:
  https://www.meilisearch.com/docs/resources/self_hosting/data_backup/overview
- Chroma releases: https://github.com/chroma-core/chroma/releases
- Chroma clients: https://docs.trychroma.com/docs/run-chroma/clients
- Chroma deletion: https://docs.trychroma.com/docs/collections/delete-data
- Chroma advisory: https://github.com/advisories/GHSA-f4j7-r4q5-qw2c
- LadybugDB repository: https://github.com/LadybugDB/ladybug
- LadybugDB releases: https://github.com/LadybugDB/ladybug/releases
- LadybugDB files: https://docs.ladybugdb.com/developer-guide/files/
- Archived Kuzu repository: https://github.com/kuzudb/kuzu

### AI-memory and retrieval frameworks

- Mem0 repository/releases: https://github.com/mem0ai/mem0
- Mem0 architecture: https://docs.mem0.ai/core-concepts/how-it-works
- Mem0 open-source configuration:
  https://docs.mem0.ai/open-source/configuration
- Mem0 deletion: https://docs.mem0.ai/core-concepts/memory-operations/delete
- Letta legacy server: https://github.com/letta-ai/letta
- Letta Code: https://github.com/letta-ai/letta-code
- Letta memory blocks:
  https://docs.letta.com/guides/core-concepts/memory/memory-blocks/index.md
- LangGraph releases: https://github.com/langchain-ai/langgraph/releases
- LangGraph persistence:
  https://docs.langchain.com/oss/python/langgraph/persistence
- LangGraph stores: https://docs.langchain.com/oss/python/langgraph/stores
- Graphiti repository/releases: https://github.com/getzep/graphiti
- Graphiti episodes:
  https://help.getzep.com/graphiti/core-concepts/adding-episodes.mdx
- Graphiti deletion: https://help.getzep.com/deleting-data-from-the-graph
- Zep Community Edition status: https://github.com/getzep/zep
- LlamaIndex releases: https://github.com/run-llama/llama_index/releases
- LlamaIndex memory:
  https://docs.llamaindex.ai/en/stable/examples/memory/custom_memory/
- Haystack releases: https://github.com/deepset-ai/haystack/releases
- Haystack document stores:
  https://docs.haystack.deepset.ai/docs/document-store

### Retrieval, memory, compression, and evaluation research

- MemGPT: https://arxiv.org/abs/2310.08560
- Generative Agents: https://arxiv.org/abs/2304.03442
- CoALA: https://arxiv.org/abs/2309.02427
- Graphiti/Zep paper: https://arxiv.org/abs/2501.13956
- HippoRAG: https://arxiv.org/abs/2405.14831
- LongMemEval: https://arxiv.org/abs/2410.10813
- LongMemEval-V2: https://arxiv.org/abs/2605.12493
- LoCoMo: https://arxiv.org/abs/2402.17753
- Dense Passage Retrieval: https://arxiv.org/abs/2004.04906
- ColBERT: https://arxiv.org/abs/2004.12832
- BEIR: https://arxiv.org/abs/2104.08663
- Reciprocal Rank Fusion: https://doi.org/10.1145/1571941.1572114
- RECOMP: https://arxiv.org/abs/2310.04408
- LongLLMLingua: https://arxiv.org/abs/2310.06839
- MemoryBank: https://arxiv.org/abs/2305.10250
- MEM1: https://arxiv.org/abs/2506.15841
- RAGAS: https://arxiv.org/abs/2309.15217

### Encryption, deletion, and Windows protection

- SQLCipher 4.17.0 release:
  https://www.zetetic.net/blog/2026/07/08/sqlcipher-4-17-0-release/
- SQLCipher design: https://www.zetetic.net/sqlcipher/design/
- SQLCipher licence: https://www.zetetic.net/sqlcipher/license/
- Windows DPAPI:
  https://learn.microsoft.com/en-us/windows/win32/api/dpapi/nf-dpapi-cryptprotectdata
- Windows CNG key storage:
  https://learn.microsoft.com/en-us/windows/win32/seccng/key-storage-and-retrieval
- Windows BitLocker countermeasures:
  https://learn.microsoft.com/en-us/windows/security/operating-system-security/data-protection/bitlocker/countermeasures
- Windows VSS:
  https://learn.microsoft.com/en-us/windows/win32/vss/volume-shadow-copy-service-overview
- NIST SP 800-88 Rev. 2:
  https://csrc.nist.gov/pubs/sp/800/88/r2/final
- libsodium releases: https://github.com/jedisct1/libsodium/releases
- libsodium secretstream:
  https://doc.libsodium.org/doc/secret-key_cryptography/secretstream

### Synchronization and conflict patterns

- Automerge repository: https://github.com/automerge/automerge
- Automerge conflicts:
  https://automerge.org/docs/reference/documents/conflicts/
- Yjs repository: https://github.com/yjs/yjs
- Yjs internals: https://github.com/yjs/yjs/blob/main/INTERNALS.md
- SQLite Session extension: https://sqlite.org/session.html
- libSQL repository: https://github.com/tursodatabase/libsql
- Turso Database repository: https://github.com/tursodatabase/turso
- Electric sync: https://electric-sql.com/docs/guides/sync
- PowerSync update conflicts:
  https://docs.powersync.com/handling-writes/handling-update-conflicts
- Syncthing releases: https://github.com/syncthing/syncthing/releases
- Signal Sesame: https://signal.org/docs/specifications/sesame/
- Messaging Layer Security: https://www.rfc-editor.org/rfc/rfc9420.html
- Hybrid Public Key Encryption: https://www.rfc-editor.org/rfc/rfc9180.html

---

## Validation Result

- The record follows every canonical framework section and the Memory research
  profile.
- Findings cover working and long-term memory, structured storage, vector,
  lexical, graph, hybrid retrieval, compression, pruning, versioning, privacy,
  encryption, synchronization, and explainability.
- Mature open-source approaches are compared by category and current version
  without adoption or rejection.
- Facts, architectural implications, performance evidence gaps, trade-offs, and
  unresolved policy/architecture questions are distinguished.
- Research maps to Memory ownership, contracts, interactions, acceptance cases,
  Local First, Privacy First, Permission First, explainability, Windows,
  licensing, maintenance, migration, and Complexity Budget requirements.
- No runtime code, schema, model, database, process boundary, synchronization
  protocol, technology selection, technology rejection, ADR, Open Source
  Registry approval, capability ownership, interaction path, or contract meaning
  changed.
