# 30 — Release Gate

Each subsystem has exactly one status.  
**PASS** requires production evidence (`architecture/29_Product_Proof.md` + `architecture/evidence/windows-product-proof.json`).  
Stub/demo results do not qualify.

Evidence host: Windows build **26200**, single monitor, commit lineage through product-proof sprint.

---

## Gate table

| Subsystem | Status | Basis |
| --- | --- | --- |
| Observation | **PASS** | Live Win32 capture on host; proof window enumerated; Win32 + product-proof tests green |
| Save | **PASS** | Live Moment persisted from Win32 capture with restore identities |
| Runtime State | **PASS** | Live publish + restart hydrate restored durable session fields |
| Restore Planning | **PASS** | Live plan against saved live Moment produced items |
| Restore Execution | **PASS WITH LIMITATIONS** | Live place/minimize/restore/focus/primary placement committed; see limitations below |
| Persistence | **PASS** | Atomic session checkpoint + hydrate verified on live path |
| Recovery | **PASS WITH LIMITATIONS** | Fence dispositions deterministic; corrupt session non-fatal; see limitations below |
| Experience chrome (frozen) | **PASS WITH LIMITATIONS** | Unchanged; freeze anchors + a11y green; Tauri click-through E2E not run |
| Multi-monitor / topology change | **FAIL** | Dual-monitor and disconnect/reconnect not evidenced on proof host |

---

## PASS WITH LIMITATIONS — detail

### Restore Execution
- `SetForegroundWindow` may be refused by Windows without foreground input permission (refusal path retained; committed on proof host).
- `window.z_order` is unsupported by Action contract; place uses `SWP_NOZORDER`.
- Closed applications are never relaunched.
- Secondary-monitor placement not evidenced (single-monitor host).

### Recovery
- Crash is simulated by durable fence left uncleared, not by OS `TerminateProcess` during Tauri mutation.
- Effects already applied to the desktop before interruption are not rolled back by the OS.

### Experience chrome
- Production IPC catalog parity and frozen screenshots pass.
- End-to-end operator click path through Tauri shell (Home → Save → Continue → Restart app → Continue) was not instrumented on this host.

---

## FAIL — blockers

### Multi-monitor / topology change
1. Dual-monitor placement not exercised — host had one display (`\\.\DISPLAY1`).
2. Monitor disconnect/reconnect not exercised — no topology change event injected or observed.

These do not revoke single-monitor PASS statuses above; they block any release claim of multi-monitor or hot-plug fidelity.

---

## Validation record (this gate)

| Suite | Result |
| --- | --- |
| `cargo test -p workspace-windows-integration --lib` | 21 passed (includes live product proof) |
| `cargo test -p workspace-kernel windows_product_proof` | 3 passed |
| `cargo test -p workspace-kernel workspace_recovery` | 12 passed |
| `cargo test -p workspace-kernel workspace_session_store` | 7 passed |
| `cargo test -p workspace-database persistent_session` | 5 passed |
| `pnpm typecheck` | ok |
| `pnpm test` (incl. freeze anchors + a11y + boundary) | 74 passed |

---

## Release recommendation

**Release the runtime core for single-monitor Windows sessions** with the documented OS limitations above.

**Do not claim** dual-monitor fidelity, monitor hot-plug recovery, or fully instrumented Tauri Experience click-through until those have production evidence.

Frozen Experience chrome remains the operator surface; runtime adapts to it.
