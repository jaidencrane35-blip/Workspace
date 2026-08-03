# 42 — Experience Validation

Status: Complete (Sprint 51)  
Date: 2026-08-03  
Branch: `v2-dev`  
Baseline: Sprint 50 commit `01e8a8c` (perceptual optimum / engineering boundary)  
Authority: `architecture/41_Perceptual_Convergence.md` · Concept boards (Aug 1 2026)  
Implementation: `app/src/dev/`

---

## 1. Objective

Replace internal parity scoring with evidence from human interaction.

Version 2 now records usability signals — confidence latency, cognitive effort proxies, abandon/recovery — instead of visual similarity to concept boards.

This document is evidence-only. It does not recommend product changes.

---

## 2. Instrumentation model

### 2.1 Gate

| Environment | Default | Override |
|---|---|---|
| Development (`import.meta.env.DEV`) | **On** | `VITE_DISABLE_EXPERIENCE_VALIDATION=1` disables |
| Production (`import.meta.env.PROD`) | **Off** | `VITE_ENABLE_EXPERIENCE_VALIDATION=1` enables locally |

Implementation: `app/src/dev/experienceValidationGate.ts`  
Bootstrap: `initExperienceInstrumentation()` in `app/src/main.tsx` (no-op when gate closed).

### 2.2 Event schema

Schema version: `1`  
Module: `app/src/dev/experienceEvents.ts`

| Field | Meaning |
|---|---|
| `seq` | Monotonic sequence within a session |
| `t` | Milliseconds from session start (relative; replay-stable) |
| `type` | Allowlisted event type |
| `destination` / `from` | Allowlisted view ids (`home`, `save`, `resume`, `pilot`, `help`, `unknown`) |
| `flow` | `save` \| `continue` \| `none` |
| `modality` | `pointer` \| `keyboard` \| `unknown` |
| `commandId` | Allowlisted command id only |
| `targetKind` | Opaque role token (`^[a-z0-9_.:-]{1,64}$`) |

Event types:

- `session_start`
- `first_meaningful_interaction`
- `navigate`
- `flow_start`
- `save_success`
- `continue_success`
- `flow_abandon`
- `command`
- `repeated_action`
- `backtrack`
- `modality`

### 2.3 Capture points (production UI behaviour unchanged)

| Signal | Source |
|---|---|
| Session start | Gate open at bootstrap |
| First meaningful interaction | First `pointerdown` / `keydown` |
| Navigation | `App.navigate`, `goContinue`, dock / home / pilot / save entry |
| Save success | `SaveContextPanel` after successful `save_workspace_context` |
| Continue flow start | `ResumeContextPanel.openPreview` after plan resolve |
| Continue success | `ResumeContextPanel.approveAndRestore` after `execute_resume_plan` |
| Abandon | Navigate away while `save` / `continue` flow active |
| Command / modality | Keyboard chord + modality listeners |

When the gate is closed, every `track*` function is a no-op. No React tree structure, layout, or copy changes for production.

### 2.4 Storage

| Property | Value |
|---|---|
| Medium | `localStorage` (browser) or in-memory adapter (tests) |
| Key | `ws.dev.experience.validation.v1` |
| Ring buffer | Max 20 sessions; max 2000 events per session |
| Transport | None — no network, IPC export, or analytics sink |

---

## 3. Privacy guarantees

| Guarantee | Mechanism |
|---|---|
| No user content | Events never include handoff notes, names, titles, window text, messages, or free-text labels |
| Forbidden keys | `FORBIDDEN_EVENT_KEYS` rejected by schema / audited in tests |
| Allowlisted enums only | Destinations, modalities, command ids, event types |
| Opaque targets | `targetKind` regex-constrained; no UI string capture |
| Local-only | `localStorage` / memory; no outbound telemetry APIs |
| Production off by default | Gate closed unless explicit local env force |
| Sanitize on write | `sanitizeEvent` strips unknown fields before append |

Forbidden event keys (audit list):

`handoff`, `handoffNote`, `note`, `name`, `title`, `label`, `text`, `content`, `summary`, `windowTitle`, `userContent`, `message`

