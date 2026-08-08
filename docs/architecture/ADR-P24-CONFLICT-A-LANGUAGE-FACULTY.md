# ADR — P24.S1 Conflict A: Language Faculty Authority

| Field | Value |
| --- | --- |
| **ADR ID** | ADR-P24-CONFLICT-A |
| **Program** | P24.S1 |
| **Date** | 2026-08-08 |
| **Status** | **ACCEPTED** |
| **Branch** | `v2-dev` |
| **Base commit** | `6283384c` |
| **Kind** | Constitutional / architectural decision — **no Language Faculty implementation in this slice** |
| **Supersedes** | Unresolved Conflict A in Behavioral Constitution §30; over-broad reading of Product Proof Rule NL Robustness line on “Intent resolution” |
| **Does not supersede** | Spec v2 Determinism for Authority/Execution; Kernel Authority; Composition; Permission Gateway; Completion Contract; Conflict C |

---

## 1. What Conflict A is

**Conflict A** is a contradiction between two authorities about *where* determinism is mandatory:

| Authority | Text | Scope |
| --- | --- | --- |
| **Constitutional Spec v2** (§12) | “Authority and Execution are not probabilistic for operating-system Effects” | Plan → Effect gate and Effect production |
| **Product Proof Rule** (NL Robustness) | “Intent resolution remains deterministic (no probabilistic AI matching)” | Reads as the whole Intent Layer, including comprehension |
| **Behavioral Constitution P10** | “Comprehension may be uncertain; authority may not” | Understanding may be probabilistic; deciding/permitting/executing/completing may not |
| **Spec v2 §7.3** | Models “MUST NOT silently become authoritative” | Contemplates models; forbids hidden authority |
| **Intent Layer Spec** | “deterministic bridge today; **model later**” | Already anticipated a model behind the bridge |

Conflict A is therefore not “may Workspace have a model?” — Spec and Intent already allow that — but **“may probabilistic comprehension produce Meaning / GoalContract without violating Product Proof Rule wording?”**

Until this ADR, that question blocked Language Faculty implementation (P24 Experience Contract §2.2).

---

## 2. Why the current architecture creates it

### 2.1 Measured path (repository evidence)

```text
Owner utterance
  → OperatorRoot.submitUtterance
  → handleOperatorUtterance (app/src/lib/operator/intelligence.ts)
  → resolveIntentWithGoal (intentBridge.ts)
       1. groundGoalInContext(comprehend(raw))     ← Meaning (GoalContract) — deterministic regex today
       2. resolveFromWorkspaceContext / answerForGoal
       3. resolveIntentCore → IntentAction          ← action selection — ~469 regexes
       4. bridgeObservationRequest / enforceSubstitutionProhibition
  → intentMap → CapabilityIntent { domain, operation, arguments }
  → IPC execute_capability_intent
  → Kernel Operator (plan / compose)                ← sole composition authority
  → Permission Gateway
  → Provider → OS
  → OperatorTurnResult / Facts
  → composeObservationAnswer / compose.rs / canned TS strings  ← Experience wording
```

| Concern | Where it lives today | Probabilistic? |
| --- | --- | --- |
| Meaning created | `comprehend()` → `GoalContract` | No (keyword/signal tables) |
| Capability / action selected | `resolveIntentCore` + maps | No |
| Provider identity | Kernel Operator + Runtime | No |
| Execution authority | Kernel Operator | No |
| Permission | Permission Gateway | No |
| Verification → truth | Completion Contract / providers | No |
| Owner-facing wording | Hand-authored TS + `compose.rs` | No |
| Model entry point | **None** | — |

### 2.2 Why product value fails

P23 proved outcome-first comprehension and answer sources help — and still cannot scale ordinary language. “Tell me about yourself” hits limitation copy because no string was authored. Compound goals that preserve a *count* outcome across folders are incomprehensible as one goal when only phrase tables exist. Natural Language Robustness and User Adaptation Prohibition demand the Owner not memorize Workspace’s dictionary — which a closed regex set structurally requires.

