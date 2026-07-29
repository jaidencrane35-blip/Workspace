# Engineering Governance Reset

| Field | Value |
|-------|-------|
| **Purpose** | Keep Workspace human-maintainable and commercially readable |
| **Owner** | Engineering |
| **Status** | Binding for DAF and subsequent work |
| **Related** | [CODING-STANDARDS.md](CODING-STANDARDS.md), [DEFINITION-OF-DONE.md](DEFINITION-OF-DONE.md), [PRODUCT-VISION-REALIGNMENT-AUDIT.md](../01-Product/PRODUCT-VISION-REALIGNMENT-AUDIT.md) |

The **repository** is the source of truth. Do not rely on previous AI conversations as architecture memory. Important decisions must exist in documentation under `docs/`.

---

## 1. Product alignment (non-negotiable)

Workspace is a **desktop workspace management application**.

Primary product:

- Managing desktop applications
- Creating workspace layouts
- Arranging real application windows
- Saving and restoring workspace states
- Switching between work modes
- Controlling the user’s desktop environment

AI is a **supporting capability**. The Assistant helps users interact with Workspace. It is not the product itself.

Next development programme: **Desktop Arrangement Foundation (DAF)**.

---

## 2. Feature quality questions

Every feature and module must answer, in its architecture doc or module header:

1. **Why does this exist?**
2. **What problem does it solve?**
3. **Who owns this responsibility?**
4. **What does it deliberately not do?**

If those answers are missing, the change is incomplete.

---

## 3. Prefer / Avoid

### Prefer

- Clear names humans recognise (`DesktopArrangement`, not `SpatialCognitiveLayoutHub`)
- Small focused modules
- Logical folders
- Good documentation in-repo
- Simple extension paths
- Editing the correct existing code

### Avoid

- AI-generated code bloat
- Duplicate abstractions
- Wrapper layers that exist only to avoid editing the right module
- Unnecessary engines
- Large files with unrelated responsibilities
- Architecture that only makes sense in an AI transcript
- New subsystems without a clear product requirement

---

## 4. Preserve / Freeze

### Preserve

- Tauri foundation
- IPC architecture (`invokeIpc` + command handlers)
- Permission Gateway
- SQLite
- Existing governance / audit patterns
- Assistant capability layers (Programme II–IV) as **future infrastructure**

### Freeze (unless a written product requirement says otherwise)

- New AI intelligence engines
- New evidence engines
- New assistant expansion batches
- Layout “recommendation engines”
- Autonomous workspace agents
- Assistant-executed window movement

**Boundary:** Window control belongs to the **desktop control layer** (`windows-integration` + kernel commands). Assistant may request or explain. Workspace controls execution.

---

## 5. Context drift prevention — alignment check

Before **each major implementation batch**, copy and fill
[`BATCH-ALIGNMENT-CHECK.md`](BATCH-ALIGNMENT-CHECK.md) into the batch completion report (or link a filled copy).

Confirm:

1. We are working on **Workspace** only.
2. The original product vision (desktop management) is still the goal.
3. Existing architecture is reused where appropriate.
4. No unrelated projects or prior AI contexts drive decisions.
5. No new subsystem exists without a clear product requirement.

Fail the check → do not start coding.

---

## 6. Documentation standards for major batches

Every major batch **must** include:

### 6.1 Completion report

Path pattern: `docs/03-Engineering/<BATCH>-COMPLETION-REPORT.md` or under a programme folder.

Must contain:

- Goal
- Architecture decisions
- Files changed
- Ownership boundaries
- Validation results
- Remaining debt
- Future considerations

### 6.2 Architecture documentation

Path pattern: `docs/02-Architecture/` or programme-specific under `docs/03-Engineering/` / `docs/01-Product/` as appropriate.

Must contain:

- Purpose
- Responsibilities
- Non-responsibilities
- Integration points

### 6.3 Commit messages

Human-readable; explain **intent**.

Good:

```text
feat(window-management): add desktop arrangement persistence model
```

Bad:

```text
update stuff
```

---

## 7. DAF documentation map

| Document | Role |
|----------|------|
| [WORKSPACE-VISUAL-DIRECTION.md](../01-Product/WORKSPACE-VISUAL-DIRECTION.md) | Visual north star |
| [PRODUCT-VISION-REALIGNMENT-AUDIT.md](../01-Product/PRODUCT-VISION-REALIGNMENT-AUDIT.md) | Product gap analysis |
| [DAF-ARCHITECTURE-AUDIT.md](DAF-ARCHITECTURE-AUDIT.md) | DAF-0 archaeology + proposed architecture |
| [DAF-0-COMPLETION-REPORT.md](DAF-0-COMPLETION-REPORT.md) | Foundation batch report |
| [HUMAN-REVIEW-POLICY.md](HUMAN-REVIEW-POLICY.md) | When humans review vs engineering-only |
| [VISUAL-REVIEW-CHECKLIST.md](VISUAL-REVIEW-CHECKLIST.md) | Batched visual checkpoint template |
| [BATCH-ALIGNMENT-CHECK.md](BATCH-ALIGNMENT-CHECK.md) | Pre-batch checklist template |
| [DAF-1A-WINDOW-CONTROLLER.md](DAF-1A-WINDOW-CONTROLLER.md) | WindowController architecture |
| [DAF-1A-ALIGNMENT-CHECK.md](DAF-1A-ALIGNMENT-CHECK.md) | DAF-1a gate record |
| [DAF-1A-COMPLETION-REPORT.md](DAF-1A-COMPLETION-REPORT.md) | DAF-1a batch report |

---

## Explicit confirmation

> Build Workspace first. AI enhances Workspace. AI does not replace Workspace.  
> A human engineer must be able to maintain this repository without an AI assistant explaining it.
