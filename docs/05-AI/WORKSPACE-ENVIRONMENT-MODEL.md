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
WorkspaceStateEngine → WorkspaceState (canonical runtime projection)
        ↓
Workspace Environment Model   ← this document (consumes WorkspaceState)
        ↓
Attention / Intelligence / Continuity consumers
```

**Future event-driven path (Sprint 117 — gateway only; Event admission still rejected):**

```
OS / adapter event (not wired yet)
        ↓
ObservationEventGateway  (validate → normalize → map)
        ↓
ObservationTriggerAuthority
        ↓
Admission (Event rejected until enabled)
```

This gateway is the **only** future path for event-driven observation. It is not a Win32 listener, not a hook, and not automation.
**CaptureCoordinator** owns capture request admission and concurrency (no queue). It does **not** schedule, timer, or event-hook captures. Actual Win32 capture and persistence remain in `WorkspaceObservationService`. Capture requests carry provenance (`source` / optional `reason` / `context`) into distinguishable lifecycle audits (`requested` → `started` → `completed` | `failed`, or `rejected_concurrent`).

**ObservationScheduler** owns schedule timing (`ObservationScheduleConfig`: `enabled`, `interval_seconds`). Starts after Ready on production `WorkspaceKernel::initialize`, stops on `begin_shutdown`, stays disabled for in-memory tests. Ticks call only `ObservationScheduledTrigger` (context `schedule:ObservationScheduler`). No Event/Plugin callers. Runtime health is exposed via `get_observation_scheduler_status` (`ObservationSchedulerStatus`) — read-only, no control, no snapshot load. Lifecycle audits: `workspace.observation.scheduler.started` / `.stopped` / `.tick`.

**ObservationTriggerAuthority** evaluates `ObservationTriggerRequest`s: audit received → **admission policy** → refresh policy → ignore / block / accept → `CaptureCoordinator` only when capture is needed. Admission allows `Manual` / `System` / `Scheduled` and rejects `Event` / `Plugin`; rate-limits repeated admits (process-local cooldown). Callers: `ObservationStartupTrigger` (once at Ready), `ObservationScheduledTrigger` (explicit tick or scheduler tick), and `ObservationEventGateway` (normalized events — admitted path not enabled). Scheduled freshness is `NotStale`.

**ObservationEventGateway** accepts `ObservationEvent` contracts, validates/normalizes them, and maps to `ObservationTriggerRequest` (`source=Event`, `reason=normalized_event`). It does **not** listen to the OS, call Win32, capture, schedule, or hook events. Until Event admission is enabled, gateway forwards are rejected before capture.
**ObservationRefreshPolicyService** answers whether a new observation should be requested (`FreshEnough` / `RefreshRequired` / `ObservationUnavailable` / `RefreshBlocked`). It is read-only: it does **not** capture. Consumer freshness needs (`ObservationConsumerFreshnessNeed`) are contracts only — not yet wired into Environment / Intelligence.

**Observation status** (`get_workspace_observation_status`) reports whether a snapshot exists, its age/freshness, counts, and last capture failure. It is read-only diagnostics: it does **not** trigger capture and does **not** repair stale observations.

**Observation delta** (`get_latest_observation_delta`) compares the latest and immediately previous persisted snapshots into `WorkspaceObservationDelta` facts (opened/closed/focus/moved/resized/minimized/monitor). Pure comparison — no capture, no decisions, no automation. Fewer than two snapshots yields an empty/no-change delta.

**WorkspaceState** (`get_workspace_state`) is the canonical **runtime state projection**:
- **Observation** = facts (persisted snapshots)
- **Delta** = change between consecutive snapshots
- **WorkspaceState** = current interpreted state built from latest observation + latest delta

`WorkspaceStateEngine` is read-only: no persistence, capture, scheduling, Win32, AI, or automation. Distinct from kernel lifecycle `WorkspaceState` (version/ready).

**WorkspaceEnvironmentService** consumes **WorkspaceState** as its primary runtime input (Sprint 119). It no longer loads observation snapshots or DesktopWindowService on the production `generate` path. Registered-app matching, gaps, groups, workflow, and layout binding remain Environment responsibilities.

**WorkspaceIntelligenceService** also consumes **WorkspaceState** (Sprint 120): one `WorkspaceStateEngine::get_current` load per intelligence cycle, passed into `WorkspaceEnvironmentService::generate_from_state`. It does not load observation snapshots or DesktopWindowSnapshot on the production path.

---

## Represents

- Running applications (matched to registered Workspace apps)
- Application windows and window groups
- Window state (open / focused / minimized from observation snapshots)
- Display label from observed monitor assignment
- Workspace / project / task association via WorkflowContext
- Layout association (canvas layout — not OS monitors)
- Gaps: missing apps, disconnected work

Does **not** capture or enumerate the OS — aggregates via **WorkspaceState** (observation + delta projection).

---

## Boundaries

May: observe, match, explain, feed intelligence.  
Must not: move windows, launch apps, grant permissions, change Gateway behavior, hide monitoring.

Audit: `workspace.environment.generated` with `authority_effect: none`.
