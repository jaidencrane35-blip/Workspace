# Assistant Context Intelligence Architecture (Programme IV — Batch 12)

**Status:** Active — implemented  
**Audience:** Architecture, Kernel, Frontend, Product, Governance  
**Depends on:**  
- [Conversational / Assistant Surface Architecture](./CONVERSATIONAL-ASSISTANT-SURFACE-ARCHITECTURE.md) (Batch 11 — accepted / implemented)  
- [Workspace Evidence Observational Scaffold](./WORKSPACE-EVIDENCE-OBSERVATIONAL-SCAFFOLD-ARCHITECTURE.md) (Batch 10 — accepted)  
- [Programme IV Interaction Runtime](./PROGRAMME-IV-INTERACTION-RUNTIME.md)  
- Programme II (Cognitive Workspace) and Programme III (Coherent Workspace Runtime)  
- [Programme IV Maintainability Audit](../03-Engineering/PROGRAMME-IV-MAINTAINABILITY-AUDIT.md)

## Purpose

Batch 11 made the assistant a **window** into workspace intelligence: presentation, citation, and non-actionable turn composition over existing Programme II–IV projections.

Batch 12 asks the next architectural question:

> **"How does the assistant manage context selection, conversation continuity, and workspace awareness without becoming memory, reasoning authority, or autonomous planning?"**

This charter defines the **Assistant Context Intelligence Contract** — the rules for packaging temporary conversation context, selecting retrieval scope, and presenting workspace awareness **without** creating durable memory authority, fabricating continuity, or silently enriching missing information.

---

## Primary principle

**Select and package context. Never invent or own it.**

Assistant context intelligence:

- **packages** what Batch 11 may present
- **scopes** which recorded projections are consulted
- **preserves** gaps, uncertainty, and provenance
- **never** becomes Memory, Cognitive Model, Intent, Decision, Plan, or autonomous goal ownership

Capability growth must not equal code duplication (Batch 10 direction lock; Batch 11 Grade A acceptance). Batch 12 reuses:

1. Batch 11 assistant surface contracts (`workspace_assistant_surface`, `AssistantSurfaceScope`, turn/citation packages)
2. Batch 10 observational scaffolding (`workspace_evidence_contract`, `evidenceProjectionContract`)
3. Existing Programme II/III/IV `load_snapshot` paths

---

## Status of this document

| State | Meaning |
|---|---|
| **Active — implemented** | `WorkspaceAssistantContextService`, migration `074`, kernel commands, projection helpers, and governance guard are implemented |
| **Depends on Batch 11** | Builds on accepted Assistant Surface — does not replace it |
| **Governed baseline** | Mutation baseline **84**; history/projection DTO inventory **35** |

---

## Relationship to Batch 11

```text
Batch 11 Assistant Surface     ← owns turn composition / utterance presentation
        │
        ▼
Batch 12 Context Intelligence  ← owns context packaging / scope / continuity display
        │
        ├── reuses Batch 11 packages where possible
        ├── load_snapshot only for upstreams
        ▼
Programme II–IV authorities    ← remain sole owners of truth, memory, intent, decisions
```

| Concern | Batch 11 | Batch 12 (this architecture) |
|---|---|---|
| Turn / utterance composition | Owns | Consumes / constrains inputs |
| Citation & lineage presentation | Owns | Ensures context packages carry provenance |
| Retrieval scope selection | Partial (`AssistantSurfaceScope` flags) | Owns explicit context-scope packaging |
| Conversation continuity packaging | Turn history as evidence | Owns session/context packaging rules |
| Durable memory | Forbidden | Still forbidden — may only *request* existing memory owners |
| Workspace awareness display | Present via snapshots | Owns which awareness slices are selected for a session |

Batch 12 must **not** fork a second assistant surface, second turn model, or second projection family when Batch 11 contracts can be extended.

---

## Ownership

### Owner

`WorkspaceAssistantContextService` in `packages/domain/src/workspace_assistant_context/` (clear folder for ownership clarification — not a clone of Batch 11 turn composition).

### May own

| Owns | Meaning |
|---|---|
| Conversation context packaging | Structured package of what the current session is “looking at” — evidence-shaped, non-actionable |
| Active session context | Dual-channel session projection: selected scope, referenced artefacts, continuity markers |
| Retrieval scope | Explicit include/exclude of upstream packages via reused `AssistantSurfaceScope` |
| Displayed context selection | Which recorded projections are shown as “in context” vs available-but-not-selected |
| Context diagnostics | Missing / unavailable / partial / stale selection reasons — diagnostic only |
| Continuity presentation | How prior turns and prior context packages are ordered for humans without inventing narrative glue |

### Must not own

