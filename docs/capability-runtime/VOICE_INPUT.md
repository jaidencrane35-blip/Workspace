# Voice Input
## Product Implementation Program P16 — Levels 1–2 (+ P16.5 naturalness)

| Field | Value |
| --- | --- |
| **Status** | **PERMANENTLY CLOSED — ACCEPTED — REPOSITORY TRUTH — DO NOT REOPEN** (bugfixes only) |
| **Kind** | Conversation **input device** (not a desktop Capability Provider domain) |
| **Adoption** | WRAP WinRT `SpeechRecognizer` behind `VoicePort` |
| **Research** | `research/VOICE_RESEARCH.md` |
| **Independence** | Satisfies Capability Independence Rule |

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
| 2 | `voice_listen_once` | Single utterance; emits `voice-listening` when capturing |
| 2 | `voice_cancel` | Stop listening |
| 2 | `voice_open_settings` | Open Microphone or Speech privacy Settings |
| UI | Mic beside composer | Preparing vs Listening; insert + submit |

---

## Ownership

| Layer | Location |
| --- | --- |
| Port | `packages/windows-integration/src/voice.rs` |
| IPC | `voice_*` commands |
| UI | `VoiceMicButton` in Conversation composer |
| Intent help | `voiceStatus` / `voiceExplain` (reply-only) |

---

## Out of scope

Conversational AI · dictation editor · hotword · continuous ambient listen · cloud STT default · provider execution  
