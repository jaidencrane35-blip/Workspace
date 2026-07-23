# Repository Structure

| Field | Value |
|-------|-------|
| **Purpose** | Define the recommended repository layout for Workspace at current and future scale |
| **Owner** | Lead Software Engineer |
| **Dependencies** | [Architecture Principles](ARCHITECTURE-PRINCIPLES.md), [Repository Standards](../03-Engineering/REPOSITORY-STANDARDS.md) |
| **Update Process** | Update when new top-level directories are introduced or structure changes. Requires Decision Log entry for structural changes. |

---

## 1. Design Goals

The repository structure must:

- Separate documentation from implementation
- Support a monorepo as the project grows
- Accommodate first-party and third-party plugins
- Keep tooling, scripts, and assets organised
- Remain navigable at 100,000+ lines of code

---

## 2. Recommended Layout

```
Workspace/
├── .github/                    # GitHub configuration
│   ├── ISSUE_TEMPLATE/         # Issue templates
│   ├── workflows/              # CI/CD pipelines
│   └── PULL_REQUEST_TEMPLATE.md
│
├── docs/                       # All project documentation
│   ├── 00-Constitution/
│   ├── 01-Product/
│   ├── 02-Architecture/
│   ├── 03-Engineering/
│   ├── 04-UX/
│   ├── 05-AI/
│   ├── 06-Plugins/
│   ├── 07-Security/
│   ├── 08-Roadmap/
│   ├── 09-Decisions/
│   ├── 10-Sprints/
│   └── README.md               # Documentation index
│
├── app/                        # Tauri + React application shell
│   ├── src/                    # React frontend (TypeScript)
│   ├── src-tauri/              # Tauri Rust backend
│   ├── package.json
│   └── vite.config.ts
│
├── packages/                   # Shared internal Rust/TS packages
│   ├── domain/                 # Shared domain models, ResourceRef, graph types (Sprint 05–12)
│   ├── kernel/                 # Platform kernel, resource services, command pipeline (Sprint 03–12)
│   ├── database/               # SQLite persistence, graph tables, repositories (Sprint 01–12)
│   ├── domain-apps/            # Application domain service (future)
│   ├── domain-windows/         # Window domain service (future)
│   ├── domain-devices/         # Device domain service (future)
│   ├── domain-audio/           # Audio domain service (future)
│   ├── domain-automation/      # Automation domain service (future)
│   ├── ai/                     # AI subsystem (future)
│   ├── shell/                  # UI shell components (future)
│   ├── windows-integration/    # Windows API abstraction (Sprint 35)
│   └── shared/                 # Shared types, utilities, constants (future)
│
├── plugins/                    # First-party plugin examples (placeholder)
│   └── README.md
│
├── tests/                      # Cross-package integration and E2E tests
│   ├── placeholder.test.ts     # Sprint 01 Vitest placeholder
│   └── vitest.config.ts
│
├── tools/                      # Developer tooling and generators (placeholder)
│   └── README.md
│
├── assets/                     # Static assets (icons, fonts, sounds) (future)
│   └── ...
│
├── package.json                # Root pnpm workspace scripts
├── pnpm-workspace.yaml         # pnpm workspace definition
├── Cargo.toml                  # Rust workspace root
├── .gitignore
├── CONTRIBUTING.md
└── README.md
```

---

## 3. Directory Rules

### 3.1 `docs/`

- **Only** documentation. No code, no configs that affect runtime.
- Numbered prefixes enforce reading order and hierarchy.
- Every document includes metadata (Purpose, Owner, Dependencies, Update Process).

### 3.2 `app/`

- Application entry point and top-level assembly.
- Tauri commands delegate to `WorkspaceKernel`; no direct database access from commands.
- State-changing IPC routes through the kernel command layer (Sprint 04+).
- Mutations pass through `CommandPipeline` and `PermissionGate` (Sprint 06).
- No domain logic — delegates to `packages/`.

### 3.3 `packages/`

- Each package is a bounded module with its own tests.
- Package names follow `domain-*` for domain services, descriptive names for infrastructure.
- Packages declare explicit dependencies. No circular references.
- Each package contains:
  ```
  packages/<name>/
  ├── src/
  ├── tests/
  ├── package.json (or equivalent)
  └── README.md
  ```

### 3.4 `plugins/`

- First-party plugin examples and SDK consumers.
- Third-party plugins are **not** stored in this repository.
- Plugin API definitions live in a dedicated package (e.g., `packages/plugin-sdk/`).

