# Production friction log

Authority: concept boards, `convergence-pass-07/`, commit `67a4fa1`, populated demo at `http://127.0.0.1:1420/`.

Method: Playwright journeys + keyboard/resize/rapid-nav probes (`validate-production.mjs`). Severity 1–5. Fix size S/M/L.

Status after this sprint’s fixes is noted on each item.

---

## F-01 — View swap key tied to intent

| Field | Value |
| --- | --- |
| journey | 1 / 3 / dock |
| step | Home → Save (and other dock switches) |
| expected | Dock destination replaces the content region |
| actual | Content could remain on the previous view while the live region announced the new intent (`Capture · Save`) and a “Back to Workspace” command appeared over Home |
| component | `WorkspaceShell` (`AnimatePresence` key) |
| severity | 5 |
| fix size | S |
| status | **Fixed** — content key is `view` only (was ``${intent}-${view}``) |

---

## F-02 — Satellite Moments not keyboard-reachable

| Field | Value |
| --- | --- |
| journey | interaction |
| step | keyboard-only navigation on Home |
| expected | Earlier Moments can be focused and activated with keyboard |
| actual | Only hero Continue + dock tabs were tab stops; sparse satellites were mouse-only `div` clicks |
| component | `MomentCard` / `WorkspaceObject` |
| severity | 4 |
| fix size | S |
| status | **Fixed** — sparse Moments are `role="button"` with Enter/Space when they do not nest action buttons |

---

## F-03 — Layout shift on destination change

| Field | Value |
| --- | --- |
| journey | performance |
| step | repeated Home ↔ Continue |
| expected | CLS under ~0.1 during realistic dock use |
| actual | Measured CLS **0.45** across full validation; **0.11** on Home↔Continue alone — driven by layout animations + `y` translation on view reveal |
| component | `WorkspaceShell`, `WorkspaceCanvas`, `motion.ts` reveal |
| severity | 2 |
| fix size | M |
| status | **Fixed (measurable)** — `layout={false}` on spatial/canvas shells; reveal is opacity-only. Remeasured Home↔Continue CLS **0.092** |

---

## F-04 — Exiting view left focusable controls in tab order

| Field | Value |
| --- | --- |
| journey | interaction |
| step | Tab after switching destinations |
| expected | Only the active view’s controls are tabbable |
| actual | During/after exit, prior view controls (e.g. Inspect) could remain in the focus order |
| component | `WorkspaceShell` / `AnimatePresence` |
| severity | 3 |
| fix size | S |
| status | **Mitigated** — exit sets `pointerEvents: "none"`; presence/`inert` attempt reverted after it blocked view swaps (see F-01). Residual risk if exit animation is slow |

---

## F-05 — Save is a two-step commit (Review → Save)

| Field | Value |
| --- | --- |
| journey | 1 |
| step | Save a new Moment |
| expected | Clear path from writing a handoff to a saved Moment |
| actual | Primary control is “Review what will be saved”; “Save this context” appears only after review. Easy to under-click in automated/first-time use; not a failure once understood |
| component | `SaveContextPanel` |
| severity | 2 |
| fix size | S |
| status | **Fixed (freeze)** — review emerges from writing; single Save CTA; Clear undoes draft |

---

## F-06 — Inspect control sits near dock band

| Field | Value |
| --- | --- |
| journey | 1 |
| step | Continue browse — Inspect |
| expected | Inspect is easy to target without competing with the dock |
| actual | Inspect summary sits low in the viewport; measured gap to dock ~48px after pass-07 (no geometric overlap), but still peripheral and easy to miss |
| component | `ResumeContextPanel` / `.continue-inspect-entry` |
| severity | 2 |
| fix size | S |
| status | **Fixed (freeze)** — single quiet Inspect button; subordinate to Continue |

---

## F-07 — Check-in chapter has no landmark navigation

| Field | Value |
| --- | --- |
| journey | 2 |
| step | Check-in chapters |
| expected | User can move between pulse chapters without guessing |
| actual | Wizard chips removed (parity CK-03); only Next / Save actions advance. No visible chapter index |
| component | `PilotMeasurementPanel` |
| severity | 2 |
| fix size | M |
| status | **Fixed (freeze)** — semantic landmarks + sr-only chapter nav; visuals unchanged |

---

## Summary

| ID | Severity | Status |
| --- | --- | --- |
| F-01 | 5 | Fixed |
| F-02 | 4 | Fixed |
| F-03 | 2 | Fixed (measured) |
| F-04 | 3 | Mitigated |
| F-05 | 2 | Fixed (freeze) |
| F-06 | 2 | Fixed (freeze) |
| F-07 | 2 | Fixed (freeze) |
