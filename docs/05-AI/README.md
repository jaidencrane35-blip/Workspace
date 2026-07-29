# AI Architecture Index

This directory contains the AI subsystem contract documents.

## Primary Canonical Documents

- `AI-PRINCIPLES.md`
- `AI-OPERATING-MODEL.md`
- `WORKSPACE-VOCABULARY.md`
- `WORKSPACE-RECOMMENDATION-ENGINE.md`
- `WORKSPACE-DECISION-ENGINE.md`
- `WORKSPACE-RECOMMENDATION-DECISION-BOUNDARY.md`
- `WORKSPACE-GOVERNANCE.md`
- `INTELLIGENCE-ROADMAP.md`
- `PROGRAMME-IV-INTERACTION-RUNTIME.md`

## Programme IV — Interaction Runtime (Evidence family)

Entry point: `PROGRAMME-IV-INTERACTION-RUNTIME.md`

| Batch | Architecture document | Implementation roots |
|---|---|---|
| 1 Semantic Query | `WORKSPACE-SEMANTIC-QUERY-ARCHITECTURE.md` | `packages/*/workspace_semantic_query*` |
| 2 Evidence Navigation | `WORKSPACE-EVIDENCE-NAVIGATION-ARCHITECTURE.md` | `packages/*/workspace_evidence_navigation*` |
| 3 Evidence Trace | `WORKSPACE-EVIDENCE-TRACE-ARCHITECTURE.md` | `packages/*/workspace_evidence_trace*` |
| 4 Evidence Coverage | `WORKSPACE-EVIDENCE-COVERAGE-ARCHITECTURE.md` | `packages/*/workspace_evidence_coverage*` |
| 5 Evidence Consistency | `WORKSPACE-EVIDENCE-CONSISTENCY-ARCHITECTURE.md` | `packages/*/workspace_evidence_consistency*` |
| 6 Evidence Dependency | `WORKSPACE-EVIDENCE-DEPENDENCY-ARCHITECTURE.md` | `packages/*/workspace_evidence_dependency*` |
| 7 Evidence Freshness | `WORKSPACE-EVIDENCE-FRESHNESS-ARCHITECTURE.md` | `packages/*/workspace_evidence_freshness*` |
| 8 Evidence Completeness | `WORKSPACE-EVIDENCE-COMPLETENESS-ARCHITECTURE.md` | `packages/*/workspace_evidence_completeness*` |
| 9 Evidence Reliability | `WORKSPACE-EVIDENCE-RELIABILITY-ARCHITECTURE.md` | `packages/*/workspace_evidence_reliability*` |
| 10 Observational Scaffold | `WORKSPACE-EVIDENCE-OBSERVATIONAL-SCAFFOLD-ARCHITECTURE.md` | `workspace_evidence_contract`, `evidenceProjectionContract.ts`, governance `EVIDENCE_ENGINE_GUARD_SPECS` |
| 11 Assistant Surface | `CONVERSATIONAL-ASSISTANT-SURFACE-ARCHITECTURE.md` | `packages/*/workspace_assistant_surface*`, `assistantSurfaceProjection.ts` |
| 12 Assistant Context Intelligence | `ASSISTANT-CONTEXT-INTELLIGENCE-ARCHITECTURE.md` | `packages/*/workspace_assistant_context*`, `assistantContextProjection.ts` |
| 13 Assistant Retrieval Intelligence | `ASSISTANT-RETRIEVAL-INTELLIGENCE-ARCHITECTURE.md` | `packages/domain/src/workspace_assistant_retrieval/`, `packages/kernel/src/services/workspace_assistant_retrieval.rs` |
| 14 Assistant Explanation Intelligence | `ASSISTANT-EXPLANATION-INTELLIGENCE-ARCHITECTURE.md` | `packages/domain/src/workspace_assistant_explanation/`, `packages/kernel/src/services/workspace_assistant_explanation.rs`, `assistantExplanationProjection.ts` |
| 15 Assistant Interaction Intelligence | `ASSISTANT-INTERACTION-INTELLIGENCE-ARCHITECTURE.md` | `packages/domain/src/workspace_assistant_interaction/`, `packages/kernel/src/services/workspace_assistant_interaction.rs`, `assistantInteractionProjection.ts` |
| 16 Assistant Personalisation Boundary | `ASSISTANT-PERSONALISATION-BOUNDARY-ARCHITECTURE.md` | `packages/domain/src/workspace_assistant_personalisation/`, `packages/kernel/src/services/workspace_assistant_personalisation.rs`, `assistantPersonalisationProjection.ts` |

Maintainability audit: `../03-Engineering/PROGRAMME-IV-MAINTAINABILITY-AUDIT.md`

## Recommendation and Decision Contract Families

- `WORKSPACE-RECOMMENDATION-*.md`
- `WORKSPACE-DECISION-ENGINE-*.md`
- `WORKSPACE-RECOMMENDATION-DECISION-*.md`

These files define bounded contracts and lifecycle slices. Treat the canonical
documents listed above as entry points, then drill into specialized contracts
for implementation details.

## Governance Contract Stubs

Files named `WORKSPACE-GOVERNANCE-*.md` intentionally forward to
`WORKSPACE-GOVERNANCE.md` for domain-specific views.

## Audit and Sprint Traceability

Cross-reference with:

- `../03-Engineering/IPC-SURFACE.md`
- `../03-Engineering/PROGRAMME-IV-MAINTAINABILITY-AUDIT.md`
- `../08-Roadmap/ROADMAP.md`
- `../10-Sprints/sprints/`
