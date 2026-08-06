# Context Lifetime Policy

| Scope | Lifetime |
| --- | --- |
| Turn utterance | Single Operator turn |
| Clarification pending (future) | Until answered, cancelled, or Conversation collapses |
| Durable shell mode | Shell runtime — not Operator memory |
| Desktop state | OS / providers — Operator does not cache as truth |

P12.7 foundation: stateless per turn except shell presentation.  
Future memory compositions attach through Operator plans — not Conversation locals.
