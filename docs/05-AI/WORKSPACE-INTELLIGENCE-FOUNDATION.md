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

`WorkspaceIntelligenceService` loads **one** `WorkspaceState` per generate cycle (Sprint 120) and passes it into Environment — it does not read observation snapshots or DesktopWindowSnapshot directly.

`DesktopWindowSnapshot` is legacy IPC/platform compatibility only (Sprint 121). Runtime desktop truth is **WorkspaceState**.

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
- Desktop understanding via **WorkspaceState** → Environment (not raw observation / DesktopWindowSnapshot)
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
