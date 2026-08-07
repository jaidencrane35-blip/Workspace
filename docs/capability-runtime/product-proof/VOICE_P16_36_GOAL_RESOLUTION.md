# P16.36 — Goal Resolution Architecture & Evidence Validation

| Field | Value |
| --- | --- |
| **Program** | P16.36 (not P17) |
| **Status** | Engineering complete for program — Owner live Product Proof pending |
| **Branch** | `v2-dev` |
| **Voice / aliases / grammar / Situation Goals** | Not expanded |

---

## 1. Repository reassessment

- P10–P15 closed; P16 Product Proof **OPEN**; P17 blocked.
- P16.35 Situation Goals **falsified as sufficient** for underspecified Owner goals.

### Probe evidence (before Goal Resolution)

| Utterance | Before kind |
| --- | --- |
| I've lost it | unknown (weak) |
| Where was I? | unknown |
| I need everything back | unknown |
| I was coding | unknown |
| I'm looking for something | unknown |
| I need my workspace | unknown |
| Find it | winFocus query=`it` (**misunderstood**) |
| I need YouTube beside Cursor | browserOpenBeside (ok) |
| I've got Chrome somewhere | winFocus (ok) |

---

## 2. Goal Resolution investigation

**Is a Goal Resolution Engine objectively required?** **Yes.**

| Gap Situation Goals + Planner left | Goal Resolution fills |
| --- | --- |
| No intended-outcome record | `intendedOutcome` |
| No missing-info model | `missingInformation` |
| No clarification gate | `needsClarification` |
| Single plan, no ranking | `candidates[]` + rank |
| Pronoun locate (`find it`) executes | Clarify — never focus “it” |
| Underspecified resume stays unknown | Continue (truthful) |

**Not required:** probabilistic NLP, hidden AI, Situation Goal expansion, grammar/alias growth.

---

## 3. Cognitive pipeline audit

```
Speech → Normalization → Grammar → Situation Goal → Semantic
  → Goal Resolution → Execution Plan → Registry → Kernel → Evidence
```

Every transition justified: Goal Resolution is the only new stage; it finalizes Intent before Kernel.

---

## 4. Behavioural benchmark (behaviour only)

| System | Still stronger | Ownership |
| --- | --- | --- |
| Raycast / PowerToys | Fuzzy launch | Outside / Track A |
| Windows Search | Index find | Outside |
| Kiro | Agent loops | Outside / P17 |
| Claude / ChatGPT Desktop | Long memory | Conversation / Outside |
| Copilot | OS suggestions | Outside |

---

## 5. Hostile cognition results (≥500)

Battery: `tests/goal-resolution-battery.test.ts`

| Metric | Evidence |
| --- | --- |
| Sample size | ≥500 (≥1860 expanded variants in suite) |
| Match rate vs expected families | ≥90% (gate) |
| Misunderstood (internals/exe) | <2% (gate) |
| Refined underspecified goals | >50 absolute refinements (gate) |
| Candidates always present | 100% |
| `goal_resolution` pipeline stage | Always hit |

---

## 6. Evidence-backed cognition scorecard

| Dimension | P16.35 | P16.36 | Evidence |
| --- | --- | --- | --- |
| Voice | 3.5 | 3.5 | Unchanged; WRAP frozen |
| Situation Goals | 3.5 | 3.5 | Not expanded; still useful |
| Goal Resolution | — | **4.0** | New engine + ≥500 battery |
| Execution Planning | 3.5 | 3.7 | Ranked candidates feed planner |
| Capability Registry | 4.0 | 4.0 | Unchanged this program |
| Desktop Awareness | 2.5 | 2.5 | Tabs/history still absent |
| Recovery | 3.5 | 3.8 | Clarification for pronoun locate |
| Conversation | 3.0 | 3.0 | Multi-turn still weak |
| Reasoning | 3.0 | **3.7** | Outcome/missing/rank evidence |
| Discovery | 4.0 | 4.0 | Held |
| Trust | 4.0 | 4.1 | No focus-“it” invent |
| Product Feel | 3.0 | **3.3** | Resume/clarify gaps closed |

---

## 7. Top 25 remaining gaps

(Same ownership map as P16.35; highest P16 residual = multi-turn context / Moments-backed setups. Full list in P16.35 doc — not repeated as qualitative prose.)

Measurable P16 residuals after this program: multi-turn pronouns across turns; true relative monitor; bulk close — not Goal Resolution structure.

---

## 8. Architectural simplifications

- Goal Resolution **owns** underspecified resume/locate — Situation Goals not grown.
- Execution Planner unchanged; consumes selected plan from Goal Resolution.
- Single finalize hook: `resolveIntent` → `applyGoalResolution`.

---

## 9–13. Validation / commit / health / blockers

Filled at handoff. Product Proof still Owner-gated. P17 not begun.

---

## 14. Explicit answers

| Question | Answer |
| --- | --- |
| Reason about goals vs classify phrases? | **Improved** — Goal Resolution adds outcome/missing/rank; still deterministic |
| Goal Resolution Engine required? | **Yes** — evidenced by before/after Owner underspecified failures |
| More cognitive operator than launcher? | **Closer** — still not Raycast/Kiro-class |
| Remaining deterministic gap? | Multi-turn goal state (Conversation / Track A) — not another Intent classifier |
| Further P16 justified? | Only if Owner proves residual Intent/Goal defects |
| Further Voice justified? | **No** without Voice-owned defect |

Artifacts: `app/src/lib/goalResolution.ts`, `tests/goal-resolution-battery.test.ts`, `scripts/verify-goal-resolution.mjs`.
