# P16.35 — Product Cognition Gap Analysis

| Field | Value |
| --- | --- |
| **Program** | P16.35 (not P17) |
| **Nature** | Investigation + minimal deterministic Situation Goal layer |
| **Status** | Engineering complete for program — Owner live Product Proof pending |
| **Branch** | `v2-dev` |
| **Voice** | Not reopened |

**Principle:** Engineering confidence is hypothesis. Owner experience is the acceptance gate.

---

## 1. Repository reassessment

- P10–P15 closed; P16 Engineering Complete claims challenged; Product Proof **OPEN**; P17 blocked.
- WinRT WRAP frozen. Architecture frozen.
- P16.34 planning/registry held as infrastructure — Owner still felt cognitively weak.

### Falsified engineering assumptions (this program)

| Assumption | Probe result |
| --- | --- |
| Discovery phrasing is fully covered | **Falsified** — “everything you know how to control” → `proposal` |
| “Work setup” covers development/coding environments | **Falsified** |
| Screenshot folder paths cover “yesterday’s screenshots” | **Falsified** until Situation Goals |
| Vague resume (“working on something”) is handled | **Falsified** |
| Intent+Registry+Planner ⇒ premium feel | **Falsified** — situation goals were missing |

---

## 2. Product cognition audit (Owner examples)

| Utterance | Before | After (P16.35) |
| --- | --- | --- |
| I've got Chrome somewhere | supported | supported |
| I was working on something | unsupported | resume (Continue) |
| Take me back | resume | resume |
| I need YouTube beside ChatGPT | supported + plan | supported + plan |
| I'm looking for yesterday's screenshots | unsupported / wrong recovery | Pictures + honest date limit |
| I want my development setup | unsupported | resume |
| I'm done with this | unsupported | honest ask (no close-all) |
| I need everything ready | unsupported | resume |
| I want my coding environment | unsupported | resume |
| Show me everything you know how to control | **misunderstood → proposal** | discovery |
| I need to continue where I left off | resume | resume |

---

## 3. Desktop reasoning audit

| Dimension | Verdict |
| --- | --- |
| Goal understanding | **Partial** — situation goals now; entity goals strong; multi-turn weak |
| Context retention | **Weak** — no dialogue memory (Track A / Conversation) |
| Desktop awareness | **Partial** — windows/monitors yes; tabs/history no |
| Capability selection | **Improved** — graph + situation routing |
| Execution planning | **Present** for compositions (P16.34) |
| Recovery | **Improved** — registry failureRecovery |
| Guidance | **Partial** — honest; not tour-like |
| Truthfulness | **Strong** — refuses invent |

**Does Workspace reason or classify?** Both. Entity/command paths largely **classify**; Situation Goals + Execution Plans add bounded **deterministic reasoning**. It is not LLM reasoning and not multi-turn situational memory.

---

## 4. Capability Graph audit

Objectively required metadata added:

- purpose (via summary)
- arguments / requirements / limitations / examples / failureRecovery  
- relationships (`related`)  
- **similar** / **alternatives** / **discoverability**

Further metadata (embeddings, probabilistic ranking) is **not** required for deterministic Intent — Outside / Rejected.

---

## 5. Competitive behavioural analysis (behaviour only)

| System | Understands better | Gap ownership |
| --- | --- | --- |
| Raycast / PowerToys | Fuzzy app/file launch | Outside / Track A |
| Windows Search | Index-wide find | Outside |
| Kiro / Cursor / VS Code | Multi-step coding agents | Outside / P17 |
| Copilot | OS suggestion surface | Outside |
| ChatGPT / Claude Desktop | Long memory / paraphrase | Conversation / Outside |
| Windows Shell | Jump lists, date filters | Outside |

**What prevents “Kiro/Raycast/Copilot feel”?** Not Voice ASR. Missing: fuzzy launch latency, index search, long memory, agent loops, OS integration — mostly Outside/Track A/P17 — plus residual situation coverage and multi-turn context (P16 Intent / Conversation).

---

## 6. Product Cognition Scorecard

Scores: 1–5 (5 = shipping premium). Industry = best of compared set for that dimension.

