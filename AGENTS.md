# AGENTS.md

## Cursor Cloud specific instructions

Workspace is a **Windows-targeted Tauri 2 desktop app** (React 18 + Vite in `app/`, Rust in `app/src-tauri` + `packages/*`, embedded SQLite).

### Product direction (source of truth = repo docs)

- **Primary product:** desktop workspace management (apps, layouts, real windows, save/restore, work modes).
- **Secondary:** AI Assistant as a supporting capability — never the product itself.
- **Next programme:** Desktop Arrangement Foundation (DAF). Read `docs/03-Engineering/DAF-ARCHITECTURE-AUDIT.md` and `docs/03-Engineering/ENGINEERING-GOVERNANCE.md` before coding.
- **Frozen:** new AI intelligence/evidence/assistant engines unless a written product requirement says otherwise.
- **Visual north star:** `docs/01-Product/WORKSPACE-VISUAL-DIRECTION.md` + `docs/01-Product/references/`.
- Do **not** treat prior chat transcripts as architecture memory; update `docs/` instead.
- Before major batches: fill `docs/03-Engineering/BATCH-ALIGNMENT-CHECK.md`.

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
