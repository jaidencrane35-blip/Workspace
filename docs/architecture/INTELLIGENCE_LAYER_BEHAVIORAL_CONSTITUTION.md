# Intelligence Layer Behavioral Constitution

**Type:** Design / Architecture (audit only — no implementation)
**Status:** Proposed — requires Owner review and, for two items, an ADR + Constitutional Review
**Authority position:** Subordinate to `docs/00-Constitution/WORKSPACE_CONSTITUTIONAL_SPECIFICATION_V2.md`. Where this document and the Specification disagree, the Specification wins and the disagreement is named in §30.
**Scope:** What the Intelligence Layer of Workspace is supposed to *be*. Not what to build next sprint.

This document was written against the repository as it exists, not against an idealised description of it. Every claim about current behaviour is traceable to a file and line. Where the current implementation contradicts the design proposed here, the contradiction is stated rather than smoothed over.

---

## 0. Grounding audit — what Workspace actually does today

This section exists first because the rest of the document is only credible if it is built on measured behaviour.

### 0.1 The pipeline as implemented

```text
OperatorRoot.tsx
  → handleOperatorUtterance()            app/src/lib/operator/intelligence.ts:24
  → resolveIntent(utterance)             app/src/lib/intentBridge.ts
      ├─ resolveFromWorkspaceContext     short-circuits and returns on a hit
      ├─ resolveIntentCore               the ordered resolver cascade
      ├─ applyGoalResolution             refines the action after the fact
      └─ commitWorkspaceContext
  → toCapabilityIntent(action)           app/src/lib/operator/intentMap.ts
  → executeCapabilityIntent(intent)      single IPC
  → CommandHandler::execute_capability_intent
  → plan_capability_intent → per-step Permission Gateway → compose_user_reply
  → { kind: "reply", text }
```

`handleOperatorUtterance` is 82 lines and terminates unconditionally after one IPC round trip. There is no loop, no observation feedback, and no goal that survives the turn.

Note the position of goal resolution: it runs *after* `resolveIntentCore` has already chosen an action, and it refines that action. It does not decide anything before the choice is made.

### 0.2 Measured shape of the Intent layer

| File | Lines | Regex literals |
| --- | --- | --- |
| `app/src/lib/intentBridge.ts` | 2,277 | ~182 |
| `app/src/lib/semanticIntentEngine.ts` | 753 | ~58 |
| `app/src/lib/intelligenceRouting.ts` | 565 | ~45 |
| `app/src/lib/situationGoals.ts` | 220 | ~40 |
| `app/src/lib/intentGrammar.ts` | 363 | ~38 |
| `app/src/lib/capabilityRegistry.ts` | 942 | ~33 |
| `app/src/lib/goalResolution.ts` | 324 | ~24 |
| `app/src/lib/workspaceContext.ts` | 462 | ~22 |
| **Intent layer total** | **~6,800** | **~450** |

`resolveIntentCore` contains roughly twenty-five ordered resolver branches, each a guard clause returning an action. Beneath them sit static lookup tables — `SEMANTIC_ALIASES` and `SITE_ALIASES` (private to `intentBridge.ts`), `KNOWN_ENTITIES` (16 catalogue entries in `semanticIntentEngine.ts`), and `CAPABILITY_GRAPH` (16 nodes). One genuinely structured component exists: `parseDesktopIntent` in `intentGrammar.ts` produces a slot-filled `DesktopIntent` struct, which is the right shape and is confined to one stage of one resolver.

The Owner's instruction "intelligence should not be a giant if/else tree" describes a condition that already exists. `resolveIntentCore` is a single function of ordered guard clauses; adding a capability means adding another branch at the correct position relative to every branch already present.

### 0.3 The ordering inversion

`resolveIntelligenceRoute` — the function that decides whether an utterance is a question, a calculation, a conversation, or desktop work — is invoked at `intentBridge.ts:2224`, near the end of `resolveIntentCore`, after roughly six hundred lines of desktop-verb matching:

```2223:2228:app/src/lib/intentBridge.ts
  // P22.S1 — intelligence kind before desktop soft-miss refusal.
  const intelligence = resolveIntelligenceRoute(raw.trim()) ??
    resolveIntelligenceRoute(matchText);
  if (intelligence) {
    return intelligence;
  }
```

Understanding therefore runs *after* action-matching has already failed. Workspace asks "is this a desktop command?" many times before it asks "what does the Owner want?". This single ordering fact explains most of the Owner's observations.

### 0.4 Traced root causes for the Owner's cases

Case 1 was confirmed by executing the resolver against the live code rather than by reading it: `classifyIntelligenceKind("What is the time in Queensland, Australia?")` returns `REASONING_PROVIDER`, and `resolveIntelligenceRoute` returns a `browserOpen` to `https://chatgpt.com/?q=What%20is%20the%20time%20in%20Queensland%2C%20Australia%3F`. The control case `"what time is it in Brisbane"` returns `REASONING_LOCAL` and answers locally.

**Case 1 — "What is the time in Queensland, Australia?" → opens ChatGPT.** `tryLocalDateTime` consults `TIMEZONE_ALIASES` (`intelligenceRouting.ts:258`), which contains `brisbane|qld` but not `queensland`. The string "queensland" does not appear anywhere in `app/src`. Local answering therefore returns `null`, and control reaches `isReasoningProviderAsk`, whose final clause is:

```382:390:app/src/lib/intelligenceRouting.ts
  if (
    /\b(who|what|where|when|why|how)\b/i.test(text) &&
    text.length >= 12 &&
    !/\b(window|screenshot|clipboard|notification|open|launch|focus|minimi|maximi|snap|beside)\b/i.test(
      text,
    )
  ) {
    return true;
  }
```

Any interrogative longer than twelve characters that avoids a short blocklist becomes `REASONING_PROVIDER`, which opens `https://chatgpt.com/?q=…`. The bug is not a missing timezone row. The bug is that *absence of a local answer is treated as authorisation to act on the desktop*.

**Case 2 — "Can you show me where Queensland, Australia is?"** takes the identical path and produces the identical outcome. Two requests with different desired outcomes — *know a fact* versus *see a place* — are indistinguishable to the current router. There is no representation in the system in which these two utterances differ.

