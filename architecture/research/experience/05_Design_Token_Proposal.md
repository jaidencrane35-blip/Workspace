# Design Token Proposal

Research ID: EXP-001 · Stage 6
Status: Proposal only — **do not implement in this research session**
Date: 2026-08-02

Aligned with concept-board composition (dark, calm, spatial) and current ship
direction (charcoal + soft blue). Avoids purple-default cliché unless later
brand decision overrides. Windows-native fonts preferred (CSP: no remote fonts
without policy change).

---

## Colour hierarchy

| Token | Role | Proposed value |
|---|---|---|
| `--color-bg` | App background | `#0B1016` |
| `--color-bg-elevated` | Chrome / sticky | `#121A24` |
| `--color-surface` | Card | `rgba(22, 32, 44, 0.92)` |
| `--color-surface-hero` | Centrepiece | slightly brighter / accent wash |
| `--color-border` | Hairline | `rgba(140, 168, 196, 0.16)` |
| `--color-text` | Primary | `#EEF3F8` |
| `--color-text-muted` | Secondary | `#93A4B5` |
| `--color-accent` | Interactive | `#6EB0D6` |
| `--color-accent-strong` | Emphasis | `#8EC8E8` |
| `--color-ok` | Success | `#86EFAC` |
| `--color-danger` | Destructive | `#F87171` |
| `--color-placeholder` | Empty structure | dashed border + 40% mute |

Dark is the Phase 1–2 product surface (concepts + ship). Light theme is
**out of Phase 2 scope** unless later authorised.

---

## Spacing (8-point)

`4, 8, 12, 16, 24, 32, 48, 64`

| Use | Token |
|---|---|
| Card padding | 16–24 |
| Section gap | 24–32 |
| Dashboard gutter | 24 |
| Compact card gap | 12–16 |

---

## Radius

| Token | Value | Use |
|---|---|---|
| `--radius-sm` | 8px | Inputs, chips |
| `--radius-md` | 14px | Standard cards |
| `--radius-lg` | 18–20px | Hero cards |
| `--radius-pill` | 999px | Nav tabs, primary buttons |

---

## Typography

| Role | Stack | Size / weight |
|---|---|---|
| Display | Segoe UI Variable Display, Segoe UI, Candara, sans-serif | 28–40 / 650 |
| Title | same | 20–24 / 600 |
| Body | Segoe UI Variable Text, Segoe UI, Candara, sans-serif | 15–16 / 400 |
| Muted | body | 13–14 / 400 |
| Mono (inspect only) | ui-monospace, Cascadia Mono, Consolas | 12–13 |

Do not load remote webfonts under current CSP.

---

## Elevation

| Level | Treatment |
|---|---|
| 0 | Flat background |
| 1 | Card: soft shadow `0 18px 48px rgba(0,0,0,0.35)` + border |
| 2 | Hero: stronger wash + shadow |
| 3 | Dialog / drawer overlay |

Avoid multi-layer neon glow (concept boards use restraint; trust > spectacle).

---

## Motion

| Token | Value |
|---|---|
| `--motion-fast` | 120ms |
| `--motion-base` | 180ms |
| `--motion-slow` | 240ms |
| Easing | `cubic-bezier(0.2, 0.8, 0.2, 1)` |

Phase 2: hover/focus only. Phase 4: view transitions.

---

## Grid & breakpoints

| Name | Width | Columns |
|---|---|---|
| compact | < 720 | 1 |
| medium | 720–1100 | 2 |
| wide | > 1100 | 3–4 |

Dashboard: CSS grid with `grid-template-columns: repeat(12, 1fr)`.

| Card span | Columns |
|---|---|
| Hero | 12 (or 8+4 with rail) |
| Standard moment | 6 |
| Compact | 4 |
| Metric | 4 |
| Placeholder | 4–6 |

---

## Card sizing

| Variant | Min height | Notes |
|---|---|---|
| Hero | 200–280px | Centrepiece |
| Standard | 140–180px | Recent |
| Compact | 100–120px | Rhythm fillers |
| Placeholder | match standard | Ghost label, no fake titles that look real |

---

## Icon system

- **Lucide** (ISC) as default stroke set, 20px / 1.75 stroke in chrome, 16px in
  dense lists.
- No emoji as UI.

---

## Implementation note

Prefer CSS variables (already started in `App.css`) ± Tailwind theme extension
when/if Tailwind is adopted. Tokens are Experience-owned; not Runtime Host.
