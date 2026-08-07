# Voice Input — Product Proof
## P16 / P16.5 — Owner checklist

| Field | Value |
| --- | --- |
| **Program** | P16 Voice Input |
| **Status** | **Pending live Product Owner acceptance** (Engineering Complete; **not** permanently closed) |
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

## Closure stamp (Owner only)

Do **not** apply until live Product Proof is accepted by the Product Owner:

~~P16 Voice Input — PERMANENTLY CLOSED~~ (not yet)

Until then, repository health remains:

`P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING`
