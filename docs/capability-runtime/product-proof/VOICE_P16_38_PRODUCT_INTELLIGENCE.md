# P16.38 — Product Intelligence Boundary & Premium Desktop Operator Validation

| Field | Value |
| --- | --- |
| **Program** | P16.38 (not P17) |
| **Status** | Engineering complete for program — Owner live Product Proof pending |
| **Branch** | `v2-dev` |
| **Voice / grammar / Goal Resolution / Context expansion** | Context: Back/Previous only (evidence-required); otherwise not expanded |

---

## 1. Repository reassessment

- P10–P15 closed; P16 Product Proof **OPEN**; P17 blocked.
- P16.37 Context is necessary but **not sufficient** for premium operator feel (F12–F14).

### Owner findings (authoritative until disproven)

| ID | Finding | Probe before P16.38 |
| --- | --- | --- |
| F12 | Conversation feels shallow / command-like | Setup/lost/screenshots routed, but replies engine-framed |
| F13 | Discoverability incomplete | Discovery lacked similar/related; leaked “Capability graph” |
| F14 | Desktop awareness incomplete | Back/Previous → unknown; full layouts still Moments/OS |

---

## 2. Premium desktop operator audit (behaviour only)

| System | Workspace stronger | Workspace weaker | Stay different |
| --- | --- | --- | --- |
| Raycast / PowerToys | Governed composition + truthful failure | Fuzzy launch index | Conversation-first, not launcher |
| Windows Search | Intention → Continue / clarify | Index breadth | No invent from index |
| Kiro | No hidden agent loops | Autonomous coding loops | Outside — Kernel composition only |
| Claude / ChatGPT Desktop | Desktop effects via Kernel | Long chat memory | Outside — deterministic Context |
| Copilot | Explicit Owner approve restore | OS suggestions | Outside |

---

## 3. Product intelligence audit

**Does Workspace still behave only as an execution engine?** Partially falsified — intention routes exist; F13 leak + thin discovery + Back/Previous confirmed remaining engine feel.

**Inside Workspace:** Conversation, Registry discovery, Context continuity, Goal Resolution, Planner, Kernel, truthful Evidence.  
**Outside:** Fuzzy OS index, LLM memory, agent loops, full filesystem (P17), tray polish (Track A).

Gap table: `PRODUCT_INTELLIGENCE_GAPS` in `app/src/lib/productIntelligence.ts`.

---

## 4. Capability Registry maturity

Every node now declares **purpose** plus prior governance fields.  
Discovery generates can / cannot / why / needs / **related·similar·alternatives** / recovery from the graph — no hardcode catalogues.  
`validateCapabilityGraphGovernance()` rejects incomplete nodes.

**Mature enough for permanent architecture?** **Yes, as discovery authority** — subject to Owner Product Proof. Growth still gated by governance.

---

## 5. Cognitive architecture review

```
Speech → Normalization → Grammar → Situation Goals → Semantic
  → Workspace Context → Goal Resolution → Execution Planning
  → Capability Registry → Kernel → Evidence
```

No new layer. Product Intelligence is boundary + trust enforcement, not a pipeline stage.  
Context gained Back/Previous only (F14 evidence).

| Layer | Responsibility |
| --- | --- |
| Situation Goals | High-level situations |
| Workspace Context | Session continuity / referents |
| Goal Resolution | Single-turn underspecify |
| Registry | Discovery + governance |
| productIntelligence | Boundary ownership + leak detection |

---

## 6. Hostile product battery

`tests/product-intelligence-battery.test.ts` — ≥500 interactions; gates: match ≥90%, goal-oriented ≥85%, engineering leaks = 0.

---

## 7. Benchmark comparison

See §2. Competitive **in Conversational Desktop Operator category** when Owner intentions + truthful limits hold — not as a Raycast/Search replacement.

---

## 8. Architectural simplifications

- Removed Owner-facing “Capability graph” wording.
- Discovery purpose + related/similar from Registry (one source).
- No Goal Resolution / grammar / Situation Goal table growth.

---

## 9. Governance additions

**Repository Evidence Before Architectural Confidence** — permanent in `PRODUCT_PROOF_RULE.md`.

---

## 10. Validation summary

`pnpm typecheck` · `build` · `test` · cargo check · product-intelligence + prior cognitive verifiers.

---

## 11–14. Handoff

| Item | Status |
| --- | --- |
| Commit | (filled at push) |
| Branch | `v2-dev` |
| Health | `pnpm sync:project-health` |
| Product Proof | Owner live review — **not** permanently closed |
| P17 | **Do not begin** |

### Explicit answers

1. **Genuine desktop operator?** Engineering path improved; **Owner experience is the gate**.  
2. **Goal-oriented conversation?** F12 families route to Continue / clarify / Pictures — not raw commands.  
3. **One responsibility per layer?** Yes.  
4. **Registry permanent architecture?** Yes as discovery authority (Owner-gated).  
5. **Competitive in category?** Yes for governed conversational desktop — not launchers/agents.  
6. **Intentionally outside?** Fuzzy index, LLM memory, agent loops, P17 files, Track A tray.  
7. **Evidence?** Probe + ≥500 battery + governance validator + leak checks.  
8. **More deterministic P16?** Only if Owner proves residual Product Proof defects — not speculative expansion.

Artifacts: `app/src/lib/productIntelligence.ts`, `tests/product-intelligence-battery.test.ts`, `scripts/verify-product-intelligence.mjs`.
