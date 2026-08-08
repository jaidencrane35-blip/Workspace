# B-DEF-001 — Prepare Coding Workspace Procedure Definition (C-PROC-002)

| Field | Value |
| --- | --- |
| **Capability ID** | **C-PROC-002** Prepare Coding Workspace |
| **Program** | B-DEF-001 definition / authority slice following P22.S7 |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Outcome** | **RESOLVED** — authoritative definition complete; **contract corrected** after pre-implementation audit; no runtime implementation |
| **Blocker ID** | **B-DEF-001 — RESOLVED** |
| **Lifecycle** | C-PROC-002 **BLOCKED → PLANNED** (contract corrected; still PLANNED) |
| **Max layer** | Capability definition + Repository Standards + Documentation |

---

## 1. Atlas definition used (complete text)

From `WORKSPACE_CAPABILITY_ATLAS.md` prior to the original blocker update:

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

No Step IDs, known targets, preconditions, success conditions, timeouts, or retry policy appear in the Atlas at that point.

---

## 2. Historical blocker

**B-DEF-001 — Incomplete Atlas procedure body** was the correct P22.S7
finding at commit `ac8c050a`.

The mission requires each procedure step to have Step ID, intended action, required capability, preconditions, observable success condition, timeout, retry policy, and failure outcome. The Atlas did not define those fields for C-PROC-002.

Mission rule: *If the Atlas definition is incomplete or contradictory, stop and report the ambiguity rather than silently expanding scope.*

The takeover found `v2-dev` clean and synchronized with `origin/v2-dev`.
There was no uncommitted partial definition to recover or discard.

---

## 3. Evidence

| Source | Finding |
| --- | --- |
| Atlas § C-PROC-002 (pre-definition) | Name/deps only; no step table |
| `situationGoals.ts` | Coding setup phrasing → Continue/Moments; **never invents app sets** |
| P21.A1 / P21.A2 | Prepare coding workspace = Moments handoff; inventing layouts is deliberate non-goal |
| Control surface eng | C-OBS-003/004, C-ACT-004/005, C-VER-002/003 exist — insufficient without procedure body |
| C-CMP-002 / `compoundOpen` | Already owns ordered multi-target open |
| Application `launch_alias` | Kernel launchability authority (Cursor/Notepad present; VS Code absent) |
| C-VER-002 `retry.rs` | Click/type-only; Launch not proven idempotent |

---

## 4. Attempted resolutions

1. Read Atlas C-PROC-002 + priority §9 + dependency graph.  
2. Searched situationGoals / P21 audits for an authoritative step list.  
3. Confirmed no constitutional Spec definition of C-PROC-002 steps.

No further coding attempts — inventing a Cursor/Terminal/layout procedure would violate Atlas authority and “never invent app sets.”

---

## 5. Initial definition (superseded shape)

The first resolution supplied an Atlas procedure body with PCW-001–PCW-006,
including an automatic Launch retry step and invented 2-second Find/Launch
deadlines. That body resolved B-DEF-001’s incompleteness, but a subsequent
adversarial pre-implementation audit found **BLOCKED — CONTRACT DEFECT**.

Audit defects corrected in the follow-on definition slice:

1. Dual Intent/Kernel target authority without whole-set preflight.
2. Unsafe automatic Launch retry via C-VER-002.
3. Duplicate Find behaviour outside C-ACT-001 / C-CMP-002.
4. Unsupported 2-second operation deadlines.
5. Final-active-window inferred from list order.
6. Product Proof proving only generic multi-app open.
7. Private ordered-target orchestration overlapping C-CMP-002.
8. Second retry orchestration risk.
9. Substring/first-match completion truth.
10. Automatic relaunch exceeding authorization.
11. Targetless Continue vs clarification unresolved.
12. Risk of inventing applications.

---

## 6. Corrected definition requirements

Atlas §C-PROC-002.1–.9 now requires:

1. Ordered Step IDs **PCW-001 through PCW-004** composing existing authorities.
2. Entry routing: Situation Goals Continue vs prepare-with-targets vs clarify.
3. Dual-authority resolved targets (Owner evidence + Intent + Kernel executability).
4. Whole-set launchability/openability preflight before any Effect.
5. Exact window identity preferring `hwnd` / observed identity evidence.
6. Timing only via existing C-VER-003; no invented operation deadlines.
7. No automatic Launch/Open retry; C-VER-002 out of scope.
8. Completion via C-CMP-001; final-active-window not mandatory.
9. Credible coding-workspace Product Proof using repository-supported targets.
10. Explicit non-goals forbidding private planner/Find/retry frameworks.

The Atlas is the sole authoritative procedure body; this report records the
blocker's discovery, initial resolution, and contract correction without
creating duplicate procedure truth.

---

## 7. Resolution

C-PROC-002 remains **PLANNED** at approximately **31% readiness**:

- Architecture 100% — corrected deterministic contract defined.
- Dependencies 100% — required capability primitives are engineering-present.
- Implementation 20% — existing Situation Goals → Continue handoff only.
- Runtime verification, Product Proof, Trusted, and Production remain 0%.

B-DEF-001 remains resolved. No application, control, layout, file, project,
terminal command, or saved Moment was selected as a default. No runtime
behaviour changed in this definition correction.

Interim product behaviour remains: Situation Goals → Continue (Owner-approved Moments restore) until Owner-authorized implementation of the corrected contract.

---

## 8. Artifact note

`scripts/verify-prepare-coding-workspace-definition.mjs` machine-checks the
corrected Atlas step schema, target-authority rules, composition dependencies,
timing/retry safety tokens, failure coverage, Product Proof, and B-DEF-001
lifecycle. It is wired into `pnpm test`. It verifies definition completeness
only and does not claim runtime behaviour.

---

## 9. Pre-flight and compliance

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
| Product Proof | Defined/corrected, not run, not accepted |

---

## 10. Stop condition

The next possible slice is C-PROC-002 runtime implementation, but it is not
authorized by this definition program. Release Hold remains active. A future
agent must receive a new explicit Owner authorization and must implement only
the corrected Atlas contract. Do not select another capability.
