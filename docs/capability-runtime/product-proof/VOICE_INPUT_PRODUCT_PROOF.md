# Voice Input — Product Proof
## P16 — Owner checklist

| Field | Value |
| --- | --- |
| **Program** | P16 Voice Input |
| **Status** | **Product Complete** — remediation accepted (speech privacy failure mode) |
| **Role** | Conversation input device |
| **Harness** | `voice-input.proof.json` + `tests/voice-input-product-proof.test.ts` |
| **Verifier** | `scripts/verify-voice-input.mjs` |

---

## Owner checklist

1. Microphone button appears beside the Conversation composer  
2. Listening indicator / pulse while listening  
3. If Windows speech privacy is **off**: mic click shows ordinary-language privacy guidance (Settings → Privacy & security → Speech) — **no** HRESULT / WinRT / stack text  
4. After speech privacy is **on**: speak “Open ChatGPT.” → same as typed  
5. “Take a screenshot.” → same as typed  
6. “Bring Chrome to the front.” → same as typed  
7. “Open GitHub beside Cursor.” → same as typed  
8. Stop listening returns to idle  
9. Permission denied / mic missing / recognition failure → truthful reply  
10. No Provider / Runtime / WinRT / SpeechRecognizer terminology  
11. Kernel Operator path unchanged for desktop actions  

---

## Remediation note (Product Owner review failure)

| Item | Truth |
| --- | --- |
| **Exact error** | `The speech privacy policy was not accepted prior to attempting a speech recognition. (0x80045509)` |
| **First failing component** | WinRT `SpeechRecognizer::RecognizeAsync` |
| **Classification** | Operating System / Environment (Windows speech privacy) + Implementation (technical error reached Conversation; Product Proof never called live `RecognizeAsync`) |
| **Why PP missed it** | Harness proved Intent routing + mic chrome only; status probe stops at compile and does not exercise recognition |

---

## Closure stamp

**P16 Voice Input**  
**PERMANENTLY CLOSED**  
**ACCEPTED**  
**REPOSITORY TRUTH**  
**DO NOT REOPEN**  
(Bug fixes only.)
