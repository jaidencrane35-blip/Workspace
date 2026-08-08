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
| **Lifecycle** | Engineering complete |
| **Dependencies** | C-REA-001 |
| **Verification** | intelligence-routing tests |
| **Product Proof** | Covered by P22.S1 |
| **Priority** | — |
| **Engineering Notes** | No world knowledge invent |

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
| **Verification** | Window product-proof tests; batteries |
| **Product Proof** | “What windows are open?” live path |
| **Priority** | — |
| **Engineering Notes** | Title match — not tab/DOM |

#### C-OBS-002 Active Window
| | |
| --- | --- |
| **Name** | Active Window |
| **Layer** | L4 Observation |
| **Status** | **IMPLEMENTED** |
| **Lifecycle** | Engineering complete |
| **Dependencies** | Window Provider `active` |
| **Verification** | Window provider / intent map `winActive` |
| **Product Proof** | Covered by window observation paths |
| **Priority** | — |
| **Engineering Notes** | Foreground window truth |

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
| **Status** | **PLANNED** |
| **Lifecycle** | Audited → Planned — definition complete; implementation not started |
| **Dependencies** | C-ITL-002/003/004; C-OBS-001/002; C-ACT-001; C-VER-001/002/003; C-CMP-001 |
| **Verification** | `verify-prepare-coding-workspace-definition` (definition only); future runtime composition + regression verification required |
| **Product Proof** | Defined in §C-PROC-002.7; **not run — Owner session required after implementation** |
| **Priority** | P0 (next Owner-authorized Atlas implementation slice) |
| **Engineering Notes** | Authoritative procedure contract below. Existing `situationGoals` → Continue/Moments remains the lawful interim path. This definition changes no runtime behaviour and creates no default coding app set. |
| **Readiness** | Overall **~31%** (definition + dependencies; implementation partial handoff only; runtime verification/PP/Trusted/Production 0%) |
| | ```text
Architecture ............ 100%  (deterministic procedure contract)
Dependencies ............ 100%  (required capability primitives eng-complete)
Implementation ..........  20%  (situationGoals Continue handoff only)
Verification ............   0%  (definition verifier is not runtime verification)
Product Proof ...........   0%
Trusted .................   0%
Production ..............   0%
Overall ................. ~31%
``` |

##### C-PROC-002.1 Scope and completion

`C-PROC-002` prepares a finite, non-empty, Owner-selected ordered set of
application targets. For each target, Workspace determines whether a matching
top-level window is already present, opens or focuses that exact target, waits
for observable window availability, and finally verifies that every requested
target is present and the final requested target is active.

The procedure **never chooses what a coding workspace contains**. There is no
default coding application, application set, layout, file, folder, terminal
command, project, control, or saved Moment. Target order is the Owner's order;
the Operator does not add, remove, substitute, or reorder targets.

Completion uses the existing Completion Contract:

- **completed** — every resolved target has an observed top-level window and
  the final requested target is observed active;
- **partial** — at least one target reached that state, but one or more did
  not, or final focus could not be verified;
- **failed** — no target reached the required state;
- **clarification required** — resolution failed before effects; no procedure
  step executes and this is not upgraded to partial or completed.

##### C-PROC-002.2 Named-target resolution

All target references are resolved atomically before the first desktop effect.
If any reference is missing or ambiguous, Workspace asks for clarification and
opens or focuses **nothing**.

| Resolution class | Authoritative rule | Procedure result |
| --- | --- | --- |
| **Explicitly named target** | An application reference supplied in the current Owner request. It must resolve deterministically through existing Intent aliases/entities and to an existing application launch/focus target. | Include the canonical target once, in Owner-supplied order. |
| **Existing known target** | A canonical application target already held in current Workspace Context and unambiguously referenced by the Owner in this request (for example, a bound “that app” or “again”). Registry knowledge, installation, or a running window alone does **not** authorize selection. | Include only the exact context-bound target. |
| **Ambiguous target** | A reference has more than one plausible application/context match, or application and window evidence disagree. | **CLARIFICATION**; name the alternatives in user language; no effect. |
| **Missing target** | The request contains no application target, a context reference is unbound, or a name has no authoritative resolver/launch mapping. | **CLARIFICATION**; ask which application(s); no effect. |

A saved Moment is not an “existing known target” for this procedure. Generic
setup/coding phrasing may continue to open Continue under C-PROC-003. Restore
remains the separate C-WF-002 path and requires the existing preview plus
**Approve and restore** authorization. C-PROC-002 never selects or restores a
Moment automatically.

##### C-PROC-002.3 Deterministic step table

Steps `PCW-002` through `PCW-005` run once per resolved target, sequentially in
Owner-supplied order. The target list is finite, and retry never changes the
target or requested effect.

