# Execution Program: P0 Conversational Shell

| Field | Value |
| --- | --- |
| **Program** | Conversational Shell (Modes 1 & 2) + Intent Bridge |
| **Date** | 2026-08-07 |
| **Product authority** | `docs/00-Constitution/PRODUCT_CONSTITUTION.md` |
| **Architecture** | Unchanged — wraps Product Proof; CommandPipeline / permissions untouched |
| **Handoff** | Awaiting Project Owner review |

---

## Implemented behaviour

### Mode 1 — Floating operator
- Compact `W` launcher, movable (drag), position persisted in `localStorage`
- Click (without drag) opens Mode 2
- Minimal footprint; does not mount Product Proof chrome

### Mode 2 — Conversational shell (default launch)
- Calculator-sized panel; brand chrome only (Collapse / Workspace)
- Transcript + bottom-anchored input
- Empty state waits silently — no onboarding, prompts, capability lists, or fake dialogue
- Progressive text reveal for replies (deterministic, not model streaming)

### Mode 3 — Expand around conversation
- **Workspace** control or intents (`expand`, Save/Continue/…) open Mode 3
- Conversation remains visible (left column)
- Existing `WorkspaceShell` + Home / Save / Continue / Check-in / Guide appear beside chat
- Product Proof not rewritten — wrapped

### Intent bridge (`app/src/lib/intentBridge.ts`)
Deterministic routing only — **no AI reasoning**:

| Utterance examples | Result |
| --- | --- |
| Save this / remember where | → Save surface (Mode 3) |
| Continue / restore / yesterday | → Continue surface |
| History / saved work / moments | → Home Moments |
| Check-in / pilot | → Check-in |
| Guide / help / limits | → Guide |
| Expand / open workspace | → Mode 3 |
| Collapse / minimize | → Mode 1 |
| Repository health | → Health panel |
| Unknown / Open Cursor | Honest refusal + closest available |

### Repository health surface
- Secondary panel in Mode 3
- Loads `/project-health.json` (synced from `docs/project-health.json` via `pnpm sync:project-health`)
- Shows program, verification, validation, backlog, milestone, handoff

### Permission model
- No new desktop mutation paths
- Save/Continue still use existing consent/plan approval flows when those surfaces run

---

## Known limitations / placeholders

| Item | Status |
| --- | --- |
| NL understanding | Keyword/phrase bridge only |
| Named Moment restore (“restore Northwind”) | Not parsed — opens Continue list |
| App launch by name | Explicitly refused |
| Settings surface | Refused; points to Guide / expand |
| Mode 1 “dockable” to screen edges | Free-position only (not edge-snap magnets) |
| Screenshots in this report | Not captured — live window left running for owner |
| Streaming | Local progressive reveal, not LLM tokens |
| Five-tab dock | Still present inside Mode 3 Product Proof wrap (by design) |

---

## Current interaction model

1. Launch → Mode 2 conversation (front door)  
2. User types intent → bridge replies truthfully → may expand Mode 3 into existing PP  
3. User may Collapse to Mode 1 or Compact back to Mode 2  
4. Health is secondary (`repository health`)

---

## Validation

| Check | Result |
| --- | --- |
| `pnpm typecheck` | Pass |
| `pnpm test` | Pass (345 tests + verifiers) |
| `pnpm build` | Pass |
| `cargo check -p workspace-app` | Pass |
| `tauri dev` launch | Pass — kernel ready |

---

## Repository health summary

See `docs/project-health.json` / in-app **repository health**.

- Constitution 2.0  
- Engineering mode: constitutional-execution  
- Handoff: `AWAITING_PROJECT_OWNER_CONVERSATIONAL_SHELL_REVIEW`

---

## Files touched (primary)

| Path | Role |
| --- | --- |
| `app/src/App.tsx` | Wraps Product Proof in `OperatorRoot` |
| `app/src/components/operator/OperatorRoot.tsx` | Modes 1–3 shell |
| `app/src/components/operator/RepositoryHealthPanel.tsx` | Health UI |
| `app/src/lib/intentBridge.ts` | Intent routing |
| `app/src/App.css` | Operator styles |
| `app/public/project-health.json` | Served health snapshot |
| `tests/intent-bridge.test.ts` | Bridge tests |
| `scripts/sync-project-health.mjs` | Dual-write public JSON |

---

## Next recommended execution program

**Deepen intent bridge** (named Moments, richer restore phrasing) **or** Mode 3 presentation polish — chosen only after owner review of this shell.

Do **not** auto-start Docs Convergence or AI ModelProvider until the conversational front door is accepted.

---

## STOP

Application left running for Project Owner review. No further implementation in this program.
