# AI Product Assistant Experience

| Field | Value |
|-------|-------|
| **Purpose** | Production UX over the governed assistant pipeline (Sprints 66–67 / P4-B4) |
| **Owner** | Lead Software Engineer |
| **Dependencies** | [AI Assistant Foundation](AI-ASSISTANT-FOUNDATION.md), [AI Orchestration Foundation](AI-ORCHESTRATION-FOUNDATION.md), [Permission Architecture](../07-Security/PERMISSION-ARCHITECTURE.md) |
| **Update Process** | Update when product workflow states / revise-compare UX change |

---

## Permanent rule

```
Human intent
    ↓
Assistant (interface)
    ↓
Planning
    ↓
Evaluation
    ↓
Permission Gateway
    ↓
Execution
```

No shortcuts. No hidden execution. No authority changes.
The assistant remains an interface layer only.

---

## Product workflow

| Product state | Domain state |
|---------------|--------------|
| idle | receiving_goal / none |
| understanding | understanding |
| planning | generating_plan |
| evaluating | evaluating |
| awaiting_confirmation | awaiting_confirmation |
| awaiting_permission | waiting_for_permission |
| executing | submitting_actions |
| completed / failed / cancelled | same |

Users can:

- Enter conversational goals
- Preview plans with structured explanations
- See memory / preference influence indicators
- Review permission requirement summaries
- Revise goals, regenerate plans, compare revisions
- Confirm, cancel, and resume paused plans

---

## Explanations

Every proposal answers:

1. Why was this suggested?
2. Why does it need permission?
3. What happens if I approve?

Never expose chain-of-thought, hidden reasoning, or raw model prompts.

---

## Auditing

| Event | Authority effect |
|-------|------------------|
| `ai.assistant.goal_updated` | none |
| `ai.assistant.plan_regenerated` | none |
| `ai.assistant.plan_compared` | none |
| `ai.assistant.explanation_viewed` | none |

Existing events (`goal_received`, `plan_presented`, `user_confirmed`, `cancelled`) remain.

---

## Surfaces

| Surface | Role |
|---------|------|
| **Assistant** tab | Primary product experience |
| **Diagnostic** Operator Console | Validates the same CommandHandler pipeline |

IPC (shared):

- `submit_assistant_goal`
- `revise_assistant_goal`
- `regenerate_assistant_plan`
- `compare_assistant_plan_revisions`
- `record_assistant_explanation_viewed`
- `confirm_assistant_workflow` / `resume_assistant_workflow` / `cancel_assistant_workflow`

---

## Explicit non-goals

- Autonomous assistant behaviour
- Background planning / automatic execution
- Recurring workflows
- Hidden personalization / self-learning
- Agent frameworks / plugin marketplace
