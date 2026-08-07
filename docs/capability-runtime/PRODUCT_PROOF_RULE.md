# Product Proof Rule
## Permanent engineering completion criterion

| Field | Value |
| --- | --- |
| **Status** | Authoritative (P12.5) |
| **Applies to** | Every Capability Provider execution program |
| **Authority** | Also recorded in `.cursor/rules/constitutional-execution-protocol.mdc` |

---

## Rule

A provider is **not Product Complete** until all of the following are true:

1. The provider exists and is independently testable.
2. It is connected to Conversation through the frozen Capability Runtime pipeline.
3. The Product Owner can naturally discover and use it without menus, dashboards, or documentation.
4. Conversation returns **truthful** success and failure feedback (never invented success).
5. Users never see Provider / Router / Registry terminology.
6. **Natural Language Robustness** (below) is satisfied.
7. **Capability Independence Rule** is satisfied (independent, composable, invisible).

Infrastructure alone is insufficient.

---

## Natural Language Robustness Rule (permanent — P14.5)

A provider is **not Product Complete** merely because commands execute correctly.  
A provider becomes Product Complete only when ordinary users can express the same intent naturally.

Natural language improvements belong **only** inside:

- Intent Layer  
- Kernel Operator (planning / compose)  
- Conversation reply generation  

They do **not** belong inside providers.  
Providers remain deterministic.  
Intent resolution remains deterministic (no probabilistic AI matching).

Future execution programs must satisfy **both**:

- Engineering Completion  
- Product Proof  

before they are considered complete.

---

## User Adaptation Prohibition (permanent — P16.6)

A capability is **not Product Complete** if the user must adapt their behaviour to accommodate implementation details.

Product Complete means the software naturally guides the user through successful interaction without requiring:

- hidden timing or sequencing tricks  
- workarounds  
- memorized commands or phrasing  
- undocumented operating system knowledge  
- precise capitalization  
- rigid wording  
- undocumented behaviour  

**The software adapts to the user. The user never adapts to the software.**

This principle sits alongside Natural Language Robustness and the Product Proof philosophy.  
Unsupported requests must still be **truthful**, with helpful nearby guidance — never repetitive mechanical fallbacks or invented success.

---

## Commodity Before Reinvention (permanent — P16.7)

Workspace owns identity, contracts, permissions, audit, Operator, and runtime authority.

Before implementing substantial new capability, research mature open-source / platform implementations and classify each candidate:

| Class | Meaning |
| --- | --- |
| **ADOPT** | Use directly under Workspace contracts |
| **WRAP** | Own the port; wrap a proven engine |
| **ADAPT** | Fork / reshape while keeping Workspace authority |
| **STUDY** | Learn; do not ship yet |
| **REJECT** | Incompatible with constitution or product identity |

Prefer wrapping proven components behind Workspace contracts rather than rebuilding commodity functionality.  
Never expose commodity implementation details (vendor APIs, HRESULT, engine names) on the product surface.

---

## Conversation Continuity (permanent — P16.8)

Voice behaves like typing. A listening session belongs to the user.  
Do not terminate recognition merely because an arbitrary short timeout elapsed while the user is still speaking.  
Listening ends only when the user clearly finishes speaking, the user explicitly stops, or a genuine recognition error occurs. Never interrupt an active speaker.

## Semantic Alias Rule (permanent — P16.8)

Deterministic semantic aliases are owned by the Kernel Operator / Intent Layer (examples: GPT→ChatGPT, Git→GitHub, YT→YouTube, VSCode→Visual Studio Code, Edge→Microsoft Edge, Chrome→Google Chrome, Settings→Windows Settings).  
Providers remain unaware of aliases. Resolution stays deterministic — no AI guessing.

## Permission Guidance Principle (permanent — P16.9)

OS permissions belong to the OS. Workspace detects, explains, guides, verifies, and remembers — it never replaces Windows dialogs and never repeatedly opens Settings after a successful grant.

## Engineering Verification Separation (permanent — P16.10)

Engineering verifies engineering. Product Proof verifies products.  
Engineering may declare **Engineering Complete**. Only the Product Owner may declare **Product Complete**.

## Owner Directed Product Proof (permanent — P16.10)

Launch Workspace once for Product Proof. Do not auto-restart, relaunch, terminate, steal focus, or reopen Settings.  
Wait for Owner completion; cleanup only after a normal exit.

## Evidence Before Commitment (permanent — P16.11)

