# Operator Intelligence Foundation
## Completed by P12 Finalization (Kernel Operator)

| Field | Value |
| --- | --- |
| **Status** | Authoritative — Kernel Operator is production authority |
| **Not** | AGI · planning · autonomous agents |
| **Is** | Deterministic Kernel authority: validate → plan → execute → compose |
| **Code** | `packages/kernel/src/operator/` + `commands/capability_intent.rs` |
| **IPC** | `execute_capability_intent` (sole Conversation execution entry) |
| **TS façade** | `app/src/lib/operator/` — Intent map + single IPC only |

---

## Permanent architecture

```
Conversation (React)
    ↓
Intent Layer (TypeScript)
    ↓
CapabilityIntent
    ↓
execute_capability_intent
    ↓
Kernel Operator
    ↓
Capability Runtime → Router → Registry → Providers → OS
```

---

## Decision record

| Option | Verdict |
| --- | --- |
| TypeScript Operator owning composition (P12.7 interim) | **SUPERSEDED** — moved to Kernel |
| Kernel Operator + single IPC | **ADOPT** (P12 Finalization) |
| Providers grow conversational APIs | REJECT |

---

## Rules

- Operator Authority  
- Kernel Authority  
- Presentation Purity  
- Capability Composition  
- Product Proof · Product Gravity  
