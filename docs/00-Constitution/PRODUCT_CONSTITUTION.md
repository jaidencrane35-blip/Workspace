# Workspace Product Constitution

| Field | Value |
| --- | --- |
| **Status** | Draft for Project Owner acceptance — product identity & experience law |
| **Version** | 0.1 |
| **Date** | 2026-08-07 |
| **Owner** | Project Owner |
| **Relationship** | Companion to `PROJECT-CONSTITUTION.md` (values/governance) and subordinate to `architecture/ARCHITECTURAL_CONSTITUTION_V2.md` for engineering architecture |
| **Does not repeal** | Architectural Constitution V2; Constitutional Execution Protocol; AI permission sequence |
| **Amendment** | Project Owner approval + Decision Log entry |

---

## Preamble

This Product Constitution defines **how Workspace should feel and be discovered**.

The Architectural Constitution defines **what Workspace may become structurally** (one pipeline, one permission model, one OS authority, honesty, local-first, no ambient capture by default).

When experience ambition and architectural law appear to conflict, **architectural law wins until this document and Architectural Constitution V2 are formally reconciled**. Product identity may evolve in presentation; it may not silently disable trust machinery.

---

## 1. Product identity

### Workspace is

A **desktop operating layer** whose **primary interface is conversational**.

| Layer | Role |
| --- | --- |
| **Conversation** | The interface |
| **Desktop operation** | The capability |
| **Trust** | The product |

The user operates Windows *through* Workspace by speaking intent. Workspace acts on the desktop only with permission and truthfulness.

### Workspace is not

| Forbidden product shape | Why |
| --- | --- |
| A chatbot | Chat is chrome for operation, not the destination |
| A menu system / feature catalogue | Capabilities are not advertised |
| A desktop launcher dock of apps | Launch may occur as an outcome of intent, not as the IA |
| An automation dashboard | Automation is earned, never showroomed |
| An IDE or agent cockpit | Wrong category |
| Ambient surveillance | Learning requires consent and inspectability |

### Irreplaceable value

> I ask Workspace to help with my desktop. It understands only what it has earned the right to know. It acts only when I allow it. It tells me the truth when it cannot.

Trusted interruption recovery (Save / restore) remains a **core capability** expressed through conversation and desktop effects — not a separate “proof app” with five equal tabs competing for identity.

---

## 2. Immutable product principles

### P1 — Conversation is the interface

The default experience is a compact conversational panel. The conversation *is* home. Surfaces may expand around it; the chat never disappears as the primary locus of intent.

### P2 — Trust before intelligence

Reliable, permissioned action beats clever inference. Prefer a correct “I don’t know” over a fluent invention.

### P3 — Capabilities emerge through use

Never advertise a capability menu, suggested-prompt carousel, or “what I can do” catalogue. Discovery is natural language and successful outcomes.

### P4 — The desktop remains primary

Windows and the user’s real apps are the stage. Workspace is a companion layer — calm, compact, often nearly invisible (floating icon), never a full-screen replacement OS.

### P5 — AI supports; AI does not dominate

Models interpret and propose. They do not own authority, identity, or the permission ceiling. No vendor, model picker, or chat personality is the hero of the product.

### P6 — The user remains in control

Every meaningful desktop mutation and every widening of observation requires an understandable permission moment. The user may reverse learning and delete memory.

### P7 — Workspace earns trust through reliable action

Trust accumulates from kept promises: correct opens, honest restores, refused overreach, inspectable memory. Not from onboarding theatre.

### P8 — Calm, invisible, predictable

Never overwhelm. Never interrupt without purpose. Prefer silence to noise. Prefer stable behaviour to novelty.

### P9 — Learn silently, gradually, truthfully

Learning comes from observation under consent and from confirmed user actions — local-first, inspectable, reversible. Never fabricate understanding. Never pretend to know.

### P10 — Workspace never asks the user to learn Workspace

No onboarding cards, feature tours, or capability quizzes. The user discovers by asking. Workspace adapts to the user; the user does not study a syllabus.

### P11 — User Adaptation Prohibition (permanent — P16.6)

A capability is not Product Complete if the user must adapt their behaviour to accommodate implementation details.  
Product Complete means Workspace guides successful interaction without hidden timing, workarounds, memorized commands, undocumented OS knowledge, precise capitalization, or rigid wording.  
**The software adapts to the user. The user never adapts to the software.**  
Authority detail: `docs/capability-runtime/PRODUCT_PROOF_RULE.md`.

### P12 — Commodity Before Reinvention (permanent — P16.7)

