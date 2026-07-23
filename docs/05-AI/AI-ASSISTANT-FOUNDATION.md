# AI Assistant Foundation

| Field | Value |
|-------|-------|
| **Purpose** | User-facing governed assistant: goal → plan preview → confirm → gateway |
| **Status** | Sprints 58–59 |
| **Dependencies** | [AI Orchestration Foundation](AI-ORCHESTRATION-FOUNDATION.md), [Permission Architecture](../07-Security/PERMISSION-ARCHITECTURE.md) |

---

## 1. Rule

**The assistant is the translator between human intent and governed intelligence.**

It is not the controller and not an authority layer.

```
Human intent
    ↓
AI Assistant (interface)
    ↓
AI Plan / Orchestrated Plan
    ↓
User confirmation
    ↓
Permission Gateway
    ↓
Execution
```

---

## 2. Separations

| Layer | Question |
|-------|----------|
| Assistant | What does the user want? |
| Planner | What actions could achieve it? |
| Permission Gateway | Are those actions allowed? |

Do not combine these layers.

---

## 3. Models

| Type | Meaning |
|------|---------|
| `AiAssistantWorkflow` | User-facing workflow instance |
| `AiAssistantPlanPreview` | Explained actions + permission note |
| `AiAssistantWorkflowState` | receiving_goal → understanding → generating_plan → evaluating → awaiting_confirmation → submitting_actions → waiting_for_permission → completed \| failed \| cancelled |

The workflow links to an `AiOrchestratedPlanId` and delegates advance/resume/cancel to Batch 6 APIs.

---

## 4. User controls

Users can:

- Express natural-language goals
- Review proposed actions and capability hints
- Confirm continuation
- Cancel before or during the workflow
- See blocked / denied outcomes in status messages

---

## 5. Auditing

Operational events only:

- `ai.assistant.goal_received`
- `ai.assistant.plan_presented`
- `ai.assistant.user_confirmed`
- `ai.assistant.cancelled`
- `ai.assistant.goal_updated` (`authority_effect: none`)
- `ai.assistant.plan_regenerated` (`authority_effect: none`)
- `ai.assistant.plan_compared` (`authority_effect: none`)
- `ai.assistant.explanation_viewed` (`authority_effect: none`)

No chain-of-thought or private reasoning.

Product UX: see [AI Product Assistant](AI-PRODUCT-ASSISTANT.md).

---

## 6. Diagnostics

| IPC | Role |
|-----|------|
| `submit_assistant_goal` | Goal → plan preview |
| `get_assistant_workflow` | Preview / refresh state |
| `revise_assistant_goal` | Revise goal → regenerate |
| `regenerate_assistant_plan` | Regenerate same goal |
| `compare_assistant_plan_revisions` | Diff plan revisions |
| `record_assistant_explanation_viewed` | Audit explanation view |
| `confirm_assistant_workflow` | Confirm → gateway path |
| `resume_assistant_workflow` | Continue after permission decision |
| `cancel_assistant_workflow` | Cancel workflow + linked plan |

Primary UI: **Assistant** tab. Operator Console validates the same pipeline.

---

## 7. Explicit non-goals

- Autonomous agents / background AI control
- Persistent AI permissions
- Self-triggering workflows
- Memory systems / multi-agent collaboration
- Hidden automation
