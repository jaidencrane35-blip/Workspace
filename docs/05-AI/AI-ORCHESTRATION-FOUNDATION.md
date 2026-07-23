# AI Orchestration Foundation

| Field | Value |
|-------|-------|
| **Purpose** | Multi-step AI plans that organize governed proposals without autonomy |
| **Status** | Sprints 56–57 |
| **Dependencies** | [AI Planning Foundation](AI-PLANNING-FOUNDATION.md), [Permission Architecture](../07-Security/PERMISSION-ARCHITECTURE.md) |

---

## 1. Rule

**AI can organize. AI can request. The system decides. Execution follows permission.**

A plan is **not** permission and **not** execution authority. It is only a structured collection of requested actions.

```
AI Orchestrated Plan
    ↓
Action Proposal (step)
    ↓
AiActionRequest
    ↓
Command Pipeline
    ↓
Permission Gateway
    ↓
Allow | Deny | ApprovalRequired
```

No shortcut path. No silent continue after denial.

---

## 2. Models

| Type | Meaning |
|------|---------|
| `AiOrchestratedPlan` | Multi-step lifecycle instance |
| `AiPlanStep` | Ordered proposal + dependencies + step state |
| `AiOrchestratedPlanState` | proposed → awaiting_approval / partially_approved / executing → completed \| failed \| cancelled |
| `AiPlanStepState` | pending → awaiting_approval / running → completed \| denied \| failed \| cancelled |

Dependencies default to sequential (step N depends on step N−1).

---

## 3. Lifecycle behavior

| Event | Behavior |
|-------|----------|
| Create | Plan stored in-memory; no execution |
| Advance | Submit next runnable step through existing AI launch path |
| ApprovalRequired | Pause plan; await human `DecideApproval` |
| Allow once + resume | Re-submit paused step through gateway (grant consumed) |
| Deny + resume | Mark step denied; plan fails; later steps do not run |
| Failure | Record failed step; plan fails |
| Cancel | Cancel non-terminal steps; plan cancelled |

Partial approval: completed steps remain completed; denied steps never execute; no hidden retries.

---

## 4. Auditing

Operational events only:

- `ai.plan.created`
- `ai.plan.action_started`
- `ai.plan.action_completed`
- `ai.plan.action_failed`
- `ai.plan.cancelled`

Store plan/step identity, permission result, execution result. Do not store chain-of-thought.

---

## 5. Diagnostics

| IPC | Role |
|-----|------|
| `create_orchestrated_ai_plan` | Create multi-step plan |
| `get_orchestrated_ai_plan` | Preview plan |
| `advance_orchestrated_ai_plan` | Run next steps through gateway |
| `resume_orchestrated_ai_plan` | Continue after human decision |
| `cancel_orchestrated_ai_plan` | Stop safely |

Operator Console: “Create multi-step plan” / “Advance plan (gateway)” / “Resume after approval” / “Cancel plan”.

---

## 6. Explicit non-goals

- Autonomous agents / background loops
- Unlimited retries
- AI permissions / self-authorizing plans
- Tool calling / multi-agent coordination
- Durable AI memory

This phase is orchestration, not autonomy.
