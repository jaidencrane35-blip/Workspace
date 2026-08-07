# Voice Input — Live Product Proof Package
## P16.26 — Canonical Owner checklist (definitive)

| Field | Value |
| --- | --- |
| **Authority** | This document is the **single canonical** live Product Proof checklist for P16 |
| **Status** | Pending Product Owner acceptance — **not** Product Complete |
| **Engineering** | Exit complete (P16.28) — `VOICE_ENGINEERING_EXIT_AUDIT.md` |
| **Engineering tip** | See `git log -1` on `v2-dev` after P16.28 |
| **Launch** | **Do not launch until Product Owner requests** |
| **Live timings** | Set `WORKSPACE_VOICE_PRODUCT_PROOF=1` for the review session only |
| **Companion evidence** | Traceability → `VOICE_OWNER_FINDINGS_TRACEABILITY.md` · Regressions → `VOICE_REGRESSION_MATRIX.md` · Instrumentation → `VOICE_LIVE_INSTRUMENTATION.md` |

Engineering Complete ≠ Product Complete. Only the Product Owner determines Product Completion.

---

## How to use this package

For each row: perform the action, judge **feel**, then mark Pass / Fail / N/A.  
If Fail: note what you saw. Engineering will not relaunch after you close Workspace.

---

## A. Voice readiness & lifecycle

| # | Scenario | Expected | Failure behaviour | Acceptance | Evidence | Regression |
| --- | --- | --- | --- | --- | --- | --- |
| A1 | Startup / mount | Mic idle `○`; no Settings spam; warm silent if already granted | Soft fail message if OS blocks; never auto-Settings | Feels quiet and ready to click | `VoiceMicButton` mount warm; R4 | R4 |
| A2 | Click → Preparing | Preparing `◌` visibly distinct | Stuck idle = fail | Click registers instantly | CSS preparing; R27 | R27 |
| A3 | Preparing → Ready | Green `◉` + pulse; **no** listening bars; title “speak when you like” | Ready without green / bars like Listening = fail | You know you may speak | Capturing contract; R1/R11/R26/R35 | R11, R26, R35 |
| A4 | First-word after Ready | First spoken words appear in transcript | Missing first words after Ready = fail | Speak immediately after Ready works | Capturing + Ready immediate; live report | R1, R11, R35 · Owner live |
| A5 | Speech → Listening | Cyan `●` + bars after speech energy | Ready stays forever while speaking = fail | Clear “it heard sound” | SoundStarted; R20/R31 | R20, R31, R36 |
| A6 | Processing / Done | Brief processing then ✓ Done chrome | Instant snap to idle with no Done = fail | Completion is noticeable | UI holds ~480ms; R28 | R28 |
| A7 | Cancel mid-listen | Session stops; no invented transcript | Freeze / crash = fail | Truthful stop | Cancel → keep engine; R7 | R7 |
| A8 | No speech | Honest “no speech” style message; engine kept warm | Cold recompile every quiet click = fail | Next click still feels hot | `listen_idle_keep_engine` | R7 |
| A9 | Soft mic fail ×1 | Amber soft-fail chrome; “Try again”; **no** Settings | Settings opens on first soft fail = fail | Retry is obvious | Soft remap; R19/R32 | R19, R32, R30 |
| A10 | Soft mic fail ×2 | Settings guidance; orange deny chrome | Silent / wrong copy = fail | You know next click opens Settings | Soft×2 gate; R23/R34 | R19, R23, R34 |
| A11 | Error flash | Error `!` visible before idle/soft | Blink invisible = fail | Failure is readable | ~720ms hold; R21/R28 | R21, R28 |
| A12 | Shutdown / close app | Clean exit; no orphan recognizer expectation | Crash on exit = fail | Closes normally | IPC cancel + process exit | Owner live |
| A13 | Cleanup after session | Next listen works after success/fail/cancel | “Couldn’t listen” forever = fail | Recovery without restart | Engine reset on poison; R6/R8 | R6, R8, R13 |

---

## B. Permission behaviour

