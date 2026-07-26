# Intelligence Layer Roadmap

| Field | Value |
|-------|-------|
| **Purpose** | Sequence safe intelligence work after AI actor + planning foundations |
| **Permanent rule** | AI → Intent → Pipeline → Permission Gateway → Execution |

---

## Phase 3 batch status

| Batch | Theme | Status |
|-------|--------|--------|
| 1 | AI Actor | Complete (S46–47) |
| 2 | AI Planning | Complete (S48–49) |
| 3 | AI Context & Workspace Understanding | Complete (S50–51) |
| 4 | Capability Discovery & Tool Awareness | Complete (S52–53) |
| 5 | Planning Quality & Evaluation | Complete (S54–55) |
| 6 | Governed Action Orchestration | Complete (S56–57) |
| 7 | Governed Assistant Foundation | Complete (S58–59) |

---

## Phase 4 batch status

| Batch | Theme | Status |
|-------|--------|--------|
| 1 | Governed Memory Foundation | Complete (S60–61) |
| 2 | Model Provider Abstraction | Complete (S62–63) |
| 3 | Personalization (preference-aware proposals) | Complete (S64–65) |
| 4 | Product Assistant UX | Complete (S66–67) |
| 5 | Workspace Intelligence Foundation | Complete (S68–69) |
| 6 | Governed Automation Contracts | Planned |

---

## Batch 3 (done)

Read-only `AiWorkspaceAwareness` from `WorkspaceContext` + desktop window titles; context-aware launch proposals; gateway unchanged.

## Batch 4 (done)

Informational `ActionCatalog` + `AiActionAwareness`; planning explanations include required capabilities; gateway still decides. See [AI Action Catalog](AI-ACTION-CATALOG.md).

## Batch 5 (done)

Operational `AiPlanEvaluationReport` + quality/outcome classification; audits `ai.planning.proposal_evaluated` / `ai.planning.outcome_recorded`. Measurement only — gateway unchanged. See [AI Evaluation Foundation](AI-EVALUATION-FOUNDATION.md).

## Batch 6 (done)

`AiOrchestratedPlan` multi-step lifecycle with per-step gateway checks; pause on approval, fail on deny, no silent continue. See [AI Orchestration Foundation](AI-ORCHESTRATION-FOUNDATION.md).

## Batch 7 (done)

User-facing `AiAssistantWorkflow`: goal input → plan preview → user confirm → orchestration → Permission Gateway. Not autonomous OS control. See [AI Assistant Foundation](AI-ASSISTANT-FOUNDATION.md).

---

## Phase 4 Batch 1 (done)

Governed memory types + store; memory assembles into planning context only. Auditable lifecycle; no permission/execution influence. See [AI Memory Foundation](AI-MEMORY-FOUNDATION.md) and [Memory Policy](MEMORY-POLICY.md).

## Phase 4 Batch 2 (done)

Model provider interface + registry/routing; planning consumes provider candidates only. Failures are safe; providers cannot execute or grant. See [AI Model Provider Foundation](AI-MODEL-PROVIDER-FOUNDATION.md).

## Phase 4 Batch 3 (done)

Explicit user preference profiles; preference-aware ranking/explanations with disable toggle. No hidden profiling; gateway unchanged. See [AI Personalization Foundation](AI-PERSONALIZATION-FOUNDATION.md).

## Phase 4 Batch 4 (done)

Production Assistant UX over the existing governed pipeline: revise/regenerate/compare, structured explanations, clear workflow states. No new authority path. See [AI Product Assistant](AI-PRODUCT-ASSISTANT.md).

## Phase 4 Batch 5 (done)

Durable work context (Project/Task/Goal/WorkflowContext) + read-only Workspace Intelligence aggregator. Product **Work** tab; Assistant is one interface. Foundation for future automation contracts. See [Workspace Intelligence Foundation](WORKSPACE-INTELLIGENCE-FOUNDATION.md).

## Phase 4 Batch 5.5 (done)

Integrity audit and hardening of Workspace Intelligence — isolation, read-only generate, shared path, governance CASE tests. No new capabilities. See [Workspace Intelligence Integrity Audit](WORKSPACE-INTELLIGENCE-INTEGRITY-AUDIT.md).

## Phase 4 Batch 6 (done)

Governed automation contract foundation — durable Project/Task-scoped definitions with approval lifecycle, trigger/intent templates, intelligence surfacing, and `PrepareAutomationContractIntent` boundary. No workers, no autonomous execution. See [Governed Automation Contracts](GOVERNED-AUTOMATION-CONTRACTS.md).

