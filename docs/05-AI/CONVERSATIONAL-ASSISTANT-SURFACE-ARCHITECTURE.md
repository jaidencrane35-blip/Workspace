# Conversational / Assistant Surface Architecture (Programme IV — Batch 11)

**Status:** Active — implemented
**Audience:** Architecture, Kernel, Frontend, Product, Governance  
**Depends on:**  
- [Programme IV Interaction Runtime](./PROGRAMME-IV-INTERACTION-RUNTIME.md)  
- [Workspace Evidence Observational Scaffold](./WORKSPACE-EVIDENCE-OBSERVATIONAL-SCAFFOLD-ARCHITECTURE.md) (Batch 10 — accepted)  
- Programme II (Cognitive Workspace) and Programme III (Coherent Workspace Runtime)  
- [Programme IV Maintainability Audit](../03-Engineering/PROGRAMME-IV-MAINTAINABILITY-AUDIT.md)

## Purpose

Batches 1–9 built a mature **evidence substrate**. Batch 10 extracted shared observational scaffolding.

Batch 11 asks the next architectural question:

> **"How do humans converse with the intelligence substrate without the assistant becoming a hidden authority?"**

This charter defines the **Conversational / Assistant Surface Contract** — a commercially maintainable human interface layer that presents retrieval, explanation, navigation, and context **without** deciding, approving, executing, or silently mutating workspace state.

---

## Primary principle

**Present intelligence. Never become authority.**

The assistant surface:

- **talks about** what Programmes II–IV already know
- **never decides** what should happen next
- **never acts** on the workspace without an existing governed command path owned elsewhere

Capability growth must not equal code duplication (Batch 10 direction lock). Assistant features must evaluate shared Programme IV contracts first, then add only ownership-specific surface code.

---

## Status of this document

| State | Meaning |
|---|---|
| **Active — implemented** | `WorkspaceAssistantSurfaceService`, migration `073`, kernel commands, projection helpers, and governance guard are implemented |
| **Interaction layer** | Presents upstream recorded evidence only; never becomes an evidence assessment engine |
| **Governed baseline** | Mutation baseline **83**; history/projection DTO inventory **34** |

---

## Ownership

### Owner

`WorkspaceAssistantSurfaceService`

### Owns

| Owns | Meaning |
|---|---|
| Conversation session projections | Human-visible turn/history packages — evidence-shaped, dual-channel where durable |
| Assistant utterance packages | Structured responses composed from upstream projections |
| Surface composition metadata | Which upstream packages were consulted for a turn |
| Presentation lineage | Provenance refs to Programme II/III/IV artefacts used in a response |
| Assistant surface diagnostics | Completeness / unavailable upstream / missing context — diagnostic only |
| UI projection contracts | How React may render assistant turns without command affordances |

### Does not own

| Does not own | Remains owned by |
|---|---|
| Semantic retrieval | Programme IV Batch 1 Semantic Query |
| Evidence navigation / trace / coverage / consistency / dependency / freshness / completeness / reliability | Programme IV Batches 2–9 |
| Explanations as synthesised packages | Programme III Explanation layer |
| Contextual understanding / knowledge / hub / decision support | Programme III |
| Cognitive model, planning, reasoning, orchestration, learning, agent cast, autonomy | Programme II |
| Intent, Decision Engine, Recommendation Engine, Decision Queue | Existing cognition / decision authorities |
| Permissions / capability grants | PermissionGateway |
| Execution / lifecycle | ExecutionLifecycle / CommandPipeline-backed lifecycle owners |
| Durable workspace state mutations | Workspace State / Intent / Work Context owners |
| Hidden memory / second ontology | Forbidden — no new memory SoT |

---

## Interaction with Programmes II, III, and IV

```text
Human UI (projection-only)
        │
        ▼
Assistant Surface (Batch 11)     ← presents / composes / cites
        │
        ├── load_snapshot / query paths only
        ▼
Programme IV evidence substrate  ← what do we already know?
Programme III coherent packages  ← situational / explanatory packages
Programme II cognitive artefacts ← plans / graphs / memories as evidence only
        │
        ▼
Existing CommandPipeline + PermissionGateway   ← only path for actions
```

### Access rules

1. **Upstream reads:** `load_snapshot` (and existing read/query commands) only.
2. **Never** call foreign `::generate` to “refresh” context for a nicer answer.
3. **Never** invent matches, lineage, or situational facts when upstreams are unavailable.
4. **Actions** the human may take from the UI must invoke **existing** mutation commands owned by other services — the assistant surface does not gain new execution authority by packaging text.

