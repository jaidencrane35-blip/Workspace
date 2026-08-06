# Experience Build Plan

Research ID: EXP-001 · Stage 7–8
Status: Planning artefact — awaits implementation authorisation
Date: 2026-08-02

Maps Experience Roadmap phases to component work, ranked by UX gain, effort,
risk, architectural impact, reuse, and pilot value.

Scoring: 1 (low) – 5 (high). **Priority score** ≈ UX×2 + Pilot×2 + Reuse − Effort − Risk − ArchImpact.

---

## A. Ranked work packages

| Rank | Package | Phase | UX | Effort | Risk | Arch | Reuse | Pilot | Priority | Depends on |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | Dashboard-first Home (hero + recents + empty structure) | 2 | 5 | 3 | 2 | 1 | 4 | 5 | **High** | Existing list_saved_contexts |
| 2 | Moment card system (hero/standard/compact/placeholder) | 2 | 5 | 2 | 1 | 1 | 5 | 5 | **High** | Tokens |
| 3 | Design tokens + Tailwind foundation | 2 | 4 | 3 | 2 | 2 | 5 | 4 | **High** | Adoption decision |
| 4 | Primitive kit (Button/Input/Textarea/Tabs/Dialog) | 2 | 4 | 3 | 2 | 2 | 5 | 3 | **High** | Tailwind/shadcn |
| 5 | Relative time + honest activity summary | 2 | 4 | 1 | 1 | 1 | 4 | 4 | **High** | date-fns |
| 6 | Continue library density + day grouping | 3 | 5 | 3 | 2 | 1 | 4 | 5 | High | Moment cards |
| 7 | Inspect drawer + AlertDialog delete | 3 | 3 | 2 | 1 | 1 | 5 | 3 | Med | Dialog/drawer |
| 8 | Command palette for moments | 3 | 3 | 2 | 2 | 1 | 4 | 2 | Med | cmdk |
| 9 | View transitions + hover craft | 4 | 4 | 3 | 2 | 1 | 3 | 3 | Med | Motion |
| 10 | Check-in / Guide show-first composition | 4 | 3 | 2 | 1 | 1 | 3 | 4 | Med | Cards |
| 11 | Sidebar / dock / Flow-Focus | Later | 2 | 4 | 3 | 3 | 2 | 1 | Low | New authority |

Architectural impact stays low when work is Experience-only (no new IPC beyond
existing saved-context reads).

---

## B. Sequencing diagram

```
Phase 2 (authorise next)
  tokens/Tailwind → primitives → moment cards → Home dashboard → soft polish
        ↓
Phase 3
  Continue library → inspect drawer → search/cmdk
        ↓
Phase 4
  motion → check-in/guide craft → micro-interaction
```

---

## C. Pilot value note

Participant #1 daily use improves most when **Home remembers** and **Continue
looks inhabited**. That is Phase 2–3. Capability work does not substitute.

---

## D. Explicit non-goals

- Ambient thumbnails, AI briefings, system monitors, audio mixer
- Contract or Action restore changes
- Multi-workspace selector until product authority asks