| Dimension | Current | Industry | Evidence | Gap | Ownership |
| --- | --- | --- | --- | --- | --- |
| Voice | 3.5 | 4 | WinRT WRAP; F9 DOCUMENT | Fast speech | Voice only if proven |
| Intent | 3.5 | 4.5 | Situation Goals + grammar | Multi-turn goals | P16 / Track A |
| Registry | 4 | 4 | Full self-description | Tour UX | P16 / Track A |
| Execution Planning | 3.5 | 4 | Plan evidence on beside | Long chains | P16 / P17 |
| Desktop Awareness | 2.5 | 4 | Titles/monitors; no tabs | Tabs/history | Outside / Track A |
| Reasoning | 3 | 4.5 | Situation + plans | Still mostly classify | P16 |
| Recovery | 3.5 | 4 | Registry recovery | Tone polish | Conversation |
| Discovery | 4 | 4 | Graph + situation fix | First-run gravity | Track A |
| Guidance | 3 | 4 | Honest limits | Proactive coach | Conversation / Track A |
| Trust | 4 | 4 | No invent | Owner feel | Owner gate |
| Conversation | 3 | 4.5 | Continuity shallow | Memory | Conversation / Track A |
| Product Feel | 3 | 4.5 | Operator not launcher | Launcher+memory gap | Mixed |

---

## 7. Top 25 remaining behavioural gaps

1. Multi-turn context / pronouns — Conversation / Track A  
2. Deep browser tabs/profiles — Outside / Track A  
3. Recent-app history — Track A  
4. Fuzzy Start-menu search — Outside  
5. Arbitrary file find — P17  
6. Date-filtered folders — Outside / Track A  
7. Multi-app layout without Moments — P17 / Save-Restore  
8. Close-everything / session end automation — P16 honesty vs P17  
9. Relative other-monitor — Desktop Operator / Track A  
10. Launcher latency — Track A / Outside  
11. First-run guided discovery tour — Track A  
12. Long LLM paraphrase — Rejected for Intent  
13. Plugin ecosystems — Outside  
14. IDE agent loops — Outside / P17  
15. OS Copilot-style suggestions — Outside  
16. Jump lists — Outside  
17. Notification Action Center parity — Outside  
18. Bulk window ops — P16 limit / Track A  
19. “Working on something” without Moments — honest Continue only  
20. Screenshot date filter — Outside  
21. Voice fast speech — Voice DOCUMENT  
22. Conversation tone richness — Conversation  
23. Proactive “you might want…” — Product Gravity / Track A  
24. Cross-session memory — Conversation / Outside  
25. Chained “do A then B then C” scripts — P17  

---

## 8. Ownership classification

| Bucket | Gaps |
| --- | --- |
| **P16** | Residual situation/goal misses Owner proves; discovery misroutes; Intent honesty |
| **Track A** | Polish, tray, latency, first-run, relative monitor |
| **P17** | File provider, automation chains |
| **Outside** | OS search, plugins, LLM chat memory, Copilot surface |
| **Voice** | Only F9-class ASR if Owner proves — **not** reopened here |

---

## 9. Architectural recommendations

**Implemented (objectively missing):** Situation Goals layer (`situationGoals.ts`) — deterministic situation→action without grammar/alias growth.

**Not missing / do not add:** Probabilistic NLP, provider-side NL, second conversational agent, duplicate recovery catalogues.

**Prove unnecessary:** Another Intent Grammar expansion cycle — Owner failures were situation/discovery routing, not missing verb aliases.

---

## 10. Validation

`pnpm typecheck` · `pnpm build` · `pnpm test` · `cargo check` · cognition battery ≥250 · registry/semantic/execution verifiers.

---

## 11–14. Commit / branch / health / blockers

Filled at handoff. Product Proof blockers remain Owner live acceptance. P16 not permanently closed.

---

## 15. Explicit answers

| Question | Answer |
| --- | --- |
| Reason or classify? | **Both** — classification dominant; situation+plans = bounded reasoning |
| Why not Kiro/Raycast/Copilot feel? | Launcher/index/memory/agent gaps (mostly Outside) + residual situation/multi-turn |
| Missing deterministic layer? | **Situation Goals** — implemented; further layers not justified without Owner evidence |
| Highest-value remaining? | Multi-turn context; Moments-backed setups; tabs/history honesty; first-run discovery |
| Further P16 justified? | **Only** for reproducible Intent/Situation/Registry defects Owner proves |
| Further Voice justified? | **No** without Voice-owned defect |

Artifacts: `tests/product-cognition-battery.test.ts`, `app/src/lib/situationGoals.ts`, `app/src/lib/productCognition.ts`, `scripts/verify-product-cognition.mjs`.
