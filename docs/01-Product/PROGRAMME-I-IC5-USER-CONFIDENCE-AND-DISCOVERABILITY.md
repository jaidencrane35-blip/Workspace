# Programme I — Implementation Contract 5  
# User Confidence & Discoverability

| Field | Value |
|-------|-------|
| **Authority** | Principal Architect |
| **Implementation agent** | Cursor |
| **Programme** | [Programme I](PROGRAMME-I-WORKSPACE-PRODUCT-CAPABILITY.md) |
| **Depends on** | [IC4](PROGRAMME-I-IC4-DESKTOP-LAYOUT-EDITING-REFINEMENT.md) (approved) |
| **Status** | Complete |
| **Date** | 2026-07-30 |
| **Approved to commence** | 2026-07-30 |

---

## Objective (satisfied)

Improve how users understand, trust, and find **Desktop → Arrangement → Restore** without introducing new sources of truth.

After IC5, an engineer reading the code should conclude: *the product became easier to understand without becoming architecturally more complicated.*

---

## Product State vs Interaction State

IC5 makes this distinction explicit in product-shell ownership comments and UI data:

| Kind | Examples | Rule |
|------|----------|------|
| **Product State** | Profile, Desktop (WorkspaceState), Arrangement, Restore | User-owned / authoritative; durable or runtime-true |
| **Interaction State** | Editing, Preview, Selection, Pending changes, Guidance dismissed | Session-only UI; **never persistence**; disappears when interaction ends |

---

## Deliverables implemented

### 1. Derived Arrangement metadata

`deriveArrangementProductMeta` computes from Arrangement entries + live WorkspaceState windows:

- Window count
- Last updated (`updated_at`)
- Overlap with current desktop (present / missing)
- Completeness (bounds)
- Restore readiness (`ready` / `partial` / `not_ready`)

Surfaced on Arrangements list/details and Desktop Arrangement chrome.  
**No metadata cache, persistence additions, or background sync.**

### 2. Save / Update / Restore feedback

Unified interpretation of existing DTOs:

- `Arrangement saved · “{name}” · N windows · completed successfully`
- `Arrangement updated · “{name}” · … · completed successfully`
- `Arrangement restored · “{name}” · {applied/gaps/failed} · completed…`

No notification engine, event history, or operation log.

### 3. First-time guidance

State-derived via `desktopFirstUseGuidance`:

- Appears when Profile has **zero** Arrangements (or no Profile)
- Disappears when Arrangements exist
- **Dismiss** is Interaction State only (session `useState`) — no onboarding persistence

Shown on Desktop and Arrangements panel with the same helper.

### 4. Cross-surface consistency

Shared verbs in `layoutsStageUi`:

- Save · Update · Restore · Edit layout

Workflow language: `Profile · Desktop · Arrangement · Restore`  
Arrangements panel and Desktop use the same feedback helpers and workflow hint.

---

## Architectural reasoning

Every IC5 capability **interprets** existing Arrangement / restore result / WorkspaceState facts. Nothing new owns desktop reality. Restore remains the sole product OS positioning path. Editing remains Interaction State composed in IC3/IC4.

---

## Preserved boundaries

| Constraint | Status |
|------------|--------|
| WorkspaceState sole runtime truth | Preserved |
| No second desktop / arrangement / persistence / editing model | Preserved |
| Restore sole `set_bounds` authority | Preserved |
| Existing IPC ownership | Preserved |
| Observation / Intelligence untouched | Preserved |
| Interaction state never persisted | Preserved |

---

## Validation

- `pnpm typecheck`
- `pnpm test`
- `pnpm build`
- Architecture / IPC / UI experience verifies
- Working tree clean after commit

---

## Files touched

- `app/src/lib/arrangementProductUi.ts` — metadata, feedback, guidance
- `app/src/lib/layoutsStageUi.ts` — shared verbs + workflow line
- `app/src/lib/desktopArrangementUi.ts` — list meta composition
- `app/src/components/WorkspaceApplicationStage.tsx`
- `app/src/components/DesktopArrangementPanel.tsx` / `List` / `Details`
- `app/src/App.css`
- `tests/arrangement-product-ui.test.ts`
- This document; Programme I charter link

---

## Stop condition

IC5 complete. **Await Principal Architect review** before further Programme I contracts.
