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

**Fix (P16.8):** WRAP `SpeechContinuousRecognitionSession` with `AutoStopSilenceTimeout` at the WinRT maximum (10s). Mic toggle `StopAsync` finalizes.

### P16.9 Product Completion

| Finding | Cause | Fix |
| --- | --- | --- |
| First words / late Ready | Inviting speech at op-create; Ready conflated with Listening | Ready = Capturing + 90ms settle; Listening = SpeechDetected only; latency stays in Preparing |
| Mid-speech / long turns | AutoStop ends WinRT session | Stitch sessions until user Stop or 5 min wall |
| Settings spam | Auto-open on deny/fail | Permission Guidance — explain, user-driven open, remember grant |

### P16.10 stability (measured)

| Symptom | Root cause | Fix |
| --- | --- | --- |
| 5–30s before usable / UI freeze | `voice_listen_once` + `voice_status` ran WinRT `CompileConstraintsAsync` / `MediaCapture` on the Tauri IPC thread; focus/`status()` re-warmed | Listen/warm on `spawn_blocking`; `status()` never warms or probes mic; MediaCapture once inside `warm_up` only |
| Occasional Workspace crash | Long WinRT listen blocked async runtime; aggressive session stitch restart | Blocking listen off IPC thread; 280ms stitch gap; soft-fail stitch returns transcript |
| Settings still opening | `bridge.ts` catch path still called `openVoiceSettings` | Removed; Permission Guidance only |

### Commodity Survey (P16.10)

| Candidate | Class | Notes |
| --- | --- | --- |
| WinRT `SpeechRecognizer` + ContinuousRecognitionSession | **WRAP** | Keep; Workspace owns Ready contract + Permission Guidance |
| Web Speech API in WebView | **REJECT** | Wrong host; weaker desktop OS permission story |
| whisper.cpp / local STT | **STUDY** | Heavy; revisit only if WinRT WRAP fails Product Proof |
| Azure / cloud STT | **REJECT** | Not local-first default |
| Custom VAD ring buffer | **REJECT** | Reinvention; stitch + Ready contract preferred |

### P16.11 Evidence Before Commitment — complete commodity survey

**Principle (permanent):** before Workspace permanently adopts a major technology — research mature production implementations, understand the complete workflow, verify licensing, document tradeoffs, then classify ADOPT / WRAP / ADAPT / STUDY / REJECT.

Surveyed complete workflows (architecture, startup, mic lifecycle, buffering, continuous recognition, permissions, UI signalling, threading, init/cleanup, failure recovery, integration boundaries). No code copied; licenses respected.

| Repository / stack | License | Architecture lessons | Class | Why |
| --- | --- | --- | --- | --- |
| **WinRT `Windows.Media.SpeechRecognition`** (+ ContinuousRecognitionSession) | OS API (Windows) | Persistent engine; Capturing before UI; continuous session + stitch; OS permission model; keep heavy work off UI/IPC thread | **WRAP** | Local-first, OS mic/speech privacy, lowest integration cost, matches Conversation input-device role |
| **whisper.cpp** | MIT | Offline models; separate capture→buffer→decode pipeline; warm models off UI thread; excellent privacy | **STUDY** | Strong offline quality; large model bundle + GPU/CPU cost not justified while WinRT WRAP meets product |
| **Vosk** | Apache-2.0 | Streaming decoder; explicit mic open/close; grammar/models | **STUDY** | Streaming patterns useful; bundling + model management deferred |
| **Sherpa-ONNX** | Apache-2.0 | ONNX streaming ASR; VAD helpers; multi-backend | **STUDY** | Modern streaming reference; revisit if WinRT fails Owner Proof |
| **WinRT / Windows Speech platform (broader)** | OS | Same family as WRAP path | **WRAP** (current) | Already the foundation |
| **Windows Speech SDK / Azure Speech SDK** | Microsoft (cloud + key) | Cloud endpoints; subscription; excellent accuracy | **REJECT** (default) | Not local-first; CSP/privacy posture; keys |
| **Web Speech API** | Browser | Easy WebView hook; often cloud | **REJECT** (primary) | Wrong host; cloud leakage; weaker desktop permission story |
| Custom VAD + ring buffer before Capturing | — | Pre-buffer leading audio | **REJECT** (now) | Commodity Before Reinvention; Capturing gate + warm covers Owner cases |

