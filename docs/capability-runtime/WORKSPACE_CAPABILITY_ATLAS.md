# Workspace Capability Atlas v2.0

| Field | Value |
| --- | --- |
| **Document** | Authoritative engineering capability roadmap |
| **Version** | **2.0** |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Kind** | Roadmap atlas — single source of truth for capability sequencing |
| **Authority** | Spec v2 + EES v1 remain architectural law; this Atlas governs **capability sequencing** |
| **Update rule** | Every capability implementation **must** update this Atlas before stop |
| **ID rule** | Permanent `C-XXX-NNN` identifiers — **never reuse · never renumber** |
| **Posture** | Release Hold (R2) — Stage 2 Owner Acceptance; Owner Execution Loop authorizes Atlas slices |

**Replaces:** Atlas v1.0 (two-digit IDs retired — see §13 Legacy ID map).  
**Does not replace:** Constitutional Spec, Kernel Authority, Product Proof Rule, R2 Release Hold.

---

## 0. How to use this Atlas

Future engineering sessions **SHALL** execute:

1. Open this Atlas.  
2. Synchronize metadata (every capability has required fields).  
3. Locate the highest-priority capability that is **not** Trusted / Production / Rejected.  
4. Resolve dependencies; skip if blocked.  
5. Implement **exactly one** bounded vertical slice (or one **Capability Pair** milestone — §0.1).  
6. Verify (Atlas Verification + `pnpm test` / typecheck / health).  
7. Product Proof when required — prepare repo, stop engineering.  
8. Update Atlas, handoff, milestone, project-health.  
9. Commit. **Stop** — do not begin the next capability / pair.

### 0.1 Capability Pair Rule

If the Atlas **explicitly pairs** two capabilities as one functional milestone (example: **C-ACT-004 Mouse Click** + **C-ACT-005 Keyboard Input**), engineering **may** implement both in the **same** bounded slice **only when all** of the following hold:

1. Both are required for meaningful user value together.  
2. Both share the same dependencies.  
3. Both share the same verification surface.  
4. Product Proof evaluates them together.

**Shall:**

- Keep **separate** Capability IDs  
- Keep **separate** Readiness Scores  
- Keep **separate** Atlas records  
- Allow **shared** implementation (ports, Kernel ops, verifiers)  

**Shall not:** merge the capabilities into one ID; expand into a third capability in the same slice.

If the paired capability would **significantly expand scope** or needs a **different** Product Proof → return to **one-capability** execution.

**Current Atlas pair:** C-ACT-004 + C-ACT-005 (Desktop Control Interaction milestone).

---

## 1. Status vocabulary

| Status | Meaning |
| --- | --- |
| **IMPLEMENTED** | Engineering evidence live (Conversation → Intent → Kernel path, or documented shell) |
| **PLANNED** | Roadmap item; executable when dependencies clear |
| **BLOCKED** | Named blocker prevents work |
| **FUTURE** | Desired later; not next |
| **REJECTED** | Explicitly out of product / constitutional scope |
| **Trusted** | Owner Product Proof accepted for this capability |
| **Production** | Trusted + production-gate ready for ship surface |

Lifecycle:

```text
Not Started → Audited → Planned → Implementing → Verifying
  → Product Proof → Trusted → Production
```

Required metadata on every capability:

`Capability ID · Name · Layer · Status · Lifecycle · Dependencies · Verification · Product Proof · Priority · Engineering Notes · Capability Readiness Score`

### 1.1 Capability Readiness Score

Seven equally weighted dimensions (0–100%). **Overall** = arithmetic mean.

| Dimension | 100% means |
| --- | --- |
| Architecture | Fits frozen Kernel Operator / Intent / Runtime path |
| Dependencies | All prerequisite Atlas capabilities eng-complete |
| Implementation | Vertical slice live in repo |
| Verification | Atlas verifiers + regression green |
| Product Proof | Owner live path accepted for this capability |
| Trusted | Owner declared Trusted |
| Production | Trusted + production-gate ready |

Record format:

```text
Architecture ............ N%
Dependencies ............ N%
Implementation .......... N%
Verification ............ N%
Product Proof ........... N%
Trusted ................. N%
Production .............. N%
Overall ................. N%
```

Default guidance: eng-complete without Owner PP → Architecture/Dependencies/Implementation/Verification = 100; Product Proof/Trusted/Production = 0; **Overall ≈ 57%**.

---

## 2. Capability Layer Map

```text
L10  Workspace Intelligence
L9   Workflow Library
L8   Operator Procedures
L7   Capability Composition
L6   Verification
L5   Desktop Interaction
L4   Desktop Observation
L3   Intent
L2   Reasoning
L1   Conversation
```

Execution authority remains: Conversation → Intent → Kernel Operator → Runtime.

---

## 3. Capability records

### Layer 1 — Conversation

#### C-CON-001 Companion Greeting
| | |
| --- | --- |
| **Name** | Companion Greeting / Companion Conversation Surface |
| **Layer** | L1 Conversation |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Verifying → Product Proof (Owner ongoing) |
| **Dependencies** | Shell Form B; Intent |
| **Verification** | `verify-companion-conversation`, `verify-conversation-quality`, `verify-product-gravity` |
| **Product Proof** | Live Owner sessions; T1 accepted-with-changes |
| **Priority** | — (foundation) |
| **Engineering Notes** | Conversation is the product surface; never provider jargon |

#### C-CON-002 Voice Input
| | |
| --- | --- |
| **Name** | Voice Input |
| **Layer** | L1 Conversation |
| **Status** | **IMPLEMENTED** (eng) |
| **Lifecycle** | Product Proof — awaiting Owner Accept |
| **Dependencies** | Mic + speech privacy; composer |
| **Verification** | `verify-voice-input`, `verify-voice-regression` |
| **Product Proof** | Pending permanent Owner Accept |
| **Priority** | P0 (acceptance, not eng by default) |
| **Engineering Notes** | Soft Send parity; no listen timeout mid-speech |

#### C-CON-003 Soft Send / Working-State Continuity
| | |
| --- | --- |
| **Name** | Soft Send / Working-State Continuity |
| **Layer** | L1 Conversation |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete |
| **Dependencies** | C-CON-001 |
| **Verification** | `verify-conversation-working-state` |
| **Product Proof** | P17.S4 eng complete |
| **Priority** | — |
| **Engineering Notes** | Working copy during long ops |

#### C-CON-004 First-Session Empty Cue
| | |
| --- | --- |
| **Name** | First-Session Empty Cue |
| **Layer** | L1 Conversation |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete |
| **Dependencies** | C-CON-001 |
| **Verification** | `verify-conversation-first-session` |
| **Product Proof** | P20.S1 |
| **Priority** | — |
| **Engineering Notes** | Invite without command catalogue |

---

### Layer 2 — Reasoning

#### C-REA-001 Intelligence Routing
| | |
| --- | --- |
| **Name** | Intelligence Routing |
| **Layer** | L2 Reasoning |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete; Owner re-proof recommended |
| **Dependencies** | Intent fallthrough; Browser for provider handoff |
| **Verification** | `tests/intelligence-routing.test.ts`, `verify-intelligence-routing` |
| **Product Proof** | P22.S1; Owner live re-proof |
| **Priority** | — |
| **Engineering Notes** | Classify kind before desktop soft-miss |

#### C-REA-002 Local Reasoning
| | |
| --- | --- |
| **Name** | Local Reasoning (arith / units / clock) |
| **Layer** | L2 Reasoning |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete — P23.S3–P23.S5 extensions awaiting Owner Product Proof |
| **Dependencies** | C-REA-001; C-ITL-006 (Goal Contract); C-ITL-007; C-OBS-001, C-OBS-002 (observation rung); Workspace Context (P23.S5 grounding) |
| **Verification** | `verify-answer-source`; `verify-observation-answer`; `tests/answer-source.test.ts`; `tests/observation-answer.test.ts`; `tests/active-window-answer.test.ts`; intelligence-routing tests |
| **Product Proof** | P22.S1 covers arith/units. The P23.S3–P23.S5 Answer Source Ladder extensions require Owner Product Proof — Owner-visible answers to time, date, open-window, and active-window questions |
| **Priority** | Architecture slices P23.S3, P23.S4, P23.S5 |
| **Engineering Notes** | No world knowledge invent. P23.S3 gave this capability the **Answer Source Ladder** (`app/src/lib/answerSource.ts`): a rung-ordered contract answering "which trusted source knows this", never "which capability executes". Only the deterministic-local rung is implemented, by the temporal source (`app/src/lib/temporalAnswerSource.ts`), which owns the single time-zone authority for the repository — region phrase → IANA zone, then `Intl.DateTimeFormat`, so daylight saving comes from the runtime's time-zone database and never from offset arithmetic (verifier-enforced). Sources emit text only: no IPC, no action, no capability or provider identity. Ambiguous places (WA, Georgia) resolve to nothing and reach the existing clarification; unlisted places fall through to C-REA-003's handoff, which remains the only external route. Gated by the same predicates as C-ITL-007, so the ladder can never pre-empt a genuine desktop request. P23.S4 added the `capability-observation` rung, whose sources cannot observe: a source declares a semantic `ObservationNeed` and composes an answer from whatever authorized observation returns, so the Kernel stays the sole observer and the Permission Gateway still decides. Exactly one need existed at that point (`open-windows`, served by C-OBS-001) and widening it was verifier-rejected. P23.S5 added the second need (`active-window`, served by C-OBS-002) to test whether the rung scales without becoming a selector. It does, because of three properties the verifier now enforces: the needs are a fixed vocabulary of information (further growth still needs its own slice), each need is covered by exactly one source so a request has one meaning rather than a shortlist, and needs are translated by a **total** `Record<ObservationNeed, IntentAction>` into requests the Intent Layer already had — a need with no existing request does not compile, so nothing can be selected or invented at that boundary. P23.S5 also grounds elliptical questions (“Which one am I using?”) through existing Workspace Context, which refines *meaning* into perception only and never into an effect. |

