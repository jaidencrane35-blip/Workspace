# Windows product-proof evidence

| File | Producer | Notes |
| --- | --- | --- |
| `windows-product-proof.json` | `cargo test -p workspace-windows-integration live_windows_product_proof` | Behaviour matrix |
| `multi-monitor-topology.json` | `cargo test -p workspace-windows-integration multi_monitor_topology` | Dual-monitor claims require `hardware_dual_monitor: true` |
| `experience-e2e-behaviour.json` | `cargo test -p workspace-kernel experience_operator_ipc_flow` | Production IPC behavioural E2E |
| `process-kill-recovery.json` | `cargo test -p workspace-kernel os_process_kill_during_restore` | OS `taskkill` recovery |

Optional multi-monitor hot-plug wait:

```powershell
$env:WORKSPACE_MULTI_MONITOR_HOTPLUG_SECONDS = "60"
cargo test -p workspace-windows-integration multi_monitor_topology -- --nocapture
```

Regenerate on a real Windows desktop before updating gate statuses.
