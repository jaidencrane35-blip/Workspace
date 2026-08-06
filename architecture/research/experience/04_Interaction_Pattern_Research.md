# Interaction Pattern Research

Research ID: EXP-001 · Stage 3
Status: Complete
Date: 2026-08-02

Do **not** copy interfaces. Extract principles applicable to interruption-recovery
Experience under Product Proof constraints.

---

## A. Reference set

| Source | Why relevant | What to borrow | What to refuse |
|---|---|---|---|
| VS Code | Recent projects / trusted local tool | Recents density, command palette later, calm chrome | IDE complexity, settings sprawl |
| Linear | Premium desktop-quality web app | Hierarchy, keyboard, issue “memory” lists | Issue-tracker ontology |
| Notion | Empty → inviting structure | Soft empty states, page-as-place | Infinite blocks, collaborative cloud assumptions |
| Arc / browser hubs | Spatial “places” | Spaces as identity (metaphor only) | Browser chrome cloning |
| Raycast / Cursor | Command-first speed | Later command palette; sparse power | Overlay-only product model |
| Obsidian | Local-first vault feeling | Vault = workspace metaphor; local trust | Graph fetish as primary UI |
| Figma | Multiplayer canvas craft | Selection clarity, panel discipline | Infinite canvas as Home |
| Apple HIG | Calm, clarity, deference | Content first, progressive disclosure | macOS-only idioms |
| Microsoft Fluent | Windows desktop native | Density, focus, acrylic *ideas* (not mandatory mica) | Fluent control kits as brand |
| Material Design | Motion/elevation language | Elevation scale ideas | Material components as shell |
| shadcn/ui ecosystem | Practical React composition | Accessible primitives + owned style | Generic “AI SaaS dashboard” skins |

---

## B. Principles for Workspace Experience

### 1. Dashboard-first, not page-first

Home is a **place that remembers**, not a form route. Actions (Save/Continue)
are available from the hub; workflows open as focused overlays or panels when
needed, then return to the hub.

### 2. Intention before mechanism

Lead with handoff text, names, and relative time. Window titles and process
metadata stay behind Inspect. Matches Blueprint: engine serves companion.

### 3. Honest emptiness

Empty states show **structure** (ghost cards, labelled zones) without inventing
saved work. Notion-like invitation ≠ fake history.

### 4. Visual rhythm

Vary card size/weight (hero vs compact). Avoid identical rectangles. Concepts
use masonry/asymmetric grids; Phase 2 must.

### 5. Progressive disclosure

Defaults: glanceable. Details: explicit Inspect / disclosure. Trust limits:
visible but not novel-length on every screen (compact strip + full Guide).

### 6. Calm motion

Short, spatial transitions (120–220ms). Prefer opacity/transform. No playful
confetti; companion principles: calm, predictable.

### 7. Desktop-native focus

Visible focus rings, keyboard tab order through primary actions, dialogs trap
focus (Radix/React Aria patterns).

### 8. Density with air

Professional tools are dense *and* breathable. Use 8px spacing scale; avoid
both vast black voids and spreadsheet packing.

### 9. One centrepiece

Every primary view has a dominant focal region. Home: current/latest moment or
structured empty hero. Continue: selected/last moment.

### 10. No ambient theatre

Do not imitate activity feeds, live thumbnails, or AI briefings without
authority. “Alive” comes from memory of **user-authored** moments + craft.

---

## C. Pattern recipes (Phase-mapped)

| Pattern | Phase | Recipe |
|---|---|---|
| Hub with hero + recents | 2 | VS Code recents × Notion empty structure × concept masonry |
| Moment card | 2–3 | Title, 2–3 line handoff, relative time, soft window count, primary Continue |
| Inspect drawer | 2–3 | Vaul/sheet or disclosure; metadata only after intent |
| Consent check-in | 2–4 | Card stats on top; forms secondary; never research-doc layout |
| View transition | 4 | Shared-element-ish fade between Home and Continue |
| Command palette | 3+ | cmdk; search moments by name/handoff |

---

## D. Typography & spacing lessons

- Display size for workspace name / hero (concepts + Fluent/HIG hierarchy).
- Body 14–16px; muted secondary for timestamps.
- Avoid all-caps section spam (current ship still echoes utility docs).
- 8-point spacing; card padding 16–24; radius large enough to feel modern
  (12–20) without toy pills everywhere.

---

## E. Card system lessons

| Weight | Use |
|---|---|
| Hero | Latest / centrepiece |
| Standard | Recent moments |
| Compact | Older / secondary column |
| Placeholder | Empty structure (dashed/ghost, labelled “Saved moments appear here”) |
| Metric | Check-in numbers only |

---

## F. Anti-patterns observed in current build

- Page-as-document vertical scroll for Home
- Identical card weights
- Empty = single muted sentence
- Forms as the identity of Check-in
- Engineering metadata competing with handoff