#### C-REA-003 Reasoning Provider Handoff (ChatGPT)
| | |
| --- | --- |
| **Name** | Reasoning Provider Handoff |
| **Layer** | L2 Reasoning |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete |
| **Dependencies** | Browser Provider; C-REA-001 |
| **Verification** | `verify-intelligence-routing`, browser tests |
| **Product Proof** | P22.S1 |
| **Priority** | — |
| **Engineering Notes** | Browser open ChatGPT `?q=`; no fake answers |

#### C-REA-004 In-Conversation Model Provider
| | |
| --- | --- |
| **Name** | In-Conversation Model Provider |
| **Layer** | L2 Reasoning |
| **Status** | **FUTURE** |
| **Lifecycle** | Not Started (research) |
| **Dependencies** | Evidence Before Commitment; licensing |
| **Verification** | TBD after research |
| **Product Proof** | Required if adopted |
| **Priority** | P3 |
| **Engineering Notes** | Blocked by B-RES-001 |

---

### Layer 3 — Intent

#### C-ITL-001 Intent Bridge + Soften
| | |
| --- | --- |
| **Name** | Intent Bridge + Soften |
| **Layer** | L3 Intent |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Production-facing eng |
| **Dependencies** | C-CON-001 |
| **Verification** | `tests/intent-bridge.test.ts`, conversation batteries |
| **Product Proof** | Ongoing Owner use |
| **Priority** | — |
| **Engineering Notes** | Prefix `C-ITL` = Intent Layer (C-INT reserved for Workspace Intelligence) |

#### C-ITL-002 Semantic Intent + Entity Aliases
| | |
| --- | --- |
| **Name** | Semantic Intent + Entity Aliases |
| **Layer** | L3 Intent |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete |
| **Dependencies** | C-ITL-001 |
| **Verification** | `verify-semantic-intent` |
| **Product Proof** | Deterministic aliases Owner-usable |
| **Priority** | — |
| **Engineering Notes** | GPT→ChatGPT etc.; never in providers |

#### C-ITL-003 Workspace Context
| | |
| --- | --- |
| **Name** | Workspace Context (pronouns / again) |
| **Layer** | L3 Intent |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete |
| **Dependencies** | C-ITL-001 |
| **Verification** | `verify-workspace-context` |
| **Product Proof** | T1 continuity |
| **Priority** | — |
| **Engineering Notes** | Session-scoped |

#### C-ITL-004 Goal Resolution + Situation Goals
| | |
| --- | --- |
| **Name** | Goal Resolution + Situation Goals |
| **Layer** | L3 Intent |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete |
| **Dependencies** | C-ITL-001 |
| **Verification** | `verify-goal-resolution` |
| **Product Proof** | Eng complete |
| **Priority** | — |
| **Engineering Notes** | Situation goals feed procedures |

#### C-ITL-005 Capability Registry Discovery
| | |
| --- | --- |
| **Name** | Capability Registry Discovery |
| **Layer** | L3 Intent |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete |
| **Dependencies** | CAPABILITY_GRAPH |
| **Verification** | `verify-capability-registry` |
| **Product Proof** | Discovery without menus |
| **Priority** | — |
| **Engineering Notes** | Includes `desktop-ui-tree` node (C-OBS-003) |

#### C-ITL-006 Goal Contract (Outcome-First Comprehension)
| | |
| --- | --- |
| **Name** | Goal Contract (Outcome-First Comprehension) |
| **Layer** | L3 Intent |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete |
| **Dependencies** | C-ITL-001; C-ITL-002; C-ITL-003 |
| **Verification** | `verify-goal-contract`; `tests/goal-contract.test.ts` |
| **Product Proof** | Not applicable alone — comprehension is internal; proved through the capability that consumes it |
| **Priority** | Architecture slice P23.S1 |
| **Engineering Notes** | Meaning-only representation of the Owner's desired outcome, comprehended before desktop matching and preserved in Workspace Context. Selects no capability and performs no Effect. Distinct from C-ITL-004, which derives its outcome from an already-chosen action and carries an ExecutionPlan. Stops at the Kernel IPC boundary: `CapabilityIntent` carries no meaning field, pending Conflict C resolution. |

Capability Readiness Score:

```text
Architecture ............ 100%
Dependencies ............ 100%
Implementation .......... 100%
Verification ............ 100%
Product Proof ........... 0%
Trusted ................. 0%
Production .............. 0%
Overall ................. 57%
```

#### C-ITL-007 Substitution Prohibition Enforcement
| | |
| --- | --- |
| **Name** | Substitution Prohibition Enforcement |
| **Layer** | L3 Intent |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete |
| **Dependencies** | C-ITL-006 (Goal Contract); C-REA-001 (Intelligence Routing) |
| **Verification** | `verify-substitution-prohibition`; `tests/substitution-prohibition.test.ts` |
| **Product Proof** | Owner Product Proof required — Owner-visible reply behaviour for knowledge, computation, and social requests |
| **Priority** | Architecture slice P23.S2 |
| **Engineering Notes** | Refuses a desktop effect resolved from action-shaped vocabulary when the Goal Contract says the request is fulfilled by knowledge. Only removes effects; may construct nothing but speaking actions (verifier-enforced). Refusal requires positive comprehension evidence, so the bare-question default can never disable a capability. Hybrid and observation goals are exempt. External handoff remains C-REA-003's decision, marked with `informationHandoff`; a KNOW outcome never implies it. |

Capability Readiness Score:

```text
Architecture ............ 100%
Dependencies ............ 100%
Implementation .......... 100%
Verification ............ 100%
Product Proof ........... 0%
Trusted ................. 0%
Production .............. 0%
Overall ................. 57%
```

---

### Layer 4 — Desktop Observation

#### C-OBS-001 Enumerate Windows
| | |
| --- | --- |
| **Name** | Enumerate Windows |
| **Layer** | L4 Observation |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Trusted candidate (Owner use) |
| **Dependencies** | Window Provider |
| **Verification** | Window product-proof tests; batteries; `verify-observation-answer`; `tests/observation-answer.test.ts` |
| **Product Proof** | “What windows are open?” live path — Owner Product Proof still required for the P23.S4 answer form |
| **Priority** | — |
| **Engineering Notes** | Title match — not tab/DOM. P23.S4 reuses this observation unchanged as the first answer source at the `capability-observation` rung: the comprehended need reaches the existing `window/enumerate` request, and the returned `items` are composed into a conversational answer. No second enumeration, no new Kernel interface, no Rust change. `ApplicationWindowItem` carries no process name, so answers name window titles; naming applications would require carrying `process_name` through the item and belongs to its own slice. |

#### C-OBS-002 Active Window
| | |
| --- | --- |
| **Name** | Active Window |
| **Layer** | L4 Observation |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete — P23.S5 answer form awaiting Owner Product Proof |
| **Dependencies** | Window Provider `active` |
| **Verification** | Window provider / intent map `winActive`; `verify-observation-answer`; `tests/active-window-answer.test.ts` |
| **Product Proof** | “Which window is active?” live path — Owner Product Proof required for the P23.S5 answer form |
| **Priority** | Architecture slice P23.S5 |
| **Engineering Notes** | Foreground window truth. P23.S5 reuses this observation unchanged as the second answer source at the `capability-observation` rung: the comprehended need reaches the existing `window/active` request, and the returned focused item is composed into a conversational answer. No second observation, no new Kernel interface, no Rust change. Like C-OBS-001, `ApplicationWindowItem` carries no process name, so the answer names the window title and says so when the Owner asked after the application — a program name would be a guess. Carrying `process_name` through the item remains its own slice. |

