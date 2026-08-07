# Rolling Capability Provider Roadmap
## Updated for repository handoff (P16 Product Proof pending)

UI Architecture, Desktop Operator, Conversation, and Capability Runtime pipeline remain **frozen**.  
Providers own **operations**. Providers never call each other.  
Voice Input is a Conversation **input device** (not a desktop provider domain).  
Every provider / input capability requires the **Permanent Provider Acceptance Standard**.  
Conversation → Intent → **Kernel Operator** → Runtime (Kernel Authority).  
**Capability Independence Rule** is permanent.

---

## Completed (immutable repository truth)

| Program | Title | Status |
| --- | --- | --- |
| **P10** | Capability Runtime Foundation + Clipboard | Closed |
| **P11** | Application Provider | Closed |
| **P12** | Window Provider | Closed |
| **P12.5** | Window Product Proof | **ACCEPTED — DO NOT REOPEN** |
| **P12.6** | Product Gravity | Closed |
| **P12.7** / Finalization / COMPLETE | Kernel Operator series | Closed |
| **P13** | Notifications Provider | Permanently closed |
| **P14** | Browser Provider | Permanently closed |
| **P15** | Screenshot Provider | Permanently closed |

---

## Current

| Program | Title | Status |
| --- | --- | --- |
| **P16** | Voice Input | **Engineering Complete** — live Product Owner Product Proof **pending final acceptance**. **Not permanently closed.** |

---

## Next five (rolling — after P16 acceptance)

| Program | Title | Why |
| --- | --- | --- |
| **P17** | File Provider | Next eligible after P16 Owner acceptance — scoped FS |
| **P18** | Terminal Provider | Governed ConPTY |
| **P19** | Memory Provider | Restoration |
| **P20** | Automation Provider | Multi-step recipes |
| **P21** | Workspace Intelligence | Continuity surface |

---

## Permanent rules

1. Frozen pipeline.  
2. Providers own operations; no provider-to-provider calls.  
3. Product Proof + Natural Language Robustness.  
4. Provider Acceptance Standard (full sequence).  
5. Capability Independence Rule.  
6. P12–P15 — do not reopen except bugfixes. P16 — do not reopen design; finish Owner Product Proof only.  
7. One program → Owner review → permanent closure → next.  
8. **Do not begin P17 until P16 is permanently closed.**  
