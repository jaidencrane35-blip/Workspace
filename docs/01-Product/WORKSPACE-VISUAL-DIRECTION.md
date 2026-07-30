# Workspace Visual Direction

| Field | Value |
|-------|-------|
| **Purpose** | Define how original concept art guides product development |
| **Owner** | Product / Engineering |
| **Status** | Authoritative visual north star — hierarchy binding; pixels not binding |
| **References** | [`references/workspace-concept-01.png`](references/workspace-concept-01.png), [`references/workspace-concept-02.png`](references/workspace-concept-02.png) |
| **Product contract** | [WORKSPACE-DESKTOP-INTERACTION-MODEL.md](WORKSPACE-DESKTOP-INTERACTION-MODEL.md) (**read first**) |
| **Interpretation rules** | [WORKSPACE-REFERENCE-INTERPRETATION.md](WORKSPACE-REFERENCE-INTERPRETATION.md) |
| **Related** | [PRODUCT-VISION.md](PRODUCT-VISION.md), [WORKSPACE-PRODUCT-REALITY-ALIGNMENT-AUDIT.md](WORKSPACE-PRODUCT-REALITY-ALIGNMENT-AUDIT.md), [PRODUCT-VISION-REALIGNMENT-AUDIT.md](PRODUCT-VISION-REALIGNMENT-AUDIT.md) |

---

## 0. Non-negotiable hierarchy (from human review)

```text
REAL DESKTOP WORKSPACE
        ↓
WORKSPACE REPRESENTATION + CONTROL LAYER
        ↓
ASSISTANT AS OPTIONAL COMPANION
```

| Rule | |
|------|--|
| Applications and desktop reality | **First** |
| Workspace controls (arrange, remember, modes, utilities) | **Second** |
| Assistant | **Third** — optional companion |

The Assistant is **not** the primary product.  
Workspace is **not** an AI dashboard.  
The user does **not** create a workspace inside the app as the main act.  
The application **represents** the user’s existing computing environment.

Full rules, drift examples, and guardrails: [WORKSPACE-REFERENCE-INTERPRETATION.md](WORKSPACE-REFERENCE-INTERPRETATION.md).

---

## 1. Why these images exist

The repository is the source of truth. Concept art must live **in-repo**, not only in chat history.

These images are **authoritative for product hierarchy and interaction intent**.  
They are **not** pixel-perfect UI specifications.

| File | Content |
|------|---------|
| `workspace-concept-01.png` | Switchable layout system — dense productive mode ↔ minimal focus mode, with AI Assistant as a persistent **right rail** (not the stage) |
| `workspace-concept-02.png` | Nine interface layout concepts — explores chrome variants while keeping **apps as the stage** |

**Problem they solve:** how a person sees, organises, and controls an **already-existing** desktop as one coherent workspace — with help available beside it.

**Provenance note:** Files in `references/` capture the original concept direction for engineering. If higher-fidelity source assets become available from Product, replace these files in place and keep this document’s interpretation rules.

---

## 2. What the images represent (interpret this)

Distinguish three layers (see Interpretation doc):

| Layer | Binding |
|-------|---------|
| Visual inspiration (colour, wallpaper, brands) | Not binding |
| Product hierarchy (apps → controls → Assistant) | **Binding** |
| Implementation details (widgets, nav placement, tech) | Flexible |

### Core product (must guide DAF and later milestones)

1. **Real desktop applications** are the primary visual and functional stage.
2. Workspace is a **representation + control layer** over that reality — not a blank canvas the user invents.
3. **Switchable work modes** (e.g. Flow ↔ Focus) transform density while apps stay open.
4. **Grouping and arrangement** of applications into coherent desktop states.
5. **Save and restore** of arrangements (memory of the live desktop).
6. **Per-application utilities** (notably an audio mixer where the OS supports it).
7. **Product chrome** oriented around workspace use — not “AI programmes” as primary navigation.
8. **AI Assistant is tertiary** — supportive sidebar / utility panel, never the main product surface.

### Concept-01 (switchable layouts)

- Left: high-density multitasking (Horizontal Flow).
- Right: low-density immersive focus (Minimal Immersive).
- Slider / transform control: user-driven mode change.
- Same Assistant rail in both modes → Assistant is stable companion chrome; the **desktop stage** changes.

### Concept-02 (layout vocabulary)

Nine explorations of how chrome and utilities can surround the app stage (Classic Grid, Command Center, Horizontal Flow, Vertical Stack, Modular Tiles, Minimal Immersive, Sidebar Focus, Workspace Hub, Professional Suite).

**Do not implement all nine.** They are a design vocabulary. Early product ships **two modes** (productive density + focus density) and a clear path to add presets later.

---

## 3. Core product requirements derived from the art

| Requirement | Product meaning |
|-------------|-----------------|
| Represent desktop apps | Observe and show real OS windows / apps as the stage |
| Control desktop apps | Launch, focus, arrange via the desktop control layer + permissions |
| Workspace layouts | Named arrangements of window geometries (and later groups) |
| Work modes | User-switchable density / presentation of the **same** running apps |
| Persistence | Save and restore arrangements across sessions |
| Utilities | Audio mixer and similar per-app controls as supporting tools |
| Assistant | Explain, suggest, answer; may **request** actions; never silently move windows |

---

## 4. Future concepts (do not build as foundation scope)

- Phone mirroring as a first-class managed window
- Nine simultaneous layout products
- Full scenic wallpaper / dock polish matching marketing art
- Multi-monitor advanced policies
- Plugin marketplace chrome
- Voice-waveform Assistant visualisations
- Autonomous “load my layout because it’s 10:30” without permission

These may appear in the art; they are **roadmap**, not licence to invert hierarchy.

---

## 5. Do not interpret literally

| Art element | How to treat it |
|-------------|-----------------|
| Exact app brands (Chrome, Discord, Spotify, …) | Examples of “real apps”; do not hard-code brand UIs |
| Pixel layout, colours, fonts, wallpaper | Inspiration only |
| “SLIDE TO TRANSFORM” widget | Intent = user-driven mode switch; implementation may be slider, toggle, or segmented control |
| Bottom nav labels | Information architecture target; names can be adjusted |
| System Monitor / Audio Mixer panels | Product capabilities to schedule; not all required immediately |
| Circular Workspace Hub | Optional future navigation metaphor; not required |
| Dense Professional Suite dashboard | Power-user extreme; avoid as default |

---

## 6. Anti-patterns (explicit)

Do **not** use these images to justify:

- Building an AI-first product surface or AI dashboard
- Interpreting Workspace as a setup tool where users **create** a workspace before seeing the desktop
- New intelligence / evidence / recommendation engines for layouts
- Assistant-controlled window movement without Permission Gateway + user intent
- Cloning nine layout engines as separate “programmes”
- Treating the canvas zone board as “done” for window management
- Pixel-perfect reproduction of concept art

---

## 7. Alignment with written vision

> Workspace represents and controls the desktop. AI enhances Workspace. AI does not replace Workspace.

See also:

- [WORKSPACE-REFERENCE-INTERPRETATION.md](WORKSPACE-REFERENCE-INTERPRETATION.md)
- [WORKSPACE-PRODUCT-REALITY-ALIGNMENT-AUDIT.md](WORKSPACE-PRODUCT-REALITY-ALIGNMENT-AUDIT.md)
- [PRODUCT-VISION-REALIGNMENT-AUDIT.md](PRODUCT-VISION-REALIGNMENT-AUDIT.md)
