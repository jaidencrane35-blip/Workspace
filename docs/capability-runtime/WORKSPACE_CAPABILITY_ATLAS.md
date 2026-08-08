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
| **Status** | **PLANNED** |
| **Lifecycle** | Planned (partial truthful sanitize exists) |
| **Dependencies** | C-VER-001; control actions |
| **Verification** | Planned with C-ACT-004/005 |
| **Product Proof** | With UI action slice |
| **Priority** | P1 |
| **Engineering Notes** | Bounded retry — not silent loops |

#### C-VER-003 Wait Conditions
| | |
| --- | --- |
| **Name** | Wait Conditions |
| **Layer** | L6 Verification |
| **Status** | **PLANNED** |
| **Lifecycle** | Planned |
| **Dependencies** | C-ACT-004 / C-ACT-005 |
| **Verification** | Planned |
| **Product Proof** | With UI action slice |
| **Priority** | P0 (with click/type) |
| **Engineering Notes** | DesktopCtl-style wait/verify ADAPT |

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
| **Status** | **PLANNED** (partial via situation goals) |
| **Lifecycle** | Partial eng via situationGoals |
| **Dependencies** | C-ACT-001; C-OBS-001; preferably control surface |
| **Verification** | situation goal tests |
| **Product Proof** | Required for full procedure |
| **Priority** | P2 |
| **Engineering Notes** | Expand after C-ACT-004/005 |

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
  C-OBS-003/004 + C-ACT-004/005 (eng done → PP) ──► C-VER-003 Wait
                                                       └─► C-PROC / C-WF (later)

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
| **B-PLAT-001** | Windows-only product | Low | All desktop | Accepted scope |

---

## 6. Verification Matrix

| Capability class | Implementation | Regression | Product Proof |
| --- | --- | --- | --- |
| Desktop UI Tree | UIA port + Window Provider | `verify-desktop-ui-tree` | Owner P22.S2 |
| Window Control Discovery | `find_control` + Intent | `verify-window-control-discovery` | Owner P22.S3 |
| Desktop Control Interaction | Invoke/SetValue + Operator compose | `verify-desktop-control-interaction` | Owner P22.S4 (joint) |
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
| C-VER-002 / C-VER-003 | Retry / Wait | PLANNED | ~29% |
| C-CMP-002..004 | Composition | IMPLEMENTED | ~71% |
| C-PROC-* / C-WF-* | Procedures / Workflows | Mixed | see records |
| C-INT-001..003 | Intelligence | IMPLEMENTED | ~57–71% |
| C-INT-004..005 | Memory / Agent | REJECTED | 0% |
| C-REL-001 | F1 Pipeline | IMPLEMENTED | ~71% |
| C-REL-002..003 | Signing / Signed | BLOCKED | ~14% |

---

## 8. Priority Queue

| Rank | ID | Capability | Status | Why |
| --- | --- | --- | --- | --- |
| **1** | **C-VER-003** | Wait Conditions | **PLANNED** | Strengthens click/type verify |
| 2 | C-REL-002 | Code Signing A2 | BLOCKED (cert) | Parallel release track |
| 3 | C-ACT-011 | File Provider | BLOCKED (Voice) | Production Before Expansion |
| 4 | C-OBS-007 | Tokenized perception | FUTURE | After interaction PP |
| 5 | C-REA-004 | In-Conversation Model | FUTURE | Research |
| 6 | C-PROC-002 / C-WF-001 | Coding procedures | FUTURE | After interaction Trusted |

**Rejected (do not queue):** C-ACT-013; C-INT-004; C-INT-005.

**Just completed eng:** C-ACT-004 + C-ACT-005 Desktop Control Interaction pair → **joint Product Proof** required.

---

## 9. Current highest remaining executable capability

### **C-VER-003 Wait Conditions** (next executable after interaction eng)

| Factor | Assessment |
| --- | --- |
| Owner Value | High (stronger post-click/type certainty) |
| Dependencies | C-ACT-004 / C-ACT-005 eng complete |
| Effort | S–M |
| Risk | Med |
| Pair note | Complements interaction; not merged into C-ACT IDs |

**Do not begin in the same session that completed C-ACT-004/005.**

Parallel: Owner Product Proof for C-OBS-003/004 and C-ACT-004/005; Authenticode → A2.

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

- [x] Capability rows updated (C-ACT-004 + C-ACT-005)  
- [x] Separate Readiness Scores (both 57%)  
- [x] Shared verification + joint Product Proof doc  
- [x] Blockers register (B-PP-005)  
- [x] Priority queue re-ranked  
- [ ] Owner Product Proof for C-OBS-003 / C-OBS-004 / C-ACT-004+005  

---

## 12. Explicit non-goals

- Replacing Conversation, Kernel Operator, or Moments  
- Adopting VLM computer-use runtimes  
- Spec / constitutional redesign via Atlas  
- Starting C-VER-003 or procedures in the same slice as C-ACT-004/005  

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
C-ACT-004 + C-ACT-005 engineering pair complete → **joint Product Proof required**.  
Do not begin C-VER-003 or procedures in this iteration.
