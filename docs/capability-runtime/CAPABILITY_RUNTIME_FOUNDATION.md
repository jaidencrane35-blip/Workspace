# Capability Runtime Foundation
## Product Implementation Program P10

| Field | Value |
| --- | --- |
| **Status** | Implemented — awaiting Product Owner review |
| **Prior** | P9 research accepted |
| **Reference provider** | Clipboard (WRAP `arboard`) |

---

## Frozen architecture

```
Conversation → Intent Layer → CapabilityIntent → execute_capability_intent
    → Kernel Operator → Capability Runtime → Capability Router
    → Provider Registry → Capability Provider → Desktop Service → OS
    → Kernel Operator (response) → Conversation Response
```

Presentation layer (UI Architecture Spec, Desktop Operator, Conversation chrome) is **stable**.  
Kernel Operator (P12 Finalization) is the sole Conversation→Runtime bridge. Capabilities extend Workspace; they never replace it.

---

## Artifacts

| Artifact | Path |
| --- | --- |
| Intent Layer Spec | `INTENT_LAYER_SPECIFICATION.md` |
| Capability Router Spec | `CAPABILITY_ROUTER_SPECIFICATION.md` |
| Provider Registry | `PROVIDER_REGISTRY.md` |
| Clipboard Provider | `CLIPBOARD_PROVIDER.md` |
| Five-program roadmap | `FIVE_PROGRAM_ROADMAP.md` |
| Contracts (updated) | `CAPABILITY_CONTRACTS.md` |
| Verifier | `pnpm verify:capability-runtime-foundation` |

---

## Workspace ownership (unchanged)

Identity · Conversation · Desktop Operator · Intent Layer · Capability Router · Contracts · Permissions · Audit · Governance · User Trust

External software may be ADOPT / ADAPT / WRAP / STUDY / REJECT — never product identity.

---

## Stop

Wait for Product Owner review before P11.  
