# 58 — Workspace Memory Evolution

Status: Complete (Sprint 67)  
Date: 2026-08-03  
Branch: `v2-dev`  
Baseline: Sprint 66 commit `2432f37`  
Authority: `architecture/57_Real_World_Adaptation_Validation.md` · Version 2 docs `40`–`57`  
Implementation: `app/src/experience/workspaceMemoryEvolution.ts`

---

## 1. Objective

Enable Workspace to evolve presentation of the user's environment over time using only certified adaptations backed by accumulated evidence.

Not AI autonomy. Nothing changes without explicit developer activation and existing governance.

No Runtime Core changes. No persistence schema changes. No navigation changes.

Evidence only — no recommendations.

---

## 2. Evolution schema

Type: `WorkspaceMemoryEvolution` (derived — not persisted)

| Field | Content |
|---|---|
| `evolutionId` | Content-addressed id (`evo-…`) |
| `affectedWorkspaceRegions` | Union of member `targetComponents` |
| `originatingAdaptationIds` | Certified adaptation ids |
| `evidenceLineage` | Evidence snapshot ids + tip |
| `certificationLineage` | Certification ids + tip |
| `architectureSnapshotIds` | Architecture snapshot ids from members |
| `evolutionEpoch` | Certification ordinal or pack version offset |
| `presentationDelta` | Density / spacing / emphasis / grouping / motion / environmentalWeight |
| `active` | `true` only when validation passes |
| `validation` | Failure reasons, composition result, stability, replay ids |
| `packId` / `packVersion` | Active pack identity when pack-sourced |

Does not duplicate runtime state. Does not store user content.

---

## 3. Resolver ordering

```
Runtime State
    ↓
Active Adaptation Pack
    ↓
WorkspaceMemoryEvolution
    ↓
Presentation
```

Function: `resolvePresentationFromRuntime(store)`

1. Derive active evolution from pack (if any) else certified active adaptations.  
2. When `evolution.active` — apply `presentationDelta` onto identity; `appliedAdaptationIds` = originating ids.  
3. When evolution missing/inactive — fall back to `resolvePresentationConfiguration(listAdaptations(store))`.

Live hook: `useResolvedPresentation` uses the runtime resolver.

---

## 4. Lineage

Every active evolution references:

- Certified adaptation(s) with clear, governance-valid, integrity-valid certifications  
- Evidence snapshot ids (member + certification)  
- Architecture snapshot ids  
- Replay session ids from adaptation validation contracts  

Pack pathway prefers `getActiveAdaptationPack` members and recorded `certificationIds`.

---

## 5. Validation

`validateMembers` / `validateMemoryEvolution` checks:

| Check | Failure |
|---|---|
| Certified members present | `no_certified_adaptations` |
| Clear covering certification | `missing_certification` / `certification_regressed` |
| Evidence / architecture / replay refs | `missing_evidence` / `missing_architecture_snapshot` / `missing_replay` |
| Composition + lineage | `composition_regressed` / `lineage_incomplete` |
| Stability (when scores exist) | `composition_unstable` |
| Member state | `member_not_active` / `member_not_passed` |

Invalid evolution remains `active: false` and does not influence presentation.

Deterministic replay: `replayMemoryEvolution(store)` — identical store ⇒ identical `evolutionId` and presentation.

---

## 6. Presentation boundaries

Evolution may influence only:

- spatial emphasis (`emphasisScale`)  
- grouping (`groupingTightness`)  
- environmental weighting (`environmentalWeight`)  
- spacing (`spacingScale`)  
- density  
- motion (`motionProfile`)

Evolution must never modify:

- user data  
- navigation  
- Runtime Core  
- persistence schemas  
- domain objects  

Shell mapping remains `presentationToShellStyle` (CSS custom properties only).

---

## 7. Development tooling

DEV overlay lazy-loads `workspaceMemoryEvolution` and surfaces evolution history, active evolution, originating adaptations, evidence / replay lineage, and presentation delta.

---

## 8. Authority linkage

```
40 → … → 57 → 58
```

No new governance / evidence / certification frameworks. Evolution reuses WorkspaceAdaptation, Certified Packs, AdaptationComposition, ExperienceEvidence, and governance primitives.
