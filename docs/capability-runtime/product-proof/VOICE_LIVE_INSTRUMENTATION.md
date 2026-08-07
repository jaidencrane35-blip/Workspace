# Voice Input — Live Product Proof Instrumentation
## P16.23

Temporary, removable instrumentation bridging engineering confidence and Owner live experience.  
**Not** Product Complete. Workspace was **not** launched by engineering.

---

## Enable (Owner / engineer live session only)

```text
WORKSPACE_VOICE_PRODUCT_PROOF=1
```

Optional logging: `RUST_LOG=info` or `WORKSPACE_DEV_LOG=1`.

Ordinary users never see instrumentation UI or Conversation jargon.  
Disable by unsetting the env var. **Remove this module after P16 permanently closes.**

---

## What is recorded

| Mark | Meaning |
| --- | --- |
| `mic_click` | Listen session begin (IPC / port entry) |
| `engine_available` | Port accepted listen |
| `warm_start` / `compile_start` / `compile_finish` | Cold recognizer compile |
| `warm_already` | Recognizer reused |
| `capturing_entered` | WinRT Capturing (or harness equivalent) |
| `ready_emitted` | Ready contract fired |
| `first_speech_detected` | SoundStarted / SpeechDetected |
| `first_token_received` | First recognition text piece |
| `transcript_completed` | Successful turn |
| `stop_requested` | User cancel / mic toggle stop |
| `engine_reset:<reason>` | Poison recovery |
| `permission_*` / `recovery:*` | Classification notes |

**Measured intervals (ms):** Click→Capturing · Click→Ready · Ready→First speech · First speech→First token · Total recognition.

Reports: `%TEMP%/workspace-voice-proof/session-<id>.json` + `sessions.jsonl`  
Logs: `voice.proof.report: …`

---

## Engineering timing report (harness — objective)

Source: `product_proof_instrumentation_measures_memory_listen_intervals` + `voice_proof` unit tests  
Platform: MemoryVoicePort (deterministic; not WinRT hardware)

| Metric | Cold listen | Warm reuse listen | Notes |
| --- | --- | --- | --- |
| Click → Capturing | ≤1 ms | ≤1 ms | Harness immediate Capturing |
| Click → Ready | ≤1 ms | ≤1 ms | Harness immediate Ready |
| Ready → First speech | ≤1 ms | ≤1 ms | Harness SoundStarted |
| First speech → First token | ≤1 ms | ≤1 ms | Harness transcript |
| Recognizer recreated | **Yes** (cold) | **No** | Second listen `recognizer_reused=true` |
| Unnecessary recreations | **None** on warm path | — | Evidence: warm report |

### Live WinRT timings (Owner session)

| Metric | Measured value |
| --- | --- |
| Click → Capturing | **Pending live Owner session** (`WORKSPACE_VOICE_PRODUCT_PROOF=1`) |
| Click → Ready | **Pending live Owner session** |
| Ready → First speech | **Pending live Owner session** |
| First speech → First token | **Pending live Owner session** |

Engineering cannot invent WinRT hardware timings without a launch. Instrumentation is ready to record them when the Product Owner requests review.

---

## Lifecycle falsification (instrumented)

| Challenge | Evidence |
| --- | --- |
| Recognizer reuse | Warm second listen marks `warm_already` / `recognizer_reused` |
| Duplicate warm | Skip when warmed (`warm_already`); compile marks once |
| Duplicate listening events | Single `voice-sound` (P16.21) held |
| Stale permission cache | Soft remap + ConfirmedDenied privacy-only held |
| Race Ready without Capturing | `capturing_contract_failed` held |
| Poison recovery | `engine_reset:<reason>` recorded when reset |

No new Voice-owned defect was proven by adding instrumentation.

---

## Owner finding evidence table

| Owner finding | Reproduced | Fixed | Objectively measured | Engineering evidence | Owner confirmation |
| --- | --- | --- | --- | --- | --- |
| Mic unavailable after success | Yes | YES | Soft remap + no sticky ConfirmedDenied | Failure matrix F1; R13/R16/R19 | Live |
| Long Ready / warm race | Yes | YES | `warm_lock` + warm_already marks | R14; proof warm path | Live |
| First-word / Ready honesty | Yes | YES | Capturing before Ready | R11; `capturing_entered`→`ready_emitted` | Live |
| Settings spam / soft deny trap | Yes | YES | Soft×2 gate + privacy sticky only | R19/R23 | Live |
| False “Voice is ready” | Yes | YES | Status prompt until Allowed | R22 | Live (“Can you hear me?”) |
| Browser → exe | Yes | YES | Conversation tests | intent-bridge | Live |
| Crashes under contention | Partial | DOCUMENT | WinRT residual | F3 | Live |
| Warm-fail permission lose | Yes | YES | `listen_outcome_from_engine_error` | R24 | Live |

---

## Commodity (final)

WinRT ContinuousRecognitionSession **WRAP — keep**.  
Local ASR STUDY. Ambient always-on REJECT.  
Instrumentation does not change foundation.

---

## Removal checklist (after P16 Owner acceptance)

1. Delete `packages/windows-integration/src/voice_proof.rs`  
2. Remove `voice_proof` marks from `voice.rs`  
3. Remove env docs / verifier / this document (or archive)  
4. Drop `serde_json` if unused elsewhere in the crate  
