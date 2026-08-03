# 32 — Version 1.0 Engineering Baseline

**Immutable engineering baseline for Version 1.0.**  
Verified facts only. No roadmap. No aspirations.

Authority chain: `30_Release_Gate.md` · `31_Version_1_Release_Checklist.md` · frozen Experience contract.

Baseline commit lineage (release train):

| Commit | Message |
| --- | --- |
| `1f66c79` | canonical workspace runtime state |
| `b16f342` | persistent workspace session lifecycle |
| `798469c` | runtime resilience and recovery model |
| `cdb528a` | Windows product proof and release validation |
| `c9e043e` | close remaining release gate evidence gaps |
| *(this commit)* | establish version 1.0 engineering baseline |

---

## Supported platform

| Fact | Value |
| --- | --- |
| OS | Windows 10/11 (proof host build **26200**) |
| Shell | Tauri 2 desktop app |
| Frontend | React 18 + Vite (`app/`) |
| Backend | Rust workspace (`packages/*`, `app/src-tauri`) |
| Persistence | Embedded SQLite (bundled migrations through `046_*`) |
| Monitors claimed | **Single-monitor** sessions only |

---

## Supported capabilities

| Capability | Gate status | Evidence |
| --- | --- | --- |
| Observation (Win32) | PASS | `evidence/windows-product-proof.json` |
| Save Moment | PASS | `evidence/experience-e2e-behaviour.json` |
| Runtime state | PASS | E2E hydrate + `26_Workspace_Runtime_State.md` |
| Restore planning | PASS | E2E `resolve_resume_plan` |
| Restore execution | PASS WITH LIMITATIONS | product-proof matrix + E2E execute |
| Session persistence | PASS | E2E restart + `27_*` |
| Crash recovery | PASS WITH LIMITATIONS | `evidence/process-kill-recovery.json` |
| Experience chrome (frozen) | PASS | behavioural IPC E2E; freeze anchors |

---

## Known limitations

1. Dual-monitor placement and monitor hot-plug — **FAIL** gate (`multi-monitor-topology.json`, harness ready).
2. `SetForegroundWindow` may refuse without input attachment.
3. `window.z_order` unsupported; place uses `SWP_NOZORDER`.
4. Closed applications are never relaunched.
5. Desktop effects already applied before process kill are not OS-rolled-back.
6. Virtual desktop / browser-tab fidelity out of scope.
7. Ambient observation at startup is disabled (Product Proof).
8. Experience validation is behavioural IPC (WebView pixels not driven).

---

## Evidence inventory

| File | Role |
| --- | --- |
| `architecture/evidence/windows-product-proof.json` | Win32 behaviour matrix |
| `architecture/evidence/multi-monitor-topology.json` | Topology harness output |
| `architecture/evidence/experience-e2e-behaviour.json` | Operator IPC E2E |
| `architecture/evidence/process-kill-recovery.json` | OS `taskkill` recovery |
| `architecture/29_Product_Proof.md` | Narrative product proof |
| `architecture/30_Release_Gate.md` | Gate statuses |
| `architecture/31_Version_1_Release_Checklist.md` | Checklist |

Regenerate evidence: see `architecture/evidence/README.md`.

---

## Architectural authority documents

| Doc | Topic |
| --- | --- |
| `23_Production_Integration_Status.md` | Demo vs production integration |
| `24_Runtime_Observation_Model.md` | Observation |
| `25_Restore_Execution_Model.md` | Restore execution |
| `26_Workspace_Runtime_State.md` | Live runtime state |
| `27_Workspace_Session_Persistence.md` | Session persistence |
| `28_Runtime_Recovery_Model.md` | Recovery |
| `29_Product_Proof.md` | Product proof |
| `30_Release_Gate.md` | Release gate |
| `31_Version_1_Release_Checklist.md` | Checklist |
| `32_Version_1_Baseline.md` | This baseline |
| `architecture/research/experience/experience-freeze.md` | Frozen Experience contract |

Superseded for Experience presentation sequencing: `21_Experience_Roadmap.md` (frozen chrome is authority).

---

## Reproducible build procedure

### Prerequisites

| Tool | Requirement |
| --- | --- |
| Node.js | ≥ 20 |
| pnpm | 9.15.x (`packageManager` in root `package.json`) |
| Rust | **stable ≥ 1.85** (`edition2024` deps in `Cargo.lock`) |
| Windows | SDK / Visual Studio Build Tools for Tauri native shell |
| Display | Required for live Win32 evidence regeneration |

### Fresh checkout

```powershell
git clone <repo-url>
cd Workspace
corepack enable
pnpm install
rustup default stable
```

### Validate (engineering)

```powershell
pnpm typecheck
pnpm build
pnpm test
cargo test -p workspace-domain
cargo test -p workspace-database
cargo test -p workspace-windows-integration --lib
cargo test -p workspace-kernel --lib -- --test-threads=1
cargo check -p workspace-app
```

Kernel observation/session tests share process-local flight state; use `--test-threads=1` for deterministic full-kernel runs.

### Release artifact (Windows)

```powershell
pnpm tauri:build
# installers under target/release/bundle/
```

### Regenerate production evidence (optional; Windows desktop)

```powershell
cargo test -p workspace-windows-integration live_windows_product_proof -- --nocapture
cargo test -p workspace-windows-integration multi_monitor_topology -- --nocapture
cargo test -p workspace-kernel experience_operator_ipc_flow -- --test-threads=1 --nocapture
cargo test -p workspace-kernel os_process_kill_during_restore -- --test-threads=1 --nocapture
```

---

## Package versions

| Package | Version source |
| --- | --- |
| Workspace product | root / workspace `0.1.0` (`CARGO_PKG_VERSION` / `package.json`) |
| Kernel constant | `WorkspaceKernel` / `KERNEL_VERSION` |

Version **1.0** denotes the engineering baseline maturity described here; crate semver may remain `0.1.0` until the tagged release bumps it.
