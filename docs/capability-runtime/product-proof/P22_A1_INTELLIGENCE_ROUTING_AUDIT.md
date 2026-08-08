# P22.A1 — Intelligence Routing Audit

| Field | Value |
| --- | --- |
| **Program** | P22.A1 — Intelligence Routing Audit |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Commit** | `1e71c98d` |
| **Kind** | Engineering audit — **no implementation** |
| **Depends on** | T1 Companion Conversation; P21.S1 / P21.S2 complete |
| **Not this audit** | Implementation · architecture redesign · new cognitive layers · AGI · replacing Kernel Operator |
| **Posture** | Release Hold remains; architecture stable |

**Core question:** Where does Workspace refuse a request that needs intelligence other than desktop manipulation — and why does it treat that as “outside the desktop”?

**Evaluation principle:** Conversation is the product. Desktop operation, reasoning, knowledge, and calculation are distinct *kinds of intelligence*. Routing must choose the kind first, then the path. Do not expand desktop capability to fake usefulness.

**Live Owner evidence (PP3):** time in WA; Rockhampton→Gladstone distance; shipping cost; general knowledge; desktop capability questions — rejected despite needing little or no desktop action.

---

## 1. Executive Summary

Workspace routes almost every utterance as a **desktop capability match**. When Intent cannot map to a Kernel `CapabilityIntent` (or a Registry discovery reply), Conversation falls through to soft-miss / generic refusal — framing the ask as **outside desktop work**.

That is correct for inventing apps, files, or fake success. It is **incorrect** for time, distance, cost, knowledge, planning, coding explanation, and many “what can you…” meta-questions that need **reasoning or knowledge**, not window control.

**Critical substrate fact (measured):** There is **no** generative Model Provider wired into today’s Conversation → Intent → Kernel path. `docs/05-AI/*` foundations are not live runtime. “Reasoning” in code today means **deterministic** Semantic Intent / Goal Resolution / Registry discovery — scoped to desktop operation. Commodity AI the Owner already uses (ChatGPT) is available only as a **Browser / Application open** target, not as an in-Conversation answer engine.

**Verdict:** Perceived unintelligence is primarily a **routing and framing failure**, not a missing desktop provider. Workspace behaves like a desktop command interpreter because the unknown path assumes *desktop-or-refuse*, with no intelligence-kind gate.

**Smallest systemic causes (three):**

1. **Desktop-or-refuse default** — unknown → `resolveUnknownGuidance` / generics.  
2. **Explicit anti-knowledge near-miss** — weather / joke / “what is the capital” refused as “not general chat.”  
3. **No Reasoning execution surface** — answers that need knowledge cannot complete in Conversation without inventing; ChatGPT handoff is unused for these asks.

**Recommended slice:** **P22.S1 — Intelligence Kind Routing + ChatGPT Reasoning Handoff** (Intent / Conversation only; reuse Browser + ChatGPT entity; no new architecture).

---

## 2. Current Intelligence Routing Model

```
Utterance
  → soften / voice-check
  → Semantic Intent (discovery, situation goals, cognitive desktop, grammar, entities)
  → IntentBridge desktop matchers (open / window / screenshot / …)
  → if match → Kernel Operator (desktop) OR capabilityExplain (Registry text)
  → else → resolveUnknownGuidance (near-miss pool) OR pickGeneric
         → “outside what I can do on the desktop”
```

| Stage | Owner | What it optimizes for |
| --- | --- | --- |
| Semantic Intent / Grammar | Intent Layer | Desktop verb + entity |
| Situation Goals / Context | Intent Layer | Resume / Continuity / Moments |
| Capability Registry discovery | Intent Layer | Desktop can / cannot text |
| Kernel Operator | Kernel | Desktop providers only |
| `conversationGuidance.ts` | Conversation | Truthful **desktop** soft-miss |

**Missing gate:** “What kind of intelligence does this require?”  
**Present gate:** “Which desktop capability string matches?”

**Evidence surfaces:**

| Mechanism | Behaviour |
| --- | --- |
| `GENERIC_REPLIES` | Rotates “outside … desktop”, “not set up for that”, “stay with desktop work” |
| Knowledge near-miss | `weather\|joke\|…\|who is\|what is the capital` → “not general chat or look-ups” |
| Files / terminal near-miss | Honest capability walls (correct LIMIT) |
| `handleOperatorUtterance` | `unknown` / `*Explain` → reply only; never Kernel |
| Product Intelligence gaps | `llm-memory` / agent loops owned **OutsideWorkspace** |

