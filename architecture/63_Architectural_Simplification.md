# 63 — Architectural Simplification

Status: Complete (Sprint 72)  
Date: 2026-08-03  
Branch: `v2-dev`  
Baseline: Sprint 71 commit `e760bbb`  
Authority: `architecture/62_Presentation_Stability.md` · Version 2 docs `40`–`62`  
Implementation: `app/src/experience/experienceMath.ts` · `architecturalComplexity.ts`

---

## 1. Objective

Reduce implementation complexity while preserving every externally observable behaviour.

Engineering simplification only. No Runtime Core, Experience, presentation, navigation, persistence, or behavioural changes.

Evidence only — no recommendations.

---

## 2. Complexity metrics

| Metric | Before | After | Δ |
|---|---|---|---|
| Duplicated clamp01/round4 sites | 5 | 0 | −5 |
| Duplicated populationVariance sites | 2 | 0 | −2 |
| Deprecated presentation aliases | 1 | 0 | −1 |
| Shared primitive modules | 1 | 2 | +1 (`experienceMath`) |
| Resolver stages | 5 | 5 | 0 |

Report: `buildArchitecturalComplexityReport()`.

---

## 3. Removed duplication

| Pattern | Consolidated into |
|---|---|
| Local `clamp01` / `round4` | `experienceMath.ts` |
| Local `populationVariance` | `experienceMath.ts` |
| Local `lerp` | `experienceMath.ts` |
| Local `meanOf` | `experienceMath.ts` |
| `mergePresentation` deprecated alias | `applyPresentationLayer` |
| Unused `deriveWorkspacePresence` in stability | removed |

---

## 4. Dependency reductions

- `workspacePresentationStability` no longer imports presence solely for a void soft-signal.  
- Math call sites depend on `experienceMath` rather than parallel helper chains.  
- `anticipationPrediction` re-exports math for compatibility; prediction logic remains the single source for Moment selection.

---

## 5. Preserved contracts

- Public experience barrel APIs  
- Persistence schemas / storage keys  
- Evidence schemas  
- Replay determinism  
- Certification pipeline  
- Resolver stage count (pack → evolution → presence → anticipation → presentation)

Compatibility verified by the full existing suite plus simplification tests.

---

## 6. Compatibility verification

| Check | Result |
|---|---|
| `pnpm typecheck` | Required green |
| `pnpm test` | Required green |
| Replay determinism suites | Unchanged expectations except documented stability damping |
| Governance / integrity authority chain | Extended `62 → 63` |

---

## 7. Authority linkage

```
40 → … → 62 → 63
```

No new resolver stages. No new governance systems. No new cognitive abstractions.
