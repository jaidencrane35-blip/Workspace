# Voice Input — Owner-Experience Closure Gate
## P16.21

Product experience is the success metric. Engineering confidence is not evidence.  
Workspace was **not** launched. P16 is **not** permanently closed.

---

## 1. Repository reassessment

| Truth | Status |
| --- | --- |
| P10–P15 | Permanently closed |
| P16 Engineering | Complete through **P16.21** |
| P16 Product Proof | **OPEN** |
| P17 | Blocked |
| Architecture | Frozen |

Permanent principles added/confirmed: **Owner Experience Before Engineering Confidence**, **Repository Quality Before Milestone Closure**.

---

## 2. Final hostile production audit (vs P16.20)

| Challenge | Evidence | Result |
| --- | --- | --- |
| Soft first mic Access Denied → Settings before retry | UI `notePermissionDenied` on every `permission_denied` | **FIX** — remap all soft mic deny to unavailable; Settings after 2 soft fails |
| Error mic phase never paints | `setPhase(error)` then immediate idle | **FIX** — 280ms error paint |
| Dual `voice-sound` + `voice-listening` race | IPC emitted both; UI hooks raced | **FIX** — single `voice-sound` emit; SoundStarted-only UI sequencing |
| Soft-unavailable Settings-promise copy drift | Multiple messages | **FIX** — unified “Try again.” |
| Redundant second warm on listen | warm under lock then again inside continuous | **FIX** — skip when already warmed |
| Soft probe ConfirmedDenied empty fallthrough | Could MediaCapture-overwrite sticky privacy | **FIX** — early return ConfirmedDenied |
| Idle vs Ready same glyph | Both `◉` | **FIX** — idle `○` / Ready `◉` |
| Mic pulse ignores reduced motion | CSS | **FIX** |
| Capturing honesty / warm_lock / sticky privacy status | Re-audit | **Hold** |
| Browser→exe / Guide softeners | Tests | **Hold** |
| Sleep/USB/COM under load | No live hardware in engineering | **DOCUMENT** — Owner live Proof |

---

## 3–4. Foundation / commodity

WinRT ContinuousRecognitionSession **WRAP — keep**.  
Sherpa / whisper / Vosk STUDY. Ambient always-on REJECT.  
No missing industry behaviour that blocks P16.

---

## 5. State machine

Lifecycle remains authoritative. P16.21: soft mic → unavailable; Settings after consecutive soft fails; single speech-energy event; error flash before Idle.

---

## 6. Permission experience

| Scenario | Class |
| --- | --- |
| Soft mic fail → retry | Voice — fixed |
| 2× soft mic → Settings | Voice — fixed |
| Speech privacy sticky + correct page | Voice — held |
| Settings return listen confirm | Voice — held |
| Polished tour | Track A |
| Windows Settings UI | OS limitation |

---

## 7. Conversation quality

Challenge set held (GPT / browser beside / focus / screenshot / hear me / softeners). No new NL defects proven.

---

## 8. Production hardening

1000 Memory sessions hold. Soft Settings trap closed. Residual WinRT COM DOCUMENT.

---

## 9. Repository quality (Voice-owned)

| Item | Disposition |
| --- | --- |
| Soft Settings trap / error paint / dual emit / second warm / ConfirmedDenied probe | **Resolved** |
| Triplicated sanitize strings | DOCUMENT — defence in depth; Track A consolidate |
| Dual warm callers (startup + mount) | DOCUMENT — safe under warm_lock |
| IPC `input_state` unused by UI | DOCUMENT — harness wire; Track A |
| Trait `listen_once` default | DOCUMENT — tests/stubs |
| Kernel unused warnings | Track A — not Voice |

---

## 10. Production polish

Idle/Ready glyph distinction; reduced-motion mic; error flash; idle contrast from P16.20 retained.

---

## 11–12. Risks

- **Voice engineering:** none proven after P16.21  
- **Owner:** live Ready latency, rare COM, residual feel  

---

## Explicit answers

| Question | Answer |
| --- | --- |
| Remaining Voice-owned defect? | **No** (after P16.21) |
| Unnecessary Voice complexity? | Residual DOCUMENT (dual warm callers, sanitize layers) — not Owner blockers |
| Obsolete Voice implementation? | **No** product path |
| Repo-quality issue inside Voice? | DOCUMENT only |
| Engineering reason not to close P16? | **Only** live Owner Product Proof |
| Before 1M users? | Latency telemetry; permission tour; consolidate Voice strings |
| Track A? | Kernel warnings; tray; string consolidate; tour |
| P17? | File Provider — blocked |
| Evidence-supported? | **Yes** |

---

## Recommendation

**Recommend P16 for final Product Owner acceptance.**  
Do not permanently close P16. Do not begin P17. Do not launch until Owner requests.
