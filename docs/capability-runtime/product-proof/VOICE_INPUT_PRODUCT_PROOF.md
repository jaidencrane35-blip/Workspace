# Voice Input — Product Proof
## P16 / P16.5 — Owner checklist

| Field | Value |
| --- | --- |
| **Program** | P16 Voice Input (+ P16.5 naturalness remediation) |
| **Status** | **Product Complete** — immediate listen + first-word capture |
| **Role** | Conversation input device |
| **Harness** | `voice-input.proof.json` + `tests/voice-input-product-proof.test.ts` |
| **Verifier** | `scripts/verify-voice-input.mjs` |

---

## Owner checklist

1. Microphone button appears beside the Conversation composer  
2. After Workspace starts, Voice pre-warms in the background  
3. Click mic → brief “Getting ready…” only if needed; **Listening** pulse only when capture has started  
4. Speak naturally without waiting — **first word is captured**  
5. Full sentence appears as editable transcript in Conversation  
6. Submit behaves exactly like typed text  
7. Permission / mic / recognition failures → ordinary-language guidance + Windows Settings opened when relevant  
8. No Provider / Runtime / WinRT / SpeechRecognizer / HRESULT terminology  
9. Kernel Operator path unchanged for desktop actions  

---

## P16.5 latency findings (measured)

| Stage | Cold | Warm (after pre-warm) |
| --- | --- | --- |
| WinRT `SpeechRecognizer::new` | ~280 ms | n/a (reused) |
| Compile constraints | ~3 ms | n/a (reused) |
| Second create+compile | — | ~8 ms |
| Prior UI bug | Listening shown during status probe + create (~300 ms+) before `RecognizeAsync` | Fixed |

**First-word loss root cause:** UI marked Listening and the user began speaking while status IPC + recognizer create/compile still ran; `RecognizeAsync` had not started capturing yet.

---

## Closure stamp

**P16 Voice Input**  
**PERMANENTLY CLOSED**  
**ACCEPTED**  
**REPOSITORY TRUTH**  
**DO NOT REOPEN**  
(Bug fixes only.)
