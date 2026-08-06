# Operator State Machine

```
Idle
  → Interpreting   (utterance received)
  → Clarifying     (insufficient args — reply & stop)
  → Planning       (build OperatorPlan)
  → Executing      (Capability Runtime steps)
  → Responding     (compose reply)
  → Idle
```

| Transition | Guard |
| --- | --- |
| Interpreting → Clarifying | Clarification policy hit |
| Interpreting → Planning | Executable intent / composition |
| Interpreting → Responding | Shell directive or honest refusal (unknown) |
| Executing → Responding | Step success or failure (stop on failure) |

Terminal states always return an `OperatorOutcome` to Conversation.