#### C-OBS-003 Desktop UI Tree
| | |
| --- | --- |
| **Name** | Desktop UI Tree |
| **Layer** | L4 Observation |
| **Status** | **IMPLEMENTED** (engineering) |
| **Lifecycle** | Verifying → **Product Proof required** |
| **Dependencies** | C-OBS-001 / C-OBS-002; Window Provider; Windows UIA |
| **Verification** | `verify-desktop-ui-tree`, `tests/desktop-ui-tree.test.ts`, kernel `window_enumerate_controls_through_router` |
| **Product Proof** | **Required** — `P22_S2_DESKTOP_UI_TREE.md`; Owner live session |
| **Priority** | P0 (PP pending) |
| **Engineering Notes** | Observation only. `UiAutomationPort` + `enumerate_controls`. No click/type. Custom-drawn UI may return honest empty. Owner Execution Loop authorized under R2. Cleared B-API-001 for observation surface. |
| **Readiness** | Overall **57%** |
| | ```text
Architecture ............ 100%
Dependencies ............ 100%
Implementation .......... 100%
Verification ............ 100%
Product Proof ...........   0%
Trusted .................   0%
Production ..............   0%
Overall .................  57%
``` |

#### C-OBS-004 Window Control Discovery
| | |
| --- | --- |
| **Name** | Window Control Discovery |
| **Layer** | L4 Observation |
| **Status** | **IMPLEMENTED** (engineering) |
| **Lifecycle** | Verifying → **Product Proof required** |
| **Dependencies** | **C-OBS-003** (eng complete) |
| **Verification** | `verify-window-control-discovery`, `tests/window-control-discovery.test.ts`, kernel `window_find_control_through_router` |
| **Product Proof** | **Required** — `P22_S3_WINDOW_CONTROL_DISCOVERY.md`; Owner live session |
| **Priority** | P0 (PP pending) |
| **Engineering Notes** | Locate-by-name via `find_control` / `match_control`. Observation only. Honest `control_not_found`. Does not click/type (C-ACT-004/005). |
| **Readiness** | Overall **57%** |
| | ```text
Architecture ............ 100%
Dependencies ............ 100%
Implementation .......... 100%
Verification ............ 100%
Product Proof ...........   0%
Trusted .................   0%
Production ..............   0%
Overall .................  57%
``` |

#### C-OBS-005 Screenshot Capture
| | |
| --- | --- |
| **Name** | Screenshot Capture |
| **Layer** | L4 Observation |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete |
| **Dependencies** | Screenshot Provider |
| **Verification** | `verify-screenshot-provider` |
| **Product Proof** | Live screenshot path |
| **Priority** | — |
| **Engineering Notes** | Legacy Atlas v1 `C-OBS-03` → **C-OBS-005** (never reuse 003) |

#### C-OBS-006 Monitor Enumeration
| | |
| --- | --- |
| **Name** | Monitor Enumeration |
| **Layer** | L4 Observation |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete |
| **Dependencies** | Window Provider monitors |
| **Verification** | Kernel monitor tests |
| **Product Proof** | Eng complete |
| **Priority** | — |
| **Engineering Notes** | Legacy v1 `C-OBS-02` |

#### C-OBS-007 Tokenized / OCR Screen Perception
| | |
| --- | --- |
| **Name** | Tokenized / OCR Screen Perception |
| **Layer** | L4 Observation |
| **Status** | **FUTURE** |
| **Lifecycle** | Not Started |
| **Dependencies** | Prefer C-OBS-003 first; license review |
| **Verification** | TBD |
| **Product Proof** | Required if adopted |
| **Priority** | P2 |
| **Engineering Notes** | B-RES-002; OmniParser STUDY |

---

### Layer 5 — Desktop Interaction

#### C-ACT-001 Launch Application
| | |
| --- | --- |
| **Name** | Launch Application |
| **Layer** | L5 Interaction |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete |
| **Dependencies** | Application Provider |
| **Verification** | `verify-capability-runtime-foundation` |
| **Product Proof** | Open app NL path |
| **Priority** | — |
| **Engineering Notes** | No invent .exe |

#### C-ACT-002 Focus Window
| | |
| --- | --- |
| **Name** | Focus Window |
| **Layer** | L5 Interaction |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete |
| **Dependencies** | C-OBS-001 |
| **Verification** | focus-window registry / batteries |
| **Product Proof** | Live focus path |
| **Priority** | — |
| **Engineering Notes** | Title match honesty |

#### C-ACT-003 Snap Window
| | |
| --- | --- |
| **Name** | Snap Window |
| **Layer** | L5 Interaction |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete |
| **Dependencies** | C-ACT-002 |
| **Verification** | Kernel snap tests; execution planner |
| **Product Proof** | Live snap path |
| **Priority** | — |
| **Engineering Notes** | Includes maximize/minimize/restore family (window-state) |

#### C-ACT-004 Mouse Click
| | |
| --- | --- |
| **Name** | Mouse Click |
| **Layer** | L5 Interaction |
| **Status** | **IMPLEMENTED** (engineering) |
| **Lifecycle** | Verifying → **Product Proof required** (joint with C-ACT-005) |
| **Dependencies** | **C-OBS-003**; **C-OBS-004** (eng complete; PP parallel) |
| **Verification** | Shared — `verify-desktop-control-interaction`, `tests/desktop-control-interaction.test.ts`, kernel `window_invoke_and_set_control_through_router` |
| **Product Proof** | **Joint** with C-ACT-005 — `P22_S4_DESKTOP_CONTROL_INTERACTION.md` |
| **Priority** | P0 (PP pending) |
| **Engineering Notes** | Kernel composition `window.click_control`: Find → Invoke → Find. UIA InvokePattern. Honest partial if unverified. Pair Rule §0.1. |
| **Readiness** | Overall **57%** |
| | ```text
Architecture ............ 100%
Dependencies ............ 100%
Implementation .......... 100%
Verification ............ 100%
Product Proof ...........   0%
Trusted .................   0%
Production ..............   0%
Overall .................  57%
``` |

#### C-ACT-005 Keyboard Input
| | |
| --- | --- |
| **Name** | Keyboard Input |
| **Layer** | L5 Interaction |
| **Status** | **IMPLEMENTED** (engineering) |
| **Lifecycle** | Verifying → **Product Proof required** (joint with C-ACT-004) |
| **Dependencies** | **C-OBS-003**; **C-OBS-004** (eng complete; PP parallel) |
| **Verification** | Shared — `verify-desktop-control-interaction`, `tests/desktop-control-interaction.test.ts`, kernel `window_invoke_and_set_control_through_router` |
| **Product Proof** | **Joint** with C-ACT-004 — `P22_S4_DESKTOP_CONTROL_INTERACTION.md` |
| **Priority** | P0 (PP pending) |
| **Engineering Notes** | Kernel composition `window.type_control`: Find → SetValue → Find. UIA ValuePattern + read-back verify. Pair Rule §0.1. |
| **Readiness** | Overall **57%** |
| | ```text
Architecture ............ 100%
Dependencies ............ 100%
Implementation .......... 100%
Verification ............ 100%
Product Proof ...........   0%
Trusted .................   0%
Production ..............   0%
Overall .................  57%
``` |

#### C-ACT-006 Browser Open / Focus / Beside
| | |
| --- | --- |
| **Name** | Browser Open / Focus / Beside |
| **Layer** | L5 Interaction |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete |
| **Dependencies** | Browser Provider; C-CMP-001 |
| **Verification** | `verify-browser-provider`, `verify-capability-completion` |
| **Product Proof** | Beside live confirm |
| **Priority** | — |
| **Engineering Notes** | Known sites / URLs |

#### C-ACT-007 Known Folder Open
| | |
| --- | --- |
| **Name** | Known Folder Open |
| **Layer** | L5 Interaction |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete |
| **Dependencies** | Shell URIs |
| **Verification** | Registry folders |
| **Product Proof** | Eng complete |
| **Priority** | — |
| **Engineering Notes** | Not arbitrary filesystem |

#### C-ACT-008 Clipboard Read / Write
| | |
| --- | --- |
| **Name** | Clipboard Read / Write |
| **Layer** | L5 Interaction |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete |
| **Dependencies** | Clipboard provider |
| **Verification** | Kernel clipboard tests |
| **Product Proof** | Eng complete |
| **Priority** | — |
| **Engineering Notes** | — |

