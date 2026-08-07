# Voice Input — Production Failure Matrix
## P16.18 / P16.19 Engineering vs Product Owner disagreement

Engineering falsification of “no remaining defects.” Does **not** equal Owner Product Complete.

| ID | Owner finding | Reproduced? | Classification | Evidence | Disposition |
| --- | --- | --- | --- | --- | --- |
| F1 | Mic unavailable after previous success | **YES** | Implementation defect | Transient `MicrophoneUnavailable` → sticky `ConfirmedDenied`; **P16.19**: mic `Access is denied` also sticky-blocked after success | **FIX** — soft `mic_unavailable`; **ConfirmedDenied only for speech privacy**; soft mic `permission_denied` resets without sticky |
| F2 | Long startup before Ready | **YES** | Implementation defect | Listen warm raced startup/UI warm (no `warm_lock` on listen); cold recompile after reset; Capturing ≤2.5s | **FIX** — serialize listen warm under `warm_lock`; keep Capturing contract |
| F3 | Occasional crashes | **PARTIAL** | WinRT / environmental | No panic on IPC path; residual COM/stitch risk under contention | **DOCUMENT** — mitigated by warm serialization; residual WinRT limitation |
| F4 | Inconsistent first-word capture | **YES (linked)** | Implementation + WinRT | Late Ready from F2; speech before Capturing discarded by design | **FIX** via F2; Capturing honesty (R11) retained |
| F5 | Inconsistent natural language | **PARTIAL** | Implementation defect | Deterministic gaps / soften edge cases | **FIX** incremental aliases; residual expectation mismatch possible |
| F6 | Browser phrasing → executable launch | **YES** | Implementation defect | “Chrome browser” / “launch browser” fell through to app launch shapes | **FIX** — strip browser suffix; bare launch browser → `browserOpen` |
| F7 | Permission confusion | **YES** | Implementation defect | F1 sticky deny + UI treated `microphone_unavailable` as Settings deny | **FIX** — Settings guidance only on `permission_denied` |
| F8 | Engineering declared complete too early | **YES** | Engineering blind spot | Optimism without falsifying sticky-deny / warm race | **DOCUMENT** — Evidence Before Completion; this matrix |

### Foundation revalidation

| Candidate | Verdict |
| --- | --- |
| WinRT ContinuousRecognitionSession | **WRAP — still correct** after F1/F2 fixes |
| whisper / Sherpa / Vosk | STUDY — would not remove WinRT permission/lifecycle classes |
| Ambient always-on session | REJECT |

### Assumptions proven incorrect

1. “MicrophoneUnavailable ⇒ permanent OS deny” — false; often transient.  
2. “Listen-time warm is safe without warm_lock” — false; races startup warm.  
3. “UI should treat microphone_unavailable like permission_denied” — false; causes Settings spam after success.  
4. “Any `permission_denied` ⇒ sticky ConfirmedDenied” — false; mic Access Denied can be transient COM/device contention (P16.19).

### Deferred (Track A — does not block Voice Product Proof)

- Kernel unused-item compiler warnings  
- Triplicated voice user-string sanitizers  
- Dual warm entry points (now serialized; still two callers)  
- Polished first-launch permission tour (current: truthful deny + once-per-cycle Settings) — Product UX polish, not a Voice lifecycle defect

### Must block P16 until Owner accepts

- Live Product Owner confirmation that F1–F7 feel resolved in Product Proof  

### P16.20 challenge of P16.19 “none proven”

| ID | Hole found | Disposition |
| --- | --- | --- |
| D1 | ConfirmedDenied `status()` used mic / preparing copy | **FIX** — speech privacy message |
| D2 | Settings recheck → ✓ ready from MediaCapture alone | **FIX** — listen confirm |
| D3 | Soft Access Denied after success → Settings cycle | **FIX** — remap unavailable |
| NL | Guide ignored softeners | **FIX** — matchText |

### P16.21 Owner-experience challenge of P16.20

| ID | Hole | Disposition |
| --- | --- | --- |
| P1 | Soft first mic deny → Settings before retry | **FIX** |
| U1 | Error phase never paints | **FIX** |
| U2 | Dual listening event race | **FIX** |
| C2 | Redundant second warm | **FIX** |

### P16.22 ship-readiness challenge of P16.21

| ID | Hole | Disposition |
| --- | --- | --- |
| S1 | False “Voice is ready” on unproven mic | **FIX** |
| S2 | Soft×2 Settings / “Try again” desync | **FIX** |
| S3 | Settings-help copy without Settings gate | **FIX** |
| S4 | Warm-fail permission misclassify | **FIX** |

### P16.24 Owner-feel lifecycle chrome

| Failure | Guard |
| --- | --- |
| Ready looks like Listening | Waveform only on speechDetected/listening |
| Preparing invisible | Distinct preparing fill + pulse |
| Error/Done blink | Longer holds + finished success chrome |
| “✓ Voice ready” at Idle | `voicePermissionSetMessage` |
| Soft fail = Settings deny look | `data-soft-fail` |
| Late Ready clobbers Listening | Preserve speech phases in onReady |

### P16.23 live instrumentation

Env-gated timing evidence for Owner Product Proof (`WORKSPACE_VOICE_PRODUCT_PROOF=1`).  
See `VOICE_LIVE_INSTRUMENTATION.md`.

### P16.26 canonical live package

Definitive Owner checklist: `VOICE_LIVE_PRODUCT_PROOF_PACKAGE.md`.  
Finding → fix → regression → evidence: `VOICE_OWNER_FINDINGS_TRACEABILITY.md`.

See also: `VOICE_SHIP_READINESS_FALSIFICATION.md`, `VOICE_OWNER_EXPERIENCE_CLOSURE_GATE.md`.
