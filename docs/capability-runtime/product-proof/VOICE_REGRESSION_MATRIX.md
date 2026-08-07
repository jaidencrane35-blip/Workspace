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
| R9 | Ready lag after Capturing | Overlong settle | Was 20ms settle; **superseded by R35** (0ms after Capturing) | proof `settleBeforeReadyMs` · R35 |
| R10 | Repeated warm IPC on click | Extra `warmUpVoice` before listen | Mount warm only; listen warms cheaply | `noListenPathWarmGate` |
| R11 | Ready UI without capture (first-word) | `capturing_wait_timeout` still emitted Ready | Fail listen if Capturing never confirmed; log `capturing_contract_failed` | `verify-voice-regression` P16.16 |
| R12 | Declared complete with Voice dead code | Noop test install + deprecated permission helpers | REMOVE dead Voice helpers; repository-quality principles | `VOICE_PRODUCTION_READINESS_AUDIT.md` |
| R13 | Mic unavailable after success | Sticky `ConfirmedDenied` on transient `MicrophoneUnavailable` | Soft recover; sticky only on speech privacy | `mic_unavailable_soft` |
| R14 | Long Ready / warm race | Listen warm without `warm_lock` | Serialize listen warm under `warm_lock` | listen path + `warm_lock` |
| R15 | Browser phrasing → exe | “Chrome browser” / “launch browser” fallthrough | Canonicalize + bare browser → `browserOpen` | conversation-quality tests |
| R16 | Sticky false deny via Access Denied | Mic `permission_denied` → ConfirmedDenied hard-block | Sticky **only** when message contains speech privacy; soft mic deny resets | `sticky_privacy_deny` · P16.19 |
| R17 | Wrong Settings / false ready after privacy | ConfirmedDenied status used mic copy; recheck treated MediaCapture as grant | Speech privacy message on status; `permission_recheck_needs_listen_confirm`; grant only on explicit granted / successful listen; `permission_denied_soft_after_success` | P16.20 |
| R18 | Guide softener miss | “What can you do for me?” → unknown | Guide + Settings matchers use softened `matchText` | conversation-quality |
| R19 | Soft first mic deny → Settings trap | `permission_denied` always entered Settings gate | Remap soft mic → unavailable; Settings after 2 soft fails | P16.21 |
| R20 | Dual emit Ready/Listening race | `voice-sound` + `voice-listening` same callback | Single `voice-sound`; SoundStarted-only UI | P16.21 |
| R21 | Error phase never painted | Immediate idle after error | 280ms error flash | P16.21 |
| R22 | Status “ready” without mic proof | Unknown warmed → “Voice is ready.” | Prompt + set-up copy until Allowed | P16.22 |
| R23 | Soft×2 Settings message desync | Arm gate but say “Try again.” | Always Settings guidance when arming | P16.22 |
| R24 | Warm fail loses permission_denied | Double sanitize → recognition_unavailable | `listen_outcome_from_engine_error` · `listen_warm_failed_classified` | P16.22 |
| R25 | Engineering confidence without live evidence | Owner repeatedly found post-“complete” defects | Env-gated `voice_proof` timing reports; removable after P16 | P16.23 |
| R26 | Ready looks like Listening | Waveform on Ready (`activeCapture`) | Wave only on speechDetected/listening; Ready = green ring/pulse | P16.24 |
| R27 | Preparing nearly invisible | Idle-like preparing chrome | Distinct preparing fill + pulse; reduced-motion safe | P16.24 |
| R28 | Error / Done blinks away | 280ms error / 180ms finished | Hold error ~720ms, finished ~480ms + success chrome | P16.24 |
| R29 | “✓ Voice ready” at Idle | Permission grant announced as Ready | `voicePermissionSetMessage` — click mic to speak | P16.24 |
| R30 | Soft fail looks like Settings deny | Shared `!` / orange / Settings aria | `data-soft-fail` + distinct labels | P16.24 |
| R31 | Late Ready clobbers Listening | Unconditional `setPhase("ready")` | Preserve speechDetected/listening in onReady | P16.24 |
| R32 | Warm-fail Access Denied → Settings trap | Early warm return skipped soft remap | `apply_listen_failure_policy` on warm-fail + post-listen | P16.25 |
| R33 | “Could you hear me?” → unknown | Voice matcher missed softened hear-me | Expand patterns + matchText retry | P16.25 |
| R34 | Soft×2 count stuck after Settings | Counter never reset on return/open | Reset `softMicDenyCountRef` on Settings open/return | P16.25 |
| R35 | Ready lag after Capturing | Unnecessary 20ms settle | Immediate Ready after Capturing (`settleBeforeReadyMs: 0`) | P16.25 |
| R36 | Late Ready/Listening UI drop | Bridge deleted callbacks when IPC returned | Defer callback teardown 120ms (`setTimeout`) | P16.26 |
| R37 | Concurrent double-listen race | No listen mutex; phase lag | `warm_lock` for whole listen + `listenInFlightRef` | P16.27 |
| R38 | False “I opened Settings” | Gate armed before open; errors swallowed | Open first; `Promise<boolean>`; truth on fail | P16.27 |
| R39 | Product-copy reclassify landmine | Bare `"access"` matched “allow access” | Classify on denied/unavailable phrases only | P16.27 |
| R40 | False available via recognitionAvailable | `available \|\| recognitionAvailable` | Use `status.available` only | P16.27 |
| R41 | NL compounds → executable names (F11) | appOpen fallthrough used raw compound as query | Intent Grammar + Kernel `open_foreground` / `open_maximize` + appOpen compound safety net | `intent-bridge` F11 tests · P16.30 |
| R42 | Voice auto-submits without review (F10) | `onVoiceTranscript` called `submitUtterance` | Draft-only + Send/Escape; mic `reviewing` phase | OperatorRoot · VoiceMicButton · P16.30 |
| R43 | Explorer folder locate failed / wrong exe | `shell:` treated as `.exe` alias | Launch `explorer.exe` + `shell:` arg | application_provider · P16.30 |
| R44 | Fast speech WER blamed on Workspace without WRAP evidence (F9) | Assumed fixable in Intent/UI | Document inherent WinRT limit; review→Send mitigation; no migrate without evidence | `VOICE_FAST_SPEECH_INVESTIGATION.md` · P16.30 |

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
