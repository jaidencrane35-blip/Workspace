# Workspace Repository Alignment Audit

Senior architecture / hygiene audit · branch v2-dev · 2026-08-07 · evidence-only; no architecture rewrite

**Validation green** · **38.74 GB recovered** · **No runtime behaviour changes**

> **Verdict**
>
> Workspace is a Windows-first Tauri 2 adaptive desktop layer with a mature Rust kernel (command pipeline + permission gateway + SQLite), a frozen Product Proof experience chrome, and a parallel V2 presentation/governance track on v2-dev. Primary hygiene win was deleting regenerable Cargo artifacts. Main architectural debt is dual documentation authority and large IPC / domain-type surfaces — not structural layering failure.

---

## 1. Repository understanding

### Purpose

Adaptive desktop environment that unifies apps, windows, devices, audio, automation, and AI — without replacing Windows. Mission wedge: trusted interruption recovery (Save Moment → resume plan → honest execute).

### Runtime model

React WebView → Tauri IPC → WorkspaceKernel → CommandPipeline → PermissionGateway → services → SQLite / Win32. Demo adapter permanent for browser/DEV. Plugins/AI workers planned (DEC-011), not shipped.

### Stack

Tauri 2 · React 18 · TypeScript · Vite · Rust ≥1.85 · SQLite (45 migrations 001–046, skip 003) · pnpm 9.15 · MIT

### Intended direction

V1 engineering baseline established (docs 23–32). V2 on v2-dev through Sprint 74: engineering confidence / maintainability only — no new architectural layers until a new brief (V2_AGENT_HANDOFF).

---

## 2. Architectural inventory

| Layer | Location | Responsibility |
| --- | --- | --- |
| Presentation | app/src | WorkspaceShell + 5 pilot destinations; design-system; experience/* adaptation |
| IPC shell | app/src-tauri | Thin Tauri commands (~197); actor context; no DB direct access |
| Kernel | packages/kernel | Lifecycle, pipeline, permissions, ~85 services, EventBus |
| Domain | packages/domain | Pure models; no persistence/UI |
| Persistence | packages/database | SQLite repos + embedded migrations |
| OS integration | packages/windows-integration | Win32 capture/enumerate/mutate; stubs off-Windows |
| Tests (TS) | tests/ | 43 Vitest suites + boundary/CSP/catalog verifiers |
| Governance docs | docs/ + architecture/ | Constitution/ADRs vs production/V2 authority track |
| Placeholders | plugins/, tools/ | README only (DEC-011 future) |

### Crate dependency graph

```
domain ← database / windows-integration ← kernel ← app
```

---

## 3. Repository structure

| Metric | Value |
| --- | --- |
| Cargo crates | 5 |
| pnpm packages | 2 (app, tests) |
| architecture/*.md | 102 |
| SQLite migrations | 45 |
| Tauri commands | ~197 |
| Experience IPC catalog | 21 |

---

## 4. Architectural strengths

- Explicit permission-first mutation path (CapabilityBoundPolicy + StandardPermissionGate)
- Clear crate boundaries; UI never imports database
- Product Proof behavioural contract + evidence JSON under architecture/evidence/
- Dense in-crate kernel contract tests (1231 lib tests green)
- Experience boundary / CSP / explanation-catalog automated verifiers
- Local-first SQLite with compile-time embedded migrations

---

## 5. Architectural drift

| ID | Severity | Area | Finding | Evidence |
| --- | --- | --- | --- | --- |
| D1 | High | Authority dual-track | docs/ claims Phase 1 ready to begin; architecture/ documents V1 baseline + V2 Sprint 74 tip on v2-dev | docs/README.md §Current Phase; architecture/32_*; architecture/V2_AGENT_HANDOFF.md |
| D2 | High | Experience freeze vs V2 | V1 freeze (experience-freeze.md, 23_*) vs V2 refoundation lifting presentation freeze (40_*) — concurrent programme surfaces | architecture/23_*; experience-freeze.md; architecture/40_Experience_Refoundation.md; 01_Current_State.md |
| D3 | Medium | Package README lag | kernel/database READMEs still describe early sprint exclusions while services/migrations implement AI, automation, DE/RE | packages/kernel/README.md; packages/database/README.md vs migrations 009–046 |
| D4 | Medium | IPC inventory vs pilot chrome | IPC-SURFACE.md frames Canvas + Diagnostic as primary consumers; mounted product chrome is pilot Home/Save/Continue/Check-in/Guide (~21 commands) | docs/03-Engineering/IPC-SURFACE.md; experienceIpcCatalog.ts; App.tsx |
| D5 | Medium | TS domain duplication | Hand-maintained app/src/types/domain.ts (~4.4k lines) mirrors Rust domain — drift risk (prior audit 2026-07-27) | app/src/types/domain.ts; packages/domain/ |
| D6 | Medium | Unmounted UI surfaces | CanvasShell, OperatorConsole, AssistantPanel, WorkspaceIntelligencePanel remain in tree but not mounted by App | tests/pilot-chrome.test.ts; architecture/22_Experience_Implementation_Snapshot.md |
| D7 | Low | Planned packages absent | REPOSITORY-STRUCTURE.md lists future domain-*/ai/shell/shared packages; only domain/database/kernel/windows-integration exist | docs/02-Architecture/REPOSITORY-STRUCTURE.md; Cargo.toml members |
| D8 | Low | Process architecture aspirational | DEC-011 multi-process plugin/AI workers not implemented; single Tauri process + stubs | SYSTEM-OVERVIEW.md §3; plugins/README.md placeholder |

