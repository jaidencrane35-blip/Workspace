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
Caller (manual command today; future triggers later)
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

**CaptureCoordinator** owns capture request admission and concurrency (no queue). It does **not** schedule, timer, or event-hook captures. Actual Win32 capture and persistence remain in `WorkspaceObservationService`.

**Observation status** (`get_workspace_observation_status`) reports whether a snapshot exists, its age/freshness, counts, and last capture failure. It is read-only diagnostics: it does **not** trigger capture and does **not** repair stale observations.

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
