# Product Proof Review Brief

| Field | Value |
| --- | --- |
| **Date** | 2026-08-07 |
| **Audience** | Project Owner |
| **Purpose** | Interactive review of the mounted Product Proof — not polish, not feature expansion |
| **Machine state** | `docs/project-health.json` |
| **Launch** | `pnpm dev` → Tauri `workspace-app.exe` + Vite `http://localhost:1420/` |
| **Ready for interactive review** | **Yes** |

---

## Current application state

| Item | Status |
| --- | --- |
| Frontend production build | Pass (`pnpm build`) |
| Typecheck / Vitest / constitutional verifiers | Pass |
| Tauri native shell | **Launched successfully** (2026-08-07) |
| Kernel | Ready — SQLite at `%AppData%\com.workspace.app\workspace.db` |
| Runtime crashes observed at launch | **None** |
| Product IPC after boot | `GetWorkspace`, `ListSavedContexts` executed (real Tauri IPC, not browser demo) |

The desktop window should be open from `pnpm --filter @workspace/app exec tauri dev` (or `pnpm dev`). Vite serves the UI; the native shell hosts full IPC.

---

## What launches

- Native Workspace window (Tauri 2)
- Mounted Experience chrome: **Home · Save · Continue · Check-in · Guide**
- Kernel + local SQLite persistence
- Product Proof IPC path (20 commands) for Save / Resume / Pilot / settings / workspace bootstrap

---

## What does not launch / is unavailable in Product Proof

| Surface | Status |
| --- | --- |
| OperatorConsole / Diagnostic | Present in codebase; **not** primary chrome |
| CanvasShell | Unmounted from Product Proof |
| WorkspaceIntelligencePanel | Unmounted |
| AssistantPanel | Unmounted |
| Ambient observation scheduler | Constitutionally **off** |
| Plugin marketplace / host | Not productized |
| Full AI orchestration UX | Experimental / developer — not Product Proof |

---

## Current user journey (review this)

1. **Home** — workspace presence; list of saved Moments  
2. **Save** — name a Moment, review capture scope, consent, save  
3. **Continue** — pick a Moment, preview restore plan, execute within Windows session limits  
4. **Check-in** — consented pilot measurement (baseline / leave-resume / interview)  
5. **Guide** — help copy (no IPC)

Honesty constraints to watch: restore limits messaging, no ambient capture, handoff note is user-authored.

---

## Known placeholders / experimental / developer-only

| Kind | Examples |
| --- | --- |
| Placeholders | Capture/restore honesty copy; pilot interview prompts |
| Experimental | ~177 non-Product IPC commands; cognition / automation / AI services registered but not Product Proof chrome |
| Developer-only | OperatorConsole, Canvas, Intelligence, Assistant; `VITE_FORCE_EXPERIENCE_DEMO` browser adapter |
| Demo adapter | Activates only when Tauri is absent in DEV — **not** what you are reviewing in the native shell |

---

## Known issues (review-relevant)

1. **Doc vocabulary drift** — older `IPC-SURFACE.md` language still says “Product” for Operator surfaces; constitutional Product = Experience catalog only (G3).  
2. **`domain.ts` remainder** — non–Product Proof types still hand-maintained (G1 remainder).  
3. **WorkspaceState name collision** — kernel lifecycle vs domain desktop (G4); does not block Save/Continue review.  
4. **Large IPC surface** — 197 registered commands; Product Proof uses 20.  
5. Kernel compiles with many unused-item warnings — noisy, not launch-blocking.

---

## Repository health review (Part 3)

| Question | Finding |
| --- | --- |
| Is the repository healthy? | **Yes** — build/test/typecheck/verifiers green; native app launches |
| Are constitutional documents aligned? | **Mostly** — Constitution V2 + milestone + backlog consistent; historical IPC “Product” wording still diverges (G3) |
| Is documentation still authoritative? | **Partially** — machine truths (`ipcTiers`, `productContracts`, `project-health.json`) are authoritative; prose inventories lag |
| Has technical debt been reduced? | **Yes** — IPC tiers, PP contracts, project-health automation |
| Are verifiers functioning? | **Yes** — explanation, ipc-tiers, contracts, project-health, UI boundary, CSP |

No architecture redesign performed in this review.

---

## What the Project Owner should review

1. Does **Home → Save → Continue → Check-in → Guide** feel like one Product Proof?  
2. Is Save consent / scope honest and understandable?  
3. Is Continue restore preview + limits messaging trustworthy?  
4. Is Check-in clearly pilot evaluation (not ambient product telemetry)?  
5. Any crash, dead end, or confusing chrome that looks like an IDE/agent product?  
6. Priority signal: after review, should engineering continue **docs convergence** or address a Product Proof UX defect first?

---

## Next constitutional execution program

**Default (after owner review):** Documentation authority convergence (Phase A #2 / G3).

**Do not start it until this Product Proof review is accepted.**

Blocked next program is recorded in `docs/project-health.json` → `nextRecommendedExecutionProgram`.

---

## Ready for interactive review?

# YES

Native Product Proof is running with kernel ready and Product IPC responding. Please review the desktop window, then direct the next engineering priority.
