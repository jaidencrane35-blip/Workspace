# Workspace

Adaptive Windows-first workspace runtime built with Tauri, React, TypeScript, Rust, and SQLite.

## Current State

- Architecture and governance foundations are established and actively maintained.
- Runtime implementation spans Tauri IPC, kernel command pipeline, recommendation/decision services, and domain contracts.
- CI runs typecheck, frontend build, Rust check/build/test, and frontend contract tests.
- Sprint history currently extends through Sprint 102.

## Developer Setup

### Prerequisites

| Tool | Version |
|------|---------|
| Node.js | >= 20 |
| pnpm | 9.x |
| Rust | stable (Rust >= 1.85 required by lockfile) |

### Install

```bash
corepack enable
pnpm install
```

### Common Commands

| Command | Purpose |
|---------|---------|
| `pnpm dev` | Launch app package dev command (`tauri dev`) |
| `pnpm typecheck` | TypeScript typecheck for app |
| `pnpm build` | App frontend build (`tsc && vite build`) |
| `pnpm test` | Frontend contract tests (Vitest suite in `tests/`) |
| `pnpm verify:explanation-catalog` | Verify generated explanation catalog parity |
| `pnpm verify:ui-experience-boundary` | Verify UI import boundary contract |
| `cargo check -p workspace-kernel` | Validate kernel crate |
| `cargo test -p workspace-domain` | Run domain contract tests |
| `cargo test -p workspace-windows-integration` | Run windows-integration tests |

## Repository Layout

```text
app/         Tauri shell + React frontend + IPC adapters
packages/    Rust crates: domain, database, kernel, windows-integration
scripts/     Governance and verification scripts
tests/       Frontend contract tests (Vitest)
docs/        Architecture, engineering, AI, security, roadmap, and sprints
plugins/     Placeholder for plugin-related documentation/artifacts
tools/       Placeholder for tool docs
```

See `docs/02-Architecture/REPOSITORY-STRUCTURE.md` for structural rules.

## Documentation

Start at `docs/README.md` for the canonical index. Key entry points:

- `docs/00-Constitution/PROJECT-CONSTITUTION.md`
- `docs/02-Architecture/ARCHITECTURE-PRINCIPLES.md`
- `docs/03-Engineering/ENGINEERING-PRINCIPLES.md`
- `docs/05-AI/WORKSPACE-RECOMMENDATION-ENGINE.md`
- `docs/05-AI/WORKSPACE-DECISION-ENGINE.md`
- `docs/05-AI/PROGRAMME-III-COHERENT-WORKSPACE-RUNTIME.md`
- `docs/05-AI/PROGRAMME-IV-INTERACTION-RUNTIME.md`
- `docs/05-AI/WORKSPACE-EVIDENCE-RELIABILITY-ARCHITECTURE.md`
- `docs/05-AI/WORKSPACE-INTELLIGENCE-HUB-ARCHITECTURE.md`
- `docs/08-Roadmap/ROADMAP.md`
- `docs/09-Decisions/DECISION-LOG.md`

## Contributing

See `CONTRIBUTING.md` for workflow and governance expectations.

## License

MIT. See `LICENSE`.
