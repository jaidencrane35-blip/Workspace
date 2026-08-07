# P16.39 — Product Operator Intelligence Validation

| Field | Value |
| --- | --- |
| **Program** | P16.39 (not P17) |
| **Status** | Engineering complete for program — Owner live Product Proof pending |
| **Branch** | `v2-dev` |
| **Voice / grammar / Context / Goal Resolution / Registry** | Not expanded |
| **Situation Goals** | Operator activity states only (evidence-required) |

---

## 1. Repository reassessment

- P10–P15 closed; P16 Product Proof **OPEN**; P17 blocked.
- P16.38 boundary/discovery maturity **falsified as sufficient** for operator-session thinking (F15).

### F15 probe (before)

| Utterance | Before |
| --- | --- |
| I need to get back into work | unknown (“outside…”) |
| Set me up | unknown |
| I'm starting my day | unknown |
| I'm coding / debugging / researching / reviewing / writing documentation | unknown |
| I'm taking a break | unknown |
| Let's continue / Take me back / I'm finished | already routed |

---

## 2. Desktop Operator investigation

Workspace understands **live windows** (enumerate) and **approved Moments** (Continue).  
It does **not** invent application groups from a work-mode label.  
Operator activities map to **user goals** → Continue / clarify — Desktop Operator executes restore only after Owner approval.

---

## 3. Session reasoning investigation

**Is a new cognitive layer required?** **No.**

| Concern | Owner |
| --- | --- |
| Operator activity / work mode | **Situation Goals** (already owns high-level situations) |
| Pronouns / again / Back | Workspace Context |
| Underspecified locate/resume phrases | Goal Resolution |
| Plans | Execution Planning |
| Discovery | Capability Registry |
| Restore layout | Desktop Operator (Moments) |

---

## 4. State vs Goal audit

Proven in `STATE_VS_GOAL_OWNERS` (`productIntelligence.ts`):

| Concern | Owner |
| --- | --- |
| User Goal | Situation Goals + Goal Resolution |
| Desktop State | Desktop Operator (enumerate) |
| Workspace State | Desktop Operator (Moments / Continue) |
| Execution State | Execution Planning |
| Conversation State | Workspace Context |
| Capability State | Capability Registry |

No mixing. No new pipeline stage.

---

## 5. Kiro behavioural comparison (behaviour only)

| Behaviour | Kiro | Workspace | Inside P16? |
| --- | --- | --- | --- |
| Capability discoverability | Agent/tool lists | Registry-generated Conversation | Already P16.38 |
| Permission handling | IDE/project grants | OS + Workspace governance | Outside / Track A polish |
| Desktop awareness | Project/workspace files | Windows + Moments | Partial — truthful limits |
| Session continuation | Agent memory/threads | Continue + Context | Yes — F15 Situation Goals |
| Operator interaction | Chat+agent loop | Conversation→Kernel | Intentional difference |
| Recovery / failure explain | Agent retries | Registry recovery + clarify | Already |
| Onboarding | IDE walkthrough | Discovery phrases | Already |
| Autonomous multi-step coding | Core | **Outside** | Never |

Nothing deterministic from Kiro was missed that belongs inside P16 without becoming an agent.

---

## 6. Commodity validation

Adopted behaviour: “resume my desk / mode” → continue-surface (Moments), like OS “resume” metaphors — **not** inventing layouts.  
Not copying Raycast fuzzy index, Kiro agent loops, or ChatGPT long memory.

---

## 7. Capability Registry review

Still permanent discovery authority. No Registry expansion this program. Governance green.

---

## 8. Trust audit

Eliminated “outside what I can operate” refusals for F15 operator states.  
Wording dependence reduced for activity phrases via Situation Goals (deterministic).  
Remaining wording dependence: novel metaphors with no session/desktop anchor → clarify or Outside.

---

## 9. Hostile product battery

`tests/operator-activity-battery.test.ts` — ≥750; match ≥90%; activity classify ≥90%; leaks = 0; outside-refusal = 0.

---

## 10. Architectural simplifications

- No new layer / engine file.
- Operator activities stay in Situation Goals (one responsibility).
- Context / Goal Resolution / Registry untouched for F15.

---

## 11–15. Handoff

| Item | Status |
| --- | --- |
| Validation | typecheck · build · test · cargo check · verify-operator-activity |
| Commit | (filled at push) |
| Branch | `v2-dev` |
| Health | Product Proof pending |
| P17 | **Do not begin** |

### Explicit answers

1. **Operator activities vs commands?** Yes for F15 families → Continue / clarify.  
2. **Desktop state ≠ user goals?** Yes — `STATE_VS_GOAL_OWNERS`.  
3. **Workflows vs requests?** Session resume/end/work-mode → Continue workflow; not multi-step invent.  
4. **Missed from Kiro?** No P16-owned deterministic gap; agent loops stay Outside.  
5. **Registry sufficient?** Yes as discovery authority.  
6. **Premium operator feel?** Engineering path improved; **Owner gate**.  
7. **Outside?** Fuzzy index, LLM memory, agent loops, P17 files, Track A tray, OS layout APIs.  
8. **Evidence?** F15 probe + ≥750 battery + verifiers.  
9. **More P16?** Only if Owner proves residual Product Proof defects.

Artifacts: `app/src/lib/situationGoals.ts`, `tests/operator-activity-battery.test.ts`, `scripts/verify-operator-activity.mjs`.
