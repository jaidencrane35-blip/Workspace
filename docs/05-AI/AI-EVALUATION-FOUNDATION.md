# AI Evaluation Foundation

| Field | Value |
|-------|-------|
| **Purpose** | Measure AI proposal quality and outcomes without increasing AI authority |
| **Status** | Sprints 54–55 |
| **Dependencies** | [AI Planning Foundation](AI-PLANNING-FOUNDATION.md), [AI Context Foundation](AI-CONTEXT-FOUNDATION.md), [Permission Architecture](../07-Security/PERMISSION-ARCHITECTURE.md) |

---

## 1. Rule

**Evaluation observes. The Permission Gateway decides. Execution requires authorization.**

```
User Goal
    ↓
AI Planner
    ↓
Action Proposal
    ↓
Evaluation (quality / necessity / outcome class)
    ↓
AiActionRequest → Command Pipeline → Permission Gateway
```

Evaluation never grants capabilities, never executes, and never bypasses the gateway.

---

## 2. Separations

| System | Question |
|--------|----------|
| AI reasoning | What should happen? |
| Evaluation | Was this proposal good? |
| Permission | Is this allowed? |
| Execution | Did it happen? |

These remain separate. Evaluation metadata is not permission metadata and not chain-of-thought.

---

## 3. Models

| Type | Meaning |
|------|---------|
| `AiProposalValidity` | Structural validity of a proposal |
| `AiProposalRelevance` | Heuristic relevance to the goal |
| `AiProposalQualityIssue` | Invalid / irrelevant / duplicate / unnecessary |
| `AiProposalOutcomeClass` | Created, approval_required, denied, succeeded, failed, … |
| `AiProposalEvaluation` | Per-proposal measurement record |
| `AiPlanEvaluationReport` | Plan-level report + summary counts |

`AiPlanEvaluationReport.authority_note` always states that evaluation is observational only.

---

## 4. Auditing

Operational events only:

- `ai.planning.proposal_evaluated` — proposal id, validity, relevance, quality issues, outcome class
- `ai.planning.outcome_recorded` — proposal id + classified outcome (not for created/not_submitted)

Do **not** store:

- chain-of-thought
- hidden model reasoning
- private inference traces

---

## 5. Diagnostics

| IPC | Role |
|-----|------|
| `diagnose_ai_plan_evaluation` | Plan → evaluate (no submit / no execute) |
| `get_ai_evaluation_history` | Derived evaluation history from audits |

Operator Console: “Evaluate plan (no submit)” / “Load evaluation history”.

Plan submission (`diagnose_ai_workspace_plan`) also records evaluation audits after governance outcomes — still without granting authority.

---

## 6. Explicit non-goals

- Autonomous execution
- AI self-improvement / RL
- AI permissions
- Tool calling / agent loops
- Memory systems that optimize in the background

Evaluation is the measurement layer that allows future intelligence to improve **safely**.
