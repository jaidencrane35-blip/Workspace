# 31 — Version 1.0 Release Checklist

Every item links to existing evidence or tests. No aspirational entries.

Authority: `architecture/30_Release_Gate.md`, `architecture/29_Product_Proof.md`.  
Baseline: `architecture/32_Version_1_Baseline.md`.

---

## Runtime

- [x] Canonical `WorkspaceRuntimeState` published — [26](26_Workspace_Runtime_State.md), tests `workspace_runtime_state_tests`
- [x] Live observation on Windows — `evidence/windows-product-proof.json`, `win32::tests::*`
- [x] Restart hydrate restores durable fields — `evidence/experience-e2e-behaviour.json` (`restart_hydrate`)
- [x] Operator runtime IPC — `get_workspace_runtime_state` in E2E evidence

## Persistence

- [x] `PersistentWorkspaceSession` schema v2 — [27](27_Workspace_Session_Persistence.md), migration `046_*`
- [x] Atomic singleton upsert — `persistent_session` DB tests
- [x] Success-path checkpoints — `workspace_session_store_tests`
- [x] File-backed session survives process restart — `experience-e2e-behaviour.json` (`post_restart_session_saved_context_id`)

## Recovery

- [x] Recovery model documented — [28](28_Runtime_Recovery_Model.md)
- [x] Fence dispositions unit-tested — `workspace_recovery_tests`
- [x] OS process kill during restore fence — `evidence/process-kill-recovery.json`, `os_process_kill_during_restore_fence_*`
- [x] Corrupt session non-fatal — `live_corrupt_session_never_crashes_startup` / recovery suite

## Restore

- [x] Planning + `RestoreCompatibilitySummary` — E2E `continue_plan` step
- [x] Execution + `RestoreExecutionSummary` — E2E `restore` / `continue_again`; product-proof matrix
- [x] Place uses `SWP_NOZORDER` — [25](25_Restore_Execution_Model.md), product-proof notes
- [x] Known OS refusals retained — gate Restore Execution limitations

## Experience

- [x] Frozen chrome unchanged this release train — Experience files not modified in gate-closure commits
- [x] Production IPC catalog parity — `tests/production-integration-parity.test.ts`
- [x] Behavioural operator flow (Launch→…→Continue again) — `evidence/experience-e2e-behaviour.json`
- [x] Demo adapter remains non-authority — [23](23_Production_Integration_Status.md)

## Accessibility

- [x] Accessibility smoke in Vitest suite — `pnpm test` (experience / pilot chrome tests)
- [x] UI experience boundary verify — `scripts/verify-ui-experience-boundary.mjs`

## Performance

- [x] Observation single-flight concurrency — `workspace_runtime_state_tests::concurrent_observation_*`
- [x] Restore history bounded (≤20) — domain `RESTORE_HISTORY_LIMIT`, recovery long-run test
- [x] No ambient capture at startup — Product Proof policy in `WorkspaceKernel::initialize`

## Testing

- [x] Windows integration suite — `cargo test -p workspace-windows-integration --lib`
- [x] Kernel recovery / session / product-proof / E2E / process-kill — filters in [30](30_Release_Gate.md)
- [x] Frontend typecheck + Vitest — `pnpm typecheck`, `pnpm test`
- [x] Freeze screenshot anchors — Vitest experience contract / freeze assets

## Windows limitations

- [x] Single-monitor only claimed — `evidence/multi-monitor-topology.json` (`hardware_dual_monitor: false`)
- [x] Dual-monitor / hot-plug **not** claimed — gate row **FAIL** until hardware evidence
- [x] `SetForegroundWindow` may refuse — product-proof + gate limitations
- [x] Closed apps not relaunched — [25](25_Restore_Execution_Model.md)
- [x] Desktop effects not OS-rolled-back after kill — recovery limitations in [30](30_Release_Gate.md)

## Evidence inventory

| File | Producer |
| --- | --- |
| `architecture/evidence/windows-product-proof.json` | `live_windows_product_proof_capture_and_behaviour_matrix` |
| `architecture/evidence/multi-monitor-topology.json` | `multi_monitor_topology_harness_writes_evidence` |
| `architecture/evidence/experience-e2e-behaviour.json` | `experience_operator_ipc_flow_launch_through_continue_again` |
| `architecture/evidence/process-kill-recovery.json` | `os_process_kill_during_restore_fence_recovers_deterministically` |
| `architecture/29_Product_Proof.md` | Narrative product proof |
| `architecture/30_Release_Gate.md` | Gate statuses |
