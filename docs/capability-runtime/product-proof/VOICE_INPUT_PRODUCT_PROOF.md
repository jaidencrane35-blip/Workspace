# Voice Input — Product Proof
## P16 / P16.5 / P16.6 — Owner checklist

| Field | Value |
| --- | --- |
| **Program** | P16 Voice Input (+ P16.6 Conversation Product Proof Finalization) |
| **Status** | **Pending live Product Owner acceptance** (Engineering Complete incl. conversation quality; **not** permanently closed) |
| **Role** | Conversation input device |
| **Harness** | `voice-input.proof.json` + `tests/voice-input-product-proof.test.ts` + `tests/conversation-quality.test.ts` |
| **Verifier** | `scripts/verify-voice-input.mjs` · `scripts/verify-conversation-quality.mjs` |
| **Principle** | User Adaptation Prohibition (permanent) |

---

## Owner checklist

### Voice capture

1. Microphone button appears beside the Conversation composer  
2. After Workspace starts, Voice pre-warms in the background  
3. Click mic → brief “Getting ready…” only if needed; **Listening** pulse only when capture has started  
4. Speak naturally without waiting — **first word is captured**  
5. Full sentence appears as editable transcript in Conversation  
6. Submit behaves exactly like typed text  

### Conversation after transcription (P16.6)

7. **Testing testing 123** → acknowledges hearing (not “I don’t have that yet…”)  
8. **Sally sells seashells by the seashore** → acknowledges hearing  
9. **Open ChatGPT** / **Open GPT** / **Open a new GPT tab** → opens ChatGPT in the browser (**not** `gpt tab.exe`)  
10. **Open my browser** / **Open GitHub beside Cursor** / **Open Chrome** → desktop action  
11. **Take a screenshot** / polite “Could you take a screenshot?” → capture  
12. **What windows are open?** / **Bring Chrome to the front** → window operations  
13. Unsupported conversational sentence → truthful guidance + nearby suggestions; **not** identical repetitive fallback  
14. Permission / mic / recognition failures → ordinary-language guidance + Windows Settings when relevant  
15. No Provider / Runtime / WinRT / SpeechRecognizer / HRESULT terminology  
16. Kernel Operator path unchanged for desktop actions  

---

## Closure stamp (Owner only)

Do **not** apply until live Product Proof is accepted by the Product Owner:

~~P16 Voice Input — PERMANENTLY CLOSED — ACCEPTED — REPOSITORY TRUTH — DO NOT REOPEN~~ (not yet)

Until then, repository health remains:

`P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING`

After Owner acceptance, stamp:

`P16_PERMANENTLY_CLOSED_P17_ELIGIBLE`
