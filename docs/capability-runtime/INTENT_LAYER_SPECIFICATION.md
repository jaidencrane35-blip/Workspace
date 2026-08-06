# Intent Layer Specification
## Capability Runtime Foundation (P10)

| Field | Value |
| --- | --- |
| **Status** | Authoritative for Track B entry |
| **Layer** | Intent Layer (between Conversation and Capability Router) |
| **UI law** | Frozen — Intent never invents a new primary interface |

---

## Role

The Intent Layer maps Conversation utterances to **capability intents**. It does not perform desktop effects.

```
Conversation utterance
        ↓
Intent Layer (deterministic bridge today; model later)
        ↓
Capability intent { domain, operation, arguments }
        ↓
Capability Router
```

---

## Ownership

| Surface | Owner |
| --- | --- |
| Phrase → intent mapping | TypeScript `app/src/lib/intentBridge.ts` |
| Shell / navigation intents | Operator presentation (existing) |
| Capability intents | Intent kinds that invoke Capability Runtime IPC only |
| Effect execution | **Forbidden** in Intent Layer |

---

## Capability intent kinds (P10)

| Kind | Domain | Operation | IPC |
| --- | --- | --- | --- |
| `clipboardRead` | clipboard | read | `read_clipboard` |
| `clipboardWrite` | clipboard | write | `write_clipboard` |

Conversation examples:

- “What’s on my clipboard?”
- “Copy to clipboard: …” / “Clipboard write: …”

---

## Laws

1. Intent Layer **must not** call Desktop Services or OS APIs.
2. Intent Layer **must not** bypass Capability Router / Provider Registry.
3. Unknown intents remain honest refusals — never invent effects.
4. Future providers add intent kinds; they do not redesign Conversation chrome.

---

## Future

- Structured `IntentEnvelope` shared Rust/TS contract  
- Voice utterance → same Intent Layer  
- Model-assisted parsing still terminates in Workspace-owned intent kinds  
