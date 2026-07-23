# AI Memory Foundation

| Field | Value |
|-------|-------|
| **Purpose** | Governed memory as an intelligence enhancement layer (Sprints 60–61 / P4-B1) |
| **Owner** | Lead Software Engineer |
| **Dependencies** | [Memory Policy](MEMORY-POLICY.md) (DEC-014), [AI Planning Foundation](AI-PLANNING-FOUNDATION.md), [Permission Architecture](../07-Security/PERMISSION-ARCHITECTURE.md) |
| **Update Process** | Update with memory model / access-path changes |

---

## Permanent rule

```
AI proposes → System evaluates → Permissions decide → Execution follows authorization
```

Memory improves understanding. Permissions control action.

---

## What memory is

Informational, inspectable, removable, permission-aware context used to improve:

- suggestions
- planning quality
- explanations
- personalization of proposals

## What memory is not

- capability
- approval
- trust
- authority
- an execution path

```
Memory ──X──> Permission Gateway
Memory ──X──> Execution
```

---

## Memory types

| Type | Scope | Examples |
|------|-------|----------|
| `session` | Temporary | conversation/task state |
| `workspace` | Workspace | preferred apps, project context |
| `user_preference` | User | explicit workflow/UI choices |
| `system_knowledge` | Stable | capability descriptions, docs |

Hidden behavioral profiling is out of scope.

---

## Model

- `MemoryEntry` — id, type, key, summary, source, workspace scope, lifecycle, metadata, timestamps
- `MemoryMetadata` — confidence, occurrence count, user visibility, optional attributes JSON
- Lifecycle: `created` → `updated` / `viewed` → `deleted` (soft delete)

Persistence: `ai_memory_entries` (migration `010_ai_memory.sql`).

Capabilities (local user): `memory.read`, `memory.write`. AI actors start with none.

---

## Governed access path

```
Memory store
    │
    v
Context assembly (AiMemoryAwareness)
    │
    v
AI Planning (ranking / explanations only)
    │
    v
Proposal
    │
    v
Permission Gateway
```

`AiPlanningContext.memory_awareness` may:

- rank preferred applications first
- append memory notes to proposal explanations

It must not:

- alter capability sets
- change gateway decisions
- call launchers or grant mutators

---

## Audits (operational only)

| Event | Meaning |
|-------|---------|
| `ai.memory.created` | Entry stored |
| `ai.memory.accessed` | Listed / assembled for context |
| `ai.memory.deleted` | Soft-deleted or cleared |

Metadata includes source, scope, lifecycle, and `"authority_effect": "none"`.

Never store chain-of-thought, private reasoning traces, or hidden model output.

---

## Diagnostics (Operator Console)

- Create test memory
- View memory context
- Generate plan with memory (`diagnose_ai_plan_preview` — no execution)
- Clear test memory

---

## User control foundation

This batch provides inspectability and deletion. Later batches extend category disablement and retention controls (DEC-014).

---

## Related

- [Intelligence Roadmap](INTELLIGENCE-ROADMAP.md)
- [IPC Surface](../03-Engineering/IPC-SURFACE.md)
