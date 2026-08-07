# P16.33 — Cognitive Capability, Desktop Semantics & Production Benchmark Validation

| Field | Value |
| --- | --- |
| **Program** | P16.33 (not P17) |
| **Benchmark** | Premium desktop operator competence — not Voice ASR |
| **Status** | Engineering complete for program — Owner live Product Proof pending |
| **Branch** | `v2-dev` |
| **Voice ownership** | Not reopened — no objective Voice defect in this program |

---

## 1. Repository reassessment

- P10–P15 permanently closed; P16 Product Proof **OPEN**; P17 blocked.
- WinRT WRAP frozen; Voice engineering remains complete unless Owner proves Voice ownership.
- P16.32 trust/exe invent refusal held; Owner evidence showed cognitive / discovery / recovery gaps below premium desktop assistants.

---

## 2. Desktop operator audit

| Challenge | Result | Ownership |
| --- | --- | --- |
| Desktop awareness (monitors list) | Present via window ops | Desktop Operator |
| Application awareness (known catalog + launch) | Present; no invent | Intent + Kernel |
| Window awareness (enumerate / focus / locate) | Present; cognitive locate phrases added | Intent |
| Browser awareness (tabs / profiles) | Titles only — no deep tab/profile APIs | Outside / Track A |
| File / folder awareness | Shell folders + truthful date-filter limit | Intent |
| Capability discovery | Registry self-describing can/cannot/why | Registry |
| Conversation continuity | Follow-ups still shallow | Conversation / Track A |
| Recovery | Registry-guided unknown recovery | Intent + Registry |

---

## 3. Cognitive language audit

Reasoning added in `reasonCognitiveDesktop` + grammar beside-before-goal — **not** alias/keyword growth:

| Owner phrase | Behaviour |
| --- | --- |
| I've got ChatGPT somewhere | Locate → winFocus |
| Where is Cursor? / Where did my browser go? | Locate → winFocus |
| Bring back my browser / I was just using Chrome | Locate → winFocus |
| Take me to YouTube | Goal → browserOpen |
| Find my Downloads | Shell folder open |
| Open the pictures from yesterday | Pictures + honest no date filter |
| Show me the folder with screenshots | Pictures + honest note |
| I want ChatGPT next to Cursor | Beside composition |
| Put Chrome on the other monitor | winMoveMonitor index 2 + honest caveat |
| What windows are open? | winEnumerate |
| I'm trying to find Explorer | Locate → winFocus |
| Show me everything you can control | Registry discovery (can/cannot/why) |

---

## 4. Capability Registry audit

`generateCapabilityDiscovery` now emits from live `CAPABILITY_GRAPH` only:

- what it can do (summary + example + requirement)
- what it won’t overclaim (limitations)
- why (graph authority — no invented success)
- `generateRecoveryGuidance` for unknown entities

If a capability cannot be described from the Registry, it is not claimed.

---

## 5. Semantic reasoning audit

Pipeline unchanged and verified:

Speech → Normalization → Intent Grammar → Semantic / Cognitive → Capability Resolution → Execution → Evidence

No transcript executes directly. Evidence via `resolveIntentWithEvidence`.

---

## 6. Desktop awareness audit (truthful)

| Fact | Known? | Notes |
| --- | --- | --- |
| Running apps / windows | Yes (enumerate / focus) | Title match |
| Browser tabs / profiles | **No** | Not faked — Outside P16 |
| Desktop folders | Common shell folders | Not arbitrary filesystem search |
| Monitors | List + move-to-index | “Other” = monitor 2 heuristic |
| Foreground / focused app | Active window query | Present |
| Active browser | Heuristic (Chrome default for “my browser”) | Not profile-aware |

---

## 7. Behavioural benchmark (behaviour only — never copy code)

| System | Stronger behaviours felt | Gap class |
| --- | --- | --- |
| **Kiro** | IDE-native agent loops, multi-step coding | Outside / P17 Automation |
| **Raycast** | Instant launcher + extensions | Outside (plugin ecosystem) |
| **PowerToys Run** | Fuzzy app/file launch latency | Track A polish / Outside |
| **Windows Search** | Index-wide file/app find | Outside OS index |
| **VS Code** | Command palette discoverability | Track A UX |
| **ChatGPT Desktop** | Long conversational memory | Conversation / Outside LLM |
| **Claude Desktop** | Tool-use narration richness | Conversation |
| **Copilot** | OS-integrated suggestions | Outside Microsoft surface |
| **Windows Shell** | Explorer/date filters, Jump Lists | Outside / Track A |

