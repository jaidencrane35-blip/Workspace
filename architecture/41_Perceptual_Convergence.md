# 41 — Perceptual Convergence

Status: Complete (Sprint 50)  
Date: 2026-08-03  
Branch: `v2-dev`  
Baseline: Sprint 49 commit `6592901` (overall parity **9.35**)  
Authority: `architecture/40_Experience_Refoundation.md` · Concept boards (Aug 1 2026)  
Evidence: `architecture/research/experience/screenshots/v2-sprint-50/`

---

## 1. Objective

Replace subjective refinement with measurable perception analysis.  
Every change must be justified by evidence against the concept boards.  
No new features, interaction models, or runtime systems.

---

## 2. Method

### 2.1 Pipeline

Automated script: `screenshots/v2-sprint-50/analyze.mjs`

For each destination screenshot (Sprint 49 capture):

| Destination | Concept reference | Mode |
|---|---|---|
| Home | board-b cell (Modular / Minimal Immersive) | `minimal-immersive` |
| Save | board-b cell (Sidebar Focus) | `focus` |
| Continue | board-a right immersive mock | `minimal-immersive` |
| Check-in | board-b cell (Sidebar Focus) | `focus` |
| Guide | board-b cell (Minimal Immersive) | `minimal-immersive` |

Machine metrics (weighted):

| Metric | Weight |
|---|---:|
| Composition similarity | 0.14 |
| Focal-point placement | 0.14 |
| Object hierarchy | 0.14 |
| Whitespace distribution | 0.12 |
| Typography density | 0.10 |
| Contrast distribution | 0.10 |
| Edge density | 0.10 |
| Visual balance | 0.10 |
| Motion intent (implementation) | 0.06 |

Motion intent is scored from the Sprint 49 four-purpose motion model (`attention` / `continuity` / `memory` / `reconstruction`), not from pixels.

### 2.2 Acceptance filter for implementation

A candidate is implemented only when **all** hold:

1. Expected overall parity gain ≥ **0.05**
2. No destination regression risk
3. Not refused by architecture/40 §9 (AI rails, gauges, live thumbnails, spectacle)

Otherwise: **do not force changes**.

---

## 3. Automated perceptual metrics

Machine output: `screenshots/v2-sprint-50/perceptual-metrics.json`

| Destination | Weighted similarity to accepted mode | Residual gap |
|---|---:|---:|
| Home | 0.627 | 0.373 |
| Save | 0.627 | 0.373 |
| Continue | 0.644 | 0.356 |
| Check-in | 0.731 | 0.269 |
| Guide | 0.611 | 0.389 |

**Overall convergence score (similarity):** **0.648**

Interpretation: residual gaps are dominated by concept-board elements Product Proof **refuses** (scenic photographic fills, multi-app window chrome, AI side rails, system gauges). Against *accepted* modes (single-focus stage, quiet dock, glass object hierarchy), Sprint 49 already occupies the viable region.

Motion intent score: **0.92** (four-purpose model present and mapped).

---

## 4. Ranked remaining deltas

### 4.1 Measured gaps (informational)

| ID | Severity | Confidence | Residual | Notes |
|---|---|---:|---:|---|
| GAP-GUIDE | S2 | 0.80 | 0.389 | Lowest similarity; instructional chrome already observational |
| GAP-HOME | S2 | 0.80 | 0.373 | Whitespace/atmosphere vs photographic immersive boards |
| GAP-SAVE | S2 | 0.80 | 0.373 | Focus mode boards include side utilities we refuse |
| GAP-CONTINUE | S2 | 0.80 | 0.356 | Board-a immersive includes multi-window cluster |
| GAP-CHECKIN | S1 | 0.80 | 0.269 | Closest of the five |

### 4.2 Candidate changes (ranked by expected value)

| ID | Expected gain | Cost | Regression risk | EV | Verdict |
|---|---:|---:|---|---:|---|
| D-WHITESPACE-HOME | 0.03 | 1 | yes | 0.03 | Reject |
| D-FOCAL-LIFT-GUIDE | 0.03 | 2 | yes | 0.015 | Reject |
| D-EDGE-SOFTEN | 0.02 | 1 | no | 0.02 | Reject (<0.05) |
| D-DELETE-OBSOLETE-VARIANTS | 0.01 | 1 | no | 0.01 | Reject as parity driver; hygiene only |
| D-ATMOSPHERE-SCENIC | 0.04 | 3 | yes | 0.013 | Reject |
| D-AI-SIDEBAR | 0.00 | 8 | yes | 0 | Discard (§9) |
| D-SYSTEM-GAUGES | 0.00 | 6 | yes | 0 | Discard (§9) |
| D-LIVE-THUMBNAILS | 0.00 | 9 | yes | 0 | Discard (§9) |

**Actionable set (gain ≥ 0.05, no regression): empty.**

---

## 5. Implemented deltas

| Change | Type | Rationale |
|---|---|---|
| Remove unused `.elevated-card--e1…e4` shadow ladder | Engineering hygiene | Dead CSS after Sprint 49 material unification; **no perceptual claim**; net LOC reduction |
| Perceptual pipeline + report artefacts | Measurement | Enables future EV-gated work |

No destination visual redesign. No new tokens, components, or interaction model.

---

## 6. Rejected deltas (rationale)

1. **Refused board literalism** (AI rail, gauges, live thumbnails) — architecture/40 §9.  
2. **Scenic atmosphere** — estimated +0.04 with Save/Continue hierarchy regression risk.  
3. **Whitespace expansion / Guide focal enlargement** — estimated +0.03; undoes S44–S48 quiet-chrome and semantic-field gains.  
4. **Further edge softening** — +0.02 < 0.05 threshold after Sprint 49 craftsmanship.

---

## 7. Decision

**Declare Sprint 49 the perceptual optimum for the current architecture.**

Reasons:

1. No candidate clears the ≥0.05 overall parity gain bar without regression risk.  
2. Remaining machine gap vs raw concept boards is largely composed of **refused** or **out-of-scope** board elements (multi-app OS chrome, AI utilities, gauges, photographic spectacle).  
3. Against accepted modes (Minimal Immersive stage + Focus single-surface + Continuity reconstruction), Sprint 49 already concentrates attention on one Moment object with shared material, four-purpose motion, and quiet chrome.  
4. Forcing toward ≥9.45 by cloning refused board features would violate Product Proof authority and likely regress trust/calm dimensions.

Parity holds at **9.35**. Target **9.45** is documented as unreachable without architecture/scope change — not as a craftsmanship debt.

---

## 8. Parity matrix (Sprint 50)

| | Sprint 49 | Sprint 50 | Δ |
|---|---:|---:|---:|
| Overall | 9.35 | **9.35** | **0.00** |
| Home | 9.40 | 9.40 | 0.00 |
| Save | 9.35 | 9.35 | 0.00 |
| Continue | 9.38 | 9.38 | 0.00 |
| Check-in | 9.29 | 9.29 | 0.00 |
| Guide | 9.33 | 9.33 | 0.00 |

No destination regression.  
Justified no-change vs forced-polish.

---

## 9. Validation

- `pnpm typecheck`  
- `pnpm test`  
- `node …/v2-sprint-50/analyze.mjs` → `decision.action = "no-change"`  
- Screenshots stored under `v2-sprint-50/` (Sprint 49 captures + concept boards)

---

## 10. Forward condition

Resume perceptual implementation only when a candidate proves **expected overall gain ≥ 0.05** under the same pipeline **and** does not require refused concept-board elements.
