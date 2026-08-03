# V2 Agent Handoff — Shutdown Snapshot

Status: Active operational handoff (not an architecture authority tip)  
Date: 2026-08-03  
Branch: `v2-dev`  
Tip commit: `84f9f02` — `feat(v2): implement architectural maintainability reporting`  
Conversation history is never authoritative (ADR-0008 / Engineering Session Protocol).

---

## 1. Read this first (new agent)

1. `architecture/04_Cursor_Protocol.md`
2. `architecture/17_Engineering_Session_Protocol.md`
3. This file
4. Authority tip: `architecture/65_Architectural_Maintainability.md`
5. Prior tip: `architecture/64_Continuous_Engineering_Certification.md`
6. Version 2 architecture docs `40`–`65` as needed
7. `architecture/01_Current_State.md` + latest `02_Engineering_Ledger.md` entry

Do not invent architecture from chat. Repository records win.

---

## 2. Authority tip

| Item | Value |
|---|---|
| Branch | `v2-dev` |
| Tip commit | `84f9f02` |
| Tip authority doc | `65_Architectural_Maintainability.md` |
| Authority chain | `40 → … → 64 → 65` |
| Code tip invariant | `ARCHITECTURE_AUTHORITY_DOCS` + `AUTHORITY_CHAIN` in `app/src/dev/` |

---

## 3. What Version 2 is

Windows-targeted Tauri 2 desktop app. Version 2 Experience / governance / engineering confidence lives primarily in:

- `app/src/experience/*` — presentation / adaptation pipeline
- `app/src/dev/*` — DEV tooling, evidence, governance, integrity, certification, maintainability
- `architecture/40_*.md` … `architecture/65_*.md` — V2 architecture evidence docs

Production Product Proof (Save / Resume / Pilot) remains separate and must stay behaviour-stable unless a sprint explicitly authorises change.

---

## 4. Completed V2 sprint track (recent)

| Sprint | Commit | Focus | Doc |
|---|---|---|---|
| 65 | `27d05c1` | Governance consolidation | 56 |
| 66 | `2432f37` | Real-world adaptation validation | 57 |
| 67 | `c9b2f5f` | Workspace Memory Evolution | 58 |
| 68 | `52d493a` | Workspace Presence | 59 |
| 69 | `ebced20` | Workspace Anticipation | 60 |
| 70 | `b032be5` | Workspace Calibration (internal) | 61 |
| 71 | `e760bbb` | Presentation stability (internal) | 62 |
| 72 | `5864c2c` | Architectural simplification | 63 |
| 73 | `3368705` | Continuous engineering certification | 64 |
| 74 | `84f9f02` | Architectural maintainability reporting | 65 |

Sprints 72–74 explicitly shifted from expanding architecture to **engineering confidence / sustainability**.

---

## 5. Stable presentation pipeline

```
Runtime → Certified Pack → Memory Evolution → Presence → Anticipation → Presentation
```

- Resolver stage count is locked at **5**
- Calibration + presentation stability are **internal** to anticipation resolution — **not** new resolver stages
- Shared math: `app/src/experience/experienceMath.ts`

---

## 6. Hard constraints for future work

Unless a future sprint brief explicitly overrides:

- No new architectural layers
- No new governance systems
- No new certification systems
- No new resolver stages
- No new persistence models / storage keys
- No Runtime Core / Experience / presentation / navigation / persistence behaviour changes “for convenience”
- No AI autonomy
- No duplicated validators or persistence
- Do not alter public experience barrel APIs unless authorised
- Forbidden event/content JSON keys include `note`, `summary`, `message`, `label`, `text`, `name`, `title`, `content`, `handoff`, etc. (`FORBIDDEN_EVENT_KEYS`) — use `detail` when needed
- DEV overlay features: **derived only**, **lazy-load only**

Future work should improve engineering quality / maintainability / confidence — not expand architecture.

---

## 7. Key implementation map

| Concern | Module |
|---|---|
| Engineering invariants + certification runner | `app/src/dev/engineeringCertification.ts` |
| Maintainability + dependency health + trend | `app/src/dev/architecturalMaintainability.ts` |
| Architecture graph / authority chain | `app/src/dev/architecturalIntegrity.ts` |
| Authority doc list | `app/src/dev/engineeringGovernance.ts` |
| Complexity report (Sprint 72) | `app/src/experience/architecturalComplexity.ts` |
| Governance primitives | `app/src/dev/governancePrimitives.ts` |
| DEV overlay | `app/src/dev/ExperienceEvidenceDashboard.tsx` |
| Public experience barrel | `app/src/experience/index.ts` |

Certification history is **in-memory only** (no new storage key).  
Maintainability inventory is a **frozen derived inventory** (deterministic; no runtime filesystem dependency in the browser overlay).

---

## 8. Validation baseline at tip

At commit `84f9f02`:

- `pnpm typecheck` — green
- `pnpm test` — **342** tests green (incl. integrity, certification, maintainability, privacy audits)

Known pre-existing issues (from `AGENTS.md`; unrelated to V2 Experience track unless touched):

- `workspace-kernel` may fail compile on `resilience_validation.rs` / DecisionCandidate types on some tips of history — verify with `cargo check -p workspace-kernel` before assuming fixed
- Some `workspace-database` tests are platform-sensitive on Linux

---

## 9. Working tree note (do not mix into V2 commits)

Unrelated dirty / untracked research and architecture files may exist locally (catalogue, capability docs, evidence JSON, `architecture/research/*`, etc.).

**Do not** stage those into V2 engineering commits unless a sprint explicitly includes them.

---

## 10. Suggested next work (not authorised until briefed)

No Sprint 75 is authorised by this handoff.

When continuing V2 engineering-quality work, prefer:

- Prove long-term maintainability / confidence
- Reuse `EngineeringInvariant`, certification runner, maintainability reports
- Avoid new layers, stages, governance, or persistence

Product Proof programme next objective (separate track): LEDGER-0013 pilot recruitment/execution — see `01_Current_State.md`.

---

## 11. Commit messages style (recent)

```
feat(v2): implement continuous engineering certification
feat(v2): implement architectural maintainability reporting
```

Use exact sprint-requested commit titles when provided.

---

## 12. Shutdown checklist

- [x] Tip commit on `v2-dev`: `84f9f02`
- [x] Authority tip doc: `65_Architectural_Maintainability.md`
- [x] Handoff written: this file
- [x] Current State + Ledger updated for V2 tip
- [ ] Next human/agent: open a **new** session; start from repository authority above
