# Conversation → Operator Protocol

Conversation (OperatorRoot chrome) may only call:

```ts
handleOperatorUtterance(utterance: string): Promise<OperatorOutcome>
```

| Outcome | Conversation does |
| --- | --- |
| `reply` | Stream/push text into the transcript |
| `shell` | Apply presentation (mode, satellite, developer) |

The façade maps Intent → `CapabilityIntent` and invokes **only**:

```ts
execute_capability_intent({ intent })
```

Conversation **must not** import or invoke provider-specific IPC:

- `read_clipboard` / `write_clipboard`
- `execute_application_operation`
- `execute_window_operation`

Intent Layer owns utterance → CapabilityIntent. Kernel Operator owns execution.
