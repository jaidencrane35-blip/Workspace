# Conversational Desktop Operator
## Product Identity Refactor — Definition Pack

| Field | Value |
| --- | --- |
| **Status** | Product definition — **not implemented** |
| **Date** | 2026-08-07 |
| **Product law (draft)** | `docs/00-Constitution/PRODUCT_CONSTITUTION.md` |
| **Preserved** | Architectural Constitution V2; Constitutional Execution Protocol |
| **Interaction reference** | ChatGPT-class density principles only — no brand/layout/colour/implementation copy |
| **Code** | None in this program |

---

# 1. Product Constitution

See **[`docs/00-Constitution/PRODUCT_CONSTITUTION.md`](00-Constitution/PRODUCT_CONSTITUTION.md)**.

Summary identity:

> Conversation is the interface. Desktop operation is the capability. Trust is the product.

---

# 2. Conversational Interaction Model

## 2.1 Default surface

A **compact conversational panel** (Mode 2):

| Element | Rule |
| --- | --- |
| Primary content | Message transcript (user + Workspace) |
| Input | Single field **anchored at the bottom** |
| Chrome | Minimal — drag, expand/collapse, maybe pin; no nav mega-menu |
| Empty state | Quiet transcript area + input only. **No** onboarding cards, feature menus, capability lists, suggested prompts, or “What would you like to do?” |
| Tone | Calm, short, operational. Prefer outcomes over essays |

## 2.2 Turn types

| Turn | Purpose | Example |
| --- | --- | --- |
| **Intent** | User asks for desktop work | “Open Cursor.” |
| **Clarify** | Workspace asks only when required | “Cursor isn’t running. Open it?” |
| **Permission** | Trust boundary | “Restore will move 4 windows. Continue?” |
| **Action** | Confirmed effect summary | “Opened Cursor.” / “Placed 3 windows; 1 skipped (closed).” |
| **Memory** | Explicit remember / recall | “Remember this.” → confirm what was stored |
| **Truth** | Limits | “I don’t have yesterday’s layout unless we saved it.” |

## 2.3 Interaction principles (adopt; do not clone ChatGPT)

- Streaming or progressive responses may exist later; **reliability > animation**.  
- One job per turn when acting on the desktop.  
- Errors are plain and recoverable (“Couldn’t find that window”).  
- Long explanations are opt-in (“Why?”), not default.

## 2.4 Mapping former Product Proof rituals into conversation

| Former chrome | Conversational expression |
| --- | --- |
| Save | “Save this moment as Northwind.” / “Remember where I am.” → consent scope when capture needed |
| Continue | “Restore Northwind.” / “Restore yesterday.” → plan summary + approve |
| Home | Not a page — **idle conversation** + Mode 1 icon |
| Guide | Answers when asked (“What do you watch?”) — no tour |
| Check-in | Pilot intents only when user asks or owner enables evaluation mode |

## 2.5 Why / constitution / timing

| | |
| --- | --- |
| **Improves product** | Removes proof-harness IA; usefulness starts at first sentence |
| **Aligns** | Product Constitution P1–P3, P10; Arch Const. permission + honesty preserved in turn types |
| **Timing** | Experience refactor **before** expanding AI platform backlog |

---

# 3. Workspace Surface Architecture

Surfaces are **satellites around conversation**, never replacements for it.

```
Mode 1: [Icon]
Mode 2: [ Conversation panel ]
Mode 3: [ Conversation | optional satellites ]
                      ├─ Moment / memory inspector
                      ├─ Restore plan detail
                      ├─ Permission / audit inspect
                      └─ (future) layout / notes — only when earned
```

| Surface | Appears when | Disappears when |
| --- | --- | --- |
| Conversation | Modes 2–3 always | Never in Mode 2–3 (icon-only in Mode 1) |
| Plan detail | User restoring / asking why | After confirm or dismiss |
| Memory inspect | User asks what is remembered / Manage memory | User closes |
| Permission sheet | Action needs approval | Decision recorded |
| Pilot / eval | Explicit eval context | Not default |

**Hard rule:** No Mode 3 layout may cover or route away from the conversation thread as the intent channel.

### Why / constitution / timing

Improves calm focus; aligns P1/P4; implement after interaction model acceptance, before feature satellites proliferate.

---

# 4. Expansion Model

## Mode 1 — Floating icon

- Movable, always available.  
- Click/hotkey → Mode 2.  
- No badge spam; status only for blocking permission waits (rare).

## Mode 2 — Compact conversational panel

- ~calculator-sized default.  
- Resizable within modest bounds.  
- Conversation focus. Minimal chrome.

## Mode 3 — Expanded Workspace

