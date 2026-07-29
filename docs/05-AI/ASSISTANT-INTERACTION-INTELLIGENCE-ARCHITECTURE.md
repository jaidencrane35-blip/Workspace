# Assistant Interaction Intelligence Architecture (Programme IV — Batch 15)

**Status:** Active — implemented  
**Audience:** Architecture, Kernel, Frontend, Product, Governance  
**Depends on:**  
- [Assistant Explanation Intelligence Architecture](./ASSISTANT-EXPLANATION-INTELLIGENCE-ARCHITECTURE.md) (Batch 14 — accepted / implemented)  
- [Assistant Retrieval Intelligence Architecture](./ASSISTANT-RETRIEVAL-INTELLIGENCE-ARCHITECTURE.md) (Batch 13 — accepted / implemented)  
- [Assistant Context Intelligence Architecture](./ASSISTANT-CONTEXT-INTELLIGENCE-ARCHITECTURE.md) (Batch 12 — accepted / implemented)  
- [Conversational / Assistant Surface Architecture](./CONVERSATIONAL-ASSISTANT-SURFACE-ARCHITECTURE.md) (Batch 11 — accepted / implemented)  
- [Workspace Evidence Observational Scaffold](./WORKSPACE-EVIDENCE-OBSERVATIONAL-SCAFFOLD-ARCHITECTURE.md) (Batch 10 — accepted)  
- [Programme IV Interaction Runtime](./PROGRAMME-IV-INTERACTION-RUNTIME.md)  
- [Programme IV Maintainability Audit](../03-Engineering/PROGRAMME-IV-MAINTAINABILITY-AUDIT.md)

## Purpose

Batches 11–14 built a stacked assistant capability:

```text
Surface → Context → Retrieval → Explanation
```

each remaining a presentation/composition layer over existing Programme II–IV intelligence.

Batch 15 answers:

> **"How does the assistant manage user interaction flow while remaining a presentation and coordination surface only?"**

This document defines the **Assistant Interaction Intelligence Contract** — conversation-flow packaging and response-composition routing across Batches 11–14 **without** becoming memory, identity, Intent, Decision, planning, recommendation, execution, or autonomous-agent authority.

---

## Primary principle

**Coordinate interaction flow. Never act for the user.**

Assistant interaction intelligence:

- **packages** conversation/session interaction structure for humans
- **routes** which assistant capability package (surface / context / retrieval / explanation) is presented next
- **exposes** interaction diagnostics and provenance of what was consulted
- **never** becomes memory, identity, Intent authority, Decision Engine, planner, recommender, executor, or autonomous agent loop

Capability growth must not equal code duplication (Batch 10 direction lock; Batches 11–14 Grade A acceptance). Batch 15 reuses:

1. Batch 11 Assistant Surface (turn/utterance presentation)
2. Batch 12 Assistant Context (scope / continuity packaging)
3. Batch 13 Assistant Retrieval (retrieval packaging)
4. Batch 14 Assistant Explanation (clarity packaging)
5. Batch 10 observational scaffolding (`workspace_evidence_contract`, `evidenceProjectionContract`)
6. Existing workspace intelligence contracts via `load_snapshot` only

---

## Status of this document

| State | Meaning |
|---|---|
| **Active — implemented** | Domain/kernel/database/React implementation accepted |
| **Depends on Batches 11–14** | Coordinates existing assistant packages — does not replace them |
| **Governed baseline** | Mutation baseline **87**; history/projection DTO inventory **38** |

---

## Relationship to prior batches

```text
Human interaction (UI)
        │
        ▼
Assistant Interaction (Batch 15)  ← flow packaging / response routing only
        │
        ├── Assistant Surface (11)
        ├── Assistant Context (12)
        ├── Assistant Retrieval (13)
        ├── Assistant Explanation (14)
        ├── load_snapshot only
        ▼
Existing Workspace intelligence contracts
Existing CommandPipeline + PermissionGateway   ← only path for actions
```

