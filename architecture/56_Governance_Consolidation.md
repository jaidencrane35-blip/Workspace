# 56 — Governance Consolidation

Status: Complete (Sprint 65)  
Date: 2026-08-03  
Branch: `v2-dev`  
Baseline: Sprint 64 commit `dbac229`  
Authority: `architecture/55_Adaptation_Packs.md` · Version 2 docs `40`–`55`  
Implementation: `app/src/dev/governanceStore.ts` · `app/src/dev/governancePrimitives.ts`

---

## 1. Objective

Reduce internal governance complexity while preserving every externally observable capability.

Consolidation sprint — no production behaviour changes, no new governance concepts. Evidence only — no recommendations.

---

## 2. Duplication inventory (pre-consolidation)

| Pattern | Sites (before) |
|---|---|
| Append-only JSON load/save/clear | certification, packs, (+ other domain bundles retained) |
| `integrityValid(store)` graph build | certification, production, packs (×3 identical) |
| Lineage refs present (proposal/eng/arch/replay) | composition, packs, production |
| Tip-of-list helpers | certification, operations, packs |
| Payload hashing via `fnv1a(parts.join("|"))` | certification `hashAdaptationSet`, packs `compositionHash` |
| Id regex `^[a-z0-9_.:-]{1,96}$` | workspaceAdaptation (+ governance modules) |

---

## 3. Consolidated primitives

| Module | Role |
|---|---|
| `app/src/dev/governanceStore.ts` | Domain-free JSON persistence layer (no circular imports) |
| `app/src/dev/governancePrimitives.ts` | Integrity / lineage / tip / hash helpers |

| Primitive | Role |
|---|---|
| `loadJsonBundle` / `loadJsonBundleOrNull` / `saveJsonBundle` / `clearJsonKey` | Unified JSON persistence access; callers keep keys + schemas |
| `storeArchitectureIntegrityValid` | Single integrity graph build + validate |
| `adaptationLineageRefsPresent` / `buildLineageIdCatalogs` | Shared lineage presence check |
| `tipOf` / `tipEvidenceId` / `latestValidArchitectureSnapshotId` | Deterministic tip helpers |
| `stablePayloadHash` / `contentAddressedId` / `fnv1a` | Hash generation |
| `GOVERNANCE_ID_RE` / `GOVERNANCE_LABEL_RE` | Shared id/label validators |
| `cloneRecords` / `architectureSnapshotById` | Small shared utilities |

Storage host remains `ExperienceStoreAdapter` (`memoryStore` / `browserStore`). All development-only governance bundles route load/save/clear through `governanceStore`.

---

## 4. Removed duplication

| Removed local helper | Replaced by |
|---|---|
| Private `integrityValid` in certification / production / packs | `storeArchitectureIntegrityValid` |
| Local tip evidence helpers in certification / operations | `tipOf` |
| Pack/composition/production lineage boolean checks | `adaptationLineageRefsPresent` (+ catalogs) |
| Inline certification/packs JSON load/save/clear | `loadJsonBundle` / `saveJsonBundle` / `clearJsonKey` |
| Pack composition hash join | `stablePayloadHash` |
| Adaptation `ID_RE` literal | `GOVERNANCE_ID_RE` |

No obsolete top-level modules deleted — helpers were private; modules remain with public APIs intact.

---

## 5. Preserved public contracts

All public exports from:

- `app/src/experience/index.ts`
- certification / packs / composition / production / operations / adaptation APIs

remain available with identical signatures and behaviour.

---

## 6. Validation equivalence

Deterministic pathways unchanged:

- `certifyAdaptationSet` / `compareCertifications`
- `certifyAdaptationPack` / `activateAdaptationPack`
- `composeAdaptations` / `validateComposition`
- `runFirstProductionAdaptation`
- `buildAdaptationCatalog` / `runBatchValidation`

Equivalence covered by existing suite plus consolidation compatibility tests.

---

## 7. Storage compatibility

| Key | Schema |
|---|---|
| `ws.experience.adaptation.certification.v1` | Unchanged |
| `ws.experience.adaptation.packs.v1` | Unchanged |
| `ws.experience.adaptation.production.v1` | Unchanged |
| `ws.experience.adaptation.v1` | Unchanged |
| `ws.dev.experience.*` | Unchanged |

No migration. No format changes. No user-data migration.

---

## 8. Authority linkage

```
40 → … → 55 → 56
```

No new governance concepts. No new adaptation primitives. Production Runtime unchanged.
