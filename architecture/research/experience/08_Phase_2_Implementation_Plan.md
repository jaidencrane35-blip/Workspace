# Experience Phase 2 Implementation Plan

Research ID: EXP-001 · Stage 8
Status: Ready for authorisation — **not authorised by this research alone**
Date: 2026-08-02

Authority to implement must cite: `21_Experience_Roadmap.md` Phase 2 + this
plan + EXP-001 package. Programme objective LEDGER-0013 remains unchanged.

---

## 1. Objective

Make Home the centre of the product: dashboard-first, visually remembered,
never dead-empty — using only data Workspace already owns.

Exit criterion (from Roadmap): Participant can open Home and feel the product
remembers and invites continuation; mental model is dashboard-first.

---

## 2. In scope

1. Design token application (CSS variables ± Tailwind if adoption approved in
   the same implementation session).
2. Moment card variants: hero, standard, compact, placeholder.
3. Home dashboard layout:
   - Workspace header (name)
   - Hero centrepiece (latest saved context **or** structured empty hero)
   - Recent moments gallery with uneven spans
   - Primary actions: Save this moment / Continue work
   - Honest summary: e.g. “3 saved moments · last saved 14 minutes ago”
4. Empty-structure scaffold when zero contexts (ghost cards; no fake names).
5. Soft trust strip or compact restore-limits one-liner (full notice remains on
   Save/Continue flows).
6. Refine primitives used by Home (buttons/cards) for consistency.

## 3. Out of scope

- Ambient observation or fabricated activity
- AI copy
- Continue library redesign beyond shared cards (Phase 3)
- Command palette, dock, sidebar
- Motion library beyond CSS hover/focus (Phase 4)
- Contract/IPC/capability changes
- New Tauri commands (use `list_saved_contexts`, settings, existing create)

## 4. Data sources (existing)

| UI need | Source |
|---|---|
| Workspace name | `Workspace` from settings/active |
| Recent moments | `list_saved_contexts` sorted by `created_at` |
| Handoff | `handoff_note` |
| Soft density | `windows.length` (count only on card face) |
| Relative time | `created_at` + date-fns (if adopted) or `Intl.RelativeTimeFormat` |
| Empty | no rows → scaffold |

Do not display process IDs / monitor indices on Home cards.

## 5. Suggested file touch set (when authorised)

Presentation-only; illustrative:

- `app/src/components/HomeWorkspacePanel.tsx` — dashboard composition
- `app/src/components/MomentCard.tsx` — **new** shared card
- `app/src/components/EmptyStructure.tsx` — **new** scaffold
- `app/src/App.css` or Tailwind theme — tokens
- Optional: shadcn primitives under `app/src/components/ui/`
- Tests: update `pilot-chrome` / add Home structure assertions; keep
  restore-limits and consent tests green

## 6. Adoption gate (optional but recommended)

At start of Phase 2 implementation session, either:

**Path A — Foundation first (preferred if time allows)**  
Approve Tailwind + shadcn primitives + Lucide + tokens, then build Home.

**Path B — Structure first**  
Build dashboard composition with current CSS variables, adopt kit immediately
after if Home exit criterion needs craft.

Both paths stay Experience-only.

## 7. Validation checklist

- [ ] No new capabilities / contracts / ambient behaviour
- [ ] `pnpm test` / typecheck green
- [ ] Home shows hero + varied cards when contexts exist
- [ ] Home shows structured empty (not a lone sentence) when none
- [ ] Handoff visible before technical metadata
- [ ] Active workspace still persists (LEDGER-0033)
- [ ] Save/Continue Product Proof flows unchanged
- [ ] CSP verifier still passes
- [ ] Participant #1 can judge dashboard-first feel

## 8. Ledger expectation

Implementation session records a new LEDGER entry referencing EXP-001 and
Roadmap Phase 2; updates Current State phase status.

## 9. Stop conditions

Stop and report if Home “aliveness” seems to require ambient screenshots,
network services, or AI generation — redesign within owned data instead.
