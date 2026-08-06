# Rolling Capability Provider Roadmap
## Updated after P12 Window Provider

UI Architecture, Desktop Operator, Conversation, and Capability Runtime pipeline remain **frozen**.  
Providers own **operations**. Providers never call each other.

---

## Completed (immutable repository truth)

| Program | Title | Commit |
| --- | --- | --- |
| **P10** | Capability Runtime Foundation + Clipboard | `23492a8` |
| **P11** | Application Provider | `330a26b` |
| **P12** | Window Provider | this milestone |

---

## Next five (rolling)

| Program | Title | Why |
| --- | --- | --- |
| **P13** | Notifications Provider | Lightweight outbound feedback |
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
4. Every program delivers user-visible value or unblocks it.  
5. One program → stop for Owner review.  