- User expands (corner control / drag).  
- Conversation stays docked (e.g. left or bottom band).  
- Satellites open **in response to conversation context**, not from a capability launcher.  
- Collapse returns to Mode 2 without losing thread.

### Transition principles

| Do | Don’t |
| --- | --- |
| Same companion, different size | Navigate to a “Home app” vs “Chat app” |
| Context-driven panels | Permanent multi-rail IDE |
| Remember last mode gently | Force expanded on every launch |

### Why / constitution / timing

Delivers “invisible → present → deep” without menus; aligns P4/P8; UX shell work is the first implementation tranche after approval.

---

# 5. Silent Learning Model

## 5.1 Definition

**Silent** means: no pedagogical UI, no “I learned something!” toast spam, no quizzes.  
**Not** silent as in secret or ambient-by-default.

## 5.2 Learning channels

| Channel | Allowed when | Examples |
| --- | --- | --- |
| **Confirmed actions** | Always (outcome logged) | App open success; restore partial result |
| **Explicit memory intents** | User says Remember / Save / Forget | “Remember this folder.” |
| **Consented observation** | User approves a scope (Save Moment, etc.) | Window layout under approved scope |
| **Preferences** | User sets or confirms | Default editor, quiet hours |

## 5.3 Candidate pattern store (internal)

May accumulate *candidates* from confirmed actions (app pairs, layouts, folders, clipboard paste targets, time-of-day habits) as **local, inspectable hypotheses** — not as asserted user truth until:

- the user confirms a suggestion, or  
- the pattern is shown in Memory inspect as “seen N times” without claiming motive.

## 5.4 Hard bounds (Architectural Constitution)

| Bound | Rule |
| --- | --- |
| Ambient capture | **Off by default** (Arch Law XII) — continuous watching is not “silent learning” |
| Fabrication | Never invent context or goals |
| Local-first | Learning durable on device unless user opts into future sync |
| Inspectable | User can see and delete learned items |
| Reversible | Forget / reset pathways |
| Permission | Suggestions that automate still hit Gateway |

## 5.5 Why / constitution / timing

Enables “Workspace learns the user” without surveillance product; aligns Product P9 + Arch IV/VII/IX/XII. Engineering for inspectable memory may follow AgentToolGate/Audit work; **do not** enable ambient to fake learning.

---

# 6. Desktop Operator UX

## 6.1 Operator loop

```
User intent (utterance)
  → Interpret (local rules and/or model behind Workspace interface)
  → Resolve targets on desktop (truthful)
  → Permission if needed
  → Effect via CommandPipeline + Win32 boundary
  → Report outcome in conversation
  → Optional learn from confirmed outcome
```

## 6.2 Example intents (discovery, not menu items)

| User says | Operator behaviour |
| --- | --- |
| “Open Cursor.” | Launch/focus if allowed; report |
| “Paste this into Cursor.” | Requires clipboard + target focus; ask if ambiguous |
| “Remember this.” | Clarify *what* → store inspectable memory |
| “Restore yesterday.” | Truth: only if a Moment/session exists; else admit |
| “Search my notes.” | Only over allowed stores; never pretend full disk omniscience |
| “Summarise what I was doing.” | Only from Moments / consented context; else admit |

## 6.3 Visual behaviour

- Panel stays out of the way; does not steal focus except for permission.  
- Desktop changes are the “wow”; the panel narrates briefly.  
- Magical: correct place/focus/open. Predictable: same permission rhythm every time.

### Why / constitution / timing

Makes conversation an operator, not a chatbot; preserves Arch Laws II/III/IV. Implementation after shell modes; reuse existing Save/restore/launch commands behind intents.

---

# 7. Capability Discovery Model

## Policy

**No capability advertising.**

| Forbidden | Replacement |
| --- | --- |
| Capabilities menu | Natural language attempts |
| Suggested prompt chips | Empty quiet input |
| Feature catalogue settings page as home | Memory/permissions inspect only when asked |
| Onboarding “try these 5 things” | First successful desktop action |

## Discovery arc

1. User asks for something possible → success → trust.  
2. User asks for something gated → clear permission → success.  
3. User asks for something impossible / unknown → truthful refusal + optional next step (“Save a moment first”).  
4. Repeated confirmed actions → optional silent candidate → later suggest (with permission).

## Permission explanation

When required, explain **this action**, not the product catalogue:

> “To restore Northwind I need to move windows on your desktop. Allow once?”

### Why / constitution / timing

P3/P10; reduces overwhelm. Enforce in UX from first conversational shell — no eng dependency beyond hiding old chrome.

---

# 8. Human Trust Model

Trust is a **score the product earns**, not a claim in marketing copy.

