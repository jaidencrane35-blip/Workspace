# AI Context Foundation

| Field | Value |
|-------|-------|
| **Purpose** | Give AI controlled, read-only awareness of the workspace |
| **Status** | Sprints 50–51 (Batch 3) |
| **Dependencies** | [AI Planning Foundation](AI-PLANNING-FOUNDATION.md), Sprint 19 WorkspaceContext |

---

## 1. Rule

**AI may observe. AI may not modify.**

```
WorkspaceContextService (ContextProvider)
        ↓  (local-user provider identity)
AiWorkspaceAwareness  ("what exists?")
        ↓
AiPlanningContext
        ↓
Action Proposals
        ↓
Permission Gateway  ("is this allowed?")
```

Awareness never grants authority.

---

## 2. ContextProvider

Existing `WorkspaceContext` + `WorkspaceContextService` are the ContextProvider.

They answer: applications, zones, layout, observations, metrics, capabilities discovery, execution summary.

AI planning does **not** call mutators and does **not** receive a private write path.

---

## 3. `AiWorkspaceAwareness`

Bounded view injected into planning:

- workspace id / name
- zone count, layout id
- applications (id, name, identifier, `appears_active`)
- recent observation count
- environment window titles (desktop enumeration)

`appears_active` is a heuristic from window titles (e.g. title contains "Notepad").

---

## 4. Context-aware planning

For “Prepare my workspace”:

- Propose launch for registered apps that do **not** appear active
- Skip apps that already appear open
- Still route every proposal through the Permission Gateway

---

## 5. Out of scope (Batches 4–7)

Action catalogs, proposal evaluation metrics, multi-step orchestration lifecycle, assistant UX — see [Intelligence Roadmap](INTELLIGENCE-ROADMAP.md).
