# AI Engineering Governance

| Field | Value |
|-------|-------|
| **Purpose** | Control AI-assisted engineering so acceleration does not sacrifice human understanding, quality, maintainability, or product direction |
| **Owner** | Engineering (binding for agents and human contributors) |
| **Status** | Binding |
| **Related** | [AGENTS.md](../../AGENTS.md), [Engineering Governance](../03-Engineering/ENGINEERING-GOVERNANCE.md), [Coding Standards](../03-Engineering/CODING-STANDARDS.md), [Engineering Principles](../03-Engineering/ENGINEERING-PRINCIPLES.md), [Governance Model](../00-Constitution/GOVERNANCE.md), [AI Principles](../05-AI/AI-PRINCIPLES.md) |

This document is the **detailed** AI-assisted engineering control specification.
[`AGENTS.md`](../../AGENTS.md) is the short operational contract.
[`ENGINEERING-GOVERNANCE.md`](../03-Engineering/ENGINEERING-GOVERNANCE.md) owns product alignment, freeze rules, and batch documentation.
Do **not** treat this file as a second product constitution — founder authority remains in `00-Constitution/`.

---

## 1. Purpose

AI tools accelerate implementation. They must not become the source of architecture memory, product direction, or unexplained complexity.

Goals:

- AI acceleration **without** sacrificing human understanding
- Software quality that survives the original author
- Maintainability by engineers who never saw the generating chat
- Product direction: **Workspace first; Assistant sidecar**

Non-goals of this document:

- Replacing the Project Constitution or founder decision authority
- Replacing AI product behaviour rules ([AI Principles](../05-AI/AI-PRINCIPLES.md))
- Inventing a parallel architecture review process

---

## 2. Human Maintainability Standard

Every major feature, subsystem, or significant refactor should be scored **0–10**.

### Score 0 — Unmaintainable without AI

Characteristics:

- Impossible to understand without AI explanation
- Hidden assumptions
- Unclear ownership
- Undocumented behaviour
- Excessive abstraction
- Magic numbers everywhere
- Meaningless naming
- Circular dependencies

**Disposition:** Do not merge. Rewrite or delete.

### Score 5 — Recoverable with investigation

Characteristics:

- Understandable but requires significant investigation
- Some missing documentation
- Some unclear boundaries
- Moderate technical debt

**Disposition:** Acceptable only with an explicit debt entry and a path to ≥ 8.

### Score 10 — Independently maintainable

Characteristics:

- Engineers can discover features easily
- Architecture matches documentation
- Ownership is obvious
- Tests explain behaviour
- Naming explains purpose
- No hidden dependencies
- Safe modification path exists

**Target for new work:** **≥ 8**. Score below 4 is a release blocker for new systems.

### How to score

Ask:

1. Can a new engineer find the entry point in under 10 minutes using only the repo?
2. Is ownership stated in docs or module headers?
3. Do tests demonstrate intended behaviour without reading the generating chat?
4. Can the feature be modified without touching unrelated subsystems?
5. Are non-responsibilities explicit (what it must **not** do)?

Record the score in the batch completion report when introducing a major subsystem.

---

## 3. No Black Box Engineering

Optimisation is allowed. Compression is allowed. Performance techniques are allowed.

**A human-readable explanation must exist.**

### Allowed (with documentation)

- Vector indexes
- Caches
- Embeddings
- Compressed storage
- Lookup tables
- Binary or packed formats
- Heuristic scorers

### Required documentation for each

Human documentation must explain:

- **Why** it exists
- **How** it works at a conceptual level
- **How to inspect** it (tools, queries, dumps)
- **How to decode** it (schemas, versioning)
- **How to modify** it safely

### Prohibited

- Systems that only the original AI session can explain
- Opaque blobs with no schema or recovery path
- Behaviour that cannot be tested without reproducing private chat context

No system should require the original AI creator to explain it.

---

## 4. Magic Number Prevention

Unexplained constants are prohibited.

### Bad

```rust
if score > 0.73 && retries < 4 {
    sleep(Duration::from_millis(350));
}
```

```ts
const COLS = 13;
if (windows.length > 7) collapse();
```

A reader cannot tell whether `0.73`, `4`, `350`, `13`, or `7` are product rules, tuned guesses, or accidents.

### Good

```rust
/// Minimum confidence before a match is offered to the user (product rule).
const MIN_MATCH_CONFIDENCE: f64 = 0.73;
/// Cap transient capture retries to avoid blocking the UI thread.
const MAX_CAPTURE_RETRIES: u32 = 4;
/// Backoff between capture retries (milliseconds).
const CAPTURE_RETRY_BACKOFF_MS: u64 = 350;
```

```ts
/** Max columns in the diagnostic arrangement preview grid. */
const ARRANGEMENT_PREVIEW_COLUMNS = 13;
/** Collapse preview when more windows than this are present. */
const PREVIEW_COLLAPSE_WINDOW_THRESHOLD = 7;
```

### Rules

