# Replaceability Matrix

| Field | Value |
| --- | --- |
| **Purpose** | Classify every major subsystem for long-term ownership decisions |
| **Parent** | `docs/workspace-strategic-review.md` |
| **Date** | 2026-08-07 |
| **Categories** | NEVER REPLACE · PRESERVE · MODERNISE · WRAP · REPLACE · DELETE |

---

## Decision definitions

| Decision | Meaning |
| --- | --- |
| **NEVER REPLACE** | Removing/replacing abandons product or constitutional identity |
| **PRESERVE** | Keep as Workspace-owned; evolve carefully in place |
| **MODERNISE** | Keep ownership; improve quality/safety/contracts |
| **WRAP** | Keep Workspace interface; commodity impl may sit behind it |
| **REPLACE** | Eligible for substitution *after* ADR — not approved here |
| **DELETE** | Eligible for removal after quarantine — not executed here |

---

## Matrix

| Subsystem | Decision | Evidence | Rationale |
| --- | --- | --- | --- |
| Save Moment / SavedContext semantics | **NEVER REPLACE** | Knowledge map; PP E2E | Product object |
| Resume plan honesty + RestoreExecutor semantics | **NEVER REPLACE** | Restore limits; evidence JSON | Trust wedge |
| Permission-before-automate sequence | **NEVER REPLACE** | Constitution; Gateway | Core governance |
| Ambient capture off (Product Proof) | **NEVER REPLACE** | Kernel init disabled; PP-B02 lineage | Privacy identity |
| Win32 sole-crate rule | **NEVER REPLACE** | windows-integration boundary | Safety architecture |
| Non-execution of RE/DE engines | **NEVER REPLACE** | Service contracts; prior audit | Authority split |
| Enhance-not-replace Windows | **NEVER REPLACE** | Constitution | Category identity |
| PermissionGateway + CapabilityBoundPolicy + StandardPermissionGate | **PRESERVE** | governance-map | Structural platform |
| CommandPipeline + AuditService | **PRESERVE** | pipeline.rs | Sole mutation path |
| Observation explicit Manual path | **PRESERVE** | CaptureCoordinator | Feeds Save |
| WorkspaceRuntimeState + PersistentWorkspaceSession | **PRESERVE** | session migrations 045–046 | Recovery |
| `workspace-domain` purity | **PRESERVE** | crate deps | Contract SoT |
| SQLite schema ownership (Moments, audit, session) | **PRESERVE** | database package | Local-first product data |
| Experience IPC catalog (21) | **PRESERVE** | experienceIpcCatalog.ts | Mounted contract |
| Product Proof chrome mount tree | **PRESERVE** | App.tsx / pilotChrome | User-facing identity |
| Pilot measurement consent model | **PRESERVE** | pilot IPC | Proof loop |
| Thin Tauri command wrappers | **PRESERVE** | app/src-tauri | Process boundary |
| CSP + verifiers | **PRESERVE** | tauri.conf + scripts | Security baseline |
| Domain TS hand sync → codegen | **MODERNISE** | domain-contract-assessment | Drift risk |
| RE/DE `debug_assert!` invariants | **MODERNISE** | AUDIT_REPORT; governance-map | Release safety |
| Documentation authority convergence | **MODERNISE** | docs/README vs architecture/32 | Contributor truth |
| Multi-monitor topology | **MODERNISE** | release gate FAIL | Known gap |
| Encryption tier decision | **MODERNISE** | NoOpEncryptionProvider | Deferred DEC-015 |
| Kernel/domain `WorkspaceState` naming | **MODERNISE** | state-authority-map | Education tax |
| Package README lag | **MODERNISE** | knowledge-map discrepancies | Doc/code sync |
| Tauri / WebView host | **WRAP** | commodity desktop host | Don’t own Chromium |
| SQLite engine (rusqlite bundled) | **WRAP** | commodity DB | Own schema, not engine |
| React / Vite / motion / lucide | **WRAP** | already external | UI commodity |
| Model provider adapters | **WRAP** → future **REPLACE** candidate | stubs today | Interface stays Workspace |
| Logging (`log`/`env_logger`) | **WRAP** | commodity | — |
| Explanation catalog pipeline | **PRESERVE** generator pattern | already generated | Model for domain codegen |
| Sync EventBus internals | **WRAP** / low **REPLACE** | commodity pattern | Interface: DomainEvent |
| DEV evidence/cert dashboards | **WRAP** / partial **REPLACE** | app/src/dev | Not product |
| LLM SDK (if adopted later) | **REPLACE** *(evaluate)* | kiro-readiness | Behind ModelProvider |
| Generic TS codegen toolchain | **REPLACE** *(evaluate)* | assessment | Behind verify script |
| Unmounted CanvasShell | **DELETE** *(candidate)* | superseded | After zero production refs |
| Unmounted OperatorConsole as default gravity | **DELETE** path *(candidate)* or DEV-gate | legacy | After Tier B policy |
| Unused Tier C IPC | **DELETE** *(candidate, late)* | IPC map | Only after parity ADR |
| ObservationStartupTrigger ambient path | **DELETE** *(candidate)* or keep DORMANT | unwired | Prefer DORMANT until ADR |
| Plugin placeholder without runtime | **PRESERVE** placeholder | DEC-011 | Don’t fake a host |
| Cognition new generators | **PRESERVE** existing; **moratorium** on growth | experimental | Consolidation C5 |

---

## Priority bands

### Band 0 — identity (no debate)

NEVER REPLACE rows above.

### Band 1 — preserve platform

Gateway, pipeline, Win32 crate, domain, SQLite product schema, PP chrome/catalog.

### Band 2 — modernise next

Contracts, release invariants, docs authority, multi-monitor, encryption decision.

### Band 3 — wrap commodity

Hosts, engines, UI libs, future model SDKs.

### Band 4 — evaluate replace

Adapters and tooling that do not own desktop truth.

### Band 5 — delete candidates (latest)

Legacy UI gravity and dormant ambient callers — only with explicit ADR and proof that PP/tests remain green.

---

## Explicit non-decisions

| Topic | Status |
| --- | --- |
| Adopt Kiro or any named external platform | **Not decided** — see kiro-integration-strategy |
| Delete IPC commands now | **Rejected for Stabilise** |
| Enable ambient observation | **Rejected** without new Product decision |
| Merge RE/DE | **Rejected** |
| Rewrite UI in non-Tauri stack | **Out of scope** |

---

## Evidence index

| Claim type | Source |
| --- | --- |
| Mounted vs diagnostic IPC | `ipc-surface-map.md` |
| Service dormancy | `runtime-service-map.md` |
| State owners | `state-authority-map.md` |
| Governance structure | `governance-map.md` |
| Contract debt | `domain-contract-assessment.md` |
| Unique vs analogue | `kiro-comparison-readiness.md` |
