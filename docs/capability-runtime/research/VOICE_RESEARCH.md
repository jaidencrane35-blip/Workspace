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

## Explicit non-goals

Conversational AI · speech intelligence · dictation editor · hotword · provider execution from Voice · OCR of speech · cloud STT as default  