**Architectural ideas adopted (patterns only — not code):**

1. Warm / compile recognizer off the UI and IPC threads (`spawn_blocking`).
2. Persistent engine instance across turns.
3. Do not invite speech until capture is actually retaining audio (Capturing contract).
4. Continuous recognition with explicit user Stop — not short end-silence as session death.
5. Permission: detect → explain → user-driven Settings → re-probe on return → remember grant → never auto-reopen.
6. Lifecycle logging at each boundary for evidence.

**Long-term foundation verdict:** **Yes — WRAP WinRT ContinuousRecognitionSession remains the correct long-term Conversation mic foundation** unless Owner Product Proof proves an irrecoverable OS limitation. Local STT (whisper/Vosk/Sherpa) stays STUDY for a future program if needed.

### P16.11 permission re-check root cause

`status()` is peek-only (P16.10). After a denied probe, returning from Settings still peeked **Denied** forever → “✓ Voice ready” never appeared.

**Fix:** `voice_recheck_permission` clears the mic cache and re-probes via MediaCapture once on `spawn_blocking`; mic UI calls it only when Settings guidance is active and the window becomes visible again; `open_settings` also clears the cache. Granted state is remembered in localStorage — future launches never auto-open Settings.

### P16.12 Technology Foundation Validation

**Principle (permanent):** before Workspace permanently adopts any foundational technology (speech, OCR, automation, memory, terminal, permissions, etc.), engineering must research mature implementations, understand the complete lifecycle, evaluate licensing / architecture / operational behaviour, document the choice, then classify ADOPT / WRAP / ADAPT / STUDY / REJECT.

#### Speech repositories (complete-workflow review)

| Technology | License | Commercial | Linkage / redistribution | Maintenance | Class | Why |
| --- | --- | --- | --- | --- | --- | --- |
| **WinRT `Windows.Media.SpeechRecognition`** | OS API | OK on Windows | No bundling | OS-maintained | **WRAP** | Local-first; OS mic + speech privacy; continuous session; fits Conversation input device |
| **whisper.cpp** | MIT | Permissive | Static/dynamic OK; attribute | Very active | **STUDY** | Excellent offline accuracy; large models / CPU-GPU cost; migration high |
| **Sherpa-ONNX** | Apache-2.0 | Permissive | NOTICE; patent grant | Active | **STUDY** | Streaming ONNX ASR + VAD patterns; revisit if WinRT fails Owner Proof |
| **Vosk** | Apache-2.0 | Permissive | NOTICE | Mature / slower cadence | **STUDY** | Streaming decoder reference; model packaging deferred |
| **Windows / Azure Speech SDK** | Microsoft terms | Cloud keys | Redistribution restricted | Active | **REJECT** (default) | Cloud-first; not local-first CSP/privacy posture |
| **Web Speech API** | Browser | N/A | N/A | Browser-dependent | **REJECT** (primary) | Wrong host; often cloud; weak desktop permission story |

#### Desktop permission architecture (studied — patterns only)

| Product | Lesson | Workspace adoption |
| --- | --- | --- |
| **VS Code** | Ask on first use of voice; remember OS grant; detect deny and guide to system Settings; package identity for mic listing | Explain once → Settings once → remember grant |
| **Windows Terminal** | Capability declarations; no spam on launch | Never open Settings on launch/warm |
| **PowerToys** | Per-module enable; settings pages owned by app where possible | Workspace guides to OS pages it does not own |
| **Kiro** (studied) | Conversational surface stays primary; permissions must not steal product gravity | Conversation remains the product; permission copy is short |

#### Connection architecture (why mature systems feel reliable)

```
Mic affordance → permission gate (once) → capture start → Ready only when retaining audio
  → partial/final transcripts → insert as typed text → Intent → Kernel Operator → Runtime → OS
```

