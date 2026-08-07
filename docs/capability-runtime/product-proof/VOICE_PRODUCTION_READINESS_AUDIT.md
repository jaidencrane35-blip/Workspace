# Voice Input — Production Readiness Audit
## P16.17

Engineering falsification pass. Does **not** equal Owner Product Complete.

---

## Classification legend

| Class | Meaning |
| --- | --- |
| KEEP | Correct; leave |
| REMOVE | Deleted this program |
| SIMPLIFY | Reduced complexity this program |
| DOCUMENT | Retained with explicit justification |
| FIX | Corrected defect / drift |

---

## Voice owned-surface findings

| Item | Class | Action |
| --- | --- | --- |
| ContinuousRecognitionSession + Capturing Ready | KEEP | Production WRAP |
| Soft MediaCapture / ConfirmedDenied | KEEP | P16.13 guard |
| Idle keep-engine / poison reset | KEEP | P16.14 guard |
| Capturing-contract honesty | KEEP | P16.16 R11 |
| `install_voice_port_for_tests` (noop + `allow(dead_code)`) | REMOVE | Deleted |
| Deprecated permission helpers | REMOVE | Deleted |
| Unused denied-permission getters | REMOVE | Deleted |
| Stale RecognizeAsync product docs / comments | FIX | Continuous session language |
| `status()` treating sticky MediaCapture Denied | SIMPLIFY | Peek denies only ConfirmedDenied |
| RecognizeAsync EndSilenceTimeout set | SIMPLIFY | Removed; AutoStop owns finish |
| Dual warm (startup + mount) | DOCUMENT | Idempotent under `warm_lock`; mount also applies permission UI |
| Dual `voice-sound` / `voice-listening` emit | DOCUMENT | Frontend uses both hooks; low risk |
| Message sanitize in port + IPC + bridge | DOCUMENT | Defence in depth; drift monitored |
| Trait `listen_once` default | DOCUMENT | Test / stub convenience over when_ready |
| Write-only frontend `setWarmed` | DOCUMENT | Poison-reset sync + regression verifier |

---

## Compiler warning audit

| Surface | Warnings | Class | Notes |
| --- | --- | --- | --- |
| `workspace-windows-integration` Voice | **0 dead_code** | KEEP | Clean |
| `workspace-app` Voice IPC | **0 dead_code** after REMOVE | KEEP | Clean |
| `workspace-kernel` | ~114 unused items | DOCUMENT | Pre-existing Track A / frozen kernel surfaces — **not Voice-owned**. Mass deletion risks unrelated Product Proof domains. Recorded as remaining repository criticism, not a Voice runtime defect. |
| ts-rs serde parse noise | Dependency tooling | DOCUMENT | Upstream ts-rs limitation; not suppressible without vendor churn |

---

## Foundation / commodity (revalidated)

| Candidate | Verdict |
| --- | --- |
| WinRT ContinuousRecognitionSession | **WRAP — still correct** |
| whisper / Sherpa / Vosk | STUDY |
| Azure / Web Speech | REJECT default |
| Ambient always-on session | REJECT (Product Gravity / consent) |

**Permanently alive:** compiled recognizer yes; continuous session per mic turn only.

---

## Falsification attempts

| Attack | Result |
| --- | --- |
| Ready without Capturing? | Blocked (R11) |
| Sticky MediaCapture deny? | Blocked (soft probe) |
| Listen-path warm gate desync? | Blocked (P16.15) |
| Dead test hook / deprecated helpers? | Removed (P16.17) |
| RecognizeAsync residue in product docs? | Fixed (P16.17) |
| Kernel dead_code as Voice blocker? | Documented — Track A, not Voice-owned |
| Sticky ConfirmedDenied on mic Access Denied? | Fixed (P16.19 — privacy sticky only) |
| Soft mic fail leaving stale ConfirmedDenied? | Fixed (clear on mic_unavailable_soft) |

---

## Remaining criticism (hostile)

1. Kernel unused-code warnings remain repository-wide debt (not Voice).  
2. Permission / voice user strings still triplicated (sanitization layers).  
3. Dual warm entry points remain (safe, slightly more ceremony than ideal).  
4. Live first-word / sleep-resume still require Owner Product Proof (cannot be fully automated without mic hardware).  
5. Polished first-launch permission tour is Track A UX — not a lifecycle defect.

**Engineering conclusion (P16.19):** No remaining **reproducible Voice-owned** engineering defect found after active falsification. P16 stays OPEN for Owner acceptance. See `VOICE_PRODUCTION_CLOSURE_INVESTIGATION.md`.
