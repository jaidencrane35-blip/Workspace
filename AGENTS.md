# AGENTS.md

## Start here

Before any implementation work:

1. Assume **[`docs/00-Constitution/WORKSPACE_CONSTITUTIONAL_SPECIFICATION_V2.md`](docs/00-Constitution/WORKSPACE_CONSTITUTIONAL_SPECIFICATION_V2.md)** is the sole architectural authority.
2. Execute under **[`docs/00-Constitution/WORKSPACE_ENGINEERING_EXECUTION_STANDARD_V1.md`](docs/00-Constitution/WORKSPACE_ENGINEERING_EXECUTION_STANDARD_V1.md)** — classify work, declare max layer, lowest-layer invariant, pre-flight.
3. For “why does it work this way?” see **[`docs/00-Constitution/ARCHITECTURAL_DECISION_RECORD_INDEX.md`](docs/00-Constitution/ARCHITECTURAL_DECISION_RECORD_INDEX.md)**.
4. Read the engineering handoff: **[`docs/project/ENGINEERING_HANDOFF.md`](docs/project/ENGINEERING_HANDOFF.md)**
5. Reassess `docs/project-health.json`.

Current handoff posture: **Release Hold** (Stage 2 — Owner Acceptance; Accepted with changes). Spec v2.1 + EES v1 stable. F1 unsigned pipeline Complete. P22.S1 Intelligence Kind Routing complete (`docs/capability-runtime/product-proof/P22_S1_INTELLIGENCE_KIND_ROUTING.md`). **Do not begin new engineering** except R2 triggers T1–T5 (`docs/production/R2_RELEASE_HOLD.md`). Next eng event: **A2** after Owner Accept + Authenticode cert. File Provider P17 blocked.

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