| Concern | Prior owner | Batch 15 |
|---|---|---|
| Turn / utterance composition | Batch 11 Surface | Routes/presents — does not redefine composition ownership |
| Context packaging / continuity | Batch 12 | Inputs / directories — does not redefine context ownership |
| Retrieval packaging | Batch 13 | Routes presentation — does not redefine retrieval |
| Explanation packaging | Batch 14 | Routes presentation — does not redefine explanation |
| Memory / identity / Intent | Existing owners | Forbidden for assistant interaction |
| Decisions / planning / recommendations / execution | Existing owners | Forbidden |
| Autonomous agent loops | Forbidden | Still forbidden |

Batch 15 must **not** fork a cognitive engine, hidden memory, or autonomous agent when Batches 11–14 can be coordinated.

---

## Ownership

### Owner

`WorkspaceAssistantInteractionService` — clear folder `packages/domain/src/workspace_assistant_interaction/` for ownership clarification beside Batches 11–14, **not** a cognitive engine / agent / memory system.

### May own

| Owns | Meaning |
|---|---|
| Conversation state packaging | Structured package of user-visible interaction state — evidence-shaped, non-actionable |
| Interaction session structure | Session/flow markers: which packages were presented, in what order, with what diagnostics |
| User-visible flow coordination | Explicit routing among surface / context / retrieval / explanation packages for display |
| Response composition routing | Which existing assistant package(s) contribute to the next human-visible response — routing only, not new intelligence |
| Interaction diagnostics | Missing capability package / unavailable upstream / incomplete flow — diagnostic only |

### Must not own

| Does not own | Remains owned by |
|---|---|
| Durable memory | Memory / preference owners |
| Identity | Identity / actor owners |
| Intent authority | Intent / Work Context |
| Decisions / approvals | Decision Engine / Decision Queue / PermissionGateway |
| Planning / goals | Planning / Cognitive Model owners |
| Recommendations | Recommendation Engine |
| Execution / lifecycle | ExecutionLifecycle / CommandPipeline |
| Autonomous behaviour / agent loops | Forbidden |
| Surface / context / retrieval / explanation ownership | Batches 11–14 |
| Workspace truth / evidence assessment | Domain / Programme IV evidence owners |

---

## Required reuse

### Must compose

| Contract | Role |
|---|---|
| Assistant Surface (Batch 11) | Turn presentation packages |
| Assistant Context (Batch 12) | Scope / continuity packages |
| Assistant Retrieval (Batch 13) | Retrieval packages |
| Assistant Explanation (Batch 14) | Explanation clarity packages |
| Existing Workspace intelligence contracts | Upstream via `load_snapshot` when needed for diagnostics only |
| Batch 10 scaffold | Digest / authority / projection helpers |

### Must not create

| Forbidden creation | Why |
|---|---|
| New cognitive engine | Programme II / other cognition owners |
| Hidden memory system | Memory owners; no silent durable interaction memory as SoT |
| Autonomous agent loop | No self-directed act-on-behalf-of-user behaviour |
| Decision layer | Decision Engine / Queue |
| Parallel assistant stack cloning 11–14 | Clone-wave anti-pattern |

### Access rules

1. Upstream reads: `load_snapshot` and existing read/query commands only.
2. Never call foreign `::generate` to refresh packages for nicer flow.
3. Never invent prior conversation commitments or user goals.
4. Actions remain human-initiated through existing CommandPipeline commands owned elsewhere.

---

## Interaction boundary

### Allowed

| Allowed | Meaning |
|---|---|
| Organise conversation flow | Order/present recorded interaction packages |
| Present retrieved information | Route Batch 13 packages into the flow |
| Expose explanations | Route Batch 14 packages into the flow |
| Preserve provenance | Carry lineage/citations from contributing packages |
| Show interaction diagnostics | Unavailable / incomplete / scoped-out capability packages |

### Forbidden

