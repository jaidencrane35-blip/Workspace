# Intelligence Layer Roadmap (Phase 3 Batches 3–7)

| Field | Value |
|-------|-------|
| **Purpose** | Sequence safe intelligence work after AI actor + planning foundations |
| **Permanent rule** | AI → Intent → Pipeline → Permission Gateway → Execution |

---

## Batch status

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

## Never build

- AI → Tool → Execution bypass
- AI-only pipelines / permission systems
- Lasting AI self-grants
- Hidden background automation
