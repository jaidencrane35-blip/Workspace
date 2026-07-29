# Workspace Engineering Rules

Short operational contract for AI-assisted development in this repository.
Detailed rules live in [`docs/00-Governance/AI_ENGINEERING_GOVERNANCE.md`](docs/00-Governance/AI_ENGINEERING_GOVERNANCE.md).
Product and batch drift controls live in [`docs/03-Engineering/ENGINEERING-GOVERNANCE.md`](docs/03-Engineering/ENGINEERING-GOVERNANCE.md).

---

## Mission

Workspace is a **desktop workspace operating environment**.

Its purpose is:

- managing applications
- arranging applications
- saving workspace states
- improving user productivity
- providing controllable desktop workflows

**AI assists the workspace. AI does not replace the workspace.**

Product hierarchy:

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

The Assistant must never become the product itself. Do not allow AI features to consume the majority of engineering focus while core workspace functionality remains incomplete.

---

## Repository Discipline

The agent must:

- remain inside the Workspace repository
- not reference unrelated previous projects as architecture authority
- not import assumptions from other codebases
- inspect existing ownership before creating new systems
- prefer extending existing systems over creating duplicates
- treat `docs/` as source of truth — not prior chat transcripts

---

## Change Discipline

Before implementing any significant change, document:

| Field | Question |
|-------|----------|
| **Purpose** | Why does this exist? |
| **Owner** | Which subsystem owns this? |
| **Inputs** | What does it consume? |
| **Outputs** | What does it produce? |
| **Dependencies** | What does it rely on? |
| **Non-responsibilities** | What does it explicitly NOT do? |

If those answers are missing, the change is incomplete. Prefer recording them in the module header and/or the batch architecture doc.

Before major batches: fill [`docs/03-Engineering/BATCH-ALIGNMENT-CHECK.md`](docs/03-Engineering/BATCH-ALIGNMENT-CHECK.md).

---

## Human Maintainability Requirement

All code must be understandable by engineers **without AI assistance**.

A future engineer must be able to:

- locate functionality
- understand ownership
- understand intent
- safely modify code
- debug failures
- extend features

The repository must not require access to AI conversations to understand the architecture.

Evaluate major features against the **0–10 Human Maintainability Standard** in [`docs/00-Governance/AI_ENGINEERING_GOVERNANCE.md`](docs/00-Governance/AI_ENGINEERING_GOVERNANCE.md). Target **≥ 8** for new work; do not merge Score 0–3 material.

---

## AI Development Restrictions

The agent must **not**:

- create unnecessary engines
- create duplicate systems
- create abstractions without justification
- rename systems without documentation
- create hidden automation
- add complexity for theoretical future use
- create AI features because they sound impressive
- bypass Permission Gateway / CommandPipeline for window or OS control
- treat Assistant as the primary interaction model for desktop control

Prefer:

- Simple
- Explicit
- Documented
- Maintainable

**Frozen unless a written product requirement says otherwise:** new AI intelligence / evidence / assistant expansion engines. See Engineering Governance.

---

## Cursor Cloud specific instructions

Workspace is a **Windows-targeted Tauri 2 desktop app** (React 18 + Vite in `app/`, Rust in `app/src-tauri` + `packages/*`, embedded SQLite).

### Product direction pointers

- **Primary product:** desktop workspace management (apps, layouts, real windows, save/restore, work modes).
- **Secondary:** AI Assistant as a supporting capability — never the product itself.
- **Programme context:** Desktop Arrangement Foundation (DAF). Read `docs/03-Engineering/DAF-ARCHITECTURE-AUDIT.md` and `docs/03-Engineering/ENGINEERING-GOVERNANCE.md` before coding.
- **Human review:** `docs/03-Engineering/HUMAN-REVIEW-POLICY.md`. Batch UI checkpoints. Prefer live app / screenshots over automatic videos unless audit/regression/explicit request.
- **Visual north star:** `docs/01-Product/WORKSPACE-VISUAL-DIRECTION.md` + `docs/01-Product/references/`.

### Rust toolchain (important)

- The committed `Cargo.lock` pins dependencies requiring `edition2024` (e.g. `getrandom 0.4.3`), so cargo needs **Rust ≥ 1.85**. The base image historically pinned the rustup default to `1.83.0`, which fails with `feature 'edition2024' is required`. The default is now set to `stable` (`rustup default stable`); if cargo errors on `edition2024`, run `rustup default stable`.

### Building/running the desktop app

- `pnpm dev` runs `tauri dev`, which needs Linux GUI system libs (webkit2gtk/gtk) plus a display, and this app targets **Windows** (`packages/windows-integration` is stubbed on non-Windows). Running the full native shell on this Linux VM is not the supported dev path.
- Frontend-only run: `cd app && pnpm exec vite` serves the React UI on **http://localhost:1420** (`strictPort`). Without Tauri, IPC is unavailable — layout/navigation may still work; live data requires the desktop runtime. Prefer fixing runtime detection in `app/src/lib/ipc.ts` over throwing TypeError on `invoke`.

### Known-good checks on Linux

- `pnpm typecheck`, `pnpm build`, and `pnpm test` (Vitest + catalog/boundary verify scripts) all pass.
- Rust crates that pass here: `cargo test -p workspace-domain` and `cargo test -p workspace-windows-integration`.

### Pre-existing / environment notes (verify before assuming fixed)

- Older `main` revisions may not compile: `packages/kernel/src/services/resilience_validation.rs` historically referenced missing `DecisionCandidateProgression` / `creation.score`. Programme IV consolidation tip compiles `workspace-kernel`; treat residual resilience debt as platform debt if it reappears on `main`.
- **Resolved on consolidation tip (`cursor/programme-iv-consolidation-34a5`):**
  - `case5_timeline_deterministic` — Activity Graph audit gap-fill now filters generation telemetry **before** the operational window (`d511006`).
  - `case11_evaluation_does_not_contaminate_its_own_inputs` — Recommendation Expired overlays reopen on same fingerprint when source returns (`c4a5fe2`).
- `cargo test -p workspace-database` has tests that fail on Linux with SQLite `READONLY_DBMOVED` because they drop the `tempdir()` `TempDir` before using the DB (passes on Windows CI due to different filesystem semantics). Platform-specific test issue, not an environment problem.
