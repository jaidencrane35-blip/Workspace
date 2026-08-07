# Voice Input — Lifecycle & State Machine
## P16.19 Final Engineering Challenge

Engineering falsification document. Does **not** equal Owner Product Complete.

---

## Native engine states (`MicAccess` + warm)

```
Unknown ──soft probe Allowed──► Allowed
   │                                │
   │                         success listen
   │                                │
   ▼                                ▼
ConfirmedDenied ◄── speech privacy only (sticky)
   │
   └── cleared by: Settings recheck, successful listen, soft mic_unavailable recovery
```

| Transition | Race? | Stale? | Poison? | Notes |
| --- | --- | --- | --- | --- |
| Unknown → Allowed | No (warm_lock on probe) | No | No | Soft probe; Denied never sticky |
| * → ConfirmedDenied | No | Was F1 bug | No | **Speech privacy only** (P16.19) |
| mic_unavailable → reset | No | Clears stale ConfirmedDenied | Recover | Soft |
| permission_denied (mic) → reset | No | Not sticky | Recover | Soft (P16.19) |
| warm compile | Serialized `warm_lock` | No | Reset on poison | Listen + startup + recheck |

---

## Recognition turn (WinRT)

```
Idle → Preparing(IPC) → Warm(locked) → ContinuousStart → Capturing
     → Ready(UI) → SoundStarted → Listening → Stop/Silence → Recognizing
     → Transcript → Reviewing(composer) → (Owner Send → Conversation Processing)
     → or Cancel/Escape → Idle
     Mic chrome: Reviewing → Idle (draft retained until Send/clear)
```

| Stage | Failure mode | Product behaviour |
| --- | --- | --- |
| Warm fail | recognition_unavailable | Truthful; no Settings spam |
| Capturing timeout | capturing_contract_failed | Never fake Ready |
| Mic unavailable | soft reset | Retry without sticky deny |
| Speech privacy | sticky ConfirmedDenied | Explain once → Settings once |
| no_speech / cancel | keep engine | Hot next click |
| recognition_failed | engine_reset | Cold recompile next click |

---

## Frontend UI phases

`idle → preparing → ready → speechDetected → listening → recognizing → processing → finished → idle`  
(+ `error` flash)

| Risk | Mitigation |
| --- | --- |
| UI Ready before Capturing | Forbidden (R11) |
| Listen gated on stale `warmed` | Removed (R10) |
| Soft mic fail → Settings | Removed (R13/F7); Settings only after 2 soft fails (P16.21) |
| Dual listening events | Single `voice-sound` (P16.21) |
| Error phase flash | 280ms before Idle (P16.21) |
| Dual warm callers | Serialized under `warm_lock` |

---

## Hardware / OS events (expected behaviour)

| Event | Deterministic response |
| --- | --- |
| USB mic remove/replace | Soft fail or fail truthful; next click re-warms |
| Sleep / resume | Next listen may cold-warm; no auto Settings |
| Permission revoked in Windows | Fail truthful; Settings gate on true deny / privacy |
| Device busy during warm | Soft mic_unavailable; retry |

Ambient always-on session remains **REJECT** (Product Gravity / consent).

---

## F1 challenge result (P16.19 / P16.20)

| Path | Sticky false deny possible? |
| --- | --- |
| `MicrophoneUnavailable` status | **No** — soft |
| Mic Access Denied after prior Allowed | **No** — remapped `microphone_unavailable` (P16.20) |
| Mic Access Denied (never Allowed) | Soft reset; Settings guidance once (not sticky ConfirmedDenied) |
| Speech privacy declined | **Yes (intentional)** — OS policy until Settings |
| MediaCapture Denied | **No** — never sticky-cached |
| Hard-block on ConfirmedDenied | **Only privacy sticky** |
| Settings recheck → ✓ ready | **No** — listen confirm required (P16.20) |
| ConfirmedDenied `status()` copy | Speech privacy message (not mic / preparing) |

---

## Attempted breaks (engineering)

| Attack | Result |
| --- | --- |
| Sticky deny after soft mic fail | Blocked |
| Sticky deny after Access Denied | Blocked (P16.19) |
| Ready without Capturing | Blocked |
| Listen warm vs startup race | Serialized |
| 1000 MemoryVoicePort listens | Pass |
| Browser → .exe phrasing | Blocked (tests) |
