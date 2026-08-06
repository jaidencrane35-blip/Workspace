# Conversation → Operator Protocol

Conversation (OperatorRoot chrome) may only call:

```ts
handleOperatorUtterance(utterance: string): Promise<OperatorOutcome>
```

| Outcome | Conversation does |
| --- | --- |
| `reply` | Stream/push text into the transcript |
| `shell` | Apply presentation (mode, satellite, developer) |

Conversation **must not** import or invoke:

- `read_clipboard` / `write_clipboard`
- `execute_application_operation`
- `execute_window_operation`
- any future provider IPC

Intent resolution runs inside the Operator path (Intent Layer remains the language mapper; Operator owns acceptance).