---

## 3. Expected Intelligence Routing Model

```
Utterance
  → Intelligence kind (deterministic classification)
       ├─ DESKTOP ONLY      → existing Intent → Kernel path
       ├─ REASONING ONLY    → answer in Conversation *if* a truthful source exists;
       │                      else hand off to existing AI (ChatGPT) without pretending
       │                      Workspace invented the answer
       ├─ HYBRID            → reason / clarify, then Kernel steps
       ├─ CLARIFICATION     → ask
       ├─ OWNER AUTHORIZATION → ask before destructive / Moments restore
       └─ CAPABILITY LIMIT  → honest wall (files, terminal, invent .exe)
```

| Kind | Expected Workspace behaviour |
| --- | --- |
| REASONING ONLY | Help with knowledge / math / planning / explanation — **do not** label as desktop failure |
| DESKTOP ONLY | Current Operator path |
| HYBRID | Explain then act (or act then explain) using existing providers |
| CLARIFICATION | Ask once |
| OWNER AUTHORIZATION | Moments restore, destructive ops |
| CAPABILITY LIMIT | Files (until P17), invent launch, unsupported OS depth |

**Constitutional constraint preserved:** Intent stays deterministic. Probabilistic paraphrase of desktop *commands* remains rejected. A future in-Conversation model would be a **Capability Provider** (intelligence feature, Kernel-gated reply), not Intent rewriting — and is **out of scope** for the recommended Hold slice.

**Pragmatic “existing AI” today:** ChatGPT via Browser Provider (`https://chatgpt.com`) — commodity WRAP the Owner already Product-Proofed for open / beside.

---

## 4. Classification Matrix

| Representative request | Class | Current routing | Expected routing | Mismatch |
| --- | --- | --- | --- | --- |
| “What time is it in WA?” | REASONING ONLY | Generic / outside desktop | Reason or ChatGPT handoff | Yes |
| “What’s today’s date?” | REASONING ONLY | Outside desktop | Reason or handoff | Yes |
| “How long from Rockhampton to Gladstone?” | REASONING ONLY | Outside desktop | Reason or handoff | Yes |
| “Estimate shipping cost to Gladstone” | REASONING ONLY | Outside desktop | Reason or handoff | Yes |
| “Convert 40°C to Fahrenheit” | REASONING ONLY | Outside desktop | Reason or handoff | Yes |
| “Who wrote Hamlet?” | REASONING ONLY | Near-miss “not general chat” **or** generic | Reason or handoff | Yes |
| “Explain how TLS works” | REASONING ONLY | Outside desktop | Reason or handoff | Yes |
| “What’s a good plan for tomorrow’s demo?” | REASONING ONLY | Outside desktop | Reason or handoff | Yes |
| “How do I structure this React component?” | REASONING ONLY | Outside desktop | Reason or handoff | Yes |
| “Hi” / “Thanks” | REASONING ONLY (conversation) | Greeting / soft path | Companion reply (T1) | Mostly ok |
| “Open Chrome” | DESKTOP ONLY | Kernel open | Kernel open | No |
| “Arrange ChatGPT beside Cursor” | DESKTOP ONLY | Beside compose (P21.S1) | Same | No |
| “Take a screenshot” | DESKTOP ONLY | Screenshots path | Same | No |
| “What windows are open?” | DESKTOP ONLY | Enumerate | Same | No |
| “What can you do?” | DESKTOP META → Registry | `capabilityExplain` when discovery matches | Same | Partial — odd phrasings miss |
| “Can you manage windows?” | DESKTOP META | May miss → outside desktop | Registry / explain | Yes when miss |
| “What’s my CPU usage?” then open Task Manager | HYBRID | Often refuse first half | Explain + offer open | Yes |
| “Find newest screenshot and open it” | HYBRID / LIMIT | Partial / file wall | File Provider later; don’t invent | Partial |
| “Open the downloads folder” | CAPABILITY LIMIT | Files near-miss | Same until P17 | No (correct) |
| “Run this PowerShell” | CAPABILITY LIMIT | Terminal near-miss | Same | No (correct) |
| “Open foobarbaz.exe” | CAPABILITY LIMIT | Honest refuse invent | Same | No (correct) |
| “Restore my coding Moment” | OWNER AUTHORIZATION | Moments confirm | Same | No (correct) |
| “Open A and B” (known) | DESKTOP ONLY | Compound open (P21.S2) | Same | No |
| “Open it” (bound) | DESKTOP ONLY | Context (T1) | Same | No |
| “Delete everything” | OWNER AUTHORIZATION / LIMIT | Refuse / clarify | Same | No (correct) |
| “Open ChatGPT and ask it the time in WA” | HYBRID | May only open ChatGPT if phrased as open | Open + handoff intent | Partial |
| Workflow advice (“how should I set up for coding?”) | HYBRID | Situation goal / Continuity sometimes | Situation + optional apps | Partial |