Workspace’s differentiator remains **Conversational Desktop Operator** — not launcher, not chat LLM, not IDE agent.

---

## 8. Recovery quality audit

Unknown app / folder / entity / intent → Conversation unknown + registry nearby examples + limitations. Never exposes Provider / Registry / Kernel / WinRT. Never invents programs.

---

## 9. Product quality audit (first-use)

| Question | Engineering judgment |
| --- | --- |
| Understand what it can do? | Improved via registry can/cannot/why |
| Natural discovery? | “everything you can control” + scoped asks |
| Trust? | Stronger (honest limits; no invent) — Owner decides |
| Why failure? | Recovery guidance present |
| What next? | Nearby examples from graph |

---

## 10. Remaining missing production-standard behaviours (top 10)

1. Deep browser tab / profile control — **Outside / Track A**
2. Fuzzy Start-menu / install search — **Outside / P17+**
3. Arbitrary filesystem search (“find that PDF”) — **Outside / P17 File**
4. Date-filtered folder contents — **Outside shell; Track A if owned**
5. True “other monitor” relative to current window — **Desktop Operator / Track A**
6. Multi-step “do A then B then C” automation — **P17**
7. Long conversation memory / pronouns — **Conversation / Track A**
8. Plugin/extension marketplace — **Outside**
9. Rich first-run guided tour without competing chrome — **Track A / Product Gravity**
10. LLM-style paraphrasing — **Rejected for Intent** (deterministic Intent Layer)

---

## 11. Validation summary

- `pnpm typecheck` / `pnpm build` / `pnpm test`
- `cargo check`
- Semantic / capability registry / cognitive desktop / voice / conversation / product-proof verifiers

---

## 12–15. Commit / branch / health / Product Proof

Filled at handoff after commit: commit hash, `v2-dev` pushed, `handoffStatus` remains `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING`. P16 not permanently closed. P17 not started.

---

## 16. Newly falsified assumptions

| Assumption | Verdict |
| --- | --- |
| Discovery listing alone is enough | **Falsified** — need can/cannot/why from graph |
| Beside requires exact “open X beside Y” | **Falsified** — “I want X next to Y” must work |
| Locate requires “locate/find” verbs only | **Falsified** — “I’ve got…somewhere”, “was just using”, “trying to find” |

---

## 17. Remaining assumptions not objectively proven

- Monitor “other” = index 2 is acceptable to Owner
- Pictures-as-screenshots-folder is acceptable honesty
- Owner will accept deterministic Intent (no LLM paraphrase) at premium bar
- Live Voice path still Owner-green under fast speech (F9 DOCUMENT)

---

## 18. Explicit answers

| Question | Answer |
| --- | --- |
| Understand intent rather than wording? | **Improved** — cognitive structural reasoning; still deterministic, not ML |
| Explain every capability from Registry? | **Yes** for declared graph (can/cannot/why/requirements/examples) |
| Guide naturally after failures? | **Yes** via registry recovery — Owner validates live |
| Registry architectural source of truth for discovery? | **Yes** |
| Behaviourally competitive with Kiro/Raycast/…? | **Partial** — strong on conversational desktop ops; weaker on launcher latency, deep tabs, automation, plugins |
| Additional engineering justified now? | Only if Owner proves a reproducible in-scope defect |

### Ownership of remaining gaps

| Gap | Own |
| --- | --- |
| Cognitive phrasing residual misses | **P16 Intent** (if Owner proves) |
| Discovery tone / length | **P16 Registry / Conversation** |
| Deep tabs / profiles | Outside / Track A |
| Automation chains | **P17** |
| Voice ASR quality | Voice (only if Owner proves) — **not** reopened here |

---

## Permanent principle

Product maturity is measured by user success, not engineering certainty.
The benchmark is a world-class desktop operating companion — not “working software.”
