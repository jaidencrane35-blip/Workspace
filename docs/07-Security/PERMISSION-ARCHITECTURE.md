# Permission Architecture

| Field | Value |
|-------|-------|
| **Purpose** | Seal document for how VibeLock / Workspace decides whether an action is allowed |
| **Owner** | Lead Software Engineer |
| **Status** | Sealed (Sprint 45) — foundation for future AI / plugin / automation actors |
| **Dependencies** | [Security Principles](SECURITY-PRINCIPLES.md), [Threat Model](THREAT-MODEL.md), [IPC Surface](../03-Engineering/IPC-SURFACE.md) |
| **Update Process** | Material authority-path changes require a Decision Log entry and this doc update |

---

## 1. One-sentence answer

**Every privileged action enters the Command Pipeline, is evaluated by the Permission Gateway, and only proceeds on explicit Allow** (capability grant or allow-once grant). No production service may skip that path.

---

## 2. Authority path

```
Frontend / IPC / future AI / plugin / automation
        ↓
CommandHandler (actor + intent attached)
        ↓
CommandPipeline
        ↓
PermissionGateway.require
        ↓
Policy (CapabilityBoundPolicy) + Gate (StandardPermissionGate)
        + active allow-once grants
        ↓
Allow | Deny | ApprovalRequired
        ↓
Execution (only on Allow)
```

### Universal boundary

| Layer | Role |
|-------|------|
| **CommandPipeline** | Sole execution entry for mutations/queries that declare capabilities |
| **PermissionGateway** | Sole authority decision + decision audit + grant merge/consume |
| **Domain services** | Perform work after authorization; launch/approval mutators are `pub(crate)` |

### Explicit exceptions (documented, not bypasses)

| Path | Why |
|------|-----|
| `get_settings` (kernel convenience) | Read-only bootstrap helper; not a privileged desktop action |
| `begin_shutdown` | Lifecycle teardown; audited as System actor |
| Test stubs `AlwaysAllowPolicy` / `AllowAllPermissionGate` | **Test-only.** Production `WorkspaceKernel` wires `CapabilityBoundPolicy` + `StandardPermissionGate` |

There is **no production AllowAll path**.

---

## 3. Actor model

Actors identify *who* requested the action. They do **not** receive private execution APIs.

| Actor type | Default capabilities | Typical outcome without grant |
|------------|----------------------|-------------------------------|
| `LocalUser` | Standard local set | Allow when capability present |
| `System` | System set | Allow for system operations |
| `AIAssistant` | Empty | ApprovalRequired |
| `Automation` | Empty | ApprovalRequired |
| `Plugin` | Empty | ApprovalRequired |
| `RemoteSession` | Empty | ApprovalRequired |

Rules:

1. Every actor type uses the same pipeline → gateway path.
2. Non-human actors start with empty capability sets (`CapabilitySet::for_actor_type`).
3. Future AI / plugins / automation **must** appear as actors; they must not call launch or approval services directly (those are crate-internal).

---

## 4. Capability model

```
Capability (required by command)
    ↓
Policy evaluation (is capability grantable / bound?)
    ↓
Gate authorization (is capability present on actor?)
    ↓
Optional allow-once CapabilityGrant (command-scoped)
    ↓
Decision → Execution
```

- Capabilities are named domain tokens (e.g. application launch / workspace write).
- Allow-once grants are **command-scoped**: `command_name` must match the request command.
- Grants are consumed when the gateway returns Allow for a matching grant (before handler body runs).
- This sprint does **not** redesign the capability catalog; it seals evaluation and lifecycle.

---

## 5. Approval lifecycle

```
Non-human request lacking authority
        ↓
Gateway → ApprovalRequired
        ↓
Persist pending PermissionApprovalRequest
        ↓
LocalUser DecideApproval (allow_once | deny)
        ↓
Allow once → CapabilityGrant(active, command-scoped)
Deny → terminal Denied (no grant)
        ↓
Retry original command as same actor
        ↓
Grant matches → Allow → grant consumed
```

Safety properties:

- Pending or denied requests **do not** authorize execution.
- Only `LocalUser` may decide approvals.
- Decide is atomic (`UPDATE … WHERE status = 'pending'`) so double-decide cannot create two grants.
- After consume, a second retry requires a new approval.

---

## 6. Audit events

Permission decisions record at least:

| Field | Source |
|-------|--------|
| Actor | `actor_id` / `actor_type` on audit row |
| Intent | metadata `intent_type` |
| Command / resource | `command_name`, metadata `subject` |
| Decision | event type + metadata `decision` |
| Reason | metadata `reason` |
| Timestamp | audit row timestamp |
| Approval id | metadata `approval_request_id` when applicable |

Event types:

- `permission.allowed`
- `permission.denied`
- `permission.approval_required`

These support debugging, security review, and future user-facing history.

---

## 7. Future extension points (do not invent yet)

Safe to add later **inside** this framework:

- AI observer / suggestion → still an `AIAssistant` actor through the pipeline
- Automation engine → `Automation` actor + approvals
- Plugins → `Plugin` actor + capability grants
- Lasting (non allow-once) grants and a permission center UI

Unsafe / out of scope until redesigned:

- Direct service calls that spawn processes or mutate approvals
- Restoring AllowAll on the production kernel
- Parallel “AI authority” APIs that skip the gateway

---

## 8. Seal checklist (Sprint 45)

- [x] Permission Gateway is the universal authority boundary for command execution
- [x] Privileged launch / approval mutators are crate-internal
- [x] Allow-once grants are command-scoped and consumed on Allow
- [x] Approval decide is pending-atomic; non-LocalUser cannot decide
- [x] Permission decision audits carry actor, intent, command, decision, reason
- [x] Actor model ready for AI / plugin / automation without private authority paths
- [x] Boundary tests cover approval lifecycle, denial, actor separation, kernel defaults

---

## Related code

| Concern | Location |
|---------|----------|
| Gateway | `packages/kernel/src/security/gateway.rs` |
| Gate / policy | `standard_gate.rs`, `policy/capability_bound.rs` |
| Pipeline | `packages/kernel/src/commands/pipeline.rs` |
| Approval service | `packages/kernel/src/services/permission_approval.rs` |
| Launch service | `packages/kernel/src/services/application_launch.rs` |
| Production wiring | `packages/kernel/src/lib.rs` (`WorkspaceKernel::bootstrap_shell`) |