---

## 3. Decision: is probabilistic language comprehension allowed?

### **YES — under a Language Faculty Boundary (Option C).**

Comprehension (Signal → Meaning) and expression (Facts → Experience wording) **MAY** use a probabilistic model.

Authority, capability selection, provider selection, planning, permission, execution, verification, completion, device facts, application identity, and memory writes **MUST remain deterministic and non-model**.

This is the interpretation required by Spec §12 + §7.3 + Interpretation Rule (§19): preserve Explicit Authority and the Transformation Chain. The Product Proof Rule’s NL Robustness sentence is **amended** by this ADR (§18) so “Intent resolution” no longer swallows Understanding.

---

## 4. Language Faculty — allowed

A formal component **Language Faculty** is authorized as architecture (implementation later).

**May:**

- natural-language comprehension and conversational understanding
- ambiguity / uncertainty detection
- reference interpretation (proposal only — grounding is deterministic)
- clause extraction and compound detection
- goal / outcome / mode / domain classification into the GoalContract vocabulary
- requested-result extraction
- context-*sensitive interpretation* when given a **read-only Context Bundle**
- Owner-facing wording from a **verified Fact Set** only

**Outputs permitted:**

1. **Meaning proposal** — candidate GoalContract-shaped structure (see §5–§6)
2. **Expression** — natural-language string derived only from Fact Set + optional style hints

---

## 5. Language Faculty — forbidden

**Must not:**

| Forbidden | Reason |
| --- | --- |
| Capability / provider / operation selection | Intent Spec; Kernel Authority; Composition |
| Constructing `CapabilityIntent` or IPC | Kernel Authority |
| Execution planning / step sequences | Conflict C / Kernel composition |
| Permission grants or bypass | Permission Gateway |
| Desktop effects | Meaning ↛ Effect (Spec) |
| Completion / verification claims | Completion Contract; Evidence owns Fact |
| Inventing device facts, window lists, process names | Observation / Answer Source ladder |
| Inventing or writing memory / preferences | Memory authority is deterministic policy |
| Capability invention / self-modification | C-INT-005 rejected; Spec §7.3 |
| Becoming the sole path with no fallback | Reliability; P24 migration Rule 1 |

---

## 6. Model output contract — GoalContract is sufficient

### 6.1 Evaluation

Existing `GoalContract` (`app/src/lib/goalContract.ts`) already carries:

`utterance`, `normalized`, `mode`, `outcome`, `domain`, `subject`, `targets[]`, `references[]`, `requestedResult`, `compound`, `clauses[]`, `clarificationNeeded`, `uncertainties[]`, `evidence[]`

Deliberately absent (and must stay absent): capability IDs, provider names, domain/operation pairs, execution steps, plans.

**Verdict:** GoalContract is **sufficient** as the Meaning contract. No authority-bearing fields are required. Adding them would *create* Conflict C leakage, not resolve Conflict A.

### 6.2 Optional non-authority metadata (allowed later, not required now)

If implementation needs operational metadata, attach **outside** GoalContract (wrapper), never inside Meaning:

| Field | Allowed? | Notes |
| --- | --- | --- |
| `ambiguityScore` / confidence | Yes (wrapper) | Informs clarify vs accept; never authorizes Effect |
| `modelId` / `latencyMs` | Yes (wrapper) | Audit / privacy |
| `schemaVersion` | Yes | Validation |
| `capabilityId` / `provider` / `operation` | **No** | Authority leakage |
| `steps[]` / plan | **No** | Conflict C |

### 6.3 Expression contract — Fact Set

```text
Fact Set (verified) → Language Faculty (expression) → Owner sentence
```

A Fact Set is a closed list of typed claims with provenance, e.g.:

