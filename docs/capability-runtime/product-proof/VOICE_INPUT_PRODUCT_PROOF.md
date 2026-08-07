# Voice Input — Product Proof
## P16 / P16.23 — Owner checklist (Live instrumentation)

| Field | Value |
| --- | --- |
| **Program** | P16 Voice Input (+ P16.23 Live Product Proof Instrumentation) |
| **Status** | **Pending live Product Owner acceptance** (**not** Product Complete) |
| **Launch** | **Do not launch until Owner requests** |

---

## Before live review (engineer / Owner)

Set for the Product Proof session only:

```text
WORKSPACE_VOICE_PRODUCT_PROOF=1
```

Optional: `RUST_LOG=info` to see `voice.proof.report` lines.  
Reports land in `%TEMP%/workspace-voice-proof/`.  
Ordinary Conversation UI stays free of instrumentation jargon.

---

## Please stress these paths

1. Cold first listen — note Ready feel; engineering will read Click→Ready from the report  
2. Immediate second listen — should feel warm (reuse)  
3. Soft mic fail once → retry; twice → Settings guidance  
4. “Can you hear me?” before first success — must not falsely claim ready  
5. Ordinary desktop phrasing — never inventing `.exe`  
6. Cancel mid-listen; long pause dictation  

When finished, **close Workspace normally**. Engineering will **not** relaunch.

---

## Closure stamp (Owner only)

~~P16 Voice Input — PERMANENTLY CLOSED — ACCEPTED — REPOSITORY TRUTH — DO NOT REOPEN~~ (not yet)

Until acceptance: `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING`  
