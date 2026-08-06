# Rolling Capability Provider Roadmap
## Updated after P10 Owner review → P11

UI Architecture, Desktop Operator, Conversation, and Capability Runtime pipeline remain **frozen**.  
Providers own **operations**. Programs expand providers only.

---

## Completed

| Program | Title | Status |
| --- | --- | --- |
| **P10** | Capability Runtime Foundation + Clipboard reference | Complete — Owner accepted direction |
| **P11** | Application Provider (operations) | Active / this milestone |

---

## Next five (rolling)

| Program | Title | Why |
| --- | --- | --- |
| **P12** | Window Provider | Placement / geometry quality; pairs with Application ops |
| **P13** | Notifications Provider | Lightweight outbound communication |
| **P14** | Browser Provider | URL open (WRAP); CDP deferred |
| **P15** | Screenshot Provider | Capture WRAP behind consent |
| **P16** | File Provider | Scoped FS — later automation dependency |

---

## Longer horizon

P17 Terminal → P18 Voice → P19 Memory deepen → P20 Automation → P21 Workspace Intelligence

Clipboard remains the P10 reference provider (not re-implemented).

---

## Permanent rules

1. Do not redesign frozen Levels 1–3 (Product / Architecture / UI).  
2. Do not bypass Conversation → Intent → Router → Registry → Provider → Desktop → Response.  
3. Providers own operations (Launch, Focus, …), not isolated one-off features.  
4. One execution program at a time; stop for Owner review.  
