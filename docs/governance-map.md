# Workspace Governance Map

| Field | Value |
| --- | --- |
| **Purpose** | Document every governance mechanism that constrains authority and safety |
| **Authority** | Kernel security/policy/pipeline/audit; Tauri CSP; constitution/DEC docs where they match code |
| **Date** | 2026-08-07 |

---

## Governance stack (implemented)

```
Constitution / DEC principles (docs)
        ↓ constrains design of
ActorContext + CapabilitySet
        ↓ carried by
CommandPipeline
        ↓ requires
PermissionGateway
   ├── PermissionPolicy (CapabilityBoundPolicy)
   ├── allow-once grants (DB)
   └── PermissionGate (StandardPermissionGate)
        ↓ on Allow
Command execute → AuditService
        ↓ side channel
EventBus → AuditEventSubscriber
```

---

## 1. Permissions

### Actor model

| Actor type | Gate behaviour (`StandardPermissionGate`) |
| --- | --- |
| `LocalUser` | Allowed (if policy allows capability) |
| `System` | Allowed (if policy allows capability) |
| `AIAssistant` | **ApprovalRequired** |
| `Automation` | **ApprovalRequired** |
| `Plugin` | **ApprovalRequired** |
| `RemoteSession` | **ApprovalRequired** |

IPC builds LocalUser context via `app/src-tauri/src/actor.rs`.

### CapabilityBoundPolicy

- **Allow** iff actor’s granted capability set contains the requested capability.
- Missing capability → **Deny**.
- Production default on kernel bootstrap.

### Capability sets

- `CapabilitySet::for_actor_type` supplies nominal capabilities for LocalUser/System.
- Non-human actors have empty nominal sets → must obtain grants / approvals.
- Discovery: `CapabilityResolver` derives/explains; does not grant.

### PermissionGateway decisions

| Decision | Effect |
| --- | --- |
| Allow | Command proceeds; `permission.allowed` audited |
| Deny | Error; `permission.denied` audited |
| ApprovalRequired | Pending approval persisted; `permission.approval_required` audited; command does not execute |

Allow-once grants from DB can satisfy a matching request without re-prompt when present.

### IPC surface for humans

- `get_permission_approvals`
- `decide_approval`

---

## 2. Command Pipeline validation

**File:** `packages/kernel/src/commands/pipeline.rs`

| Step | Behaviour |
| --- | --- |
| Optional intent validation | `ActionIntentValidationService` when mapping present |
| Mutations | Always `PermissionGateway::require` |
| Queries | `read_is_governed` (DEC-017): human + ungoverned may skip; else gateway + audit |
| Success audit | `command.executed` |
| Failure audit | `command.failed` (+ metadata) |

Pipeline is the **only** supported path for state-changing kernel work from IPC.

---

## 3. Audit logging

| Writer | Trigger | Store |
| --- | --- | --- |
| `AuditService::record_command` | Pipeline success/failure | SQLite audit tables |
| `AuditService::record_permission_decision` | Gateway | SQLite |
| `AuditService` AI/planning helpers | AI diagnose paths | SQLite |
| `AuditEventSubscriber` | `DomainEvent` publish | SQLite |

**Properties (by design comments/tests):** command audits avoid payload secrets; permission decisions are durable; observation/scheduler can audit tick outcomes when enabled.

**Reader IPC:** `get_audit_history` (parity / diagnostic).

---

## 4. Safety checks (product proof & observation)

| Check | Implementation | Effect |
| --- | --- | --- |
| Zero ambient capture at startup | Scheduler disabled; startup trigger unwired; ambient gate `false` | No silent desktop read |
| Capture single-flight | `CaptureCoordinator` AtomicBool | Reject concurrent captures |
| Save refuses empty stub | SavedContextService | No fake Moment from stub capturer in production path |
| Restore rematch | DesktopActionService exact-session matching | Fail closed on identity mismatch |
| Restore requires approval path | execute_resume_plan after resolve/plan review in UX | UX + command governance |
| Closed apps not relaunched | RestoreExecutor behaviour | Honest partial outcomes |
| Pilot measurement consent | PilotMeasurementService | Records require consent |
| Experience panels use `invokeIpc` only | Convention + tests | No demo/prod layout branching in frozen panels |

