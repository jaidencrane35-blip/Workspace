# Voice Input
## Product Implementation Program P16 — Levels 1–2

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
Mic button → VoicePort (OS speech) → transcript
    → Conversation composer → Intent → Kernel Operator → Providers → OS
```

Voice never invokes providers. Voice never owns orchestration.  
Kernel Operator is unchanged for recognition.

---

## Operations

| Level | Surface | Effect |
| --- | --- | --- |
| 1 | `voice_status` | Availability, mic, recognition, permission, input state |
| 2 | `voice_listen_once` | Single utterance recognition |
| 2 | `voice_cancel` | Stop listening |
| UI | Mic beside composer | Listening indicator + pulse; insert + submit |

---

## Ownership

| Layer | Location |
| --- | --- |
| Port | `packages/windows-integration/src/voice.rs` |
| IPC | `voice_status` / `voice_listen_once` / `voice_cancel` |
| UI | `VoiceMicButton` in Conversation composer |
| Intent help | `voiceStatus` / `voiceExplain` (reply-only) |

---

## Out of scope

Conversational AI · dictation editor · hotword · continuous ambient listen · cloud STT default · provider execution  