Reliability comes from: (1) heavy work off UI thread, (2) persistent engine, (3) honest Ready signalling, (4) permission state machine that never loops, (5) Conversation treating voice identically to typing after transcript.

#### WinRT foundation recommendation (P16.12)

| | |
| --- | --- |
| **Verdict** | **WRAP WinRT ContinuousRecognitionSession remains the correct long-term Voice foundation** |
| **Strengths** | OS-integrated permissions; continuous recognition; no model bundle; local-first; lowest integration cost |
| **Weaknesses** | Occasional cold warm latency; Capturing gap before Ready; OS speech privacy prerequisite; less tunable than local ASR |
| **Migration cost** | High (models, packaging, VAD, permission redesign) — unjustified while WRAP meets Product Proof |
| **Architectural fit** | Voice as Conversation **input device** — not a Capability Provider |

Do **not** migrate in this program.

#### Permanent permission architecture (P16.12)

Session gate: `idle → explain → awaiting_return → (granted | still_denied)`.  
Settings opens **once per deny cycle** on explicit mic click. Return triggers `voice_recheck_permission`. Grant remembered in localStorage. Future launches never auto-open Settings.

### P16.13 Voice regression — root cause (false-deny cache)

**Owner findings:** mic “unavailable” while Windows mic worked; “I couldn’t listen just now”; 5–30s startup; failures after earlier successes.

**Exact failure (repository truth):**

1. `warm_up` ran `MediaCapture::Initialize` and **cached `MicAccess::Denied` permanently** on any access-shaped error (including transient device busy / COM race / concurrent warm).
2. `listen_once_when_ready` **hard-blocked** when cache was `Denied` — never reached `ContinuousRecognitionSession`.
3. Concurrent `startup_warm` + UI `voice_warm_up` contended on WinRT compile → multi-second / ~30s latency.
4. Failed sessions left a poisoned `SpeechRecognizer` while `warmed=true` → next clicks reused a dead engine.

**Timing path (intended after fix):**

| Stage | Work | Thread |
| --- | --- | --- |
| Mic click | UI → IPC | UI |
| `voice_listen_once` | `spawn_blocking` | blocking pool |
| `warm_lock` + compile (if cold) | WinRT CompileConstraints | blocking |
| Soft mic probe | MediaCapture; **Allowed sticky only** | blocking |
| Continuous start | Capturing wait ≤2.5s + 45ms settle | blocking |
| Ready / SoundStarted | events → UI | UI |
| Transcript → Conversation → Intent → Kernel | product path | UI / IPC |

**Architectural prevention:**

- Soft probe: never sticky-cache MediaCapture Denied (`mic_probe_denied_soft`)
- Listen hard-blocks only on `ConfirmedDenied` (SpeechRecognizer `MicrophoneUnavailable`)
- `warm_lock` serializes warm
- `engine_reset` after listen failure (idle recovery)
- Activity visualization remains SoundStarted-driven — WinRT continuous path exposes **no calibrated audio level API**

**Foundation verdict (revalidated):** WRAP WinRT ContinuousRecognitionSession remains correct. Local ASR stays STUDY. No migration in P16.13.

### P16.14 Technology Validation — foundation audit + mic race

#### Is WinRT still the correct production foundation?

| Criterion | WinRT Continuous | whisper.cpp / Sherpa / Vosk | Azure / Web Speech |
| --- | --- | --- | --- |
| Local-first / CSP | Yes | Yes | No / weak |
| OS mic + speech privacy | Native | Custom | Mixed |
| Continuous natural pauses | Yes (session + stitch) | DIY | Mixed |
| Bundle / model cost | None | High | Cloud |
| Conversation input-device fit | Excellent | Heavy | Poor |
| Migration cost now | — | Very high | Wrong default |

**Verdict: YES — WRAP WinRT ContinuousRecognitionSession remains the correct production foundation.**  
Migration is **not** objectively required. Local engines remain **STUDY**. Evidence: OS permission story, zero model packaging, continuous session already Product-Proofed, remaining defects were lifecycle races — not engine incapacity.

