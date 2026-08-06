# Execution Program — P6 Desktop Operator Shell Completion
## Milestone: Collapse becomes a true shell mode

| Field | Value |
| --- | --- |
| **Program** | P6 Desktop Operator Shell Completion |
| **Date** | 2026-08-07 |
| **Prior** | P5 (`4316230`) — two-form model; restore UX incomplete |
| **Authority** | Product Owner review (authoritative) |
| **Handoff** | `AWAITING_PROJECT_OWNER_SHELL_COMPLETION_REVIEW` |

---

## Defect addressed

Collapse was still experienced as a **tiny application / resized conversation**.  
Restore required a **context menu** because drag-on-pointerdown stole the click.

Both rejected. Replaced.

---

## Architecture (stable)

```
Conversation Window  ⇄  Desktop Operator
```

These are **different shell modes** (different windows), not large→small resize.

| Form | Window | When active |
| --- | --- | --- |
| Conversation | `main` | Only surface; operator hidden; taskbar = conversation |
| Desktop Operator | `operator` | Only surface; conversation hidden + off taskbar |

---

## Interaction contract

| Action | Result |
| --- | --- |
| Collapse / Close conversation | Conversation disappears completely; companion appears |
| Single click operator | Conversation restores immediately |
| Double-click operator | Conversation restores immediately |
| Drag (after move threshold) | Moves companion — does not steal click |
| Exit | Process ends (from conversation Exit) |

No context menu required for restore. No right-click workflow for open.

---

## Tracks (owner direction)

| Track | Owns |
| --- | --- |
| **A — Shell** | Lifecycle, operator, windows, tray, polish |
| **B — Desktop capabilities** | Launch, clipboard, screenshots, automation… |
| **C — Intelligence** | Memory, routing, personalization… |

Capabilities and intelligence plug in **after** shell excellence.

---

## Product Owner checklist

- [ ] Collapse → conversation gone; companion only
- [ ] Companion is not a miniature conversation window
- [ ] Click (or double-click) restores instantly — no menu
- [ ] Drag moves companion
- [ ] Exit fully exits; no orphan terminals after Exit
- [ ] Fresh runtime for this review

---

## Stop

Await Product Owner approval. Reassess next program from owner feedback — do not assume prior roadmap.
