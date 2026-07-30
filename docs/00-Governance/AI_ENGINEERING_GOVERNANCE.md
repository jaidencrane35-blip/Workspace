# AI Engineering Governance

| Field | Value |
|-------|-------|
| **Purpose** | Permanent control of AI-assisted engineering so acceleration does not sacrifice human understanding, quality, maintainability, or product direction |
| **Owner** | Engineering (binding for agents and human contributors) |
| **Status** | Binding — complete package |
| **Related** | [AGENTS.md](../../AGENTS.md), [Engineering Governance](../03-Engineering/ENGINEERING-GOVERNANCE.md), [Human Review Policy](../03-Engineering/HUMAN-REVIEW-POLICY.md), [Coding Standards](../03-Engineering/CODING-STANDARDS.md), [Engineering Principles](../03-Engineering/ENGINEERING-PRINCIPLES.md), [Visual Direction](../01-Product/WORKSPACE-VISUAL-DIRECTION.md), [Optimisation Log](../04-Operations/OPTIMISATION_LOG.md), [Governance Model](../00-Constitution/GOVERNANCE.md), [AI Principles](../05-AI/AI-PRINCIPLES.md) |

This is the **detailed permanent governance specification** for AI-assisted development.

- [`AGENTS.md`](../../AGENTS.md) — short operational contract for agents  
- [`ENGINEERING-GOVERNANCE.md`](../03-Engineering/ENGINEERING-GOVERNANCE.md) — product alignment, freezes, batch docs  
- [`HUMAN-REVIEW-POLICY.md`](../03-Engineering/HUMAN-REVIEW-POLICY.md) — when humans review visuals/workflows  

Do **not** treat this file as a second constitution. Founder authority remains in `00-Constitution/`.

---

## 1. Purpose

AI tools accelerate implementation. They must not become the source of architecture memory, product direction, or unexplained complexity.

The repository must remain suitable for **professional acquisition, handover, and long-term development**.

Goals:

- human engineers can maintain the project without AI
- architecture remains understandable
- product direction does not drift
- AI does not create unnecessary complexity
- future engineers understand why every system exists

---

## 2. Human Maintainability Scoring

Every major feature, subsystem, or significant refactor should be scored **0–10**.

### 0/10 — Unmaintainable

A system is unmaintainable if:

- ownership is unclear
- naming is meaningless
- logic exists only in AI-generated abstractions
- behaviour requires hidden context
- magic numbers exist without explanation
- dependencies are unclear
- documentation is missing
- code cannot be safely modified without the original creator

**Disposition:** Do not merge. Rewrite or delete.

### 5/10 — Partially maintainable

A system is partially maintainable if:

- engineers can understand it with investigation
- documentation exists but has gaps
- some abstractions are unclear
- ownership boundaries need explanation

**Disposition:** Acceptable only with an explicit debt entry and a path to ≥ 8.

### 10/10 — Highly maintainable

A system is highly maintainable if:

- folders clearly communicate purpose
- naming explains intent
- architecture matches documentation
- tests explain behaviour
- ownership boundaries are obvious
- future engineers can safely extend it

**Target for new work:** **≥ 8**. Score below 4 is a release blocker for new systems.

### Engineer self-check

A future engineer must answer without chat history:

1. What does this do?
2. Why does this exist?
3. Who owns it?
4. What does it not own?
5. Where do I modify it?
6. How do I test it?

If any answer requires asking the AI that created it, the implementation has failed.

Record the score in the batch completion report when introducing a major subsystem.

---

## 3. No Black Box Engineering

Optimisation is allowed.

### Allowed (examples)

- vector databases / indexes
- embeddings
- compression
- caching
- indexing
- lookup tables

### Required for every optimisation

Human explanation of:

- what it stores
- why it exists
- how it works
- how it can be inspected
- how it can be modified

### Required understanding path

```text
Human concept
      ↓
Optimised representation
      ↓
Decoder / inspector
      ↓
Human understanding
```

The repository must never depend on undocumented AI knowledge.  
No system should require the original AI creator to explain it.