| Step ID | Action | Required Capability | Target | Preconditions | Observable Success Condition | Timeout | Retry Policy | Failure Outcome |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| **PCW-001** | Resolve and canonicalize the complete ordered target set. | C-ITL-002 Semantic Intent; C-ITL-003 Workspace Context; C-ITL-004 Goal Resolution | Every application reference in the current Owner request | Request maps to Prepare Coding Workspace; no desktop effect has begun | A non-empty finite ordered set exists; every item records explicit-name or context-bound evidence and maps to one canonical launch/focus target | Single deterministic pass; no polling or wait | **Not permitted.** Missing/ambiguous information is not transient. | **CLARIFICATION**; no target effect executes. |
| **PCW-002** | Enumerate top-level windows and record whether the current target is already present. | C-OBS-001 Enumerate Windows | Current canonical target | PCW-001 completed for the entire set | Truthful `present` or `not present` fact for the exact target; `not present` is the lawful launch branch, not invented failure | 2s operation deadline; one observation pass | **Not permitted.** | Observation error/timeout: stop this target; final outcome **failed** or **partial** according to completed targets. |
| **PCW-003** | Ensure the target is available: focus the exact running target, otherwise launch the exact resolved target (`app.open_or_focus`). | C-ACT-001 Launch Application (existing focus-or-launch composition) | Current canonical target only | PCW-002 produced an availability fact; effect remains authorized for this Owner turn | Provider/OS fact identifies focus or launch acceptance for the exact target; this is provisional until PCW-004 | 2s effect-response deadline; launch readiness is measured by PCW-004 | No immediate loop. **One re-attempt only through PCW-005** after a retryable PCW-004 miss. | Permission denial, unsupported target, access denial, focus refusal, or interaction failure: stop immediately; no retry; final **failed/partial**. |
| **PCW-004** | Wait for and observe the exact target window. | C-VER-003 Wait Conditions (`window_available`) | Exact current target window | PCW-003 returned, or PCW-002 found the target present | `condition_met` plus an observed matching top-level window | 8s maximum, 50ms polling | On `condition_timeout` or post-action `not_found`, proceed to PCW-005 once. No retry for policy/auth/unsupported/interaction failures. | Retryable miss → PCW-005. Otherwise stop target and aggregate **failed/partial**. |
| **PCW-005** | Apply one bounded same-target re-attempt: wait 400ms, repeat PCW-003 once, then repeat PCW-004 once. | C-VER-002 Retry; C-VER-003 Wait Conditions; C-ACT-001 | Same canonical target and same open/focus effect only | First PCW-004 ended with retryable `condition_timeout` or post-action `not_found`; original Owner authorization still applies | Second PCW-004 returns `condition_met` for the same target | 400ms pause + 2s effect response + 8s verification (**10.4s maximum**) | **No further retry. Maximum two action attempts total.** | **Retry exhaustion**; retain prior completed targets and aggregate **failed/partial** truthfully. |
| **PCW-006** | Re-enumerate requested targets, observe the active window, and compose the final result. | C-OBS-001 Enumerate Windows; C-OBS-002 Active Window; C-VER-001 Action Verification; C-VER-003 (`window_active`); C-CMP-001 Completion Contract | Entire resolved target set; final requested target for active-window check | Every target has completed or stopped; no retries remain | Every requested target window is observed and the final requested target is observed active; result lists per-target facts | 2s final active-window wait, 50ms polling; one final enumeration | **Not permitted.** Per-target retry boundaries are already exhausted. | Return **completed**, **partial**, or **failed** from observed facts; never upgrade missing evidence to success. |

##### C-PROC-002.4 Authorization

| Step | Authorization classification | Rule |
| --- | --- | --- |
| PCW-001 | **No additional authorization** | Resolution is Meaning/Plan only; it produces no Effect. |
| PCW-002 | **No additional authorization** | Read-only observation still follows the Kernel/Runtime permission path. |
| PCW-003 | **Existing Owner confirmation** | The current request explicitly names or unambiguously references each target. The Kernel Permission Gateway remains mandatory at effect time. |
| PCW-004 | **No additional authorization** | Bounded observation of the already-authorized target. |
| PCW-005 | **Existing Owner confirmation** | Covers only the same target and same effect once. Permission denial is never retried around. |
| PCW-006 | **No additional authorization** | Observation and truthful completion composition only. |
| Separate C-WF-002 restore | **Existing Moments authorization** | Preview plus Owner **Approve and restore** with plan-digest binding; not a C-PROC-002 step. |

No procedure-wide approval token is invented. Every Effect remains
Conversation → Intent → `execute_capability_intent` → Kernel Operator →
Permission Gateway → Runtime. Providers never call each other.