---

## 4. Derived metrics (cognitive friction model)

Pure function: `computeFrictionScore(session)` in `app/src/dev/frictionModel.ts`.  
Same session bytes → same score (no clocks, no I/O).

| Component | Derivation |
|---|---|
| `timeToConfidenceMs` | `t` of first `first_meaningful_interaction` (else last event `t`) |
| `unnecessaryNavigation` | Navigations to `save`/`resume` without corresponding success (or all such navs if never succeeded) |
| `interactionRedundancy` | Count of `repeated_action` |
| `attentionSwitching` | Consecutive destination changes among `navigate` events |
| `flowInterruption` | Count of `flow_abandon` |
| `successfulRecovery` | `1` if `continue_success` after any abandon or backtrack; else `0` |

Evidence block (counts only):

- `eventCount`, `saveSuccess`, `continueSuccess`
- `firstInteractionAt`
- `pointerEvents`, `keyboardEvents`

---

## 5. Friction calculation

Composite score ∈ `[0, 1]` (0 = low friction, 1 = high).

Normalized inputs (capped at 1):

| Input | Normalization |
|---|---|
| time | `min(1, timeToConfidenceMs / 15000)` |
| unnecessary nav | `min(1, unnecessaryNavigation / 6)` |
| redundancy | `min(1, interactionRedundancy / 8)` |
| switching | `min(1, attentionSwitching / 10)` |
| interruption | `min(1, flowInterruption / 3)` |

Weights:

| Term | Weight |
|---|---:|
| time | 0.22 |
| unnecessary nav | 0.20 |
| redundancy | 0.18 |
| switching | 0.18 |
| interruption | 0.22 |

Recovery relief: subtract `0.08` when `successfulRecovery === 1`.  
Final score rounded to 4 decimal places.

Reproducibility: `frictionEquals(a, b)` compares score + all components.

---

## 6. Validation workflow

1. Run app in development (instrumentation on by default).
2. Perform Save / Continue / navigation tasks with real users or scripted operators.
3. Sessions persist under `ws.dev.experience.validation.v1`.
4. Load session → `computeFrictionScore` → record score + components as evidence.
5. Optional: `replaySession(session)` to reproduce destination sequence and recompute friction.
6. Compare friction across builds using identical traces (deterministic).

Production builds: instrumentation remains inactive unless explicitly force-enabled for a local validation session.

---

## 7. Session replay

Module: `app/src/dev/sessionReplay.ts`

| Property | Evidence |
|---|---|
| Deterministic | Applies events in `seq` / array order; uses relative `t`; no wall-clock sleep |
| Scope | Navigation destinations + flow markers; no IPC; no production runtime mutation |
| Output | `applied` count, destination list, timeline `{t,type,destination}`, friction |
| Gate | Development tooling only; not mounted in production UI |

Replay purpose recorded here: reproduce UX issues, compare versions, validate regressions against stored traces.

---

## 8. Evidence format

### 8.1 Stored bundle

```json
{
  "schemaVersion": 1,
  "sessions": [
    {
      "schemaVersion": 1,
      "sessionId": "ev-…",
      "startedAt": 0,
      "events": [
        { "seq": 0, "t": 0, "type": "session_start", "destination": "home" }
      ]
    }
  ]
}
```

### 8.2 Friction evidence record

```json
{
  "score": 0.0,
  "components": {
    "timeToConfidenceMs": 0,
    "unnecessaryNavigation": 0,
    "interactionRedundancy": 0,
    "attentionSwitching": 0,
    "flowInterruption": 0,
    "successfulRecovery": 0
  },
  "evidence": {
    "eventCount": 0,
    "saveSuccess": 0,
    "continueSuccess": 0,
    "firstInteractionAt": null,
    "pointerEvents": 0,
    "keyboardEvents": 0
  }
}
```

### 8.3 Engineering boundary

- No Runtime Core changes.
- No production Experience behaviour changes when gate is closed.
- No additional runtime dependencies introduced for this sprint.
- Visual parity scoring (Sprint 50) remains closed; further product change requires human-interaction evidence from this pipeline.