---

## 5. Misrouting Assessment

### By class

| Class | Current | Expected | Why mismatch | User impact | Effort | Risk |
| --- | --- | --- | --- | --- | --- | --- |
| REASONING ONLY | Refuse as outside desktop / not chat | Help or handoff to existing AI | No intelligence-kind gate; anti-knowledge near-miss | Feels dumb; Owner adapts or leaves | S–M | Low–Med |
| DESKTOP ONLY | Strong when matched | Same | — | — | — | — |
| DESKTOP META | Discovery when regex hits; else refuse | Always Registry truth | Discovery matchers incomplete | “Even help about itself fails” | S | Low |
| HYBRID | First part often refused | Reason + Kernel | Pipeline is single desktop intent | Extra turns; abandoned goals | M | Med |
| CLARIFICATION | Good for pronouns / Moments | Same | — | — | — | — |
| OWNER AUTHORIZATION | Good for Moments | Same | — | — | — | — |
| CAPABILITY LIMIT | Honest files/terminal/invent | Same | Over-applied to reasoning | Contaminates trust | — | — |

### Premature desktop-only classification

Occurs at the **unknown fallthrough**, not in Kernel. Kernel never sees these utterances. Presentation and Intent jointly decide “unsupported desktop” before asking whether desktop was required.

### Where reasoning could answer immediately

Time, date, units, distance/cost *estimation*, general knowledge, technical explanation, coding advice, planning — **none** require Window/Browser/Screenshot providers. Today Conversation **cannot** truthfully answer them in-process (no model). It **can** stop refusing and **hand off** to ChatGPT without inventing numbers.

### Where desktop is genuinely required

Open / focus / snap / screenshot / clipboard / notification / Moments / enumerate — current path is appropriate.

### Hybrid

“Explain then show Task Manager”, “calculate then open Explorer” — need kind split + optional Kernel step. Not the highest-leverage first slice; depends on reasoning handoff existing first.

### Unnecessary refuse vs help

| Pattern | Refuse today? | Should help? |
| --- | --- | --- |
| Knowledge / calc / time | Yes | Yes (handoff or future model provider) |
| Desktop meta | Sometimes | Yes (Registry) |
| Files / terminal | Yes | No — truthful LIMIT |
| Invent executable | Yes | No — truthful LIMIT |

---

## 6. Root Cause Analysis

Avoid prompt laundry lists. Three systemic causes explain Owner PP3:

### RC-1 — Desktop-or-refuse default (Critical)

Intent resolves **desktop operations**. Anything else shares one sink: `resolveUnknownGuidance` → near-miss or `GENERIC_REPLIES`. The product narrates failure as a **desktop coverage gap** even when the ask never needed the desktop.

**Effect:** Companion identity (T1) softens tone but still points every soft miss back to “desktop work.”

### RC-2 — Explicit non-chat product policy in Conversation (Critical)

`conversationGuidance.ts` near-miss actively rejects look-ups and general chat. That was a truthful boundary when Workspace claimed *only* desktop operation. It now **collides** with Product Gravity: Conversation is the product; reasoning/knowledge are expected intelligence kinds.

**Effect:** “What time is it in WA?” and capital/knowledge phrasings are not merely unmatched — some are **matched in order to refuse**.

### RC-3 — No Reasoning execution surface on the live pipeline (Structural)

- Deterministic “reasoning” helps **map desktop intent**, not answer world questions.  
- Model Provider docs exist historically; **not connected**.  
- ChatGPT is a first-class open target but **never selected** as the completion path for reasoning asks.

**Effect:** The only truthful options today are refuse or open an external AI. The product chooses refuse.

**Why it feels like a command interpreter:** The pipeline asks “which command?” not “which intelligence?” Unknown ⇒ help-shaped desktop recovery (even after T1 removed catalogues).

---

## 7. Highest-Leverage Engineering Opportunities