## Phase 4 Batch 6.5 (done)

Integrity audit and hardening — definition fingerprints, stale-approval invalidation, prepare metadata, trust UX, CASE 1–10. Trigger engine readiness: **READY WITH CONDITIONS**. See [Governed Automation Contract Integrity Audit](GOVERNED-AUTOMATION-CONTRACT-INTEGRITY-AUDIT.md).

## Phase 4 Batch 7 (done)

Governed trigger evaluation foundation — `TriggerEvent`, `TriggerEvaluator`, explainable rejections, `AutomationIntentProposal`. No schedulers, workers, or automatic execution. See [Governed Trigger Evaluation](GOVERNED-TRIGGER-EVALUATION.md).

## Phase 4 Batch 8 (done)

Governed Decision Queue foundation — aggregate pending human decisions into one Workspace inbox with lifecycle overlay; sources remain authoritative; no execution or permission grants. See [Governed Decision Queue](GOVERNED-DECISION-QUEUE.md).

## Phase 4 Batch 9 (done)

Workspace Activity Graph foundation — pure aggregation read model linking work objects into timeline and relationships; synthetic IDs only; no second database or authority. See [Workspace Activity Graph](WORKSPACE-ACTIVITY-GRAPH.md).

## Phase 4 Batch 9.5 (done)

Workspace platform integration & coherence audit — singular ownership, Decision Queue as attention SoT, Activity Graph as relationship SoT, Intelligence/Assistant as consumers, unified vocabulary, continuous Work IA. No authority changes. See [Workspace Platform Coherence](WORKSPACE-PLATFORM-COHERENCE.md) and [Vocabulary](WORKSPACE-VOCABULARY.md).

## Phase 5 Batch 1 (done)

Workspace Continuity Engine foundation — read-only projection of focus, interrupted/resumable work, outstanding decisions, and suggested next steps across sessions. No autonomy. See [Workspace Continuity Engine](WORKSPACE-CONTINUITY-ENGINE.md).

## Phase 5 Batch 2 (done)

Workspace Attention Engine foundation — deterministic explainable prioritization over Continuity / Decision Queue / Activity Graph; Intelligence Recommendations consume Attention. No autonomy. See [Workspace Attention Engine](WORKSPACE-ATTENTION-ENGINE.md).

## Phase 5 Batch 3 (done)

Governed Decision Engine foundation (Sprints 80–81) — synthesize Attention + memory + personalization + goals into ranked, explainable `DecisionCandidate`s; accept → planner handoff only. Distinct from Decision Queue inbox. No autonomy. See [Workspace Decision Engine](WORKSPACE-DECISION-ENGINE.md).

## Phase 5 Batch 4 (done)

Workspace Task Graph foundation (Sprints 82–83) — persistent canonical work model with dependencies and progress; Attention / Decision Engine / Intelligence / Planner inputs consume the graph. No autonomy. See [Workspace Task Graph](WORKSPACE-TASK-GRAPH.md).

## Phase 5 Batch 5 (done)

Workspace Environment Model foundation (Sprint 84) — live desktop read model aggregating Windows Integration with work context; Intelligence and Attention become environment-aware. No window control or autonomy. See [Workspace Environment Model](WORKSPACE-ENVIRONMENT-MODEL.md).

## Phase 5 Batch 6 (done)

Workspace Composition Engine foundation (Sprint 85) — logical working environments from Environment + Task Graph + Continuity + Activity + Workflow; Attention and Intelligence consume compositions. No launch, grouping, or autonomy. See [Workspace Composition Engine](WORKSPACE-COMPOSITION-ENGINE.md).

## Phase 5 Batch 7 (done)

Workspace Purpose Model foundation (Sprint 86) — why work exists, projected from WorkGoals + projects + Task Graph + Composition + Continuity; Attention and Intelligence consume Purpose. WorkGoal remains DurableStore. No autonomy. See [Workspace Purpose Model](WORKSPACE-PURPOSE-MODEL.md).

## Phase 5 Batch 8 (done)

Workspace Evolution Model foundation (Sprint 87) — how work changed, projected from Activity Graph + Task Graph + Purpose + Composition + Continuity; Attention and Intelligence consume Evolution. Activity Graph remains history SoT. No prediction or second memory. See [Workspace Evolution Model](WORKSPACE-EVOLUTION-MODEL.md).

## Phase 5 Batch 9 (done)

