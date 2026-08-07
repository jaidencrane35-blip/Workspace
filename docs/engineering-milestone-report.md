# Engineering Milestone Report
## P16.7 Voice Capture Reliability & Product Completion

| Field | Value |
| --- | --- |
| **Execution program** | P16.7 Voice Capture Reliability & Product Completion |
| **Date** | 2026-08-07 |
| **Status** | **Engineering Complete** — live Product Owner Product Proof **pending** |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` |
| **Not** | P17 |

---

## Exact first-word failure point

WinRT discards audio until `SpeechRecognizerState::Capturing`.

Prior code emitted Listening when `RecognizeAsync()` returned an `IAsyncOperation` — **before** Capturing. Waiting after click did not help if the user spoke during that Idle?Capturing gap (or if UI invited speech early).

Lifecycle logs: `voice.lifecycle:*` (warm ? recognize_async_op_created ? capturing_contract ? on_ready_emitted).

---

## Fix

- Gate Ready/Listening on Capturing (`StateChanged`)
- `voice-sound` on SoundStarted for live activity
- Ready UI phase + Commodity Before Reinvention permanent

---

## Commodity

WinRT SpeechRecognizer remains **WRAP**. Continuous session **STUDY**. Custom VAD buffer **REJECT** for now.

---

## Explicit

- **P16 permanently closed:** **No — awaiting Owner**
- **P17:** Not begun