| # | Opportunity | Owner Value | Effort | Risk | Notes |
| --- | --- | --- | --- | --- | --- |
| **1** | **Intelligence kind gate + ChatGPT handoff for REASONING ONLY** | **High** | **S–M** | **Low–Med** | Reuses Browser + ChatGPT; no LLM in Intent; no Kernel redesign |
| 2 | Widen desktop-meta → `capabilityExplain` | High (trust) | S | Low | Stops self-help refusals |
| 3 | Retire anti-knowledge near-miss / desktop-framed generics for reasoning class | High | S | Low | Framing only; must pair with handoff or still feels empty |
| 4 | Hybrid templates (explain → open Task Manager, etc.) | Med–High | M | Med | After #1 |
| 5 | In-Conversation Model Provider (reply-only, Kernel-gated) | Very High | L | High | New capability program; not Hold default; not Intent ML |
| 6 | Local deterministic calculators (timezones, unit convert) | Med | M | Med (correctness) | Narrow; does not replace knowledge |

**Do not prioritize:** Replacing Operator; AGI planner; LLM paraphrase of desktop intents; File Provider under Hold; expanding desktop APIs to fake knowledge.

---

## 8. Recommend — ONE bounded engineering slice

### P22.S1 — Intelligence Kind Routing + ChatGPT Reasoning Handoff

**Not a new architecture.** Not AGI. Not Intent-layer probabilistic command parsing. Reuses Browser Provider + existing ChatGPT entity; Conversation / Intent classification only.

**Problem:** Reasoning/knowledge/calculation asks hit desktop refusal copy. Owner experiences a command interpreter that cannot think — even when thinking needs no desktop control.

**Outcome:** For clear REASONING ONLY asks, Workspace either:

1. Offers / performs a **ChatGPT open** (truthful handoff to existing AI), or  
2. Asks once (“Want me to open ChatGPT for that?”) then opens —

and **never** claims the answer was computed inside Workspace if it was not. Desktop meta-questions route to Registry discovery instead of soft-miss.

**In scope:**

1. Deterministic **intelligence-kind classifier** in Intent / Conversation (before `resolveUnknownGuidance`).  
2. REASONING ONLY → handoff path using existing `browserOpen` / app open to ChatGPT (query handoff if a stable URL pattern exists; otherwise open ChatGPT + truthful “ask it there” line).  
3. Retarget knowledge near-miss + generics so reasoning class is not “outside the desktop.”  
4. Ensure common desktop-capability questions hit `capabilityExplain`.  
5. Verifier + tests: time / distance / knowledge samples → not GENERIC outside-desktop; desktop opens unchanged; invent-exe and files walls unchanged.

**Out of scope:**

- Embedding an LLM inside Intent  
- Kernel Operator redesign  
- File Provider / terminal  
- Hybrid CPU→Task Manager templates (follow-on)  
- Claiming invented facts in Conversation  
- Spec / constitutional layer changes  

**Success test:** Owner asks “What time is it in WA?” → Workspace does **not** refuse as outside desktop; it collaboratively routes to existing AI (or answers only if a later truthful source exists). “Open Notepad” and file walls still behave as today.

| Constraint | Posture |
| --- | --- |
| Preserve constitutional architecture | Yes |
| Preserve Release Hold | Yes — Owner-authorized slice only |
| Reuse existing AI | Yes — ChatGPT via Browser |
| Reduce unnecessary refusals | Yes |
| Increase perceived intelligence | Yes |
| No new architecture / AGI / Operator replacement | Yes |

---

## Explicit answers

| Audit question | Answer |
| --- | --- |
| Premature desktop-only? | Yes — unknown sink + anti-knowledge near-miss |
| Reasoning could answer immediately? | Yes for kind; in-process answers need model provider (absent); handoff available now |
| Desktop genuinely required? | Open/arrange/capture/Moments/enumerate paths |
| Hybrid appropriate? | Yes for explain-then-show; after reasoning handoff |
| Unnecessary refuse? | Yes for Owner PP3 knowledge/calc/time/meta samples |
| Why command-interpreter feel? | Desktop-or-refuse routing (RC-1–RC-3) |
| Highest leverage? | **Intelligence Kind Routing + ChatGPT Reasoning Handoff** |

| Constraint | Posture |
| --- | --- |
| No implementation | Observed |
| No architecture redesign | Observed |
| No new cognitive layers | Observed |
| Release Hold | Remains |
| Stop after audit | Observed |

---

## Stop

P22.A1 Intelligence Routing Audit complete. **Do not implement** until Owner authorizes **P22.S1 — Intelligence Kind Routing + ChatGPT Reasoning Handoff** (or another ranked opportunity).
