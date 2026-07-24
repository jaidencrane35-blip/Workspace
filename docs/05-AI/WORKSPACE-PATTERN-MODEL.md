# Workspace Pattern Model

| Field | Value |
|-------|-------|
| **Purpose** | Read-only projection of recurring workspace structures |
| **Owner** | Architecture |
| **Status** | Phase 5 Batch 11 foundation (Sprint 90) |
| **Authority** | Never creates authority (`authority_effect: none`) |

---

## Principle

Patterns describe repetition.

They are not predictions, surveillance, automation, or authority.

```
Activity Graph + Evolution + Operating State
  + Composition + Task Graph + Environment + Purpose + Continuity + Decision Queue
        ↓
Workspace Pattern Model   ← this document
        ↓
Recommendation Engine (evidence) / Attention (surface) / Intelligence / Assistant
```

Activity Graph remains history SoT. Pattern Model invents nothing and persists nothing.

---

## Ownership

| Concept | Kind | Owner |
|---------|------|-------|
| Pattern | Aggregator | `WorkspacePatternService` |

---

## Pattern kinds

- ApplicationPattern
- WorkflowPattern
- TaskPattern
- DecisionPattern
- EnvironmentPattern

Every pattern includes observation, evidence, confidence (high/medium/low), and impact.

---

## Boundaries

May: observe, aggregate, explain, feed Recommendation / Attention / Intelligence.  
Must not: predict, profile users, store a second history, execute, grant permissions, create automation, bypass Permission Gateway.

Audits:
- `workspace.pattern.generated`
- `workspace.pattern.updated`
- `workspace.pattern.used_for_recommendation`

All carry `authority_effect: none`.

IPC: `generate_workspace_pattern`