| Forbidden | Why |
|---|---|
| Infer user goals silently | Intent / Cognitive Model owners; no hidden goal inference |
| Create hidden state | No private interaction SoT other layers cannot audit |
| Act on behalf of the user | No execution / approval / mutation authority |
| Make commitments | No promises that imply future autonomous action |
| Execute workflows | ExecutionLifecycle / CommandPipeline only |

### Flow vs agency

| Flow coordination (allowed) | Agency (forbidden) |
|---|---|
| “Next response presents retrieval package X + explanation package Y” | “I will handle this for you” |
| “Context package unavailable; flow continues with gaps” | Inferring missing Intent to keep flow smooth |
| “Human may open Decision Queue via existing command” | Creating or accepting decisions from interaction flow |

---

## Authority boundaries

Assistant interaction intelligence **must never**:

| Forbidden | Why |
|---|---|
| Own memory or identity | Existing owners |
| Own Intent | Intent / Work Context |
| Decide / approve / recommend | Existing authorities |
| Plan or hold autonomous goals | Planning / Autonomy owners |
| Execute or mutate silently | ExecutionLifecycle / CommandPipeline |
| Bypass PermissionGateway | Sole `require()` authority |
| Replace Batches 11–14 | Those remain package owners |
| Run autonomous agent loops | Forbidden behaviour |

**Authority effect:** all interaction packages use `authority_effect: "none"` and `actionable: false`.

---

## Implementation

| Layer | Location |
|---|---|
| Domain | `packages/domain/src/workspace_assistant_interaction/` |
| Kernel service | `packages/kernel/src/services/workspace_assistant_interaction.rs` |
| Commands | `PackageWorkspaceAssistantInteraction`, `GetWorkspaceAssistantInteraction`, `GetWorkspaceAssistantInteractionSummary`, `ExplainAssistantInteraction` |
| Migration | `packages/database/migrations/077_workspace_assistant_interaction.sql` |
| Repository | `packages/database/src/repositories/workspace_assistant_interaction.rs` |
| React | `app/src/components/assistantInteractionProjection.ts` |

### Maintainability decisions

1. **Ownership clarification folder** — `workspace_assistant_interaction/` clarifies ownership beside surface/context/retrieval/explanation; it is not a cognitive engine, agent, or memory system.
2. **Reuse** — `AssistantSurfaceScope::interaction_default()`, `workspace_evidence_contract`, thin React wrappers; one `EVIDENCE_ENGINE_GUARD_SPECS` entry.
3. **Compose via `load_snapshot` only** — Batches 11–14 assistant packages (Surface / Context / Retrieval / Explanation).
4. **Dual-channel migration `077`** with interaction-specific columns — not a clone of turn/context tables as “assistant memory”.

---

## Memory and state boundary

| Layer | Batch 15 role |
|---|---|
| Temporary conversation / interaction package | Packages for presentation; dual-channel evidence only |
| Batch 12 context | Input — not redefined |
| Batches 11/13/14 packages | Routed for presentation — not rewritten as agency |
| Durable memory / identity / Intent / Decision | Never written; never inferred as commitments |
| Hidden private interaction graph | Forbidden |

Displayed interaction state ≠ durable memory ≠ user Intent ≠ autonomous commitment.

---

## UI boundary

### Projection-only

- React renders interaction packages via Batch 10–14 projection helpers (extend; do not invent another parallel contract family without audit justification).
- No mutation from render paths.
- Flow controls that mutate must invoke **existing** product commands owned elsewhere — never imply the assistant acted.

### Information vs action

| Information (interaction-owned) | Action (other owners) |
|---|---|
| “Flow presents explanation package…” | “Approve contract” |
| “Retrieval package unavailable…” | “Compose retrieval” via existing Batch 13 command |
| “Decision Queue item exists (recorded)…” | “Open / accept” via Decision Queue commands |

UI must not imply the assistant will act, commit, or remember beyond recorded packages.

---

## Governance

### Permissions