##### C-PROC-002.5 Failure rules

| Failure | Deterministic behaviour | Retry | Final procedure result |
| --- | --- | --- | --- |
| Missing target | Ask which application(s) the Owner wants; execute nothing. | No | **CLARIFICATION** |
| Ambiguous target | Present the concrete alternatives; execute nothing. | No | **CLARIFICATION** |
| Application not open | Treat as PCW-002 `not present`; launch the exact resolved target through PCW-003. | Only if later verification times out | Continue |
| Application cannot be located after launch/focus | Wait up to 8s, then one same-target re-attempt under PCW-005. | Once | **failed/partial** after exhaustion |
| Control cannot be located | Controls are outside this procedure contract. Do not guess a control or add a click/type step; ask for a separate named control action if needed. | No C-PROC-002 retry | **CLARIFICATION** for the out-of-contract request; existing prepared targets remain truthfully reported |
| Interaction failure | Report the exact target failure in user language; retain earlier completed targets. | No | **failed/partial** |
| Verification timeout | Enter PCW-005 only for the same target/effect. | Once | Continue or retry exhaustion |
| Retry exhaustion | Stop; do not substitute another application or continue retrying. | No | **failed/partial** |
| Permission/authorization denial | Report denial; do not retry around policy. | No | **failed/partial** |

##### C-PROC-002.6 Retry boundary

C-VER-002 supplies the permanent bound: original attempt plus one re-attempt
(`MAX_INTERACTION_ATTEMPTS = 2`) with a 400ms C-VER-003 pause. For this
procedure, the retry boundary is exactly one canonical application target and
the same `app.open_or_focus` effect. Retry may follow only a verification
`condition_timeout` or post-action `not_found`.

The current C-VER-002 runtime wiring is click/type-only. Future C-PROC-002
implementation must reuse its policy and add the application composition
without changing the bound; this definition does **not** claim that runtime
wiring already exists.

##### C-PROC-002.7 Future Owner Product Proof

**Status:** Definition complete; not executed; not accepted.

| Field | Required future proof |
| --- | --- |
| **Exact Owner input (success)** | `Prepare my coding workspace with Notepad and Calculator.` The application names are explicit proof targets, not product defaults. |
| **Starting state** | Workspace is launched once by Owner for Product Proof; Conversation is ready; Notepad and Calculator are closed; no saved Moment is selected; no prior target context is required. |
| **Expected visible outcome** | Notepad and Calculator become visibly open; Calculator (the final Owner-named target) is active; Conversation reports completed only after both windows and final focus are observed. |
| **Missing-target case** | `Prepare my coding workspace.` → ask which applications; launch/focus nothing. |
| **Unknown-target case** | `Prepare my coding workspace with Notepad and NoSuchCodingAppZZZ.` → resolve the entire set first, ask for clarification, and do not open Notepad. |
| **Ambiguous-target case** | With more than one context candidate, `Prepare my coding workspace with that app.` → name the alternatives and execute nothing. |
| **Application-already-open case** | Start Notepad first, then use the success input → focus/reuse Notepad rather than duplicate it; Calculator may launch. |
| **Failure/timeout case** | If a resolved target refuses interaction or never presents a window, stop after the defined bound and identify that target; never claim completed. |
| **Partial-completion behaviour** | If the first of two fully resolved targets is observed ready and the second later fails, retain the first effect and report partial with one-of-two target facts and recovery guidance. |
| **Success criteria** | Exact target fidelity; no added/substituted app; correct input order; finite timing; at most two attempts for one retryable target; all requested windows observed; final target active; truthful completed/partial/failed/clarification response. |

Product Proof must also attempt ordinary variants such as “Set up my coding
space with Notepad and Calculator” without requiring exact capitalization or
memorized wording. Engineering verification must be green first. Only the
Owner may mark Product Proof, Trusted, or Production.

##### C-PROC-002.8 Explicit non-goals

- Selecting a default IDE, editor, browser, terminal, project, or app set.
- Opening files/folders, running terminal commands, typing into controls, or
  clicking controls.
- Choosing, approving, or restoring a saved Moment.
- Arranging windows unless a separate named layout capability is authorized.
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

  C-ITL-002/003/004 + C-OBS-001/002 + C-ACT-001
       + C-VER-001/002/003 + C-CMP-001
                     └─► C-PROC-002 (**PLANNED**; definition complete)
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
| **B-DEF-001** | **RESOLVED 2026-08-08** — C-PROC-002 now defines target-resolution authority, ordered Step IDs, actions, required capabilities, targets, preconditions, observable success, timeouts, retry boundaries, failure outcomes, authorization, and future Product Proof. | C-PROC-002.1–.8; `verify-prepare-coding-workspace-definition` | C-PROC-002 **BLOCKED → PLANNED**. Implementation, Product Proof, Trusted, and Production remain incomplete. |

