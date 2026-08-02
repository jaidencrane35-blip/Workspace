# Experience Phase 2 — Completion Report

Date: 2026-08-02  
LEDGER: LEDGER-0037  
Commits: `2a8a1f2`, `a574473`, (+ screenshots commit)

## Architecture Delta Report

| Area | Change |
|---|---|
| Capabilities | None |
| Contracts | None |
| Product Proof behaviour | Unchanged (Save review/confirm; Continue preview/approve; limits; consent) |
| Ownership | Experience presentation only |
| Trust / ambient / AI / telemetry | Unchanged / not introduced |
| Settings persistence | Still fixed (LEDGER-0033); verified `active_workspace_id` present |

## Experience Delta Report

| Before Phase 2 | After Phase 2 |
|---|---|
| Page-first Home | 12-column dashboard Home |
| Header + buttons + one card | Hero / empty-hero + rail + scaffold / moment rhythm |
| Uniform cards | Hero / standard / compact / placeholder weights |
| Absolute timestamps | Relative time (`Intl.RelativeTimeFormat`) |
| Sparse Continue empty | Structured empty + continuation copy |
| Form/doc/survey tone | Note / onboarding / feedback tone |
| Partial CSS vars | Unified token system (colour, space, radius, elevation, motion) |

## OSS adopted

| Package | Version | Licence | Use |
|---|---|---|---|
| `lucide-react` | 0.511.0 | ISC | Icon language |

Not adopted (deferred): Tailwind, shadcn/ui, Radix npm (Path B — owned CSS design system achieved Phase 2 exit without them). Eligible later per EXP-001.

## Components replaced / added

- Added: `MomentCard`, `EmptyStructure`, `lib/time.ts`
- Reworked: `HomeWorkspacePanel`, `ResumeContextPanel` (browse), `SaveContextPanel` (naming), `PilotHelpPanel`, `PilotMeasurementPanel` (chrome), `App.css` tokens + dashboard

## Screens changed

- Home, Continue, Save, Guide, Check-in

## Screenshots

- `architecture/research/experience/screenshots/phase2-home.png`
- `architecture/research/experience/screenshots/phase2-continue.png`

## Installer

- `target/release/bundle/nsis/Workspace_0.1.0_x64-setup.exe` (rebuilt + silent reinstall verified)

## Validation

- `pnpm typecheck` pass
- `pnpm test` 54/54 + CSP/boundary verifiers pass
- Active workspace persistence verified in SQLite

## Remaining gaps (Phase 3–4)

- Denser Continue gallery when many moments exist (day grouping, search)
- Inspect as true drawer (Vaul) vs disclosure
- Tailwind/shadcn adoption if desired for primitive consistency
- Motion/view transitions (Phase 4)
- Check-in still partially form-based under the dashboard chrome

## Self-review score

**8.2 / 10** for Experience Phase 2 exit criterion (dashboard-first Home with memory shape and continuation invitation).  
Not concept-board pixel parity; structural mental model shift achieved.

## Recommendation

**Experience Phase 2 is complete.** Next authorised Experience work: Phase 3 (rich Continue library) per `21_Experience_Roadmap.md`.