**"How much is 3 tonnes at $87 per 200kg?"** — a pure arithmetic question from the Owner's own benchmark — also reaches `REASONING_PROVIDER`. `tryLocalArithmetic` strips only `what's|what is|calculate|compute`, so the phrase survives with units attached, fails the numeric-only guard, and falls through. Local computation is spot-coded to a handful of phrasings rather than modelled as a capability.

**Case 5 — "Hey, how are you?"** never reaches intelligence routing at all. A greeting guard at `intentBridge.ts:1656` matches the `hey` prefix and returns a fixed string, `"Hi — I'm here with you on the desktop."` (`conversationGuidance.ts:75`). The question inside the greeting is discarded. The Owner-reported "Here when you need the desktop" is the sibling farewell reply at `intelligenceRouting.ts:74`.

### 0.5 Planning is retrospective, not generative

`buildExecutionPlan` (`executionPlanner.ts:53`) is a `switch (action.kind)`. The plan is derived *from the action already chosen*, and its steps are descriptive strings. Nothing in the system ever selects a capability because a goal required it.

It is worse than retrospective: it is discarded. `buildExecutionPlan` is called from `goalResolution.ts` during candidate ranking, but `applyGoalResolution` returns only `.action` — the selected plan is dropped, and `handleOperatorUtterance` never reads a plan. Plans survive only into `intentPipeline.ts`, which is an evidence recorder for tests and verifiers and is not on the production path. `productCognition.ts` is likewise test-only. So the repository contains a planner, a pipeline, and a cognition assessor, none of which influence what happens when the Owner speaks.

### 0.6 Capability metadata is prose, not contract

`CapabilityNode` (`capabilityRegistry.ts:11`) declares 24 fields per capability — `purpose`, `verbs`, `aliases`, `objects`, `modifiers`, `arguments`, `requirements`, `limitations`, `examples`, `failureRecovery`, `related`, `similar`, `alternatives`, `discoverability`, `documentation`, `ownership`, `permissions`, `dependencies`, `benchmark`, `tests`, `architecturalJustification`, and others. This is a genuinely rich registry, and it is the right instinct. But every field is natural-language text intended for generating Owner-facing discovery copy. No field states what a capability *consumes* or *produces* in a form another component could match against. `related` / `similar` / `alternatives` are hand-authored identifier lists, which is O(n²) manual maintenance and cannot support "I need X, which capabilities produce X?".

### 0.7 A finding that must be verified separately

`KernelOperator::execute_turn` (`packages/kernel/src/operator/mod.rs:71`) has **no callers anywhere in the repository**. A repository-wide search for `KernelOperator` returns the type definition, nine `KernelOperator::plan` uses inside that file's own unit tests, and one re-export in `lib.rs`. The production IPC path is `commands/capability_intent.rs`, which handles exactly two composition identifiers — `browser.open_beside` and `app.open_or_focus` — and sends everything else through a generic sequential step loop.

The compositions `desktop.open_compound`, `desktop.prepare_coding_workspace`, and `screenshots.capture_and_copy` are implemented only inside `execute_turn_inner`. If that reading is correct, the C-PROC-002 preflight and wait/identity-confirmation stages do not execute on the Owner's machine, and the verifiers that assert their presence pass by inspecting source text rather than behaviour.

**This finding is reported, not acted on.** It is a runtime-correctness question outside this document's remit, it requires measured evidence under Evidence Before Modification, and this task prohibits code changes. It is recorded here because it is the sharpest available illustration of §15: a composition can be *verified* and still not be *real*.

---

## 1. Executive definition of Workspace Intelligence

The Intelligence Layer is the part of Workspace that converts an Owner utterance into a **commitment**: a statement of what will be true when the interaction ends, what Workspace is authorised to do to make it true, and how Workspace will know whether it succeeded.

It is not a parser, a router, or a dispatcher. Parsing, routing, and dispatching are consequences of a commitment, not substitutes for one. A system that routes without committing can execute a step correctly and still fail the Owner completely — which is precisely what "open ChatGPT" does when the Owner asked what time it is.

Three responsibilities, and only three:

1. **Comprehend** — build one structured representation of what the Owner wants, before considering what Workspace can do.
2. **Commit** — bind that representation to an outcome, an authorisation envelope, and a completion test.
3. **Pursue** — drive bounded execution toward the outcome, observing results, adapting inside the envelope, and stopping honestly at its edge.

Everything else in this document elaborates those three words.

---

## 2. Behavioral identity

The proposed wording in the brief was:

> A conversational intelligence that understands the Owner's goals and can use the computer as an instrument to accomplish bounded tasks.

This is close and it is worth adopting in substance, but it has a defect worth correcting. "Use the computer as an instrument" describes capability, not restraint, and restraint is the harder half of this product. The failure mode Workspace actually exhibits is *over-instrumentation*: reaching for the desktop when the desktop was not asked for.

**Proposed identity:**

> Workspace is a conversational intelligence that holds the Owner's goal, answers from what it truthfully knows, and operates the computer only as far as that goal requires.

The three clauses map to the three failure modes observed in this repository: forgetting the goal after one step (Cases 3, 4, 7), substituting action for knowledge (Cases 1, 2), and acting beyond what was asked.

| | |
| --- | --- |
| **What it is** | A single operator that understands goals and can act on this machine |
| **What it is not** | A command interpreter, a keyword router, a browser launcher, an autonomous agent, a chat model with a desktop attached |
| **Optimises for** | The Owner's goal reaching a true, verified state with the fewest side effects |
| **Success** | The Owner stops thinking about how Workspace works |
| **Acts when** | The goal requires machine state to change, or the Owner asked to perceive something only the machine can show |
| **Answers when** | The goal is a change in the Owner's knowledge and Workspace holds a truthful source |
| **Asks when** | Two readings of the request would produce materially different effects, and the wrong one is not cheaply reversible |
| **Stops when** | The outcome is verified, the authorisation envelope is reached, the budget is spent, or truth can no longer be established |
| **Uses context to** | Resolve reference, avoid redundant work, and maintain a goal across turns — never to expand authority |
| **Handles uncertainty by** | Naming the uncertainty in terms of the goal, never by guessing an effect |
| **Handles capability gaps by** | Reporting the boundary at the point in the goal where it occurs |
| **Handles authorisation by** | Treating every effect as separately authorised, never inheriting breadth from a narrow request |
| **Communicates failure by** | Stating what became true, what did not, and what is now different |
| **Maintains continuity by** | Keeping the goal, not the transcript |

---

## 3. Core principles

**P1 — Outcome before mechanism.** The first question is never "which capability matches?" but "what does the Owner want to be true?". Mechanism is selected to serve a decided outcome.

**P2 — The Substitution Prohibition.** Workspace must never substitute a desktop effect for a missing answer. Inability to answer is a reason to say so; it is never authorisation to open something. This is the single rule that resolves Case 1, and it is stated as a prohibition because it must survive every future capability addition.

P2 is not new law. Specification v2 defines the transformation chain `Signal → Meaning → Plan → [Authority] → Effect → Fact → Experience`, assigns Meaning to the Understanding owner, and states that Understanding **must not produce Effect**. It then lists `Meaning → Effect (direct)` among the forbidden transitions. Case 1 is precisely that forbidden transition: a comprehension outcome ("I have no local answer") producing a desktop Effect with no intervening Plan or Authority. P2 is the behavioural restatement of a rule the Specification already carries, which is the strongest possible footing for it.

**P3 — Truth is a constraint, not a goal.** Truth cannot be traded against helpfulness, completeness, or conversational smoothness. A refusal is a valid outcome; a fabricated success is not an outcome at all.

**P4 — Authorisation is narrow by default and never inherited.** "Open this" does not authorise "find and open the next one too". Standing authority is never inferred from a single request.

**P5 — Acceptance is not completion.** A dispatched action is an event. A completed goal is a verified state. The system must be structurally unable to confuse them.

**P6 — Minimum sufficient effect.** Among plans that achieve the outcome, prefer the one with fewest effects, least irreversibility, and least disruption to the Owner's focus.

**P7 — The goal outlives the step.** A multi-step request is one commitment, not a sequence of unrelated commitments. Execution continues while the goal is unmet, authorised, and within budget.

**P8 — Capabilities declare, the layer decides.** New capability must become usable through declaration. If making a capability reachable requires editing a matching ladder, the architecture has failed.

**P9 — Limits are expressed in the Owner's goal.** "I can't work with files" is a statement about Workspace. "I can open the Pictures folder, but I can't read what's inside it, so I can't count the screenshots" is a statement about the Owner's goal. Only the second is acceptable.

**P10 — Comprehension may be uncertain; authority may not.** Understanding is allowed to be probabilistic. Deciding, permitting, executing, and declaring completion are not. This principle is load-bearing for §17 and is in tension with a current governance rule; see §30.

---

## 4. Behavioral priority hierarchy

The candidate ordering in the brief is close but mixes two different kinds of rule. "Preserve truth" and "preserve authorisation" are not objectives to be balanced — they are constraints that are never traded. "Understand the goal" and "complete the goal" are objectives. Flattening them into one list invites a future engineer to trade truth for completion because completion sits only two places lower.

**Constraints — inviolable, in order. A lower constraint never justifies breaching a higher one.**

1. **Do not create a false belief in the Owner.** No invented answers, no claimed completion without verification, no success language for partial results.
2. **Do not exceed what the Owner authorised.** No effect outside the envelope, no standing authority from a narrow request, no irreversible act inside a reversible request.
3. **Do not damage recoverability.** Prefer reversible means; never destroy state the Owner did not ask to have destroyed.

**Objectives — pursued in order, subject to all constraints.**

4. Understand the Owner's actual goal, not the surface form of the sentence.
5. Complete the goal rather than executing its first step.
6. Verify before reporting.
7. Use available context to reduce what the Owner must repeat.
8. Select the minimum sufficient capability set.
9. Preserve the Owner's attention — do not steal focus, open surfaces, or change layout as a side effect.
10. Communicate naturally and in the Owner's terms.
11. Ask for clarification only when a wrong reading would cause a materially different, non-trivially reversible effect.

Two changes from the candidate list are deliberate. Truth and authorisation were promoted out of the objective list into constraints, because an ordering in which they can be outranked is an ordering in which they will eventually be outranked. Preservation of the Owner's attention was added, because focus theft is the most common way a technically correct desktop action becomes a bad experience — and Product Gravity already requires it.

---

## 5. Canonical request understanding pipeline

### 5.1 Assessment of the proposed nine-level model

The brief proposes LEVEL 0 through LEVEL 8 as a waterfall. The brief also asks that this not be accepted merely because it was proposed. It should not be, in that form. It is substantially right about *what must be determined* and wrong about *when and in what shape*.

Three specific objections.

**Objection 1 — a waterfall of classifiers becomes nine if/else trees instead of one.** If each level is a stage that emits a label consumed by the next, each stage acquires its own matching ladder. That is the current architecture with more layers: `intentBridge` (L1-ish), `semanticIntentEngine` (L3-ish), `intelligenceRouting` (L1/L7-ish), `goalResolution` and `situationGoals` (L2-ish) already exist as separate ladders, and their interaction order is now itself a source of defects — which is exactly why intelligence routing ended up at line 2224.

**Objection 2 — the levels are not sequential, they are joint.** Consider "Show me where Queensland is." Communicative intent (L1) cannot be settled without the desired outcome (L2), because the illocution is a question but the outcome is perceptual. Required capabilities (L4) cannot be settled without context (L5), because whether a browser must be opened depends on whether one is already open. These are *dimensions of one interpretation*, resolved together and mutually constrained, not a conveyor belt.

**Objection 3 — the model terminates where the hard part begins.** It ends at LEVEL 8, COMPLETION CONTRACT. But Case 3 is precisely the situation in which the required capabilities are *unknowable in advance*: whether Explorer must be opened depends on observing whether it is open. A strictly hierarchical pass cannot express "the next step depends on the result of the previous step", which the brief itself identifies as the essential property. Levels 4 and 7 must be re-entered after every observation.

Additionally, two levels are placed wrongly. **Authorisation cannot be level 6**, evaluated once between planning and execution; it must be a continuous predicate checked at every effect, because bounded adaptation (§23) changes what is being attempted mid-goal. And **the completion contract cannot be level 8**, decided last; it must be authored *before* the first effect, because a contract written after execution is a description of what happened, which is the exact mechanism by which "action accepted" becomes "goal completed".

### 5.2 The proposed model — three phases, one frame, one loop

```text
        ┌──────────────────────────────────────────────┐
        │  PHASE A — COMPREHENSION      (no effects)   │
        │  utterance + context ──► REQUEST FRAME       │
        │  one structured object, all dimensions       │
        │  resolved jointly                            │
        └───────────────────┬──────────────────────────┘
                            │
        ┌───────────────────▼──────────────────────────┐
        │  PHASE B — COMMITMENT         (no effects)   │
        │  REQUEST FRAME ──► GOAL CONTRACT             │
        │  outcome predicate · authorisation envelope  │
        │  budget · completion test · answer source    │
        │  ── may exit here: ANSWER / CLARIFY / DECLINE│
        └───────────────────┬──────────────────────────┘
                            │
        ┌───────────────────▼──────────────────────────┐
        │  PHASE C — BOUNDED PURSUIT    (effects)      │
        │                                              │
        │    ┌──► OBSERVE  (what is true now?)         │
        │    │       │                                 │
        │    │       ▼                                 │
        │    │    SELECT   (which capability closes    │
        │    │              the gap to the outcome?)   │
        │    │       │                                 │
        │    │       ▼                                 │
        │    │    AUTHORISE (inside the envelope?)     │
        │    │       │                                 │
        │    │       ▼                                 │
        │    │    ACT      (one bounded effect)        │
        │    │       │                                 │
        │    │       ▼                                 │
        │    │    VERIFY   (did it become true?)       │
        │    │       │                                 │
        │    └───────┴──► outcome met? budget left?    │
        │                 envelope intact?             │
        └───────────────────┬──────────────────────────┘
                            │
                     REPORT (truthful, goal-shaped)