#### C-ACT-009 Notifications Show / Dismiss
| | |
| --- | --- |
| **Name** | Notifications |
| **Layer** | L5 Interaction |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete |
| **Dependencies** | Notifications Provider |
| **Verification** | `verify-notifications-provider` |
| **Product Proof** | Eng complete |
| **Priority** | — |
| **Engineering Notes** | — |

#### C-ACT-010 Screenshot Copy Compose
| | |
| --- | --- |
| **Name** | Screenshot Copy Compose |
| **Layer** | L5 Interaction |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete |
| **Dependencies** | C-OBS-005; C-CMP-001 |
| **Verification** | `verify-capability-completion` |
| **Product Proof** | P21.S1 |
| **Priority** | — |
| **Engineering Notes** | `capture_and_copy` |

#### C-ACT-011 File Provider
| | |
| --- | --- |
| **Name** | File Provider |
| **Layer** | L5 Interaction |
| **Status** | **BLOCKED** |
| **Lifecycle** | Blocked |
| **Dependencies** | Owner Voice Accept; Spec File program |
| **Verification** | TBD |
| **Product Proof** | Required when started |
| **Priority** | P1 (after unblock) |
| **Engineering Notes** | B-PP-001 + B-CAP-001. Do not begin under Hold. |

#### C-ACT-012 Terminal / Shell Commands
| | |
| --- | --- |
| **Name** | Terminal Provider |
| **Layer** | L5 Interaction |
| **Status** | **FUTURE** |
| **Lifecycle** | Not authorized |
| **Dependencies** | Explicit Owner program |
| **Verification** | TBD |
| **Product Proof** | Required if adopted |
| **Priority** | P4 |
| **Engineering Notes** | B-CAP-002 |

#### C-ACT-013 Arbitrary Mouse/Keyboard Agent Loop
| | |
| --- | --- |
| **Name** | Arbitrary Mouse/Keyboard Agent Loop |
| **Layer** | L5 Interaction |
| **Status** | **REJECTED** |
| **Lifecycle** | Rejected |
| **Dependencies** | — |
| **Verification** | — |
| **Product Proof** | — |
| **Priority** | — |
| **Engineering Notes** | VLM CUAs out of scope; Intent determinism |

---

### Layer 6 — Verification

#### C-VER-001 Action Verification
| | |
| --- | --- |
| **Name** | Action Verification / Capability Completion Contract |
| **Layer** | L6 Verification |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete |
| **Dependencies** | Kernel compose |
| **Verification** | `verify-capability-completion` |
| **Product Proof** | P21.S1 |
| **Priority** | — |
| **Engineering Notes** | Aggregate truthful complete/fail |

#### C-VER-002 Retry
| | |
| --- | --- |
| **Name** | Retry |
| **Layer** | L6 Verification |
| **Status** | **IMPLEMENTED** (engineering) |
| **Lifecycle** | Verifying → **Product Proof required** |
| **Dependencies** | C-VER-001; C-ACT-004/005; **C-VER-003** (Wait reuse) |
| **Verification** | `verify-bounded-retry`, `tests/bounded-retry.test.ts`, kernel `operator::retry` |
| **Product Proof** | `P22_S6_BOUNDED_RETRY.md` — Owner session required |
| **Priority** | P0 (PP pending) |
| **Engineering Notes** | Operator-owned. Max 2 attempts. Retryable: control_not_found / unverified click|type / window not_found. Non-retryable: need_* / click_failed / type_failed. Reuses WaitCondition (400ms). Same authorized control only — not an agent loop. Happy-path click/type unchanged (no wait when first attempt completes). |
| **Readiness** | Overall **57%** |
| | ```text
Architecture ............ 100%
Dependencies ............ 100%
Implementation .......... 100%
Verification ............ 100%
Product Proof ...........   0%
Trusted .................   0%
Production ..............   0%
Overall .................  57%
``` |

#### C-VER-003 Wait Conditions
| | |
| --- | --- |
| **Name** | Wait Conditions |
| **Layer** | L6 Verification |
| **Status** | **IMPLEMENTED** (engineering) |
| **Lifecycle** | Verifying → **Product Proof required** |
| **Dependencies** | C-OBS-003/004; C-ACT-004/005 (compose path; click/type unchanged); C-VER-001 |
| **Verification** | `verify-wait-conditions`, `tests/wait-conditions.test.ts`, kernel `wait_condition_*` + `window_wait_condition_through_router` |
| **Product Proof** | `P22_S5_WAIT_CONDITIONS.md` — Owner session required |
| **Priority** | P0 (PP pending) |
| **Engineering Notes** | Bounded poll (default 2s / max 8s / 50ms). Conditions: control_available, control_gone, window_available, window_active. Operator composition `window.wait_condition`: Wait → Find (when control_available). Not a retry/agent/workflow engine. Uses existing Window/UIA observe primitives. |
| **Readiness** | Overall **57%** |
| | ```text
Architecture ............ 100%
Dependencies ............ 100%
Implementation .......... 100%
Verification ............ 100%
Product Proof ...........   0%
Trusted .................   0%
Production ..............   0%
Overall .................  57%
``` |

---

### Layer 7 — Capability Composition

#### C-CMP-001 Capability Completion Contract
| | |
| --- | --- |
| **Name** | Capability Completion Contract (compose authority) |
| **Layer** | L7 Composition |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete |
| **Dependencies** | Kernel Operator |
| **Verification** | `verify-capability-completion` |
| **Product Proof** | P21.S1 |
| **Priority** | — |
| **Engineering Notes** | Overlaps C-VER-001 surface; Kernel owns compose |

#### C-CMP-002 Compound Goal Decomposition
| | |
| --- | --- |
| **Name** | Compound Goal Decomposition |
| **Layer** | L7 Composition |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete |
| **Dependencies** | C-ACT-001; C-ACT-006; C-CMP-001 |
| **Verification** | `verify-compound-open` |
| **Product Proof** | P21.S2 |
| **Priority** | — |
| **Engineering Notes** | “Open A and B” sequential |

#### C-CMP-003 Beside Layout Composition
| | |
| --- | --- |
| **Name** | Beside Layout Composition |
| **Layer** | L7 Composition |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete |
| **Dependencies** | C-ACT-006; C-ACT-003 |
| **Verification** | `verify-capability-completion` |
| **Product Proof** | P21.S1 |
| **Priority** | — |
| **Engineering Notes** | — |

#### C-CMP-004 Kernel Operator Plan / Execute
| | |
| --- | --- |
| **Name** | Kernel Operator Plan / Execute |
| **Layer** | L7 Composition |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Foundation Trusted |
| **Dependencies** | Spec Kernel Authority |
| **Verification** | Kernel batteries |
| **Product Proof** | Constitutional |
| **Priority** | — |
| **Engineering Notes** | Sole execution authority |

---

### Layer 8 — Operator Procedures

#### C-PROC-001 Import Asset
| | |
| --- | --- |
| **Name** | Import Asset |
| **Layer** | L8 Procedures |
| **Status** | **FUTURE** |
| **Lifecycle** | Not Started |
| **Dependencies** | C-ACT-011 (files) likely |
| **Verification** | TBD |
| **Product Proof** | Required |
| **Priority** | P3 |
| **Engineering Notes** | Named procedure — not File Provider itself |

#### C-PROC-002 Prepare Coding Workspace
| | |
| --- | --- |
| **Name** | Prepare Coding Workspace |
| **Layer** | L8 Procedures |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Audited → Planned → **Engineering complete** — Owner Product Proof required; Trusted/Production incomplete |
| **Dependencies** | C-ITL-002/003/004; C-ACT-001; C-ACT-006; C-CMP-002; C-CMP-001; C-CMP-004; C-VER-001; C-VER-003; C-OBS-001 |
| **Verification** | `verify-prepare-coding-workspace-definition`; `verify-prepare-coding-workspace`; `tests/prepare-coding-workspace.test.ts`; Kernel plan/preflight/compose tests |
| **Product Proof** | Defined in §C-PROC-002.8; **not run — Owner session required** |
| **Priority** | P0 (highest eng-complete Atlas capability awaiting Owner Product Proof) |
| **Engineering Notes** | Intent `prepareCodingWorkspace` → Kernel `desktop.prepare_coding_workspace`: PCW-001 whole-set `launch_alias`/URL preflight → PCW-002 reuse C-ACT-001/C-CMP-002/C-ACT-006 open units → PCW-003 C-VER-003 `window_available` (hwnd preferred) → PCW-004 Completion Contract on `identity_confirmed`. No Launch retry; no C-VER-002; no final-focus inference; no default app set. |
| **Readiness** | Overall **~57%** (Arch/Deps/Impl/Ver 100%; PP/Trusted/Production 0%) |
| | ```text
Architecture ............ 100%  (deterministic corrected contract)
Dependencies ............ 100%  (required capability primitives eng-complete)
Implementation .......... 100%  (Intent + Kernel composition wired)
Verification ............ 100%  (definition + runtime verifiers + focused tests)
Product Proof ...........   0%
Trusted .................   0%
Production ..............   0%
Overall ................. ~57%
``` |