| Does not own | Remains owned by |
|---|---|
| Durable memory authority | Existing Memory / preference / persistence owners |
| Workspace truth / SoT | Domain owners of artefacts |
| User Intent | Intent / Work Context |
| Decisions / approvals | Decision Engine / Decision Queue / PermissionGateway |
| Plans / goals / autonomous goals | Planning / Cognitive Model / Autonomy owners |
| Evidence assessment semantics | Programme IV Batches 1–9 |
| Turn utterance synthesis rules already owned by Batch 11 | `WorkspaceAssistantSurfaceService` |
| Execution / lifecycle / permissions | CommandPipeline / ExecutionLifecycle / PermissionGateway |
| Hidden second ontology or private context DB that other layers cannot audit | Forbidden |

---

## Context boundary

Four distinct layers — never collapsed:

| Layer | Lifetime | Persist? | Authority |
|---|---|---|---|
| **Temporary conversation context** | Session / turn | Dual-channel evidence only (`074_workspace_assistant_context.sql`) | Presentation packaging only |
| **Referenced workspace evidence** | As recorded by upstream owners | Already persisted by owners | Evidence only — cited with provenance |
| **Explicit durable memory requests** | Durable only via existing memory owners | Only through existing authorised persistence commands | Never written silently by assistant context |
| **Cognitive model / Intent / Decision state** | Owned elsewhere | Owned elsewhere | Read via `load_snapshot` only; never mutated here |

### Rules

1. **Displayed context ≠ durable memory.**
2. Continuity across turns may reference prior **assistant evidence** (Batch 11 history) and prior **context packages** — it must not invent missing turns or implied goals.
3. Promoting conversation context into durable memory requires an **explicit** human-initiated command owned by an existing memory/persistence authority — never a side effect of compose/select.
4. Every context package item must be one of: cited upstream ref, explicit gap, or prior non-actionable assistant evidence ref.

---

## Retrieval boundary

Assistant context **must**:

- Use existing evidence / intelligence contracts (`load_snapshot` and existing read/query commands)
- Preserve lineage and provenance refs on every included item
- Preserve uncertainty, partiality, freshness, completeness, and reliability as reported upstream
- Preserve missing information as gaps (Unknown remains unknown)
- Make retrieval scope explicit and auditable

Assistant context **must not**:

- Infer missing context to make answers feel complete
- Silently enrich context from uncited sources
- Fabricate continuity (“you were working on X”) without a provenance-backed package
- Call foreign `::generate` to refresh upstreams for nicer continuity
- Collapse multiple uncertain packages into a single confident situational claim
- Treat context selection as a decision, recommendation, or plan

---

## Memory boundary

| Kind | Display in context UI | Reference in context package | Persist via assistant context | Classification |
|---|---|---|---|---|
| Temporary conversation context | Yes | Yes | Dual-channel evidence only | Ephemeral packaging |
| Retrieved evidence | Yes | Yes, with provenance | Already owned upstream | Evidence only |
| Batch 11 turn history | Yes | Yes, as prior evidence | Owned by Batch 11 dual-channel rules | Non-actionable evidence |
| Durable memory artefacts | Yes if selected with provenance | Yes | Only via existing memory owners’ commands | Never invent |
| Cognitive model / Intent / Decision snapshots | Yes as read-only awareness | Yes with provenance | Never via assistant context | Read-only |
| Fabricated recall / implied goals | No | No | No | Forbidden |

### Explicit separation checklist (implementation must prove)

- [x] Conversation context struct ≠ memory write path
- [x] Retrieved evidence refs ≠ assistant-owned facts
- [x] Durable memory mutations only through existing authorised owners
- [x] Cognitive model state never written or “completed” by context packaging

---

## Continuity model (presentation only)

Allowed continuity:

| Allowed | Forbidden |
|---|---|
| Ordering prior turns / context packages by recorded timestamps or ids | Inventing turns that were never composed |
| Restating cited prior excerpts with provenance | Summarising into new goals or commitments |
| Saying “prior context package unavailable / superseded” | Filling gaps with plausible session narrative |
| Carrying forward explicit retrieval scope unless human changes it | Silently widening scope to include unrequested authorities |
| Showing which workspace artefacts remain selected | Implying the assistant “remembers” beyond recorded packages |

Continuity is **packaging of recorded packages**, not autobiographical memory.

---

## Authority boundaries

Assistant context intelligence **must never**:

| Forbidden | Why |
|---|---|
| Create durable memory authority | Memory owners remain sole SoT |
| Infer unsupported workspace awareness | Fabrication violates evidence honesty |
| Create hidden plans or autonomous goals | Planning / Autonomy owners |
| Present context selection as approved action | PermissionGateway / Decision Queue |
| Bypass PermissionGateway | Sole `require()` authority |
| Silently mutate workspace state | No background writes from context refresh |
| Replace Batch 11 surface ownership | Composition remains Batch 11 |
| Clone a parallel assistant subsystem | Batch 10/11 maintainability lock |

**Authority effect:** all context packages use `authority_effect: "none"` and `actionable: false`.

---

## UI boundary

### Projection-only

