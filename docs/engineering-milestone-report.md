# Engineering Milestone Report
## P16.10 Product Completion & Engineering Stabilization

| Field | Value |
| --- | --- |
| **Execution program** | P16.10 Product Completion & Engineering Stabilization |
| **Date** | 2026-08-07 |
| **Status** | **Engineering Complete** — Product Complete **Owner-only** |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` |
| **Not** | P17 / Product Complete |

---

## Crash root cause

Long-running WinRT recognition ran **on the Tauri IPC/async worker**, freezing the shell and destabilizing WebView (exit `0xcfffffff` / abrupt exit). Aggressive continuous-session stitch restart compounded COM teardown races.

## Latency root cause

`voice_status` / focus re-checks called `warm_up` + **MediaCapture** probe + recognizer compile on the hot path (5–30s when cold or contended).

## Fixes

- `spawn_blocking` for listen + warm  
- `status()` peek-only (no compile, no MediaCapture)  
- MediaCapture once inside `warm_up`  
- Stitch gap + soft-fail  
- Bridge no longer auto-opens Settings  
- Glass denser; Owner Directed Product Proof + Engineering Verification Separation permanent  

## Explicit

- **P16 Product Complete:** **No — Owner only**  
- **P17:** Not begun  