Log material optimisation cycles in [`OPTIMISATION_LOG.md`](../04-Operations/OPTIMISATION_LOG.md).

---

## 4. Magic Number Prevention

Unexplained constants are prohibited.

### Bad

```text
timeout = 347
```

### Good

```text
ASSISTANT_REQUEST_TIMEOUT_MS
Purpose:
Maximum wait time before displaying timeout state.
```

Every non-obvious constant requires:

- meaningful name
- explanation
- reason for existence

Also see [Coding Standards §2.11](../03-Engineering/CODING-STANDARDS.md).

---

## 5. Abstraction Rules

Before creating an abstraction, verify it:

- removes **real** duplication?
- improves ownership?
- improves testing?
- simplifies understanding?

If **no** → do not create it.

### Avoid

- empty frameworks
- speculative future systems
- unnecessary interfaces
- one-use generic layers
- wrapper layers that exist only to avoid editing the correct module

Prefer extending existing ownership boundaries.

---

## 6. Documentation Requirements

Every **major feature** requires:

| Field | Content |
|-------|---------|
| **Purpose** | Why it exists |
| **Owner** | Subsystem / package that owns it |
| **Responsibilities** | What it does |
| **Non-responsibilities** | What it explicitly does **not** do |
| **Inputs** | What it consumes |
| **Outputs** | What it produces |
| **Dependencies** | What it relies on |
| **Testing** | How behaviour is verified |
| **Known limitations** | Current gaps |
| **Future considerations** | Deferred work (not speculative engines) |

Place this in the architecture doc and/or module headers; verify in the completion report.

---

## 7. Batch Development Rules

AI should work in **meaningful batches**.

### Do not

- change one tiny thing
- stop
- request approval for every trivial fragment when the work is one coherent unit

### Do

- group related improvements
- complete a coherent slice with docs and tests

### Example

A UI improvement batch may include:

- layout improvements
- accessibility
- documentation
- validation

Still obey [Human Review Policy](../03-Engineering/HUMAN-REVIEW-POLICY.md): batch visual checkpoints; do not invent review theatre.

---

## 8. Human Review Policy

Full policy: [`HUMAN-REVIEW-POLICY.md`](../03-Engineering/HUMAN-REVIEW-POLICY.md).

### Human review **is** required for

- visual product changes
- major workflow changes
- architecture boundaries
- irreversible decisions

### Human review is **not** required for

- documentation
- internal refactoring
- safe cleanup
- tests

### Prefer

- live application review
- screenshots

Avoid unnecessary videos unless they provide real audit/regression value.

---

## 9. Visual Reference Governance

Canonical store: [`docs/01-Product/references/`](../01-Product/references/).

Interpretation rules: [`WORKSPACE-VISUAL-DIRECTION.md`](../01-Product/WORKSPACE-VISUAL-DIRECTION.md).

Images represent:

- product direction
- hierarchy
- user experience goals

They are **not** pixel-perfect requirements.

Ask: **“What problem does this design solve?”**  
Not: **“How do we copy this image?”**

---

## 10. Product Drift Prevention

Before major implementation batches, record (in the batch alignment check / completion report):

| Field | Content |
|-------|---------|
| **Current mission** | What Workspace is right now |
| **Product goal** | What this batch advances |
| **Allowed changes** | In-scope work |
| **Forbidden changes** | Explicit non-goals |
| **Validation method** | How success is proven |

Template: [`BATCH-ALIGNMENT-CHECK.md`](../03-Engineering/BATCH-ALIGNMENT-CHECK.md).

### Product priority (binding)

1. Workspace functionality  
2. Desktop control  
3. User workflows  
4. Usability  
5. Performance  
6. Assistant expansion  

### Drift signals (stop and realign)

- AI/evidence/assistant engines dominate while core desktop gaps remain
- UI leads with AI organisation instead of user-authored control
- Assistant gains execution that bypasses CommandPipeline / PermissionGateway
- Canvas Layout and Desktop Arrangement merged without a product decision

---

## 11. Optimisation Framework

Material performance / storage / indexing / chrome-quality optimisations must be logged:

→ [`docs/04-Operations/OPTIMISATION_LOG.md`](../04-Operations/OPTIMISATION_LOG.md)

Each cycle records goal, problem, analysis, changes, validation, maintainability score, product alignment, and whether human review is required.

Optimisation without a human understanding path (§3) is prohibited.

### Controlled optimisation — Plateau Detection v2

Binding protocol:

→ [`docs/04-Operations/OPTIMISATION_PROTOCOL_V2.md`](../04-Operations/OPTIMISATION_PROTOCOL_V2.md)

| Rule | Requirement |
|------|-------------|
| **Category exhaustion** | Evaluate all 26 approved categories independently; mark PLATEAUED only when no measurable improvement remains in that category |
| **Global plateau** | Only when every category is PLATEAUED **and** two consecutive full-category evaluation passes find nothing measurable |
| **Anti-slop** | Never create work for LOC, commits, files changed, or speculative architecture |
| **Boundary stop** | Immediate stop on product philosophy, ownership, engines, Permission Gateway, Desktop Arrangement behaviour beyond approval, reference reinterpretation, or required human product decisions |

Supersedes the premature rule “stop after two consecutive plateau detections” without category exhaustion.

Quality metrics to update when measurable: Reference Alignment, Maintainability, Commercial Readiness, Human Readability, Accessibility, Performance, Developer Experience, Test Health.

---

## 12. Repository Discipline

The AI must:

- remain inside Workspace
- not reference unrelated projects as architecture authority
- not import old assumptions from other codebases
- inspect current architecture first
- preserve existing ownership
- prefer extending over duplicating

---

## 13. AI Feature Gate

Before AI-facing product work, confirm **all** of:

1. Written product requirement exists  
2. Core workspace capability for the same journey is not unfinished without justification  
3. No new intelligence/evidence/assistant **engine** unless Engineering Governance freeze is lifted  
4. Execution path: user → CommandPipeline → PermissionGateway → approved boundary  
5. [AI Principles](../05-AI/AI-PRINCIPLES.md): Observe → Learn → Suggest → Receive Permission → Automate  

Assistant may ask, explain, and retrieve. Assistant must not become the workspace manager.

---

## 14. Relationship to existing documents

| Document | Role |
|----------|------|
| `00-Constitution/*` | Founder authority — this file does not override |
| `03-Engineering/ENGINEERING-GOVERNANCE.md` | Product alignment, freezes, batch docs |
| `03-Engineering/HUMAN-REVIEW-POLICY.md` | Visual / workflow human review |
| `03-Engineering/CODING-STANDARDS.md` | Naming, magic numbers, file rules |
| `01-Product/WORKSPACE-VISUAL-DIRECTION.md` | Visual north star |
| `01-Product/references/` | Concept references |
| `04-Operations/OPTIMISATION_LOG.md` | Optimisation cycle log |
| `04-Operations/OPTIMISATION_PROTOCOL_V2.md` | Controlled optimisation + Plateau Detection v2 |
| `05-AI/AI-PRINCIPLES.md` | Product AI behaviour |
| `AGENTS.md` | Short agent contract |

**Conflict resolution:** Constitution wins on product authority. This file + Engineering Governance win on agent engineering process — update the lagging doc.

---

## 15. Enforcement checklist

Before merging significant AI-assisted work:

- [ ] Full documentation fields recorded (§6)
- [ ] Batch alignment / drift fields recorded (§10)
- [ ] No unjustified abstraction or duplicate system (§5)
- [ ] No unexplained magic numbers (§4)
- [ ] Optimisations have human inspection path + log entry if material (§3, §11)
- [ ] Maintainability score ≥ 8 for new systems (§2)
- [ ] Human review policy followed (§8)
- [ ] Assistant remains sidecar; Workspace remains primary (§10, §13)
- [ ] Docs updated in-repo (not left in chat)

---

## Explicit confirmation

> Build Workspace first. AI enhances Workspace. AI does not replace Workspace.  
> Optimise freely — but leave a human trail.  
> A future engineer must maintain this repository without the original AI conversation.  
> The repository must remain suitable for professional acquisition, handover, and long-term development.
