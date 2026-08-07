# Voice Input — Owner Findings Traceability
## P16.26

Nothing relies on memory. Every historical Owner / falsification finding maps to root cause → fix commit → regression → evidence → status.

| Field | Value |
| --- | --- |
| **Tip baseline** | P16.25 `42263bf` + P16.26 package commit |
| **Canonical live checklist** | `VOICE_LIVE_PRODUCT_PROOF_PACKAGE.md` |
| **Regression authority** | `VOICE_REGRESSION_MATRIX.md` |

Status legend: **CLOSED** (engineering + regression) · **OWNER-LIVE** (requires Product Owner feel) · **DOCUMENT** (Windows / Track A residual)

---

## A. Owner failure matrix (F1–F11)

| Finding | Root cause | Fix commit | Regression | Evidence | Status |
| --- | --- | --- | --- | --- | --- |
| F1 Mic unavailable after success | Sticky `ConfirmedDenied` on transient mic fail / Access Denied | `b926878` · `90f42bf` | R5, R13, R16 | Failure matrix; soft probe; privacy-only sticky | CLOSED · OWNER-LIVE feel |
| F2 Long startup before Ready | Listen warm raced without `warm_lock`; cold recompile | `b926878` · `f287364` | R9→R35, R14 | warm_lock; settle→0 | CLOSED · OWNER-LIVE latency |
| F3 Occasional crashes | WinRT COM under contention | Mitigations across P16.10–P16.18 | — | spawn_blocking; warm serialize | DOCUMENT (Windows) |
| F4 First-word inconsistent | Ready before Capturing; speech before Capturing discarded | `638fff5` · `0fd7381` · `42263bf` | R1, R11, R35 | Capturing contract; immediate Ready | CLOSED · OWNER-LIVE |
| F5 NL inconsistent | Softener / alias gaps | `ee9bfa7` · `9cbd350` · `42263bf` | R15, R18, R33 | conversation-quality | CLOSED · OWNER-LIVE residual expectation |
| F6 Browser → exe | Fallthrough to app launch | `b926878` | R15 | intentBridge canonicalize | CLOSED |
| F7 Permission confusion | Soft unavailable treated as Settings deny | `b926878` · `526d4fb` | R4, R19, R30 | soft×2; soft-fail chrome | CLOSED · OWNER-LIVE |
| F8 Declared complete too early | Optimism without falsification | Process principles P16.17–P16.25 | R25 | Evidence Before Completion; hostile passes | CLOSED (process) |
| F9 Fast speech quality degrades | WinRT dictation segmentation / hypothesis timing under rapid speech | P16.30 (document + review→Send mitigation; no WRAP migrate) | R44 | `VOICE_FAST_SPEECH_INVESTIGATION.md` | DOCUMENT (WinRT) · OWNER-LIVE |
| F10 Dictation workflow incomplete | Auto-submit on transcript; no review/send/cancel | P16.30 | R42 | OperatorRoot + VoiceMicButton reviewing | CLOSED · OWNER-LIVE |
| F11 NL → executable names | Compound open phrases fell through to `appOpen` query | P16.30 | R41, R43 | Intent Grammar + Kernel compose | CLOSED · OWNER-LIVE |

---

## B. Hostile / validation holes (D / P / U / S / feel)

| Finding | Root cause | Fix commit | Regression | Evidence | Status |
| --- | --- | --- | --- | --- | --- |
| D1 ConfirmedDenied wrong status copy | Mic/preparing message on privacy sticky | `9cbd350` | R17 | speech privacy status | CLOSED |
| D2 Recheck → false ready | MediaCapture Allowed stamped grant | `9cbd350` | R17, R29 | listen confirm | CLOSED |
| D3 Soft Access Denied → Settings | permission_denied Settings path | `9cbd350` · `526d4fb` | R19, R32 | soft remap (+ warm-fail) | CLOSED |
| NL Guide softener | Guide used raw text only | `9cbd350` | R18 | matchText | CLOSED |
| P1 Soft first deny Settings trap | Always notePermissionDenied | `526d4fb` | R19 | soft×2 | CLOSED |
| U1 Error phase never paints | Immediate idle | `526d4fb` · `e4d7dd6` | R21, R28 | 720ms hold | CLOSED |
| U2 Dual listening emit race | voice-sound + voice-listening | `526d4fb` | R20 | single emit | CLOSED |
| C2 Redundant second warm | Double warm on listen | `526d4fb` | R10 | warm_already | CLOSED |
| S1 False “Voice is ready” | Unknown/Allowed copy | `1ec84d3` · `42263bf` | R22, R29 | set-up / click mic copy | CLOSED |
| S2 Soft×2 message desync | Arm gate + “Try again” | `1ec84d3` | R23 | Settings copy when arming | CLOSED |
| S3 Settings-help without gate | Bridge/IPC copy | `1ec84d3` | R23 | retry-only copy | CLOSED |
| S4 Warm-fail misclassify | Double sanitize | `1ec84d3` | R24 | listen_outcome_from_engine_error | CLOSED |
| R26–R31 Owner-feel chrome | Ambiguous mic states | `e4d7dd6` | R26–R31 | VoiceMicButton + CSS | CLOSED · OWNER-LIVE |
| R32 Warm-fail Settings trap | Early warm return skipped remap | `42263bf` | R32 | apply_listen_failure_policy | CLOSED |
| R33 Hear-me soften miss | matchText not applied to voice | `42263bf` | R33 | intentBridge + tests | CLOSED |
| R34 Soft count after Settings | Counter not reset | `42263bf` | R34 | VoiceMicButton reset | CLOSED |
| R35 Ready settle lag | 20ms sleep after Capturing | `42263bf` | R35 | settleBeforeReadyMs: 0 | CLOSED |
| Bridge Ready paint race | Callback delete on IPC return | `42263bf` | R36 | deferred teardown 120ms | CLOSED |
| Concurrent double-listen | No whole-listen lock / phase lag | *(P16.27)* | R37 | warm_lock whole listen + listenInFlightRef | CLOSED |
| False “opened Settings” | Gate before open; swallow errors | *(P16.27)* | R38 | open returns boolean | CLOSED |
| Classify “allow access” landmine | Bare `access` match | *(P16.27)* | R39 | denied/unavailable phrases only | CLOSED |
| False available chrome | `available \|\| recognitionAvailable` | *(P16.27)* | R40 | `status.available` only | CLOSED |

