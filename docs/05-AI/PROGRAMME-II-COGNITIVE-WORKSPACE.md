# Programme II — Cognitive Workspace

| Field | Value |
|-------|-------|
| **Purpose** | Evolve Workspace from object/lifecycle/evidence awareness into a governed cognitive system |
| **Status** | Active — Batch 6 in progress |
| **Depends on** | Programme I governance, operational recovery, Intent/Task Graph, RE/DE/DQ, Planning, Reasoning Memory, Cognitive Graph, Orchestration |
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
11. **Learning is observational meta-evidence** — adaptation is suggestion-only; evidence precedes change; no self-modifying authority.

## Batch map

| Batch | Theme | Role | Status |
|-------|-------|------|--------|
| **1** | Workspace Cognitive Model | Durable semantic layer | Done |
| **2** | Planning Architecture | Non-executing sequencing / assumptions / risks / gaps | Done |
| **3** | Reasoning Memory | Rationale / alternatives / reflection — evidence only | Done |
| **4** | Cognitive Graph | Cross-domain reference-only integration topology | Done |
| **5** | Orchestration Engine | Governed refresh ordering / staleness coordination | Done |
| **6** | Adaptive Learning | Evidence-producing learning; proposes only | Active |
| **7** | Multi-Agent Architecture | Governed actors with empty-by-default capabilities | Planned |
| **8** | Autonomous Workspace | Observe→Understand→Plan→Ask→Execute→Learn — gated on 1–7 | Planned |

## Batch 6 summary

`WorkspaceLearningAdaptationService` observes outcomes across the cognitive stack and
adjacent lifecycle surfaces, producing patterns, confidence evolution, effectiveness
signals, and adaptation suggestions. Dual-channel `LearningSnapshot`. History is
append-only terminal evidence. React is projection-only. Learning never auto-applies
adaptations or mutates foreign domains.
See [Learning & Adaptation Architecture](./LEARNING-ADAPTATION-ARCHITECTURE.md).

## Sequencing rule

Ship Batch *N* only when Batch *N−1* has durable contracts, tests, and governance ownership entries. Batch 8 is gated on 1–7. **Do not begin Batch 7 until Batch 6 is accepted.**

## Related

- [Workspace Cognitive Model](./WORKSPACE-COGNITIVE-MODEL.md) (Batch 1)
- [Planning Architecture](./PLANNING-ARCHITECTURE.md) (Batch 2)
- [Reasoning Memory Architecture](./REASONING-MEMORY-ARCHITECTURE.md) (Batch 3)
- [Cognitive Graph Architecture](./COGNITIVE-GRAPH-ARCHITECTURE.md) (Batch 4)
- [Cognitive Orchestration Architecture](./COGNITIVE-ORCHESTRATION-ARCHITECTURE.md) (Batch 5)
- [Learning & Adaptation Architecture](./LEARNING-ADAPTATION-ARCHITECTURE.md) (Batch 6)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
