# Operator → Capability Runtime Protocol

The Operator is the **only** Conversation-side caller of Capability Runtime IPC.

| Domain | IPC | Notes |
| --- | --- | --- |
| clipboard | `read_clipboard` / `write_clipboard` | Via Operator runtime bridge |
| application | `execute_application_operation` | Via Operator runtime bridge |
| window | `execute_window_operation` | Via Operator runtime bridge |

Steps execute **in Operator-planned order**.  
Providers never see each other.  
Permission Gateway remains enforced inside Kernel CommandPipeline.
