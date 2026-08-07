# Voice Input — Production Failure Matrix
## P16.18 Engineering vs Product Owner disagreement

Engineering falsification of “no remaining defects.” Does **not** equal Owner Product Complete.

| ID | Owner finding | Reproduced? | Classification | Evidence | Disposition |
| --- | --- | --- | --- | --- | --- |
| F1 | Mic unavailable after previous success | **YES** | Implementation defect | Transient `MicrophoneUnavailable` → sticky `ConfirmedDenied` → hard-block all later listens | **FIX** — soft recover (`mic_unavailable_soft`); sticky only on true `permission_denied` |
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

### Deferred (Track A — does not block Voice Product Proof)

- Kernel unused-item compiler warnings  
- Triplicated voice user-string sanitizers  
- Dual warm entry points (now serialized; still two callers)

### Must block P16 until Owner accepts

- Live Product Owner confirmation that F1–F7 feel resolved in Product Proof  