- React may render context packages via Batch 10/11 projection helpers (`assistantContextProjection.ts` thin-wraps `evidenceProjectionContract.ts`).
- No mutation from render paths.
- Context chips / panels are informational — not command affordances.

### Explicit information vs action

| Information (context-owned) | Action (other owners) |
|---|---|
| “These artefacts are in session context…” | “Save to memory” via existing memory command |
| “Retrieval scope excludes Decision Support…” | “Open Decision Queue item” via existing DQ command |
| “Prior context package superseded…” | “Compose new assistant turn” via Batch 11 command |

UI must not imply the assistant autonomously keeps or advances goals.

---

## Governance

### Permissions

- Context read / package inspection: `work_context.read`
- Context package mutation: `work_context.write` via `PackageWorkspaceAssistantContext`
- Durable memory writes: **existing** memory-owner commands only
- No capability grants issued by assistant context

### Audit expectations

- Observational events for context package compose/select (`workspace.assistant.context.packaged`) with workspace id, scope, upstream refs, `authority_effect: none`
- Never audit context packaging as execution, approval, or memory commit
- Append-only evidence only

### Provenance requirements

- Every selected context item carries artefact ref + origin domain + optional revision
- Unavailable upstreams produce diagnostics — never silent omission
- Continuity references must point at recorded package ids

### Explainability requirements

- Humans can see **why** an item is in context (selected / carried scope / prior evidence ref)
- Uncertainty and gaps remain visible
- Explainability never becomes a trust or action directive

---

## Implementation roots

| Layer | Path |
|---|---|
| Domain | `packages/domain/src/workspace_assistant_context/` |
| Kernel service | `packages/kernel/src/services/workspace_assistant_context.rs` |
| Commands | `PackageWorkspaceAssistantContext`, `GetWorkspaceAssistantContext`, `GetWorkspaceAssistantContextSummary`, `ExplainAssistantContext` |
| Migration | `packages/database/migrations/074_workspace_assistant_context.sql` |
| Repository | `packages/database/src/repositories/workspace_assistant_context.rs` |
| React | `app/src/components/assistantContextProjection.ts` |
| Governance | one `EVIDENCE_ENGINE_GUARD_SPECS` entry (Assistant context) |

---

## Forbidden behaviour (explicit)

1. **Silent durable memory** — no write of conversation context into a private long-lived store presented as “the assistant remembers.”
2. **Fabricated continuity** — no invented prior asks, goals, or workspace states.
3. **Unsupported enrichment** — no filling gaps with model-plausible context.
4. **Autonomous goal holding** — no persistent goals owned by the assistant context layer.
5. **Hidden planning** — no plan objects implied by context packaging.
6. **Authority escalation** — no actionable context, no PermissionGateway bypass.
7. **Clone-wave architecture** — no parallel Batch 11 surface + Batch 12 surface with duplicated contracts.

---

## Out of scope for Batch 12

- Model provider / prompt engineering details
- Product UX layouts beyond information vs action
- Resolving case5 / case11 (non–Programme IV debt)
- Replacing or rewriting Batch 11
- Retrieval request packaging / evidence presentation composition (see Batch 13 charter)

---

## Next

Batch 13 — [Assistant Retrieval Intelligence Architecture](./ASSISTANT-RETRIEVAL-INTELLIGENCE-ARCHITECTURE.md) (**charter only** — do not implement until accepted).

---

## Acceptance criteria

- [x] Ownership / non-ownership tables are unambiguous
- [x] Context vs memory vs evidence vs cognitive state separation is enforceable
- [x] Retrieval and continuity rules forbid inference and fabrication
- [x] Maintainability reuse path vs Batch 10/11 is explicit
- [x] Mutation baseline / DTO inventory updated for intentional implementation (84 / 35)
- [x] Status is **Active — implemented**

---

## Related documents

- [Conversational / Assistant Surface Architecture](./CONVERSATIONAL-ASSISTANT-SURFACE-ARCHITECTURE.md)
- [Assistant Retrieval Intelligence Architecture](./ASSISTANT-RETRIEVAL-INTELLIGENCE-ARCHITECTURE.md)
- [Workspace Evidence Observational Scaffold](./WORKSPACE-EVIDENCE-OBSERVATIONAL-SCAFFOLD-ARCHITECTURE.md)
- [Programme IV Interaction Runtime](./PROGRAMME-IV-INTERACTION-RUNTIME.md)
- [Workspace Vocabulary](./WORKSPACE-VOCABULARY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Programme IV Maintainability Audit](../03-Engineering/PROGRAMME-IV-MAINTAINABILITY-AUDIT.md)

---

## Explicit confirmation

> Assistant Context Intelligence packages and selects conversational context from recorded Programme II–IV intelligence.
> It never owns durable memory, never fabricates continuity, never infers missing context,
> never holds autonomous goals or plans, never decides or approves, never executes,
> never bypasses PermissionGateway, and never silently mutates workspace state.
