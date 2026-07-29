# Visual Review Checklist — Milestone A

| Field | Value |
|-------|-------|
| **Checkpoint name** | Milestone A — Apps + Workspace Switcher foundation |
| **Date** | 2026-07-29 |
| **Branch / PR** | `cursor/milestone-a-workspace-apps-switcher-34a5` |
| **Related batch** | Milestone A (Product Alignment Audit) |
| **Human review required?** | **Yes** ([HUMAN-REVIEW-POLICY.md](HUMAN-REVIEW-POLICY.md) — Workspace home, switching UI, application interaction) |

---

## 1. What changed

- Primary navigation: **Home**, **Workspaces**, **Applications**, **Layouts**
- Assistant demoted to secondary tab styling; Work/Diagnostic quiet
- Workspace switcher: list / create / activate saved workspaces
- Applications: register, identity, governed launch, observed actives
- Workspaces view pairs switcher with existing desktop arrangements rail (unavailable windows via restore diagnostics)

## 2. Why it changed

Make core Workspace product loops visible without expanding AI or replacing DAF owners.

## 3. What needs verification

- Home reads as Workspace-first (not Assistant-first)
- Switching workspaces is obvious and current workspace is clear
- Applications show identity and launch only when executable path exists
- Arrangements rail still shows unavailable/missing windows honestly after restore
- Assistant does not feel like the main product

## 4. Expected behaviour

- Tauri: list/create/switch/launch invoke real IPC
- Browser Vite: runtime hints; controls disabled where appropriate; UI still renders

## 5. Known limitations

- No OS app store/discovery
- Observation empty until capture
- Sidecar Assistant deferred

## 6. How to inspect

| Method | Used? | Notes |
|--------|-------|-------|
| Live app (browser Vite) | Yes | `cd app && pnpm exec vite` → http://localhost:1420 |
| Screenshot(s) | Optional | Capture Home, Workspaces, Applications if reviewing remotely |
| Screen recording | No | Not required |

**Launch:** `cd app && pnpm exec vite` → open **Home**, then **Workspaces**, **Applications**, **Layouts**.

## 7. Reviewer outcome

| Criterion | Pass / Fail / N/A | Notes |
|-----------|-------------------|-------|
| Usability | ☐ | Pending human |
| Visual quality | ☐ | Pending human |
| Product feel | ☐ | Pending human |
| Matches visual direction (non-literal) | ☐ | Pending human |
| Navigation / interaction | ☐ | Pending human |
| Apps + switcher | ☐ | Pending human |

**Decision:** ☐ Accept  ☐ Changes requested  ☐ Defer