- `window_titles: string[]` from observation items
- `active_title` / `process_name` from C-OBS-002 path
- `completion: { status, completed[], failed[] }` from Completion Contract
- `refusal_reason` from substitution / permission

**Invariant:** every entity mentioned in the sentence must appear in the Fact Set or in a fixed template slot. No new proper nouns, counts, or success claims.

---

## 7. Probabilistic model boundary — options evaluated

| Option | Shape | Safety | Determinism of Effect path | Testability | Hallucination | Authority leakage | Latency | Privacy | Fallback | Verdict |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| **A** | Model → GoalContract directly accepted | Medium | OK if Kernel still selects | Harder (accepts model JSON) | Higher | Higher if schema soft | Best | Same as model site | Weak | **Reject as primary** |
| **B** | Model → new IR → map → GoalContract | High | Strong | Strong | Medium | Low | Extra hop | Same | Strong | Acceptable but larger |
| **C** | Model → **proposal** → **deterministic validator** → GoalContract | **Highest** | **Strong** | **Strong** | Contained | **Lowest** | One validate pass | Same | **Cascade fallback** | **Adopt** |

### Adopted: **Option C**

```text
Utterance + Context Bundle (read-only)
        ↓
Language Faculty (probabilistic) → MeaningProposal (GoalContract-shaped JSON)
        ↓
Deterministic Validator (schema + vocabulary + constitutional checks)
        ↓
    accept → GoalContract
    reject → Deterministic comprehend() fallback  OR  clarify  OR  limitation (no Effect)
        ↓
Existing Intent path (deterministic action selection / answer ladder / substitution)
        ↓
Kernel Operator → Permission → Effect → Fact
        ↓
Fact Set → Language Faculty (expression) → Experience   [optional; canned strings remain valid]
```

**Smallest architecture that preserves constitutional boundaries.** GoalContract stays the Meaning type; the cascade stays; Kernel authority unchanged.

---

## 8. Model is not authority — hard invariants

```text
MODEL OUTPUT ≠ AUTHORITY
```

| Claim | Forbidden for model |
| --- | --- |
| Authorize Effect | Yes |
| Create capability | Yes |
| Select provider | Yes |
| Bypass Permission Gateway | Yes |
| Declare completion | Yes |
| Declare verification | Yes |
| Invent device / desktop facts | Yes |
| Invent application identity | Yes |
| Write memory / preferences | Yes |
| Rewrite constitution / this ADR | Yes |

A valid GoalContract from the Faculty still cannot cross Plan → Effect without Kernel planning and Permission Gateway. Substitution Prohibition still runs on the *goal*, not on model confidence.

---

## 9. Deterministic validation (mandatory after proposal)

Order is fixed:

1. **Schema validation** — required fields; types; closed enums for `mode` / `outcome` / `domain` / target roles  
2. **Vocabulary validation** — no unknown outcome/domain; no capability-/provider-shaped strings in evidence  
3. **Authority-leak scan** — reject if proposal contains capability ID, provider ID, operation, IPC name, step plan  
4. **Semantic consistency** — e.g. `KNOW`/`COMPUTE`/`SOCIAL` must not carry action-only mode without hybrid evidence; `REACH_STATE` requires effect evidence  
5. **Context grounding (deterministic)** — bind `references[]` via Workspace Context / registers; Faculty proposals for referents are hints, not truth  
6. **Constitutional gates** — Substitution-facing outcomes preserved; observation vs effect not collapsed  

**On failure (no unbounded retries):**

| Failure | Behaviour |
| --- | --- |
| Malformed / leak fields | Discard proposal → **deterministic `comprehend()` fallback** |
| Low confidence / high uncertainty | Prefer **clarify** if `clarificationNeeded` or unresolved refs |
| Contradictory clauses | Clarify or fallback |
| Hallucinated target (unrecognized + invented canonical) | Strip canonical; mark unrecognized; clarify if action needs it |
| Model unavailable / timeout / refusal | **Deterministic fallback** (whole current system) |
| Unsafe semantic output | **No Effect** — limitation reply |

