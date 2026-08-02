# Experience Roadmap — Post LEDGER-0034

Status: Accepted sequencing guide for Experience presentation
Authority: Subordinate to Blueprint, LEDGER-0013/0031, Product Proof behaviour,
and contracts. Does not supersede capability research (`ROADMAP-001`).
Version: 1.0
Date: 2026-08-02

Session mode: Experience product sequencing (documentation). Conversation is
not authority. Participant #1 review after LEDGER-0034 and the concept boards
are accepted inputs for this roadmap only.

---

## 1. Purpose

Evolve how Workspace is *presented* so the shipped Experience matches the
companion identity in the concept boards — without adding product capabilities,
changing trust guarantees, or altering Product Proof Save/Continue behaviour.

This is intentionally **not** a capability roadmap. Next gains come from how
existing architecture is shown, not from new underlying power.

---

## 2. Current standing (after LEDGER-0034)

| Measure | Assessment |
|---|---|
| Before Phase 1 | ~3/10 — engineering validation utility |
| After Phase 1 (now) | ~6.5–7/10 — early professional desktop application |
| Product visibility | The intended product is visible |
| Remaining gap | Personality, aliveness, and dashboard-first memory |

### What Phase 1 achieved

- Home communicates “My Workspace” instead of absence.
- Navigation is product-oriented: Home / Save / Continue / Check-in / Guide.
- Empty states invite a first moment rather than lecturing about missing data.
- Typography and basic card hierarchy exist.
- Check-in and Guide are calmer cards, not raw research chrome.
- Active workspace persistence works (LEDGER-0033).

### What remains (Participant #1 review)

1. **Not alive** — Home still reads empty: title, two buttons, one card, space.
2. **Little visual memory** — concepts feel like they remember you; the build
   still feels nearly stateless.
3. **Continue is too blank** — needs recent-projects energy, not “no pages.”
4. **Uniform cards** — identical weight; concepts use varied rhythm.
5. **No Home centrepiece** — nothing dominates as “what Workspace is.”
6. **Check-in / Guide still page/form/read** — nicer forms and reading, not yet
   show-first companion surfaces.

### Core diagnosis

| Model | Shape |
|---|---|
| Concept boards | **Dashboard-first** — Workspace → hub of memory and action |
| Current build | **Page-first** — Workspace → page → fill in / read |

Closing that gap is the job of Phases 2–4.

---

## 3. Non-negotiable constraints (every phase)

- No new product capabilities.
- No ambient observation, hidden execution, or AI-generated handoffs.
- No contract, ownership, or trust-guarantee changes.
- Product Proof Save → review → confirm and Continue → preview → approve remain
  explicit and honest.
- Restore limits stay truthful (same session, still-open windows, no silent
  relaunch).
- Check-in measurement stays consented and local.
- Prefer surfacing data Workspace already owns (saved contexts, handoff notes,
  timestamps, window counts, workspace name). Do not invent activity.
- Empty states may use **placeholder structure** that shows the shape of memory
  before data exists — without fabricating saved work.
- LEDGER-0013 pilot evidence remains the programme objective; this roadmap
  sequences Experience presentation in parallel and must not delay or replace
  pilot recruitment.

---

## 4. Phased roadmap

### Experience Phase 1 — Companion identity

**Status: Largely achieved (LEDGER-0034)**

Intent: When the app launches, it should feel like *my workspace*, not an
internal tool.

Done when:

- Product-oriented navigation is primary chrome.
- Home is the default landing.
- Empty states invite beginning work.
- Engineering metadata is secondary to intention.
- Participant reaction shifts from “internal tool” toward “product emerging.”

Exit criterion met for identity. Remaining work is personality and density, not
re-establishing identity from zero.

---

### Experience Phase 2 — Dashboard-first Home with meaningful recent activity

**Status: Structurally attempted (LEDGER-0037); experientially rejected by
Participant #1; Recovery Sprint LEDGER-0038 supersedes presentation approach**

