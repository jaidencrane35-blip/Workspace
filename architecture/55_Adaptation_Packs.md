# 55 — Adaptation Packs

Status: Complete (Sprint 64)  
Date: 2026-08-03  
Branch: `v2-dev`  
Baseline: Sprint 63 commit `0a01ae0`  
Authority: `architecture/54_Adaptation_Certification.md` · Version 2 docs `40`–`54`  
Implementation: `app/src/experience/adaptationPacks.ts`

---

## 1. Objective

Move from individual adaptations to curated, certified adaptation packs.

A pack is a deterministic collection of compatible, certified adaptations activated together. Evidence only — no recommendations.

---

## 2. Pack schema

Type: `WorkspaceAdaptationPack` (`schemaVersion: 1`)  
Storage: `ws.experience.adaptation.packs.v1`

| Field | Meaning |
|---|---|
| `packId` | `pack-<fnv1a(sorted adaptation ids)>` |
| `version` | Monotonic per `packId` (immutable versions) |
| `adaptationIds` | Sorted adaptation id references |
| `certificationIds` | Covering `AdaptationCertification` ids |
| `compositionHash` | Hash of composition order + resolved presentation digest + conflict count |
| `stabilitySummary` | `composedStabilityScore`, `adaptationCount` |
| `evidenceSummary` | Evidence snapshot ids from certifications + tip id |
| `rolloutStatus` | `certified` (stored packs only) |

Active pack pointer: bundle `activePackId` = `packId@version` (does not mutate pack records).

No duplicated adaptation / certification payloads — identifiers only.

---

## 3. Certification rules

Function: `certifyAdaptationPack(store, { adaptationIds?, now? })`

Default adaptation set: latest certification’s `adaptationIds`, else all passed adaptations.

Succeeds only when:

| Gate | Condition |
|---|---|
| Non-empty set | ≥1 adaptation id |
| Members exist | Every id resolves to a `WorkspaceAdaptation` |
| Passed validation | `validationResult === passed` |
| Certified | Covered by a clear, governance-valid, integrity-valid certification |
| Governance lineage | Proposal, engineering, architecture, replay refs present |
| Composition | `composePackCandidate` yields full composition order |
| Conflicts | Every conflict has an explicit resolution strategy |
| Integrity | Architecture integrity graph valid |
| Uniqueness | Not a duplicate of latest version with identical `compositionHash` |

On failure: sorted allowlisted reasons; **write nothing**.  
On success: append immutable pack version.

Failure reasons:

`empty_adaptation_set` · `adaptation_not_found` · `adaptation_not_certified` · `adaptation_not_passed` · `composition_invalid` · `unresolved_conflicts` · `governance_incomplete` · `integrity_invalid` · `duplicate_pack`

---

## 4. Activation model

| Pathway | API |
|---|---|
| Single adaptation | Existing `activateAdaptation` |
| Certified pack | `activateAdaptationPack` → calls `activateAdaptation` per member |

Pack activation:

1. Preflight all members (no partial activation).
2. Deactivate active adaptations outside the pack (`deactivateAdaptation`).
3. Activate each pack member via `activateAdaptation`.
4. Set `activePackId`.

Deactivation: `deactivateAdaptationPack` → `deactivateAdaptation` per active member; clear pointer.

Manual only. No second activation mechanism. Resolver unchanged (`resolvePresentationConfiguration` / composition).

---

## 5. Composition guarantees

Pack composition uses the same deterministic rules as Sprint 62:

- Order by `adaptationId` ascending
- Explicit conflict strategies retained
- Composition hash frozen at pack certification

`packActivationReady` requires certified status, members present, passed validation, not rolled back.

---

## 6. Governance linkage

Reuses:

- `WorkspaceAdaptation`
- `AdaptationComposition` / `composeAdaptations`
- `AdaptationCertification`
- `EngineeringChangeRecord` / proposal / architecture ids via member lineage
- `ArchitectureSnapshot` integrity check

Authority chain:

```
40 → … → 54 → 55
```

No new adaptation primitives. No new lifecycle states on adaptations. Production Runtime unchanged.