---

## 5. Security boundaries

| Boundary | Mechanism | Status |
| --- | --- | --- |
| OS API | Only `workspace-windows-integration` | Enforced by crate deps |
| DB access from UI | Forbidden; IPC only | No UI import of database crate |
| WebView CSP | `tauri.conf.json` + `verify-content-security-policy` | Enforced |
| Tauri capabilities | `capabilities/default.json` → `core:default` | Minimal |
| Encryption | `NoOpEncryptionProvider` Tier 0 | OS file protection only |
| Plugin sandbox | Vision docs only | **Not implemented** |
| AI worker isolation | DEC-011 | **Not implemented** |

---

## 6. Policy enforcement beyond the gate

| Policy | Where | Notes |
| --- | --- | --- |
| RE vs DE ownership | DecisionEngineService / Recommendation services | Invariants; many via `debug_assert!` (release gap — prior audit) |
| Automation definition ≠ execution | AutomationContractService | prepare intent only |
| Trigger scheduled kind | Domain comments | Never auto-fires |
| Explanation boundary | `verify-ui-experience-boundary.mjs` | UI must not import forbidden reason types |
| Experience freeze catalog | `experienceIpcCatalog.ts` + parity tests | Product Proof command set |
| V2 presentation stage lock | `app/src/experience` + certification tests | 5 resolver stages |

---

## 7. Evidence generation

| Evidence artifact | Producer | Role |
| --- | --- | --- |
| `architecture/evidence/windows-product-proof.json` | windows-integration live test | Win32 behaviour matrix |
| `architecture/evidence/multi-monitor-topology.json` | multi_monitor_proof | Topology (gate FAIL) |
| `architecture/evidence/experience-e2e-behaviour.json` | kernel experience E2E test | Operator IPC behaviour |
| `architecture/evidence/process-kill-recovery.json` | kernel recovery test | Crash/kill recovery |
| DEV localStorage evidence | `app/src/dev/experienceEvidence.ts` | Presentation/governance DEV only |

Release gate narrative: `architecture/30_Release_Gate.md`, baseline `32_Version_1_Baseline.md`.

---

## 8. Frontend / DEV governance (non-OS)

| Mechanism | Location | Authority |
| --- | --- | --- |
| Adaptation certification / packs | `app/src/experience/*` | Presentation adaptation only |
| Engineering certification runner | `app/src/dev/engineeringCertification.ts` | DEV confidence |
| Architectural integrity / maintainability | `app/src/dev/architectural*.ts` | DEV reporting |
| Experience governance proposals | `app/src/dev/experienceGovernance.ts` | DEV localStorage |

These **do not** grant desktop mutation rights.

---

## 9. Documentation governance (process)

| Mechanism | Location |
| --- | --- |
| Project Constitution | `docs/00-Constitution/PROJECT-CONSTITUTION.md` |
| Decision Log / ADRs | `docs/09-Decisions/`, `architecture/decisions/` |
| Cursor / Engineering Session Protocol | `architecture/04_*`, `17_*` |
| Architecture Guardian (proposed) | `architecture/14_Architecture_Guardian.md` — status proposed |
| V2 Agent Handoff constraints | `architecture/V2_AGENT_HANDOFF.md` |

Where docs disagree with code, **code wins** for runtime truth (see knowledge map discrepancies).

---

## 10. Governance gaps (factual)

| Gap | Evidence |
| --- | --- |
| Critical RE/DE invariants in `debug_assert!` | Compiled out in release |
| Large registered IPC surface | Increases review surface even if gated |
| No application-level encryption | Tier 0 noop |
| Plugin/AI process isolation absent | Single process |
| Dual doc authority tracks | Phase language vs V1/V2 architecture docs |