Workspace owns identity, contracts, permissions, audit, Operator, and runtime authority.  
Before substantial new capability work, research mature implementations and classify ADOPT / WRAP / ADAPT / STUDY / REJECT. Prefer wrapping proven components behind Workspace contracts. Never expose commodity internals on the product surface.  
Authority detail: `docs/capability-runtime/PRODUCT_PROOF_RULE.md`.

### P13 — Conversation Continuity (permanent — P16.8)

Voice behaves like typing. A listening session belongs to the user.  
Workspace must never terminate recognition merely because an arbitrary short timeout elapsed while the user is still speaking.  
Listening ends only when the user clearly finishes speaking, the user explicitly stops, or a genuine recognition error occurs. Never interrupt an active speaker.

### P14 — Semantic Alias Rule (permanent — P16.8)

The Kernel Operator (via the Intent Layer) owns deterministic semantic aliases (GPT→ChatGPT, Git→GitHub, YT→YouTube, VSCode→Visual Studio Code, Edge→Microsoft Edge, Chrome→Google Chrome, Settings→Windows Settings).  
Providers remain unaware of aliases. No probabilistic guessing.

### P15 — Permission Guidance Principle (permanent — P16.9)

Operating system permissions belong to the operating system. Workspace never replaces Windows permission dialogs.  
Workspace detects permission state, explains it, guides the user, verifies success, and remembers completed permission flows.  
Workspace never repeatedly opens Settings once permissions are correctly configured.

---

## 3. Presentation modes (binding intent)

| Mode | Form | Role |
| --- | --- | --- |
| **1 — Presence** | Floating icon, movable | Always available; minimal |
| **2 — Converse** | Compact panel (~calculator-sized) | Default working interface; chat input anchored bottom; minimal chrome |
| **3 — Expand** | Workspace grows around conversation | Additional surfaces appear; **conversation remains visible** |

Mode transitions must feel like the same companion changing size — not navigating to a different product.

---

## 4. Permission & honesty (product expression of engineering law)

Product UX must make these felt, not optional:

1. **Ask** when an action is irreversible or crosses a trust boundary.  
2. **Explain** permissions in plain language when required — not via a capabilities menu.  
3. **Admit limits** (cannot restore closed apps, cannot invent yesterday, will not watch ambiently without consent).  
4. **Audit** meaningful authority decisions (engineering duty; product may expose inspect later).

AI governance sequence remains binding:

```
Observe → Learn → Suggest → Receive Permission → Automate
```

---

## 5. Silent learning bounds

Allowed learning inputs (when constitutionally permitted):

- Confirmed user actions in conversation (“Open Cursor.” → success/failure).  
- Explicit Remember / Save intents.  
- Consented observation scopes (e.g. Save Moment capture).  
- User-editable preferences and inspectable memory entries.

Forbidden by default:

- Ambient continuous desktop surveillance.  
- Invented goals, emotions, or “you seem to be…”.  
- Training narratives that imply cloud exfiltration of desktop contents.  
- Learning that cannot be inspected or undone.

---

## 6. Relationship to prior product docs

| Document | Status after this Constitution |
| --- | --- |
| `docs/product-proof-refoundation.md` | Superseded as **primary product direction** where it conflicts (e.g. five-tab proof harness as default identity; AI deferred as absolute). Recovery loop remains a core *capability*. |
| Experience chrome Home/Save/Continue/Check-in/Guide | Becomes **surfaces reachable through conversation and Mode 3 expansion**, not the default home IA |
| Architectural Constitution V2 | Unchanged; still highest engineering architecture authority |

---

## 7. Success feelings

**User**

- “I just asked, and my desktop did the right thing.”  
- “It didn’t do anything shady.”  
- “It remembered only what I allowed.”  
- “It got out of my way.”

**Product team**

- Conversation-first default ships without a feature catalogue.  
- Expansion never orphans the chat.  
- Every demo starts with a sentence, not a menu.

---

## 8. Prohibitions

- Shipping a default experience that is primarily tabs, dashboards, or capability lists.  
- Suggested-prompt walls or “What would you like to do?” empty states.  
- Copying ChatGPT (or any vendor) branding, colours, layout systems, or wording.  
- Treating conversation as a chatbot destination with no desktop effect path.  
- Enabling ambient learning to “feel smart” without consent machinery.  
- Amending Architectural Constitution by implication in UI copy.

---

## 9. Acceptance

This draft becomes binding product law only when the Project Owner accepts it (Decision Log). Until acceptance, it guides design definition documents only — **no implementation mandate**.