- Interaction package read / compose inspection: `work_context.read` (or existing assistant read paths)
- Durable dual-channel package mutation: `work_context.write` — **not** `assistant.interaction.superuser`
- No capability grants issued by assistant interaction

### Audit expectations

- Observational events for interaction packaging/routing (`workspace.assistant.interaction.packaged`) with workspace id, routed package refs, `authority_effect: none`
- Never audit interaction packages as executions, approvals, commitments, or memory writes
- Append-only evidence only

### Provenance requirements

- Every routed contribution must cite a Batch 11–14 package id or an explicit gap
- Missing capability packages produce diagnostics — never silent omission
- Flow order is packaging metadata — not a plan of action

---

## Forbidden behaviour (explicit)

1. **Silent goal inference** — no inventing user Intent from flow.
2. **Hidden interaction memory** — no private durable SoT presented as “the assistant remembers.”
3. **Acting for the user** — no execution, approval, or mutation side effects from flow packaging.
4. **Commitments** — no promises of future autonomous work.
5. **Workflow execution** — no launching lifecycle/execution from interaction packages.
6. **Autonomous agent loops** — no self-directed multi-step agency.
7. **Clone-wave architecture** — no parallel Batches 11–14 stack wrapped as “interaction intelligence.”

---

## Out of scope for Batch 15

- Model provider / prompt engineering details
- Product UX layouts beyond information vs action
- Resolving case5 / case11 (non–Programme IV debt)
- Replacing Batches 11–14
- Building an autonomous agent framework
- Presentation preference packaging / personalisation boundary (see Batch 16 charter)

---

## Next

Batch 16 — [Assistant Personalisation Boundary Architecture](./ASSISTANT-PERSONALISATION-BOUNDARY-ARCHITECTURE.md) (**charter only** — do not implement until accepted).

---

## Acceptance criteria

- [x] Ownership / non-ownership tables are unambiguous
- [x] Reuse of Batches 11–14 (+ Batch 10 scaffold) is explicit
- [x] Flow vs agency rules forbid silent goals, hidden state, commitments, and execution
- [x] Interaction diagnostics / provenance requirements are enforceable
- [x] Maintainability reuse path vs Batches 10–14 is explicit
- [x] Mutation baseline **87** / DTO inventory **38** for `PackageWorkspaceAssistantInteraction`
- [x] Status is **Active — implemented**

---

## Related documents

- [Assistant Explanation Intelligence Architecture](./ASSISTANT-EXPLANATION-INTELLIGENCE-ARCHITECTURE.md)
- [Assistant Personalisation Boundary Architecture](./ASSISTANT-PERSONALISATION-BOUNDARY-ARCHITECTURE.md)
- [Assistant Retrieval Intelligence Architecture](./ASSISTANT-RETRIEVAL-INTELLIGENCE-ARCHITECTURE.md)
- [Assistant Context Intelligence Architecture](./ASSISTANT-CONTEXT-INTELLIGENCE-ARCHITECTURE.md)
- [Conversational / Assistant Surface Architecture](./CONVERSATIONAL-ASSISTANT-SURFACE-ARCHITECTURE.md)
- [Workspace Evidence Observational Scaffold](./WORKSPACE-EVIDENCE-OBSERVATIONAL-SCAFFOLD-ARCHITECTURE.md)
- [Programme IV Interaction Runtime](./PROGRAMME-IV-INTERACTION-RUNTIME.md)
- [Workspace Vocabulary](./WORKSPACE-VOCABULARY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Programme IV Maintainability Audit](../03-Engineering/PROGRAMME-IV-MAINTAINABILITY-AUDIT.md)

---

## Explicit confirmation

> Assistant Interaction Intelligence coordinates user-visible conversation flow across Batches 11–14.
> It never owns memory or identity, never infers Intent, never decides or recommends,
> never plans or executes, never makes commitments, never runs self-directed agent loops,
> never bypasses PermissionGateway, and never silently mutates workspace state.
> Status: **Active — implemented**.
