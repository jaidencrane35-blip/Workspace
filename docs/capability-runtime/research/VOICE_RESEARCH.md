# Voice Input Research (P16)

| Field | Value |
| --- | --- |
| **Program** | P16 Voice Input |
| **Status** | **Implemented — WRAP decision locked** |
| **Primary recommendation** | **WRAP** WinRT `SpeechRecognizer` (dictation) behind `VoicePort` |
| **Role** | Conversation **input device** — not a desktop Capability Provider |

---

## Problem

Users need to speak into Conversation with the same outcome as typing.  
Voice must never orchestrate desktop operations, never call providers, and never invent transcripts.

CSP forbids opening `connect-src` to cloud speech endpoints, so browser Web Speech cloud paths are unsuitable as the production path.

---

## Options evaluated

| Option | License / model | Offline | Privacy | Verdict |
| --- | --- | --- | --- | --- |
| **WinRT `Windows.Media.SpeechRecognition`** | OS API | Yes (language packs) | On-device preferred | **WRAP (primary)** |
| **Web Speech API** (`SpeechRecognition`) | Browser | Often cloud | Audio may leave device | **STUDY / REJECT as primary** (CSP + privacy) |
| **Windows Speech SDK / Azure Speech** | Microsoft | Cloud-first | Cloud | **STUDY only** (not local-first) |
| **Whisper** | MIT | Yes (local model) | Excellent | **STUDY** (bundle size / later) |
| **Vosk** | Apache-2.0 | Yes | Excellent | **STUDY** (bundle size / later) |
| **Legacy Windows Speech Recognition UI** | OS | Yes | On-device | **REJECT** (wizard / chrome) |
| **Always-on hotword** | — | — | High risk | **REJECT** for P16 |

---

## Decision matrix

| Option | Class | Rationale |
| --- | --- | --- |
| WinRT SpeechRecognizer behind `VoicePort` | **WRAP** | Commodity STT; Workspace owns Conversation insert + submit |
| Web Speech API | **REJECT** as primary | Conflicts with CSP / local-first posture |
| Azure Speech SDK | **STUDY** | Cloud dependency |
| Whisper / Vosk | **STUDY** | Offline excellence; defer model bundling |
| Hotword / ambient listening | **REJECT** | Product Gravity + consent |

**Headline:** WRAP WinRT dictation for single-utterance listen. Frontend mic is Conversation chrome only.

---

## Levels delivered (P16)

| Level | Capability | Effect |
| --- | --- | --- |
| 1 | `voice_status` | Mic / recognition / permission / input state |
| 2 | `voice_listen_once` | Push-to-talk single utterance → transcript |
| 2 | `voice_cancel` | Stop listen (best-effort) |
| UI | Mic button | Insert transcript → submit as typed |

Continuous listening deferred (not required for clean architecture).

---

## Product Proof remediation (speech privacy)

Owner mic click failed immediately with WinRT `RecognizeAsync` HRESULT `0x80045509` (speech privacy not accepted). Status probe only creates/compiles the recognizer, so it reported available while recognition could not start. Fix: map that OS failure (and similar) to desktop language via `classify_speech_failure`; never surface HRESULT / WinRT terms in Conversation.

---

## Product Proof remediation (P16.5 naturalness)

Owner found Voice “worked” but felt late: first words lost because Listening UI + user speech began during ~280 ms cold `SpeechRecognizer::new` plus a status IPC round-trip, before `RecognizeAsync` captured audio. Fix: pre-warm and reuse the compiled recognizer; show Listening only after capture starts; skip status on the listen hot path; detect mic denial and open the correct Windows Settings URI.

### P16.6 residual first-word loss

WinRT does not buffer audio before `RecognizeAsync()`. Occasional loss remained when (1) the frontend still awaited `listen("voice-listening")` on each click before invoke, and (2) the user clicked before warm completed. Remediation: hoist the listening event subscription; require warm completion before listen when cold; enlarge mic + explicit Idle/Preparing/Listening/Recognizing/Processing/Finished states. No calibrated audio-energy API on this WRAP path — listening waveform is activity indication only.

### P16.7 Capturing-contract failure (measured)

Owner waited ~2s after click and still lost first words → not merely frontend latency.

Lifecycle evidence (logged as `voice.lifecycle:*`):

1. `warm_done`  
2. `recognize_async_call` / `recognize_async_op_created` — **IAsyncOperation exists but State is still Idle**  
3. Gap until `SpeechRecognizerState::Capturing`  
4. Prior code called `on_ready` at step 2 → Listening UI lied; WinRT discarded audio until Capturing  

**Exact discard point:** audio spoken after click / after `RecognizeAsync()` returns the op, but **before** `SpeechRecognizerState::Capturing`.

**Fix (smallest production WRAP):** wait on `StateChanged` for `Capturing` (or `SpeechDetected` / `SoundStarted`) before emitting Ready/Listening; emit `voice-sound` on `SoundStarted` for live activity. Timeout fallback 2s if state never arrives.

### Commodity evaluation (P16.7)

| Candidate | Class | Notes |
| --- | --- | --- |
| WinRT `SpeechRecognizer` (current) | **WRAP** | Keep; gate UI on Capturing |
| WinRT ContinuousRecognitionSession | **WRAP** (P16.8) | Conversation Continuity — natural pauses must not end the session |
| Cloud STT / Whisper local | **REJECT** (now) | Wrong default for local-first Conversation mic |
| Custom VAD + ring buffer | **REJECT** (now) | Rebuild commodity; revisit only if Capturing gate fails Owner Proof |

### P16.8 premature session end (measured)

**Root cause:** `RecognizeAsync` + `EndSilenceTimeout` ≈ 2 seconds. Natural mid-speech pauses ended the turn while the Owner was still speaking — not frontend/Operator timeouts.

**Fix:** WRAP `SpeechContinuousRecognitionSession` with `AutoStopSilenceTimeout` at the WinRT maximum (10s post-speech silence = “user finished”). Mic toggle calls `StopAsync` (finalize transcript). Session accumulates `ResultGenerated` fragments. Capturing-contract Ready/Listening gate retained.

---

## Explicit non-goals

Conversational AI · speech intelligence · dictation editor · hotword · provider execution from Voice · OCR of speech · cloud STT as default  
