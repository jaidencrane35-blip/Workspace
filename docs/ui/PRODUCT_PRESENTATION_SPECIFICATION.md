# Product Presentation Specification
## Companion to UI Architecture Specification

| Field | Value |
| --- | --- |
| **Authority** | `docs/ui/UI_ARCHITECTURE_SPECIFICATION.md` |
| **Program** | P8 |

---

## Visual hierarchy

1. **Conversation transcript** — primary visual mass  
2. **Composer** — secondary, anchored  
3. **Quiet chrome** — Collapse / Exit / brand whisper  
4. **Satellite dock** — only when earned; never louder than conversation  

## Product Gravity (P12.6)

See `PRODUCT_GRAVITY_RULE.md`. Default attention belongs to Conversation.  
Conversation Complete ≠ Product Complete until the surface feels like desktop operation.

## Chrome inventory (Form B)

| Element | Keep? | Notes |
| --- | --- | --- |
| Brand “Workspace” | Yes, quiet | Not a hero dashboard title |
| Collapse | Yes | Mode switch to Operator — whisper control |
| Exit | Yes | Process end — quieter than Collapse |
| Expand | No | Rejected as shell control |
| Settings | No | No catalogue surface |
| Health | Developer only | Hidden from product chrome |
| Teaching dock copy | No | Capability discovery via conversation only |
| OS title bar | No (P12.6) | Undecorated transparent host; drag via quiet chrome |

## Message presentation

- Workspace turns: plain text, minimal frame  
- User turns: slight contrast, not chat-bubble identity  
- Empty state: quiet void — no onboarding cards  
- Composer: Enter sends; Send control stays whisper-level  

## Operator presentation

- Circular companion; transparent window host  
- No embedded conversation viewport  
- Click = open; drag = move  
- Idle / collapse surface — not the launch hero under Product Gravity  

## Productization checklist (Track A)

- [x] Compact default size  
- [x] Work-area clamp  
- [x] Collapse ≠ resize  
- [x] Click restore  
- [x] Conversation-first launch gravity (P12.6)  
- [x] Transparent undecorated conversation host (P12.6)  
- [ ] Tray (debt)  
- [ ] Richer native show/hide motion (debt)  
