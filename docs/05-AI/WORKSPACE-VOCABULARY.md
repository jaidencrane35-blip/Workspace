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
| **Attention priority** | Prioritized existing signal from Attention Engine (Intelligence field `recommended_actions`) | Recommendation Engine candidate |
| **Recommendation** | Prefer **Attention priority** or **Recommendation Candidate** — avoid bare “Recommendation” in new UI | Suggestion (product UI), Proposal |
| **Recommendation Candidate** | Typed next-step suggestion from Recommendation Engine (never executes or accepts) | Decision Engine candidate, Attention priority |
| **Operating State** | Unified current-situation snapshot over understanding systems (never executes) | WorkflowContext, Intelligence |
| **Pattern** | Recurring structure observation from existing workspace signals (never predicts or profiles) | Memory, Analytics, Preference |
| **Adaptation Proposal** | Possible Workspace improvement for human review (never applies; accept → Intent handoff) | Recommendation, Automation Contract |
| **Workspace Readiness** | Preparedness for current work (ready / partially ready / blocked; never prepares) | WorkspaceHealth, Recommendation, Adaptation |
| **Workspace Session** | Runtime orchestration snapshot over Intelligence (canonical runtime model; owns nothing) | Continuity session_anchor, Intelligence, Operating State, Experience |
| **Workspace Experience** | Presentation model over Session for the Work surface (visibility groupings only; owns nothing) | Session, Intelligence, Operating State, UI layout state |
| **Work Context** | Semantic kind-of-work classification over Session/Experience/Intelligence (not a Project/Task/Session; owns nothing) | Project, Task, Session, Experience, Operating State |
| **Workspace Navigation** | Interaction paths over existing understanding (where to inspect next; owns nothing) | Planner, Router, Session, Work Context, Experience |
| **Workspace Milestones** | Coordination projection of progress toward meaningful outcomes (owns nothing) | Planner, Scheduler, Project management, Task Graph |
| **Workspace Working Style** | Observable operating-pattern projection (owns nothing; observed ≠ preferred) | Profiling, Surveillance, Prediction, Preferences |
| **Workspace Transition** | Semantic movement between work states (owns nothing; never restores) | Restoration, Automation, Scheduler, Executor |
| **Workspace Interaction** | Unified interaction opportunities over understanding (owns nothing; select → Intent handoff only) | Planner, Executor, Recommendation Engine, Decision Queue, Automation |
| **Workspace Profile** | Durable user-owned preferred environment setup (references entities; never activates) | UserPreferenceProfile, Automation, Restoration, Environment Model live aggregator |
| **Suggestion** | Legacy deterministic context hint (diagnostic/Sprint 20) | Recommendation in product Work UI |
| **Intent** | Pipeline request entering Command Pipeline | Proposal, Decision |
| **Activity** | Synthetic Activity Graph node (read model) | Audit event, Decision |
| **Continuity** | Cross-session resume narrative (focus, interrupted, what changed) | Memory (durable notes), Activity (timeline) |
| **Current Focus** | Active project/task from WorkflowContext | Inferred first project |
| **Attention** | Canonical prioritization of existing Workspace information | Notification spam, autonomous ranking |

## Explainability fields (consistent meaning)

| Field | Answers |
|-------|---------|
| `explanation` / why | Why does this object exist? |
| `source_type` + `source_id` | Where did it come from? |
| `parent` / `related_*` | What depends on it / connects to it? |
| `recommended_action` / unresolved | What am I waiting for / what happens next? |
| `authority_effect` | Always `none` for aggregators and intelligence |

No chain-of-thought. No hidden reasoning storage as authority.