---

## 6. Technical debt

| Item | Risk | Evidence |
| --- | --- | --- |
| debug_assert! for RE↔DE invariants | Release builds drop checks | docs/architecture/AUDIT_REPORT.md |
| ~197 registered Tauri commands vs ~21 experience catalog | Large attack/maintenance surface | app/src-tauri/src/lib.rs; experienceIpcCatalog.ts |
| NoOpEncryptionProvider (Tier 0) | Encryption tiers unimplemented | packages/database/src/encryption/ |
| Ambient observation retained but unwired | Dead path retained under Product Proof | observation_startup_trigger.rs |
| Kernel dead_code warnings (~33 in test build) | Unused service helpers / summary_projection variants | cargo test -p workspace-kernel |
| Multi-monitor gate FAIL | Known V1 limitation | architecture/32_Version_1_Baseline.md |

---

## 7–11. Hygiene, cleanup, size

| Metric | Value |
| --- | --- |
| Before (excl .git) | 39.04 GB |
| After cleanup | 0.30 GB |
| Space recovered | 38.74 GB |

Post-validation working tree is 3.33 GB (target/ rebuilt to ~3.1 GB for checks). Net recovery vs start: ~35.7 GB still free of the prior debug cache bloat.

### Working tree size (GB)

| Label | Size (GB) |
| --- | --- |
| Before cleanup | 39.04 |
| After cleanup | 0.3 |
| After validation rebuild | 3.33 |

Source: local filesystem measure 2026-08-07 (working tree excluding .git for before/after cleanup; post-validation includes rebuilt target/).

### Largest folders before cleanup (GB)

| Folder | Size (GB) |
| --- | --- |
| target/ | 38.74 |
| node_modules/ | 0.15 |
| architecture/ | 0.14 |
| .git/ | 0.13 |
| app/ | 0.01 |
| packages/ | 0.01 |

| Path | Classification | Action | Note |
| --- | --- | --- | --- |
| target/ | SAFE TO DELETE | Deleted (~38.74 GB) | Cargo/Tauri build artifacts; gitignored; regenerable |
| app/dist/ | SAFE TO REGENERATE | Deleted then rebuilt by pnpm build | Vite production output; gitignored |
| app/src-tauri/gen/ | SAFE TO REGENERATE | Deleted | Tauri codegen; gitignored |
| node_modules/ | SAFE TO REGENERATE | Kept | Required for validation; regenerable via pnpm install |
| architecture/research/experience/screenshots/ | KEEP | Kept (~137 MB) | Experience evidence / freeze anchors; not build cache |
| Source, docs, tests, config, plugins/, tools/ | KEEP | Kept | Never deleted under this audit charter |

> **.gitignore recommendations (not applied)**
>
> Already covers target/, dist/, gen/, node_modules/, logs, coverage. Consider documenting that architecture/research screenshots are intentional evidence (not cache), and that local target/debug can exceed 30+ GB after full builds — periodic cargo clean is safe.

---

## 12. Validation results

| Check | Result |
| --- | --- |
| pnpm typecheck | PASS |
| pnpm build | PASS |
| pnpm test (342 Vitest + 3 verifiers) | PASS |
| cargo check -p workspace-kernel | PASS |
| cargo check -p workspace-app | PASS |
| cargo test -p workspace-domain (260) | PASS |
| cargo test -p workspace-windows-integration --lib | PASS |
| cargo test -p workspace-database persistent_session | PASS |
| cargo test -p workspace-kernel version_1_hardening | PASS |
| cargo test -p workspace-kernel --lib --test-threads=1 (1231) | PASS |

---

## 13. Recommended next architectural priorities

| # | Priority | Detail |
| --- | --- | --- |
| 1 | Reconcile documentation authority | Single status page: docs/ Phase language vs architecture/ V1 baseline vs V2 tip on v2-dev |
| 2 | Keep Product Proof behaviour stable | Save/Resume/Pilot path remains the proven wedge; do not conflate with V2 presentation work |
| 3 | Harden RE/DE release invariants | Replace critical debug_assert! with release-safe Result errors (prior audit) |
| 4 | Contract-generate TS domain types | Eliminate hand drift between packages/domain and app/src/types/domain.ts |
| 5 | Decide fate of unmounted diagnostic UI | Keep as DEV-only, quarantine, or remove — document explicitly |
| 6 | Multi-monitor topology | Only open V1 release-gate FAIL; harness ready per baseline |

---

Operating constraints honored: no feature work, no refactors, no renames, no architecture introduction. Cleanup limited to SAFE TO DELETE regenerable artifacts. Uncommitted architecture research WIP left untouched.
