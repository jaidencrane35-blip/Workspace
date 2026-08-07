# Workspace Product Quality Standard

| Field | Value |
| --- | --- |
| **Status** | Canonical subordinate product-quality authority (P16.PQ1) |
| **Authority class** | Product quality — **not** constitutional law |
| **Subordinate to** | Workspace Constitutional Specification v2 · Engineering Execution Standard v1 · Capability Integration Standard · Workspace Interaction Language |
| **Peers** | `PRODUCT_GRAVITY_RULE.md` · `WORKSPACE_INTERACTION_LANGUAGE.md` · Product Proof Rule |
| **Date** | 2026-08-07 |
| **Commit** | _(stamped on ship)_ |
| **Branch** | `v2-dev` |

This document defines what **excellent** means for Workspace as a premium conversational desktop product.  
Every future capability, production gate (user-facing), UX improvement, and Product Proof session **shall** use this standard.  
It does not modify Spec concepts. Review Triggers still govern constitutional change.

---

## What “premium” means here

Premium is not feature count.  
Premium is: **effortless · calm · trustworthy · intentional · conversational · responsive** — the experience users remember.

Technically correct ≠ premium. Architecture complete ≠ product complete.

---

## 1. Interaction Quality

| Expectation | Excellent means |
| --- | --- |
| Effort | Fewest safe actions for the outcome |
| Clarity | User always knows what just happened and what to do next |
| Discoverability | Ordinary language and Conversation — not menus |
| Responsiveness | Immediate acknowledgment; honest wait states |
| Trust | Truthful success/failure; no invented completion |
| Predictability | Same intent → same shape of outcome |
| Calmness | No unnecessary colour, motion, or words |
| Recoverability | Retry / Settings / clear paths — never punish |

**Rule:** every interaction should reduce cognitive load.

---

## 2. Conversation Quality

Workspace must feel: conversational · trustworthy · direct · intentional · calm · effortless.

Avoid: robotic behaviour · engineering terminology · unnecessary confirmations · unnecessary clicks · menu-driven workflows · modal overload.

Conversation remains the product (Product Gravity). Capabilities appear inside Conversation.

---

## 3. Voice Quality

F10 **review-before-send remains unchanged**.

| Step | Excellent |
| --- | --- |
| Activate | One clear control; preparing is honest |
| Listen | Soft motion; calm chrome (PX2) |
| Stop | Click, Enter, or Space (PX1) |
| Cancel | Escape during capture; clear draft after |
| Review | Transcript in composer; short calm cue (PQ1) |
| Send | Enter / Send — softly emphasized after voice (PX3) |
| Edit | Full edit before Send |
| Interrupt | Cancel/stop without desktop side effects |
| Errors / recovery | Soft retry vs Settings; desktop language |

---

## 4. Motion Quality

| Principle | Meaning |
| --- | --- |
| Purpose | Motion communicates state — never decorates |
| Calm over spectacle | Soft breathing, not neon status boards |
| Attention | Reinforce transcript → composer → one next action |
| Never distract | No competing ambient motion |
| Accessible | `prefers-reduced-motion` disables decorative motion |
| Interruptible | Capture stop/cancel always available |
| Timing / easing | Consistent, predictable, unhurried |

---

## 5. Visual Quality

Hierarchy · spacing · typography · density · noise · chrome · icons · colour · emphasis · readability  
should feel **intentional**, not engineered.  
Mic and Send whisper unless they are the one obvious next action.

---

## 6. Capability Quality

Every capability must answer:

- Is it obvious?
- Is it discoverable naturally in Conversation?
- Is it faster than doing it manually?
- Would a user choose it again?
- Does it reduce effort?
- Does it preserve Product Gravity?
- Does it feel like Workspace (Interaction Language)?

---

## 7. Reliability Quality

Graceful failure · retry · recovery · interruption · partial success honesty · diagnostics · support package · installer/updater recovery (as gates land).  
Users recover; they are not blamed.

---

## 8. Production Quality (user perspective)

Installation confidence · update confidence · crash recovery · tray · support · onboarding · first launch · shutdown  
evaluated as **felt confidence**, not CI green.  
(Engineering production gates remain in Production Gate Spec — this section is the user lens.)

---

## 9. Delight

Exceed expectations **without** adding complexity: subtle guidance, calm confidence, reduced friction, intelligent defaults.  
Reject novelty for its own sake.

---

## 10. Product heuristics (permanent; not Spec)

1. Calm over clever  
2. Trust over spectacle  
3. One obvious next action  
4. Explain rather than expose implementation  
5. Recover instead of punish  
6. Reduce clicks whenever constitutional safety permits  
7. Consistency over novelty  
8. Every interaction should reduce effort  
9. Premium products remove work rather than add features  
10. Friction preference: prefer reducing user friction over exposing engineering complexity unless Spec or Product Proof forbids it  

---

## 11. Product Quality Checklist

Reusable. Every future capability / experience change answers:

| # | Question |
| --- | --- |
| 1 | Does it reduce effort? |
| 2 | Does it preserve trust? |
| 3 | Does it preserve Product Gravity? |
| 4 | Does it feel conversational? |
| 5 | Does it reduce cognitive load? |
| 6 | Would a user voluntarily use this again? |
| 7 | Does it feel premium? |
| 8 | Is it consistent with the Workspace Interaction Language? |
| 9 | Does it comply with the Constitutional Specification? |
| 10 | Does it comply with the Engineering Execution Standard? |

Ship only if answers are honestly yes (or N/A with reason).

---

## 12. Product Experience Audit (repository evidence)

| Surface | Evidence-based state | Class |
| --- | --- | --- |
| Conversation / composer | Gravity-stable; Enter send; Escape clear; soft Send after voice (PX3) | Already Excellent / minor Track A |
| Dictation stop/cancel | Keyboard Stop/Cancel (PX1) | Already Excellent for keyboard |
| Listening motion | Calm wave/pulse (PX2) | Already Excellent vs prior neon |
| Review guidance copy | Long instructional line post-transcript | **Implement Now (PQ1)** |
| Thinking / executing | Streaming truthful replies | Already Excellent |
| Success / failure / recovery | Conversation-native; soft vs Settings deny | Already Excellent |
| Notifications | Provider + Conversation language | Track A polish |
| Motion | Reduced-motion respected; purpose-led | Already Excellent post-PX2/PX3 |
| Focus / keyboard / mouse | Composer focus after voice; mic click | Already Excellent |
| Accessibility | Labels + reduced-motion | Already Excellent |
| Window lifecycle | Undecorated transparent; collapse gravity | Already Excellent |
| Tray / updater | Spec’d gates; not user-complete | Future (production gates) |
| Installer | Foundation exists (PI1) | Track A / production |
| Diagnostics / support package | PI3 B1 | Track A trust polish |
| Empty / loading / first-run | Sparse empty; preparing honest; no tour chrome | Track A empty calm |
| Stop/Cancel visual buttons | Keyboard only | Track A |
| Auto-send after voice | Forbidden by F10 | **Reject** |

---

## 13. Premium Benchmark (interaction quality only)

| Reference | Interaction quality to study | Workspace relative |
| --- | --- | --- |
| ChatGPT Desktop | Quiet listen; clear send path | Closer after PX1–PX3; copy was heavier |
| Claude Desktop | Calm composer gravity | Similar intent |
| Raycast | Instant, keyboard-first, minimal chrome | Keyboard path improved; chrome still denser in places |
| PowerToys | Utility clarity without spectacle | Match for honesty; not for Conversation identity |
| Windows Dictation | Stop/cancel familiarity | Keyboard Stop/Cancel aligned |
| VS Code | Predictable command feel | Not the product metaphor — avoid IDE density |

No feature-count comparison. Branding/visual copying forbidden.

---

## 14. Highest-priority friction ranking

| Rank | Friction | Class | Notes |
| --- | --- | --- | --- |
| 1 | Long instructional post-dictation copy | **Implement Now** | Redundant after soft Send (PX3); breaks calm Conversation Quality |
| 2 | No dedicated Stop/Cancel chrome | Track A | Keyboard exists; visual optional |
| 3 | Empty composer has no calm cue | Track A | Placeholder empty by design — light cue possible |
| 4 | Production tray/updater/signing confidence | Future / gates | User production quality |
| 5 | Auto-send after voice | **Reject** | F10 Product Proof |

---

## 15. PQ1 implementation (single improvement)

**Calm post-dictation Conversation line**

Before: `Review your words, then Send — or clear the box to cancel.`  
After: `Send when you're ready.`

- Preserves F10 (review still required; no auto-send)  
- Soft Send cue (PX3) carries the “obvious action”  
- Aligns Interaction Language: fewer words, conversational, calm  

---

## 16. Product Proof guidance (updated)

Owner live Product Proof should expect:

- Dictation → stop (click/Enter/Space) → short Conversation cue → soft Send → Enter  
- Not a tutorial paragraph after every utterance  
- Trust remains: transcript editable; Escape clears; never silent submit  

---

## 17. Explicit answers (PQ1)

| Question | Answer |
| --- | --- |
| Does Workspace now feel more premium? | **Yes, incrementally** — less instructional noise after voice; standard now defines “excellent” for all future work |
| Single largest remaining friction? | **Visual Stop/Cancel affordances** (keyboard exists) and/or **production user confidence** (tray/updater) — Owner chooses experience vs release track |
| What would most increase delight? | Effortless voice→Send with almost no teaching chrome |
| What would most increase trust? | Continued truthful failure + confirmations before desktop mutation |
| What would most increase perceived quality? | Consistency with this standard on every future change |
| What should never be changed? | Spec v2 · F10 review-before-send · Product Gravity · truthful Conversation · Kernel authority |
| What should be improved next? | Track A: Stop/Cancel chrome **or** calm empty composer — one at a time; or Owner-directed production gate |
| Closer to voluntarily chosen desktop product? | **Yes for supported Conversation/Voice workflows** — still pending Owner Product Proof Accept and production gates for release confidence |

---

## 18. Stop

One improvement only. Await Product Owner review.  
No P17. No Constitution reopen. No governance reopen. No architecture investigation. No production gate in this program.
