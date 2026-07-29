# AGENTS.md

## Cursor Cloud specific instructions

Workspace is a single product: a **Windows-targeted Tauri 2 desktop app** (React 18 + Vite frontend in `app/`, Rust backend in `app/src-tauri` + `packages/*`, embedded SQLite). There are no microservices, Docker, or external databases. Standard commands live in `README.md` and the `package.json` scripts; prefer those. Notes below are the non-obvious gotchas.

### Rust toolchain (important)
- The committed `Cargo.lock` pins dependencies requiring `edition2024` (e.g. `getrandom 0.4.3`), so cargo needs **Rust ≥ 1.85**. The base image historically pinned the rustup default to `1.83.0`, which fails with `feature 'edition2024' is required`. The default is now set to `stable` (`rustup default stable`); if cargo errors on `edition2024`, run `rustup default stable`.

### Building/running the desktop app
- `pnpm dev` runs `tauri dev`, which needs Linux GUI system libs (webkit2gtk/gtk) plus a display, and this app targets **Windows** (`packages/windows-integration` is stubbed on non-Windows). Running the full native shell on this Linux VM is not the supported dev path.
- Frontend-only run: `cd app && pnpm exec vite` serves the React UI on **http://localhost:1420** (`strictPort`). **Browser mode is intentionally unsupported for IPC.** `invokeIpc` checks `isTauri()` + `window.__TAURI_INTERNALS__` before calling `@tauri-apps/api` `invoke`, so a plain browser does **not** throw `Cannot read properties of undefined (reading 'invoke')`. Instead the shell shows a notice banner and Assistant Intelligence explains that the Tauri desktop runtime is required. Layout and tab navigation still work; live data requires the Tauri backend.

### Known-good checks on Linux
- `pnpm typecheck`, `pnpm build`, and `pnpm test` (Vitest + catalog/boundary verify scripts) all pass.
- Rust crates that pass here: `cargo test -p workspace-domain`, `cargo test -p workspace-windows-integration`, and on Programme IV consolidation tip also `cargo check -p workspace-kernel` plus `case5_timeline_deterministic` / `case11_evaluation_does_not_contaminate_its_own_inputs`.

### Pre-existing / environment notes (verify before assuming fixed)
- Older `main` revisions may not compile: `packages/kernel/src/services/resilience_validation.rs` historically referenced missing `DecisionCandidateProgression` / `creation.score`. Programme IV consolidation tip compiles `workspace-kernel`; treat residual resilience debt as platform debt if it reappears on `main`.
- **Resolved on consolidation tip (`cursor/programme-iv-consolidation-34a5`):**
  - `case5_timeline_deterministic` — Activity Graph audit gap-fill now filters generation telemetry **before** the operational window (`d511006`).
  - `case11_evaluation_does_not_contaminate_its_own_inputs` — Recommendation Expired overlays reopen on same fingerprint when source returns (`c4a5fe2`).
- `cargo test -p workspace-database` has tests that fail on Linux with SQLite `READONLY_DBMOVED` because they drop the `tempdir()` `TempDir` before using the DB (passes on Windows CI due to different filesystem semantics). Platform-specific test issue, not an environment problem.