Maximum comprehension attempts per Owner turn: **one** Faculty call + **one** deterministic fallback. No retry loops.

---

## 10. Context boundary

```text
CONTEXT BUNDLE (read-only, curated) → Faculty → MeaningProposal
OBSERVED FACTS → deterministic policy → MEMORY / REGISTERS
```

**May be provided to the Faculty (read-only):**

- current utterance
- session referents (R3) needed for pronoun binding *as candidates*
- explicit preferences summaries the Owner has set (R2), if relevant
- device facts that are already verified for the turn (optional)
- capability self-model summaries that are registry/Atlas-derived (optional, for META questions only)

**Must not be provided / must not be writable by Faculty:**

- raw observation buffers as unbounded history
- screen pixels / document contents
- credential material
- write handles to SQLite / preferences / memory APIs

**Invariant:** Faculty may *propose* that a reference resolves to X; only deterministic grounding confirms X.  
**Invariant:** Faculty never writes R1–R7.

---

## 11. Memory boundary

```text
MODEL ↛ MEMORY
```

Memory formation remains P24 Retention Test + deterministic services. Faculty output is never a memory source. If the Owner *states* a preference in language, Understanding may classify it as preference-shaped Meaning; **persistence** still requires deterministic policy and Owner-visible write — not silent model side-effect.

---

## 12. Privacy architecture (no provider chosen)

| Mode | Allowed by this ADR? | Data leaving device |
| --- | --- | --- |
| **No model** (deterministic only) | Yes — default until Faculty implemented | None for comprehension |
| **Local model** | Yes — preferred long-term | None |
| **Remote model** | Yes — only with **explicit per-use or standing Owner authorization** | Utterance + Context Bundle only; never raw desktop capture unless separately authorized |
| **Hybrid** | Yes — local first; remote on authorized escalate | Same as remote when used |

Architecture must support swapping implementations behind one Faculty interface. **This ADR does not select GPT/Claude/Grok/Ollama/etc.** B-RES-001 / Evidence Before Commitment still applies before a concrete backend is adopted.

G9 (regex ChatGPT URL handoff) remains a separate product defect; Faculty must not worsen silent external transfer.

---

## 13. Fallback and regex migration

### Role of the ~469-regex cascade

| Role | Adopt? |
| --- | --- |
| Delete immediately | **No** |
| Sole comprehension forever | **No** |
| **Deterministic fallback** when Faculty unavailable/invalid | **Yes — primary** |
| **Safety / authority recognizer** (effect verbs, explicit commands) | **Yes** |
| **Regression oracle** in tests | **Yes** |
| Low-latency fast path for exact known phrases | **Optional later** |
| Validation cross-check (Faculty vs cascade disagree → clarify or prefer safer) | **Yes — recommended** |

**Migration strategy:** Faculty wraps *comprehension only*. `resolveIntentCore` remains until Conflict C / generative binding is separately addressed. Never “replace 469 regexes with an LLM” as a single step.

P24 Experience Contract Rule 1 is affirmed: Workspace must never become less reliable than today’s cascade.

---

## 14. Multi-step implication and Conflict C

### Example

> “Open File Explorer, go to Pictures, open Screenshots, count the images.”

**Faculty MAY produce (Meaning):**

- `outcome`: hybrid KNOW + REACH_STATE (or structured compound with `requestedResult: "count"`)
- `compound: true`
- clauses / targets: File Explorer, Pictures, Screenshots
- `requestedResult`: count of images
- uncertainties if File Provider / folder nav unsupported

**Faculty MUST NOT produce:**

- `C-ACT-001`, `explorer.exe`, click plans, IPC, provider names

### Bridge

