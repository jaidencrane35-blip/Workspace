# Milestone A — Vision Gap Report

| Field | Value |
|-------|-------|
| **Status** | Review only — no implementation |
| **Date** | 2026-07-29 |
| **Branch inspected** | `cursor/milestone-a-workspace-apps-switcher-34a5` |
| **Visual references** | [docs/01-Product/references/](../01-Product/references/), [WORKSPACE-VISUAL-DIRECTION.md](../01-Product/WORKSPACE-VISUAL-DIRECTION.md) |
| **Related** | [MILESTONE-A-HUMAN-REVIEW.md](MILESTONE-A-HUMAN-REVIEW.md), [WORKSPACE-PRODUCT-ALIGNMENT-AUDIT.md](../01-Product/WORKSPACE-PRODUCT-ALIGNMENT-AUDIT.md) |

**Not a feature batch. Do not expand AI from this document.**

Images are interpreted for hierarchy and workflow intent — not pixel copying.

---

## Original direction (principles extracted)

| Principle | Meaning |
|-----------|---------|
| Workspace-first | Apps / layouts / switching are the product stage |
| Calm professional UI | Clear hierarchy; low chrome noise |
| Application control | See, launch, arrange real desktop applications |
| Saved arrangements | Named setups restore on demand |
| Assistant as support | Persistent side capability — not navigation centre |
| Human workflow priority | User drives modes and restores; AI does not own control |

---

## Gap table

| Vision capability | Current capability (post Milestone A) | Missing capability | Priority |
|-------------------|----------------------------------------|--------------------|----------|
| Manage applications | Product **Applications** tab: register, identity, governed launch; observed actives via WorkspaceState | OS discovery / app tiles as stage; grouping; per-app utilities (audio) | High |
| Arrange applications | DAF-1a–1e capture/restore + arrangements rail on Workspaces/Layouts | Group move/resize; lock OS arrangements; feel of apps-as-stage | High |
| Switch workspace environments | **WorkspaceSwitcher** list/create/activate + current workspace on Home | Visual switcher (tiles/previews); fast “desktop tool” feel vs form | High |
| Control layouts / modes | Canvas zones + desktop arrangements; Layouts tab | Flow ↔ Focus (or equivalent) density modes | High |
| Improve workflow | Foundations present; empty states dominate in browser preview | End-to-end daily loop polish; multi-monitor; chrome simplification | Medium |
| Assistant side panel | Demoted tab + copy; dashed secondary styling | True persistent sidecar; not peer full-page surface | High (chrome) |
| Audio controls | None | Per-app audio | Future |
| Phone mirroring / scenic chrome | None | Deferred | Future |

---

## Alignment verdict

Milestone A **correctly redirected navigation** toward Home / Workspaces / Applications / Layouts and reused existing IPC.  

It does **not** yet deliver the concept-art experience of “applications on stage with switchable modes and a side Assistant.” Surfaces still read partly as **admin forms** over a companion shell, especially in browser Vite empty states.

Trajectory is good. Product gap remains concentrated in **modes, apps-as-stage feel, and Assistant chrome**.

---

## Recommended next milestone

**Preferred next batch: Milestone B — Work modes foundation** (productive ↔ focus), extending arrangements without new AI engines.

**Optional preceding polish (A.1)** if human prefers UX hardening before modes:

- Soften browser-runtime banner (info vs error)
- Further demote / group Work + Diagnostic
- Clearer Apps↔Layouts relationship copy; fix stale “Workspace tab” Assistant strings
- Richer empty-state education (no fake data)

Non-goals for either batch: Batch 17, new intelligence engines, Assistant-owned window movement.
