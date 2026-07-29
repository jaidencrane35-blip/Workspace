# Human Review Policy

| Field | Value |
|-------|-------|
| **Purpose** | Define when human visual review is required vs when engineering validation is enough |
| **Owner** | Engineering / Product |
| **Status** | Binding for DAF and subsequent work |
| **Related** | [ENGINEERING-GOVERNANCE.md](ENGINEERING-GOVERNANCE.md), [BATCH-ALIGNMENT-CHECK.md](BATCH-ALIGNMENT-CHECK.md), [VISUAL-REVIEW-CHECKLIST.md](VISUAL-REVIEW-CHECKLIST.md), [AI Engineering Governance](../00-Governance/AI_ENGINEERING_GOVERNANCE.md) |

Human review is valuable, but it must be **intentional and grouped**.  
Do **not** require manual review after every code change.

---

## 1. When human visual review **is** required

Human visual review is required when:

* A user-facing UI layout changes significantly
* New major screens are introduced
* Existing navigation changes
* Interaction patterns change
* Visual product direction may have drifted
* Desktop arrangement behaviour needs physical verification (real windows on Windows)

---

## 2. When human visual review is **not** required

Human review is **not** required for:

* Backend refactors
* Documentation changes
* Internal architecture changes
* Tests
* Non-visual bug fixes
* Small implementation changes with no user-facing impact

**Example:** DAF-1a (`WindowController` trait + stubs + Win32 foundation) is backend-only → engineering validation only; no human visual checkpoint.

---

## 3. Review batching

Prefer collecting visual items into a **review checkpoint**.

Avoid:

```text
Change → Human review
Change → Human review
Change → Human review
```

Prefer:

```text
Multiple related UI changes → Human visual audit checkpoint
```

At each checkpoint, maintain a filled [VISUAL-REVIEW-CHECKLIST.md](VISUAL-REVIEW-CHECKLIST.md) with:

* What changed
* Why it changed
* What needs verification
* Expected behaviour
* Known limitations

---

## 4. Browser / visual verification behaviour

When a visual review **is** requested:

1. Start the application (Vite browser preview and/or Tauri desktop where appropriate).
2. Open the application so a human can manually inspect it.
3. Prefer a **live view** or a **screenshot** for normal review.

Do **not** automatically create videos of application usage unless:

* The video is required for a specific audit
* A regression needs recorded evidence
* The user explicitly requests a recording

---

## 5. Automated visual artifacts

Automated screenshots, recordings, or visual artifacts **may** be created when they provide engineering value:

* Regression testing
* Comparing UI states
* Documenting a completed milestone
* Capturing evidence for a PR

Do **not** generate large media files purely because the tooling exists.  
Use the **simplest** verification method that provides confidence.

---

## 6. Division of responsibility

| Who | Validates |
|-----|-----------|
| **Engineering** | Tests, architecture, code quality, ownership boundaries |
| **Humans** | Usability, visual quality, product feel, resemblance to visual north star |

Human review verifies **product direction**. It does not replace engineering validation.

---

## 7. DAF continue rule (gate to DAF-1a+)

Before DAF implementation batches begin (and again before each major DAF UI checkpoint):

| # | Confirm |
|---|---------|
| 1 | DAF-0 documentation is merged or approved for continuation |
| 2 | Visual references are stored under `docs/01-Product/references/` |
| 3 | This review workflow is documented |
| 4 | No additional AI subsystem expansion is planned |

Then proceed with the next DAF batch (e.g. DAF-1a — WindowController foundation).