Before Workspace permanently adopts a major technology: research mature production implementations; understand the complete workflow (startup, capture, buffering, permissions, threading, cleanup, failure recovery); verify licensing; document architectural tradeoffs; then classify ADOPT / WRAP / ADAPT / STUDY / REJECT.  
Do not commit the product surface to a stack on partial evidence.

## Technology Foundation Validation (permanent — P16.12)

Before Workspace permanently adopts any foundational technology (speech, OCR, automation, memory, terminal, permissions, etc.), engineering must: research mature implementations; understand the complete lifecycle; evaluate licensing, architecture, and operational behaviour; document why Workspace chooses its implementation; then classify ADOPT / WRAP / ADAPT / STUDY / REJECT.  
Permission UX must match modern desktop software: request once, remember success, never spam Settings on later launches.

## Capability Regression Prevention (permanent — P16.13)

A previously working Product Proof path must not regress without a measured root cause and a verifier that prevents recurrence. Symptom-only patches are forbidden.

## Engineering Completion Gate (permanent — P16.13)

Engineering Complete means engineering validation is finished. It does **not** authorize repeated Product Proof launches.  
After engineering validation: engineering stops; Workspace remains closed; the Product Owner alone decides when to launch Workspace.  
Engineering must never repeatedly launch, relaunch, terminate, or restart Workspace during Product Proof.

## Production Before Expansion (permanent — P16.15)

No additional capability may be introduced while an existing capability still fails Product Proof.  
Perfect the open capability first. Expansion waits for Owner Product Complete (or an explicit Owner waiver).  
Engineering confidence is not Product Completion. Objective Product Proof is Product Completion.

## Evidence Before Modification (permanent — P16.16)

Every engineering modification begins with observed behaviour and measured evidence.  
Do not change code to satisfy intuition, polish pressure, or speculative completeness.  
If evidence cannot prove a defect, do not invent a fix.

## Root Cause Before Rewrite (permanent — P16.16)

Corrections follow: Observed behaviour → Measured evidence → Root cause → Minimal correction → Regression verification → Product Proof verification.  
Symptom-only patches and speculative rewrites are forbidden. Prefer the smallest change that removes the proven root cause.

## Production Quality Includes Repository Quality (permanent — P16.17)

A capability is not Production Complete while avoidable compiler warnings, dead code, stale experiments, unnecessary lifecycle complexity, undocumented technical debt, or architectural ambiguity remain in the capability’s owned surfaces.  
Production quality is measured by **both** runtime behaviour **and** repository health.

## Evidence Before Completion (permanent — P16.17)

Engineering Complete may only be declared after engineering has actively attempted to falsify its own implementation.  
Completion is evidence-based, not optimism-based. Engineering must attempt to prove itself wrong before declaring success.

## Repository Health Before Milestone Closure (permanent — P16.17)

A milestone may not permanently close while avoidable technical debt remains that materially affects maintainability, determinism, reliability, or future engineering confidence for that milestone’s owned surfaces.

## Commodity Before Reinvention — survey obligation

Before major capability work, perform a Commodity Survey (ADOPT / WRAP / ADAPT / STUDY / REJECT) and record it under capability research. Workspace owns identity, contracts, Operator, permissions, conversation, and runtime; commodity stays an implementation detail.

---

## Frozen pipeline (no shortcuts)

```
Conversation
    ↓
Intent Layer → CapabilityIntent
    ↓
execute_capability_intent
    ↓
Kernel Operator
    ↓
Capability Runtime → Capability Router
    ↓
Provider Registry
    ↓
Capability Provider
    ↓
Desktop Service
    ↓
Kernel Operator (response)
    ↓
Conversation Response
```

---

## Required Product Proof harness (every provider)

Each provider ships a harness under `docs/capability-runtime/product-proof/`:

| Artifact | Purpose |
| --- | --- |
| `{provider}.proof.json` | Machine-checkable utterances → expected intent kinds |
| `{PROVIDER}_PRODUCT_PROOF.md` | Owner review checklist + failure / permission notes |

Harness contents (minimum):

- Conversation examples  
- Expected routing (intent kind → operation)  
- Expected execution surface (IPC)  
- Expected response character (truthful, user language)  
- Failure examples  
- Permission notes  
- Validation examples (wired into Vitest / verifier)

---

## Ownership

| Concern | Owner |
| --- | --- |
| Language understanding | Conversation / Intent Layer |
| Execution | Capability Provider |
| Permissions / audit | Capability Runtime (unchanged) |
| Product Proof acceptance | Product Owner experience |

Providers never call each other. Conversation never bypasses the runtime.