##### C-PROC-002.1 Scope and completion

`C-PROC-002` prepares a finite, non-empty, Owner-selected ordered set of
desktop open targets for the current Conversation turn. It resolves the whole
set first, executes through existing compound/open capabilities, waits for
observable window availability where needed, and aggregates truthful
completed / partial / failed results under the Completion Contract.

The procedure **never chooses what a coding workspace contains**. There is no
default coding application, application set, layout, file, folder, terminal
command, project, control, or saved Moment. Target order is the Owner's order;
the Operator does not add, remove, substitute, or reorder targets.

Completion uses the existing Completion Contract (`C-CMP-001` / `C-VER-001`):

- **completed** — every resolved target has an observed top-level window
  matching that target's identity evidence;
- **partial** — at least one resolved target reached that state and at least
  one did not;
- **failed** — no resolved target reached the required state;
- **clarification required** — resolution/preflight failed before effects; no
  procedure Effect executes and this is not upgraded to partial or completed.

Final-active-window state is **not** a mandatory completion criterion. List
order never implies focus. Explicit focus remains a separate Owner request
through existing focus capabilities.

##### C-PROC-002.2 Entry routing (Continue vs clarification vs procedure)

| Owner request shape | Authority | Result |
| --- | --- | --- |
| Targetless coding/setup/resume family already owned by Situation Goals (examples: “I'm coding”, “coding environment”, “set me up”, “development setup”) | **C-PROC-003** + **C-WF-002** | Open Continue; restore only after Owner **Approve and restore**. Not C-PROC-002. |
| Prepare/set-up coding workspace phrasing **with** one or more named desktop targets | **C-PROC-002** | Enter PCW-001. |
| Prepare/set-up coding workspace phrasing **without** named targets and **not** already owned by Situation Goals Continue routing | Clarification Policy | Ask which application(s) or site(s); invent nothing; open/focus nothing. |
| Named targets that fail resolution/preflight | Clarification Policy | Clarify; open/focus nothing. |

A saved Moment is never an inferred C-PROC-002 target. Moments restore remains
the separate C-WF-002 path.

##### C-PROC-002.3 Target authority and launchability

A **resolved target** is a finite tuple produced before any Effect:

1. **Owner evidence** — explicit name in the current request, or an unambiguous
   Workspace Context referent that the Owner references in this request
   (“that app”, “again” only when Context binds exactly one prior open target).
2. **Intent canonical form** — existing C-ITL-002 entity resolution
   (`resolveDesktopEntity` / compound-open encoding): `app:<query>` or
   `browser:<url>`.
3. **Kernel executability** — for apps/protocols/shell targets, the query must
   be executable under the existing Application Provider `launch_alias` /
   explicit path-or-protocol rules used by C-ACT-001. For sites, the URL must
   satisfy the existing browser-open plausibility rules used by C-ACT-006 /
   C-CMP-002.

| Resolution class | Authoritative rule | Procedure result |
| --- | --- | --- |
| **Explicitly named target** | Current-turn Owner name that passes Intent resolution **and** Kernel executability. | Include once, Owner order. |
| **Existing known target** | Exact Context-bound prior open target unambiguously referenced now. Installation, registry presence, or a running window alone does **not** authorize selection. | Include only that exact bound target. |
| **Ambiguous target** | More than one plausible Intent/Context match, or no unique Context bind. | **CLARIFICATION**; no effect. |
| **Missing target** | No named/bound target, or Continue routing does not already own the utterance. | Continue (if C-PROC-003 owns it) or **CLARIFICATION**; never invent apps. |
| **Intent-known / Kernel-unexecutable** | Intent entity exists but Kernel cannot launch/open it under existing rules (example today: Intent knows “Visual Studio Code”, Application `launch_alias` does not). | **CLARIFICATION** for the whole set before any Effect; do not open earlier targets. |

**Preflight rule:** PCW-001 resolves and validates the **entire** ordered set
before the first desktop Effect. If any member fails Intent resolution, Kernel
executability, or uniqueness, Workspace clarifies and opens nothing.

**Existing limitation (documented, not invented away):** Intent entities and
Kernel `launch_alias` are separate tables today; there is no shared exported
launchability API. Future implementation must enforce the intersection using
those existing authorities — it must **not** invent a third alias table or
guess executables.

**Identity model:**

- Running windows expose existing `ApplicationWindowItem` fields: `hwnd`,
  `process_id`, `title`.
- When Find/open returns an `hwnd`, later focus/wait/verify for that running
  instance should prefer `hwnd` (already supported by Application/Window
  request fields).
- When only a closed target exists, identity starts as the canonical
  Intent/Kernel query/label; after the first observed window appears, prefer
  the returned `hwnd` for subsequent verification.
- Substring/first-match title search alone is **not** completion truth. Final
  verification must confirm an observed window for each resolved target using
  the strongest available identity (`hwnd` when present; otherwise exact
  canonical label / query match against observation results). If multiple
  windows still match after observation, report that target unconfirmed
  rather than inventing which one counts.

##### C-PROC-002.4 Deterministic step table

C-PROC-002 does **not** privately loop Target A→B→C. Ordered multi-target
execution is owned by **C-CMP-002**. Single-target open/focus is owned by
**C-ACT-001** / **C-ACT-006**. Waiting is owned by **C-VER-003**. Completion
aggregation is owned by **C-CMP-001** / **C-VER-001**. Application Launch is
**not** automatically retried.

| Step ID | Action | Required Capability | Target | Preconditions | Observable Success Condition | Timeout / bounded timing authority | Retry Policy | Failure Outcome |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| **PCW-001** | Resolve the complete ordered authorized target set; enforce launchability/openability preflight for every member; encode for existing compound/open execution. | C-ITL-002; C-ITL-003; C-ITL-004; Kernel Application `launch_alias` / browser openability rules (existing) | Every Owner-named or exactly Context-bound desktop open target in the current request | Request enters C-PROC-002 per §C-PROC-002.2; no Effect has begun | Non-empty finite ordered set; every member has Owner evidence + Intent canonical form + Kernel executability; encode ready for C-CMP-002 (`app:` / `browser:`) or single open | Single deterministic pass; no polling | **Not permitted.** | **CLARIFICATION** or Continue routing; no target Effect. |
| **PCW-002** | Execute the authorized set through existing composition: C-CMP-002 for ≥2 targets; C-ACT-001 `app.open_or_focus` for one app; C-ACT-006 browser open for one site. | C-CMP-002; C-ACT-001; C-ACT-006; C-CMP-004; Permission Gateway | Exact PCW-001 set, Owner order | PCW-001 succeeded for the entire set; current-turn Owner authorization applies | Existing composition returns per-target Focus/Launch/Open facts. Find then Focus or Launch remains inside C-ACT-001 / C-CMP-002 — not reimplemented here. Provisional until PCW-003 observation. | No invented operation deadline. Use existing provider / composition completion behaviour. | **Not permitted** for Launch/Open. Permission/unsupported/interaction failures stop that unit under existing composition rules. | Permission denial / unsupported / interaction failure: stop further units per existing C-CMP-002 stop-on-failure behaviour; retain earlier units; proceed to PCW-003/004 for truthful aggregation. |
| **PCW-003** | For each requested target, wait/observe until a matching top-level window is available or the wait bound expires. Prefer `hwnd` when PCW-002 produced one; otherwise wait on the canonical window query and confirm against observation. | C-VER-003 (`window_available`); C-OBS-001 | Each resolved target from PCW-001 | PCW-002 finished (completed, partial, or failed units) | Per target: `condition_met` **and** observed window identity evidence for that exact target | Existing C-VER-003 authority only: default 2s, max 8s, poll 50ms per wait | **No Launch/Open retry.** One bounded wait per target. C-VER-002 is **not** used (click/type-only; Launch is not proven idempotent). | `condition_timeout` / unconfirmed identity: mark that target unverified; continue remaining waits; never relaunch; never substitute another app. |
| **PCW-004** | Aggregate observed facts into completed / partial / failed under the Completion Contract. | C-CMP-001; C-VER-001; C-OBS-001 | Entire resolved target set | PCW-003 finished for all targets; no retries remain | Conversation reports completed only if every target has observed identity evidence; otherwise partial/failed with per-target facts; never upgrade missing evidence | One final observation pass; no invented deadline | **Not permitted.** | Return **completed**, **partial**, or **failed** from observed facts only. |

##### C-PROC-002.5 Authorization

| Step | Authorization classification | Rule |
| --- | --- | --- |
| PCW-001 | **No additional authorization** | Meaning/Plan only; no Effect. |
| PCW-002 | **Existing Owner confirmation** | Current-turn explicit names or exact Context bind authorize only those targets. Permission Gateway remains mandatory per Effect. Historical coding-workspace requests never create standing launch authority. |
| PCW-003 | **No additional authorization** | Bounded observation/wait of already-authorized targets. |
| PCW-004 | **No additional authorization** | Truthful completion composition only. |
| Separate C-WF-002 restore | **Existing Moments authorization** | Preview + Owner **Approve and restore** + plan-digest binding. Not a C-PROC-002 step. |

No procedure-wide approval token is invented. No automatic second Launch is
authorized. Every Effect remains Conversation → Intent →
`execute_capability_intent` → Kernel Operator → Permission Gateway → Runtime.
Providers never call each other.

##### C-PROC-002.6 Failure rules

| Failure | Deterministic behaviour | Retry / wait | Final procedure result |
| --- | --- | --- | --- |
| Targetless coding/setup owned by Situation Goals | Route to Continue/Moments | No | Continue path (not C-PROC-002) |
| Missing named target for prepare phrasing | Ask which application(s)/site(s); execute nothing | No | **CLARIFICATION** |
| Ambiguous target | Present concrete alternatives; execute nothing | No | **CLARIFICATION** |
| Intent-known / Kernel-unexecutable | Clarify the unexecutable target; execute nothing for the whole set | No | **CLARIFICATION** |
| Application not open | Existing C-ACT-001 / C-CMP-002 Find→Launch branch opens the exact resolved target once | No automatic relaunch | Continue to wait/verify |
| Application cannot be located after open/focus | One C-VER-003 `window_available` wait | Wait only; no Launch retry | Unverified target → **failed/partial** |
| Control cannot be located | Outside this procedure; do not invent click/type | No | Clarify the out-of-contract control request; report any prepared targets truthfully |
| Interaction / permission / unsupported failure | Report exact target failure; do not retry around policy | No | **failed/partial** |
| Verification timeout | Mark target unverified; do not relaunch | No | **failed/partial** |

##### C-PROC-002.7 Timing and retry model

**Timing authority:** only existing C-VER-003 Wait Conditions
(default 2s / max 8s / 50ms). No invented 2-second operation deadlines for
Find, Launch, Focus, or aggregate completion.

**Retry authority:** C-VER-002 is **out of scope** for C-PROC-002. It remains
click/type-only, and application Launch is not proven idempotent under current
repository evidence. Therefore:

- Observe → Wait → Verify is permitted;
- automatic Launch/Open re-attempt is **forbidden**;
- C-PROC-002 must not create a second retry orchestration mechanism.

##### C-PROC-002.8 Future Owner Product Proof

**Status:** Engineering complete; Product Proof **not executed; not accepted**.

Primary Product Proof must validate a credible coding-workspace preparation
using only repository-supported dual-authority targets. Neutral multi-app
opening is a regression case, not the sole proof.

| Field | Required future proof |
| --- | --- |
| **Exact Owner input (primary success)** | `Prepare my coding workspace with Cursor and Notepad.` Both names are Intent-known and Kernel-launchable today; Cursor is a coding-relevant explicit target, not a default stack. |
| **Neutral regression (optional)** | `Prepare my coding workspace with Notepad and Calculator.` Proves multi-target open only; not sufficient alone. |
| **Starting state** | Owner launches Workspace once; Conversation ready; named proof apps closed unless a case says otherwise; no Moment auto-selected. |
| **Expected visible outcome** | Cursor and Notepad are visibly open; Conversation reports **completed** only after both windows are observed. Final focus is **not** required unless the Owner separately asks to focus one. |
| **Targetless Continue case** | `I need my coding environment.` → Continue/Moments path; no invented apps. |
| **Targetless clarify case** | `Prepare my coding workspace.` → ask which applications/sites; launch/focus nothing. |
| **Unsupported / Kernel-unexecutable case** | `Prepare my coding workspace with Cursor and Visual Studio Code.` → whole-set clarification before effects (Intent may know VS Code; Kernel `launch_alias` does not); Cursor must not open first. |
| **Unknown-target case** | `Prepare my coding workspace with Cursor and NoSuchCodingAppZZZ.` → clarify; open nothing. |
| **Ambiguous-target case** | Unbound `Prepare my coding workspace with that app.` → clarify alternatives; open nothing. |
| **Already-open case** | Start Cursor first, then primary success input → reuse/focus Cursor; open Notepad; no duplicate Cursor invent. |
| **Partial-completion case** | First target observed ready, later target fails/times out → retain first effect; report **partial** with per-target facts. |
| **Permission / interaction failure** | Gateway denial or focus/launch refusal on a target → truthful failed/partial; no policy retry. |
| **Success criteria** | Exact target fidelity; no invented/default apps; whole-set preflight; no Launch retry; C-CMP-002 owns ordered execution; C-VER-003 owns waits; C-CMP-001 owns completion; no mandatory final-focus inference; ordinary phrasing variants work without rigid capitalization. |

Engineering verification must be green first. Only the Owner may mark Product Proof, Trusted, or Production.

##### C-PROC-002.9 Explicit non-goals

- Selecting a default IDE, editor, browser, terminal, project, or app set.
- Private Target A→B→C planner duplicating C-CMP-002.
- Private Find loop duplicating C-ACT-001 / C-CMP-002.
- Automatic Launch/Open retry or a second retry framework.
- Invented operation-timeout framework outside C-VER-003.
- Mandatory final-active-window inference from list order.
- Opening files/folders, running terminal commands, typing/clicking controls.
- Choosing, approving, or restoring a saved Moment.
- OCR/VLM perception, probabilistic target selection, autonomous agent loops,
  indefinite polling, or provider-to-provider calls.
- Treating definition verification as runtime implementation or Product Proof.

#### C-PROC-003 Situation Goals (setup / coding / done)
| | |
| --- | --- |
| **Name** | Situation Goals |
| **Layer** | L8 Procedures |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete |
| **Dependencies** | C-ITL-004 |
| **Verification** | goal / situation tests |
| **Product Proof** | Eng complete |
| **Priority** | — |
| **Engineering Notes** | — |

#### C-PROC-004 Shell Mode / Tray Continuity
| | |
| --- | --- |
| **Name** | Shell Mode Collapse / Tray Continuity |
| **Layer** | L8 Procedures |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete |
| **Dependencies** | Conversation shell |
| **Verification** | `verify-tray-shellmode` |
| **Product Proof** | P19.S1 |
| **Priority** | — |
| **Engineering Notes** | — |

---

### Layer 9 — Workflow Library

#### C-WF-001 Daily Coding Session
| | |
| --- | --- |
| **Name** | Daily Coding Session |
| **Layer** | L9 Workflow |
| **Status** | **FUTURE** |
| **Lifecycle** | Not Started |
| **Dependencies** | C-PROC-002; Moments; preferably C-ACT-004 |
| **Verification** | TBD |
| **Product Proof** | Required |
| **Priority** | P3 |
| **Engineering Notes** | Named multi-step workflow pack |

#### C-WF-002 Moments Save / Browse / Restore
| | |
| --- | --- |
| **Name** | Moments |
| **Layer** | L9 Workflow |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete; Owner daily use |
| **Dependencies** | Observation + restore auth |
| **Verification** | `verify-moments-*` |
| **Product Proof** | P18 suite |
| **Priority** | — |
| **Engineering Notes** | Restore needs Owner authorization |

---

### Layer 10 — Workspace Intelligence

#### C-INT-001 Reasoning Provider Routing
| | |
| --- | --- |
| **Name** | Reasoning Provider Routing |
| **Layer** | L10 Workspace Intelligence |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete |
| **Dependencies** | C-REA-001 / C-REA-003 |
| **Verification** | `verify-intelligence-routing` |
| **Product Proof** | P22.S1 |
| **Priority** | — |
| **Engineering Notes** | Atlas ID for intelligence routing surface (pairs with C-REA-001) |

#### C-INT-002 Product Intelligence Boundary
| | |
| --- | --- |
| **Name** | Product Intelligence Boundary |
| **Layer** | L10 Workspace Intelligence |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete |
| **Dependencies** | — |
| **Verification** | `verify-product-intelligence` |
| **Product Proof** | Gaps documented |
| **Priority** | — |
| **Engineering Notes** | OutsideWorkspace honesty |

#### C-INT-003 Capability Evolution Proposals
| | |
| --- | --- |
| **Name** | Capability Evolution Proposals |
| **Layer** | L10 Workspace Intelligence |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Proposal pipeline only |
| **Dependencies** | Owner approve |
| **Verification** | `verify-capability-evolution` |
| **Product Proof** | No self-rewrite |
| **Priority** | — |
| **Engineering Notes** | — |

#### C-INT-004 Long Semantic Chat Memory
| | |
| --- | --- |
| **Name** | Long Semantic Chat Memory |
| **Layer** | L10 Workspace Intelligence |
| **Status** | **REJECTED** |
| **Lifecycle** | Rejected |
| **Dependencies** | — |
| **Verification** | — |
| **Product Proof** | — |
| **Priority** | — |
| **Engineering Notes** | Chat-agent memory out of product |

#### C-INT-005 Autonomous Multi-Step Agent Loops
| | |
| --- | --- |
| **Name** | Autonomous Multi-Step Agent Loops |
| **Layer** | L10 Workspace Intelligence |
| **Status** | **REJECTED** |
| **Lifecycle** | Rejected |
| **Dependencies** | — |
| **Verification** | — |
| **Product Proof** | — |
| **Priority** | — |
| **Engineering Notes** | P22.A2 IGNORE |

---

### Cross-cutting — Release / Production

#### C-REL-001 Unsigned Release Pipeline (F1)
| | |
| --- | --- |
| **Name** | Unsigned Release Pipeline |
| **Layer** | Release |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Complete |
| **Dependencies** | CI |
| **Verification** | `verify-f1-ci-automation` |
| **Product Proof** | Pipeline eng |
| **Priority** | — |
| **Engineering Notes** | — |

#### C-REL-002 Code Signing (A2)
| | |
| --- | --- |
| **Name** | Code Signing |
| **Layer** | Release |
| **Status** | **BLOCKED** |
| **Lifecycle** | Blocked |
| **Dependencies** | Authenticode certificate |
| **Verification** | Playbook |
| **Product Proof** | Signed path |
| **Priority** | P0 (parallel release track) |
| **Engineering Notes** | B-EXT-001 |

#### C-REL-003 Signed Release + Updater (F2 / B2)
| | |
| --- | --- |
| **Name** | Signed Release + Updater |
| **Layer** | Release |
| **Status** | **BLOCKED** |
| **Lifecycle** | Blocked |
| **Dependencies** | C-REL-002 |
| **Verification** | TBD |
| **Product Proof** | Ship |
| **Priority** | P1 |
| **Engineering Notes** | — |

---

## 4. Capability Dependency Graph

```text
C-CON-001 Conversation
   └─ C-ITL-* Intent
         ├─ C-REA-001 / C-INT-001 Reasoning routing
         └─ C-CMP-004 Kernel Operator
               ├─ C-OBS-001..003 Observation (003 eng done → PP)
               ├─ C-ACT-001..003,006..010 Interaction (impl)
               ├─ C-VER-001 / C-CMP-001 Completion
               └─ C-WF-002 Moments

PLANNED spine:
  C-OBS-003/004 + C-ACT-004/005 (eng done → PP)
       └─► C-VER-003 Wait (eng done → PP)
              └─► C-VER-002 Retry (eng done → PP)

  C-ITL-002/003/004 + C-ACT-001/006 + C-CMP-002
       + C-VER-001/003 + C-OBS-001 + C-CMP-001/004
                     └─► C-PROC-002 (**IMPLEMENTED** eng; Owner PP required)
                              └─► C-WF-001 (later)

BLOCKED spine:
  B-PP-001 Voice Accept ──► C-ACT-011 File Provider
  B-EXT-001 Cert ──► C-REL-002 A2 ──► C-REL-003 F2/B2
```

---

## 5. Blocked Capability Register

| ID | Blocker | Severity | Affected | Recommended resolution |
| --- | --- | --- | --- | --- |
| **B-HOLD-001** | Release Hold — eng only via Owner Execution Loop / T1–T5 | High | New capability eng | Owner authorizes Atlas slice |
| **B-PP-001** | Owner Voice Product Proof Accept pending | High | Voice Product Complete; File gate | Owner Accept |
| **B-CAP-001** | File Provider gated on Voice Accept | High | C-ACT-011 | After B-PP-001 |
| **B-CAP-002** | Terminal provider not authorized | Med | C-ACT-012 | Explicit Owner program |
| **B-EXT-001** | Authenticode certificate unavailable | High | C-REL-002/003 | Obtain cert → A2 |
| **B-RES-001** | In-Conversation Model not researched to ADOPT | Med | C-REA-004 | Evidence Before Commitment |
| **B-RES-002** | Screen perception WRAP undecided | Med | C-OBS-007 | After UIA interaction; license |
| **B-API-001** | ~~No UIA observation surface~~ **CLEARED** (C-OBS-003 eng) | — | — | Interaction still needs C-ACT-004/005 |
| **B-PP-003** | C-OBS-003 Desktop UI Tree Product Proof pending | High | C-OBS-003 Trusted | Owner live session per P22.S2 |
| **B-PP-004** | C-OBS-004 Window Control Discovery Product Proof pending | High | C-OBS-004 Trusted | Owner live session per P22.S3 |
| **B-PP-005** | C-ACT-004/005 Desktop Control Interaction Product Proof pending | High | C-ACT-004 + C-ACT-005 Trusted | Joint Owner session per P22.S4 |
| **B-PP-006** | C-VER-003 Wait Conditions Product Proof pending | High | C-VER-003 Trusted | Owner live session per P22.S5 |
| **B-PP-007** | C-VER-002 Bounded Retry Product Proof pending | High | C-VER-002 Trusted | Owner live session per P22.S6 |
| **B-PLAT-001** | Windows-only product | Low | All desktop | Accepted scope |

### 5.1 Resolved definition blockers

| ID | Resolution | Evidence | Lifecycle effect |
| --- | --- | --- | --- |
| **B-DEF-001** | **RESOLVED 2026-08-08** — initial procedure body landed; **contract corrected** after pre-implementation audit; **runtime implemented** 2026-08-08 (Intent + Kernel composition). | C-PROC-002.1–.9; `verify-prepare-coding-workspace-definition`; `verify-prepare-coding-workspace` | C-PROC-002 is **IMPLEMENTED** (eng). Product Proof, Trusted, and Production remain incomplete. |

---

## 6. Verification Matrix

| Capability class | Implementation | Regression | Product Proof |
| --- | --- | --- | --- |
| Desktop UI Tree | UIA port + Window Provider | `verify-desktop-ui-tree` | Owner P22.S2 |
| Window Control Discovery | `find_control` + Intent | `verify-window-control-discovery` | Owner P22.S3 |
| Desktop Control Interaction | Invoke/SetValue + Operator compose | `verify-desktop-control-interaction` | Owner P22.S4 (joint) |
| Wait Conditions | Bounded WaitCondition + compose | `verify-wait-conditions` | Owner P22.S5 |
| Bounded Retry | Operator retry + Wait reuse | `verify-bounded-retry` | Owner P22.S6 |
| Prepare Coding Workspace definition | Atlas C-PROC-002.1–.9 | `verify-prepare-coding-workspace-definition` | Contract corrected |
| Prepare Coding Workspace runtime | Intent `prepareCodingWorkspace` + Kernel `desktop.prepare_coding_workspace` | `verify-prepare-coding-workspace`; `tests/prepare-coding-workspace.test.ts` | Eng complete; Owner Product Proof required |
| Providers | `cargo` + verify scripts | `pnpm test` | Owner NL |
| Composition | Kernel + completion/compound | Hostile NL | Live confirm |
| Reasoning | intelligence-routing | verify script | Owner re-proof |
| Voice | voice verifiers | regression | Owner Accept |
| Release | F1/R2 | CI | Signed later |

**No new verification framework.**

---

## 7. Product Proof Matrix

| Capability | Eng complete? | Product Proof status |
| --- | --- | --- |
| C-OBS-003 Desktop UI Tree | **Yes** | **Required — pending Owner** |
| C-OBS-004 Window Control Discovery | **Yes** | **Required — pending Owner** |
| C-ACT-004 Mouse Click | **Yes** | **Required — joint with C-ACT-005** |
| C-ACT-005 Keyboard Input | **Yes** | **Required — joint with C-ACT-004** |
| C-VER-003 Wait Conditions | **Yes** | **Required — pending Owner** |
| C-VER-002 Retry | **Yes** | **Required — pending Owner** |
| C-PROC-002 Prepare Coding Workspace | **Yes (eng)** | **Required — pending Owner** |
| Core desktop providers | Yes | Mixed Owner trust |
| Voice | Yes | Pending Accept |
| Intelligence routing | Yes | Owner re-proof |
| Beside / Compound | Yes | Owner confirm |
| File Provider | No | Blocked |
| Signed release | No | Cert blocked |

---

## 7.5 Capability Readiness Matrix

Overall = mean of seven dimensions. Eng-complete without Owner PP defaults to **≈57%**. Trusted/Production remain Owner gates.

| ID | Name | Status | Overall |
| --- | --- | --- | --- |
| C-CON-001 | Companion Greeting | IMPLEMENTED | ~71% (PP partial) |
| C-CON-002 | Voice Input | IMPLEMENTED (eng) | ~57% (PP pending Accept) |
| C-CON-003 | Soft Send Continuity | IMPLEMENTED | ~71% |
| C-CON-004 | First-Session Cue | IMPLEMENTED | ~71% |
| C-REA-001 | Intelligence Routing | IMPLEMENTED | ~57% |
| C-REA-002 | Local Reasoning (+ P23.S3–S5 Answer Source Ladder) | IMPLEMENTED | ~57% |
| C-REA-003 | Provider Handoff | IMPLEMENTED | ~57% |
| C-REA-004 | In-Conversation Model | FUTURE | ~14% |
| C-ITL-001..005 | Intent stack | IMPLEMENTED | ~71% |
| **C-ITL-006** | **Goal Contract (Outcome-First Comprehension)** | **IMPLEMENTED (eng)** | **57%** |
| **C-ITL-007** | **Substitution Prohibition Enforcement** | **IMPLEMENTED (eng)** | **57%** |
| C-OBS-001 | Enumerate Windows | IMPLEMENTED | ~71% |
| C-OBS-002 | Active Window (+ P23.S5 answer form) | IMPLEMENTED | ~71% (answer form PP pending) |
| **C-OBS-003** | **Desktop UI Tree** | **IMPLEMENTED (eng)** | **57%** |
| **C-OBS-004** | **Window Control Discovery** | **IMPLEMENTED (eng)** | **57%** |
| C-OBS-005 | Screenshot | IMPLEMENTED | ~71% |
| C-OBS-006 | Monitors | IMPLEMENTED | ~71% |
| C-OBS-007 | OCR Perception | FUTURE | ~14% |
| C-ACT-001..003,006..010 | Core actions | IMPLEMENTED | ~71% |
| **C-ACT-004** | **Mouse Click** | **IMPLEMENTED (eng)** | **57%** |
| **C-ACT-005** | **Keyboard Input** | **IMPLEMENTED (eng)** | **57%** |
| C-ACT-011 | File Provider | BLOCKED | ~14% |
| C-ACT-012 | Terminal | FUTURE | ~14% |
| C-ACT-013 | Agent Loop | REJECTED | 0% |
| C-VER-001 / C-CMP-001 | Completion | IMPLEMENTED | ~71% |
| **C-VER-002** | **Retry** | **IMPLEMENTED (eng)** | **57%** |
| **C-VER-003** | **Wait Conditions** | **IMPLEMENTED (eng)** | **57%** |
| C-CMP-002..004 | Composition | IMPLEMENTED | ~71% |
| **C-PROC-002** | **Prepare Coding Workspace** | **IMPLEMENTED (eng)** | **~57%** (PP/Trusted/Production open) |
| C-PROC-* / C-WF-* (other) | Procedures / Workflows | Mixed | see records |
| C-INT-001..003 | Intelligence | IMPLEMENTED | ~57–71% |
| C-INT-004..005 | Memory / Agent | REJECTED | 0% |
| C-REL-001 | F1 Pipeline | IMPLEMENTED | ~71% |
| C-REL-002..003 | Signing / Signed | BLOCKED | ~14% |

---

## 8. Priority Queue

| Rank | ID | Capability | Status | Why |
| --- | --- | --- | --- | --- |
| **1** | **C-PROC-002** | Prepare Coding Workspace | **IMPLEMENTED (eng)** | Owner Product Proof required; do not mark Trusted/Production |
| 2 | C-REL-002 | Code Signing A2 | BLOCKED (cert) | Parallel release track |
| 3 | C-ACT-011 | File Provider | BLOCKED (Voice) | Production Before Expansion |
| 4 | C-OBS-007 | Tokenized perception | FUTURE | After interaction PP |
| 5 | C-REA-004 | In-Conversation Model | FUTURE | Research |
| 6 | C-WF-001 | Daily Coding Session | FUTURE | After C-PROC-002 |

**Rejected (do not queue):** C-ACT-013; C-INT-004; C-INT-005.

**Latest runtime slice:** C-PROC-002 engineering complete. Owner Product Proof
is required before Trusted/Production. No silent scope expansion.

---

## 9. Current highest remaining executable capability

### **C-PROC-002 — IMPLEMENTED (eng); Product Proof pending**

C-PROC-002 composes existing authorities (C-CMP-002, C-ACT-001/006, C-VER-003,
C-CMP-001) via Intent `prepareCodingWorkspace` and Kernel
`desktop.prepare_coding_workspace`. Release Hold remains active for Product
Proof launch — Owner session only.

| Candidate | Status |
| --- | --- |
| C-PROC-002 | **IMPLEMENTED (eng)** — await Owner Product Proof |
| C-REL-002 | **BLOCKED** — B-EXT-001 (Authenticode cert) |
| C-ACT-011 | **BLOCKED** — B-PP-001 / B-CAP-001 (Voice Accept) |

**Recommended Owner action for C-PROC-002:** Launch Workspace once and run the
§C-PROC-002.8 Product Proof session. Do not auto-restart Workspace.

Parallel: Owner Product Proof for C-OBS-003/004, C-ACT-004/005, C-VER-003, C-VER-002; Authenticode → A2.

---

## 10. Engineering Operating Procedure

```text
LOOP:
  Capability Atlas
    → Highest executable (not Trusted/Production/Rejected)
    → If Atlas Pair Rule (§0.1) qualifies → one coordinated pair slice
      else → one capability slice
    → Verification (shared verifiers OK for pairs)
    → Product Proof (joint when pair rule applies)
    → Update separate Readiness Scores / Atlas records
    → Commit
    → STOP
```

---

## 11. Atlas maintenance checklist (per program)

- [x] C-PROC-002 deterministic procedure body complete
- [x] Pre-implementation audit defects corrected in Atlas contract
- [x] C-PROC-002 **BLOCKED → PLANNED → IMPLEMENTED (eng)**; B-DEF-001 resolved
- [x] Readiness Score ~57% (Arch/Deps/Impl/Ver; PP open)
- [x] Definition + runtime verifiers wired into `pnpm test`
- [x] Priority queue / highest-planned candidate synchronized
- [x] Owner authorized C-PROC-002 runtime implementation
- [x] C-PROC-002 runtime implementation / engineering verification
- [ ] C-PROC-002 Owner Product Proof / Trusted / Production
- [ ] Owner Product Proof for C-OBS-003 / C-OBS-004 / C-ACT-004+005 / C-VER-003 / C-VER-002  

---

## 12. Explicit non-goals

- Replacing Conversation, Kernel Operator, or Moments  
- Adopting VLM computer-use runtimes  
- Spec / constitutional redesign via Atlas  
- Adding default C-PROC-002 applications, controls, files, layout, or app sets
- Turning procedures into agent loops, OCR, or multi-app workflows without Atlas authority  

---

## 13. Legacy ID map (Atlas v1.0 → v2.0)

Two-digit IDs are **retired**. Meanings remapped to permanent three-digit IDs (**never reuse** a number for a different meaning).

| Legacy | Meaning | Permanent ID |
| --- | --- | --- |
| C-OBS-01 | Window enumeration | **C-OBS-001** |
| C-OBS-02 | Monitors | **C-OBS-006** |
| C-OBS-03 | Screenshot | **C-OBS-005** (not 003) |
| C-OBS-04 | UIA tree | **C-OBS-003** Desktop UI Tree |
| C-ACT-08 | UIA invoke/set | Split → **C-ACT-004** / **C-ACT-005** |
| C-VER-01 | Completion | **C-VER-001** / **C-CMP-001** |
| C-CMP-03 | Compound | **C-CMP-002** |
| C-INT-* (Intent) | Intent layer | **C-ITL-*** |
| C-INTL-* | Intelligence | **C-INT-*** |

---

## Stop

**Workspace Capability Atlas v2.0** is the authoritative capability roadmap.  
C-PROC-002 Prepare Coding Workspace → **IMPLEMENTED (eng); Owner Product Proof required**.
Do not launch Workspace, run Product Proof, mark Trusted/Production, or begin another capability without a new explicit Owner authorization.
