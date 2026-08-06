# Operator Authority Rule
## Permanent architectural law (P12.7 · confirmed P12 Finalization)

| Field | Value |
| --- | --- |
| **Status** | Authoritative |
| **Program** | P12.7 · refined by P12 Finalization (Kernel) |
| **Also in** | `.cursor/rules/constitutional-execution-protocol.mdc` |
| **Companion** | `KERNEL_AUTHORITY_RULE.md` · `PRESENTATION_PURITY_RULE.md` |

---

## Rule

**Conversation never invokes providers directly.**  
**Conversation communicates only with the Operator.**

The Operator alone determines:

- whether execution should occur  
- whether clarification is required  
- which provider(s) participate  
- execution order  
- permission flow  
- response generation  
- truthful failure handling  

Providers remain completely independent and never become conversational.  
Conversation never contains operational logic.

**The Operator is the bridge.**  
**Production home of the Operator is the Kernel** (`packages/kernel/src/operator/`).

---

## Pipeline (authoritative)

```
Conversation (React)
    ↓
Intent Layer (TypeScript) → CapabilityIntent
    ↓
execute_capability_intent (single IPC)
    ↓
Kernel Operator
    ↓
Capability Runtime (Router → Registry → Provider)
    ↓
Operating System
    ↓
Kernel Operator (response composition)
    ↓
Conversation Response
```

No capability path may skip the Kernel Operator.

---

## Separation

| Layer | May | Must not |
| --- | --- | --- |
| Conversation | Present thread, composer, shell chrome; call façade | Call providers / IPC effects |
| Intent Layer | Utterance → CapabilityIntent | Plan, compose, invoke providers |
| Kernel Operator | Decide, clarify, orchestrate, compose replies | Own OS effects or product chrome |
| Capability Runtime | Route & execute provider ops | Talk to the user |
| Providers | Domain operations via ports | Call each other or Conversation |