---

## 6. Verification Matrix

| Capability class | Implementation | Regression | Product Proof |
| --- | --- | --- | --- |
| Desktop UI Tree | UIA port + Window Provider | `verify-desktop-ui-tree` | Owner P22.S2 |
| Window Control Discovery | `find_control` + Intent | `verify-window-control-discovery` | Owner P22.S3 |
| Desktop Control Interaction | Invoke/SetValue + Operator compose | `verify-desktop-control-interaction` | Owner P22.S4 (joint) |
| Wait Conditions | Bounded WaitCondition + compose | `verify-wait-conditions` | Owner P22.S5 |
| Bounded Retry | Operator retry + Wait reuse | `verify-bounded-retry` | Owner P22.S6 |
| Prepare Coding Workspace definition | Atlas C-PROC-002.1–.8 | `verify-prepare-coding-workspace-definition` | Defined; Owner session only after runtime implementation |
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
| C-PROC-002 Prepare Coding Workspace | **No — definition only** | **Defined; not run; required after implementation** |
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
| C-REA-002 | Local Reasoning | IMPLEMENTED | ~57% |
| C-REA-003 | Provider Handoff | IMPLEMENTED | ~57% |
| C-REA-004 | In-Conversation Model | FUTURE | ~14% |
| C-ITL-001..005 | Intent stack | IMPLEMENTED | ~71% |
| C-OBS-001 | Enumerate Windows | IMPLEMENTED | ~71% |
| C-OBS-002 | Active Window | IMPLEMENTED | ~71% |
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
| **C-PROC-002** | **Prepare Coding Workspace** | **PLANNED** | **~31%** (definition complete; implementation/PP open) |
| C-PROC-* / C-WF-* (other) | Procedures / Workflows | Mixed | see records |
| C-INT-001..003 | Intelligence | IMPLEMENTED | ~57–71% |
| C-INT-004..005 | Memory / Agent | REJECTED | 0% |
| C-REL-001 | F1 Pipeline | IMPLEMENTED | ~71% |
| C-REL-002..003 | Signing / Signed | BLOCKED | ~14% |

---

## 8. Priority Queue

| Rank | ID | Capability | Status | Why |
| --- | --- | --- | --- | --- |
| **1** | **C-PROC-002** | Prepare Coding Workspace | **PLANNED** | Definition complete; next only when Owner authorizes implementation |
| 2 | C-REL-002 | Code Signing A2 | BLOCKED (cert) | Parallel release track |
| 3 | C-ACT-011 | File Provider | BLOCKED (Voice) | Production Before Expansion |
| 4 | C-OBS-007 | Tokenized perception | FUTURE | After interaction PP |
| 5 | C-REA-004 | In-Conversation Model | FUTURE | Research |
| 6 | C-WF-001 | Daily Coding Session | FUTURE | After C-PROC-002 |

**Rejected (do not queue):** C-ACT-013; C-INT-004; C-INT-005.

**Latest definition slice:** B-DEF-001 resolved. C-PROC-002 is **PLANNED**;
runtime implementation has not started. No silent scope expansion.

---

## 9. Current highest remaining executable capability

### **C-PROC-002 — PLANNED (not active)**

C-PROC-002 is now completely defined and its prerequisite capabilities are
engineering-present. It is the highest planned implementation candidate, but
Release Hold remains active: implementation starts only after a new explicit
Owner authorization. This definition slice does not authorize implementation.

| Candidate | Status |
| --- | --- |
| C-PROC-002 | **PLANNED** — definition complete; await Owner implementation authorization |
| C-REL-002 | **BLOCKED** — B-EXT-001 (Authenticode cert) |
| C-ACT-011 | **BLOCKED** — B-PP-001 / B-CAP-001 (Voice Accept) |

**Recommended Owner action for C-PROC-002:** Review C-PROC-002.1–.8 and,
when desired, explicitly authorize one bounded runtime implementation slice.

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
- [x] C-PROC-002 **BLOCKED → PLANNED**; B-DEF-001 resolved
- [x] Readiness Score ~31% (definition/dependencies only)
- [x] Definition verifier wired into `pnpm test`
- [x] Priority queue / highest-planned candidate synchronized
- [ ] Owner authorizes C-PROC-002 runtime implementation
- [ ] C-PROC-002 runtime implementation / engineering verification
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
C-PROC-002 Prepare Coding Workspace → **PLANNED; B-DEF-001 resolved**.
Do not implement C-PROC-002 or begin another capability without a new explicit Owner authorization.
