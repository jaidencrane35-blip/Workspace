# AI Planning Foundation

| Field | Value |
|-------|-------|
| **Purpose** | Define the cognitive planning layer: goals → proposals → governed requests |
| **Status** | Sprints 48–49 |
| **Dependencies** | [AI Actor Foundation](AI-ACTOR-FOUNDATION.md), [Permission Architecture](../07-Security/PERMISSION-ARCHITECTURE.md) |

---

## 1. Rule

**AI proposes. The system evaluates. Humans authorize when required.**

Planning output is never execution authority.

```
User Goal
    ↓
AI Planner (deterministic diagnostic / future models)
    ↓
Action Proposals
    ↓
AiActionRequest
    ↓
Command Pipeline
    ↓
Permission Gateway
    ↓
Allow | Deny | ApprovalRequired
```

---

## 2. Models

| Type | Meaning |
|------|---------|
| `AiGoal` | What the user is trying to achieve (statement only) |
| `AiPlanningContext` | Goal + available context hints (not long-term memory) |
| `AiActionProposal` | An action AI thinks may help — not executable authority |
| `AiPlan` | Goal + list of proposals |
| `AiProposalSubmission` | Proposal + `AiActionRequest` + authority outcome |

---

## 3. Planning boundary

The planner **may**:

- Accept a goal statement
- Read provided context (e.g. available application ids)
- Emit proposals and convert them to `AiActionRequest`

The planner **must not**:

- Call `ProcessLauncher` / launch services
- Create capability grants
- Modify permissions
- Auto-retry denied / ApprovalRequired outcomes
- Store chain-of-thought or vector memory

---

## 4. Auditing

Operational events only:

- `ai.planning.plan_created` — goal id, statement, proposal count
- `ai.planning.proposal_created` — goal id, proposal id, command, target
- Existing `permission.*` events when proposals are submitted

No private reasoning traces.

---

## 5. Diagnostic

IPC `diagnose_ai_workspace_plan` — Operator Console “Plan as AI (prepare workspace)”.

Expected default: proposals submit → `ApprovalRequired` for each (AI has zero capabilities).

---

## 6. Out of scope

Autonomous agents, self-directed loops, AI memory, tool calling, lasting AI grants, multi-agent systems.
