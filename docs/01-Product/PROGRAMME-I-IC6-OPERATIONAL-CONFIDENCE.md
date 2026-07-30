# Programme I — Implementation Contract 6 (Planning)  
# Operational Confidence

| Field | Value |
|-------|-------|
| **Authority** | Principal Architect |
| **Implementation agent** | Cursor (after approval to commence) |
| **Programme** | [Programme I](PROGRAMME-I-WORKSPACE-PRODUCT-CAPABILITY.md) |
| **Depends on** | [IC5](PROGRAMME-I-IC5-USER-CONFIDENCE-AND-DISCOVERABILITY.md) (approved) |
| **Status** | **Planned** — awaiting Principal Architect approval to commence |
| **Date** | 2026-07-30 |
| **Nature** | Planning contract — scopes IC6; does not authorise implementation until approved |

---

## Context

Programme I has completed and approved IC1–IC5. The architecture has stabilized; product experience has matured through composition, editing refinement, and discoverability — without new sources of truth.

The Principal Architect recommends the next phase shift from expanding desktop editing toward **operational confidence**: helping users understand what Workspace is doing and why, using explanations derived only from existing state.

---

## Objective (planned)

Help users answer, from the product surface:

1. **What is Workspace doing right now?**
2. **Why is it doing it?**
3. **What will happen if I press Restore?**
4. **Is this Arrangement current?**
5. **What changed since it was captured?**

All answers must be **explanations derived from existing state** — not new runtime models, logging systems, or event histories.

Success means an engineer concludes: *the product became more understandable without becoming architecturally more complicated.*

---

## Architectural constraints (mandatory)

| Constraint | Rule |
|------------|------|
| WorkspaceState | Sole runtime desktop truth |
| Arrangement | Existing persistence only |
| Restore | Sole product OS `set_bounds` path |
| IPC | Ownership unchanged |
| Observation / Intelligence | Untouched as expansion targets |
| Product vs Interaction State | Preserved (IC5) — interaction never persists |
| Sources of truth | **No new ones** |
| Logging / telemetry | No new operation log, audit UI, or notification engine |

IC6 is an **explanation / confidence sprint**, not an architecture or telemetry sprint.

---

## Exploration areas (in scope for planning)

### 1. Current desktop / product status line

Compose a quiet “what’s happening” interpretation from facts already available, for example:

- Observation load state (reading / ready / error / runtime unavailable)
- Window / monitor counts from WorkspaceState
- Active Profile name
- Selected Arrangement (if any)
- Edit-session phase when editing (Interaction State — already derived in IC4)

No background status service.

### 2. Restore consequence preview (pre-flight explanation)

Before Restore, surface what Restore is expected to do using existing Arrangement entries + live window identity match (IC5 meta / IC4 diff patterns):

- How many windows would be targeted
- How many appear present vs missing on the desktop now
- Bounds completeness / restore readiness
- Reminder that Restore is the path that moves windows (Gateway → WindowController)

Still **visual/explanatory only** until the user presses Restore. No plan/preview IPC that applies bounds.

### 3. Arrangement currency

Answer “is this Arrangement current?” by deriving from:

- Overlap of membership with live desktop
- Bounds comparison (same pure field comparison as IC4 change awareness)
- `updated_at` already on Arrangement

Labels such as **Current**, **Out of date**, **Partial** — computed, never stored.

### 4. What changed since capture

Reuse / extend the IC4 `diffLayoutEditingChanges` style comparison for **viewing** (not only edit mode):

- Added / removed / moved relative to saved Arrangement
- Unchanged count
- Presented as explanation copy, not a second comparison engine

### 5. Why feedback (post-operation)

Enrich existing Save / Update / Restore success messages (IC5) with optional one-line “why” context when facts are already on the result DTO (e.g. gaps, simulated, failed counts) — still interpretation only.

---

## Explicit non-goals

- New desktop / Arrangement / editing / persistence models
- New IPC for positioning or “dry-run restore apply”
- Operation history, event bus, notification engine
- Background synchronization or metadata cache
- Expanding Intelligence / Assistant ownership of Desktop control
- Redesign or experimental chrome
- Further expansion of interactive layout editing mechanics (unless required for explanation clarity)

---

## Proposed deliverables (when approved to implement)

1. Operational status / explanation helpers derived from WorkspaceState + Arrangement + restore DTOs.
2. Pre-Restore consequence summary on Desktop / Arrangements (compose existing readiness/overlap).
3. Arrangement currency + “what changed” explanation (view mode, not only edit mode).
4. Documentation of Product vs Interaction State for any new UI cues.
5. This contract updated to **Complete** with validation evidence.
6. Validation: `pnpm typecheck`, `pnpm test`, `pnpm build`; working tree clean.

---

## Validation requirements (implementation phase)

- No new runtime model
- No duplicate persistence
- No operation log / telemetry pipeline
- Restore remains sole OS positioning authority
- WorkspaceState remains runtime truth
- Explanations remain pure derivations
- Green typecheck / test / build
- Clean working tree

---

## Stop condition (this planning document)

IC6 planning is complete when objectives, constraints, and non-goals are recorded and the Principal Architect can approve or amend scope.

**Cursor must not commence IC6 implementation until the Principal Architect approves this contract for execution.**

---

## Recommendation to Principal Architect

Approve IC6 to commence as an **operational confidence** refinement: status, Restore consequences, Arrangement currency, and change-since-capture explanations — all derived from WorkspaceState, Arrangement, and existing restore/result DTOs only.
