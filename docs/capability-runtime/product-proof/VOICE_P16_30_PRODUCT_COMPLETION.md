# P16.30 — Product Completion Reopened
## Intent Architecture, Voice UX & Production Validation

| Field | Value |
| --- | --- |
| **Program** | P16.30 (not P17) |
| **Status** | Engineering complete for this reopen — awaiting Owner live Product Proof |
| **Branch** | `v2-dev` |
| **Prior exit** | P16.28 Engineering Exit **falsified** by Owner live findings F9–F11 |

---

## 1. Repository reassessment

- P10–P15 permanently closed.
- P16 Product Proof **open** (not permanently closed).
- P17 **blocked**.
- WinRT `SpeechRecognizer` + `ContinuousRecognitionSession` remains the WRAP unless Owner evidence disproves it.
- Pipeline treated as one production path: Speech → Transcript → Intent Grammar → Desktop Intent → Capability Resolution → Execution → Truthful Feedback → Conversation.

---

## 2. Voice architecture review

Voice remains a Conversation **input device**. Recognition never dispatches to executable launching. After transcript, the same Intent Layer path as typed text applies — with an explicit Owner **review → Send** step (F10).

---

## 3. Intent architecture review

Permanent Intent Grammar layer: `app/src/lib/intentGrammar.ts` → `parseDesktopIntent` → structured `DesktopIntent` (Action / Target / Modifier / Object / Context / Confidence) → `resolveFromDesktopGrammar` in `intentBridge.ts` → `IntentAction` → Kernel composition (`open_foreground`, `open_maximize`, shell folder launch).

Raw transcripts must never become executable names.

---

## 4. Commodity survey (behaviour only — no code copied)

| Area | Studied | Adopted behaviour |
| --- | --- | --- |
| Recognition lifecycle | WinRT ContinuousRecognitionSession, OS dictation | Keep WRAP; AutoStop finish; warm lifecycle |
| Intent parsing | PowerToys Run, Spotlight, Raycast, Alfred, VS Code Command Palette | Structured parse before launch; desktop verbs |
| Desktop routing | Windows Shell / Search, Finder | Open / Launch / Focus / Maximize / Locate / shell folders |
| Dictation UX | OS dictation, ChatGPT Desktop voice | Stop → review text → Send / Cancel (not auto-fire) |
| Permissions | Windows privacy Settings | Explain → guide → remember → recover; Settings once per deny cycle |

Custom-built only where Workspace differentiates: Kernel Operator composition + Conversation gravity.

---

## 5. Fast speech investigation (F9)

See `VOICE_FAST_SPEECH_INVESTIGATION.md`.

**Verdict:** Rapid speech quality is primarily **inherent WinRT dictation segmentation / hypothesis timing**, not a Workspace Intent defect. Configurable timeouts were reviewed; no safe EndSilence knobs remain on ContinuousRecognitionSession (by prior design). Migration not justified without a measured superior WRAP on the same utterances. WRAP **remains frozen**.

---

## 6. WinRT validation

Attempted falsification of continuous session, warm lifecycle, permission model, capturing timing, buffering, rapid speech, first-word capture, recovery. No objective evidence that another WRAP outperforms ContinuousRecognitionSession for Workspace’s dictation scenario. **WRAP frozen.**

---

## 7. Permission review

No change to the once-per-cycle Settings architecture. Soft fail vs hard deny chrome retained. Never claim Ready at Idle; never falsely claim microphone unavailable after soft remap policy.

---

## 8. Conversation improvements

- Voice transcript inserts into the composer for review (not auto-submit).
- Guidance: none in Conversation after dictation (P16.PX4 — composer + soft Send only; F10 review-before-send unchanged)
- Escape clears draft (cancel).
- Mic chrome: Listening — click to stop; Reviewing after recognition.

---

## 9. Intent Grammar implementation

- Compounds: open+foreground, open+fullscreen, locate application, explorer+locate folder, open beside.
- Kernel: `browser.open_foreground`, `app.open_maximize`.
- Application launch: `shell:` URIs via `explorer.exe` + arg.

---

## 10. Desktop language

Tolerates please / can you / for me / the / my; aliases GPT→ChatGPT, Explorer folders, maximize synonyms, bring to front / forward.

---

## 11. Dictation workflow (F10)

| Phase | Owner experience |
| --- | --- |
| Idle | Mic available |
| Preparing | Preparing… |
| Ready | Ready — speak, or click to stop |
| Listening | Listening — click to stop |
| Stop | Click mic during capture |
| Review Transcript | Text in composer; selected for edit |
| Send | Enter / Send |
| Cancel | Escape or clear box |
| Processing | After Send (Conversation / Kernel) |
| Finished / Error | Truthful Conversation reply |

---

## 12. Production hardening

Existing 1000-session memory stress + regression matrix retained. New guards: R41–R43 (Intent Grammar F11, dictation review F10, shell: launch).

---

## 13. Explicit Owner questions

| Question | Engineering answer |
| --- | --- |
| Does Voice behave like a premium desktop assistant? | Closer — review/send/cancel + Intent Grammar; **Owner decides** |
| Is rapid speech objectively reliable? | **Not guaranteed** — F9 documents WinRT limitation; see investigation |
| Does Intent Grammar prevent executable-name failures? | **Yes** for the four Owner F11 phrases (automated tests) |
| Does every spoken request pass through structured intent? | Grammar for desktop compounds; all utterances still through `resolveIntent` before Kernel |
| Is WinRT still objectively the correct WRAP? | **Yes** until measured superior WRAP exists |
| Remaining reproducible Voice-owned engineering defect? | **None known to engineering**; Owner live review is the gate |
| Blockers for 100k / 1M users? | Owner acceptance; WinRT rapid-speech ceiling; OS permission UX; long-tail NL |
| Remaining engineering justified by objective evidence? | **Only if Owner demonstrates a new reproducible Voice-owned defect** |

---

## 14. STOP

Do not begin P17. Do not mark P16 permanently closed. Await Product Owner review only.