Workspace Recommendation Engine foundation (Sprint 88) — what might help next, projected from Attention + Continuity + Evolution + Purpose + Task Graph + Composition + Decision Queue + Environment; Attention may surface recommendations; Intelligence embeds; Assistant explains. Suggestions only — not Decision Engine, not Planner. See [Workspace Recommendation Engine](WORKSPACE-RECOMMENDATION-ENGINE.md).

## Phase 5 Batch 10 (done)

Workspace Operating State foundation (Sprint 89) — what is happening right now, unified snapshot over existing understanding systems; Intelligence embeds; Assistant explains. Aggregator only — no durable store, no autonomy. See [Workspace Operating State](WORKSPACE-OPERATING-STATE.md).

## Phase 5 Batch 11 (done)

Workspace Pattern Model foundation (Sprint 90) — recurring structures from Activity + Evolution + Operating State + Composition + Task Graph; Recommendation may use as evidence; Attention may surface; Assistant explains. Not prediction or profiling. See [Workspace Pattern Model](WORKSPACE-PATTERN-MODEL.md).

## Phase 5 Batch 12 (done)

Workspace Adaptation Proposal foundation (Sprint 91) — possible improvements from Pattern + Recommendation + Operating State + Composition; human review required; accept → Intent handoff only. Never mutates layout or bypasses Gateway. See [Workspace Adaptation Proposal](WORKSPACE-ADAPTATION-PROPOSAL.md).

## Phase 5 Batch 13 (done)

Workspace Readiness Model foundation (Sprint 92) — preparedness for current work from Operating State + Environment + Composition + Task Graph + Purpose + Continuity + Evolution + Patterns + Decision Queue; Recommendation and Adaptation may consume gaps; never prepares or executes. Distinct from runtime WorkspaceHealth. See [Workspace Readiness Model](WORKSPACE-READINESS-MODEL.md).

## Phase 5.5 (done)

Architectural integrity & hardening audit of the cognition stack through Sprint 92 — ownership map, authority framing, assemble-order rules, `GateIntelligenceRead`, `attempt_execute` guards for remaining aggregators, Work UI regenerate fan-out reduction. No new features. See [Workspace Cognition Integrity Audit](WORKSPACE-COGNITION-INTEGRITY-AUDIT.md).

## Phase 6 Batch 1 (done)

Workspace Session Engine foundation (Sprint 94) — runtime orchestration projection over Intelligence; Work default view; owns no source data; never executes/prepares/restores. See [Workspace Session Engine](WORKSPACE-SESSION-ENGINE.md).

## Phase 6 Batch 2 (done)

Workspace Experience Layer foundation (Sprint 95) — presentation projection over Session; calm Work surface groupings with deterministic visibility; Assistant explain-only; owns no data/cognition/authority. See [Workspace Experience Layer](WORKSPACE-EXPERIENCE-LAYER.md).

## Phase 6 Batch 3 (done)

Workspace Work Context Engine foundation (Sprint 96) — semantic kind-of-work projection over Session/Experience/Intelligence; Intelligence embeds `work_context`; Attention/RE/Adaptation may reference as evidence only; never plans/executes/persists. See [Workspace Work Context Engine](WORKSPACE-WORK-CONTEXT-ENGINE.md).

## Phase 6 Batch 4 (done)

Workspace Navigation Engine foundation (Sprint 97) — interaction paths over understanding; Intelligence embeds `navigation` after Work Context; evidence-only consumption; never plans/executes/routes autonomously. See [Workspace Navigation Engine](WORKSPACE-NAVIGATION-ENGINE.md).

## Phase 6 Batch 5 (done)

Workspace Milestone Engine foundation (Sprint 98) — progress coordination over understanding; Intelligence embeds `milestones` after Navigation; evidence-only consumption; never plans/schedules/executes. See [Workspace Milestone Engine](WORKSPACE-MILESTONE-ENGINE.md).

## Phase 6 Batch 6 (done)

Workspace Working Style Model foundation (Sprint 99) — observable operating patterns; Intelligence embeds `working_style` after Milestones; separates observed behaviour from explicit preference; evidence-only consumption; never profiles/predicts/executes. See [Workspace Working Style Model](WORKSPACE-WORKING-STYLE-MODEL.md).

---

## Never build

- AI → Tool → Execution bypass
- AI-only pipelines / permission systems
- Lasting AI self-grants
- Hidden background automation
- Memory → permission bypass
- Model → permission / execution bypass
- Hidden preference learning / behavioral profiling
- Autonomous model switching / self-optimizing loops
- Chain-of-thought / hidden reasoning storage as authority