| # | Scenario | Expected | Failure behaviour | Acceptance | Evidence | Regression |
| --- | --- | --- | --- | --- | --- | --- |
| B1 | First deny (speech privacy) | Clear speech-privacy explanation; Settings once on click | Mic Settings when speech privacy needed = fail | Understand what/why/next | Sticky privacy; R16/R17 | R16, R17 |
| B2 | First deny (mic) | Soft unavailable → retry; Settings after ×2 | Instant Settings spam = fail | Separates busy mic from OS deny | Soft remap; R19/R32 | R19, R32 |
| B3 | Grant / remember | After success, no Settings on relaunch | Settings every launch = fail | Remembered grant | `permissionGuidance` | R4 |
| B4 | Return from Settings | Confirm message; click mic to prove listen | “Voice ready” at Idle = fail | Honest confirm | Recheck needs listen; R17/R29 | R17, R29 |
| B5 | Revoke while running | Next listen fails truthfully; guidance | Silent / invent success = fail | Truthful | Soft/hard classify | Owner live + R16 |
| B6 | Hardware unavailable → returns | Soft fail then recovery on retry | Sticky forever = fail | Recovers | Soft mic; R13 | R13 |

---

## C. Conversation & natural language

| # | Scenario | Expected | Failure behaviour | Acceptance | Evidence | Regression |
| --- | --- | --- | --- | --- | --- | --- |
| C1 | Open GPT / Launch GPT / Open ChatGPT | Opens ChatGPT site — not an `.exe` | Executable launch = fail | Desktop feel | Intent bridge | R15 · conversation-quality |
| C2 | Open another / my browser | Browser open — not exe | Exe = fail | | | R15 |
| C3 | Open YouTube beside ChatGPT / Cursor | Beside composition | Wrong target / exe = fail | | | conversation-quality |
| C4 | Open Chrome / Bring Chrome|Cursor forward | App open / window focus | Provider jargon = fail | | | conversation-quality |
| C5 | Take a screenshot / Capture this window | Capture succeeds truthfully | Invented success = fail | | Screenshot provider closed | product-proof harness |
| C6 | Can / Could / Would / Do you hear me? | Voice status — click mic guidance | Unknown / “don’t have that” = fail | | R33 | R33 |
| C7 | What can you do? / for me? | Guide / help — no provider terms | Implementation jargon = fail | | R18 | R18 |
| C8 | Please / Could you / Would you… | Softeners work | Softener ignored = fail | | Soften + matchText | R18, R33 |

---

## D. Production stress (Owner live)

| # | Scenario | Expected | Failure behaviour | Acceptance | Evidence | Regression |
| --- | --- | --- | --- | --- | --- | --- |
| D1 | Long dictation (~1–5 min) | Continuous stitch; no mid-cut | Cuts mid-sentence = fail | Conversation continuity | Continuous + stitch; R2 | R2 |
| D2 | Long pause then resume speaking | Session continues or honest end | Silent discard = fail | Predictable | AutoStop silence | R2 · Owner live |
| D3 | Rapid click / cancel / retry | Recover or fail truthfully; no freeze | Freeze / Settings spam = fail | Trust | spawn_blocking; R3/R4 | R3, R4 |
| D4 | Immediate second listen | Feels warm | Cold lag every time = fail | | Warm keep; R7/R10 | R7, R10 |
| D5 | Sleep / resume | Recover or fail truthfully next listen | Permanent dead Voice = fail | | WinRT env | **Owner live** (Windows-owned residual) |
| D6 | USB / headset unplug-replug | Soft fail then recover | Sticky deny forever = fail | | Soft mic; R13 | R13 · Owner live |
| D7 | CPU / Defender pressure | Slow Ready OK; freeze not OK | UI freeze = fail | | spawn_blocking; R3 | R3 · Owner live |
| D8 | Live Click→Ready timing | Prefer hot reuse after first; cold first may be slower | Ready without Capturing = fail | | `WORKSPACE_VOICE_PRODUCT_PROOF=1` reports | R25 · Owner measured |

---

## E. Composition (Voice as input only)

| # | Scenario | Expected | Failure behaviour | Acceptance | Evidence | Regression |
| --- | --- | --- | --- | --- | --- | --- |
| E1 | Spoken desktop request | Transcript → same path as typed Intent → Kernel | Voice orchestrates providers = fail | Invisible input device | Composition audit | Independence |
| E2 | Voice never shows WinRT/HRESULT | Desktop language only | Jargon = fail | | `desktopVoiceMessage` | verify-voice-input |

---

## F. Pass / fail stamp (Owner only)

| Result | Stamp |
| --- | --- |
| Accept | `P16 Voice Input — PERMANENTLY CLOSED — ACCEPTED — REPOSITORY TRUTH — DO NOT REOPEN` |
| Reject | Keep `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` + list failing rows |

Until accept: Engineering must not begin P17 (Production Before Expansion).
