# Workspace Attention Engine

| Field | Value |
|-------|-------|
| **Purpose** | A governed prioritization layer over Workspace context |
| **Status** | Foundation — read model / inference only (Sprint 125 contract lock) |
| **Owner** | Lead Software Engineer |

---

## Permanent rule

Attention is informational. Attention grants nothing, authorizes nothing, executes nothing.

```
Human Intent → … → Permission Gateway → Execution → Audit
```

Attention is **not** an observer, desktop monitor, automation engine, or executor.

---

## Stack position

```
Observation
        ↓
WorkspaceStateEngine
        ↓
WorkspaceState
        ↓
Environment
        ↓
Decision Queue / Continuity / Activity / Task Graph / Composition / Purpose / Evolution
        ↓
Attention Engine (canonical prioritization — inference)
        ↓
Workspace Intelligence / Recommendations / Assistant
```

Desktop facts reach Attention **only** via Environment ← WorkspaceState.  
Attention never reads Observation snapshots, DesktopWindow DTOs, or Win32.

No new persistence. Synthetic ids: `attention:{source_type}:{source_id}`.

---

## Facts vs inference

| Layer | Role |
|-------|------|
| Environment, Decision Queue, Continuity, Activity, Task Graph, Composition, Purpose, Evolution | **Facts** (owned elsewhere) |
| Attention `score` / `priority` / `urgency` / `category` / ranked `top_items` | **Inference** |

---

## Canonical APIs

| API | Role |
|-----|------|
| `generate_with_task_graph` | Preferred shared-input path (Intelligence / Operating State) |
| `generate` | Standalone / IPC diagnostics — loads Environment via WorkspaceStateEngine once |

Ordering is deterministic: score DESC → priority rank ASC → id ASC.  
When Environment is present, Composition does not re-project desktop-like gaps (`disconnected_work`, `missing_application`).

---

## Scoring

Deterministic integer scores with `score_factors` explanations:

| Source | Base guidance |
|--------|----------------|
| Blockers | high / immediate |
| Outstanding decisions | high / soon |
| Interrupted / commitments | medium |
| Resumable / current focus | normal |
| Environment gaps | medium (desktop disconnect / missing apps) |
| Dormant / recent progress | low / can wait |

Intelligence may surface Attention tops as `recommended_actions` (priorities).  
The Recommendation Engine is a separate suggestion surface.

---

## Audits (`authority_effect: none`)

- `workspace.attention.generated`
- `workspace.attention.summary.generated`

---

## Related

- [Workspace Environment Model](WORKSPACE-ENVIRONMENT-MODEL.md)
- [Workspace Continuity Engine](WORKSPACE-CONTINUITY-ENGINE.md)
- [Workspace Platform Coherence](WORKSPACE-PLATFORM-COHERENCE.md)
- [Governed Decision Queue](GOVERNED-DECISION-QUEUE.md)
- [Workspace Intelligence Foundation](WORKSPACE-INTELLIGENCE-FOUNDATION.md)
