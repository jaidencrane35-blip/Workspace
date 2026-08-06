# Engineering Milestone Report
## Conversational Operator Foundation

| Field | Value |
| --- | --- |
| **Execution milestone** | Conversational Operator Foundation |
| **Date** | 2026-08-07 |
| **Branch** | `v2-dev` |
| **Commit hash** | _(filled after push)_ |
| **Repository version** | `0.1.0` (`package.json`) |
| **Repository health** | healthy |
| **Product authority** | `docs/00-Constitution/PRODUCT_CONSTITUTION.md` |
| **Architecture authority** | `architecture/ARCHITECTURAL_CONSTITUTION_V2.md` |
| **Commit review** | `docs/repository-commit-review.md` |

---

## Executive Summary

Conversation is the durable front door. Owner review of the P0 shell was accepted; this milestone refines the operator into desktop presence (not a nested mini-app), completes developer-only repository health, and ships a non-rewriting capability evolution proposal pipeline. Product Proof remains wrapped — Save / Continue / Check-in / Guide are reached through conversation and Expand.

---

## Repository Health

| Signal | Status |
| --- | --- |
| Overall | healthy |
| Typecheck | pass |
| Frontend build | pass |
| Vitest | 349 pass |
| Constitutional verifiers | pass (incl. capability-evolution) |
| `cargo check -p workspace-app` | pass |
| Machine state | `docs/project-health.json` |

---

## Product Progress

- Mode 1 lightweight floating operator retained
- Mode 2 calculator presence: solid message bubbles, drag frame on Workspace chrome, reduced nested borders / chrome
- Mode 3 expands Workspace around conversation; duplicate menubar brand hidden
- Engineering controls + Check-in evidence hidden unless developer mode (`Ctrl+Shift+D`, triple-click brand, or “developer mode” / health intents)
- Intent bridge still honest about unimplemented desktop claims

---

## Engineering Progress

- Capability Evolution Foundation: classify → propose → approve/reject → audit/undo/backlog
- `pnpm verify:capability-evolution` wired into `pnpm test`
- Repository health dashboard consumes full project-health fields (warnings, backlog, verifiers, debt, local proposals)
- Permanent cadence documented: review → refine → milestone → commit → push → next program

---

## Verification Added

| Artifact | Role |
| --- | --- |
| `scripts/verify-capability-evolution.mjs` | Schema + law presence |
| `docs/capability-evolution/registry.json` | Registry template |
| `tests/capability-evolution.test.ts` | Classifier + bridge routing |

---

## Validation

Ran: `pnpm typecheck`, `pnpm test`, `pnpm build`, `cargo check -p workspace-app`, domain/kernel tests as available, `pnpm sync:project-health` / `verify:project-health`.

---

## Technical Debt

Trend: down (new verifier + proposal pipeline). Remaining: G1 remainder (manual domain.ts), G3 docs convergence, G4 WorkspaceState naming, AgentToolGate / AuditIntegrity.

---

## Architecture Health

Architectural Constitution V2 unchanged. No new runtime ownership abstractions. Capability evolution explicitly forbids self-rewrite; execution stays constitutional.

---

## Product Health

Identity expression improved: conversation-primary, desktop-capability, trust-preserving permissions on Save/Continue paths. Operator feels closer to presence than a tool collection.

---

## Capability Evolution Status

Foundation complete. Proposals only. Approved items require a dedicated execution program. Registry seed at `docs/capability-evolution/`.

---

## Lessons Learned

- Wrapping Product Proof beats rewriting it for constitutional speed
- Developer surfaces must be gated or they reintroduce “tool collection” energy
- Every program should leave a verifier/generator when behaviour is machine-checkable

---

## Next Approved Execution Program

**Intent bridge deepening** — named Moments routing, clearer launch honesty, tighter Continue/Save phrases — single program after this milestone commit/push.
