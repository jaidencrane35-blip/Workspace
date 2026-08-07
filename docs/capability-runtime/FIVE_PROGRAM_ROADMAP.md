# Rolling Capability Provider Roadmap
## Updated after P14.5 — P14 permanently closed

UI Architecture, Desktop Operator, Conversation, and Capability Runtime pipeline remain **frozen**.  
Providers own **operations**. Providers never call each other.  
Every provider requires **Engineering Completion + Product Proof** (including Natural Language Robustness).  
Conversation → Intent → **Kernel Operator** → Runtime (Kernel Authority).

---

## Completed (immutable repository truth)

| Program | Title | Commit |
| --- | --- | --- |
| **P10** | Capability Runtime Foundation + Clipboard | `23492a8` |
| **P11** | Application Provider | `330a26b` |
| **P12** | Window Provider (engineering) | `66465b8` |
| **P12.5** | Window Conversation Product Proof | `40dd746` |
| **P12.6** | Conversational Desktop Surface / Product Gravity | `d1dc8b8` |
| **P12.7** | Operator Intelligence Foundation (TS interim) | `644c125` |
| **P12 Finalization** | Kernel Operator + Presentation Purity | `daf3ae9` |
| **P12 COMPLETE** | Final Repository Closure | `030fa3f` |
| **P13** | Notifications Provider | `f0e03c4` (closed `f1a359c`) |
| **P14** | Browser Provider | `1670822` |
| **P14.5** | Browser Product Proof Remediation | `0ed6711` |

P12 series remains **permanently closed**.  
**P13 is permanently closed.**  
**P14 is permanently closed** (Product Proof + NL Robustness + Composition Audit).

---

## Next five (rolling)

| Program | Title | Why |
| --- | --- | --- |
| **P15** | Screenshot Provider | Prep pack ready — next eligible program |
| **P16** | File Provider | Scoped FS for later automation |
| **P17** | Terminal Provider | Governed ConPTY |
| **P18** | Voice Provider | Spoken operator entry |
| **P19** | Memory Provider | Workspace restoration |

Longer: P20 Automation → P21 Intelligence

---

## Permanent rules

1. Frozen Levels 1–3 stay frozen.  
2. Pipeline only — no bypasses.  
3. Providers own operations; independently testable.  
4. Every provider ends with Conversation Product Proof.  
5. Natural Language Robustness (Intent / Operator / replies only).  
6. One program → stop for Owner review.  
7. Kernel Operator is sole composition / execution authority after Intent.  