#### P16.14 microphone failure root cause

Owner message: “I couldn’t listen just now. Check that a microphone is connected.”

Two stacked defects after P16.13:

1. **MediaCapture warm race** — `warm_up` still ran MediaCapture soft-probe after compile. Initialize briefly exclusive-locks the mic; immediate `ContinuousRecognitionSession` start then fails with a generic HRESULT → sanitized to “couldn’t listen…”.
2. **Over-aggressive engine reset** — every `!ok` outcome (including `no_speech` / `cancelled`) called `engine_reset`, forcing cold `CompileConstraints` on the next click (multi-second / ~30s) and more start failures.

**Permanent prevention:**

- `warm_up` compiles only — **no MediaCapture**
- MediaCapture only on explicit `voice_recheck_permission`
- Keep engine on `no_speech` / `cancelled`
- Reset only on poison statuses (`recognition_failed` / `recognition_unavailable` / `microphone_unavailable`)
- `AudioQualityFailure` is `audio_quality` — never `ConfirmedDenied`
- Verifier: `verify-voice-regression.mjs` (P16.14 clauses)

#### End-to-end timing (healthy path)

| Stage | Expected |
| --- | --- |
| UI click → IPC | &lt; 50ms |
| spawn_blocking enter | immediate |
| warm (already compiled) | &lt; 20ms (lock + check) |
| Continuous start → Capturing | typically &lt; 500ms; timeout 2.5s |
| Ready settle | 0ms after Capturing (P16.25; was 20ms) |
| SoundStarted → Listening UI | event-driven |
| User Stop / silence stitch | continuous; AutoStop 10s stitches |

### P16.15 Product Completion — final technology validation

#### Objective

Transform Voice from Engineering Complete into Owner-reviewable Production Complete posture.  
No new providers. No architecture redesign. Evidence-based Product Proof remediations only.

#### Commodity comparison (revalidated)

| Candidate | Architecture | Permissions | Lifecycle | UX fit | Class |
| --- | --- | --- | --- | --- | --- |
| WinRT ContinuousRecognitionSession | OS speech | Native mic + speech privacy | Compile once → Capturing → Ready | Instant after warm | **WRAP** |
| Windows App SDK Speech | Overlaps WinRT | Similar | Similar | No clear win | STUDY |
| whisper.cpp / Sherpa-ONNX / Vosk | Local ASR | Custom | Model load / VAD | Heavy packaging | STUDY |
| Azure / Web Speech | Cloud | Cloud consent | Network | CSP / privacy fail | REJECT default |
| Kiro / VS Code / PowerToys | IDE / utilities | App-specific | N/A for STT | Terminology study only | STUDY aliases |

**Verdict: WinRT ContinuousRecognitionSession remains the objectively correct production foundation.**  
Indefinite keep-warm after compile is the WRAP strategy (engine retained across idle outcomes; reset only on poison). WinRT does not expose a separate “permanently armed” OS service — Workspace approximates permanent readiness by avoiding recompile / MediaCapture / Settings spam.

#### P16.15 Product Proof remediations

| Failure | Root cause | Correction | Validation |
| --- | --- | --- | --- |
| Extra click latency / desync after reset | Frontend awaited `warmUpVoice` when `!warmed`, and could skip after stale warm | Remove listen-path warm gate; clear `warmed` on poison | `noListenPathWarmGate` |
| Ready slightly late after Capturing | 45ms settle | 20ms settle | proof `settleBeforeReadyMs: 20` |
| “Open another browser” / desktop phrasing gaps | Incomplete deterministic aliases | Browser / Explorer / soften expansions | conversation-quality tests |
| Expansion pressure while Proof open | Missing permanent rule | **Production Before Expansion** | protocol + Product Proof Rule |

#### Permanent readiness answer

| Question | Answer |
| --- | --- |
| Feel permanently ready? | Yes after first warm — compile retained; listen hot path does not re-warm via separate IPC |
| First-word loss eliminated? | Engineering: Capturing contract + 20ms settle; Owner must confirm under stress |
| WinRT still correct? | **Yes** — WRAP; no migration |

