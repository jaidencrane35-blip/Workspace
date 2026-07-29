# Optimisation Log

| Field | Value |
|-------|-------|
| **Purpose** | Record material performance, storage, indexing, and compression work so optimisations remain human-inspectable |
| **Owner** | Engineering |
| **Status** | Active template |
| **Related** | [AI Engineering Governance §3 & §11](../00-Governance/AI_ENGINEERING_GOVERNANCE.md), [Performance Budgets](../02-Architecture/PERFORMANCE-BUDGETS.md) |

Use one cycle entry per meaningful optimisation. Do **not** log trivial micro-edits.

Every cycle must preserve the understanding path:

```text
Human concept → Optimised representation → Decoder / inspector → Human understanding
```

---

## Cycle template (copy below)

```markdown
### Cycle: <short-id>

| Field | Value |
|-------|-------|
| **Date** | YYYY-MM-DD |
| **Goal** | |
| **Problem** | |
| **Analysis** | |
| **Changes** | |
| **Files affected** | |
| **Validation** | |
| **Maintainability score** | /10 |
| **Product alignment score** | /10 |
| **Human review required** | Yes / No |
| **Inspector / decoder path** | How a human inspects or decodes the representation |
```

---

## Cycles

_None yet. Newest first._
