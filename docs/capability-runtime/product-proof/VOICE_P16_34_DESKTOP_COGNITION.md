# P16.34 — Desktop Cognition Validation & Capability Intelligence

| Field | Value |
| --- | --- |
| **Program** | P16.34 (not P17) |
| **Benchmark** | Desktop Operating Companion — goal → plan → execute |
| **Status** | Engineering complete for program — Owner live Product Proof pending |
| **Branch** | `v2-dev` |
| **Voice ownership** | Not reopened — no objective Voice-owned defect |

**Permanent principle:** Desktop cognition outranks voice transcription. Understanding user intent is more valuable than recognizing user speech.

---

## 1. Repository reassessment

- P10–P15 closed; P16 Product Proof **OPEN**; P17 blocked.
- P16.33 cognitive phrasing held; Owner evidence still shows gaps in planning, chaining, context, recovery depth.
- Prior “engineering complete” claims remain untrusted until Owner acceptance.

---

## 2. Desktop cognition audit

| Stage | Falsification attempt | Result |
| --- | --- | --- |
| Speech | Fast speech (F9) | DOCUMENT WinRT — not Voice reopen |
| Normalization | Soften / discourse tails | Held |
| Semantic understanding | Hostile cognitive suite | Improved (P16.33–34) |
| Intent resolution | Goal / need / beside | Held + `i need` |
| Capability discovery | Registry self-description | Expanded (args/recovery/related) |
| Execution planning | Beside / focus / monitor | **New** `buildExecutionPlan` |
| Execution | Kernel composition | Unchanged authority |
| Recovery | Registry failureRecovery | Strengthened |
| Evidence | Pipeline stages | Includes `execution_plan` |

---

## 3. Desktop intelligence audit (truthful)

| Fact | Known? | Ownership if not |
| --- | --- | --- |
| Running applications / windows | Yes (enumerate / focus) | — |
| Browser windows | Title match | — |
| Browser tabs / profiles | **No** | Outside / Track A |
| Desktop folders | Common shell folders | — |
| Foreground / focused window | Yes | — |
| Monitors | List + move-to-index | — |
| Active browser | Heuristic only | Intent honesty |
| Recent application history | **No** (honest recovery) | Track A / Outside |
| Full “where I was” session | Via Continue Moments only | Product surface |

---

## 4. Intent pipeline audit

```
Speech → Normalization → Semantic → Intent → Capabilities → Execution Plan → Execution → Recovery → Evidence
```

No transcript executes directly. Plans are Intent-Layer evidence; Kernel still owns composition.

---

## 5. Capability Registry audit

Every `CAPABILITY_GRAPH` node now declares:

- summary / does / cannot  
- requirements / limitations  
- arguments  
- examples  
- failureRecovery  
- related capability ids  

`describeCapability`, discovery, and recovery read only from the graph.

---

## 6. Execution Planning audit

Example: “Open ChatGPT beside Cursor.”

1. Resolve site  
2. Locate Cursor  
3. Choose layout  
4. Open beside (Kernel)  
5. Complete  

Evidence: `resolveIntentWithEvidence(...).plan` + `execution_plan` stage.

---

## 7. Behavioural benchmark (behaviour only)

| System | Stronger feel | Gap class |
| --- | --- | --- |
| Kiro | Multi-step coding agents | Outside / P17 |
| Raycast | Instant launcher + extensions | Outside |
| PowerToys Run | Fuzzy app latency | Track A / Outside |
| Windows Search | Index-wide find | Outside |
| VS Code / Cursor | Command palette density | Track A |
| ChatGPT / Claude Desktop | Long memory / narration | Conversation / Outside |
| Copilot | OS suggestion surface | Outside |
| Windows Shell | Jump lists / date filters | Outside |

---

## 8. Hostile reasoning results

Covered in `tests/execution-planner.test.ts`: Chrome somewhere, ChatGPT just-using, YouTube next to Cursor, browser I had before, find screenshots, everything you can do, where I was, work setup, ChatGPT beside Cursor plan.

---

## 9. Architectural simplifications

- Single recovery path via `failureRecovery` + related examples (no parallel hard-coded recovery catalogues).  
- Discovery / describe / recover all generated from one graph.  
- Planning centralized in `executionPlanner.ts` — not phrase-hardcoded Conversation copy.

---

## 10. Top remaining production gaps

1. Deep tab/profile control — Outside / Track A  
2. True recent-app history — Track A  
3. Multi-app “work setup” automation without Moments — P17 / Save-Restore product  
4. Fuzzy install search — Outside  
5. Arbitrary filesystem search — P17 File  
6. Date-filtered folders — Outside / Track A  
7. Relative “other monitor” from current window — Desktop Operator / Track A  
8. Pronoun / long dialogue memory — Conversation / Track A  
9. Launcher-grade latency — Track A / Outside  
10. LLM paraphrase of intent — Rejected for Intent Layer  

---

## 11. Validation summary

`pnpm typecheck` · `pnpm build` · `pnpm test` · `cargo check` · semantic / registry / cognitive / execution-planner / voice / conversation verifiers.

---

## 12–15. Commit / branch / health / Product Proof

Filled at handoff: implementation commit hash; `v2-dev` pushed; `handoffStatus` = `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING`. P16 not permanently closed. P17 not started.

---

## 16. Newly falsified assumptions

| Assumption | Verdict |
| --- | --- |
| Intent action alone is enough evidence of “planning” | **Falsified** — need declarative plan steps |
| Registry summary/examples suffice for self-description | **Falsified** — need args + failureRecovery + related |
| “I need” is covered by “I want” | **Falsified** until grammar updated |
| “Where I was” can be invented from window titles | **Falsified** — Continue Moments only |

---

## 17. Explicit answers

| Question | Answer |
| --- | --- |
| Reason about goals vs phrase match? | **Improved** — cognitive + grammar + plans; still deterministic |
| Build execution plans before execution? | **Yes** at Intent Layer (`buildExecutionPlan`); Kernel executes compositions |
| Registry fully describe every capability? | **Yes** for declared graph fields |
| Understand desktop context vs isolated commands? | **Partial** — live windows/monitors yes; tabs/history no |
| Further Voice engineering justified? | **No** unless Owner proves Voice-owned defect |
| Further P16 engineering justified? | Only for reproducible Intent/Registry/Desktop Operator defects Owner proves |

### Ownership of remaining limitations

| Gap | Own |
| --- | --- |
| Residual cognitive misses | P16 Intent (if proven) |
| Plan depth / chaining beyond Kernel compose | P16 Intent / Kernel (if proven) |
| Tabs / history / index search | Outside / Track A |
| File provider / automation | P17 |
| Voice ASR | Voice only if proven — **not** reopened here |