```text
SEMANTIC GOAL (GoalContract)
  → deterministic Intent / answer / limitation  (today)
  → OR future Kernel GoalContract binding      (Conflict C)
  → Permission → Effect → Fact → Expression
```

### Conflict C interaction — verified

| Claim | Verdict |
| --- | --- |
| Conflict A resolved independently of Conflict C? | **Yes** |
| What Conflict A unlocks | Language Faculty Meaning + Fact-Set expression; ordinary phrasing; conversational answers from facts |
| What Conflict C still blocks | Generative multi-step *capability binding* / open-ended observe→act→verify loops from GoalContract inside Kernel |
| New Kernel interface required for Faculty? | **No** for Conflict A |
| Kernel accepting GoalContract? | **Future Conflict C ADR** — not this decision |
| Capability selection remains Kernel-side? | **Yes — unchanged** |

P24’s claim stands: Conflict C does not block Personal AI identity; it blocks generative multi-step binding.

---

## 15. Natural-language response (expression)

Same Faculty **may** generate wording when given a Fact Set. It **must not** fabricate facts, results, completion, verification, or device state.

Canned strings and `compose.rs` remain valid forever. Expression is an improvement path, not a requirement for first Faculty ship.

---

## 16. Testability (semantic invariants — not exact strings)

Probabilistic tests assert **GoalContract invariants**, not reply prose.

| Utterance | Must hold |
| --- | --- |
| “What time is it in Queensland?” | `outcome ∈ {KNOW, COMPUTE}` / time domain; not `REACH_STATE` |
| “Open Chrome.” | `REACH_STATE` (or action mode); not pure KNOW |
| “What does minimize mean?” | `KNOW`; not window minimize effect |
| “Open Pictures and count the images.” | Preserves count/`requestedResult`; compound true; not “open only” |
| “What application am I using?” | `PERCEIVE_MACHINE` / observation |
| “Open that.” (no context) | Unresolved reference / clarification; **no Effect** |
| “How are you?” | `SOCIAL`; no desktop effect |
| Action vocabulary inside a question (“what windows do I have open”) | Not `REACH_STATE` from “open” |
| Model invents `capabilityId` | Validator **rejects**; fallback or limit |

Adversarial suite: multi-action lists, ambiguous targets, unknown apps, pronouns, desktop vocabulary in questions, fabricated capabilities.

---

## 17. Verifier design (Language Faculty Boundary)

**When Faculty code exists**, `scripts/verify-language-faculty.mjs` (name reserved) must fail if Faculty modules:

- import capability registry / provider identity for selection
- construct `CapabilityIntent` / execution plans
- call `invokeIpc` / `@tauri-apps` / desktop providers
- grant permission / declare completion / verification
- emit capability IDs or invent target identity without observation

Must pass if Faculty modules:

- emit MeaningProposal / GoalContract-shaped meaning only
- detect ambiguity
- express wording only from Fact Set builders

**Not implemented in this slice** — design only.

---

## 18. Product Proof Rule amendment (applied by this ADR)

### Prior text (over-broad)

> Intent resolution remains deterministic (no probabilistic AI matching).

### Amended text (authoritative)

> **Authority-bearing Intent resolution** — mapping accepted Meaning to actions / `CapabilityIntent`, and all Kernel planning, permission, and Effects — remains deterministic (no probabilistic AI matching).  
> **Comprehension** (Signal → Meaning / `GoalContract`) and **expression** (verified Fact Set → Owner wording) **MAY** use a probabilistic Language Faculty **only** behind the Language Faculty Boundary (ADR-P24-CONFLICT-A): model output is a proposal, deterministically validated, never authority, never Effect, never fact invention.  
> Providers remain deterministic.

Natural Language Robustness otherwise unchanged: improvements belong in Intent Layer, Kernel compose, and Conversation replies — not inside providers.

---

## 19. Relationships to standing authorities

