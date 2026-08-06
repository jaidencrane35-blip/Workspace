# Rolling Capability Provider Roadmap
## Updated after P12.6 Conversational Desktop Surface

UI Architecture, Desktop Operator, Conversation, and Capability Runtime pipeline remain **frozen**.  
Providers own **operations**. Providers never call each other.  
Every provider requires **Engineering Completion + Product Proof**.  
Conversation surface follows **Product Gravity** (P12.6) — providers plug into a product-ready conversational host.

---

## Completed (immutable repository truth)

| Program | Title | Commit |
| --- | --- | --- |
| **P10** | Capability Runtime Foundation + Clipboard | `23492a8` |
| **P11** | Application Provider | `330a26b` |
| **P12** | Window Provider (engineering) | `66465b8` |
| **P12.5** | Window Conversation Product Proof | `40dd746` |
| **P12.6** | Conversational Desktop Surface / Product Gravity | this milestone |

---

## Next five (rolling)

| Program | Title | Why |
| --- | --- | --- |
| **P13** | Notifications Provider | Lightweight outbound feedback — **must include Product Proof**; plugs into gravity surface |
| **P14** | Browser Provider | URL open (WRAP) |
| **P15** | Screenshot Provider | Capture under consent |
| **P16** | File Provider | Scoped FS for later automation |
| **P17** | Terminal Provider | Governed ConPTY |

Longer: P18 Voice → P19 Memory → P20 Automation → P21 Intelligence

---

## Permanent rules

1. Frozen Levels 1–3 (Product / Architecture / UI) stay frozen.  
2. Pipeline only — no bypasses.  
3. Providers own operations; independently testable.  
4. Every provider ends with Conversation Product Proof (`PRODUCT_PROOF_RULE.md`).  
5. One program → stop for Owner review.  
