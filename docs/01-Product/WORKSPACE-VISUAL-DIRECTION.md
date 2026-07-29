# Workspace Visual Direction

| Field | Value |
|-------|-------|
| **Purpose** | Define how original concept art guides product development |
| **Owner** | Product / Engineering |
| **Status** | Authoritative visual north star for DAF and subsequent desktop work |
| **References** | [`references/workspace-concept-01.png`](references/workspace-concept-01.png), [`references/workspace-concept-02.png`](references/workspace-concept-02.png) |
| **Related** | [PRODUCT-VISION.md](PRODUCT-VISION.md), [PRODUCT-VISION-REALIGNMENT-AUDIT.md](PRODUCT-VISION-REALIGNMENT-AUDIT.md) |

---

## 1. Why these images exist

The repository is the source of truth. Concept art must live **in-repo**, not only in chat history.

These images represent the **intended product direction**: Workspace as a desktop workspace management application. They are **visual references**, not pixel-perfect UI specifications.

| File | Content |
|------|---------|
| `workspace-concept-01.png` | Switchable layout system — dense productive mode ↔ minimal focus mode, with AI Assistant as a persistent right rail |
| `workspace-concept-02.png` | Nine interface layout concepts — explores chrome variants while keeping apps as the stage |

**Provenance note:** Files in `references/` capture the original concept direction for engineering. If higher-fidelity source assets become available from Product, replace these files in place and keep this document’s interpretation rules.

---

## 2. What the images represent (interpret this)

### Core product (must guide DAF)

1. **Real desktop applications** are the primary visual and functional stage (browser, chat, editor, media, system tools).
2. **Switchable workspace layouts / work modes** (e.g. Flow ↔ Focus) transform density while apps stay open.
3. **Grouping and arrangement** of applications into coherent workspace states.
4. **Save and restore** of arrangements.
5. **Per-application utilities** (notably an audio mixer where the OS supports it).
6. **Product chrome** oriented around workspace use: Home, Apps, Layouts, Automation, Files, Settings — not “AI programmes” as primary navigation.
7. **AI Assistant is secondary** — a supportive sidebar / utility panel, never the main product surface.

### Concept-01 (switchable layouts)

- Left: high-density multitasking (Horizontal Flow).
- Right: low-density immersive focus (Minimal Immersive).
- Slider / transform control: user-driven mode change.
- Same Assistant rail in both modes → Assistant is stable chrome; the **workspace stage** changes.

### Concept-02 (layout vocabulary)

Nine explorations of how chrome and utilities can surround the app stage (Classic Grid, Command Center, Horizontal Flow, Vertical Stack, Modular Tiles, Minimal Immersive, Sidebar Focus, Workspace Hub, Professional Suite).

**DAF does not implement all nine.** They are a design vocabulary. Early DAF ships **two modes** (productive density + focus density) and a clear path to add presets later.

---

## 3. Core product requirements derived from the art

| Requirement | Product meaning |
|-------------|-----------------|
| Manage desktop apps | Discover, launch, track, and arrange real OS windows |
| Workspace layouts | Named arrangements of window geometries (and later groups) |
| Work modes | User-switchable density / presentation of the same running apps |
| Persistence | Save and restore arrangements across sessions |
| Desktop control | Workspace executes window moves/resizes via the desktop control layer |
| Utilities | Audio mixer and similar per-app controls as supporting tools |
| Assistant | Explain, suggest, answer; may **request** actions; never silently move windows |

---

## 4. Future concepts (do not build in DAF-0 / DAF-1)

- Phone mirroring as a first-class managed window
- Nine simultaneous layout products
- Full scenic wallpaper / dock polish matching marketing art
- Multi-monitor advanced policies
- Plugin marketplace chrome
- Voice-waveform Assistant visualisations
- Autonomous “load my layout because it’s 10:30” without permission

These may appear in the art; they are **roadmap**, not DAF foundation scope.

---

## 5. Do not interpret literally

| Art element | How to treat it |
|-------------|-----------------|
| Exact app brands (Chrome, Discord, Spotify, …) | Examples of “real apps”; do not hard-code brand UIs |
| Pixel layout, colours, fonts, wallpaper | Inspiration; follow UX principles + existing shell until a design system lands |
| “SLIDE TO TRANSFORM” widget | Intent = user-driven mode switch; implementation may be slider, toggle, or segmented control |
| Bottom nav labels | Information architecture target; names can be adjusted |
| System Monitor / Audio Mixer panels | Product capabilities to schedule; not all required in DAF-1 |
| Circular Workspace Hub | Optional future navigation metaphor; not required |
| Dense Professional Suite dashboard | Power-user extreme; avoid as default |

---

## 6. Anti-patterns (explicit)

Do **not** use these images to justify:

- Building an AI-first product surface
- New intelligence / evidence / recommendation engines for layouts
- Assistant-controlled window movement without Permission Gateway + user intent
- Cloning nine layout engines as separate “programmes”
- Treating the canvas zone board as “done” for window management

---

## 7. Alignment with written vision

Written product docs and these images agree:

> Workspace manages the desktop. AI enhances Workspace. AI does not replace Workspace.

See also: [PRODUCT-VISION-REALIGNMENT-AUDIT.md](PRODUCT-VISION-REALIGNMENT-AUDIT.md).
