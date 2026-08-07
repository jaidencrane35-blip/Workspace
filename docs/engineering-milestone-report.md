# Engineering Milestone Report
## P16.13 Voice Production Hardening & Product Proof Stabilization

| Field | Value |
| --- | --- |
| **Execution program** | P16.13 Voice Production Hardening & Product Proof Stabilization |
| **Date** | 2026-08-07 |
| **Status** | **Engineering Complete** — Product Complete **Owner-only** |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` |
| **Not** | P17 / Product Complete / permanently closed |

---

## Repository reassessment

- P10–P15 permanently closed.
- P16 Engineering Complete (through P16.13 hardening).
- P16 Product Proof **OPEN** — Owner launches Workspace when ready.
- P17 blocked.
- **Engineering Completion Gate (permanent):** engineering does not relaunch for Product Proof.

## Root cause (microphone regression)

Warm-time `MediaCapture` probe cached `Denied` permanently; listen hard-blocked on that cache while Windows mic still worked. Concurrent warm contended WinRT compile (~30s). Failed sessions left a poisoned recognizer with `warmed=true`.

## Fix

- Soft mic probe — never sticky-cache MediaCapture Denied  
- Listen hard-blocks only on `ConfirmedDenied` (SpeechRecognizer MicrophoneUnavailable)  
- `warm_lock` serializes warm  
- Engine reset after listen failure  
- Verifier: `pnpm verify:voice-regression`

## Explicit

- **P16 Product Complete:** **No — Owner only**  
- **P17:** Not begun  
- **Workspace left closed** for Owner-directed review  