---

## C. Foundational engineering findings (pre-F matrix)

| Finding | Root cause | Fix commit | Regression | Evidence | Status |
| --- | --- | --- | --- | --- | --- |
| UI freeze on listen | WinRT on IPC thread | `fe1e885` | R3 | spawn_blocking | CLOSED |
| Settings spam | Auto-open Settings | `e8f745c` | R4 | Permission Guidance | CLOSED |
| Mid-speech cut | Short EndSilence / RecognizeAsync | `ee9bfa7` · `e8f745c` | R2 | Continuous + stitch | CLOSED |
| Listening before Capturing | UI Listening ≠ audio retained | `638fff5` | R1 | Ready on Capturing | CLOSED |
| Ready without Capturing | Timeout still emitted Ready | `0fd7381` | R11 | capturing_contract_failed | CLOSED |
| MediaCapture warm race | Probe during warm | `77b3b4d` | R6 | no MediaCapture in warm | CLOSED |
| False sticky MediaCapture Denied | Cached Denied forever | `bb8ab78` | R5 | soft probe | CLOSED |
| Dead Voice test install | Declared complete with dead code | `80ab8fd` | R12 | readiness audit | CLOSED |
| Engineering confidence w/o live timings | No measurable Click→Ready | `008f110` | R25 | voice_proof env gate | CLOSED · OWNER-LIVE measure |

---

## D. Commit index (P16.5–P16.26)

| Program | Commit | Subject |
| --- | --- | --- |
| P16.5 | `1434aa3` | make Voice listen immediate and capture first words |
| P16.6 | `7b9451a` / `0093b93` | conversation Product Proof quality / first-word remediate |
| P16.7 | `638fff5` | Capturing contract |
| P16.8 | `ee9bfa7` | continuous listen + Semantic Alias |
| P16.9 | `e8f745c` | Ready contract, stitch, Permission Guidance |
| P16.10 | `fe1e885` | off IPC thread |
| P16.11 | `b90529c` | foundation + permission recheck |
| P16.12 | `0bd7f34` | foundation + permission architecture |
| P16.13 | `bb8ab78` | false-deny mic regression |
| P16.14 | `77b3b4d` | MediaCapture warm race |
| P16.15 | `f287364` | production hardening |
| P16.16 | `0fd7381` | Capturing honesty |
| P16.17 | `80ab8fd` | dead code / readiness |
| P16.18 | `b926878` | sticky mic deny + warm_lock |
| P16.19 | `90f42bf` | F1 sticky-deny falsification |
| P16.20 | `9cbd350` | permission / Guide fixes |
| P16.21 | `526d4fb` | Owner experience gate |
| P16.22 | `1ec84d3` | ship-readiness falsification |
| P16.23 | `008f110` | live instrumentation |
| P16.24 | `e4d7dd6` | Owner-feel chrome |
| P16.25 | `42263bf` | Owner readiness falsification |
| P16.26 | *(this commit)* | Final Live Product Proof Package |

---

## E. Engineering conclusion — evidence chain

**Statement:** No remaining reproducible Voice-owned engineering defects were identified.

| Support | Location |
| --- | --- |
| Code | `packages/windows-integration/src/voice.rs`, `VoiceMicButton.tsx`, `intentBridge.ts`, `bridge.ts`, `permissionGuidance.ts` |
| Regressions | R1–R36 in `VOICE_REGRESSION_MATRIX.md` |
| Verifiers | `pnpm verify:voice-input` · `verify:voice-regression` · `verify:voice-proof-instrumentation` · `verify:conversation-quality` |
| Instrumentation | `WORKSPACE_VOICE_PRODUCT_PROOF=1` → `%TEMP%/workspace-voice-proof/` |
| Hostile passes | P16.19–P16.25 investigation docs |
| Validation | `pnpm typecheck` · `build` · `test` · `cargo check` · health verifiers |

This is **not** Product Complete. Owner live rows marked OWNER-LIVE remain judgment gates.
