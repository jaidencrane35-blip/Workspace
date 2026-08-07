# Voice Input — Regression Matrix
## P16.18 / P16.19 Production Closure

Engineering verification only. Does **not** equal Owner Product Complete.

See also: `VOICE_PRODUCTION_FAILURE_MATRIX.md`, `VOICE_LIFECYCLE_STATE_MACHINE.md`.

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
| R9 | Ready lag after Capturing | Overlong settle | 20ms settle once Capturing confirmed | proof `settleBeforeReadyMs` |
| R10 | Repeated warm IPC on click | Extra `warmUpVoice` before listen | Mount warm only; listen warms cheaply | `noListenPathWarmGate` |
| R11 | Ready UI without capture (first-word) | `capturing_wait_timeout` still emitted Ready | Fail listen if Capturing never confirmed; log `capturing_contract_failed` | `verify-voice-regression` P16.16 |
| R12 | Declared complete with Voice dead code | Noop test install + deprecated permission helpers | REMOVE dead Voice helpers; repository-quality principles | `VOICE_PRODUCTION_READINESS_AUDIT.md` |
| R13 | Mic unavailable after success | Sticky `ConfirmedDenied` on transient `MicrophoneUnavailable` | Soft recover; sticky only on speech privacy | `mic_unavailable_soft` |
| R14 | Long Ready / warm race | Listen warm without `warm_lock` | Serialize listen warm under `warm_lock` | listen path + `warm_lock` |
| R15 | Browser phrasing → exe | “Chrome browser” / “launch browser” fallthrough | Canonicalize + bare browser → `browserOpen` | conversation-quality tests |
| R16 | Sticky false deny via Access Denied | Mic `permission_denied` → ConfirmedDenied hard-block | Sticky **only** when message contains speech privacy; soft mic deny resets | `sticky_privacy_deny` · P16.19 |

### Engineering stress (non-Owner)

| Scenario | Method | Gate |
| --- | --- | --- |
| 1000 consecutive listens | `memory_voice_survives_1000_consecutive_listen_sessions` | Pass |
| Cancel / no_speech recovery | Keep engine; next Ready must be hot | Manual Owner + R7 |
| Permission once-per-cycle | Settings open only on mic click while denied | Manual Owner + R4 |
| Capturing timeout honesty | No Ready without Capturing | R11 |

### Permanent readiness (WinRT constraint)

| Layer | Permanently alive while Workspace runs? | Notes |
| --- | --- | --- |
| Compiled `SpeechRecognizer` | **Yes** (after warm; kept across idle) | WRAP keep-warm |
| `ContinuousRecognitionSession` | **No** | Starts per mic turn — ambient continuous listen is REJECT (Product Gravity / consent) |
| MediaCapture probe | **No on hot path** | Recheck only after Settings |

### Commodity foundation (revalidated P16.16)

| Candidate | Class | Notes |
| --- | --- | --- |
| WinRT SpeechRecognizer + ContinuousRecognitionSession | **WRAP** | Production foundation |
| Windows App SDK Speech | STUDY | Overlap with WinRT; no migration win |
| whisper.cpp / Sherpa-ONNX / Vosk | STUDY | Bundle / VAD cost |
| Azure / Web Speech | REJECT as default | Cloud / CSP |
| Kiro / VS Code / PowerToys / Windows Terminal | STUDY behaviour | Push-to-talk / permission once — aliases & UX patterns only |

### Adopted behavioural patterns (not code)

| Pattern | Source | Workspace adoption |
| --- | --- | --- |
| Push-to-talk, not ambient | PowerToys / desktop norms | Mic toggle starts/stops session |
| Explain permission once | Mature Windows apps | Permission state machine |
| Keep engine warm after first use | Local ASR practise | Compiled recognizer retained |
| Honest failure when capture missing | Production STT | R11 — no fake Ready |
