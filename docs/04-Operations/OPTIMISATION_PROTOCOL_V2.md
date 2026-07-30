# Optimisation Protocol v2 — Plateau Detection

| Field | Value |
|-------|-------|
| **Purpose** | Binding rules for controlled optimisation sessions: exhaust safe engineering categories before declaring plateau; prevent premature stop and AI slop |
| **Owner** | Engineering |
| **Status** | Active — replaces premature “two consecutive weak cycles” plateau rule |
| **Related** | [AI Engineering Governance §11](../00-Governance/AI_ENGINEERING_GOVERNANCE.md), [OPTIMISATION_LOG.md](OPTIMISATION_LOG.md), [AGENTS.md](../../AGENTS.md), [WORKSPACE-VISUAL-DIRECTION.md](../01-Product/WORKSPACE-VISUAL-DIRECTION.md) |
| **Supersedes** | Ad-hoc “stop after two consecutive plateau detections” without category exhaustion |

---

## 1. Why Plateau v2 exists

The previous controlled-optimisation run correctly refused to invent product direction (e.g. bottom nav vs top chrome; OS geometry on Flow/Focus). That refusal was right.

It stopped too early on **engineering quality** because plateau was defined as “two consecutive cycles with nothing obvious,” which allowed unexamined categories (spacing, keyboard UX, CSS dead code, test gaps, error copy, DX, etc.) to remain unexplored.

**Plateau v2** separates:

| Concern | Rule |
|---------|------|
| Product / architecture / ownership | Still **immediate stop** — never invent direction |
| Safe engineering quality | Must **exhaust approved categories** before a global plateau |

---

## 2. Why it prevents premature stopping

A plateau may **not** be declared because:

- the last two cycles were weak
- the next product leap needs human approval
- the agent “feels done”
- reference alignment already looks high

A category may only be marked **PLATEAUED** after it has been **evaluated** and no **measurable** improvement remains inside approved ownership.

A **global plateau** (session end for plateau) requires:

1. **Every** approved optimisation category has been marked PLATEAUED, **and**
2. **Two consecutive full-category evaluation passes** produce no measurable improvement.

Until then, the optimiser continues: pick one non-plateaued category → analyse → improve if measurable → validate → log → continue.

---

## 3. Why it still prevents AI slop (anti-slop)

The optimiser must **never** create work simply because work is available.

Progress is measured by **measurable quality improvements**, **not** by:

- lines of code
- commit count
- files touched
- features added
- architecture expansion
- speculative “cleanup” that does not improve a named metric

If a category evaluation finds no measurable improvement, mark it **PLATEAUED** and move on — do **not** invent cosmetic churn.

---

## 4. Why it protects product direction

Mandatory **immediate stop** conditions (unchanged in spirit; always win over category exhaustion):

- Product philosophy would change
- Architecture ownership would change
- Governance / constitutional boundaries would be crossed
- New engines would be required
- Permission Gateway behaviour would change
- Desktop Arrangement behaviour beyond approved architecture would change
- Reference image interpretation would change
- A human product decision between equally valid directions is required

Those stops are **not** category plateaus. They are **boundary stops**. Log them as such and await human instruction.

---

## 5. Approved optimisation categories

Evaluate every category independently. One category per cycle.

| # | Category |
|---|----------|
| 1 | Visual hierarchy |
| 2 | Layout consistency |
| 3 | Spacing consistency |
| 4 | Typography consistency |
| 5 | Navigation clarity |
| 6 | Accessibility |
| 7 | Keyboard UX |
| 8 | Responsiveness |
| 9 | Animation polish |
| 10 | Component consistency |
| 11 | CSS simplification |
| 12 | Duplicate removal |
| 13 | Dead code removal |
| 14 | Documentation quality |
| 15 | Naming clarity |
| 16 | Maintainability |
| 17 | Human readability |
| 18 | Developer experience |
| 19 | Code organisation |
| 20 | Test quality |
| 21 | Performance |
| 22 | Memory efficiency |
| 23 | Build cleanliness |
| 24 | IPC cleanliness |
| 25 | Error messaging |
| 26 | Commercial readiness |

---

## 6. Cycle procedure

For every cycle:

1. Re-read governance + visual direction (and references when UI-affecting).
2. Select **ONE** approved category that is not yet PLATEAUED (or re-check during a full-pass).
3. Analyse current implementation against that category.
4. If measurable improvement exists **and** is safe: implement → validate → log → update metrics → continue.
5. If no meaningful measurable improvement: mark category **PLATEAUED** in the log (analysis-only cycle is allowed) → continue to another category.
6. Never expand product scope to “find” work.

### Validation (when code changes)

Prefer repository scripts: `pnpm typecheck`, `pnpm test`, architecture / IPC / UI-boundary verifies, `pnpm build` as applicable. Only continue if validation passes.

### Logging

Append to [`OPTIMISATION_LOG.md`](OPTIMISATION_LOG.md). Include category id/name, problem, reason, files, validation, metrics, why safe, and whether the category is now PLATEAUED.

---

## 7. Quality metrics

Update when movement is measurable (do not invent decimal noise):

| Metric |
|--------|
| Reference Alignment |
| Maintainability |
| Commercial Readiness |
| Human Readability |
| Accessibility |
| Performance |
| Developer Experience |
| Test Health |

Document movement only when evidence supports it (user-visible change, test coverage delta, measured perf, clearer ownership, etc.).

---

## 8. Global plateau and self-evaluation

Before ending a session for plateau, ask:

> Have all approved optimisation categories genuinely plateaued?

| Answer | Action |
|--------|--------|
| **NO** | Continue — pick an unevaluated or non-plateaued category |
| **YES**, and two consecutive full-category passes found nothing | Stop — produce one concise engineering report |

Boundary stops (§4) end the session immediately without requiring category exhaustion.

---

## 9. Relationship to prior runs

Prior “two consecutive plateau detections” stops remain historically valid as **conservative** stops under the old rule. Under Protocol v2, those sessions may be **resumed** to exhaust remaining categories without inventing product direction.

---

## 10. Enforcement

Agents running controlled optimisation must follow this document and [AI Engineering Governance §11](../00-Governance/AI_ENGINEERING_GOVERNANCE.md). Conflict: Constitution wins on product authority; this protocol wins on optimisation process until updated.
