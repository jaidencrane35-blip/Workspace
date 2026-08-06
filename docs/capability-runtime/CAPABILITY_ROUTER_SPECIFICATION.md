# Capability Router Specification
## Capability Runtime Foundation (P10)

| Field | Value |
| --- | --- |
| **Status** | Authoritative |
| **Rust** | `packages/kernel/src/capability_runtime/router.rs` |
| **Entry** | Kernel `CommandPipeline` → Router → Provider |

---

## Permanent pipeline

```
Conversation
    ↓
Intent Layer
    ↓
Capability Router
    ↓
Provider Registry
    ↓
Capability Provider
    ↓
Desktop Service (port / windows-integration)
    ↓
Conversation Response
```

**No capability may bypass this pipeline.**

---

## Responsibilities

| Component | Does | Does not |
| --- | --- | --- |
| **Capability Router** | Resolve domain → provider; forward invoke | Touch OS; grant permissions |
| **Provider Registry** | Own provider set; one provider per domain | Execute effects |
| **Capability Provider** | Domain logic via ports | Own Conversation / product identity |
| **CommandPipeline** | Permission Gateway + audit | Skip Router for Track B effects |

---

## Routing contract

Request: `ProviderInvokeRequest { domain, operation, text? }`  
Response: `ProviderInvokeResponse { ok, format?, bytes?, text?, preview?, message? }`

Missing provider → `KernelError::CapabilityRuntime`.

---

## Permission & audit

- Commands declare `required_capability` (`clipboard.read` / `clipboard.write`).
- Gateway validates before Router invoke (via CommandPipeline).
- Audit records command + capability; write metadata includes format/length/preview — **not** full secrets by default.

---

## Extension rule

New domains register a provider. They never add a parallel effect path from Conversation or Intent.  
