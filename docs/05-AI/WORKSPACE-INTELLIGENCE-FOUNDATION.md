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
Workspace
    ↓
Project / Task / Goal (intent)
    ↓
Workspace Intelligence (understanding)
    ↓
Assistant / Recommendations (interfaces)
    ↓
Plan → Permission Gateway → Execution
```

The AI is not the product. The Workspace is the product.
Intelligence is informational. Authority remains at the Permission Gateway only.

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
- Pending plans and approvals
- Blocked actions
- Recommendations with explanations
- Memory and preference highlights
- Applications and health

`authority_effect: none` on all intelligence audits.

Cannot execute, approve, grant, or bypass CommandPipeline / Permission Gateway.

---

## Product surface

Primary tab: **Work** (`WorkspaceIntelligencePanel`).
Assistant remains one interface into governed planning — not the primary intelligence surface.

Diagnostics: Operator Console validates the same CommandHandler APIs.

---

## Audits

| Event | Authority |
|-------|-----------|
| `workspace.intent.created` | none |
| `workspace.intent.updated` | none |
| `workspace.context.updated` | none |
| `workspace.intelligence.generated` | none |
| `workspace.summary.created` | none |
| `workspace.insight.generated` | none |
| `workspace.recommendation.generated` | none |

---

## Explicit non-goals

- Autonomous behaviour / background agents
- AI-triggered actions
- Hidden workflows
- Second permission system
- Memory/preferences granting authority
- Governed Automation Contracts (deferred to next batch)