### 3.5 `tests/`

- Cross-package integration tests and end-to-end tests.
- Package-level unit tests live inside each package.

### 3.6 `scripts/` and `tools/`

- `scripts/` — operational scripts (build, deploy, setup)
- `tools/` — developer tools (generators, linters, custom tooling)

### 3.7 `assets/`

- Static files only. No code.
- Organised by type: `icons/`, `fonts/`, `sounds/`, etc.

---

## 4. What Not to Put in the Repository

| Item | Where It Belongs |
|------|-----------------|
| Secrets and credentials | Environment variables / secure vault — never committed |
| Build artifacts | `.gitignore` — generated locally or in CI |
| User data / layouts | Runtime local storage — not in repo |
| Third-party plugin source | Plugin author's repository |
| Large binary blobs | Asset management system or LFS (if needed) |

---

## 5. Monorepo Tooling — DEC-012

**Decision:** pnpm workspaces for TypeScript/JavaScript packages. Cargo workspace for Rust crates.

Expectations:

- `pnpm-workspace.yaml` at repository root
- `Cargo.toml` workspace manifest for Rust packages
- Shared dependency management via pnpm
- Per-package versioning internally
- Workspace-level scripts for build, test, lint
- CI runs affected packages when supported

---

## 6. Current State (Phase 1 — Sprint 38)

Phase 1 scaffolding is in progress. Implemented structure:

```
Workspace/
├── app/                    # Tauri + IPC + operator console + spatial canvas shell
├── packages/
│   ├── domain/             # … execution_reconciliation list fold
│   ├── kernel/             # … GetExecutionStates + context enrichment + IPC-ready handlers
│   └── database/           # Repositories + graph + layout tables
```

Resource services own graph-backed entity persistence. Layout owns spatial state. Projection aggregates derived read models. Action intents provide metadata-first command mapping. Capability discovery derives actor authority from existing policy without a permission database. Observation derives a neutral, read-only activity stream over the audit trail without adding persistence. Analytics deterministically aggregates observations into `WorkspaceMetrics` — the "Learn" stage — with no AI and no persistence. Context deterministically composes state, activity, metrics, authority, execution outcome summary, and reconciled execution states into `WorkspaceContext` — the "Context" boundary — with no persistence and no inference. Suggestion derives deterministic **proposals** from `WorkspaceContext` via simple threshold rules — the "Suggest" stage — with no AI and no suggestion store. Sprints 21–29 build the governed execution intelligence pipeline (approval → execution → outcomes → guard → cancellation → reconciliation). Sprint 30 adds **Execution States Projection Foundation** — bounded list reconciliation over outcome history. Sprint 31 enriches **WorkspaceContext** with those reconciled states without persistence or automation. Sprint 32 wires **execution IPC** (S23–S30) and an **operator console** so the suggestion→intent→execute path is exercisable front-to-back without Spatial Canvas. Sprint 33 adds the **Spatial Workspace Canvas shell** (DEC-009). Sprint 34 wires **layout save/restore** through existing layout IPC.

Implementation directories will be created during Phase 1 scaffolding.

---

## 7. Phase 1 Additions

These directories may be added as the project matures:

| Directory | When |
|-----------|------|
| `.github/workflows/` | CI/CD setup (Phase 1) |
| `app/` | Application scaffolding (Phase 1) |
| `packages/` | First module creation (Phase 1) |
| `plugins/` | Plugin SDK ready (Phase 2+) |
| `tests/e2e/` | E2E framework selected (Phase 2) |

Each addition requires a Decision Log entry if it changes the structure defined here.

---

## 8. Naming Conventions (Sprint 06)

| Term | Reserved for | Example |
|------|--------------|---------|
| **Registry** | Runtime/service health tracking only | `ServiceRegistry` |
| **Service** | Domain operations coordinating repos + events | `WorkspaceService`, future `ZoneService` |
| **Repository** | SQL persistence only | `WorkspaceRepository` |
| **Pipeline** | Uniform command dispatch + permission | `CommandPipeline` |

Do **not** introduce `ResourceRegistry`. Future resources use sibling `*Service` types under `CommandHandler`, not nested under `WorkspaceService`.

---

## Related Documents

- [Architecture Principles](ARCHITECTURE-PRINCIPLES.md)
- [Repository Standards](../03-Engineering/REPOSITORY-STANDARDS.md)
- [Coding Standards](../03-Engineering/CODING-STANDARDS.md)
