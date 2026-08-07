# P16.31 — Semantic Intent Engine & Desktop Operator Completion

| Field | Value |
| --- | --- |
| **Program** | P16.31 (not P17) |
| **Status** | Engineering complete — awaiting Owner live Product Proof |
| **Branch** | `v2-dev` |
| **Prior** | P16.30 Intent Grammar — still required; Semantic Engine reasons over it |

---

## 1. Repository reassessment

Owner live review falsified “no remaining Voice-owned defects.” Remaining failures were **Semantic Understanding**, not Speech Recognition.

Pipeline now:

```
Speech → Transcript → Intent Grammar → Semantic Intent Engine → Desktop Intent
  → Capability Resolution → Execution → Truthful Feedback → Conversation
```

P17 remains blocked. WinRT WRAP unchanged.

---

## 2. Semantic Engine architecture

| Layer | Authority |
| --- | --- |
| `intentGrammar.ts` | Extract Action / Target / Modifier / Object / Context / FollowUp |
| `semanticIntentEngine.ts` | Reason over grammar + known desktop entities → `IntentAction` |
| `capabilityRegistry.ts` | Live capability graph for discovery |
| Kernel Operator | Composition only (`focus_minimize`, `open_foreground`, …) |

Deterministic. No ML. Providers never see grammar or the registry.

---

## 3. Capability Registry

`CAPABILITY_GRAPH` advertises per capability: verbs, aliases, objects, modifiers, requirements, examples.

`What can you do?` / `Show me your capabilities` / `List desktop actions` → `capabilityExplain` **generated** from the graph (not canned Guide-only copy). Explicit “open Guide” still opens Guide chrome.

---

## 4. Commodity comparison (behaviour only)

| System | Behaviour adopted |
| --- | --- |
| Windows Search / Shell | Protocol launches (`ms-windows-store:`, `ms-settings:`); shell folders |
| PowerToys Run / Raycast | Verb + object reasoning; no inventing `name.exe` for spaced titles |
| VS Code / Cursor command palette | Structured parse before execute |
| ChatGPT / Claude Desktop | Capability self-description from a catalogue of what the product can do |
| Kiro | Conversational desktop ops; Workspace keeps Kernel composition + truthful failure |

Not copied: any vendor code.

---

## 5. Kiro behavioural comparison

Kiro-style expectation: ordinary language → desktop effect without the user knowing providers. Gaps closed in P16.31: Store protocol, locate+minimise compose, browser-with-title locate, entity focus (GPT→ChatGPT), generated capability discovery. Remaining vs Kiro/Raycast: deep tab control inside browsers, fuzzy app install search, multi-step automation scripting (P17+ territory — not Voice).

---

## 6. Desktop reasoning improvements

- Microsoft Store → `ms-windows-store:` via rundll32 protocol handler (never `microsoft store.exe`)
- Multi-word unknown launches refuse `.exe` invention
- Locate browser with YouTube → window focus on title
- Locate app with ChatGPT open and minimise → `window.focus_minimize`
- Bring GPT / Focus Chrome / Edge / Restore / Maximise → entity-resolved window ops

---

## 7. Remaining language gaps

- Unlisted apps still use Find→Focus|Launch (honest failure if missing)
- In-browser tab APIs beyond window title matching are not owned
- Long multi-clause automation (“open A, then B, then email C”) is out of scope for P16

---

## 8. Explicit Owner questions

| Question | Answer |
| --- | --- |
| Reason about desktop ops vs phrase match? | **Yes** — grammar + entity catalog + Kernel compose |
| Discovery generated? | **Yes** — from `CAPABILITY_GRAPH` |
| Remaining executable-name failure path? | Unknown multi-word names no longer get `.exe`; unknown single tokens may still append `.exe` after Find fails — known entities never do |
| Missing vs Kiro/Raycast/Search/ChatGPT Desktop? | Deep tab automation, fuzzy Store search UI, plugin ecosystems — not P16 Voice scope |

---

## 9. STOP

Do not begin P17. Do not mark P16 permanently closed. Await Product Owner review.
