# Window Lifecycle Specification

## Forms

| Form | Window label | Visible |
| --- | --- | --- |
| A Desktop Operator | `operator` | Yes when Form A |
| B Conversation | `main` | Yes when Form B |

## Transitions

```
Running → Form B Conversation (Product Gravity default)
Form B --Collapse|Close--> Form A Operator
Form A --click--> Form B
Form A|B --Exit--> Process end
```

Durable mode still restores the last form. Fresh / unset mode opens Conversation.

## Invariants

1. Close of `main` never exits the process.  
2. Collapse never resizes `main` into the operator.  
3. While Form A: `main` hidden + skip taskbar.  
4. While Form B: `operator` hidden.  
5. Geometry of `main` persisted independently of operator.  
6. Sizes normalized via `normalizeConversationSize` (compact-first).  
7. Conversation host is undecorated / transparent — presence on the desktop, not a framed utility.  

## Expanded Workspace (presentation)

Not a window. Dock flag on Form B (`data-dock="on"`). Conversation column remains.

## Exit

`exit_workspace` ends both windows. After Exit, no orphan Workspace processes.
