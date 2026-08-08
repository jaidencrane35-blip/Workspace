# P22.S1 — Intelligence Kind Routing & Reasoning Provider

| Field | Value |
| --- | --- |
| **Program** | P22.S1 — Intelligence Kind Routing & Reasoning Provider |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Kind** | Bounded engineering slice |
| **Authority** | P22.A1 Intelligence Routing Audit (accepted) |
| **Max layer** | Intent Layer / Conversation reply composition |
| **Desktop execution** | Unchanged Kernel Operator; ChatGPT via existing Browser Provider |
| **Not** | Probabilistic Intent · new constitutional layers · embedded LLM · File Provider |

---

## 1. Objective

Classify the intelligence kind of an utterance **before** desktop soft-miss refusal, so arithmetic, time/date, capability meta, and world-knowledge asks are not framed as “outside the desktop.”

---

## 2. Files modified

| Path | Change |
| --- | --- |
| `app/src/lib/intelligenceRouting.ts` | Kind classifier + local reasoning + ChatGPT handoff + hybrid |
| `app/src/lib/intentBridge.ts` | Call `resolveIntelligenceRoute` before `resolveUnknownGuidance` |
| `app/src/lib/conversationGuidance.ts` | Retire anti-knowledge near-miss; soften generics |
| `app/src/lib/capabilityRegistry.ts` | Widen desktop-capability meta discovery |
| `tests/intelligence-routing.test.ts` | Regression |
| `scripts/verify-intelligence-routing.mjs` | Machine check |
| `package.json` | Wire verifier into `pnpm test` |

---

## 3. Intelligence routing before / after

### Before

```text
Utterance → desktop match → unknown → “outside the desktop” / “not general chat”
```

### After

```text
Utterance → desktop match (unchanged)
         → intelligence kind
              REASONING_LOCAL      → truthful local answer / Registry explain
              REASONING_PROVIDER   → Browser open ChatGPT (?q= handoff)
              HYBRID               → local answer + open known target
              CLARIFICATION        → ask once (e.g. WA timezone)
              CAPABILITY_LIMIT     → existing file/terminal walls
              OWNER_AUTHORIZATION  → existing Moments path
         → soft-miss only for true unknowns
```

---

## 4. Example interactions (every kind)

| Kind | Utterance | Result |
| --- | --- | --- |
| REASONING_LOCAL | “what is 2 + 2” | `4` |
| REASONING_LOCAL | “what is 15% of 80” | `12` |
| REASONING_LOCAL | “what time is it” | Local clock string |
| REASONING_LOCAL | “Can you manage windows?” | Capability Registry explain |
| REASONING_PROVIDER | “How long from Rockhampton to Gladstone?” | Open ChatGPT with `?q=` |
| HYBRID | “calculate 15% of 80 then open Calculator” | Reply `12` + open Calculator |
| CLARIFICATION | “What time is it in WA?” | Ask Western Australia vs Washington |
| DESKTOP | “Open Notepad” / “Take a screenshot” | Unchanged Kernel path |
| OWNER_AUTHORIZATION | Moments restore phrasing | Unchanged |
| CAPABILITY_LIMIT | “organize my downloads folder” | Files wall unchanged |

---

## 5. Validation

| Check | Result |
| --- | --- |
| Arithmetic not desktop refusal | Pass |
| Time/date local + WA clarify | Pass |
| Desktop capability → Registry | Pass |
| World knowledge → ChatGPT `?q=` | Pass |
| Desktop commands unchanged | Pass |
| Invent-exe + file walls | Pass |
| `pnpm typecheck` | Pass |
| Vitest | **482** passed |
| `pnpm test` (incl. verifiers) | Pass |
| `verify-intelligence-routing` | ok |

---

## 6. Stop

P22.S1 complete. Do not begin another engineering program until Owner directs.
