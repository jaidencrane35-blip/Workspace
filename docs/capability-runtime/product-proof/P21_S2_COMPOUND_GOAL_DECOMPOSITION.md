# P21.S2 — Compound Goal Decomposition

| Field | Value |
| --- | --- |
| **Program** | P21.S2 — Compound Goal Decomposition |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Kind** | Bounded engineering slice |
| **Authority** | P21.A2 Task Decomposition & Initiative Audit |
| **Depends on** | P21.S1 Capability Completion Contract |
| **Max layer** | Intent Layer + Kernel Operator compose |
| **Not** | Autonomy · new planner · Moments invent · File Provider |

---

## 1. Objective

When an Owner utterance contains multiple **independently resolvable** open targets, execute them in deterministic order and report the aggregate outcome under the Capability Completion Contract — without asking for another turn that adds no information.

---

## 2. Files modified

| Path | Change |
| --- | --- |
| `app/src/lib/compoundOpen.ts` | Resolve/encode resolvable dual+ opens |
| `app/src/lib/intentBridge.ts` | `compoundOpen` IntentAction; use resolver instead of blind `\band\` reject |
| `app/src/lib/operator/intentMap.ts` | Map → `application` / `open_compound` |
| `app/src/lib/executionPlanner.ts` | Declarative multi-step evidence plan |
| `app/src/lib/workspaceContext.ts` | Commit compound targets into session referents |
| `packages/kernel/src/operator/plan.rs` | Plan sequential Find/Open steps |
| `packages/kernel/src/operator/mod.rs` | Execute open_or_focus units + browser opens |
| `packages/kernel/src/operator/compose.rs` | completed / partial / failed aggregate reporting |
| `tests/compound-open.test.ts` | Regression |
| `scripts/verify-compound-open.mjs` | Machine check |

---

## 3. Before / after execution timeline

### Before

```text
"Open Cursor and Chrome."
  → unknown / clearer request
  → Owner must restate
```

or first-only open then stop.

### After

```text
"Open Cursor and Chrome."
  → Intent: compoundOpen (both resolved)
  → Kernel: open_or_focus Cursor → open_or_focus Chrome
  → Compose: completed | partial (N of M) | failed
```

Beside path unchanged:

```text
"Open Browser beside Cursor."
  → browserOpenBeside → P21.S1 Completion Contract
```

---

## 4. Validation

- Resolvable compounds execute sequentially.
- Unresolved compounds do not invent targets.
- Beside utterances stay on beside composition.
- Single open intents unchanged.
- Partial completion reports N of M truthfully.

---

## 5. Additional compound scenarios now supported

| Utterance pattern | Behaviour |
| --- | --- |
| Open Cursor and Chrome | Dual app open_or_focus |
| Open Chrome and Notepad | Dual app |
| Open ChatGPT and YouTube | Dual browser Open (when both site entities resolve) |
| Open A and B and C | N-way when every part resolves |
| Open X and Unknown | unknown — no invent |

---

## Stop

P21.S2 complete. Do not begin another engineering program.
