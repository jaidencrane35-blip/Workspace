# Voice Input — Product Proof
## P16 / P16.25 — Owner checklist (Final Owner readiness)

| Field | Value |
| --- | --- |
| **Program** | P16 Voice Input (+ P16.25 Final Owner Readiness Investigation) |
| **Status** | **Pending live Product Owner acceptance** (**not** Product Complete) |
| **Launch** | **Do not launch until Owner requests** |
| **Engineering brief** | `VOICE_FINAL_OWNER_READINESS_INVESTIGATION.md` |

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

## Please stress these paths (feel first)

1. Click mic — **Preparing** should be obvious, then **Ready** (green) before you speak  
2. Speak — **Listening** (cyan + bars) must feel different from Ready  
3. Finish — brief **Done** (✓) before idle  
4. Soft mic fail once → amber retry chrome; twice → Settings guidance  
5. Permission message must say click mic to speak — never claim ready while idle  
6. Ordinary desktop phrasing — never inventing `.exe`  
7. Cancel mid-listen; immediate second listen (warm)  

When finished, **close Workspace normally**. Engineering will **not** relaunch.

---

## Closure stamp (Owner only)

~~P16 Voice Input — PERMANENTLY CLOSED — ACCEPTED — REPOSITORY TRUTH — DO NOT REOPEN~~ (not yet)

Until acceptance: `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING`  
