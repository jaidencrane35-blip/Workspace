# 65 — Architectural Maintainability

Status: Complete (Sprint 74)  
Date: 2026-08-03  
Branch: `v2-dev`  
Baseline: Sprint 73 commit `3368705`  
Authority: `architecture/64_Continuous_Engineering_Certification.md` · Version 2 docs `40`–`64`  
Implementation: `app/src/dev/architecturalMaintainability.ts`

---

## 1. Objective

Establish deterministic maintainability metrics for the existing implementation.

Engineering sustainability measurement only. No Runtime Core, Experience, presentation, navigation, persistence, or behavioural changes.

Evidence only — no recommendations.

---

## 2. Maintainability schema

`MaintainabilityReport` (schemaVersion: 1, reportId: `arch-maintainability-v1`):

| Field | Contents |
|---|---|
| `moduleCount` | Count of inventoried experience + dev TypeScript modules |
| `dependencyFanOutTotal` | Sum of declared import edges |
| `dependencyDepthMax` | Longest acyclic import depth |
| `exportedSymbolCount` | Sum of `export` declarations |
| `duplicatedImplementation` | Clamp/round, variance, deprecated alias site counts (via complexity report) |
| `averageModuleSize` | Mean line count |
| `largestModules` | Top modules by lines |
| `validationCoverage` | Distinct validation methods vs invariant count |
| `invariantCoverage` | Scope / subsystem coverage of `ENGINEERING_INVARIANTS` |
| `subsystems` | Per-subsystem module / lines / exports |

No subjective scoring. Builder: `buildMaintainabilityReport()`.

---

## 3. Dependency model

`DependencyHealthReport` (schemaVersion: 1, reportId: `dependency-health-v1`):

| Field | Contents |
|---|---|
| `circularDependencyCount` / `circularDependencies` | Detected import cycles |
| `isolatedModules` | Modules with zero fan-in and zero fan-out |
| `highFanOutModules` | Fan-out ≥ threshold |
| `highFanInModules` | Fan-in ≥ threshold |
| `architecturalBoundaryCrossings` | Edges crossing `experience` ↔ `dev` |
| `subsystemOwnership` | Module ids grouped by owning subsystem |
| `edgeCount` | Total declared import edges |

Derived from `MODULE_INVENTORY` only. Builder: `buildDependencyHealthReport()`.

---

## 4. Trend model

`compareMaintainabilityTrend()` compares current metrics against `MAINTAINABILITY_BASELINE` (Sprint 73 certification tip):

| Field | Meaning |
|---|---|
| `modulesAdded` / `modulesRemoved` | Inventory id deltas |
| `dependencyIncreases` / `dependencyReductions` | Absolute edge-count deltas |
| `invariantCoverageDelta` | Percent point change |
| `validationCoverageDelta` | Percent point change |

Engineering metrics only — no implementation diff.

---

## 5. Subsystem ownership

Reuses `EngineeringSubsystem` from Continuous Engineering Certification:

`authority` · `governance` · `evidence` · `adaptation` · `presentation` · `certification` · `complexity` · `integrity`

Each inventoried module declares one owning subsystem.

---

## 6. Authority linkage

```
40 → … → 64 → 65
```

Reuse:

- `ArchitectureGraph` / `AUTHORITY_CHAIN`
- `ENGINEERING_INVARIANTS` / engineering certification coverage counts
- `buildArchitecturalComplexityReport()` for duplication sites

No duplicated validators. No duplicated persistence. No public API changes. No new persistence keys.

---

## 7. Development tooling

DEV overlay (lazy-loaded `architecturalMaintainability`):

- Maintainability report
- Dependency health
- Trend comparison
- Subsystem ownership
- Implementation metrics

Derived only. Lazy-load only.