```

The Owner's LEVEL 0–8 are preserved — as **fields of the Request Frame and Goal Contract**, not as stages. Nothing in the brief's analysis is discarded; it is re-seated so that it cannot fragment into nine ladders.

| Brief's level | Becomes | Where |
| --- | --- | --- |
| L0 Raw input | `utterance`, `modality`, `normalised` | Frame |
| L1 Communicative intent | `illocution` | Frame — derived, not primary |
| L2 Desired outcome | `outcome` | Frame — **primary key** |
| L3 Domain / object | `subject` | Frame |
| L4 Required capabilities | Re-derived every loop iteration | Phase C, not Frame |
| L5 Context | `references`, resolved during Frame construction | Frame |
| L6 Authorization | `envelope` — a continuous predicate | Contract, checked every effect |
| L7 Execution mode | `mode` | Contract |
| L8 Completion contract | `completionTest` — authored **before** acting | Contract |

**The single most important reordering:** desired outcome (L2) is promoted above communicative intent (L1) as the primary discriminator. Case 1 and Case 2 have the same illocution — both are questions — and differ only in outcome. A model keyed on illocution cannot separate them; a model keyed on outcome separates them immediately. This is not a refinement of the brief's model. It inverts its first two levels, and it is the reason Cases 1 and 2 currently produce identical behaviour.

---

## 6. Intent taxonomy

Illocution is a Frame *field*, not a routing decision. It informs tone and clarification strategy; it does not select capabilities.

| Illocution | Recognised by | Note |
| --- | --- | --- |
| `converse` | Social move with no informational or state demand | May carry a real question ("how are you") — must not be swallowed by a greeting prefix |
| `ask_fact` | Requests a proposition | Routes by answer source, never by desktop capability |
| `ask_computation` | Requests a derived value | Must be attempted locally before any external source |
| `ask_explanation` | Requests understanding, not a datum | Longer-form answer; may legitimately exceed local knowledge |
| `ask_observation` | Requests a fact about *this machine* | Only Workspace can answer; never external |
| `ask_self` | Requests Workspace's own capabilities or state | Answered from the capability graph and session state |
| `direct_action` | Requests machine state change | The only illocution that presumes effects |
| `direct_goal` | Requests an end state reached by unspecified means | Triggers bounded pursuit |
| `continue` | Extends the live goal | Requires a live goal to attach to |
| `correct` | Replaces part of the live goal | "Actually, use Chrome instead" — must revise, not restart |
| `clarify_response` | Answers Workspace's own question | Must consume the pending question, never re-ask |
| `meta` | About the interaction itself | "Stop", "never mind", "what are you doing" |

Two of these are absent from the current implementation in any first-class form. `correct` is handled only where a specific branch happens to anticipate it, and `clarify_response` exists as `unresolvedClarification` in `workspaceContext.ts` but is not a routing concept. Both are required by the Owner's benchmark.

---

## 7. Desired-outcome taxonomy

The primary key. Every Request Frame carries exactly one outcome; hybrids carry an ordered outcome *chain* (§13).

| Outcome | The Owner wants… | Locus of change | Desktop mutation |
| --- | --- | --- | --- |
| `KNOW` | to hold a fact | Owner's knowledge | Prohibited |
| `UNDERSTAND` | to grasp a concept | Owner's knowledge | Prohibited |
| `COMPUTE` | a derived value | Owner's knowledge | Prohibited |
| `PERCEIVE_MACHINE` | to know something about this machine | Owner's knowledge | Read-only observation |
| `PERCEIVE_WORLD` | to see something the machine can display | Owner's knowledge, via the screen | Permitted as *medium* |
| `REACH_STATE` | the machine to be in a state | Machine | Permitted as *end* |
| `PRODUCE_ARTIFACT` | a thing to exist | Machine | Permitted as *end* |
| `MAINTAIN_STATE` | a condition held over time | Machine | Permitted, bounded |
| `SOCIAL` | acknowledgement | Neither | Prohibited |
| `META` | to control the interaction | Interaction | Prohibited |

The column that does the work is **locus of change**. It is the formal expression of P2: if the locus is the Owner's knowledge, a desktop effect can only ever be a *medium* or a *means*, never a *response*.

Case 1 is `KNOW`. Case 2 is `PERCEIVE_WORLD`. Same subject, same illocution, different outcome, different permitted effects. That distinction is currently unrepresentable.

---

## 8. Domain taxonomy

The subject a request concerns. Its function is to constrain capability selection and to make capability gaps expressible in the Owner's language.

```text
KNOWLEDGE      world facts · concepts · language · calculation
TIME           clock · date · timezone · duration
MACHINE        this PC · resources · settings · power
DESKTOP        windows · layout · monitors · focus · z-order
APPLICATION    installed apps · processes · launch surfaces
WEB            browser · sites · pages · navigation
CONTENT        files · folders · documents · images · media
INTERACTION    controls · clicks · text entry · UI elements
TRANSFER       clipboard · notifications · capture
SESSION        the current goal · what was just done · Moments
WORKSPACE      Workspace's own capabilities, limits, and state
```

A gap is reported as *domain × outcome*, which is what makes P9 mechanical rather than aspirational. Case 3 fails at `CONTENT` × `PERCEIVE_MACHINE` — Workspace can reach the folder (`DESKTOP`/`APPLICATION` succeed) but cannot read its contents. That yields the Owner's preferred phrasing directly: *"I can open the Pictures folder, but I can't inspect the files inside it yet, so I can't reliably count the screenshots."* The sentence is generated from the point of failure, not chosen from a table of apologies.

---

## 9. Context model

Context must be split by *lifetime and trust*, because the current single `WorkspaceContextState` blends things with very different validity.

**Tier 1 — Utterance context.** Modality, timestamp, raw and normalised text. Lifetime: one turn.

**Tier 2 — Conversational context.** The live goal, the last completed goal, pending clarification, referents introduced by Workspace or the Owner, the correction history. Lifetime: session. This is roughly what `workspaceContext.ts` holds today (`currentObjective`, `lastAction`, `lastWindowQuery`, `lastAppQuery`, `windowsReferenced`, `appsLaunched`, `unresolvedClarification`), and that module is well-scoped. Its documented boundary — session continuity and referent binding only — should be preserved.

**Tier 3 — Machine context.** Observed desktop state with an explicit observation timestamp. Lifetime: until invalidated. Detailed in §18.

**Tier 4 — Capability context.** What Workspace can currently do, including capabilities disabled by unmet preconditions or ungranted permissions. Lifetime: process.

### Reference resolution

Every referring expression resolves against a **referent stack** carrying, for each entry: the referent, how it entered the conversation (Owner-named, Workspace-opened, Workspace-observed), the turn index, and whether it is still valid.

| Expression | Resolves to |
| --- | --- |
| "it" / "that" | Most recent salient referent of compatible type |
| "the one I just opened" | Most recent referent with provenance `workspace-opened` |
| "this window" | Active window at utterance time — Tier 3, must be re-observed |
| "the latest" | Requires a domain and an ordering; unresolvable without both |
| "again" | Re-runs the last **goal**, not the last action |
| "there" | Most recent location referent |

Two rules govern failure. **Ambiguity is not resolved by recency alone** when the candidates would produce different effects — that is a clarification (§21). And **a stale referent is not silently refreshed**: if "that window" no longer exists, Workspace says so rather than picking a different window.

Note that "again" resolving to the last *goal* rather than the last *action* is not achievable today, because no goal survives the turn (§0.1).

---

## 10. Authorization model

Authorisation is a **continuous predicate over effects**, not a gate passed once.

### The envelope

Each Goal Contract carries an envelope with four dimensions:

| Dimension | Meaning |
| --- | --- |
| **Effect classes** | Which kinds of effect are permitted: `observe`, `navigate`, `launch`, `arrange`, `input`, `mutate-content`, `destroy` |
| **Subjects** | Which named objects may be affected |
| **Breadth** | `single` · `enumerated-set` · `owner-confirmed-set`. Never `unbounded` |
| **Duration** | This turn only, unless the Owner asked for a maintained state |

Every effect is checked against the envelope at the moment of execution. An effect outside the envelope stops the goal and returns to the Owner; it never triggers a silent broadening.

### The escalation ladder

The brief's four phrasings map to strictly increasing authority, and each rung must be earned explicitly:

| Utterance | Authorises |
| --- | --- |
| "What is this?" | Observation only. No effect. |
| "Open this." | One `launch`/`navigate` effect on one resolved subject. |
| "Find this and open it." | Observation to resolve the subject, then one effect on the *unique* result. Multiple results ⇒ clarification, not first-match. |
| "Keep doing this until…" | A maintained state with an explicit termination predicate, a budget, and an interrupt. Requires confirmation. |

The third rung deserves emphasis because it is where systems quietly fail. "Find and open it" authorises acting on *the* thing, singular. Acting on the first of several matches is the system inventing a decision the Owner did not delegate. The C-PROC-002 work already reached this conclusion independently — `confirm_prepare_target_identity` requires either a window handle or a unique match and explicitly rejects first-substring-match — and that rule should be generalised rather than left as one procedure's local discipline.

### Non-inference rules

- Authority never widens through composition. A plan of three authorised steps does not authorise a fourth.
- Authority never widens through failure. A failed step does not authorise trying a more invasive alternative.
- Authority never persists. Consent to an effect this turn is not consent next turn.
- Reversibility is not authority. That an effect is undoable does not make it permitted.

### Relationship to the Kernel

This model is a *pre-filter*, not a replacement for enforcement. The Permission Gateway in the Kernel command pipeline remains the sole enforcement point. The envelope prevents Workspace from *attempting* what the Owner did not ask for; the Gateway prevents the system from *performing* what is not permitted. Both are required, and the Intelligence Layer must never treat a Gateway grant as evidence that the Owner wanted the effect.

---

## 11. Execution-mode model

The mode is decided in Phase B and recorded on the Goal Contract.

| Mode | Effects | Exit |
| --- | --- | --- |
| `ANSWER` | None | Reply from a truthful source |
| `OBSERVE` | Read-only | Reply describing machine state |
| `ACT_SINGLE` | One bounded effect | Verified state, then report |
| `PURSUE` | Multiple, dependent | Completion test satisfied, or bounded stop |
| `SHOW` | Effects as medium | Owner can perceive the target |
| `CLARIFY` | None | One question, one pending slot |
| `DECLINE` | None | Honest boundary in goal terms |
| `HANDOFF` | One navigation, with consent | Owner is at the surface they asked for |

`HANDOFF` is separated from `ACT_SINGLE` deliberately. Handing the Owner's question to a third party is a distinct act with distinct consent requirements, and treating it as an ordinary browser open is how Case 1 became possible.

---

## 12. Capability metadata contract

### The requirement

For a capability to become usable *by declaration*, the Intelligence Layer must be able to answer three questions mechanically: *Which capabilities can produce X? What must be true before this one can run? How will I know it worked?* None of the 24 fields on `CapabilityNode` answers any of the three.

### Minimum canonical schema

The existing registry is not replaced. It is **augmented with a machine-readable facet**, keyed by the same `id`, so that Owner-facing prose and planner-facing contract stay in one place without collapsing into each other.

| Field | Type | Purpose |
| --- | --- | --- |
| `id` | identifier | Joins to the existing `CapabilityNode` and Atlas record |
| `effectClass` | `informational` \| `observational` \| `navigational` \| `computational` \| `mutating` \| `conversational` \| `external` | Gates against outcome locus (§7) — the mechanical form of P2 |
| `consumes` | typed slots, each `{ type, required, resolvedBy }` | What must be supplied and how it may be obtained |
| `produces` | typed facts | What enters the goal's fact set on success |
| `preconditions` | predicates over Desktop Context | Machine-checkable; drives *why* a capability is unavailable |
| `postconditions` | predicates | **This is the verification method.** Not a prose description of one |
| `observedBy` | capability ids | Which observation capability can evaluate the postconditions |
| `authorization` | `{ effectClasses[], breadth, requiresConsent }` | Envelope contribution (§10) |
| `reversibility` | `reversible` \| `restorable` \| `irreversible` | Feeds the minimum-sufficient-effect rule (P6) |
| `failureModes` | `[{ signal, class }]` where class ∈ `transient` \| `precondition-unmet` \| `ambiguous` \| `unauthorized` \| `unsupported` | Determines whether adaptation is permitted (§23) |
| `cost` | `instant` \| `fast` \| `slow` \| `disruptive` | Tie-break and focus preservation |
| `stability` | `stable` \| `provisional` | Whether the planner may rely on it |

Twelve fields. Deliberately smaller than the existing 24, because every field here must be machine-consumed, and a field that nothing consumes is documentation.

### Composition edges are derived, not authored

`related`, `similar`, and `alternatives` are hand-maintained lists today. Under this schema, composability is computed: **capability A can feed capability B when A's `produces` satisfies one of B's `consumes` slots.** Alternatives are capabilities producing the same fact type with a different `effectClass` or `cost`. This is what makes P8 real — a newly declared capability becomes reachable the moment its types line up, without editing a matching ladder or a relationship list.

### Explicitly not in the schema

Embeddings, learned rankings, and confidence scores. `VOICE_P16_35_PRODUCT_COGNITION.md` already classified these as Outside/Rejected for deterministic Intent, and nothing in this design needs them: type matching over declared contracts is deterministic and inspectable.

---

## 13. Capability composition model

```text
GOAL CONTRACT
 ├─ outcome            predicate over the world/machine
 ├─ envelope           §10
 ├─ budget             max effects · max wall time · max clarifications
 ├─ completionTest     evaluated from observed facts only
 └─ PLAN  (regenerated after every observation — not fixed at commit)
     ├─ STEP
     │   ├─ capability          selected by produces/consumes matching
     │   ├─ inputs              bound from facts or context
     │   ├─ expectedPostcond    from the capability contract
     │   ├─ verification        the observation that evaluates it
     │   └─ result              → contributes facts
     ├─ STEP  (inputs bound from prior step's produced facts)
     └─ …
```

The plan is a *current best route*, not a script. It is recomputed after each observation, which is what allows Case 3 to work: whether "open File Explorer" is needed at all is a function of an observation that has not yet been made at commit time.

### Worked example — "Find the latest screenshot and open it in Paint"

| Step | Capability requirement | Produces | Verified by |
| --- | --- | --- | --- |
| 1 | Something producing `file[]` filtered to screenshots | candidate set | count > 0 |
| 2 | Ordering by capture time, selecting max | one `file` | selection is unique |
| 3 | Resolving "Paint" to a launchable application | `application` | resolvable |
| 4 | Opening a `file` with an `application` | `window` | window exists |
| 5 | Confirming the window holds the requested file | `identity` | title/handle match |

Note what this exposes. Steps 1 and 2 require a capability that produces `file[]` — and `C-ACT-011 File Provider` is **BLOCKED** in the Atlas. Under this model that is not a mysterious failure: the planner cannot bind step 1, so the goal is declined at commit time with the precise sentence *"I can find and open Paint, but I can't look through your screenshots yet, so I can't pick the latest one."* No effect occurs. Today, by contrast, the request would descend the matching ladder and produce whichever branch happens to catch it.

### Composition authority

Providers never call providers, and each step is a separate Kernel operation rather than a compound instruction handed to a provider. That much is settled by the Capability Composition Rule.

What is *not* settled is which side of the IPC boundary performs the `produces`/`consumes` binding described above. This document sketches it in the Intelligence Layer, and that placement conflicts with the Intent Layer Specification and the Capability Composition Rule. The conflict is set out in §30 as Conflict C, and it should be resolved before any of this is built. The model itself — type-matched binding against declared contracts, replanned after each observation — is unaffected by where it runs.

---

## 14. Goal state model

```text
Goal {
  id
  utterance                originating request
  outcome                  §7
  subject                  §8
  envelope                 §10
  budget                   { effects, wallTime, clarifications }
  facts                    accumulated observations, each with provenance + timestamp
  steps                    attempted, with results
  status                   forming | committed | pursuing | awaiting-owner
                           | satisfied | partially-satisfied | abandoned | declined
  blockingQuestion         set only when awaiting-owner
}
```

**Statuses that matter.** `partially-satisfied` is the honest state for Case 4 when the video plays but fullscreen cannot be verified. A `partial` status does already exist in the Kernel, per composition (§15) — the missing piece is not the concept but its scope: it describes one IPC call, not a goal, so a goal spanning several calls has nowhere to record that it half-succeeded. `awaiting-owner` has no analogue at all today; it makes a clarification a *suspended goal* rather than a terminated turn, which is what allows `clarify_response` to resume rather than restart.

**Lifetime.** A goal survives across turns while `pursuing` or `awaiting-owner`. It is superseded by a new unrelated request, revised by a `correct` illocution, and closed by any terminal status. Exactly one goal is live; Workspace is an operator, not a scheduler.

**Corrections revise, they do not restart.** "Actually, use Chrome instead" mutates the subject binding of the live goal and replans from current facts. Work already completed and still valid is not repeated — that is the practical difference between a companion and a command line.

---

## 15. Completion model

**Completion is a predicate over observed facts. It is never a function of dispatch.**

```text
completed        ⟺ completionTest(facts) = true, where every contributing
                    fact has provenance = observed  (not assumed, not inferred
                    from a successful call)
partial          ⟺ some outcome components verified true, others not
                    → report both halves, name what is now different
failed           ⟺ outcome not achieved and no permitted adaptation remains
declined         ⟺ no plan bindable at commit; zero effects taken
```

Four rules follow, and each closes a specific way systems lie:

1. **A successful call is not a verified fact.** Launch returning success means a request was accepted, not that a window exists.
2. **Absence of error is not evidence.** If verification could not be performed, the outcome is unverified, and unverified is reported as unverified.
3. **Identity must be confirmed, not assumed.** The window that appeared is not necessarily the window requested. This is the generalisation of the C-PROC-002 identity rule.
4. **Partial results are reported as partial, with the state change named.** "I opened Chrome and started the video, but I couldn't confirm it went fullscreen" is a complete and honest report. "Done" is not.

Substantial parts of this already exist, and the existing work is good. `compose_completion` in `packages/kernel/src/operator/compose.rs` emits `completed` / `partial` / `failed` for ten compositions — `browser.open_beside`, `browser.open_foreground`, `window.focus_minimize`, `window.click_control`, `window.type_control`, `window.wait_condition`, `screenshots.capture_and_copy`, `app.open_maximize`, `desktop.open_compound`, and `desktop.prepare_coding_workspace`. The set-valued ones already count verified units against expected (`0 < units_ok < expected` ⇒ `partial`), which is exactly the right semantics. C-VER-003 supplies real bounded wait conditions rather than sleeps.

The gap is not that completion is missing. It is that completion is **per-composition and hand-authored**, so it exists only where an engineer wrote a rule, and it is scoped to one IPC call rather than to the Owner's goal. Under this model completion is per-goal and derived from the postconditions the participating capabilities declared, which means it exists for every goal automatically — including goals whose step sequence nobody anticipated.

And §0.7 is the cautionary case: a completion rule that is verified by source inspection but sits behind an uncalled function is not a completion guarantee. Completion logic must be verified on the path the Owner's utterance actually travels.

---

## 16. Information-vs-action routing

### The canonical rule

> **Locus-of-Change Rule.** Determine where the change must occur for the Owner to be satisfied. If the change is in the Owner's knowledge, a desktop effect is permitted only when the Owner requested the desktop as the medium, or when an effect is the only means to the observation requested. A desktop effect is never permitted as a *response to not knowing*.

The second sentence is the Substitution Prohibition (P2) in operational form, and it is the whole of Case 1.

### Derivation table

| Locus | Source of the change | Route |
| --- | --- | --- |
| Knowledge | Reasoning or computation | `ANSWER` |
| Knowledge | This machine's state | `OBSERVE` |
| Knowledge | External world | `RETRIEVE` (§17) |
| Knowledge | Something the Owner must *see* | `SHOW` — effects permitted as medium |
| Machine | The state itself is the goal | `ACT` |
| Both, ordered | Knowledge gates an effect, or an effect enables knowledge | `HYBRID` |

### The `SHOW` discriminator

`SHOW` is the subtle one and it is where Case 2 lives. The signal is not the word "show" — a keyword test would be exactly the if/else tree the Owner rejected, and it would misfire on "show me what's on my clipboard" (which is `OBSERVE`). The signal is that **the outcome is perceptual and the target is inherently spatial, visual, or continuous** — a location, an image, a video, a page, a rendering. "Show me where Queensland is" is spatial. "Show me my open windows" is enumerable, and therefore `OBSERVE` answered in conversation.

The honest position: this discrimination is a genuine natural-language judgement. It is where §17's reasoning tier earns its place, and it is where a purely deterministic implementation will produce its most defensible-looking wrong answers.

### Benchmark against the rule

| Utterance | Locus | Route |
| --- | --- | --- |
| "What time is it?" | Knowledge / computation | `ANSWER` |
| "What is the capital of Queensland?" | Knowledge / external | `ANSWER` via retrieval |
| "Show me Queensland on a map." | Knowledge / perceptual + spatial | `SHOW` |
| "What windows are open?" | Knowledge / machine | `OBSERVE` |
| "Open Chrome." | Machine | `ACT` |
| "Open Chrome and search for today's weather." | Machine, then knowledge-in-place | `HYBRID` |
| "Find my latest screenshot and tell me what's in it." | Machine → knowledge | `HYBRID` (observe, reason) |
| "Find my latest screenshot and open it in Paint." | Machine → machine | `HYBRID` (observe, act) |

---

## 17. External AI policy

### Assessment of P22.S1

P22.S1 was a reasonable response to a real problem: reasoning requests were hitting desktop refusal copy, and the Owner experienced a command interpreter that could not think. The mechanism chosen — classify the kind, hand broad knowledge to ChatGPT rather than invent an answer — correctly refused to fabricate.

But the implementation has three defects that go beyond tuning.

**It is a fallback, not a route.** `resolveIntelligenceRoute` runs at line 2224. Reasoning is what happens when desktop matching has already failed, so the *shape* of the system says reasoning is the exception. It should be the front door.

**It converts a knowledge gap into a desktop effect.** This is a direct violation of the Locus-of-Change Rule. "I don't know" and "let me open a browser" are different responses, and only one of them was requested.

**It sends the Owner's words to a third party without consent.** `chatgptReasoningUrl` places the raw utterance into a URL query string to `chatgpt.com`. That is a privacy-relevant transfer performed on the basis of a regex matching an interrogative pronoun. For a local-first product whose Specification names Local Trust as an attribute, this warrants explicit Owner consent regardless of the routing argument.

### The Answer Source Ladder

Every knowledge-locus outcome descends this ladder and stops at the first rung that can answer *truthfully*. Descending is not failure — it is the design.

| Rung | Source | Consent | Notes |
| --- | --- | --- | --- |
| 1 | **Deterministic local computation** | None | Time, dates, timezones, arithmetic, units, currency-free ratios. Must be a *real capability with coverage*, not a phrase list |
| 2 | **Machine truth** | None | Observation of this desktop. Only Workspace can supply it |
| 3 | **Workspace self-knowledge** | None | Capabilities, limits, session history, current goal |
| 4 | **Reasoning over rungs 1–3** | None | Combining known facts. Not world knowledge |
| 5 | **External knowledge, returned into Conversation** | Owner policy, once | An answer provider whose result comes *back*. Does not exist today (`C-REA-004`, FUTURE) |
| 6 | **Handoff to a third-party surface** | Per-invocation, explicit | Opening ChatGPT. A distinct act, not a fallback |
| 7 | **Honest stop** | None | "I don't know that, and I don't have a way to find out." Always available, always acceptable |

The reclassification is precise: **P22.S1's ChatGPT handoff belongs at rung 6, and it is currently occupying rung 5.** Rung 5 is empty, so the system skips from rung 4 to rung 6 automatically. That is the defect. With rung 5 empty, the correct behaviour for an unanswerable question is rung 7 plus an *offer* — "I don't have a way to look that up. Want me to open ChatGPT with it?" — which is one sentence away from what exists and materially different in kind.

Opening ChatGPT is right when the Owner asked for ChatGPT (Case 6, "Open GPT" — already covered by the P14 Semantic Alias Rule), or when the Owner accepted the offer. Never otherwise.

### When external AI should be invisible infrastructure

Rung 5, if adopted, is invisible: it returns text into Conversation, Workspace attributes it, and the Owner never leaves. Rung 6 is inherently visible because it relocates the Owner. Conflating them is what makes the current behaviour feel like a browser wrapper. `C-REA-004` is the Atlas entry for rung 5 and is correctly gated behind Evidence Before Commitment; nothing here proposes unblocking it.

### The local computation gap

Rung 1 is currently a list of phrasings, and it fails on ordinary language: "How much is 3 tonnes at $87 per 200kg?" — from the Owner's own benchmark — is pure arithmetic, and it opens ChatGPT. Rung 1 must be a capability with declared coverage over quantities, units, rates, times, and timezones, so that its boundary is a *known* boundary rather than an accident of which prefixes were stripped.

---

## 18. Desktop context model

### Raw observation and semantic understanding are different things

The brief is right to insist on this separation, and it is right that UIA does not supply semantics. A UIA tree is a bag of typed nodes; "the search box" is an interpretation. Conflating them produces confident wrong actions, which are worse than refusals.

**Layer 1 — Observations.** Facts with provenance and timestamps: window list, active window, monitors, control tree for a named window, screenshot, clipboard contents. Every observation carries *when* it was taken. An observation without a timestamp cannot be reasoned about safely.

**Layer 2 — Interpretations.** Derived, always defeasible, never treated as ground truth for an irreversible effect: which window is "the browser", which control is "the search box", which application is "in front", whether a video is "playing".

**Layer 3 — Salience.** What the Owner is likely referring to: recently mentioned, recently opened, recently observed, currently focused.

### Freshness discipline

| Fact | Validity |
| --- | --- |
| Window list | Invalid after any effect; re-observe |
| Active window | Single-effect lifetime |
| Control tree | Invalid after any interaction with that window |
| Screenshot | Point-in-time; never re-used as current state |
| Clipboard | Volatile; read at point of use |

**Rule:** an interpretation may never be carried across an effect. After acting, re-observe. This is the discipline that makes verification meaningful rather than ceremonial.

### What should be continuously maintained

Continuous polling is rejected — it is cost without benefit and the Atlas already excludes indefinite polling. Instead: **observe on demand, cache with timestamps, invalidate on effect.** The only continuously maintained items are the live goal, the referent stack, and the capability availability set, all of which are cheap and none of which require touching the OS.

---

## 19. Conversation behavior

Workspace speaks as one operator with a goal, not as a dispatcher reporting on a subsystem.

**Register.** Report state changes in the Owner's terms. Prefer what is now true over what was executed. Length follows content: an answer is a sentence, a multi-step result is a short paragraph naming what became true and what did not.

**Ordinary conversation is answered, not deflected.** "Hey, how are you?" contains a greeting and a question. Answering the greeting and dropping the question — the current behaviour, since the `hey` prefix guard returns a fixed string — is a small failure that reads as a large one, because it is the Owner's first evidence about whether anything is listening.

**Never.** Command suggestions in the imperative ("Try 'open X'"). Capability catalogues. Meta-commentary about invention ("I won't invent an answer here" — the Owner should never have to hear about the guardrail). Menu-shaped replies. Provider, router, registry, or kernel vocabulary.

**Limits are stated at the point of failure in the goal.** Not "I can't work with files and folders yet", but "I can open the Pictures folder, but I can't read what's inside it, so I can't count the screenshots." §8 makes this generable rather than hand-written.

**Proactivity is bounded by the live goal.** Workspace may volunteer that a step failed, that something needs the Owner, or that a related fact was observed *while pursuing the goal*. It may not volunteer suggestions, capabilities, or observations unrelated to it.

---

## 20. Failure behavior

Every failure is classified before it is spoken, because the class determines both the response and whether adaptation is permitted.

| Class | Meaning | Response | Adaptation |
| --- | --- | --- | --- |
| `transient` | Timing; the same action may succeed | Bounded retry (C-VER-002) then report | Same action only |
| `precondition-unmet` | A required state is absent | Achieve it if inside the envelope, else report | Permitted inside envelope |
| `ambiguous` | Multiple valid subjects | Clarify — never first-match | Not permitted; ask |
| `unauthorized` | Outside the envelope or denied | Stop; explain in goal terms | Never |
| `unsupported` | No capability produces what is needed | Decline at commit, before effects | Never |
| `unverifiable` | Acted, cannot confirm | Report as unverified | Not permitted |

Three obligations. **Never report success for an unverified outcome** — `unverifiable` is a distinct outcome and must survive into the reply. **Never convert a failure into a different goal** — the failure of "count the screenshots" does not authorise "open the Screenshots folder so you can count them" unless the Owner is offered that and accepts. **Always report residual state** — if three of five steps took effect, the Owner needs to know the machine changed.

---

## 21. Ambiguity behavior

Clarify only when a wrong reading would produce a materially different, non-trivially reversible effect. Every avoidable question is a small tax on the product's core promise.

**Resolve without asking when:** context supplies a unique referent; one reading is a no-op; the readings converge on the same effect; or the effect is trivially reversible and the likelier reading is clearly likelier.

**Ask when:** multiple subjects match and the effect is not trivially reversible ("find this and open it" with three matches); the outcome class itself is genuinely undetermined (`KNOW` vs `SHOW` on a spatial subject); or a referent has gone stale.

**Form.** One question, in the Owner's terms, offering the actual candidates. Never a list of commands. `"Did you mean Western Australia or Washington state?"` — which the current implementation already gets right at `intelligenceRouting.ts:302` — is the correct shape.

**Suspension.** Asking sets the goal to `awaiting-owner` with a `blockingQuestion`. The Owner's reply is a `clarify_response` that resumes the same goal with the slot filled. It is never re-parsed from scratch, which is the current behaviour and the reason clarifications feel like starting over.

---

## 22. Multi-step reasoning model

A multi-step goal is **one commitment pursued through a loop**, not a queue of commands.

The distinguishing property, stated in the brief and worth restating precisely: *the next step depends on the result of the previous step.* This makes up-front planning impossible in principle for Case 3, and it is why §5 replaces the waterfall's tail with a loop.

### Case 3 traced — "Open File Explorer, go to Pictures, open Screenshots and count how many pictures are in there"

```text
Outcome        KNOW( count of image files in Pictures/Screenshots )
Locus          Owner's knowledge
Envelope       observe, navigate, launch · subject: Pictures/Screenshots · breadth: single
CompletionTest a verified integer count exists in facts

COMMIT — bind a plan
  need: file[] in a named folder      → capability producing file[]?
                                        NONE (C-ACT-011 BLOCKED)
  ⇒ DECLINE before any effect
```

The result today would be some branch of the matching ladder firing on "open" and File Explorer appearing — an effect the Owner did not receive value from, followed by no count. The result under this model is zero effects and: *"I can open the Screenshots folder for you, but I can't read the files inside it yet, so I can't count the pictures. Want me to open it anyway?"*

That is the whole thesis of this document in one example. **The correct behaviour for a capability gap is to recognise it before acting, and the current architecture cannot, because capability sufficiency is never evaluated against a goal.**

### Case 4 traced — "Open YouTube, search for X, play it, put it fullscreen, make sure volume is on"

```text
Outcome        REACH_STATE( video X playing, fullscreen, audio audible )
Envelope       navigate, launch, input, arrange · subject: browser + YouTube
CompletionTest playing ∧ fullscreen ∧ ¬muted

Loop
  observe browser present?      → launch if not, verify window
  navigate youtube.com          → verify page
  input search query            → requires C-ACT-005 on a resolved control
  select + play                 → requires C-OBS-003/004 to resolve the result
  fullscreen                    → input or arrange
  audio                         → NO capability observes or sets audio
                                  (Atlas: volume is outside current scope)

⇒ PARTIAL: pursue what is bindable, report the rest honestly
```

`partially-satisfied` is doing real work here. Without it, this goal has to be reported as either success (false) or failure (also false, since the video is playing).

### Bounds

Every pursuit carries a step budget, a wall-clock budget, and a clarification budget. Exhaustion is a normal terminal state reported honestly. This is what separates bounded pursuit from the autonomous agent loops the Atlas rejects as `C-INT-005`: the loop is bounded by a contract authored before it started, and it cannot expand its own authority.

---

## 23. Bounded adaptation model

Adaptation is permitted only when **all four** hold:

1. It serves the *same* outcome predicate.
2. It stays inside the *existing* envelope — same effect classes, same subjects, no breadth increase.
3. It stays inside the remaining budget.
4. Its `reversibility` is no worse than the step it replaces.

Permitted: retrying a transient failure within C-VER-002 bounds; achieving an unmet precondition already inside the envelope; choosing an alternative capability that produces the same fact with equal-or-better reversibility; re-observing after a stale interpretation.

The existing bounds are the right model and should be the template rather than the exception. C-VER-002 allows `MAX_INTERACTION_ATTEMPTS = 2` — the original plus one retry — separated by a `400ms` wait that reuses C-VER-003 rather than sleeping, and it classifies statuses into retryable (`control_not_found`, `condition_timeout`, and similar) and explicitly non-retryable (`clarify`, `denied`, `permission_denied`, `unsupported`). C-VER-003 waits are bounded at 2 seconds by default and 8 seconds maximum, polling at 50ms. Its module header states the principle better than this section can: *"Not an agent loop. Retries only the same authorized action."*

Not permitted: a new effect class; a new subject; broadening from one to many; substituting a different outcome because the requested one failed; asking a third party because the local attempt failed.

**Recovery versus escalation.** Recovery is a different *means* to the committed outcome and needs no new consent. Escalation is a change to *what is being attempted* and always returns to the Owner. Almost every unsafe agent behaviour is escalation performed silently under the label of recovery, and the four conditions above exist to make that distinction mechanical rather than judgemental.

---

## 24. Companion identity

Continuity is what makes a companion different from a command line, and it is specifically continuity **of goal**, not of transcript.

- The live goal persists across turns; "again" re-runs the goal, not the last effect.
- Corrections revise the goal in place and preserve completed valid work.
- Clarifications suspend and resume rather than terminate.
- Workspace remembers what *it* did, and distinguishes it from what the Owner did — the provenance field on the referent stack.
- Workspace does not accumulate a long semantic memory of the Owner. `C-INT-004` is REJECTED in the Atlas, and nothing here reopens it. Session-scoped goal continuity is sufficient for every case in the benchmark.

The companion register: present, not eager; concise, not terse; honest without narrating its own guardrails.

---

## 25. Jarvis behavioural benchmark

Not a feature list. A behavioural standard: **a request from this set should be handled correctly without the Owner knowing anything about how Workspace is built.** Correctly includes declining well.

The benchmark is met when every row produces either the outcome or an honest, goal-shaped boundary — and when no row produces an effect the Owner did not ask for. By that measure, the rows that currently fail do so mostly by *acting* rather than by refusing.

| # | Request | Illocution | Outcome | Mode | Completion condition |
| --- | --- | --- | --- | --- | --- |
| 1 | "Open Chrome." | `direct_action` | `REACH_STATE` | `ACT_SINGLE` | Chrome window exists and is identified |
| 2 | "Open Chrome and search for YouTube." | `direct_goal` | `REACH_STATE` | `PURSUE` | Results page for the query is displayed |
| 3 | "Open YouTube and play this video." | `direct_goal` | `REACH_STATE` | `PURSUE` | Referent bound; playback observed |
| 4 | "Find my latest screenshot." | `ask_observation` | `PERCEIVE_MACHINE` | `OBSERVE` | One file identified and named |
| 5 | "…and tell me what it shows." | `direct_goal` | `KNOW` | `PURSUE` | Description produced from real perception |
| 6 | "…and open it in Paint." | `direct_goal` | `REACH_STATE` | `PURSUE` | Paint window confirmed holding that file |
| 7 | "Open Pictures, find Screenshots, count the images." | `direct_goal` | `KNOW` | `PURSUE` | Verified integer count |
| 8 | "Open Chrome beside Notepad." | `direct_action` | `REACH_STATE` | `PURSUE` | Both windows placed and verified |
| 9 | "Prepare my coding workspace." | `direct_goal` | `REACH_STATE` | `PURSUE` | Each target confirmed by identity |
| 10 | "What's the time?" | `ask_fact` | `KNOW` | `ANSWER` | Correct local time stated |
| 11 | "How much is 3 tonnes at $87 per 200kg?" | `ask_computation` | `COMPUTE` | `ANSWER` | Correct value, no desktop effect |
| 12 | "How's my day going?" | `converse` / `ask_self` | `SOCIAL` | `ANSWER` | Acknowledged; optionally grounded in session |
| 13 | "What windows do I have open?" | `ask_observation` | `PERCEIVE_MACHINE` | `OBSERVE` | Current window list named |
| 14 | "Open that one." | `direct_action` | `REACH_STATE` | `ACT_SINGLE` | Referent bound uniquely, or clarify |
| 15 | "Close the one I just opened." | `direct_action` | `REACH_STATE` | `ACT_SINGLE` | Provenance-matched referent closed |
| 16 | "Actually, use Chrome instead." | `correct` | inherits | inherits | Live goal revised, not restarted |

**Context and capability requirements for the harder rows.**

| # | Context required | Capability requirement | Acceptable failure |
| --- | --- | --- | --- |
| 3 | Referent for "this video" | Page observation + interaction | Clarify which video; never guess |
| 5 | Selected file | Content perception (`C-OBS-007` OCR, FUTURE) | Decline: can find it, cannot see inside it |
| 6 | Selected file | File enumeration (`C-ACT-011`, BLOCKED) + open-with | Decline before effects |
| 7 | Folder path | File enumeration (BLOCKED) | Decline with the folder-level offer |
| 9 | Target set | Launchability preflight, wait, identity — C-PROC-002 | Report per-target truthfully |
| 11 | None | Rung-1 computation with unit/rate coverage | Honest stop — **never** a ChatGPT handoff |
| 12 | Session facts | Self-knowledge | Plain acknowledgement — **never** a ChatGPT handoff |
| 14 | Referent stack | Any | Clarify on ambiguity or staleness |
| 16 | Live goal | Any | Ask what to switch if no live goal |

Rows 11 and 12 currently open ChatGPT (§0.4). They are the cheapest available proof that the routing defect is structural rather than incidental.

---

## 26. Example request classifications — the Owner's cases

**Case 1 — "What is the time in Queensland, Australia?"**
`ask_fact` · `KNOW` · TIME · locus: knowledge · rung 1 · mode `ANSWER`.
Completion: correct time stated. Desktop effects: prohibited. Correct reply: the Brisbane time. If timezone coverage were absent, rung 7 — *"I can't work that timezone out reliably"* — not a browser.
*Current: opens ChatGPT.*

**Case 2 — "Can you show me where Queensland, Australia is?"**
`ask_fact` · `PERCEIVE_WORLD` · KNOWLEDGE + WEB · locus: knowledge via visual medium · mode `SHOW`.
Effects permitted as medium: opening a map is serving the request, not substituting for it. Completion: the location is visible.
*Current: opens ChatGPT — indistinguishable from Case 1.*

**Case 3 — "Open File Explorer, go to Pictures, open Screenshots and count how many pictures are in there."**
`direct_goal` · `KNOW` · CONTENT · mode `PURSUE`.
Declines at commit; no capability produces `file[]`. Zero effects. Reply names the boundary at the point it occurs and offers what *is* possible.
*Current: no goal exists; the ladder resolves the first verb it recognises.*

**Case 4 — "Open YouTube, search for X, play it, put it fullscreen and make sure the volume is on."**
`direct_goal` · `REACH_STATE` · WEB + INTERACTION · mode `PURSUE`.
Conjunctive completion test. Terminates `partially-satisfied` when audio state cannot be observed. Reports which conjuncts hold.
*Current: no conjunctive completion exists.*

**Case 5 — "Hey, how are you?"**
`converse` · `SOCIAL` · locus: neither · mode `ANSWER`.
The embedded question is answered. No effects, no capability list, no deflection.
*Current: fixed greeting; the question is discarded.*

**Case 6 — "Open GPT."**
`direct_action` · `REACH_STATE` · APPLICATION/WEB · mode `ACT_SINGLE`.
"GPT" resolves through the deterministic alias table the P14 Semantic Alias Rule already mandates. Here, and only here, opening ChatGPT is exactly what was asked for.
*Current: handled correctly.*

**Case 7 — "Take the latest screenshot and open it in Paint."**
`direct_goal` · `REACH_STATE` · CONTENT + APPLICATION · mode `PURSUE`.
Observe → select unique → resolve Paint → open-with → confirm identity. Ambiguity at selection is a clarification, not a first-match. Blocked today at enumeration; declines before effects.
*Current: no observe-then-act composition exists.*

---

## 27. Architecture interaction diagram

```text
              ┌───────────────────────────────────────────┐
              │  CONVERSATION  (L1)                       │
              │  input · presentation · voice             │
              │  Presentation Purity: no orchestration    │
              └──────────────┬────────────────────────────┘
                             │ utterance
              ┌──────────────▼────────────────────────────┐
              │  INTELLIGENCE LAYER                       │
              │                                           │
              │  ┌─ COMPREHENSION ──────────────────────┐ │
              │  │ normalise · resolve reference ·      │ │
              │  │ classify outcome/subject/illocution  │ │◄── Referent stack
              │  │ ⇒ REQUEST FRAME                      │ │◄── Session goal
              │  └──────────────┬───────────────────────┘ │
              │  ┌──────────────▼───────────────────────┐ │
              │  │ COMMITMENT                            │ │
              │  │ outcome · envelope · budget ·         │ │◄── Capability
              │  │ completion test · answer source       │ │    contracts (§12)
              │  │ ⇒ GOAL CONTRACT   (may exit here)     │ │
              │  └──────────────┬───────────────────────┘ │
              │  ┌──────────────▼───────────────────────┐ │
              │  │ PURSUIT LOOP                          │ │
              │  │ observe → select → authorise → act →  │ │
              │  │ verify → continue?                    │ │
              │  └──────────────┬───────────────────────┘ │
              └─────────────────┼─────────────────────────┘
                                │ one CapabilityIntent per effect
              ┌─────────────────▼─────────────────────────┐
              │  KERNEL OPERATOR    (sole execution        │
              │  authority)                                │
              │  plan → Permission Gateway → invoke →      │
              │  compose · Completion Contract             │
              └─────────────────┬─────────────────────────┘
                                │
              ┌─────────────────▼─────────────────────────┐
              │  CAPABILITY RUNTIME → Router → Providers   │
              │  providers never call providers            │
              └────────────────────────────────────────────┘

Observations flow back up into the goal's fact set.
Nothing bypasses the Kernel. Nothing in Conversation orchestrates.
```

The authority chain is unchanged from the Kernel Authority Rule: Conversation → Intent → single IPC → Kernel Operator → Runtime. What changes is that the Intelligence Layer issues **one intent per effect within a pursued goal**, instead of one intent per utterance. That is the structural change, and §30 assesses its constitutional cost.

---

## 28. What the Intelligence Layer MUST NOT do

1. Substitute a desktop effect for a missing answer. (P2)
2. Report completion from dispatch rather than observation.
3. Infer standing or broadened authority from a narrow request.
4. Act on the first of several matches when the request implied one subject.
5. Carry an interpretation across an effect without re-observing.
6. Send Owner content to a third party without explicit consent.
7. Convert a failed goal into a different goal without asking.
8. Orchestrate providers, sequence provider calls, or make permission decisions — those are Kernel authority.
9. Expose provider, router, registry, kernel, or capability-ID vocabulary to the Owner.
10. Grow by adding branches to a matching ladder.
11. Persist a semantic model of the Owner beyond the session. (`C-INT-004`, REJECTED)
12. Run an unbounded loop, poll indefinitely, or self-authorise continuation. (`C-INT-005`, REJECTED)
13. Steal focus or alter layout as a side effect of an unrelated goal.
14. Speak in command suggestions or capability catalogues.

---

## 29. What remains outside the Intelligence Layer

| Concern | Owner |
| --- | --- |
| Permission enforcement | Kernel Permission Gateway |
| Execution authority and step invocation | Kernel Operator |
| Provider routing and registry | Capability Runtime |
| Effect implementation | Providers |
| Audit and governance records | Kernel |
| Input capture, rendering, voice lifecycle | Conversation / Presentation |
| Windows permission dialogs | Windows |
| Moment save/restore decisions | Owner, through Moments |
| Capability implementation | Execution programs, under the Provider Acceptance Standard |

The Intelligence Layer *decides* and *commits*. It never *enforces* and never *performs*.

---

## 30. Migration implications for current Workspace architecture

### The shape of the change

This is not additive. Three inversions are required, and each is a real cost.

**Inversion 1 — comprehension moves to the front.** `resolveIntelligenceRoute` at `intentBridge.ts:2224` becomes a Request Frame constructed before any desktop matching. The 425 matching constructs in `intentBridge.ts` do not disappear; they are re-seated as *evidence contributing to Frame fields* rather than as *decisions returning actions*. That distinction is the entire migration, and it can be done incrementally, one field at a time.

**Inversion 2 — planning becomes generative.** `buildExecutionPlan`'s `switch (action.kind)` inverts: capabilities are selected because a goal requires their `produces`, rather than described after an action was picked.

**Inversion 3 — the turn becomes a loop.** `handleOperatorUtterance` currently issues one IPC and returns. Pursuit requires it to issue several within one Owner turn, with observation between them.

### Constitutional conflicts — stated, not resolved

**Conflict A — determinism scope.** Two authorities say different things.

`docs/capability-runtime/PRODUCT_PROOF_RULE.md:47`:

> Intent resolution remains deterministic (no probabilistic AI matching).

`docs/00-Constitution/WORKSPACE_CONSTITUTIONAL_SPECIFICATION_V2.md:230`:

> **Determinism** | Authority and Execution are not probabilistic for operating-system Effects

These are not the same rule. The Specification constrains **Authority and Execution**. The Product Proof Rule constrains **Intent resolution** as a whole, which includes comprehension.

§16 concedes that the `KNOW`/`SHOW` discrimination is a genuine language judgement. A fully deterministic implementation of it is possible but will be a keyword table — precisely the if/else tree the Owner rejected, and the mechanism that produced the Case 1 and Case 2 defects.

The design in this document depends on P10: comprehension may be uncertain, authority may not. Under the Specification's wording that is permissible, because a Request Frame is not an Effect. Under the Product Proof Rule's wording it is not.

One further piece of Specification text is relevant and cuts in the design's favour. §7.3 states that "no component MAY acquire Authority implicitly", and it names the categories it is worried about: "interfaces, providers, helpers, **models**, registries, plugins, and agents MUST NOT silently become authoritative." The Specification therefore already contemplates models existing. What it forbids is a model becoming authoritative. That is precisely the line P10 draws, and it suggests the Specification's authors separated comprehension from authority deliberately.

**This conflict is named, not resolved.** Resolving it requires an ADR and Constitutional Review. If the Owner declines to relax it, this document remains valid with one substitution: comprehension stays deterministic, and §16's discrimination is implemented as a declared, inspectable, and *acknowledged-incomplete* rule set rather than a judgement. Every other section is unaffected — which is worth stating plainly, because the value here does not hinge on adopting a model.

**Conflict B — one IPC per turn.** The Kernel Authority Rule specifies `Conversation → Intent (TS) → single IPC execute_capability_intent → Kernel Operator → Runtime`. Bounded pursuit needs several intents per Owner turn.

Two readings. Strict: "single IPC" means one per Owner utterance, and pursuit violates it. Structural: "single IPC" names *one entry point* rather than one call, forbidding side channels, not iteration. The structural reading is more consistent with the rule's stated purpose — the Kernel is the sole authority — and with C-PROC-002, which already performs multi-effect compositions per turn. But it is a reading, and it should be confirmed by the Owner rather than assumed.

An alternative that avoids the conflict entirely: express pursuit as **Kernel-side composition**, so the Intelligence Layer still sends one intent and the Kernel runs the observe/act/verify loop. This preserves the letter of the rule and keeps composition authority in the Kernel, at the cost of putting goal semantics into Rust. §32 does not choose between these; the choice belongs to the Owner.

**Conflict C — who selects capabilities.** This is the sharpest of the three, and §12 and §13 as written are on the wrong side of it.

`docs/capability-runtime/INTENT_LAYER_SPECIFICATION.md` says of the Intent Layer:

> It does not perform desktop effects, plan steps, or choose providers.

`docs/operator/CAPABILITY_COMPOSITION_RULE.md`:

> Composition authority belongs only to the Kernel Operator.

And the Operator Authority Rule assigns to the Operator alone the decisions of which providers participate and in what order.

§13 proposes that the Intelligence Layer select capabilities by matching `produces` against `consumes` and sequence them into steps. Under the current authority map that is *planning steps and choosing providers*, which the Intent Layer Specification forbids in terms. Softening it to "proposes a step sequence" does not dissolve the conflict; a proposal that determines the sequence is the decision.

The honest resolution is that **capability selection belongs to the Kernel Operator**, and it carries a concrete consequence for §12: the machine-readable capability contract facet must be readable by the Kernel, not merely by TypeScript. The Intelligence Layer would then emit a Goal Contract — outcome, subject, envelope, budget, completion test — and the Kernel would bind capabilities to it and run the loop. That reading also resolves Conflict B, since one Goal Contract is one intent.

This is a materially different implementation than the one §32 sequences, and it should be settled before step 3 rather than after. It does not change any behavioural conclusion in this document; it changes which side of the IPC boundary the machinery lives on.

### What is preserved unchanged

Presentation Purity, the Operator Authority Rule, Capability Composition (providers never call providers), the Permission Gateway, the Completion Contract, Product Gravity, the rejection of `C-INT-004` and `C-INT-005`, and the Semantic Alias Rule. Nothing in this document weakens any of them; §10 and §15 strengthen two.

---

## 31. Highest-priority architectural gaps

**G1 — No representation of desired outcome.** Nothing in the system distinguishes "know a fact" from "see a place". Root cause of Cases 1 and 2. *Highest priority: every other gap is harder to fix while this is open.*

**G2 — Understanding runs after action matching.** `intentBridge.ts:2224`. Makes reasoning the exception and desktop action the default.

**G3 — No goal survives a turn.** `handleOperatorUtterance` is one-shot. Cases 3, 4, 7 are structurally impossible, not merely unimplemented.

**G4 — Capability metadata is not machine-reasonable.** 24 prose fields; no `produces`/`consumes`. Prevents "which capability yields X?" and forces every new capability into a matching branch.

**G5 — Planning is retrospective.** Plans are derived from chosen actions, so capability sufficiency is never tested against a goal *before acting*.

**G6 — Substitution is unguarded.** A knowledge gap converts to a browser open with no rule preventing it. G6 is a consequence of G1 but needs an explicit prohibition, because it will otherwise reappear with each new provider.

**G7 — Partial completion is scoped to a call, not a goal.** The Kernel emits `partial` for ten compositions, with correct verified-versus-expected semantics. But it describes one IPC round trip, and every rule is hand-authored per composition. A goal spanning several calls, or one whose step sequence nobody anticipated, has no way to report that it half-succeeded.

**G8 — Local computation is phrase-matched.** Rung 1 has no declared coverage, so ordinary arithmetic reaches a third party.

**G9 — Third-party transfer without consent.** Owner utterances are placed into a `chatgpt.com` query string on a regex match.

**G10 — Composition logic exists on a path that appears unreachable.** §0.7. `KernelOperator::execute_turn` has no callers; three compositions live only inside it. **Requires independent verification with measured evidence before any conclusion is drawn.** Listed here because if confirmed it means engineering verification and product behaviour have diverged, which is a governance gap rather than a code defect.

---

## 32. Recommended implementation sequence

Sequenced so that each step is independently verifiable and each makes the next cheaper. **This is a recommendation. Nothing here is authorised, and no step should begin without Owner selection and a constitutional execution program.**

| # | Slice | Closes | Owner-visible? |
| --- | --- | --- | --- |
| 1 | **Request Frame + outcome classification, ahead of desktop matching, with the Substitution Prohibition enforced** | G1, G2, G6 | Yes — Cases 1, 2, 5, rows 11, 12 |
| 2 | Answer Source Ladder: rung 1 as a real capability; ChatGPT demoted to consented rung 6 | G8, G9 | Yes |
| 3 | Capability contract facet (`produces`/`consumes`/`pre`/`post`) alongside the existing registry | G4 | No |
| 4 | Goal Contract + completion from postconditions + `partially-satisfied` | G7 | Yes |
| 5 | Generative plan binding; decline-before-effects on unbindable goals | G5 | Yes — Cases 3, 7 decline honestly |
| 6 | Bounded pursuit loop | G3 | Yes — Cases 3, 4, 7 complete |
| 7 | Correction and clarification as first-class goal operations | — | Yes — rows 14, 16 |

Steps 1, 2, and 4 are Owner-visible improvements that require no new desktop capability, and none of them touches the authority boundary — they are safe to pursue while the conflicts in §30 are still open.

Steps 3, 5, and 6 are not. All three depend on **Conflict C** being resolved first, because the answer determines whether the capability contract facet and the binding logic live in TypeScript or in the Kernel — and building them on the wrong side would be expensive to undo. Step 6 additionally depends on Conflict B. The sequencing recommendation is therefore: do 1, 2, and 4; resolve Conflict C by ADR; then reassess 3 onward.

Separately and not part of this sequence: **G10 must be investigated before any further capability work**, under Evidence Before Modification. If the finding is confirmed, Capability Regression Prevention applies and a verifier that exercises the production path — rather than inspecting source text — is required.

---

## THE ONE-SENTENCE BEHAVIORAL CONTRACT

> Workspace Intelligence is responsible for determining what the Owner wants to be true, deciding truthfully whether it can make it true within what the Owner authorised, and then either making it true and verifying it or saying plainly why it cannot — never substituting an action for an answer, and never reporting a step as a goal.

---

## THE ONE-SENTENCE OWNER EXPERIENCE

> The Owner should feel they are talking to someone who understood what they meant, did only what was needed, and told them the truth about what happened.

---

## RECOMMENDED NEXT ARCHITECTURAL SLICE

**Outcome-First Comprehension: construct a Request Frame — carrying desired outcome, subject, illocution, and resolved references — before any desktop matching runs, and enforce the Substitution Prohibition against it.**

Chosen over the alternatives for four reasons. It is the only slice that fixes Owner-visible defects (Cases 1, 2, 5, and benchmark rows 11 and 12) without adding a single desktop capability. It is bounded — one new module plus an inversion at one call site in `intentBridge.ts`, with the existing branches left in place beneath it. It is deterministically verifiable: a fixed corpus of utterances, each with an expected outcome class, plus an assertion that no `KNOW`, `COMPUTE`, or `SOCIAL` outcome ever produces a desktop effect. And every later slice consumes the Frame, so nothing downstream can be designed properly until it exists.

### Status — P23.S1 delivered (comprehension half)

Implemented as **C-ITL-006 Goal Contract**, `app/src/lib/goalContract.ts`. What shipped:

- `comprehend(utterance)` runs before Workspace Context and before any desktop matching, and derives the desired outcome from accumulated signals rather than from a first-matching command branch.
- The contract carries outcome, mode, domain, subject, targets, references, requested result, compound structure, clarification need, and uncertainty. It carries no capability id, provider, operation, or step.
- The result is preserved: `resolveIntentWithGoal` returns it beside the action, `commitWorkspaceContext` stores it as `currentGoal`, and `handleOperatorUtterance` returns it on `OperatorOutcome`.
- `scripts/verify-goal-contract.mjs` fails the build if comprehension imports a planner, provider, or transport surface, if the contract shape grows an execution field, or if `CapabilityIntent` grows a meaning field.

What did **not** ship in S1, deliberately:

- **Enforcement.** Delivered separately in P23.S2 (below).
- **Consumption.** No consumer reads the contract to decide *which capability runs*. It is preserved and now constrains refusal, but it does not select.

### Status — P23.S2 delivered (enforcement half)

Implemented as **C-ITL-007 Substitution Prohibition Enforcement**, `app/src/lib/substitutionProhibition.ts`, applied at the single Intent entry point after the action resolves.

Measured substitutions that existed before the slice, and what changed:

| Utterance | Before | After |
| --- | --- | --- |
| "What does minimize mean?" | Collapsed the conversation surface, because a vocabulary regex matched "minimize" anywhere | No effect; the collapse branch now requires a command, not a question |
| "How's your day?" | Opened a browser | Conversational reply |
| "How are you?" | "Want to try a desktop step…" | Conversational reply, no desktop suggestion |
| "Do you know the time in Queensland?" | "I can't take that on yet." | Limitation stated in terms of the Owner's goal |

Two constraints keep enforcement from becoming a second planner:

- It may only *remove* an effect. The verifier rejects the file if it constructs any action that is not a speaking action.
- Refusal requires **positive** comprehension evidence. Comprehension classifies any unrecognised question as `KNOW` by default, and a default is not evidence; allowing it to override a specific desktop match silently disabled real capabilities (measured: 14 failing tests, three Owner batteries dropping to 76–83%). The bare-question default now never refuses.

External handoff is unchanged. `C-REA-003` marks its own decision with `informationHandoff`, and only routing may set that mark (verifier-enforced). A `KNOW` outcome never implies opening ChatGPT by itself.

**Still open at the end of S2.** Workspace still could not *answer* "What time is it in Queensland?" — it handed off. Obtaining the answer is an answer-source problem, not a substitution problem, and was closed by S3 below.

### Status — P23.S3 delivered (the first answer source)

Implemented as an extension of **C-REA-002 Local Reasoning**, not a new capability: that capability already owned deterministic local answering, and a second one would have created a competing authority for the same question.

Two modules, both meaning-only:

- `app/src/lib/answerSource.ts` — the **Answer Source Ladder**. It answers *which trusted source knows this*, and is structurally prevented from answering *which capability executes*: the verifier rejects either file for referencing IPC, a capability registry, an executable action, or provider identity. All five rungs are named so later sources slot in; only `deterministic-local` is implemented. The `authorized-external` rung is deliberately empty — external information remains C-REA-003's existing decision, reached by falling *through* the ladder rather than by a source registered in it.
- `app/src/lib/temporalAnswerSource.ts` — the local clock, and now the repository's single time-zone authority. Region phrase → IANA zone, then `Intl.DateTimeFormat`. Daylight saving belongs to the runtime's time-zone database; the module contains no offset arithmetic and the verifier fails the build if any appears (falsified: injecting a `10 * 60 * 60 * 1000` Brisbane offset was rejected).

The measured Owner failure now resolves locally: `KNOW → domain time → Queensland → Australia/Brisbane → runtime clock → "It’s 7:41 PM in Queensland."`, with the desktop cascade never running, so no browser and no handoff are reachable for it.

Three properties keep this from becoming a second planner:

- **It cannot pre-empt desktop work.** The ladder is gated by S2's own predicates, so it never acts on the bare-question default and never runs for a goal that wants machine change.
- **It declines rather than guesses.** Ambiguous places (WA, Georgia) resolve to nothing and reach the existing clarification; unlisted places fall through to the existing external path. Bare "WA" resolves only when the previous turn was already an Australian region — evidence, not assumption.
- **It emits text.** A source returns `{ text, sourceId, rung }` and nothing else. Nothing it produces could be executed.

Comprehension gained clock/date *shapes* rather than phrases, so "Can you tell me the time in Queensland?" and "What is the time right now in QLD?" comprehend identically to "What time is it in Queensland?".

### Status — P23.S4 delivered (the first observed answer)

The measured failure: "What windows are currently open?" was refused with "I can work with open windows when I know which one you mean.", and "What applications are open?" got a generic refusal — while "Which windows are open?" worked. Comprehension had already produced `PERCEIVE_MACHINE` in the desktop domain for all three. **The meaning was correct and was being discarded**, which is the clearest demonstration so far of the gap this constitution describes.

The audit found the Kernel side already complete. `execute_capability_intent` carries `window/enumerate` through the Permission Gateway and returns structured `items`; Conversation was already receiving them and displaying the Kernel's operational bullet list. No missing interface, no new IPC, and no Rust change — the slice needed two connections, not an authority model:

1. **Meaning reaches the existing request.** When comprehension identifies the open-window inventory need and literal matching produced nothing, the need is carried to the *same* authorized observation `"Which windows are open?"` already used. An action the cascade resolved is never overridden, so this cannot select capabilities.
2. **The observation becomes the answer.** The façade composes the returned items into "You've got 3 windows open: Chrome, Cursor, and File Explorer."

The ladder gained its `capability-observation` rung, and its shape is the point: a source at this rung **cannot observe**. It declares a semantic `ObservationNeed` and composes whatever authorized observation returns — the authority boundary is structural rather than a matter of discipline. Exactly one need exists (`open-windows`), and widening it is verifier-rejected, so the rung cannot become a second planner without a deliberate slice.

Truthfulness is enforced by derivation: the answer contains only observed titles (proved with titles that appear nowhere in the repository), and a refused or empty observation is reported as it stands. Because `ApplicationWindowItem` carries no process name, answers name window titles rather than applications; carrying `process_name` through is a separate slice.

**Conflict C is narrowed, not resolved.** Meaning now reaches execution for exactly one semantic need, through the existing `CapabilityIntent`, which still has no meaning field. The general case — arbitrary comprehended goals driving Kernel composition — remains blocked on the same missing interface.

**Blocking dependency — the missing interface.** The contract stops at the Conversation façade. `CapabilityIntent` is a flat `{ domain, operation, arguments }` record with no field for meaning, and the Intent Layer Specification forbids Intent from planning or choosing providers while the Kernel Authority Rule makes the Kernel Operator the sole composition authority. Sending the Goal Contract over `execute_capability_intent` today would either place a second planning input in front of the Operator or require Intent to pre-select the capability — both violations. Conflict C (§30) must be resolved, and the Kernel must expose a meaning-accepting entry point, before comprehension can drive execution.

## Stop
