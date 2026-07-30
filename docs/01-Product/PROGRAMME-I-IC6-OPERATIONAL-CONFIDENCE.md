# Programme I — Implementation Contract 6  
# Operational Confidence

| Field | Value |
|-------|-------|
| **Authority** | Principal Architect |
| **Implementation agent** | Cursor |
| **Programme** | [Programme I](PROGRAMME-I-WORKSPACE-PRODUCT-CAPABILITY.md) |
| **Depends on** | [IC5](PROGRAMME-I-IC5-USER-CONFIDENCE-AND-DISCOVERABILITY.md) (approved) |
| **Status** | Complete |
| **Date** | 2026-07-30 |
| **Approved to commence** | 2026-07-30 |

---

## Objective (satisfied)

Make Workspace **explain itself** — operational transparency as a projection of existing truth, not a new model.

An operator should answer without documentation:

| Question | Answered by |
|----------|-------------|
| What is Workspace doing? | Current activity surface |
| Why is it doing it? | Explain Ownership (`because … · Owner`) |
| Is Restore safe / what happens next? | Pre-Restore explanation |
| Is this Arrangement current? | Currency: Current / Out of date / Partial / Unavailable |
| What changed? | Since-capture change line |

---

## Philosophy

> Explanation is not another model. It is a projection of existing truth.

| Kind | Examples | Rule |
|------|----------|------|
| **Product State** | Profile, Desktop, Arrangement, Restore | Authoritative |
| **Interaction State** | Editing, Preview, In-flight op, Last restore result, Guidance dismissed | Session-only; never persisted |

---

## Deliverables implemented

### 1. Current activity surface
`explainOperationalActivity` derives activity + why + owner from load state, Arrangement selection, edit/preview flags, and in-flight Interaction State (observe / save / update / restore).

Examples: Observing desktop · Arrangement selected · Preview active · Restore executing · Restore complete · Waiting for desktop changes.

No activity engine, scheduler, or status persistence.

### 2. Pre-Restore explanation
`explainPreRestore` projects comparison + IC5 meta into bullets:

- move N windows  
- leave N unchanged (already matching)  
- ignore unavailable  
- skip entries without stored bounds  

States **why Restore is available** (Arrangement data + Restore as sole apply path). Visual/explanatory only until Restore runs.

### 3. Arrangement currency
Four derived states only: **Current · Out of date · Partial · Unavailable**.  
Never persisted; recomputed when WorkspaceState / Arrangement selection changes.

Each line uses Explain Ownership:  
`Arrangement is current because every tracked window matches the current desktop membership and bounds · Desktop`

### 4. What changed since capture
Reuses IC4 `diffLayoutEditingChanges` → `+ N · − N · moved · unchanged`.

### 5. Richer post-Restore explanation
`explainPostRestore` / `arrangementRestoredExplanationMessage` interpret existing restore DTO counts (applied, skipped, gaps, failed, simulated) with why + Restore ownership.

---

## Explicit non-goals (honoured)

No event history, timeline, telemetry, operation log, explanation cache, confidence scoring, background analysis, or persistent diagnostics.

---

## Preserved boundaries

| Constraint | Status |
|------------|--------|
| WorkspaceState sole runtime truth | Preserved |
| No new models / persistence | Preserved |
| Restore sole `set_bounds` path | Preserved |
| IPC ownership | Preserved |
| Observation / Intelligence untouched | Preserved |
| Explanations disappear/change with state | Preserved |

---

## Validation

- `pnpm typecheck` · `pnpm test` · `pnpm build`
- Architecture / IPC / UI experience verifies
- Working tree clean

---

## Files touched

- `app/src/lib/operationalConfidenceUi.ts`
- `app/src/components/WorkspaceApplicationStage.tsx`
- `app/src/components/DesktopArrangementPanel.tsx`
- `app/src/App.css`
- `tests/operational-confidence-ui.test.ts`
- This document; Programme I charter

---

## Stop condition

IC6 complete. **Await Principal Architect review** before further Programme I contracts.
