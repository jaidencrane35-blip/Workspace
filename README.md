# Workspace

> One workspace that brings your PC, phone, audio, and apps together.

Workspace is an adaptive desktop environment that unifies applications, windows, devices, audio, automation, and AI into a single operating experience — without replacing Windows.

**Status:** Version 1.0 engineering baseline — see [`architecture/32_Version_1_Baseline.md`](architecture/32_Version_1_Baseline.md).

---

## Mission

Users should eventually feel:

- *"Everything is finally organised."*
- *"My desktop works the way I want it to."*

The software adapts to the user. The user never adapts to the software.

---

## Repository Status

| Area | Status |
|------|--------|
| Documentation | Version 1.0 baseline (`architecture/32_*`) |
| Architecture | Production integration authority `architecture/23`–`32` |
| Technology stack | Tauri 2 + React 18 + TypeScript + Rust + SQLite |
| Implementation | Runtime core release-ready (single-monitor Windows) |
| Experience | Frozen chrome — behavioural IPC proven |
| CI/CD | PR workflow active (`.github/workflows/ci-pr.yml`) |

---

## Developer Setup

### Prerequisites

| Tool | Version | Notes |
|------|---------|-------|
| Node.js | ≥ 20 | Required for frontend tooling |
| pnpm | 9.x | `corepack enable` then `corepack prepare pnpm@9.15.0 --activate` |
| Rust | stable ≥ 1.85 | Install via [rustup](https://rustup.rs/); required for `edition2024` lockfile deps |
| Windows SDK | — | Required for Tauri on Windows (Visual Studio Build Tools) |

### Install

```powershell
git clone https://github.com/jaidencrane35-blip/Workspace.git
cd Workspace
corepack enable
pnpm install
```

### Common Commands

| Command | Purpose |
|---------|---------|
| `pnpm dev` | Launch Tauri dev shell (React + Rust IPC) |
| `pnpm build` | Build React frontend (`app/dist`) |
| `pnpm installer:build` | Produce Windows NSIS setup.exe (`Workspace_*_x64-setup.exe`) |
| `pnpm checksums:generate` | Write SHA-256 sidecars beside setup.exe |
| `pnpm verify:release` | Full unsigned release validation (no signing) |
| `pnpm typecheck` | TypeScript validation |
| `pnpm test` | Run Vitest + boundary/CSP/production verifiers |
| `cargo check --workspace` | Validate Rust workspace |
| `cargo build --workspace` | Build all Rust crates including Tauri shell |
| `cargo test -p workspace-kernel --lib -- --test-threads=1` | Kernel tests (serial; observation flight) |
| `cargo test -p workspace-database` | Run database crate tests |

### Repository Layout

```
app/                 Tauri + React application shell
packages/
  kernel/            Platform Kernel boundary (placeholder)
  database/          SQLite persistence foundation
plugins/             Future first-party plugins (placeholder)
tools/               Developer tooling (placeholder)
tests/               Cross-package tests
docs/                Project documentation
```

See [Repository Structure](docs/02-Architecture/REPOSITORY-STRUCTURE.md) for the full layout.

---

## Documentation

All project knowledge lives in [`docs/`](docs/README.md). Start here:

| Document | Purpose |
|----------|---------|
| [Project Constitution](docs/00-Constitution/PROJECT-CONSTITUTION.md) | Non-negotiable project rules |
| [Product Vision](docs/01-Product/PRODUCT-VISION.md) | What we are building and why |
| [Architecture Principles](docs/02-Architecture/ARCHITECTURE-PRINCIPLES.md) | How the system is designed |
| [Engineering Principles](docs/03-Engineering/ENGINEERING-PRINCIPLES.md) | How we build |
| [Roadmap](docs/08-Roadmap/ROADMAP.md) | Planned phases and milestones |
| [Open Questions](docs/09-Decisions/OPEN-QUESTIONS.md) | Unresolved decisions requiring review |
| [Sprint 01](docs/10-Sprints/sprints/2026-07-23-sprint-01.md) | Phase 1 architecture validation |

---

## Core Philosophy

- Workspace **enhances** Windows; it does not replace it.
- The user is **always in control**.
- AI follows: **Observe → Learn → Suggest → Receive Permission → Automate**.
- Architecture, documentation, and planning take priority over shortcuts.

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for workflow, standards, and review expectations.

---

## License

MIT License. See [LICENSE](LICENSE) and [DEC-006](docs/09-Decisions/DECISION-LOG.md).
