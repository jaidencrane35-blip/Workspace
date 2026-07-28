# Programme II — Cognitive Workspace

| Field | Value |
|-------|-------|
| **Purpose** | Evolve Workspace from object/lifecycle/evidence awareness into a governed cognitive system |
| **Status** | Active — Batch 3 in progress |
| **Depends on** | Programme I governance, operational recovery, Intent/Task Graph, RE/DE/DQ, Planning foundations |
| **Non-goals** | Embeddings/vector RAG as architectural memory; silent autonomous mutation; duplicate lifecycle systems |

## Programme principles (non-negotiable)

1. **No duplicate lifecycle systems** — RE/DE/DQ/execution/intent overlays remain sole lifecycle authorities for their domains.
2. **No new persistence authorities** — repositories stay transition/immutability guards only.
3. **No CommandPipeline bypass** — cognitive mutations route as governed commands.
4. **No PermissionGateway bypass** — agents and planners are actors with capabilities, not privileged runtimes.
5. **Reasoning is evidence until a governed command is approved.**
6. **Learning proposes; it never silently mutates production state.**
7. **Planning is advisory until authorised.** Planning artefacts are permanently non-executing.
8. **Every autonomous action must be attributable, auditable, and replayable.**

## Batch map

| Batch | Theme | Role | Status |
|-------|-------|------|--------|
| **1** | Workspace Cognitive Model | Durable semantic layer | Done |
| **2** | Planning Architecture | Governed non-executing sequencing / assumptions / risks / gaps | Done |
| **3** | Reasoning Memory | Architectural memory of rationale, alternatives, reflection — evidence only | Active |
| **4** | Cognitive Graph | Traversable relationships across cognitive + existing durable entities | Planned |
| **5** | Orchestration Engine | Governed multi-stage workflows | Planned |
| **6** | Adaptive Learning | Evidence-producing learning; proposes only | Planned |
| **7** | Multi-Agent Architecture | Governed actors with empty-by-default capabilities | Planned |
| **8** | Autonomous Workspace | Observe→Understand→Plan→Ask→Execute→Learn — gated on 1–7 | Planned |

## Existing foundations (reuse, do not replace)

| Capability | Owner today | Programme II use |
|------------|-------------|------------------|
| WorkGoal / Project / Task | WorkspaceIntentService | Referenced by cognitive goals and reasoning links |
| Task Graph | TaskGraphService | Referenced by planning steps and reasoning evidence |
| Planning snapshots | WorkspacePlanningService | Reasoning cites active plans by reference |
| AiPlan / Orchestration | AiPlanning / AiOrchestration | Distinct from durable Cognitive Planning + Reasoning Memory |
| Memory entries | AiMemoryService | Reasoning may cite memory summaries by reference |
| RE / DE / DQ | Domain services | Reasoning cites candidates by reference; keep lifecycle ownership |

## Batch 3 summary

`WorkspaceReasoningMemoryService` owns reasoning history and reflection only.
It produces `ReasoningSnapshot` dual-channel projections. History is append-only
terminal evidence. React is projection-only. See
[Reasoning Memory Architecture](./REASONING-MEMORY-ARCHITECTURE.md).

## Sequencing rule

Ship Batch *N* only when Batch *N−1* has durable contracts, tests, and governance ownership entries. Batch 8 is gated on 1–7.

## Related

- [Workspace Cognitive Model](./WORKSPACE-COGNITIVE-MODEL.md) (Batch 1)
- [Planning Architecture](./PLANNING-ARCHITECTURE.md) (Batch 2)
- [Reasoning Memory Architecture](./REASONING-MEMORY-ARCHITECTURE.md) (Batch 3)
- [Workspace Vocabulary](./WORKSPACE-VOCABULARY.md)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
