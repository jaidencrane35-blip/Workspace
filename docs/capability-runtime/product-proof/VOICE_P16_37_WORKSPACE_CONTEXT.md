# P16.37 — Workspace Context Model & Layer Validation

| Field | Value |
| --- | --- |
| **Program** | P16.37 (not P17) |
| **Status** | Engineering complete for program — Owner live Product Proof pending |
| **Branch** | `v2-dev` |
| **Voice / aliases / grammar / Situation Goals / Goal Resolution expansion** | Not expanded |

---

## 1. Repository reassessment

- P10–P15 closed; P16 Product Proof **OPEN**; P17 blocked.
- P16.36 Goal Resolution **falsified as sufficient** for multi-turn continuity (pronouns, again, session referents).

### Probe evidence (Goal Resolution alone)

| Utterance (after Open Chrome) | Without Context |
| --- | --- |
| Put it beside Cursor | Misroute / proposal / unbound |
| Do that again | unknown |
| Close that | `appClose` query=`"that"` |
| Open the other one | unknown / invent refusal |
| Bring everything back | Wrong focus target |
| I'm still working | unknown |

---

## 2. Workspace Context investigation

**Is a deterministic Workspace Context Model objectively required?** **Yes.**

| Gap Goal Resolution left | Context fills |
| --- | --- |
| No session referents | `lastAppQuery` / `lastUrl` / `lastBeside` / `lastWindowQuery` |
| No replay | `again` → last action |
| Pronoun close/move/find | Bind to referent or clarify |
| Continuity phrases | Continue surface (truthful) |
| Clarification memory | `unresolvedClarification` |

**Not required:** probabilistic NLP, hidden AI, LLM planning, grammar/alias growth, Goal Resolution expansion for pronouns.

**Exclusive Context responsibility:** session continuity + referent binding across turns.

**Goal Resolution stays simpler:** underspecified single-turn goals (unbound “find it”, vague resume) — not multi-turn “it/that/again”.

---

## 3. Cognitive architecture audit

```
Speech → Normalization → Grammar → Situation Goal → Semantic
  → Workspace Context → Goal Resolution → Execution Plan
  → Capability Registry → Kernel → Evidence
```

| Layer | One responsibility |
| --- | --- |
| Grammar | Structured desktop commands |
| Situation Goals | High-level situations (no pronouns) |
| Workspace Context | Session continuity / referents |
| Goal Resolution | Underspecified single-turn goals |
| Execution Planning | Declarative capability steps |
| Registry | Discovery + governance |
| Kernel | Composition / execution |
| Evidence | Truthful outcomes |

No layer merge justified; Context is the missing continuity owner.

---

## 4. Hostile layer validation

Suite: `tests/cognitive-layer-hostile.test.ts`

Grammar · Situation Goals · Goal Resolution · Workspace Context · Execution Planning · Registry governance · Recovery · Conversation · Desktop Awareness — each gated independently.

---

## 5. Context battery results

Battery: `tests/workspace-context-battery.test.ts`

| Metric | Evidence |
| --- | --- |
| Sample size | ≥500 multi-turn expansions |
| Match rate | ≥90% (gate) |
| Context stage hits | Measured in suite |
| Internals jargon | 0 |

---

## 6. Capability Registry governance

Every `CAPABILITY_GRAPH` node declares: ownership, arguments, permissions, dependencies, recovery, limitations, examples, benchmark, tests, architecturalJustification.

Validator: `validateCapabilityGraphGovernance()`.

---

## 7. Behaviour benchmark (behaviour only)

| System | Still stronger | Ownership |
| --- | --- | --- |
| Raycast / PowerToys | Fuzzy launch | Outside / Track A |
| Windows Search | Index find | Outside |
| Kiro | Agent loops | Outside / P17 |
| Claude / ChatGPT Desktop | Long semantic memory | Conversation / Outside |
| Copilot | OS suggestions | Outside |

---

## 8. Evidence-backed cognition scorecard

| Dimension | P16.36 | P16.37 | Evidence |
| --- | --- | --- | --- |
| Single-turn underspecify | Strong | Strong | Goal Resolution battery |
| Multi-turn pronouns | Weak | Strong | Context battery + falsification test |
| Again / continue session | Weak | Strong | Context continuity |
| Layer hostility | Partial | Per-layer suite | `cognitive-layer-hostile` |
| Registry governance | Partial | Full fields | `validateCapabilityGraphGovernance` |

---

## 9. Architectural simplifications

- Context finalizes continuity actions — Goal Resolution does not re-unbind concrete referents.
- Vitest `beforeEach` resets session context so batteries stay isolated.
- No new Situation Goal / grammar / alias tables.

---

## 10. Validation summary

Run: `pnpm typecheck` · `pnpm build` · `pnpm test` · `cargo check` · `verify-workspace-context` (+ prior cognitive verifiers).

---

## 11–14. Handoff

| Item | Status |
| --- | --- |
| Commit | (filled at push) |
| Branch | `v2-dev` |
| Health | `pnpm sync:project-health` |
| Product Proof | Owner live review required — **not** permanently closed |
| P17 | **Do not begin** |

### Explicit answers

1. **Context required?** Yes — measurable multi-turn failures without it.
2. **Exclusive Context?** Session continuity + referent binding.
3. **Goal Resolution simpler?** Yes — single-turn underspecify only.
4. **One responsibility per layer?** Yes (audit table).
5. **Hostile survival?** Gated in `cognitive-layer-hostile.test.ts`.
6. **Evidence?** Batteries ≥500 + layer suite + registry validator.
7. **Deterministic gaps?** Full desktop arrangement memory, long-term Moments auto-restore, fuzzy OS index — Track A / Outside / later programs.
8. **Never implement here?** LLM planning, provider-calls-provider, chat memory theatre, Kiro-style agent loops.

Artifacts: `app/src/lib/workspaceContext.ts`, `tests/workspace-context-battery.test.ts`, `tests/cognitive-layer-hostile.test.ts`, `scripts/verify-workspace-context.mjs`.
