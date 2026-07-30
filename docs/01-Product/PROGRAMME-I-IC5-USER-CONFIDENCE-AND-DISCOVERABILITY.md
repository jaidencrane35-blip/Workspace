# Programme I — Implementation Contract 5 (Planning)  
# User Confidence & Discoverability

| Field | Value |
|-------|-------|
| **Authority** | Principal Architect |
| **Implementation agent** | Cursor (after approval to commence) |
| **Programme** | [Programme I](PROGRAMME-I-WORKSPACE-PRODUCT-CAPABILITY.md) |
| **Depends on** | [IC4](PROGRAMME-I-IC4-DESKTOP-LAYOUT-EDITING-REFINEMENT.md) (approved) |
| **Status** | **Planned** — awaiting Principal Architect approval to commence |
| **Date** | 2026-07-30 |
| **Nature** | Planning contract — scopes IC5; does not authorise implementation until approved |

---

## Context

Programme I has completed IC1–IC4. IC4 was **approved** by the Principal Architect: editing matured without architectural drift.

The programme has moved from proving architecture to **maturing product experience** on that architecture.

IC5 shifts focus from editing mechanics to **user confidence and discoverability**.

---

## Objective (planned)

Improve how users understand, trust, and find Desktop → Arrangement → Edit → Restore capability — without introducing new sources of truth.

Success means a first-time or returning user can:

1. Discover that Arrangements exist and what they are for.
2. Understand what just happened after Save / Update / Restore.
3. See enough Arrangement context (derived metadata) to choose confidently.
4. Experience consistent language and feedback across Desktop, Arrangements panel, Stage, and Restore.

---

## Architectural constraints (mandatory)

Unchanged from Programme I:

| Constraint | Rule |
|------------|------|
| WorkspaceState | Sole runtime desktop model |
| Arrangement persistence | Existing `DesktopArrangement` only — no second store |
| Restore | Sole product path that issues OS `set_bounds` |
| IPC ownership | Unchanged |
| Observation / Intelligence | Untouched |
| Canvas Layout | Not an HWND / editing authority |
| Sources of truth | **No new ones** — metadata and guidance derive from existing state |

IC5 is a **product confidence sprint**, not an architecture sprint.

---

## Exploration areas (in scope for planning)

### 1. Richer Arrangement metadata (derived)

Surface facts already on Arrangement / observation / restore results, for example:

- Window count
- Last updated (`updated_at`)
- Membership overlap with current desktop (from existing identity match helpers)
- Last restore outcome summary (when a restore result is already in session UI — ephemeral, not new persistence)

**Out of scope unless later approved:** new durable metadata fields, tags store, lock model (IC1 G6).

### 2. Clearer capture / restore success feedback

Unify and enrich transient product messages so Save, Update, and Restore communicate:

- Arrangement name
- Window counts / applied vs gap counts (from existing restore result DTOs)
- Simulated vs real apply (already on restore outcomes)
- Edit-session confirmation already introduced in IC4 — align panel messages with Desktop tone

No new event bus or audit UI unless composed from existing diagnostics views.

### 3. First-time user guidance

Lightweight empty / first-use copy when:

- No Profile selected
- No Arrangements saved
- Desktop map ready but user has never Restored / Edited

Guidance must point to existing actions (Save Arrangement, Edit layout, Restore) — not a tutorial engine or new onboarding persistence.

### 4. Cross-surface consistency

Audit and align naming, button hierarchy, and success/error tone across:

| Surface | Today |
|---------|--------|
| Desktop Stage | Arrangement select, Edit layout, Update, Restore, Done |
| Arrangements panel | Save, Update from desktop, Restore, diagnostics |
| Profiles / Apps | Open Desktop links (IC2) |

Prefer shared copy helpers over duplicated strings. No redesign; refinement only.

---

## Explicit non-goals

- New desktop or Arrangement persistence models
- New IPC for positioning or capture
- Interactive drag/`set_bounds` organise UI (separate from Restore authority)
- OS application discovery (IC1 G1)
- Arrangement lock (IC1 G6)
- Assistant-owned layout control
- Visual redesign or experimental chrome

---

## Proposed deliverables (when approved to implement)

1. Derived Arrangement metadata presentation on Desktop and/or Arrangements list.
2. Harmonised success/failure feedback for capture and Restore.
3. First-use / empty-state guidance tied to the IC2 workflow language.
4. Consistency pass across Desktop ↔ Arrangements ↔ Restore wording.
5. Documentation: this contract updated to **Complete** with validation evidence.
6. Validation: `pnpm typecheck`, `pnpm test`, `pnpm build`; working tree clean.

---

## Validation requirements (implementation phase)

Same Programme I bar:

- No new runtime model
- No duplicate persistence
- Restore remains sole OS positioning authority
- WorkspaceState remains runtime truth
- Green typecheck / test / build
- Clean working tree

---

## Stop condition (this planning document)

IC5 planning is complete when:

- Objectives and constraints are recorded
- In-scope exploration areas are clear
- Non-goals and sources-of-truth rule are explicit

**Cursor must not commence IC5 implementation until the Principal Architect approves this contract for execution.**

---

## Recommendation to Principal Architect

Approve IC5 to commence as a **confidence & discoverability** refinement over IC2–IC4 composition — metadata and feedback derived from existing Arrangement / restore / WorkspaceState facts only.
