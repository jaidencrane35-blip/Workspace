# Voice Input — Production Closure Investigation
## P16.19 Final Engineering Challenge

**Status:** Engineering Complete for Owner review.  
**Not:** Product Complete · permanently closed · P17.

Workspace was **not** launched during this program.

---

## 1. Repository reassessment

| Truth | Status |
| --- | --- |
| P10–P15 | Permanently closed |
| P16 Engineering | Complete through P16.19 |
| P16 Product Proof | **OPEN** — Owner acceptance required |
| P17 | **Blocked** |
| Architecture | Frozen |
| Foundation | WRAP WinRT ContinuousRecognitionSession |

Authorities re-read: ENGINEERING_HANDOFF, project-health, Product Constitution, Product Gravity, Product Proof Rule, Provider Acceptance Standard, Constitutional Execution Protocol, VOICE_RESEARCH, Failure Matrix, Readiness Audit, Regression Matrix.

---

## 2. Voice lifecycle audit

Documented in `VOICE_LIFECYCLE_STATE_MACHINE.md`.

Every Idle→Preparing→Warm→Capturing→Ready→Listening→Recognizing→Processing→Finished→Idle transition was challenged for race, deadlock, stale/poisoned state, UI inconsistency, and Product Proof incorrectness.

**Proven fix this program:** mic Access Denied no longer sticky-blocks after prior success (F1 residual).

---

## 3. State machine audit

| Concern | Finding |
| --- | --- |
| Duplicate states | None material (UI phases vs native stages intentionally layered) |
| Unreachable | None |
| Conflicting | Removed: sticky ConfirmedDenied vs soft mic fail |
| Hardware disconnect / replace | Soft fail → reset → next click re-warm |
| Sleep / resume | Next listen may cold-warm; truthful fail |
| Permission mid-run | Fail truthful; Settings only on true deny / privacy |
| Ambient always-on | REJECT (Gravity / consent) |

---

## 4. Permission architecture review

| Scenario | Product behaviour | Blocks Product Completion? |
| --- | --- | --- |
| First launch | Soft probe; listen tries; deny explains once | No |
| Permission denied | Truthful Conversation + mic click → Settings | No |
| Granted / remembered | Allowed cache; hot path | No |
| Revoked | Fail truthful; Settings path | No |
| Mic remove/replace | Soft recover or truthful fail | No |
| Speech privacy | Sticky until Settings recheck (intentional) | No |
| Settings spam | Mitigated (no soft→Settings; once-per-cycle) | No |
| Polished onboarding tour | Missing vs mature OS apps | **No — Track A polish** |

Owner should not need to understand WinRT/HRESULTs. Residual: Windows still owns the Settings pages Workspace opens — commodity reality, documented.

---

## 5. Commodity revalidation

| Candidate | Lifecycle / permissions / recovery | Verdict |
| --- | --- | --- |
| WinRT ContinuousRecognitionSession | OS mic + speech privacy; compile once; Capturing gate | **WRAP — keep** |
| Sherpa / whisper.cpp / Vosk | Custom capture + model warm; packaging cost | STUDY |
| PowerToys / VS Code / Kiro / Terminal | Push-to-talk, explain once, no ambient | Behavioural STUDY only |

Missing industry ambient always-on: **intentionally REJECT**.  
Mandatory Commodity Before Reinvention reaffirmed in Product Proof Rule.

---

## 6. Natural language audit

Deterministic Intent Layer challenged for ordinary desktop phrasing (Open GPT / ChatGPT / browser beside Cursor / Bring Chrome forward / screenshot / Can you hear me? / softeners).  

No AI guessing added. Browser→exe fallthrough remains guarded (P16.18). Residual alias gaps possible under Owner expectation — Intent Layer Track B polish after acceptance, not Voice lifecycle ownership.

---

## 7. Production hardening report

| Attack | Result |
| --- | --- |
| Sticky deny after MicrophoneUnavailable | Blocked (P16.18) |
| Sticky deny after Access Denied | **Blocked (P16.19)** |
| Ready without Capturing | Blocked (R11) |
| Listen vs startup warm race | Serialized |
| Soft mic → Settings spam | Blocked |
| 1000 MemoryVoicePort sessions | Pass |
| Rapid cancel / no_speech | Keep engine |
| WinRT COM crash under contention | Residual DOCUMENT (environmental) |

Could not prove remaining sticky false-deny path after P16.19 narrowing.

---

## 8. Remaining engineering risks (Voice-owned)

**None proven.** Residual WinRT environmental crashes are DOCUMENT, not an open implementation defect with a known Workspace fix short of STUDY migration.

---

## 9. Remaining Product Owner risks

- Live feel of first-word / Ready latency on real hardware  
- Residual NL expectation mismatch  
- Rare COM crash under extreme contention  
- Windows Settings still required for true OS denials  

---

## 10. Explicit answers

| Question | Answer |
| --- | --- |
| F1–F8 engineering defects eliminated? | **Yes** for reproduced FIX items; F3 residual DOCUMENT; F8 process debt DOCUMENT |
| Remaining issues outside Voice ownership? | **Yes** — Track A kernel warnings, doc authority, Owner-live feel |
| Engineering reason to delay P16 closure? | **No** — recommend Owner acceptance review |
| Shipping tomorrow — change first? | Instrument live Capturing→Ready latency telemetry (Track A); optional permission tour polish |
| Belongs in P17 / Track A? | File Provider (P17); tray/native polish; permission tour; kernel warnings |

---

## Recommendation

**Recommend P16 for final Product Owner acceptance.**  
Do **not** permanently close P16 in this program.  
Do **not** begin P17.  
Launch only when the Product Owner requests it.