Regression matrix: `docs/capability-runtime/product-proof/VOICE_REGRESSION_MATRIX.md`.

### P16.16 Final Product Proof Resolution

#### Objective

Remove uncertainty with measured evidence. Not feature expansion.

#### Foundation revalidation

| Candidate | Verdict |
| --- | --- |
| WinRT ContinuousRecognitionSession | **WRAP — remains correct** |
| Windows App SDK Speech | STUDY — no objective migration win |
| whisper.cpp / Sherpa / Vosk | STUDY — packaging / VAD cost |
| Azure / Web Speech | REJECT as default |
| Kiro / VS Code / PowerToys | Behavioural patterns only |

**Permanently alive session?** WinRT does not expose a consent-safe always-on capture service suitable for Conversation. Workspace keeps the **compiled recognizer** alive; each mic turn starts a **ContinuousRecognitionSession**. Ambient continuous listen remains REJECT.

#### Lifecycle timing (healthy hot path)

| Stage | Log marker | Expected |
| --- | --- | --- |
| UI click → IPC enter | `stage=ipc_listen_enter` | &lt; 50ms |
| spawn_blocking begin | `stage=ipc_listen_blocking_begin` | immediate |
| Warm (already compiled) | `stage=warm_done` | &lt; 20ms |
| Continuous start | `continuous_started` | typically &lt; 500ms |
| Capturing contract | `capturing_contract ready=true` | ≤ 2.5s |
| Ready emit | `on_ready_emitted click_to_ready=` | Capturing + 20ms |
| Sound / Listening UI | `speech_detected` / `voice-sound` | event-driven |

#### P16.16 measured defect

| Failure | Evidence | Root cause | Correction |
| --- | --- | --- | --- |
| First-word loss after “Ready” | Code path after `capturing_wait_timeout` still called `on_ready` | Ready lied when Capturing never confirmed | Abort listen with `capturing_contract_failed`; never emit Ready without Capturing |

Principles adopted: **Evidence Before Modification**, **Root Cause Before Rewrite**.

### P16.17 Production Readiness Audit

Hostile falsification of Voice-owned surfaces. Removed dead Voice IPC test hook, deprecated permission helpers, RecognizeAsync EndSilenceTimeout residue, and stale RecognizeAsync product docs.  

**WinRT ContinuousRecognitionSession remains WRAP.**  

Repository-wide `workspace-kernel` unused-item warnings are **DOCUMENT**ed Track A debt (not Voice-owned) — see `VOICE_PRODUCTION_READINESS_AUDIT.md`.  

Principles adopted: **Production Quality Includes Repository Quality**, **Evidence Before Completion**, **Repository Health Before Milestone Closure**.

### P16.18 Production Acceptance Investigation

Owner findings treated as repository truth. Engineering vs Owner disagreement audit:

| Finding | Root cause | Fix |
| --- | --- | --- |
| Mic unavailable after success | Sticky `ConfirmedDenied` on transient `MicrophoneUnavailable` | Soft recover; sticky only on true `permission_denied` |
| Long Ready | Listen warm raced startup warm (no lock) | `warm_lock` on listen warm |
| Permission confusion | UI mapped soft mic fail → Settings deny | Settings only on `permission_denied` |
| Browser → executable | “Chrome browser” / “launch browser” fallthrough | Canonicalize + bare browser open |

Matrix: `VOICE_PRODUCTION_FAILURE_MATRIX.md`. **WinRT remains WRAP.**

### P16.19 Production Closure Investigation

Final falsification before Owner acceptance:

| Challenge | Result |
| --- | --- |
| F1 residual — Access Denied sticky ConfirmedDenied | **FIX** — sticky only when message contains speech privacy; soft mic deny resets |
| Soft mic_unavailable after stale ConfirmedDenied | Clears stale sticky |
| Lifecycle / state machine | Documented `VOICE_LIFECYCLE_STATE_MACHINE.md` |
| Commodity (WinRT / Sherpa / whisper / Vosk / PowerToys / VS Code / Kiro / Terminal) | Revalidated — WRAP WinRT; no missing production behaviour that blocks P16 |
| 1000 consecutive MemoryVoicePort listens | Pass |

