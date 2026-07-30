# Workspace Engineering Rules

Concise operational contract for AI coding agents.
Detailed specification: [`docs/00-Governance/AI_ENGINEERING_GOVERNANCE.md`](docs/00-Governance/AI_ENGINEERING_GOVERNANCE.md).
Product alignment & freezes: [`docs/03-Engineering/ENGINEERING-GOVERNANCE.md`](docs/03-Engineering/ENGINEERING-GOVERNANCE.md).

---

## Workspace Mission

Workspace is a **desktop workspace operating environment**.

Primary product:

- application management
- desktop arrangements
- window organisation
- workspace persistence
- user workflow improvement
- controllable layouts

The **Assistant** is a supporting capability.  
The **Assistant is not the product**.

### Product priority

1. Workspace functionality
2. Desktop control
3. User workflows
4. Usability
5. Performance
6. Assistant expansion

AI assists the workspace. AI does not replace the workspace.

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
```

---

## AI Development Rules

AI agents **must**:

- inspect before changing
- reuse existing ownership boundaries
- avoid duplicate systems
- document reasons for major changes
- preserve human readability
- treat `docs/` as source of truth (not prior chat transcripts)
- remain inside the Workspace repository

AI agents **must not**:

- create unnecessary engines
- create abstraction without purpose
- introduce hidden automation
- change product direction
- expand AI features without product need
- optimise blindly
- import assumptions from unrelated projects
- bypass Permission Gateway / CommandPipeline for window or OS control

Prefer: **Simple. Explicit. Documented. Maintainable.**

---

## Controlled optimisation (Plateau v2)

Binding process for bounded optimisation sessions:

→ [`docs/04-Operations/OPTIMISATION_PROTOCOL_V2.md`](docs/04-Operations/OPTIMISATION_PROTOCOL_V2.md)

**Do not** stop after two weak cycles. Exhaust the **26 approved optimisation categories** (visual hierarchy, a11y, dead code, tests, DX, …) with measurable improvements only.

**Do** stop immediately on product-direction ambiguity, ownership/architecture changes, new engines, Permission Gateway / Desktop Arrangement behaviour beyond approved architecture, or governance boundary crossings.

Anti-slop: never create work for LOC/commit/file metrics. Log cycles in [`docs/04-Operations/OPTIMISATION_LOG.md`](docs/04-Operations/OPTIMISATION_LOG.md).

---

## Human Maintainability Rule

All code must be understandable by engineers **without access to AI conversations**.

A future engineer must be able to answer:

- What does this do?
- Why does this exist?
- Who owns it?
- What does it not own?
- Where do I modify it?
- How do I test it?

If the answer requires asking the AI that created it, the implementation has failed.

Score major systems with the **0–10 Human Maintainability Standard** in AI Engineering Governance. Target **≥ 8** for new work.

---

## Change Discipline

Before significant changes, document Purpose, Owner, Responsibilities, Non-responsibilities, Inputs, Outputs, Dependencies, Testing, Known limitations, and Future considerations (see AI Engineering Governance).

Before major batches: fill [`docs/03-Engineering/BATCH-ALIGNMENT-CHECK.md`](docs/03-Engineering/BATCH-ALIGNMENT-CHECK.md).

Work in **meaningful batches** of related improvements — do not fragment every tiny change into a separate approval cycle when the work forms one coherent unit.

---

## Cursor Cloud specific instructions

Workspace is a **Windows-targeted Tauri 2 desktop app** (React 18 + Vite in `app/`, Rust in `app/src-tauri` + `packages/*`, embedded SQLite).

### Product direction pointers

- **Primary product:** desktop workspace management (apps, layouts, real windows, save/restore, work modes).
- **Secondary:** AI Assistant as a supporting capability — never the product itself.
- **Programme context:** Desktop Arrangement Foundation (DAF). Read `docs/03-Engineering/DAF-ARCHITECTURE-AUDIT.md` and `docs/03-Engineering/ENGINEERING-GOVERNANCE.md` before coding.
- **Human review:** `docs/03-Engineering/HUMAN-REVIEW-POLICY.md`. Batch UI checkpoints. Prefer live app / screenshots over automatic videos unless audit/regression/explicit request.
- **Visual north star:** `docs/01-Product/WORKSPACE-REFERENCE-INTERPRETATION.md` (hierarchy binding) + `docs/01-Product/WORKSPACE-VISUAL-DIRECTION.md` + `docs/01-Product/references/`.
- **Product delivery (next):** Milestone **R** Slice 1 — `docs/03-Engineering/MILESTONE-R-SLICE-1-DESKTOP-REALITY-STAGE.md` (await human visual review). Charter: `docs/01-Product/MILESTONE-R-DESKTOP-REALITY-STAGE-CHARTER.md`. Do **not** start Milestone F or later R slices until Slice 1 review.

### Rust toolchain (important)

- The committed `Cargo.lock` pins dependencies requiring `edition2024` (e.g. `getrandom 0.4.3`), so cargo needs **Rust ≥ 1.85**. The base image historically pinned the rustup default to `1.83.0`, which fails with `feature 'edition2024' is required`. The default is now set to `stable` (`rustup default stable`); if cargo errors on `edition2024`, run `rustup default stable`.

### Building/running the desktop app

- `pnpm dev` runs `tauri dev`, which needs Linux GUI system libs (webkit2gtk/gtk) plus a display, and this app targets **Windows** (`packages/windows-integration` is stubbed on non-Windows). Running the full native shell on this Linux VM is not the supported path.
- Frontend-only run: `cd app && pnpm exec vite` serves the React UI on **http://localhost:1420** (`strictPort`). Without Tauri, IPC is unavailable — layout/navigation may still work; live data requires the desktop runtime. Prefer fixing runtime detection in `app/src/lib/ipc.ts` over throwing TypeError on `invoke`.

### Known-good checks on Linux

- `pnpm typecheck`, `pnpm build`, and `pnpm test` (Vitest + catalog/boundary verify scripts) all pass.
- After intentional architecture inventory changes: `pnpm run verify:architecture-governance -- --write` then commit `scripts/generated/architecture-map.json`.
- Rust crates that pass here: `cargo test -p workspace-domain` and `cargo test -p workspace-windows-integration`.

### Pre-existing / environment notes (verify before assuming fixed)

- Older `main` revisions may not compile: `packages/kernel/src/services/resilience_validation.rs` historically referenced missing `DecisionCandidateProgression` / `creation.score`. Programme IV consolidation tip compiles `workspace-kernel`; treat residual resilience debt as platform debt if it reappears on `main`.
- **Resolved on consolidation tip (`cursor/programme-iv-consolidation-34a5`):**
  - `case5_timeline_deterministic` — Activity Graph audit gap-fill now filters generation telemetry **before** the operational window (`d511006`).
  - `case11_evaluation_does_not_contaminate_its_own_inputs` — Recommendation Expired overlays reopen on same fingerprint when source returns (`c4a5fe2`).
- `cargo test -p workspace-database` has tests that fail on Linux with SQLite `READONLY_DBMOVED` because they drop the `tempdir()` `TempDir` before using the DB (passes on Windows CI due to different filesystem semantics). Platform-specific test issue, not an environment problem.
