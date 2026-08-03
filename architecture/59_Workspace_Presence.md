# 59 — Workspace Presence

Status: Complete (Sprint 68)  
Date: 2026-08-03  
Branch: `v2-dev`  
Baseline: Sprint 67 commit `c9b2f5f`  
Authority: `architecture/58_Workspace_Memory_Evolution.md` · Version 2 docs `40`–`58`  
Implementation: `app/src/experience/workspacePresence.ts`

---

## 1. Objective

Transform Workspace from a collection of adaptive surfaces into a coherent environmental presence.

Improves perception, continuity, and environmental presence only.

No Runtime Core changes. No persistence changes. No navigation changes.  
No AI decision-making. All presentation flows through the certified adaptation pipeline.

Evidence only — no recommendations.

---

## 2. Presence schema

Type: `WorkspacePresence` (derived — not persisted)

| Field | Content |
|---|---|
| `presenceId` | Content-addressed id (`presence-…`) |
| `environmentalCalm` | 0–1 calm quality |
| `spatialContinuity` | 0–1 continuity (from grouping) |
| `focalGravity` | 0–1 focal emphasis quality |
| `contextualAtmosphere` | 0–1 atmosphere quality |
| `visualBreathingRhythm` | 0–1 spacing/motion rhythm |
| `contributingAdaptationIds` | Certified adaptation ids |
| `evolutionId` / `evolutionEpoch` | Memory evolution lineage |
| `certificationIds` | Certification lineage |
| `resolvedEnvironment` | Lighting, spacing rhythm, atmosphere, motion cadence, focal emphasis, depth weighting |
| `active` | `true` only when validation passes |
| `validation` | Failure reasons + composition status |

Derived only from Runtime State, WorkspaceMemoryEvolution, and active certified adaptations.  
No user content. No AI state. No persistence.

---

## 3. Resolver ordering

```
Runtime State
    ↓
Certified Adaptation Pack
    ↓
Workspace Memory Evolution
    ↓
Workspace Presence
    ↓
Presentation
```

Functions:

- Intermediate: `resolvePresentationFromRuntime` (pack → evolution)  
- Final: `resolvePresentationWithPresence`  
- Live hook: `useResolvedPresentation` → presence resolver  

When presence is inactive/invalid, presentation equals the evolution/adaptation fallback.

---

## 4. Environmental model

| Presence quality | Source (certified presentation) | Resolved environment |
|---|---|---|
| environmentalCalm | inverse of env/motion extremes | — |
| spatialContinuity | `groupingTightness` | `groupingTightness` |
| focalGravity | `emphasisScale` | `focalEmphasis` |
| contextualAtmosphere | `environmentalWeight` | `atmosphericIntensity` / `lighting` |
| visualBreathingRhythm | spacing + motion cadence | `spacingRhythm` / `motionCadence` |

Presence may influence only: environmental lighting, spacing rhythm, atmospheric intensity, motion cadence, focal emphasis, depth weighting.

Presence must not alter: layout topology, information hierarchy, navigation, Runtime Core, persistence.

---

## 5. Validation

| Check | Failure |
|---|---|
| Certified contributing adaptations | `no_certified_lineage` |
| Active memory evolution | `evolution_inactive` |
| Composition passed | `composition_regressed` |
| Governance lineage intact | `lineage_incomplete` |
| Stability when scores exist | `composition_unstable` |
| Contributors ⊆ evolution originating ids | `presentation_bypass_attempt` |

Failed validation ⇒ `active: false` — presence not applied.  
Presence never bypasses certified adaptations.

Deterministic replay: `replayWorkspacePresence(store)`.

---

## 6. Governance linkage

Presence reuses:

- WorkspaceMemoryEvolution  
- WorkspaceAdaptation  
- Certified Adaptation Packs  
- AdaptationComposition  
- Governance primitives  

No new governance systems. No new adaptation abstractions.

Authority:

```
40 → … → 58 → 59
```
