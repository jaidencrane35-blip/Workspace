# Programme II — Cognitive Workspace

| Field | Value |
|-------|-------|
| **Purpose** | Evolve Workspace from object/lifecycle/evidence awareness into a governed cognitive system |
| **Status** | Active — Batch 5 in progress |
| **Depends on** | Programme I governance, operational recovery, Intent/Task Graph, RE/DE/DQ, Planning, Reasoning Memory, Cognitive Graph |
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
9. **The Cognitive Graph is topology only** — never a second source of truth or authority boundary.
10. **Orchestration is coordination only** — never a planner, lifecycle owner, autonomy layer, or executor.

## Batch map

| Batch | Theme | Role | Status |
|-------|-------|------|--------|
| **1** | Workspace Cognitive Model | Durable semantic layer | Done |
| **2** | Planning Architecture | Non-executing sequencing / assumptions / risks / gaps | Done |
| **3** | Reasoning Memory | Rationale / alternatives / reflection — evidence only | Done |
| **4** | Cognitive Graph | Cross-domain reference-only integration topology | Done |
| **5** | Orchestration Engine | Governed refresh ordering / staleness coordination | Active |
| **6** | Adaptive Learning | Evidence-producing learning; proposes only | Planned |
| **7** | Multi-Agent Architecture | Governed actors with empty-by-default capabilities | Planned |
| **8** | Autonomous Workspace | Observe→Understand→Plan→Ask→Execute→Learn — gated on 1–7 | Planned |

## Batch 5 summary

`WorkspaceCognitiveOrchestrationService` coordinates Cognitive Model, Planning,
Reasoning Memory, and Cognitive Graph. It builds dependency DAGs, describes ordered
refresh stages, and records staleness / blocked / skipped observations. Dual-channel
`WorkspaceOrchestrationSnapshot`. History is append-only terminal evidence. React is
projection-only. Orchestration never executes refreshes or owns foreign lifecycles.
See [Cognitive Orchestration Architecture](./COGNITIVE-ORCHESTRATION-ARCHITECTURE.md).

## Sequencing rule

Ship Batch *N* only when Batch *N−1* has durable contracts, tests, and governance ownership entries. Batch 8 is gated on 1–7. **Do not begin Batch 6 until Batch 5 is accepted.**

## Related

- [Workspace Cognitive Model](./WORKSPACE-COGNITIVE-MODEL.md) (Batch 1)
- [Planning Architecture](./PLANNING-ARCHITECTURE.md) (Batch 2)
- [Reasoning Memory Architecture](./REASONING-MEMORY-ARCHITECTURE.md) (Batch 3)
- [Cognitive Graph Architecture](./COGNITIVE-GRAPH-ARCHITECTURE.md) (Batch 4)
- [Cognitive Orchestration Architecture](./COGNITIVE-ORCHESTRATION-ARCHITECTURE.md) (Batch 5)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
