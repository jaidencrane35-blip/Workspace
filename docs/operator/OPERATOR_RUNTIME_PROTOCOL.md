# Operator → Capability Runtime Protocol

Conversation uses **one** governed entry:

| IPC | Owner |
| --- | --- |
| `execute_capability_intent` | Kernel Operator |

Inside the kernel, the Operator plans steps and executes each through CommandPipeline (Permission Gateway intact) into Capability Runtime → Router → Provider.

Provider-specific IPC may remain registered for diagnostics/legacy, but **must not** be called from Conversation or the TS façade.

Steps execute **in Kernel Operator-planned order**.  
Providers never see each other.