| Authority | Relationship |
| --- | --- |
| **Kernel Authority** | Unchanged. Faculty never calls providers. |
| **Permission Gateway** | Unchanged. Model cannot grant or bypass. |
| **Completion Contract** | Unchanged. Completion from verified Facts only. |
| **Substitution Prohibition** | Unchanged. Runs on GoalContract after acceptance. |
| **Capability Composition** | Unchanged. Providers never call providers; Kernel composes. |
| **Product Proof** | Owner outcomes in P24 §32; Faculty ships only with boundary verifiers + PP for conversational truthfulness |
| **Conflict C** | Remains open; separate ADR when generative binding is prioritized |
| **C-REA-004 / B-RES-001** | Rescope target = Language Faculty; still needs Evidence Before Commitment before a backend is chosen — **ACCEPT here does not adopt a vendor** |

---

## 20. Product-value impact

| Owner failure | How this decision helps |
| --- | --- |
| Time in Queensland → ChatGPT | Better KNOW/time comprehension + existing answer ladder (P23.S3) |
| “What application am I using?” natural phrasing | Observation meaning without phrase enumeration |
| Compound requests lose the real outcome | `requestedResult` + compound preserved in Meaning |
| Conversational questions become desktop/limitation | SOCIAL/KNOW classification without keyword tables |
| Giant command dictionary feel | Faculty understands; cascade is safety net |
| Future multi-step as one goal | Meaning can represent one goal; **binding still Conflict C** |
| AI should understand what Owner means | Explicitly authorized as Understanding ownership |

---

## 21. Migration strategy (implementation later — not this slice)

1. Introduce Faculty interface + MeaningProposal validator + Fact Set type (still no model).  
2. Wire validator in front of `comprehend()` with Faculty stub = current deterministic comprehend.  
3. Add boundary verifier.  
4. Complete B-RES-001 research → choose local/remote/hybrid backend.  
5. Enable Faculty for comprehension with cascade fallback.  
6. Optionally enable expression from Fact Sets for observation/completion replies.  
7. Do **not** remove cascade until Faculty + verifier + Owner PP prove equal-or-better reliability.

---

## 22. Decision

# **ACCEPT**

**Precise reason:**

Constitutional Spec v2 already constrains determinism to **Authority and Execution** for OS Effects, already forbids models from becoming authoritative (§7.3), and already separates Understanding (Meaning) from Authority. Intent Layer Spec already says “model later.” Behavioral Constitution P10 states the correct split. The Product Proof Rule’s single over-broad sentence was the blocker; it is amended herein so probabilistic **comprehension and expression** are permitted **only** as Option C proposals behind deterministic validation, with the regex cascade retained as fallback and safety net.

This ACCEPT authorizes the **architecture**. It does **not** authorize installing a model, choosing a vendor, implementing Faculty code, resolving Conflict C, or advancing C-REA-004 past research.

---

## 23. Remaining blockers before implementation

| Blocker | Blocks |
| --- | --- |
| **B-RES-001** / Evidence Before Commitment | Concrete model backend (local/remote) |
| Faculty interface + validator + verifier (engineering) | Safe wiring |
| Owner selection of P24.S2 scope | What ships next |
| **Conflict C** (separate) | Generative multi-step Kernel binding |
| G9 consent for external handoff | Privacy polish (related, not Faculty-specific) |

---

## 24. Recommended P24.S2 — DONE

Shipped as P24.S2: `app/src/lib/languageFaculty.ts`, `validateMeaningProposal`,
`comprehendViaFaculty`, `scripts/verify-language-faculty.mjs`,
`tests/language-faculty.test.ts`. Deterministic `comprehend()` is the sole Faculty.
No model. Conflict C untouched.

**Recommended next:** B-RES-001 model-backend research (ADOPT/WRAP/…) **or** a
thin Fact-Set expression port still without a model — Owner selects.

---

## Stop

No Language Faculty runtime. No model. No Kernel change. No Conflict C resolution. No Workspace launch.
