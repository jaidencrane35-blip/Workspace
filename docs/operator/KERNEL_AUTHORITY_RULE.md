# Kernel Authority Rule
## Permanent law (P12 Finalization)

The **Kernel Operator** is the sole execution authority between Intent and Capability Runtime.

```
Conversation (React)
    ↓
Intent Layer (TypeScript) → CapabilityIntent
    ↓
Single IPC: execute_capability_intent
    ↓
Kernel Operator
    ↓
Capability Runtime → Router → Registry → Providers → OS
```

Every future provider after P12 shall execute through this authority.  
Conversation must not retain provider-specific execution paths.
