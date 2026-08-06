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
Operator Intelligence (accept / clarify / plan / orchestrate)
        ↓
Capability Router
```

Intent Layer maps language. Operator decides. Conversation never skips the Operator.

---

## Ownership

| Surface | Owner |
| --- | --- |
| Phrase → intent mapping | TypeScript `app/src/lib/intentBridge.ts` |
| Shell / navigation intents | Operator presentation (existing) |
| Capability intents | Intent kinds that invoke Capability Runtime IPC only |
| Effect execution | **Forbidden** in Intent Layer |

---

## Capability intent kinds

### Clipboard (P10)

| Kind | Domain | Operation | IPC |
| --- | --- | --- | --- |
| `clipboardRead` | clipboard | read | `read_clipboard` |
| `clipboardWrite` | clipboard | write | `write_clipboard` |

### Application (P11)

| Kind | Domain | Operation(s) | IPC |
| --- | --- | --- | --- |
| `appOpen` | application | find → focus \| launch | `execute_application_operation` |
| `appLaunch` | application | launch | same |
| `appFocus` | application | focus | same |
| `appClose` | application | close | same |
| `appMinimize` | application | minimize | same |
| `appRestore` | application | restore | same |
| `appEnumerate` | application | enumerate | same |

Conversation examples:

- “Open notepad” / “Launch chrome” / “Switch to Chrome” / “Close Spotify” / “List apps”

### Window (P12 + P12.5 Product Proof)

| Kind | Domain | Operation(s) | IPC |
| --- | --- | --- | --- |
| `winEnumerate` | window | enumerate | `execute_window_operation` |
| `winActive` | window | active | same |
| `winMonitors` | window | monitors | same |
| `winBounds` | window | bounds | same |
| `winMaximize` | window | maximize | same |
| `winMinimize` | window | minimize | same |
| `winRestore` | window | restore | same |
| `winSnap` | window | snap | same |
| `winCenter` | window | center | same |
| `winMoveMonitor` | window | move + monitorIndex | same |
| `winFocus` | window | focus | same |
| `winResize` | window | resize | same |

Natural examples (Product Proof): “What windows are open?” / “Move this window to the left.” / “Center this window.” / “Maximize Cursor.” / “Restore Chrome.” / “Bring Chrome to the front.” / “Move Chrome to monitor two.”

Deixis: `this` / `this window` / bare operate verbs resolve to the active window inside Window Provider.  
Clarification: “Move this window.” / “Resize this window.” ask where/size — never invent effects.  
User-facing replies never mention providers.

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