| Trust deposit | Trust withdrawal |
| --- | --- |
| Correct desktop effect | Acting without ask |
| Honest “I don’t know” | Invented yesterday / fake restore |
| Clear, rare permissions | Permission fatigue / vague asks |
| Inspectable memory | Hidden learning |
| Staying quiet | Interrupting without purpose |
| Stable Mode 1–3 behaviour | Random chrome reshuffles |

### Trust stages (product, not gamification)

1. **Presence** — icon exists; user opens when needed.  
2. **Reliability** — simple opens/focus work every time.  
3. **Recovery** — Save/restore via conversation works with honesty.  
4. **Memory** — user allows Remember; inspects; keeps.  
5. **Delegation** — user accepts rare automations after suggestions.

Never skip stages with a personality or a tour.

### Why / constitution / timing

Trust-before-intelligence (P2/P7); Arch honesty/permission. Roadmap must not jump to stage 5.

---

# 9. Product Refactor Roadmap

| Phase | Outcome | Notes |
| --- | --- | --- |
| **R0 — Accept identity** | Owner accepts Product Constitution | This pack |
| **R1 — Conversational shell** | Modes 1–2; bottom input; no menus/prompts; hide five-tab default as home | Presentation only; keep kernel |
| **R2 — Intent → existing PP commands** | Map open/save/restore/remember utterances to current IPC | No new architecture |
| **R3 — Mode 3 satellites** | Plan/memory/permission around chat | Context-triggered |
| **R4 — Truthful recall language** | “Yesterday” only with evidence | Honesty craft |
| **R5 — Silent candidates** | Inspectable pattern store from confirmed actions | Still no ambient default |
| **R6 — Suggest + automate** | Permissioned suggestions | After AgentToolGate / ModelProvider as needed |
| **R7 — Defer/demote old chrome** | Home/Save/Continue tabs retired as default IA; remain as Mode 3 tools if useful | Do not delete kernel capabilities |

### Ordering vs engineering backlog

| Engineering backlog item | Relative to refactor |
| --- | --- |
| Docs convergence / WorkspaceState naming | Parallel OK; does not deliver identity |
| Domain contracts remainder | Parallel OK |
| AgentToolGate / AuditIntegrity | **Before** R6 autonomous-leaning AI tools |
| ModelProvider | **Before** rich NL interpretation if not rule-based |
| BackgroundWorkerSupervisor | Not required for R1–R4; reject ambient desktop jobs |

**Product priority:** R1–R4 before Phase C platform expansion.

---

# 10. Implementation Strategy

## 10.1 Constraints

- Do **not** amend Architectural Constitution V2 or Execution Protocol in the same breath as UI work.  
- Prefer **adapters**: conversational UI invokes existing Product IPC / CommandPipeline.  
- If NL understanding needs a model, it sits behind Workspace-owned interfaces (Law X).  
- Minor prototypes (paper, static HTML, or a feature-flagged panel) only to communicate Modes 1–3 — optional; **not started here**.

## 10.2 First implementation program (when owner approves)

**Single program recommendation:** *Conversational Shell (Modes 1–2) + intent bridge to Save/Continue/Launch*

Out of scope for that first program: ambient learning, suggested prompts, Mode 3 feature sprawl, ChatGPT visual clone, new automation engine.

## 10.3 Migration of current Experience

| Current | Strategy |
| --- | --- |
| WorkspaceShell + 5 tabs | Feature behind flag or Mode 3; default becomes conversation |
| Experience demo dataset | Reuse as conversational fixtures |
| Pilot Check-in | Reachable by intent / eval mode only |
| OperatorConsole | Remains developer-only |

## 10.4 Validation of success (product)

- Cold open → Mode 2 conversation with **zero** capability advertising.  
- User can cause one real desktop effect via a sentence.  
- User can Save/restore via conversation with permission + honesty.  
- Expand to Mode 3 without losing the thread.  
- Collapse to Mode 1 feels like the same companion.

## 10.5 Risk register

| Risk | Mitigation |
| --- | --- |
| Appears as “just another chatbot” | Lead demos with desktop effects; keep panel compact |
| Conflicts with Arch Law I wording | Treat recovery as core capability; update Arch Const. only via formal amendment if owner wants identity clause rewritten |
| Ambient pressure to “learn faster” | Product Constitution §5 + Law XII — refuse |
| Permission fatigue | Batch asks; remember grants carefully; never nag |

---

# STOP

| Produced | Path |
| --- | --- |
| Product Constitution | `docs/00-Constitution/PRODUCT_CONSTITUTION.md` |
| This definition pack (items 2–10) | `docs/conversational-desktop-operator.md` |

**Not done:** code, architecture redesign, protocol changes, ambient capture enablement.

**Handoff:** Project Owner reviews and accepts/amends the Product Constitution. Only then select **one** implementation execution program (recommended: Conversational Shell R1–R2).
