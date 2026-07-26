# Workspace Environment Model

| Field | Value |
|-------|-------|
| **Purpose** | Canonical read model of the live desktop as it relates to Workspace work |
| **Owner** | Architecture |
| **Status** | Phase 5 foundation |
| **Authority** | Never creates authority (`authority_effect: none`) |

---

## Principle

The Workspace remains a companion over Windows. The Environment Model understands **where work is happening** without becoming an OS.

```
ObservationScheduler (runtime ticks; production initialize only)
        ↓
ObservationScheduledTrigger (Scheduled / scheduled_refresh)
        ↓
ObservationTriggerAdmissionPolicy
        ↓
ObservationTriggerAuthority
        ↓
ObservationRefreshPolicyService
        ↓
CaptureCoordinator (single-flight lifecycle authority)
        ↓
WorkspaceObservationService (SoT for desktop observation)
        ↓
SQLite observation snapshots
        ↓
DesktopWindowService (adapter/read model)
        ↓
Workspace Environment Model   ← this document
        ↓
Attention / Intelligence / Continuity consumers
```

**CaptureCoordinator** owns capture request admission and concurrency (no queue). It does **not** schedule, timer, or event-hook captures. Actual Win32 capture and persistence remain in `WorkspaceObservationService`. Capture requests carry provenance (`source` / optional `reason` / `context`) into distinguishable lifecycle audits (`requested` → `started` → `completed` | `failed`, or `rejected_concurrent`).

**ObservationScheduler** owns schedule timing (`ObservationScheduleConfig`: `enabled`, `interval_seconds`). Starts after Ready on production `WorkspaceKernel::initialize`, stops on `begin_shutdown`, stays disabled for in-memory tests. Ticks call only `ObservationScheduledTrigger` (context `schedule:ObservationScheduler`). No Event/Plugin callers. Runtime health is exposed via `get_observation_scheduler_status` (`ObservationSchedulerStatus`) — read-only, no control, no snapshot load. Lifecycle audits: `workspace.observation.scheduler.started` / `.stopped` / `.tick`.

**ObservationTriggerAuthority** evaluates `ObservationTriggerRequest`s: audit received → **admission policy** → refresh policy → ignore / block / accept → `CaptureCoordinator` only when capture is needed. Admission allows `Manual` / `System` / `Scheduled` and rejects `Event` / `Plugin`; rate-limits repeated admits (process-local cooldown). Callers: `ObservationStartupTrigger` (once at Ready) and `ObservationScheduledTrigger` (explicit tick or scheduler tick). Scheduled freshness is `NotStale`.

**ObservationRefreshPolicyService** answers whether a new observation should be requested (`FreshEnough` / `RefreshRequired` / `ObservationUnavailable` / `RefreshBlocked`). It is read-only: it does **not** capture. Consumer freshness needs (`ObservationConsumerFreshnessNeed`) are contracts only — not yet wired into Environment / Intelligence.

**Observation status** (`get_workspace_observation_status`) reports whether a snapshot exists, its age/freshness, counts, and last capture failure. It is read-only diagnostics: it does **not** trigger capture and does **not** repair stale observations.

**Observation delta** (`get_latest_observation_delta`) compares the latest and immediately previous persisted snapshots into `WorkspaceObservationDelta` facts (opened/closed/focus/moved/resized/minimized/monitor). Pure comparison — no capture, no decisions, no automation. Fewer than two snapshots yields an empty/no-change delta.

---

## Represents

- Running applications (matched to registered Workspace apps)
- Application windows and window groups
- Window state (open / focused / minimized from observation snapshots)
- Display label from observed monitor assignment
- Workspace / project / task association via WorkflowContext
- Layout association (canvas layout — not OS monitors)
- Gaps: missing apps, disconnected work

Does **not** capture or enumerate the OS — aggregates persisted observation via `DesktopWindowService`.

---

## Boundaries

May: observe, match, explain, feed intelligence.  
Must not: move windows, launch apps, grant permissions, change Gateway behavior, hide monitoring.

Audit: `workspace.environment.generated` with `authority_effect: none`.
