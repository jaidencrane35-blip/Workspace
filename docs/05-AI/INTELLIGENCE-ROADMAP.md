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
| 3 | Personalization (preference-aware proposals) | Planned |
| 4 | Product Assistant UX | Planned |
| 5 | Governed Automation Contracts | Planned |

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

---

## Never build

- AI → Tool → Execution bypass
- AI-only pipelines / permission systems
- Lasting AI self-grants
- Hidden background automation
- Memory → permission bypass
- Model → permission / execution bypass
- Autonomous model switching / self-optimizing loops
- Chain-of-thought / hidden reasoning storage as authority
