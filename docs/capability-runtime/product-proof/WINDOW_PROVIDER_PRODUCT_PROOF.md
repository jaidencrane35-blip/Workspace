# Window Provider — Product Proof (P12.5)

| Field | Value |
| --- | --- |
| **Program** | P12.5 Conversation Integration & Product Proof |
| **Engineering base** | P12 Window Provider (complete — not reimplemented) |
| **Harness** | `window-provider.proof.json` |
| **Handoff** | `AWAITING_PROJECT_OWNER_WINDOW_PRODUCT_PROOF_REVIEW` |

---

## Product finding this program closes

Repository progressed. Product did not.  
Window Provider existed in the runtime but was not naturally usable through Conversation.  
P12.5 connects natural Conversation language to the completed provider.

---

## Product Owner review checklist

Verify entirely through Conversation (no menus, no docs required):

### Window discovery
- “What windows are open?”
- “Show me my open windows.”
- “Which window is active?”

### Window focus
- “Bring Chrome to the front.” (use any open app name)

### Window state
- “Maximize Cursor.”
- “Restore Chrome.”

### Window placement
- “Move this window to the left.”
- “Center this window.”
- “Move Chrome to monitor two.”

### Clarification & honesty
- “Move this window.” → asks where (does not invent a move)
- “Resize this window.” → asks for a size
- Named window that is not open → clear miss, not fake success

---

## Expected product experience

Workspace should feel like it gained a real desktop ability.  
Replies use ordinary language. They never mention providers, routers, or registries.

---

## Failure / permission notes

| Case | Expected |
| --- | --- |
| No match | “No matching window.” (or equivalent truthful miss) |
| Denied / runtime error | Surface the real failure text |
| Missing placement target | Clarifying question, no effect |

Permissions remain `window.read` / `window.focus` / `window.state` / `window.place`.

---

## Acceptance

P12.5 is accepted only when the Product Owner can successfully use Window Provider through Conversation.  
Only then may P13 become eligible.
