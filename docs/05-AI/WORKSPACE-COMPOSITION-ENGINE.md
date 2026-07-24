# Workspace Composition Engine

| Field | Value |
|-------|-------|
| **Purpose** | Read-only model of how applications, layouts, projects, and tasks belong together as a logical working environment |
| **Owner** | Architecture |
| **Status** | Phase 5 Batch 6 foundation |
| **Authority** | Never creates authority (`authority_effect: none`) |

---

## Principle

Applications are resources. Projects are work. Layouts are presentation.

A **Workspace Composition** is meaning — how those pieces naturally belong together — not execution.

```
Environment + Task Graph + Continuity + Activity + Workflow + Decision Queue
        ↓
Workspace Composition Engine   ← this document
        ↓
Attention / Intelligence / Assistant consumers
```

---

## Represents

- Applications (present / missing) as composition members
- Windows and layout associations from the Environment Model
- Task Graph nodes and relationships
- Active project / work from WorkflowContext
- Continuity focus
- Recent Activity Graph evidence
- Gaps: missing applications, disconnected work
- Explainable relationships (`supports`, `belongs_to`, `expects`, `focuses`, Task Graph kinds)

Does **not** duplicate persistence — aggregates existing read models only.

---

## Boundaries

May: observe, aggregate, explain, feed Attention / Intelligence.  
Must not: launch apps, group or move windows, invent AI compositions, grant permissions, change Gateway behavior.

Audit: `workspace.composition.generated` with `authority_effect: none`.
