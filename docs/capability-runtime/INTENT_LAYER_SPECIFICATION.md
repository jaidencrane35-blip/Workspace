# Intent Layer Specification
## Capability Runtime Foundation (P10) · P12 Finalization

| Field | Value |
| --- | --- |
| **Status** | Authoritative for Track B entry |
| **Layer** | Intent Layer (utterance → CapabilityIntent) |
| **UI law** | Frozen — Intent never invents a new primary interface |
| **Execution** | Kernel Operator via `execute_capability_intent` only |

---

## Role

The Intent Layer maps Conversation utterances to **CapabilityIntent**. It does not perform desktop effects, plan steps, or choose providers.

```
Conversation utterance
        ↓
Intent Layer (deterministic bridge today; model later)
        ↓
CapabilityIntent { domain, operation, arguments }
        ↓
execute_capability_intent (single IPC)
        ↓
Kernel Operator (accept / clarify / plan / orchestrate)
        ↓
Capability Runtime → Router → Provider → OS
```

Intent Layer maps language. Kernel Operator decides. Conversation never skips the Operator.

---

## Ownership

| Surface | Owner |
| --- | --- |
| Phrase → intent mapping | TypeScript `app/src/lib/intentBridge.ts` |
| Intent → CapabilityIntent | TypeScript `app/src/lib/operator/intentMap.ts` |
| Shell / navigation intents | Presentation (existing) |
| Effect execution / composition | **Kernel Operator only** |

---

## Capability intent kinds

All capability kinds map to one IPC: **`execute_capability_intent`**.

### Clipboard (P10)

| Kind | Domain | Operation |
| --- | --- | --- |
| `clipboardRead` | clipboard | read |
| `clipboardWrite` | clipboard | write |

### Application (P11)

| Kind | Domain | Operation |
| --- | --- | --- |
| `appOpen` | application | open *(Kernel composes find → focus \| launch)* |
| `appLaunch` | application | launch |
| `appFocus` | application | focus |
| `appClose` | application | close |
| `appMinimize` | application | minimize |
| `appRestore` | application | restore |
| `appEnumerate` | application | enumerate |

### Window (P12 + P12.5 Product Proof)

| Kind | Domain | Operation |
| --- | --- | --- |
| `winEnumerate` | window | enumerate |
| `winActive` | window | active |
| `winMonitors` | window | monitors |
| `winBounds` | window | bounds |
| `winMaximize` | window | maximize |
| `winMinimize` | window | minimize |
| `winRestore` | window | restore |
| `winSnap` | window | snap |
| `winCenter` | window | center |
| `winMove` | window | move |
| `winResize` | window | resize |
| `winFocus` | window | focus |
| `winFind` | window | find |

### Notifications (P13)

| Kind | Domain | Operation |
| --- | --- | --- |
| `notifyStatus` | notifications | status |
| `notifyShow` | notifications | show |
| `notifyDismiss` | notifications | dismiss |

Deferred “notify me when…” watching clarifies truthfully (not Level 1–2).

### Browser (P14)

| Kind | Domain | Operation |
| --- | --- | --- |
| `browserStatus` | browser | status |
| `browserOpen` | browser | open |
| `browserOpenBeside` | browser | open_beside (Operator → Window snap) |

Site aliases (ChatGPT, Google, GitHub, YouTube, Bing) and raw `https://` URLs. Missing website → clarify. App opens remain Application Provider.

Provider-specific IPC remains registered for diagnostics/legacy but **must not** be called from Conversation or the TS Operator façade.
