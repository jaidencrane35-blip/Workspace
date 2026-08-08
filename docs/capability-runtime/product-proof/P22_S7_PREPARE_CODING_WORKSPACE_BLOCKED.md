# B-DEF-001 — Prepare Coding Workspace Procedure Definition (C-PROC-002)

| Field | Value |
| --- | --- |
| **Capability ID** | **C-PROC-002** Prepare Coding Workspace |
| **Program** | B-DEF-001 definition / authority slice following P22.S7 |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Outcome** | **RESOLVED** — authoritative definition complete; no runtime implementation |
| **Blocker ID** | **B-DEF-001 — RESOLVED** |
| **Lifecycle** | C-PROC-002 **BLOCKED → PLANNED** |
| **Max layer** | Capability definition + Repository Standards + Documentation |

---

## 1. Atlas definition used (complete text)

From `WORKSPACE_CAPABILITY_ATLAS.md` prior to this blocker update:

| Field | Value |
| --- | --- |
| Name | Prepare Coding Workspace |
| Layer | L8 Procedures |
| Status | PLANNED (partial via situation goals) |
| Lifecycle | Partial eng via situationGoals |
| Dependencies | C-ACT-001; C-OBS-001; preferably control surface |
| Verification | situation goal tests |
| Product Proof | Required for full procedure |
| Engineering Notes | Expand after C-ACT-004/005 |

No Step IDs, known targets, preconditions, success conditions, timeouts, or retry policy appear in the Atlas.

---

## 2. Historical blocker

**B-DEF-001 — Incomplete Atlas procedure body** was the correct P22.S7
finding at commit `ac8c050a`.

The mission requires each procedure step to have Step ID, intended action, required capability, preconditions, observable success condition, timeout, retry policy, and failure outcome. The Atlas does not define those fields for C-PROC-002.

Mission rule: *If the Atlas definition is incomplete or contradictory, stop and report the ambiguity rather than silently expanding scope.*

The takeover found `v2-dev` clean and synchronized with `origin/v2-dev`.
There was no uncommitted partial definition to recover or discard.

---

## 3. Evidence

| Source | Finding |
| --- | --- |
| Atlas § C-PROC-002 | Name/deps only; no step table |
| `situationGoals.ts` | Coding setup phrasing → Continue/Moments; **never invents app sets** |
| P21.A1 / P21.A2 | Prepare coding workspace = Moments handoff; inventing layouts is deliberate non-goal |
| Control surface eng | C-OBS-003/004, C-ACT-004/005, C-VER-002/003 exist — insufficient without procedure body |

---

## 4. Attempted resolutions

1. Read Atlas C-PROC-002 + priority §9 + dependency graph.  
2. Searched situationGoals / P21 audits for an authoritative step list.  
3. Confirmed no constitutional Spec definition of C-PROC-002 steps.

No further coding attempts — inventing a Cursor/Terminal/layout procedure would violate Atlas authority and “never invent app sets.”

---

## 5. Definition requirements

The resolution supplied the required Atlas procedure definition:

1. Ordered Step IDs PCW-001 through PCW-006, each with action, capability,
   target, preconditions, observable success, timeout, retry, and failure.
2. Explicitly named / existing known / ambiguous / missing target rules.
3. Final observed target-window and active-window completion criteria.
4. Bounded C-VER-002/C-VER-003 policy and authorization classification.
5. Failure, partial-completion, future Product Proof, and explicit non-goals.

The Atlas is the sole authoritative procedure body; this report records the
blocker's discovery and resolution without creating duplicate procedure truth.

---

## 6. Resolution

C-PROC-002 is now **PLANNED** at approximately **31% readiness**:

- Architecture 100% — deterministic contract defined.
- Dependencies 100% — required capability primitives are engineering-present.
- Implementation 20% — existing Situation Goals → Continue handoff only.
- Runtime verification, Product Proof, Trusted, and Production remain 0%.

B-DEF-001 is removed from the active blocker register and retained in the
resolved-definition register. No application, control, layout, file, project,
terminal command, or saved Moment was selected as a default.

Interim product behaviour remains: Situation Goals → Continue (Owner-approved Moments restore).

---

## 7. Artifact note

`scripts/verify-prepare-coding-workspace-definition.mjs` machine-checks the
Atlas step schema, named-target rules, authorization/retry/failure coverage,
future Product Proof, and B-DEF-001 lifecycle. It is wired into `pnpm test`.
It verifies definition completeness only and does not claim runtime behaviour.

---

## 8. Pre-flight and compliance

| Check | Result |
| --- | --- |
| Program type | **Documentation** primary; Capability Definition / Repository Standards secondary |
| Lowest layer | Documentation plus one repository verifier; no runtime/provider change |
| Maximum layer | Capability |
| Constitutional review trigger | None |
| Information owners | Existing Understanding, Orchestration, Authority, Execution, Evidence, Presentation only |
| Transformation chain | Preserved; future Effects remain Kernel/Permission-gated |
| Hidden authority | None; current Owner target request authorizes only exact targets |
| Presentation purity / composition | Preserved; Kernel Operator remains sole composition authority |
| Product Proof | Defined, not run, not accepted |

---

## 9. Stop condition

The next possible slice is C-PROC-002 runtime implementation, but it is not
authorized by this definition program. Release Hold remains active. A future
agent must receive a new explicit Owner authorization and must implement only
the fixed Atlas contract. Do not select another capability.