Intent: Home becomes the centre of the product — a living hub of memory and
obvious next actions, not a heading-plus-buttons page.

Primary question for this phase:

> How can Home become the centre of the product?

Target composition (presentation only):

- A **centrepiece** the eye lands on (current / most-relevant moment, or a
  clear “start here” spatial block when empty).
- **Today / recent moments** as a glanceable strip or masonry of uneven cards
  (name, handoff snippet, relative time, window count as soft signal — not
  process IDs).
- Primary actions available from the hub (Save this moment, Continue) without
  feeling like form entry is the first job.
- Empty Home still shows the *shape* of a remembered workspace (placeholder
  cards / zones) so the screen never feels like “nothing.”
- Varied card sizes and visual rhythm; avoid identical-weight rectangles.

Explicitly out of scope for Phase 2:

- Ambient desktop watching to fabricate “activity.”
- AI summaries of what the user was doing.
- New restore/launch capabilities.
- Pixel-perfect reproduction of concept boards (composition and hierarchy only).

Done when Participant #1 (or reviewer) can open Home and feel the product
*remembers* and *invites continuation*, even with sparse data — and the mental
model is dashboard-first rather than page-first.

---

### Experience Phase 3 — Rich visual continuation and saved moments

**Status: Sequenced after Phase 2**

Intent: Continue becomes a remembered library of moments — closer to recent
projects / mail than a blank empty state.

Target composition:

- Dense-but-calm list or card gallery of saved moments led by handoff and time.
- Clear “continue this” affordance as the default action; Inspect remains
  secondary.
- Soft visual differentiation (featured last-used, group by day if useful)
  without claiming confidence Workspace does not have.
- Empty Continue invites exploration / first Save with structure, not a lone
  muted sentence.
- Optional light visual cues for “still open in this session” only if already
  knowable from existing Product Proof data — never invent match state before
  preview.

Done when Continue feels like returning to a shelf of work, not opening an
empty CRUD page.

---

### Experience Phase 4 — Refined polish, motion, and transitions

**Status: Sequenced after Phase 3**

Intent: Personality through craft — motion, spacing, transitions — without
new product surface area.

Target craft:

- Intentional enter/leave transitions between Home ↔ Save ↔ Continue.
- Hover/focus states that feel desktop-native.
- Hierarchy tightening (centrepiece vs supporting rails).
- Check-in and Guide move further from “form/document” toward calm companion
  panels that still hold the same truthful content.
- Reduce remaining page-scroll form feeling without removing required consent
  or measurement fields.

Done when the app feels alive in micro-interaction, not only in layout
structure.

---

## 5. Sequencing rules

1. Complete or explicitly accept Phase N before starting Phase N+1 coding.
2. Do not interleave capability research or `PP-M1-03` into Experience phases
   unless repository authority separately authorises it.
3. Prefer one Experience phase per implementation session; stop when the phase
   exit criterion is met rather than chasing endless polish.
4. If a phase would require ambient observation or new contracts, **stop and
   report** — redesign the presentation within existing data.
5. Pilot dogfood feedback may reorder Phase 2–4 emphasis but must not silently
   widen Product Proof claims.

---

## 6. Relationship to other authorities

| Authority | Relationship |
|---|---|
| LEDGER-0013 / Product Proof Review | Programme objective remains pilot evidence |
| LEDGER-0034 / `20_Experience_Fidelity_Review.md` | Phase 1 convergence record; this roadmap continues from it |
| `ROADMAP-001` | Capability research order — unchanged and not active |
| Contracts / SCRI / ADM | Unchanged; Experience may only reorder presentation |

---

## 7. Recommended next action

**Do not start an open-ended UI tweak sprint.**

When Experience implementation resumes, authorise **Experience Phase 2 only**:
dashboard-first Home with meaningful recent activity from data Workspace
already has, plus honest empty-structure that never feels dead.

Until then, Participant #1 continues dogfood on the LEDGER-0034 build, and
programme attention stays on LEDGER-0013 pilot execution.