1. Named constants for any non-obvious literal used in control flow, layout, timing, thresholds, or limits.
2. Comment **why** the value exists when it encodes product or platform policy.
3. Prefer deriving values from config/settings when operators need to change them.
4. Test-only literals may remain inline when clearly scoped to the test and self-explanatory.

---

## 5. Product hierarchy (anti-drift)

Workspace is the product. The Assistant is a supporting capability.

```text
Workspace Desktop Environment
|
├── Desktop Arrangement
├── Application Management
├── Layout Systems
├── User Workflow
├── Window Control
├── Workspace Persistence
├── User Experience
|
└── Assistant Sidecar
    ├── Questions
    ├── Explanations
    ├── Retrieval
    ├── Helpful Interaction
    └── Future capabilities
```

### Drift signals (stop and realign)

- New work is mostly AI/evidence/assistant engines while DAF / window / arrangement gaps remain
- UI leads with “AI can organise your workspace” instead of user-authored save/restore
- Assistant gains execution paths that bypass CommandPipeline / PermissionGateway
- Canvas Layout and Desktop Arrangement are merged without an explicit product decision

### Required response

Fill [`BATCH-ALIGNMENT-CHECK.md`](../03-Engineering/BATCH-ALIGNMENT-CHECK.md). Fail the check → do not start coding.

---

## 6. Change documentation template

Significant changes document:

| Field | Question |
|-------|----------|
| **Purpose** | Why does this exist? |
| **Owner** | Which subsystem owns this? |
| **Inputs** | What does it consume? |
| **Outputs** | What does it produce? |
| **Dependencies** | What does it rely on? |
| **Non-responsibilities** | What does it explicitly NOT do? |

Place this in:

- Module / crate rustdoc or file header for code-owned units
- Architecture doc for major batches
- Completion report for ownership verification

This mirrors §2 of [Engineering Governance](../03-Engineering/ENGINEERING-GOVERNANCE.md) — do not invent a second checklist.

---

## 7. Prefer extend; forbid unjustified novelty

### Prefer

- Extending existing owners (`windows-integration`, kernel commands, domain modules, existing UI panels)
- Clear human names
- Small focused modules
- In-repo documentation
- Editing the correct existing file

### Forbid without written justification

- Parallel control layers
- Duplicate repositories / services for the same responsibility
- Abstraction frameworks “for later”
- Renames that break mental models without a migration note
- Hidden automation (especially window moves)
- AI features added because they sound impressive

---

## 8. AI feature gate

Before adding AI-facing product work, confirm **all** of:

1. A written product requirement exists (not “it would be cool”).
2. Core workspace capability is not blocked by unfinished DAF / window / arrangement / UX foundations for the same user journey.
3. The change does **not** introduce a new intelligence/evidence/assistant **engine** unless [Engineering Governance](../03-Engineering/ENGINEERING-GOVERNANCE.md) freeze is explicitly lifted.
4. Execution remains: user authority → CommandPipeline → PermissionGateway → approved integration boundary.
5. [AI Principles](../05-AI/AI-PRINCIPLES.md) sequence is respected: Observe → Learn → Suggest → Receive Permission → Automate.

Assistant may ask, explain, and retrieve. Assistant must not become the workspace manager.

---

## 9. Relationship to existing documents

| Document | Role | This file |
|----------|------|-----------|
| `00-Constitution/*` | Founder authority, non-negotiables | Does not override |
| `03-Engineering/ENGINEERING-GOVERNANCE.md` | Product alignment, freeze, batch docs | Complements; do not duplicate freeze lists here beyond pointers |
| `03-Engineering/CODING-STANDARDS.md` | Naming, file size, comments | Magic-number and naming rules align here |
| `03-Engineering/ENGINEERING-PRINCIPLES.md` | Maintainability before speed | Scoring system operationalises this |
| `05-AI/AI-PRINCIPLES.md` | Product AI behaviour / permissions | Separate: product AI vs engineering AI process |
| `AGENTS.md` | Short agent contract + cloud ops notes | Must stay consistent with this file |

If documents conflict on **product authority**, Constitution wins.
If documents conflict on **engineering process for agents**, this file + Engineering Governance win; update the lagging doc.

---

## 10. Enforcement checklist (agents and reviewers)

Before merging significant AI-assisted work:

- [ ] Purpose / Owner / Inputs / Outputs / Dependencies / Non-responsibilities recorded
- [ ] Batch alignment check passed (major batches)
- [ ] No new unjustified engine or duplicate subsystem
- [ ] No unexplained magic numbers in control paths
- [ ] Any optimisation/black-box technique has human inspection docs
- [ ] Maintainability score estimated (≥ 8 for new systems)
- [ ] Assistant remains sidecar; Workspace remains primary
- [ ] Docs updated in-repo (not left in chat)

---

## Explicit confirmation

> Build Workspace first. AI enhances Workspace. AI does not replace Workspace.  
> Optimise freely — but leave a human trail.  
> A future engineer must maintain this repository without the original AI conversation.
