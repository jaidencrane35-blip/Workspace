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

## Chrome inventory (Form B)

| Element | Keep? | Notes |
| --- | --- | --- |
| Brand “Workspace” | Yes, quiet | Not a hero dashboard title |
| Collapse | Yes | Mode switch to Operator |
| Exit | Yes | Process end — visually quieter than Collapse |
| Expand | No | Rejected as shell control |
| Settings | No | No catalogue surface |
| Health | Developer only | Hidden from product chrome |
| Teaching dock copy | No | Capability discovery via conversation only |

## Message presentation

- Workspace turns: plain text, minimal frame  
- User turns: slight contrast, not chat-bubble identity  
- Empty state: quiet void — no onboarding cards  

## Operator presentation

- Circular companion; transparent window host  
- No embedded conversation viewport  
- Click = open; drag = move  

## Productization checklist (Track A)

- [x] Compact default size  
- [x] Work-area clamp  
- [x] Collapse ≠ resize  
- [x] Click restore  
- [ ] Tray (debt)  
- [ ] Richer native show/hide motion (debt)  
