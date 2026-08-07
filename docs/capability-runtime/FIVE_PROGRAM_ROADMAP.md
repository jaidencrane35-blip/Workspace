# Rolling Capability Provider Roadmap
## Updated after P15 permanent closure

UI Architecture, Desktop Operator, Conversation, and Capability Runtime pipeline remain **frozen**.  
Providers own **operations**. Providers never call each other.  
Every provider requires the **Permanent Provider Acceptance Standard**.  
Conversation → Intent → **Kernel Operator** → Runtime (Kernel Authority).  
**Capability Independence Rule** is permanent (P15).

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
| **P15** | Screenshot Provider | **PERMANENTLY CLOSED — ACCEPTED — DO NOT REOPEN** |

---

## Next five (rolling)

| Program | Title | Why |
| --- | --- | --- |
| **P16** | Voice Input | Next eligible — spoken entry into Conversation |
| **P17** | File Provider | Scoped FS |
| **P18** | Terminal Provider | Governed ConPTY |
| **P19** | Memory Provider | Restoration |
| **P20** | Automation Provider | Multi-step recipes |

---

## Permanent rules

1. Frozen pipeline.  
2. Providers own operations; no provider-to-provider calls.  
3. Product Proof + Natural Language Robustness.  
4. Provider Acceptance Standard (full sequence).  
5. Capability Independence Rule.  
6. P12 / P13 / P14 / P15 series — do not reopen except bugfixes.  
7. One program → Owner review → permanent closure → next.  
