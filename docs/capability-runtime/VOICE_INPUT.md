# Voice Input
## Product Implementation Program P16 — Levels 1–2 (+ P16.5 naturalness)

| Field | Value |
| --- | --- |
| **Status** | **Engineering Complete** — live Product Owner Product Proof **pending final acceptance**. **Not permanently closed.** |
| **Kind** | Conversation **input device** (not a desktop Capability Provider domain) |
| **Adoption** | WRAP WinRT `SpeechRecognizer` behind `VoicePort` |
| **Research** | `research/VOICE_RESEARCH.md` |
| **Independence** | Satisfies Capability Independence Rule |
| **Handoff** | `docs/project/ENGINEERING_HANDOFF.md` |

---

## Architecture

```
Mic button → VoicePort (warm WinRT speech) → transcript
    → Conversation composer → Intent → Kernel Operator → Providers → OS
```

Voice never invokes providers. Voice never owns orchestration.  
Kernel Operator is unchanged for recognition.

---

## Responsiveness (P16.5)

- Speech engine is **pre-warmed** at Workspace startup and on Conversation mount.
- Listening indicator appears only after `RecognizeAsync` has started (never during create/compile).
- Mic click does **not** await a heavy status round-trip before listening.
- Warm engine is reused across listen turns.

---

## Windows prerequisites (Owner)

Dictation requires Windows speech privacy acceptance and microphone access for Workspace.

On denial, Conversation explains the fix in ordinary language and opens the matching Settings page:

- Speech: Settings → Privacy & security → Speech  
- Microphone: Settings → Privacy & security → Microphone  

---

## Operations

| Level | Surface | Effect |
| --- | --- | --- |
| 1 | `voice_status` / `voice_warm_up` | Availability + pre-warm |
| 1 | `voice_recheck_permission` | Clear deny cache + re-probe after Settings return |
| 2 | `voice_listen_once` | Continuous turn; Ready on Capturing; Listening on speech |
| 2 | `voice_cancel` | Stop listening (finalize transcript) |
| 2 | `voice_open_settings` | Open Microphone or Speech privacy Settings (user-driven) |
| UI | Mic beside composer | Preparing / Ready / Listening; insert + submit |

---

## Ownership

| Layer | Location |
| --- | --- |
| Port | `packages/windows-integration/src/voice.rs` |
| IPC | `voice_*` commands |
| UI | `VoiceMicButton` in Conversation composer |
| Intent help | `voiceStatus` / `voiceExplain` (reply-only) |

---

## Closure rule

Permanent closure requires **Product Owner live Product Proof acceptance**.  
Engineering must not mark P16 permanently closed unilaterally.  
P17 must not begin until that acceptance is recorded in repository truth.

---

## Out of scope

Conversational AI · dictation editor · hotword · continuous ambient listen · cloud STT default · provider execution  
