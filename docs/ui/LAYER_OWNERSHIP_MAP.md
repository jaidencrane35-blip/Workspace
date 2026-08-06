# Layer Ownership Map

```
┌─────────────────────────────────────────────┐
│ Layer 1 — Desktop Operator     (Track A)    │
└──────────────────▲──────────────────────────┘
                   │ click / collapse
┌──────────────────┴──────────────────────────┐
│ Layer 2 — Conversation         (Track A)    │
│   ± Expanded presentation (satellite dock)  │
└──────────────────▲──────────────────────────┘
                   │ intents / permissions
┌──────────────────┴──────────────────────────┐
│ Layer 3 — Capability Runtime   (Track B)    │
└──────────────────▲──────────────────────────┘
                   │ memory / routing / adapt
┌──────────────────┴──────────────────────────┐
│ Layer 4 — Workspace Intelligence (Track C)  │
└─────────────────────────────────────────────┘
```

| Layer | May change shell? | May add primary UI? |
| --- | --- | --- |
| 1–2 | Yes (Track A only) | Shell surfaces only |
| 3 | No | Satellites / effects only |
| 4 | No | No — proposes via conversation |

Future voice, screenshots, clipboard, automation, memory, search, companion behaviour → Layers 3–4 through Conversation → Capability Runtime → Intelligence. **Never new primary interfaces.**
