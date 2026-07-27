# AGENTS.md

## Cursor Cloud specific instructions

Workspace is a single product: a **Windows-targeted Tauri 2 desktop app** (React 18 + Vite frontend in `app/`, Rust backend in `app/src-tauri` + `packages/*`, embedded SQLite). There are no microservices, Docker, or external databases. Standard commands live in `README.md` and the `package.json` scripts; prefer those. Notes below are the non-obvious gotchas.

### Rust toolchain (important)
- The committed `Cargo.lock` pins dependencies requiring `edition2024` (e.g. `getrandom 0.4.3`), so cargo needs **Rust ≥ 1.85**. The base image historically pinned the rustup default to `1.83.0`, which fails with `feature 'edition2024' is required`. The default is now set to `stable` (`rustup default stable`); if cargo errors on `edition2024`, run `rustup default stable`.

### Building/running the desktop app
- `pnpm dev` runs `tauri dev`, which needs Linux GUI system libs (webkit2gtk/gtk) plus a display, and this app targets **Windows** (`packages/windows-integration` is stubbed on non-Windows). Running the full native shell on this Linux VM is not the supported dev path.
- Frontend-only run: `cd app && pnpm exec vite` serves the React UI on **http://localhost:1420** (`strictPort`). Standalone in a browser, all Tauri IPC calls fail, so a red "Cannot read properties of undefined (reading 'invoke')" banner appears and data actions (create workspace, zones, etc.) error out. UI rendering and tab navigation still work. Full data flows require the Tauri backend.

### Known-good checks on Linux
- `pnpm typecheck`, `pnpm build`, and `pnpm test` (Vitest + catalog/boundary verify scripts) all pass.
- Rust crates that pass here: `cargo test -p workspace-domain` and `cargo test -p workspace-windows-integration`.

### Pre-existing failures unrelated to environment (verify before assuming fixed)
- `main` may not compile: `packages/kernel/src/services/resilience_validation.rs` references `workspace_domain::DecisionCandidateProgression` (only `...Request`/`...Acknowledgement` exist) and a `creation.score` field that does not exist on `DecisionEngineCandidateCreation`. This blocks `cargo build/test --workspace`, `cargo check -p workspace-kernel`, and `tauri dev`. GitHub Actions CI (`windows-latest`) has been red for this reason. Run `cargo check -p workspace-kernel` to check current state; this is a code defect, not an environment issue.
- `cargo test -p workspace-database` has tests that fail on Linux with SQLite `READONLY_DBMOVED` because they drop the `tempdir()` `TempDir` before using the DB (passes on Windows CI due to different filesystem semantics). Platform-specific test issue, not an environment problem.
