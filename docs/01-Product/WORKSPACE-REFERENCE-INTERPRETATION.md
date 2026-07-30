# Workspace Reference Interpretation

| Field | Value |
|-------|-------|
| **Purpose** | Authoritative rules for reading concept reference images — hierarchy and intent, not pixels |
| **Owner** | Product / Engineering |
| **Status** | Binding interpretation after human visual review (2026-07-30) |
| **References** | [`references/workspace-concept-01.png`](references/workspace-concept-01.png), [`references/workspace-concept-02.png`](references/workspace-concept-02.png) |
| **Related** | [WORKSPACE-VISUAL-DIRECTION.md](WORKSPACE-VISUAL-DIRECTION.md), [WORKSPACE-PRODUCT-REALITY-ALIGNMENT-AUDIT.md](WORKSPACE-PRODUCT-REALITY-ALIGNMENT-AUDIT.md), [PRODUCT-VISION.md](PRODUCT-VISION.md) |
| **Audience** | Anyone designing, implementing, or reviewing Workspace UI |

**This document does not authorise implementation.**  
**Do not start Milestone R, F, or UI redesign from this file alone.**

---

## 1. The problem the reference images solve

Modern desktops are fragmented: apps float as disconnected windows; users constantly rearrange; tools either ignore the live desktop or force a separate “setup” world.

The reference images answer:

> How does a person **see, organise, and control their existing computing environment** as one coherent workspace — with help available, but never as the product itself?

They do **not** answer:

- How to build an AI chat product with workspace tabs
- How to design a SaaS admin dashboard for “creating workspaces”
- How to pixel-match wallpaper, brands, or nine chrome variants

### Problem → solution hierarchy (non-negotiable)

```text
REAL DESKTOP WORKSPACE
        ↓
WORKSPACE REPRESENTATION + CONTROL LAYER
        ↓
ASSISTANT AS OPTIONAL COMPANION
```

| Layer | Meaning |
|-------|---------|
| **Real desktop workspace** | The user’s actual applications and windows on Windows — already exist outside this app |
| **Representation + control** | Workspace observes, shows, organises, and controls that reality |
| **Assistant companion** | Optional help beside the workspace — never the centre |

**The Assistant is not the primary product.**  
**Workspace is not an AI dashboard.**  
**The user does not create a workspace inside the application as the main act.**  
**The application represents the user’s existing computing environment.**

---

## 2. Three different things (do not conflate)

| Kind | What it is | Binding? | Examples from the art |
|------|------------|----------|------------------------|
| **Visual inspiration** | Mood, density, polish cues | **No** — not pixel requirements | Colours, wallpaper, glass dock, exact fonts, brand logos, scenic Focus background |
| **Product hierarchy** | What is primary / secondary / tertiary | **Yes — authoritative** | Apps fill the stage; control chrome surrounds; Assistant is a side rail |
| **Implementation details** | How engineering ships the hierarchy | **Flexible** — reuse existing systems | Slider vs Flow/Focus toggle; bottom vs top nav labels; thumbnail tech vs honest placeholders |

### How to use each

1. **Inspiration** — may inform future polish (Milestone I); never blocks shipping hierarchy.  
2. **Hierarchy** — every milestone and review must pass the hierarchy test below.  
3. **Implementation** — prefer existing observation, WindowController, arrangements, Permission Gateway; no new engines for layout/AI.

### Hierarchy test (required)

After any product change, ask:

1. Does the first meaningful view show **desktop/application reality** (or an honest “no windows observed” state of that reality)?  
2. Are **organise / control / remember** actions clearly in service of that reality?  
3. Is the Assistant **optional and secondary** — removable without emptying the product?

If (1) fails because the UI demands “create a workspace” or opens as an AI console, the change is **product drift** — reject it.

---

## 3. Non-negotiable hierarchy (detail)

### First — Applications and desktop reality

- Subject of the product = apps already in the user’s environment.  
- Stage represents that environment (observe → show → act).  
- Empty state means “nothing observed / nothing running,” not “you haven’t designed a workspace yet.”

### Second — Workspace controls

- Arrange, group, save/restore, Flow/Focus, launch, focus window, utilities (e.g. audio later).  
- Controls operate **on** the represented desktop.  
- Named profiles / “workspaces” as labels over reality are optional power tools — **not** a gate to seeing the desktop.

### Third — Assistant

- Companion rail / panel.  
- Explain, suggest, answer; may **request** gated actions.  
- Never owns navigation, never replaces the stage, never silently moves windows.

```text
Priority:  Applications & desktop reality  >  Workspace controls  >  Assistant
```

---

## 4. Examples of product drift (observed / forbidden)

| Drift | What it looks like | Why it violates the references |
|-------|--------------------|--------------------------------|
| **AI as primary surface** | Assistant tab or chat is the first thing users must use; product feels like an AI app with extras | Art keeps Assistant as a **side rail** beside a full app stage |
| **AI dashboard** | Briefings, programmes, evidence queues dominate chrome | Hierarchy puts desktop reality first |
| **Dashboard / setup interpretation** | Home/Stage teach “configure your environment” with admin forms and go-to panels | Art shows an already-populated computing environment |
| **Workspace creation instead of representation** | “Create a workspace” / “Choose a workspace first” before any desktop map | User’s workspace **already exists** on the OS; the app represents it |
| **Registry-as-product** | Manual app cards replace observed windows as the Stage hero | Art shows live applications, not a membership admin list |
| **Canvas mistaken for desktop** | Zone board treated as “done” window management | Companion practice ≠ OS control layer |
| **Pixel obedience** | Blocking work to clone wallpaper, brands, or all nine layouts | References are hierarchy/intent — not a mock to photocopy |

Related finding: [WORKSPACE-PRODUCT-REALITY-ALIGNMENT-AUDIT.md](WORKSPACE-PRODUCT-REALITY-ALIGNMENT-AUDIT.md).

---

## 5. Future implementation guardrails

Apply on every batch, PR, and human review:

1. **Hierarchy before polish** — Never ship AI-first or setup-first chrome “temporarily.”  
2. **Represent before invent** — Prefer observation of existing apps over forcing create/register flows.  
3. **Controls serve reality** — Arrangements, modes, and grouping act on real windows via existing control + permission paths.  
4. **Assistant last** — No new AI engines; no Assistant-owned window movement; companion may shrink or hide.  
5. **No parallel products** — Do not build a second layout engine, mode engine, or admin “workspace builder” product beside the desktop layer.  
6. **Reuse infrastructure** — Window observation, WindowController, DesktopArrangement, Permission Gateway, registry/launch remain; demote misuse in IA/copy, do not delete.  
7. **Honest emptiness** — If thumbnails or live previews are unavailable, say so; do not fake OS windows; do not substitute “create workspace” as the empty-state story.  
8. **Concept art usage** — Cite hierarchy/intent in reviews; do not cite art to demand pixel clones or nine simultaneous layouts.  
9. **Milestone discipline** — Do not start Milestone R/F/G/… from interpretation docs alone; await explicit product authorisation.  
10. **Identity test** — If the UI could pass for a generic management SaaS after removing brand + Assistant, it has drifted.

---

## 6. One-sentence summary

> The references mean: **show and control the user’s real desktop; keep the Assistant optional; never make setup or AI the product.**

---

## 7. Stop

Interpretation documentation only.  
**Await human confirmation** before any implementation (including Milestone R).
