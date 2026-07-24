# Workspace Recommendation Engine

| Field | Value |
|-------|-------|
| **Purpose** | Read-only projection of what might be useful next |
| **Owner** | Architecture |
| **Status** | Phase 5 Batch 9 foundation (Sprint 88) |
| **Authority** | Never creates authority (`authority_effect: none`) |

---

## Principle

Recommendations are suggestions.

They are not instructions, permissions, execution, or automation.

```
Attention + Continuity + Evolution + Purpose
  + Task Graph + Composition + Decision Queue + Environment
        ↓
Workspace Recommendation Engine   ← this document
        ↓
Attention (surface) / Intelligence / Assistant (explain)
```

Existing systems remain authoritative. The Recommendation Engine invents nothing.

---

## Ownership

| Concept | Kind | Owner |
|---------|------|-------|
| Recommendation candidate | Aggregator | `WorkspaceRecommendationEngineService` |

Distinct from:

| Concept | Owner | Notes |
|---------|-------|-------|
| Intelligence `recommended_actions` | Attention projection | Product Attention shortcuts |
| Decision Engine candidates | `DecisionEngineService` | Accept → Planner handoff |

Recommendation Engine owns **no persistence** of candidates.

---

## Explainability

Every recommendation includes:

- **Reason** — why suggested
- **Evidence** — which workspace signals support it
- **Impact** — what would improve
- **Confidence** — high / medium / low (display hint, not opaque AI score)

---

## Types (informational only)

`ContinueWork` · `ResolveBlocker` · `ReviewDecision` · `CompleteTask` · `ReorganizeWorkspace` · `RestoreContext` · `ExploreOpportunity`

None of these execute.

---

## Boundaries

May: observe, aggregate, explain, feed Attention / Intelligence / Assistant.  
Must not: execute, approve, grant permissions, modify tasks/projects, trigger automation, accept into Planner, bypass Permission Gateway.

Audit: `workspace.recommendation_engine.generated` with `authority_effect: none`.

IPC: `generate_workspace_recommendation_engine`
