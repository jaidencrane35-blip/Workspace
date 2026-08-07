# Voice Input — Regression Matrix
## P16.15 Product Completion

Engineering verification only. Does **not** equal Owner Product Complete.

| ID | Failure (Owner / measured) | Root cause | Permanent guard | Verifier / test |
| --- | --- | --- | --- | --- |
| R1 | First words lost after click | Listening UI before WinRT Capturing | Ready only on Capturing; Listening on SoundStarted | `verify-voice-input` · lifecycle logs |
| R2 | Mid-speech cut-off | Short EndSilence / RecognizeAsync | ContinuousRecognitionSession + stitch | proof `conversationContinuity` |
| R3 | UI freeze 5–30s | WinRT on IPC thread; status re-warm | `spawn_blocking`; status peek-only | `verify-voice-input` |
| R4 | Settings spam | Auto-open + bridge catch | Permission state machine; no auto-Settings | `permissionGuidance` checks |
| R5 | False mic deny forever | Sticky MediaCapture Denied | Soft probe; ConfirmedDenied only | `verify-voice-regression` |
| R6 | “Couldn’t listen” after success | MediaCapture during warm raced SpeechRecognizer | No MediaCapture in `warm_up` | `verify-voice-regression` |
| R7 | Cold recompile every quiet click | `engine_reset` on `no_speech` / `cancelled` | Keep engine on idle outcomes | `listen_idle_keep_engine` |
| R8 | Stale frontend warm skipped re-warm | Frontend `warmed` after native reset | No listen-path warm gate; clear on poison | `VoiceMicButton` + regression |
| R9 | Ready lag after Capturing | 45ms settle | 20ms settle once Capturing confirmed | proof `settleBeforeReadyMs` |
| R10 | Repeated warm IPC on click | Extra `warmUpVoice` before listen | Mount warm only; listen warms cheaply | `noListenPathWarmGate` |

### Engineering stress (non-Owner)

| Scenario | Method | Gate |
| --- | --- | --- |
| 100 consecutive listens | `memory_voice_survives_100_consecutive_listen_sessions` | Pass |
| Cancel / no_speech recovery | Keep engine; next Ready must be hot | Manual Owner + R7 |
| Permission once-per-cycle | Settings open only on mic click while denied | Manual Owner + R4 |

### Commodity foundation (revalidated P16.15)

| Candidate | Class | Notes |
| --- | --- | --- |
| WinRT SpeechRecognizer + ContinuousRecognitionSession | **WRAP** | Production foundation |
| Windows App SDK Speech | STUDY | Overlap with WinRT; no migration win |
| whisper.cpp / Sherpa-ONNX / Vosk | STUDY | Bundle / VAD cost |
| Azure / Web Speech | REJECT as default | Cloud / CSP |
| Kiro / VS Code / PowerToys | STUDY terminology | Aliases only — no code copy |
