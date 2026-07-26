# Workspace Intelligence Foundation

| Field | Value |
|-------|-------|
| **Purpose** | Give the Workspace a unified, read-only understanding of user work context (Sprints 68–69 / P4-B5) |
| **Owner** | Lead Software Engineer |
| **Dependencies** | Memory, Personalization, Planning, Evaluation, Orchestration, Assistant, Permission Architecture |
| **Update Process** | Update when intent model or aggregator contracts change |

---

## Permanent principle

```
Observation → Delta → WorkspaceStateEngine → WorkspaceState
                                              ↓
                         WorkspaceEnvironmentService
                                              ↓
                         WorkspaceIntelligenceService (understanding)
                                              ↓
                         Future consumers / Assistant / Recommendations
```

The AI is not the product. The Workspace is the product.
Intelligence is informational. Authority remains at the Permission Gateway only.

`WorkspaceIntelligenceService` loads **one** `WorkspaceState` per generate cycle (Sprint 120) and passes it into Environment — it does not read observation snapshots or platform capture DTOs directly.

Runtime desktop truth is **WorkspaceState** only (Sprint 123 finalized IPC/UI surfaces).

**Attention vs Recommendations (Sprint 126):** Attention answers “what deserves focus?” with ranked items + structured `reasons`. The Recommendation Engine answers “what could the user do?”. Intelligence `recommended_actions` are Attention tops re-projected for Assistant display — not a merge of those systems and not automation.

**Attention consumption rules (Sprint 127):** Intelligence preserves Attention's ranking and reasons verbatim. `recommendations_from_attention` re-labels the top items for Assistant display in Attention order — it does not sort, re-score, or rewrite `reasons`. Enrichment (`enrich_with_recommendations`, `enrich_with_patterns`) adds new items and re-ranks the merged set through `WorkspaceAttentionState::from_items`, so ordering and id uniqueness stay Attention-owned. Enrichers that only append narrative (`enrich_with_milestones`, `_navigation`, `_work_context`, `_working_style`, `_transition`) must never touch scores. Downstream consumers read `priority` for banding — see the consumer rules in [Workspace Attention Engine](WORKSPACE-ATTENTION-ENGINE.md).

**Evaluation is not observation (Sprint 128):** one Intelligence generate writes roughly fifty audit events. Those reach the Activity Graph as `AuditSignal` entries, so cognitive consumers must window the timeline through `WorkspaceActivityGraph::cognitive_timeline()`, which excludes telemetry *before* taking the newest N. Filtering inside the loop instead let a single evaluation burst fill windows of 5, 8, and 20 entirely, changing the next evaluation's inputs. Telemetry remains fully audited and visible in the timeline — it simply never counts as a cognitive source fact. `recommended_actions` now carry the Attention item's `reasons` verbatim rather than dropping them at the Assistant boundary.

**Structured rationale downstream (Sprint 129):** rationale stays structured past Attention instead of collapsing into strings. `DecisionReason.attention_reason` carries the originating `AttentionReason` whole, and `RecommendationItem.attention_reasons` does the same for Attention-derived suggestions; both are empty or `None` when the reasoning is the consumer's own. Decision Engine previously rebuilt attention rationale from the first three `score_factors` strings, which described score math rather than why the item mattered, and discarded `reasons` entirely.

Each layer keeps its own question. Attention explains why something deserves focus; Decision Engine explains what should be considered next, adding goal, memory, personalization, plan, and approval reasons of its own; Experience explains how it is shown. Decision Engine contributes Attention's `score` to its own candidate total but never reorders or rescores Attention itself.

**Experience presentation contract (Sprint 131–132):** all user-facing surfaces resolve
`AttentionReason.explanation_key` through the Experience catalog — never raw keys,
never `score_factors` as rationale, never UI-invented wording. Canonical catalog:
`packages/kernel/resources/explanation-catalog.json` with sync verification
(`pnpm verify:explanation-catalog`). Operating State keeps
`score_factors` as arithmetic evidence only (valid diagnostic trail).

Rule: **UI does not interpret cognition. Experience translates cognition.**

---

## Intent model (durable, non-executable)

| Type | Role |
|------|------|
| `Project` | Durable container for meaningful work |
| `Task` | Specific work item inside a project |
| `WorkGoal` | Desired outcome (distinct from ephemeral `AiGoal`) |
| `WorkflowContext` | Active project/task + pending notes per workspace |

Permission-neutral. Persisted in migration `012_workspace_intent.sql`.
Capabilities: `work_context.read` / `work_context.write`.

---

## Intelligence aggregator (read-only)

`WorkspaceIntelligenceService` consumes existing systems and produces `WorkspaceIntelligenceState`:

- Current workspace / project / task
- Recent goals and activity
- Desktop understanding via **WorkspaceState** → Environment (not raw observation or platform capture DTOs)
- Memory awareness (counts / types — not raw content dump)
- Pending plans and approvals
- Blocked actions
- Recommendations with explanations
- Memory and preference highlights
- Applications and health

`authority_effect: none` on all intelligence audits.

Cannot execute, approve, grant, or bypass CommandPipeline / Permission Gateway.

Aggregation is **workspace-scoped** (Batch 5.5): plans, approvals, assistant workflows, and activity must attribute to the target workspace. Generate is read-only (no create-on-read for `WorkflowContext`).

Integrity audit: [Workspace Intelligence Integrity Audit](WORKSPACE-INTELLIGENCE-INTEGRITY-AUDIT.md).

---

## Product surface

Primary tab: **Work** (`WorkspaceIntelligencePanel`).
Assistant is a governed interface into Workspace Intelligence — not a parallel brain.
Work, Assistant, and Operator Console all call `generate_workspace_intelligence`.

---

## Audits

| Event | Authority |
|-------|-----------|
| `workspace.intent.created` | none |
| `workspace.intent.updated` | none |
| `workspace.context.updated` | none |
| `workspace.intelligence.generated` | none (single event per generate; Batch 5.5) |

---

## Explicit non-goals

- Autonomous behaviour / background agents
- AI-triggered actions
- Hidden workflows
- Second permission system
- Memory/preferences granting authority
- Governed Automation Contracts (Batch 6)
