# Operator Authority Rule
## Permanent architectural law (P12.7)

| Field | Value |
| --- | --- |
| **Status** | Authoritative |
| **Program** | P12.7 Operator Intelligence Foundation |
| **Also in** | `.cursor/rules/constitutional-execution-protocol.mdc` |

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

---

## Pipeline (authoritative)

```
Conversation
    ↓
Intent Layer (language → intent)
    ↓
Operator Intelligence
    ↓
Capability Runtime (Router → Registry → Provider)
    ↓
Desktop Service / OS
    ↓
Operator (response composition)
    ↓
Conversation Response
```

No capability path may skip the Operator.

---

## Separation

| Layer | May | Must not |
| --- | --- | --- |
| Conversation | Present thread, composer, shell chrome; call Operator | Call providers / IPC effects |
| Operator | Decide, clarify, orchestrate, compose replies | Own OS effects or product chrome |
| Capability Runtime | Route & execute provider ops | Talk to the user |
| Providers | Domain operations via ports | Call each other or Conversation |