### Composition model (human-facing synthesis)

Allowed synthesis is **presentation composition**, not truth creation:

| Allowed | Forbidden |
|---|---|
| Ordering and grouping retrieved packages for readability | Asserting new facts not present upstream |
| Quoting / summarising cited evidence with provenance | Filling gaps with plausible narrative |
| Stating upstream completeness/freshness/reliability as recorded | Claiming confidence as truth |
| Saying “unavailable / unknown / partial” when packages missing | Inferring missing intent or decisions |

---

## Interaction model

The assistant surface supports five human interaction modes — all informational unless the human separately triggers a governed command elsewhere.

### 1. Retrieval

- Route natural-language or structured asks into Programme IV Semantic Query (and related evidence engines as scoped).
- Return matches, gaps, and lineage as already defined by those engines.
- Do not score “best action.”

### 2. Explanation

- Present Programme III explanation packages and related diagnostics.
- Keep limitations and uncertainty visible.
- Do not escalate explanation into recommendation or approval language.

### 3. Navigation

- Expose Programme IV evidence navigation / trace paths for “show me how this connects.”
- Do not invent edges or repair broken lineage.

### 4. Context presentation

- Display active workspace situational packages (state, continuity, contextual understanding, hub aggregates) as **read-only projections**.
- Label freshness / completeness / reliability when those snapshots exist.
- Never treat presented context as an implicit grant to act.

### 5. Human-facing synthesis

- Compose a single conversational turn from cited upstream packages.
- Every synthesised claim must be attachable to provenance refs.
- If provenance is missing, the turn must say so — Unknown remains unknown.

---

## Authority boundaries

The assistant surface **must never**:

| Forbidden | Why |
|---|---|
| Make decisions | Decision Engine / human Decision Queue own decisions |
| Approve actions | PermissionGateway / approval flows own consent |
| Execute operations | ExecutionLifecycle + CommandPipeline own execution |
| Bypass PermissionGateway | Sole `require()` authority |
| Replace Intent | Intent / Work Context remain durable intent owners |
| Replace Decision Engine | Candidates and acceptance remain DE-owned |
| Replace Recommendation systems | Suggestions remain recommendation-owned |
| Create hidden memory authority | No second memory SoT or silent long-term store |
| Silently mutate workspace state | No background writes from “helpful” turns |

**Authority effect:** all assistant artefacts use `authority_effect: "none"` and `actionable: false` unless a *separate*, existing mutation command is explicitly invoked by the human through CommandPipeline.

---

## Memory boundary

| Kind | Display | Reference in turn | Persist | Classification |
|---|---|---|---|---|
| Upstream evidence snapshots | Yes | Yes, with provenance | Already persisted by owners | Evidence only |
| Conversation turn projections | Yes | Yes | Optional dual-channel history if accepted later | Evidence only — never commandable |
| Session working buffer (in-memory) | Yes (current session) | Yes | Must not silently become durable memory | Ephemeral presentation |
| “Assistant learned preference” | Only if sourced from existing preference owners | Only with provenance | Must use existing preference/persistence owners | Never invent |
| Fabricated recall | No | No | No | Forbidden |

### Rules

1. **Displayed context** ≠ **durable memory**.
2. Anything that survives restart must use an **existing** persistence owner or a future accepted dual-channel assistant history that remains non-actionable evidence.
3. Referencing context in a turn requires a provenance ref or an explicit “unavailable” gap.
4. The assistant must not create a parallel memory graph, vector SoT, or private scratch DB that other layers cannot audit.

---

## UI boundary

### Projection-only requirements

- React renders assistant packages via projection helpers / contracts (extend Batch 10 `evidenceProjectionContract` patterns where applicable — do not clone a new family without cause).
- No hidden `invoke` / mutation from render paths.
- History and prior turns are evidence-shaped (`actionable: false`).

### No hidden command affordances

- Buttons that mutate must be clearly labeled as **existing product commands** (e.g. “Open Decision Queue item”) and route through known CommandPipeline commands.
- Chat send must not imply execution; it requests a **projection refresh / turn composition** only.
- Do not style informational chips as primary action CTAs.

### Information vs action

| Information (assistant-owned) | Action (other owners) |
|---|---|
| “Here is what evidence says…” | “Approve contract” |
| “Coverage is partial…” | “Create task” |
| “This recommendation candidate exists…” | “Accept recommendation” via Recommendation lifecycle |

