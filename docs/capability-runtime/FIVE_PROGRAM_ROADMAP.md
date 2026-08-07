# Rolling Capability Provider Roadmap
## Updated after P14 permanent closure

UI Architecture, Desktop Operator, Conversation, and Capability Runtime pipeline remain **frozen**.  
Providers own **operations**. Providers never call each other.  
Every provider requires the **Permanent Provider Acceptance Standard** (P15+).  
Conversation → Intent → **Kernel Operator** → Runtime (Kernel Authority).

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
| **P14** | Browser Provider | **Permanently closed** (`0ed6711` + finalization) |

---

## Next five (rolling)

| Program | Title | Why |
| --- | --- | --- |
| **P15** | Screenshot Provider | Next eligible — prep pack ready |
| **P16** | File Provider | Scoped FS |
| **P17** | Terminal Provider | Governed ConPTY |
| **P18** | Voice Provider | Spoken entry |
| **P19** | Memory Provider | Restoration |

---

## Permanent rules

1. Frozen pipeline.  
2. Providers own operations; no provider-to-provider calls.  
3. Product Proof + Natural Language Robustness.  
4. Provider Acceptance Standard (full sequence).  
5. P12 series — do not reopen.  
6. One program → Owner review → permanent closure → next.  
