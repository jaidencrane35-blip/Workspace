# Engineering Milestone Report
## P12.6 Conversational Desktop Surface

| Field | Value |
| --- | --- |
| **Execution program** | P12.6 Conversational Desktop Surface |
| **Date** | 2026-08-07 |
| **Prior** | P12.5 Window Product Proof (`40dd746`) — not a provider program |
| **Commit** | pending |
| **Handoff** | `AWAITING_PROJECT_OWNER_PRODUCT_GRAVITY_REVIEW` |
| **Index** | `docs/ui/PRODUCT_GRAVITY_RULE.md` |

---

## Mission

Make Conversation finally feel like the product: a lightweight desktop companion where attention lands on the thread, the frame recedes, and Workspace exists on top of Windows — not as a framed utility.

---

## Permanent rules adopted

| Rule | Authority |
| --- | --- |
| **Product Gravity Rule** | `docs/ui/PRODUCT_GRAVITY_RULE.md` · protocol v1.3 |
| Conversation Complete ≠ Product Complete | Conversation must feel like desktop operation, not chat |

---

## Deliverables

| Artifact | Path |
| --- | --- |
| Product Gravity Rule | `docs/ui/PRODUCT_GRAVITY_RULE.md` |
| Conversation-first launch | `shellRuntime` / `bootstrapShellOnLaunch` default Form B |
| Transparent undecorated host | `app/src-tauri/tauri.conf.json` + `shellWindows` |
| Gravity surface presentation | `OperatorRoot` + `App.css` |
| Verifier | `pnpm verify:product-gravity` |

---

## User-visible

- Launch opens Conversation (not the floating W)
- Conversation sits on a translucent host without OS title bar
- Chrome whispers; transcript owns attention; composer focuses ready to speak
- Collapse still returns to the Desktop Operator companion

---

## Explicit non-goals

P13 Notifications · new providers · redesign Form A/B model · Conversation chrome IA expansion

---

## Stop

Wait for Product Owner Product Gravity review before P13.  
Acceptance: Conversation feels like desktop operation, not chat.
