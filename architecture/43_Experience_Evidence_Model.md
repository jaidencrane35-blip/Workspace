# 43 — Experience Evidence Model

Status: Complete (Sprint 52)  
Date: 2026-08-03  
Branch: `v2-dev`  
Baseline: Sprint 51 commit `8456188`  
Authority: `architecture/42_Experience_Validation.md` · Version 2 docs `40`–`42`  
Implementation: `app/src/dev/traceAnalysis.ts` · `experienceEvidence.ts` · `ExperienceEvidenceDashboard.tsx`

---

## 1. Objective

Close the loop between development, human interaction, replay, and measured comparison.

Experience work consumes derived evidence from Sprint 51 traces. This document records the model, algorithms, and privacy surface. Evidence only — no product recommendations.

---

## 2. Trace lifecycle

```
human interaction
  → Sprint 51 instrumentation (interaction events, local)
  → session ring buffer (`ws.dev.experience.validation.v1`)
  → analyzeTraces (deterministic aggregate)
  → ExperienceEvidence snapshot (derived metrics only)
  → evidence store (`ws.dev.experience.evidence.v1`)
  → baseline comparison / dashboard (DEV)
  → optional replaySession (integrity / reproduction)
```

| Stage | Storage key | Contents |
|---|---|---|
| Raw traces | `ws.dev.experience.validation.v1` | Allowlisted interaction events |
| Evidence | `ws.dev.experience.evidence.v1` | Derived metrics + hotspot/loop summaries |
| Production | — | Neither active by default |

Gate: `isExperienceValidationEnabled()` (Sprint 51). Dashboard mount additionally requires `import.meta.env.DEV` so production bundles omit the viewer chunk.

---

## 3. Aggregation model

Module: `app/src/dev/traceAnalysis.ts`  
Function: `analyzeTraces(sessions)`

Properties:

- Pure function — no I/O, no wall clock
- Sessions sorted by `sessionId` before analysis (stable identity)
- Same session set → same `TraceAggregate`

Derived signals:

| Signal | Derivation |
|---|---|
| Median time-to-confidence | Median of per-session `timeToConfidenceMs` from friction model |
| Friction distribution | min / max / mean / median / p25 / p75 of session friction scores |
| Hesitation hotspots | Gaps ≥ 800ms ending at a `navigate`, attributed to prior destination; median gap + sample count |
| Navigation loops | Count of A→B→A triples in navigate destination sequences |
| Abandoned flows | Counts of `flow_abandon` by `save` / `continue` |
| Recovery success | Sessions with `successfulRecovery` / sessions with `flowInterruption` |
| Replay divergence | Per session: two replays + friction vs `computeFrictionScore`; destination sequence vs navigate events |

No user content is read. Only allowlisted event fields and destinations participate.

---

## 4. Evidence schema

Type: `ExperienceEvidence` (`schemaVersion: 1`)  
Builder: `buildEvidenceFromSessions` → `aggregateToEvidence`

| Field | Meaning |
|---|---|
| `evidenceId` | Stable id (`evd-<fingerprint>` or allowlisted override) |
| `fingerprint` | FNV-1a of sorted session ids + metrics JSON |
| `tag` | Opaque short token (`^[a-z0-9_.:-]{1,64}$`) |
| `sourceSessionIds` | Session id list only |
| `metrics` | Flat numeric / allowlisted destination metrics |
| `hotspots` | `{ destination, medianGapMs, samples }[]` |
| `loops` | `{ pattern, count }[]` (pattern from allowlisted destinations) |

`ExperienceEvidence` never embeds raw interaction events.

Metrics surface (`ExperienceEvidenceMetrics`):

- `sessionCount`
- `medianTimeToConfidenceMs`
- `meanFrictionScore`, `medianFrictionScore`, `frictionMin`, `frictionMax`, `frictionP25`, `frictionP75`
- `hesitationHotspotCount`, `topHesitationDestination`, `topHesitationMedianGapMs`
- `navigationLoopCount`
- `abandonedFlowTotal`, `abandonedSave`, `abandonedContinue`
- `recoverySuccessRate`, `recoveryCount`, `interruptionCount`
- `replayCount`, `replayDivergenceRate`
- `saveSuccessTotal`, `continueSuccessTotal`

Evidence bundle also stores:

- `baselineId` — selected snapshot id
- `replayInvocations` — local count of dashboard/tool replay calls
- Ring buffer of snapshots (max 40)

---

## 5. Comparison algorithm

Function: `compareEvidence(baseline, candidate)`

Comparable metrics (`COMPARABLE_METRICS`) with direction and epsilon:

| Metric | Direction | Epsilon |
|---|---|---:|
| `medianTimeToConfidenceMs` | lower better | 1 |
| `meanFrictionScore` | lower better | 0.0001 |
| `medianFrictionScore` | lower better | 0.0001 |
| `frictionP75` | lower better | 0.0001 |
| `hesitationHotspotCount` | lower better | 0 |
| `topHesitationMedianGapMs` | lower better | 1 |
| `navigationLoopCount` | lower better | 0 |
| `abandonedFlowTotal` | lower better | 0 |
| `recoverySuccessRate` | higher better | 0.0001 |
| `replayDivergenceRate` | lower better | 0.0001 |

Verdict per metric:

1. If `|candidate − baseline| ≤ epsilon` → `unchanged`
2. Else if direction is lower-better and candidate < baseline → `improved`
3. Else if direction is higher-better and candidate > baseline → `improved`
4. Else → `regressed`

Summary counts: `improved` / `unchanged` / `regressed`.  
No subjective or visual scores participate.

---

## 6. Privacy guarantees

| Guarantee | Mechanism |
|---|---|
| No user content in traces | Sprint 51 `sanitizeEvent` + forbidden keys |
| No user content in evidence | Metrics / allowlisted destinations / opaque labels only |
| Separate persistence | Evidence key distinct from trace key |
| Local-only | `localStorage` / memory adapters; no network APIs in `app/src/dev` |
| DEV visibility | Dashboard dynamic-imported under `import.meta.env.DEV` |
| Production off by default | Validation gate closed in PROD unless explicit force |

Forbidden content keys remain those listed in `architecture/42_Experience_Validation.md` §3. Evidence writers do not introduce free-text fields.

---

## 7. Development workflow

1. Run app in development (instrumentation on by default).
2. Perform Save / Continue / navigation with human operators.
3. Open **Evidence** control (DEV host overlay — not a product route).
4. **Analyze traces** → writes `ExperienceEvidence` snapshot from current sessions.
5. **Set as baseline** on a chosen snapshot.
6. After further sessions / builds, analyze again and read regression summary.
7. **Replay sessions** to exercise deterministic replay and increment local replay count.

Production behaviour: unchanged. No product route, no Runtime Core changes, no feature expansion.

---

## 8. Engineering boundary

- Reuses Sprint 51 event schema, friction model, and replay.
- Does not duplicate event models.
- Evidence tooling remains behind the Experience Validation gate.
- Production UI and Runtime Core are out of scope for this sprint.
