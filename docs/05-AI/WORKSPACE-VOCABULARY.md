# Workspace Vocabulary (Phase 4 Batch 9.5)

Canonical product language. Prefer these terms in UI, docs, and Assistant copy.

| Term | Meaning | Do not call it |
|------|---------|----------------|
| **Work Goal** | Durable desired outcome on a Project/Task | Goal (bare), Assistant goal |
| **Assistant Goal** | Ephemeral planning statement for one Assistant/plan session | Work Goal |
| **Task** | Durable work item | Action |
| **Action** | Executable catalog/command candidate | Task |
| **Automation Contract** | Stored future-intent definition (approval ≠ execution) | Automation, Policy |
| **Intent Proposal** | Trigger evaluation output awaiting human review | Suggestion, Recommendation, bare Proposal |
| **Action Proposal** | AI planning step candidate | Intent Proposal |
| **Decision** | Inbox item in the Decision Queue (human attention) | Approval (unless it is one) |
| **Permission Approval** | Gateway consent for a capability/command | Decision (broader), Contract Approval |
| **Contract Approval** | Consent for an Automation Contract *definition* | Permission Approval, execution grant |
| **Recommendation** | Advisory next step from Intelligence (never executes) | Suggestion (product UI), Proposal |
| **Suggestion** | Legacy deterministic context hint (diagnostic/Sprint 20) | Recommendation in product Work UI |
| **Intent** | Pipeline request entering Command Pipeline | Proposal, Decision |
| **Activity** | Synthetic Activity Graph node (read model) | Audit event, Decision |

## Explainability fields (consistent meaning)

| Field | Answers |
|-------|---------|
| `explanation` / why | Why does this object exist? |
| `source_type` + `source_id` | Where did it come from? |
| `parent` / `related_*` | What depends on it / connects to it? |
| `recommended_action` / unresolved | What am I waiting for / what happens next? |
| `authority_effect` | Always `none` for aggregators and intelligence |

No chain-of-thought. No hidden reasoning storage as authority.
