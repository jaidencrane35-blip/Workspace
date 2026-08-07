# P16.PX1 — Conversation Experience Audit

| Field | Value |
| --- | --- |
| **Program** | P16.PX1 — Conversation Experience Refinement |
| **Kind** | Experience polish (not constitutional; not production gate; not P17) |
| **Date** | 2026-08-07 |
| **Commit** | `d4ffb90` |
| **Constraint** | Workspace Constitutional Specification v2 + Engineering Execution Standard |
| **Rule** | One isolated UX improvement only → Product Owner review |

---

## 1. Conversation Experience Audit

| Surface | Current state | Premium gap |
| --- | --- | --- |
| Conversation surface | Operator shell composer + transcript; Product Gravity toward Conversation | Feels correct structurally; micro-polish uneven vs daily-driver chat apps |
| Composer | Textarea + mic + Send; Enter sends; Escape clears draft | Clear; voice→review→send still feels stepwise |
| Voice / dictation | Toggle mic; F10 review-before-send; phase chrome | **Too many interactions** (Owner PP evidence); stop requires re-finding mic |
| Dictation controls | Single mic toggle; tooltips; wave on speech | No lightweight Stop/Cancel keyboard path before PX1 |
| Transitions | Preparing → ready → listening → reviewing | Reviewing message helps trust; motion quality uneven |
| Loading / progress | Preparing pulse; recognizing/processing phases | Honest; not glamorous |
| Window focus | Settings-return recheck; composer autofocus on mode | Adequate |
| Interruptions | Mic toggle cancel; Escape clears draft in composer | Escape during capture was not a first-class cancel before PX1 |
| Message lifecycle | Streaming workspace replies; truthful failures | Strong trust posture |
| Confirmation | Restore/Moments review before move | Correct constitutional honesty |
| Errors | Soft fail vs Settings deny chrome | Good distinction; copy sometimes long |
| Notification UX | Provider-backed; Conversation language | Separate Track A polish |
| Restore/Moments | Conversation-led open + approve | Not in PX1 scope beyond audit |
| Micro-interactions / motion | Mic pulse + 4-bar wave; reduced-motion respected | **Speaking animation feels low quality** (Owner PP evidence) |
| Accessibility / keyboard | Mic labels; composer Enter/Escape | Capture keyboard incomplete before PX1 |

**Verdict:** Architecture and trust are strong. Friction is interactional: dictation stop/send and listening feedback quality — not missing capabilities.

---

## 2. Voice Interaction Audit

Compared for **interaction quality only** (ChatGPT / Claude / Windows Dictation patterns — not branding or architecture).

| Step | Workspace (pre-PX1) | Modern pattern | Gap |
| --- | --- | --- | --- |
| Activate mic | Click mic | Click / hotkey | OK |
| Recording feedback | Pulse + bars on speech | Clear listening chrome | Animation quality low |
| Stop | Click mic again | Click stop / key / release | **Must re-target mic** |
| Cancel | Implicit via cancel listen / clear after | Explicit cancel | Weak during capture |
| Send | Enter or Send after review (F10) | Edit then send | Correct; still one more action |
| Transcript lifecycle | Insert draft, focus, select | Edit-before-send | F10 preserved — do not auto-send |
| Edit-before-send | Yes | Yes | Keep |
| Latency perception | Preparing state honest | Instant arm preferred | Acceptable; not PX1 change |

**Owner Product Proof evidence (authoritative for this program):**

1. Dictation requires too many interactions.
2. Press-to-start / press-to-stop / press-enter feels cumbersome.
3. Speaking animation feels low quality.
4. Conversation should feel more natural; controls modern and lightweight.
5. Prefer ChatGPT-like start / stop / cancel / send **interaction model** within Workspace visual identity.

**F10 constraint:** Auto-send after stop is out of scope. Review-before-send stays.

---

## 3. Premium UX gap analysis

| Gap | Severity | Risk to change | Notes |
| --- | --- | --- | --- |
| Stop requires second mic click | High | Low | Keyboard Stop reduces hunt |
| No Escape cancel while capturing | Medium | Low | Aligns with cancel mental model |
| Listening wave / pulse quality | High (Owner) | Low (CSS) | Isolated; deferred to next single improvement |
| Distinct Stop vs Cancel chrome | Medium | Medium | Extra controls → Track A |
| Hold-to-talk | Medium | Medium | Platform + a11y → Future |
| Auto-send after voice | High desire / blocked | — | Violates F10 Product Proof |
| Notification / Moments motion | Low–Med | Varies | Future / Track A |

---

## 4. Prioritized UX improvements

| ID | Improvement | Class | Rationale |
| --- | --- | --- | --- |
| **PX1-1** | Capture keyboard: **Enter / Space = stop**, **Escape = cancel** (tooltips updated); F10 review unchanged | **Implement Now** | Highest friction reduction; isolated to `VoiceMicButton`; no architecture/capability change |
| PX1-2 | Premium listening waveform + pulse (CSS only, reduced-motion safe) | **Track A** | Owner animation evidence; next single polish |
| PX1-3 | Lightweight Stop / Cancel affordances beside mic while capturing | **Track A** | ChatGPT-like chrome; keep Workspace identity |
| PX1-4 | Soft Send emphasis when draft just filled from voice | **Track A** | Makes post-stop Send obvious |
| PX1-5 | Shorter reviewing copy; less instructional tone | **Track A** | Natural conversation feel |
| PX1-6 | Hold-to-talk / push-to-talk option | **Future** | New interaction mode; a11y design needed |
| PX1-7 | Any new Voice/desktop capability or provider work | **P17** | Blocked until P16 Product Proof Accept |
| PX1-8 | Auto-send after dictation | **Rejected** | Breaks F10 review-before-send |

---

## 5. Implementation (single improvement)

**Shipped: PX1-1 — capture-phase keyboard Stop / Cancel**

- File: `app/src/components/operator/VoiceMicButton.tsx`
- While `preparing` / `ready` / `speechDetected` / `listening`: window capture-phase `keydown`
  - `Enter` or `Space` → `finish()` (stop listen; transcript still reviews)
  - `Escape` → `finish()` (cancel listen)
- Tooltips advertise keyboard stop/cancel
- Verifier: `scripts/verify-voice-input.mjs` (P16.PX1 checks)

**Not shipped in PX1:** animation redesign, new buttons, auto-send, P17, governance, production gates.

---

## 6. Product Proof impact

| Before | After (PX1-1) |
| --- | --- |
| Start → click mic to stop → Enter to send | Start → **Enter/Space to stop** (hands stay on keyboard) → Enter to send |
| Cancel during capture unclear | **Escape** cancels listen |
| F10 review | **Unchanged** — no auto-submit |
| Owner “too many interactions” | Partially addressed (stop no longer requires mic re-click) |
| Owner “speaking animation low quality” | **Open** — Track A PX1-2 |

**Owner review asked:** Confirm keyboard stop/cancel feels natural in live dictation; then approve PX1-2 (animation) as next single improvement.

---

## 7. Verification

```bash
node scripts/verify-voice-input.mjs
```

---

## 8. Stop

PX1 implements **one** improvement only. Await Product Owner review before further Conversation polish or P17.
