# Voice Input — Final Product Proof Validation
## P16.20

Engineering hostile validation against real-world production standards.  
**Does not** permanently close P16. Workspace was **not** launched.

---

## 1. Repository reassessment

| Truth | Status |
| --- | --- |
| P10–P15 | Permanently closed |
| P16 Engineering | Complete through **P16.20** |
| P16 Product Proof | **OPEN** — Owner acceptance required |
| P17 | **Blocked** (Production Before Expansion) |
| Architecture | Frozen |
| Foundation | WRAP WinRT ContinuousRecognitionSession |

Permanent principles verified present in protocol + Product Proof Rule:  
Product Gravity · Product Proof · Capability Independence · User Adaptation Prohibition · Commodity Before Reinvention · Engineering Verification Separation · Owner Directed Product Proof · Production Before Expansion · Evidence Before Completion · Technology Foundation Validation · Capability Regression Prevention · Engineering Completion Gate.

---

## 2. Hostile production validation (vs P16.19)

| P16.19 conclusion | Challenge | Evidence | Result |
| --- | --- | --- | --- |
| No sticky false deny left | status() used mic copy / preparing for ConfirmedDenied | `voice.rs` status branches | **FIX** — speech privacy message always |
| Soft Access Denied after success OK | Still entered Settings cycle via `permission_denied` | listen + VoiceMicButton | **FIX** — remap to `microphone_unavailable` after prior Allowed |
| Recheck after Settings safe | MediaCapture Allowed → ✓ Voice ready while speech privacy off | recheck + applyStatus treated prompt as grant | **FIX** — recheck needs listen confirm; grant only on `permission === "granted"` or successful listen |
| Ready without Capturing | Re-audit Capturing contract | `capturing_contract_failed` | **Hold** — blocked |
| Warm races | Re-audit warm_lock | listen + warm_up + recheck | **Hold** — serialized |
| Browser→exe | Re-run conversation tests | intent-bridge + conversation-quality | **Hold** — pass |
| Guide + softener | “What can you do for me?” → unknown | intentBridge Guide matcher used raw `text` only | **FIX** — match softened text |
| 1000 sessions | MemoryVoicePort stress | unit test | **Hold** — pass |
| Sleep/USB/CPU | No live hardware in engineering | OS/WinRT environmental | **DOCUMENT** — Owner live Proof |

---

## 3. Voice foundation validation

| Candidate | Startup / ready / permission / recovery | Verdict |
| --- | --- | --- |
| WinRT ContinuousRecognitionSession | OS privacy; compile-once; Capturing gate; soft recover | **WRAP — keep** |
| Sherpa / whisper.cpp / Vosk | Model load; custom capture; packaging | STUDY |
| Windows Speech platform | Same family | WRAP (current) |
| PowerToys / VS Code / Terminal / Kiro | Push-to-talk, explain once | Behavioural STUDY |

No industry behaviour missing that blocks P16. Ambient always-on remains REJECT.

---

## 4. State machine validation

Documented in `VOICE_LIFECYCLE_STATE_MACHINE.md` (still authoritative).  
Additional P16.20 transitions: soft-after-success remap; recheck→listen-confirm; ConfirmedDenied status identity = speech privacy.

---

## 5. Permission experience audit

| Scenario | Classification |
| --- | --- |
| First launch / denied / granted / remembered | Voice engineering — OK |
| Speech privacy sticky + correct Settings page | Voice — **fixed P16.20** |
| Settings return false ready | Voice — **fixed P16.20** |
| Soft mic after prior success → Settings spam | Voice — **fixed P16.20** |
| Polished onboarding tour | Track A polish |
| Windows owns Settings UI | OS limitation |

---

## 6. Natural language audit

Challenge set (GPT / browser beside / focus / screenshot / hear me / softeners) passes.  
**Fixed:** Guide discovery with softeners (`What can you do for me?`, `Please open Settings`).

---

## 7. Production hardening

| Attack | Result |
| --- | --- |
| Sticky privacy wrong Settings target | Fixed |
| False ✓ ready after Settings | Fixed |
| Soft deny Settings cycle after success | Fixed |
| 1000 Memory listens | Pass |
| Capturing honesty / warm_lock | Hold |
| WinRT COM under contention | DOCUMENT residual |

---

## 8. Production quality audit

Mic idle contrast / disabled opacity improved for discoverability. Phase colours (Ready / Listening / Recognizing / Error) retained. Residual glass/typography polish is Track A if Owner requests — not a functional blocker.

---

## 9. Explicit answers

| Question | Answer |
| --- | --- |
| Remaining Voice-owned engineering defect? | **None proven after P16.20 fixes** |
| Remaining reproducible engineering blocker? | **No** |
| Reason not to permanently close P16? | **Only** pending live Owner Product Proof acceptance |
| Improve before 100k users? | Live Capturing→Ready latency telemetry; optional permission tour |
| Track A? | Kernel warnings; tray; permission tour; glass polish |
| P17? | File Provider — blocked until P16 Owner closure |
| Evidence-supported conclusions? | **Yes** — each P16.19 hole had code-path evidence before FIX |

---

## Recommendation

**Recommend P16 for Product Owner acceptance.**  
Do **not** permanently close P16 in this program.  
Do **not** begin P17.  
Do **not** launch Workspace until the Product Owner requests it.
