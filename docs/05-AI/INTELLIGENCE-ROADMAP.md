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
