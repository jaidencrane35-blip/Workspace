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
DesktopWindowService (SoT for raw windows)
        ↓
Workspace Environment Model   ← this document
        ↓
Attention / Intelligence / Continuity consumers
```

---

## Represents

- Running applications (matched to registered Workspace apps)
- Application windows and window groups
- Window state (open / focused; minimized unknown without deeper Win32)
- Soft display label (monitors not yet exposed by enumerator)
- Workspace / project / task association via WorkflowContext
- Layout association (canvas layout — not OS monitors)
- Gaps: missing apps, disconnected work

Does **not** duplicate Windows Integration — aggregates `DesktopWindowService`.

---

## Boundaries

May: observe, match, explain, feed intelligence.  
Must not: move windows, launch apps, grant permissions, change Gateway behavior, hide monitoring.

Audit: `workspace.environment.generated` with `authority_effect: none`.
