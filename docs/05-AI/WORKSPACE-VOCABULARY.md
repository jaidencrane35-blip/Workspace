# Workspace Vocabulary (Phase 4 Batch 9.5)

Canonical product language. Prefer these terms in UI, docs, and Assistant copy.

| Term | Meaning | Do not call it |
|------|---------|----------------|
| **Work Goal** | Durable desired outcome on a Project/Task | Goal (bare), Assistant goal |
| **Cognitive Goal** | Semantic Goal node in Cognitive Model; references a Work Goal | Work Goal payload, Assistant Goal |
| **Objective** | Measurable outcome under a Goal/Initiative (Cognitive Model) | Work Goal, Milestone Engine row |
| **Initiative** | Coordinated effort spanning objectives/milestones (Cognitive Model) | Project |
| **Cognitive Milestone** | Semantic outcome checkpoint (Cognitive Model DurableStore) | Milestone Engine projection |
| **Working Set** | Current material set under cognitive focus | Composition members alone |
| **Cognitive Constraint / Risk / Opportunity** | Semantic understanding nodes (never execute) | GovernanceRisk, SessionRisk, ExploreOpportunity kind |
| **Assistant Goal** | Ephemeral planning statement for one Assistant/plan session | Work Goal |
| **Planning Snapshot** | Durable Cognitive Planning Engine projection (current + history) | AiPlan, Assistant plan |
| **Planning Proposal** | Active non-executing sequenced plan artefact | Executable workflow, Task Graph mutation |
| **Planning History** | Terminal superseded/abandoned plan evidence only | Actionable plan queue |
| **Reasoning Record** | Durable evidence of hypothesis/assumptions/reflection (never executes) | Conversational memory, AiPlan |
| **Reasoning History** | Append-only superseded/archived reasoning evidence | Actionable reasoning queue |
| **Cognitive Graph** | Reference-only cross-domain topology (never SoT) | Second ownership model, planner |
| **Cognitive Orchestration** | Coordination of refresh ordering / staleness (never executes) | Planner, lifecycle owner, autonomy layer |
| **Learning & Adaptation** | Observational meta-evidence and suggestion-only adaptations | Self-modifying authority, auto-apply, upstream truth |
| **Cognitive Agent Cast** | Specialised cognitive role perspectives (evidence only) | Actor, executor, permission owner, swarm autonomy |
| **Cognitive Autonomy** | Governed suggestion / opportunity layer (approval always required) | Autonomous agent, self-approving executor, hidden workflow |
| **Unified Workspace State** | Read-only composition envelope over authoritative sources | New source of truth, lifecycle owner, silent refresh |
| **Policy & Governance Engine** | Explainable policy evaluation evidence (Gateway remains final) | Second permission system, capability grantor, executor |
| **Historical Reconstruction** | Temporal comparison / change explanation from durable evidence | Event sourcing, authoritative replay log, invented history |
| **Temporal Intelligence** | Scoped historical understanding over reconstruction evidence | Simulation, forecasting, autonomous correction, invented causes |
| **Workspace Explanation Layer** | Cross-surface evidence-backed explanation package | Decision authority, simulator, command surface, conflict resolver |
| **Contextual Workspace Understanding** | Situational understanding from durable multi-surface evidence | Predictor, simulator, decision owner, certainty engine, planner |
| **Workspace Knowledge Synthesis** | Provenance-bound derived concepts / clusters / relationships from evidence | Memory SoT, Cognitive Model, decision authority, causation engine, planner |
| **Workspace Knowledge Integration** | Provenance-bound retrieval / join views over accumulated evidence layers | Memory SoT, second ontology, autonomous agent, decision authority, ranked-truth engine |
| **Graph History** | Terminal superseded graph snapshot evidence | Actionable graph mutations |
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
| **DecisionCandidate** | Ranked candidate produced by Decision Engine after intake, scoring, and ranking | Recommendation Engine candidate, Attention priority |
| **Package seal** | Immutable digest that freezes a Recommendation Engine intake package for Decision Engine review | Handshake, transfer, execution |
| **Handoff request** | Non-executing request from Recommendation Engine for Decision Engine to consider a sealed package | Handoff perform, transfer of authority |
| **Decision Engine acceptance** | Acknowledgement of a sealed handoff request without planner or execution authority | Planner handoff, execution |
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
