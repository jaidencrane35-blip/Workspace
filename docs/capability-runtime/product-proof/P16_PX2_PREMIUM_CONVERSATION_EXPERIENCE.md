# P16.PX2 — Premium Conversation Experience

| Field | Value |
| --- | --- |
| **Program** | P16.PX2 — Premium Conversation Experience |
| **Kind** | Product experience polish (not production gate; not constitutional; not P17) |
| **Date** | 2026-08-07 |
| **Commit** | `318affb` |
| **Prior** | P16.PX1 (`d4ffb90`) — capture keyboard Stop/Cancel |
| **Constraint** | Spec v2 · Engineering Execution Standard · Production Gate Spec (unchanged) |
| **Rule** | One isolated UX improvement → Product Owner review |

---

## Product design heuristic (enduring, not constitutional)

> When there is a choice between exposing engineering complexity and reducing user friction, prefer reducing user friction unless doing so violates the Constitutional Specification or Product Proof requirements.

This is a **product design heuristic**, not a constitutional law and not a new meta-framework.  
Engineering foundation is strong; remaining differentiation is effortless feel.

Recorded also in `docs/project/ENGINEERING_HANDOFF.md`.

---

## 1. Premium Conversation Experience report

### Repository reassessment (evidence)

| Area | Evidence | Premium read |
| --- | --- | --- |
| Composer | `OperatorRoot` textarea + mic + Send; Enter send; Escape clear | Correct; still stepwise after voice |
| Voice controls | Single mic; PX1 keyboard Stop/Cancel | Interaction better; **visual chrome still loud** |
| Listening / speaking | 4-bar wave + neon rings + unicode glyphs (pre-PX2) | **Low perceived quality** (Owner PP) |
| Dictation lifecycle | F10 review-before-send | Trust-correct; keep |
| Stop/Cancel | Mic toggle + Enter/Space/Esc (PX1) | Keyboard path shipped |
| Review-before-send | Transcript → draft focus/select | Honest; Send still required |
| Transitions / motion | Phase colors + preparing blink | Functional, not calm |
| Focus | Composer autofocus; Settings return | Adequate |
| Errors | Soft fail vs Settings deny | Clear; copy can be long |
| Empty / loading | Preparing + streaming replies | Honest |
| A11y / keyboard | Labels; PX1 capture keys | Keyboard-first improved |

### Premium dimensions

| Dimension | Assessment |
| --- | --- |
| Cognitive load | Phase glyphs + bright rings competed with Conversation |
| Interaction count | Reduced by PX1; Send after review remains (F10) |
| Discoverability | Mic present; tooltips advertise keyboard stop |
| Responsiveness | Preparing state honest |
| Visual hierarchy | Mic capture chrome was louder than transcript |
| Perceived latency | Preparing helps; motion quality hurt “live” feel |
| Consistency | Ready green vs listening cyan felt status-board like |
| Delight | Weak before PX2 motion |
| Trust | Strong (truthful failures, F10) |
| Accessibility | Reduced-motion disables mic animations |
| Keyboard efficiency | PX1 Enter/Space/Esc during capture |

### Owner Product Proof evidence (not reinterpreted)

1. Dictation should feel closer to modern conversational interfaces.
2. Current speaking/listening animation feels low quality.
3. Voice controls should be visually simpler.
4. Conversation composer should reduce friction.
5. Product should feel calm, lightweight, and trustworthy.
6. Conversation surface remains the primary product experience.

### Comparison (interaction quality only)

Mature conversational apps use **quiet idle controls**, **one clear listening motion**, and **minimal status glyph stacking**. Workspace already owns trust (review-before-send). Gap was **visual calm + listening motion quality**, not missing architecture.

---

## 2. Remaining UX friction inventory

| ID | Friction | Status after PX2 |
| --- | --- | --- |
| F1 | Capture stop required mic re-click | Addressed in PX1 |
| F2 | Listening animation / neon chrome low quality | **Addressed in PX2** |
| F3 | No dedicated Stop/Cancel buttons | Open (Track A) |
| F4 | Review → Send still an extra step | Open (F10 — keep; soften emphasis Track A) |
| F5 | Long instructional voice copy | Open (Track A) |
| F6 | Composer friction beyond voice | Open (Track A) |
| F7 | Hold-to-talk | Future |
| F8 | Auto-send after dictation | Rejected (F10) |

---

## 3. Prioritized improvements

| ID | Improvement | Class |
| --- | --- | --- |
| **PX2-1** | Calm premium listening motion + quieter mic chrome (CSS + glyph quieting) | **Implement Now** |
| PX2-2 | Soft Send emphasis when draft filled from voice | **Track A** |
| PX2-3 | Lightweight Stop/Cancel affordances (Workspace identity) | **Track A** |
| PX2-4 | Shorter reviewing / voice guidance copy | **Track A** |
| PX2-5 | Composer empty-state calm (placeholder / focus ring polish) | **Track A** |
| PX2-6 | Hold-to-talk | **Future** |
| — | Production gates / P17 capabilities | Out of PX2 |

---

## 4. Implementation (single improvement)

**Shipped: PX2-1 — premium calm capture visuals**

- `app/src/App.css` — softer ready/listening fills; breathing pulse; centered 5-bar wave with smoother easing
- `app/src/components/operator/VoiceMicButton.tsx` — ready/listening hide center glyphs (`data-quiet`); wave carries speaking state
- Reduced-motion path unchanged (animations still disabled)
- F10 review-before-send unchanged; no auto-send; no new capabilities
- Verifier: `scripts/verify-voice-input.mjs` (P16.PX2 checks)

---

## 5. Product Proof expectations (updated)

| Expectation | Notes |
| --- | --- |
| Live dictation | Owner should feel listening is **calm and modern**, not neon/status-board |
| Stop | Still: click, Enter, or Space (PX1); Esc cancels |
| Review | Transcript still requires Send — intentional Product Proof honesty |
| Conversation gravity | Mic chrome must not overpower transcript (PX2 reduces mic visual noise) |
| Next polish | Prefer PX2-2/PX2-3 one-at-a-time after Owner review |
| Alternation | After experience sprints: resume production gates (e.g. A1) when Owner directs — not auto |

---

## 6. Verification

```bash
node scripts/verify-voice-input.mjs
```

---

## 7. Stop

One improvement only. Await Product Owner review. No A1, no P17, no further PX work until directed.