UI copy must not collapse these into “the assistant will handle it.”

---

## Governance

### Permissions

- Read/composition paths: `work_context.read` (or narrower read capabilities if introduced later).
- Any mutation triggered from the UI: existing command capabilities (`work_context.write`, approval subjects, etc.) — **not** a new “assistant.superuser” capability.
- No capability grants issued by the assistant surface.

### Audit expectations

- Record observational events for turn composition (e.g. `workspace.assistant.turn.composed`) with workspace id, upstream package refs, and `authority_effect: none`.
- Do not audit turns as if they were executions or approvals.
- Audit must remain append-only evidence.

### Provenance requirements

- Every assistant turn package includes lineage to upstream snapshot ids / revisions consulted.
- Missing upstreams produce diagnostics/gaps — never silent omission of uncertainty.

### Explainability requirements

- Users must be able to see **why** a statement appeared (cited packages).
- Uncertainty, partiality, staleness, and unreliability must remain visible when upstreams report them.
- Explainability never becomes a decision or trust directive.

---

## Forbidden behaviour (explicit)

The assistant surface is prohibited from:

1. **Autonomous execution** — no background runs of tools, launches, or lifecycle transitions.
2. **Silent action planning** — no hidden plan objects that imply upcoming execution.
3. **Invented context** — no fabricated files, tasks, decisions, or memories.
4. **Unsupported reasoning** — no causal/normative conclusions beyond cited observational packages.
5. **Hidden state mutation** — no writes outside explicit CommandPipeline mutations initiated as such.
6. **Authority escalation** — no self-granted capabilities, no bypass of PermissionGateway, no overlay that makes history actionable.
7. **Clone-wave architecture** — no new parallel engine/repo/guard/projection stack when Batch 10 contracts suffice; justify any new file against the maintainability audit.

---

## Maintainability constraints applied (Batch 10 lock)

Batch 11 implementation follows these constraints:

1. Upstream Programme IV access uses existing `load_snapshot` paths + Batch 10 contracts.
2. React projection logic extends shared `evidenceProjectionContract.ts`.
3. Migration `073` is justified by durable dual-channel non-actionable conversation evidence.
4. [Programme IV Maintainability Audit](../03-Engineering/PROGRAMME-IV-MAINTAINABILITY-AUDIT.md) records Batch 11 extraction vs deferral decisions.

---

## Implementation confirmation

Batch 11 implementation confirms:

- [x] Ownership / non-ownership tables are unambiguous
- [x] Programme II/III/IV interaction rules are correct
- [x] Authority, memory, UI, and governance boundaries are enforceable
- [x] Forbidden behaviour list is complete for commercial due diligence
- [x] Implementation does not create a hidden assistant SoT
- [x] Status header updated to **Active — implemented**

---

## Out of scope for Batch 11

- Model provider selection / prompt engineering details
- Product UX layouts beyond projection/action distinction
- Resolving case5 / case11 (non–Programme IV debt)
- Context selection / continuity packaging (see Batch 12 — implemented)

---

## Next

Batch 12 — [Assistant Context Intelligence Architecture](./ASSISTANT-CONTEXT-INTELLIGENCE-ARCHITECTURE.md) (**implemented** — context packaging / continuity; does not replace this surface).

---

## Related documents

- [Programme IV Interaction Runtime](./PROGRAMME-IV-INTERACTION-RUNTIME.md)
- [Workspace Evidence Observational Scaffold](./WORKSPACE-EVIDENCE-OBSERVATIONAL-SCAFFOLD-ARCHITECTURE.md)
- [Assistant Context Intelligence Architecture](./ASSISTANT-CONTEXT-INTELLIGENCE-ARCHITECTURE.md)
- [Workspace Semantic Query Architecture](./WORKSPACE-SEMANTIC-QUERY-ARCHITECTURE.md)
- [Workspace Explanation Layer Architecture](./WORKSPACE-EXPLANATION-LAYER-ARCHITECTURE.md)
- [Workspace Vocabulary](./WORKSPACE-VOCABULARY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Programme IV Maintainability Audit](../03-Engineering/PROGRAMME-IV-MAINTAINABILITY-AUDIT.md)

---

## Explicit confirmation

> The Conversational / Assistant Surface presents Programme II–IV intelligence to humans.
> It never makes decisions, never approves, never executes, never bypasses PermissionGateway,
> never replaces Intent / Decision / Recommendation authorities, never creates hidden memory,
> and never silently mutates workspace state.
