# Operator Intelligence — Product Proof (P12.7)

| Field | Value |
| --- | --- |
| **Handoff** | `AWAITING_PROJECT_OWNER_OPERATOR_INTELLIGENCE_REVIEW` |

## Checklist (Conversation only)

| Check | Try |
| --- | --- |
| Discovery still works | “What windows are open?” |
| Application open still works | “Open notepad” |
| Clarification still honest | “Move this window.” |
| No provider jargon | Replies never say Provider / Runtime / Registry |
| Collapse / Save still work | Shell directives still function |

## Engineering Proof

- `OperatorRoot` does not call provider IPC  
- `pnpm verify:operator-intelligence` passes  
- Vitest covers Operator plan + authority boundary  

## Acceptance

Owner confirms Conversation still operates the desktop, and architecture places the Operator as the sole bridge.
