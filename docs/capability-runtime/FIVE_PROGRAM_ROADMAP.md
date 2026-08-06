# Five-Program Execution Roadmap
## Designed in P10 — not executed beyond P10

UI Architecture, Desktop Operator, and Conversation remain **frozen**.  
Programs expand Capability Runtime only.

---

## Sequence

| Program | Title | Scope | Depends on |
| --- | --- | --- | --- |
| **P10** | Capability Runtime Foundation | Runtime freeze + Provider Registry + **Clipboard reference provider** | P8, P9 accepted |
| **P11** | Application Provider | Launch/focus via Conversation → Runtime (ADAPT Win32 ports) | P10 |
| **P12** | Window Provider | Placement / focus quality behind Runtime | P10, P11 recommended |
| **P13** | Notifications Provider | Sparse system feedback (WRAP Tauri notifications) | P10 |
| **P14** | Browser Provider | URL open only (WRAP `webbrowser`); CDP deferred | P10 |

---

## Order justification (vs nominal list)

Nominal Owner list placed Clipboard at P13. **P9 research** ranked Clipboard as the first V1 vertical (safe I/O, clear Conversation entry). P10 therefore ships Clipboard as the **sole reference provider** to prove the pipeline.

| Nominal | Adjusted | Reason |
| --- | --- | --- |
| P13 Clipboard | **Absorbed into P10** as reference | Prove runtime with highest-value thin slice |
| — | **P13 → Notifications** | Next P9 Phase 1 candidate after Clipboard |
| P11 Application / P12 Window | Unchanged | Owner priority for desktop operator value; builds on existing Win32 ADAPT ports |
| P14 Browser | Unchanged | P9 Phase 1 #3 |

This roadmap is **reassessed after every Owner review**. It is not a commitment to execute P11 next if reassessment says otherwise.

---

## Explicit non-execution

P11–P14 are **design only** in this milestone. No Application / Window / Notifications / Browser provider registration in P10.

---

## Exit for each future program

- Provider contract complete  
- Pipeline-only effects  
- Permission + audit coverage  
- UI Architecture verifier green  
- One milestone commit + Owner review  
