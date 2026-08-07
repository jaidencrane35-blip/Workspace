# Engineering Milestone Report
## P16.6 Voice & Conversation Product Proof Remediation

| Field | Value |
| --- | --- |
| **Execution program** | P16.6 Voice & Conversation Product Proof Remediation |
| **Date** | 2026-08-07 |
| **Status** | **Engineering Complete** — live Product Owner Product Proof **pending** |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` |
| **Not** | P17 |
| **Owner brief** | `docs/capability-runtime/product-proof/VOICE_INPUT_PRODUCT_PROOF.md` |

---

## Repository truth

| Item | Result |
| --- | --- |
| P10–P15 | Permanently closed |
| P16 | Engineering Complete — Product Proof **OPEN** |
| P17 | Blocked |

---

## First-word loss — root cause

Speech before `RecognizeAsync()` is never buffered by WinRT. Remaining gap sources:

1. **Event subscribe on the listen hot path** — `await listen("voice-listening")` before `invoke` delayed capture.  
2. **Click while engine still cold** — create/compile (~280?ms) during “Getting ready…”.

### Changes

- Hoist `voice-listening` subscription (`ensureVoiceListeningBridge`)  
- Finish warm-up before listen when `warmed` is false  
- Clearer mic phases + larger control + listening waveform (activity, not calibrated energy — WinRT path exposes none)

---

## Conversation / Operator

- GPT/tab phrasing ? browser (not `gpt tab.exe`)  
- `Open YouTube beside ChatGPT` ? `browserOpenBeside` (Kernel `browser.open_beside` composition)  
- Expanded intents + truthful unsupported for volume / single-tab close / minimize-all / transcription  

---

## Explicit

- **P16 permanently closed:** **No — awaiting Owner**  
- **P17:** Not begun  
