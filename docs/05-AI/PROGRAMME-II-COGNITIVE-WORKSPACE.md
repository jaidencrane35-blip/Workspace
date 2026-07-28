# Programme II — Cognitive Workspace

| Field | Value |
|-------|-------|
| **Purpose** | Evolve Workspace from object/lifecycle/evidence awareness into a governed cognitive system |
| **Status** | Complete — Batch 8 shipped (Programme II closed) |
| **Depends on** | Programme I governance, operational recovery, Intent/Task Graph, RE/DE/DQ, Planning, Reasoning Memory, Cognitive Graph, Orchestration, Learning, Agent Cast |
| **Non-goals** | Embeddings/vector RAG as architectural memory; silent autonomous mutation; duplicate lifecycle systems; autonomous execution |

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
11. **Learning is observational meta-evidence** — adaptation is suggestion-only; evidence precedes change; no self-modifying authority.
12. **Cognitive agents are role representations** — not users, not actors, not executors; evidence only; cannot self-authorise.
13. **Autonomy may suggest; authority must still approve** — confidence ≠ permission; proposals ≠ commands; execution remains external.

## Batch map

| Batch | Theme | Role | Status |
|-------|-------|------|--------|
| **1** | Workspace Cognitive Model | Durable semantic layer | Done |
| **2** | Planning Architecture | Non-executing sequencing / assumptions / risks / gaps | Done |
| **3** | Reasoning Memory | Rationale / alternatives / reflection — evidence only | Done |
| **4** | Cognitive Graph | Cross-domain reference-only integration topology | Done |
| **5** | Orchestration Engine | Governed refresh ordering / staleness coordination | Done |
| **6** | Adaptive Learning | Evidence-producing learning; proposes only | Done |
| **7** | Multi-Agent Architecture | Governed cognitive roles with empty authority | Done |
| **8** | Governed Cognitive Autonomy | Suggestion / opportunity layer — never executes | Done |

## Final stack

```
Cognitive Model
        ↓
Planning
        ↓
Reasoning Memory
        ↓
Cognitive Graph
        ↓
Orchestration
        ↓
Learning & Adaptation
        ↓
Agent Cast
        ↓
Governed Cognitive Autonomy
```

Autonomy is the top observational layer. It does not replace any prior architecture.

## Batch 8 summary

`WorkspaceCognitiveAutonomyService` evaluates cognitive, planning, reasoning, graph,
orchestration, learning, and agent-cast evidence to produce autonomy opportunities,
automation proposals, confidence assessments, safety constraints, and approval
requirements. Dual-channel `CognitiveAutonomySnapshot`. History is append-only
terminal evidence. Autonomy cannot execute, self-approve, grant permissions, or
mutate lifecycles. See [Cognitive Autonomy Architecture](./COGNITIVE-AUTONOMY-ARCHITECTURE.md).

## Programme complete

Programme II is complete when Batch 8 is implemented and audited. Do not invent
Batch 9 under this programme unless a new programme charter is opened.

## Related

- [Workspace Cognitive Model](./WORKSPACE-COGNITIVE-MODEL.md) (Batch 1)
- [Planning Architecture](./PLANNING-ARCHITECTURE.md) (Batch 2)
- [Reasoning Memory Architecture](./REASONING-MEMORY-ARCHITECTURE.md) (Batch 3)
- [Cognitive Graph Architecture](./COGNITIVE-GRAPH-ARCHITECTURE.md) (Batch 4)
- [Cognitive Orchestration Architecture](./COGNITIVE-ORCHESTRATION-ARCHITECTURE.md) (Batch 5)
- [Learning & Adaptation Architecture](./LEARNING-ADAPTATION-ARCHITECTURE.md) (Batch 6)
- [Cognitive Agent Cast Architecture](./COGNITIVE-AGENT-CAST-ARCHITECTURE.md) (Batch 7)
- [Cognitive Autonomy Architecture](./COGNITIVE-AUTONOMY-ARCHITECTURE.md) (Batch 8)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
