# AGENTS.md

## Start here

Before any implementation work, read the canonical engineering handoff:

**[`docs/project/ENGINEERING_HANDOFF.md`](docs/project/ENGINEERING_HANDOFF.md)**

Then reassess machine state in `docs/project-health.json`. Treat repository documentation as authoritative over chat history.

Current handoff posture (see handoff + health for truth): **P16 Voice Input is Engineering Complete (P16.18 Production Acceptance Investigation); live Product Owner Product Proof is pending — not permanently closed. After Owner closes Workspace: cleanup only, never relaunch. Do not begin P17.**

## Cursor Cloud specific instructions

Workspace is a single product: a **Windows-targeted Tauri 2 desktop app** (React 18 + Vite frontend in `app/`, Rust backend in `app/src-tauri` + `packages/*`, embedded SQLite). There are no microservices, Docker, or external databases. Standard commands live in `README.md` and the `package.json` scripts; prefer those. Notes below are the non-obvious gotchas.

Version 1.0 engineering baseline: `architecture/32_Version_1_Baseline.md`.

### Rust toolchain (important)
- The committed `Cargo.lock` pins dependencies requiring `edition2024` (e.g. `getrandom 0.4.3`), so cargo needs **Rust ≥ 1.85**. The base image historically pinned the rustup default to `1.83.0`, which fails with `feature 'edition2024' is required`. The default is now set to `stable` (`rustup default stable`); if cargo errors on `edition2024`, run `rustup default stable`.

### Building/running the desktop app
- `pnpm dev` runs `tauri dev`, which needs Linux GUI system libs (webkit2gtk/gtk) plus a display, and this app targets **Windows** (`packages/windows-integration` is stubbed on non-Windows). Running the full native shell on this Linux VM is not the supported dev path.
- Frontend-only run: `cd app && pnpm exec vite` serves the React UI on **http://localhost:1420** (`strictPort`). Standalone in a browser, all Tauri IPC calls fail, so a red "Cannot read properties of undefined (reading 'invoke')" banner appears and data actions (create workspace, zones, etc.) error out. UI rendering and tab navigation still work. Full data flows require the Tauri backend.

### Known-good checks
- `pnpm typecheck`, `pnpm build`, and `pnpm test` (Vitest + catalog/boundary verify scripts).
- `cargo check -p workspace-kernel` and `cargo check -p workspace-app` compile on current `main`.
- Rust crates: `cargo test -p workspace-domain`, `cargo test -p workspace-windows-integration --lib`.
- Full kernel lib tests: prefer `cargo test -p workspace-kernel --lib -- --test-threads=1` (process-local observation flight / runtime owner).

### Platform-specific test notes
- `cargo test -p workspace-database` has historically failed on Linux with SQLite `READONLY_DBMOVED` when tests drop a `tempdir()` `TempDir` before using the DB (passes on Windows CI due to different filesystem semantics).