Closure report: `VOICE_PRODUCTION_CLOSURE_INVESTIGATION.md`. **WinRT remains WRAP.** No P17.

### P16.20 Final Product Proof Validation

Hostile challenge of P16.19 “no remaining defects”:

| Hole | Fix |
| --- | --- |
| ConfirmedDenied status mic copy / preparing | Speech privacy message on all ConfirmedDenied status paths |
| Recheck MediaCapture → false ✓ ready | Listen-confirm after Settings; grant only on explicit granted / successful listen |
| Soft Access Denied after Allowed → Settings cycle | Remap to `microphone_unavailable` |
| Guide softener miss | Intent Guide/Settings use `matchText` |

Final validation: `VOICE_FINAL_PRODUCT_PROOF_VALIDATION.md`. **WinRT remains WRAP.** No P17.

### P16.21 Owner-Experience Closure Gate

Engineering confidence ≠ evidence. Owner experience overrides assumptions until disproven.

| Hole | Fix |
| --- | --- |
| Soft first mic deny Settings trap | Remap all soft mic deny → unavailable; Settings after 2 fails |
| Dual event UI race | Single `voice-sound` emit |
| Invisible error phase | Brief error paint |
| Redundant listen warm | Skip when already warmed |

Gate report: `VOICE_OWNER_EXPERIENCE_CLOSURE_GATE.md`. Principles: Owner Experience Before Engineering Confidence; Repository Quality Before Milestone Closure. **WinRT remains WRAP.**

### P16.22 Product Proof Falsification & Ship Readiness

Attempted to prevent closure. Proven holes fixed:

| Hole | Fix |
| --- | --- |
| Status “Voice is ready” without Allowed | Prompt + set-up copy |
| Soft×2 Settings message desync | Always Settings guidance when arming |
| Settings-help copy without gate | Retry-only bridge/IPC copy |
| Warm-fail loses permission_denied | `listen_outcome_from_engine_error` + classify speech privacy phrase |

Report: `VOICE_SHIP_READINESS_FALSIFICATION.md`. **WinRT remains WRAP.**

### P16.23 Live Product Proof Instrumentation

Env-gated (`WORKSPACE_VOICE_PRODUCT_PROOF=1`) timing marks for Click→Capturing→Ready→speech→token, warm/reuse, reset reasons.  
Reports under `%TEMP%/workspace-voice-proof/`. Not shown to ordinary users. Removable after P16 closure.  
Harness timings measured on MemoryVoicePort; live WinRT timings await Owner session.  
Doc: `VOICE_LIVE_INSTRUMENTATION.md`. **WinRT remains WRAP.**

### P16.24 Final Live Product Proof Execution

Owner-feel chrome: Ready≠Listening; Preparing visibility; Error/Finished holds; permission copy never claims Ready at Idle; soft-fail vs Settings deny; onReady must not clobber Listening.  
Doc: `VOICE_FINAL_LIVE_PRODUCT_PROOF.md`. **WinRT remains WRAP.** Engineering Complete ≠ Product Complete.

### P16.25 Final Owner Readiness Investigation

Hostile falsification found and fixed: warm-fail soft remap (R32), hear-me NL soften (R33), soft-mic counter after Settings (R34), remove Ready settle after Capturing (R35), defer bridge callback teardown.  
Doc: `VOICE_FINAL_OWNER_READINESS_INVESTIGATION.md`. **WinRT remains WRAP.**

### P16.26 Final Live Product Proof Preparation

Canonical Owner checklist + finding→fix→regression traceability. R36 documents bridge teardown. Product Gravity cross-ref in Product Proof Rule.  
Docs: `VOICE_LIVE_PRODUCT_PROOF_PACKAGE.md`, `VOICE_OWNER_FINDINGS_TRACEABILITY.md`. **WinRT remains WRAP.**

---

## Explicit non-goals

Conversational AI · speech intelligence · dictation editor · hotword · provider execution from Voice · OCR of speech · cloud STT as default  
