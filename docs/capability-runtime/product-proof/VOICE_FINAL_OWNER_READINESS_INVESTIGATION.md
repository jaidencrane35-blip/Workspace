# Voice Input — Final Owner Readiness Investigation
## P16.25

Hostile falsification before Product Owner live review.  
Workspace was **not** launched. P16 is **not** permanently closed.  
Engineering may only state: **no remaining reproducible engineering defects were identified** (after fixes below).

---

## 1. Repository reassessment

| Truth | Status |
| --- | --- |
| P10–P15 | Permanently closed |
| P16 Engineering | Complete through **P16.25** |
| P16 Product Proof | **OPEN** |
| P17 | Blocked |
| Architecture | Frozen — WinRT WRAP unchanged |

Permanent principles confirmed documented: Product Proof · Product Gravity · User Adaptation Prohibition · Commodity Before Reinvention · Engineering Verification Separation · Owner Directed Product Proof · Production Before Expansion · Owner Experience Before Engineering Confidence · Repository Quality Before Milestone Closure.

---

## 2. Historical defect audit

| Finding | Revalidated | Status |
| --- | --- | --- |
| R1/R11 First words / Ready w/o Capturing | Code + verifiers | Fixed + regression-tested; WinRT live latency Owner-measured |
| R2 Mid-speech cut | Code + continuity proof | Fixed + regression-tested |
| R3 UI freeze | `spawn_blocking` | Fixed + regression-tested |
| R4 Settings spam | Permission machine | Fixed + regression-tested |
| R5/R13/R16 False sticky deny | Soft / privacy-only | Fixed; **R32 closed warm-fail hole** |
| R19 Soft Settings trap | ×2 soft fails | Fixed; **R32/R34 closed residual holes** |
| R22/R29 False “ready” copy | Status + Conversation | **Measured + tightened** (Allowed no longer says “Voice is ready”) |
| R26–R31 Owner-feel chrome | UI + verifiers | Fixed + regression-tested |
| F3 Crashes / COM under load | — | DOCUMENT — Owner live |
| Live Click→Ready timings | Instrumentation ready | Owner-measured only |

---

## 3. Hostile falsification results (this program)

| ID | Proven defect | Fix |
| --- | --- | --- |
| R32 | Warm-fail Access Denied skipped soft remap → first-click Settings | `apply_listen_failure_policy` on warm-fail |
| R33 | Softened “Could you hear me?” → unknown | Hear-me patterns + matchText |
| R34 | Soft×2 counter stuck after Settings return | Reset on Settings open/return |
| R35 | Unnecessary 20ms Ready settle after Capturing | Immediate Ready (`settleBeforeReadyMs: 0`) |
| Bridge race | Callback delete on IPC return could drop Ready paint | Defer teardown 120ms |

---

## 4. Voice lifecycle audit

Idle / Preparing / Ready / Listening / Processing / Finished / Soft fail / Deny — held from P16.24 chrome. Ready emits immediately once Capturing confirms.

---

## 5. First-word investigation

| Stage | Code truth |
| --- | --- |
| Ready without Capturing | **Impossible** — `capturing_contract_failed` |
| Speech before Capturing | WinRT discards — Owner must wait for Ready (honest) |
| Artificial settle after Capturing | **Removed** (was 20ms) |
| Late Ready vs Listening UI | onReady preserves Listening; deferred bridge teardown |

---

## 6–8. Permission / NL / production hardening

Permission soft remap unified. Hear-me / GPT / beside / softeners revalidated in conversation-quality. Rapid click / cancel / 1000 Memory sessions held. Sleep / USB / Bluetooth / Defender — **Owner live** (recover or fail truthfully expected).

---

## 9. Commodity validation

WinRT ContinuousRecognitionSession **WRAP — keep**.  
No missing objectively superior production behaviour that blocks Owner review. Local ASR remains STUDY.

---

## 10. Regression additions

R32–R35 + verifier guards + conversation-quality hear-me / beside cases.

---

## 11–12. Remaining risks

**Engineering (reproducible Voice-owned):** none identified after R32–R35.  
**Owner-only / Track A:** live WinRT latency under CPU/Defender; USB/Bluetooth device churn; Windows Settings UX; Track A tray/polish.

### If shipped to 100,000 users tomorrow — top three engineering risks

1. **WinRT COM / device churn under load** — Voice-owned residual (OS WRAP surface); recover or fail truthfully.  
2. **First-session Windows privacy ballet** — Voice + OS; Workspace guides but cannot replace Settings.  
3. **Track A native polish / tray / deeper shell** — **not Voice**; future Track A.

---

## Explicit answers

| Question | Answer |
| --- | --- |
| Historical findings independently revalidated? | R1–R31 held; R16/R19/R22/R29 tightened |
| Required additional fixes? | R32–R35 + bridge defer |
| Already fixed but now measured? | Allowed status copy; Ready settle removed |
| Remaining reproducible Voice-owned defect? | **No** (identified none after fixes) |
| Evidence preventing Owner acceptance? | **No** engineering blocker |
| WinRT still correct WRAP? | **Yes** |

---

## Permanent statement

Engineering Complete ≠ Product Complete.  
Only the Product Owner determines Product Completion.
