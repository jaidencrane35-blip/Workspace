# Programme II — Cognitive Workspace

| Field | Value |
|-------|-------|
| **Purpose** | Evolve Workspace from object/lifecycle/evidence awareness into a governed cognitive system |
| **Status** | Active — Batch 1 in progress |
| **Depends on** | Programme I governance, operational recovery, Intent/Task Graph, RE/DE/DQ, Planning foundations |
| **Non-goals** | Embeddings/vector RAG as architectural memory; silent autonomous mutation; duplicate lifecycle systems |

## Programme principles (non-negotiable)

1. **No duplicate lifecycle systems** — RE/DE/DQ/execution/intent overlays remain sole lifecycle authorities for their domains.
2. **No new persistence authorities** — repositories stay transition/immutability guards only.
3. **No CommandPipeline bypass** — cognitive mutations route as governed commands.
4. **No PermissionGateway bypass** — agents and planners are actors with capabilities, not privileged runtimes.
5. **Reasoning is evidence until a governed command is approved.**
6. **Learning proposes; it never silently mutates production state.**
7. **Planning is advisory until authorised.**
8. **Every autonomous action must be attributable, auditable, and replayable.**

## Batch map

| Batch | Theme | Role |
|-------|-------|------|
| **1** | Workspace Cognitive Model | Durable semantic layer: Goal/Objective/Initiative/Milestone/Context/Working Set/Dependency/Constraint/Risk/Opportunity + importance/confidence/uncertainty/focus |
| **2** | Planning Architecture | Governed Planner service: hierarchical plans, alternatives, scheduling, rollback/recovery plans |
| **3** | Reasoning Memory | Architectural memory (rationale, rejected alternatives, failures, patterns) — not embeddings |
| **4** | Cognitive Graph | Traversable relationships across cognitive + existing durable entities |
| **5** | Orchestration Engine | Governed multi-stage workflows (Goal→Plan→Permission→Execute→Observe→Evaluate→Learn) |
| **6** | Adaptive Learning | Evidence-producing learning; proposes only |
| **7** | Multi-Agent Architecture | Planner/Researcher/Reviewer/Auditor/Operator/Recovery/Safety as governed actors |
| **8** | Autonomous Workspace | Observe→Understand→Plan→Ask→Execute→Learn loop — only after 1–7 |

## Existing foundations (reuse, do not replace)

| Capability | Owner today | Programme II use |
|------------|-------------|------------------|
| WorkGoal / Project / Task | WorkspaceIntentService | Goal references WorkGoal; Intent remains work SoT |
| Task Graph deps | TaskGraphService | Cognitive Dependency may *reference* task edges; does not own them |
| AiPlan / Orchestration | AiPlanning / AiOrchestration | Batch 2 Planner consumes cognitive model; does not absorb Intent |
| Memory entries | AiMemoryService | Batch 3 adds typed reasoning memory *kinds* without replacing Memory |
| Activity / Attention / Purpose / Milestone Engines | Aggregators | Consume Cognitive Model; remain `authority_effect: none` aggregators |
| RE / DE / DQ | Domain services | Consume cognitive focus/importance; keep lifecycle ownership |
| Actors / Capabilities | Domain + Gateway | Batch 7 agents = actors with empty-by-default capabilities |

## Sequencing rule

Ship Batch *N* only when Batch *N−1* has durable contracts, tests, and governance ownership entries. Batch 8 is gated on 1–7.

## Related

- [Workspace Cognitive Model](./WORKSPACE-COGNITIVE-MODEL.md) (Batch 1)
- [Workspace Vocabulary](./WORKSPACE-VOCABULARY.md)
- [Intelligence Roadmap](./INTELLIGENCE-ROADMAP.md)
