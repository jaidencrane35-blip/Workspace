# 60 — Workspace Anticipation

Status: Complete (Sprint 69)  
Date: 2026-08-03  
Branch: `v2-dev`  
Baseline: Sprint 68 commit `52d493a`  
Authority: `architecture/59_Workspace_Presence.md` · Version 2 docs `40`–`59`  
Implementation: `app/src/experience/workspaceAnticipation.ts`

---

## 1. Objective

Allow Workspace to anticipate the user's next likely interaction without taking autonomous action.

The system predicts. The user decides. Nothing executes automatically.

No Runtime Core changes. No persistence schema changes. No navigation changes.  
No AI decision-making. Presentation continues through the certified adaptation pipeline.

Evidence only — no recommendations.

---

## 2. Anticipation schema

Type: `WorkspaceAnticipation` (derived — not persisted)

| Field | Content |
|---|---|
| `anticipationId` | Content-addressed id (`anticipate-…`) |
| `likelyFocalRegion` | Allowlisted adaptation target region |
| `likelyNextMoment` | Allowlisted Moment destination |
| `likelyContinuationTarget` | Allowlisted continuation Moment |
| `confidence` | 0–1 deterministic score from evidence |
| `evidenceLineage` | Evidence snapshot ids + tip |
| `replayLineage` | Replay session ids + bundle replay invocations |
| `architectureSnapshotIds` | Architecture snapshots from evolution |
| `contributingAdaptationIds` | Certified adaptation ids |
| `certificationIds` | Certification lineage |
| `presenceId` / `evolutionId` | Upstream presentation lineage |
| `readiness` | Subtle emphasis, env weighting, pre-attentive focus, object readiness, motion preparation |
| `active` | `true` only when validation passes |
| `validation` | Failure reasons + composition status |

Derived only from Runtime State, Memory Evolution, Presence, ExperienceEvidence, and Replay history.  
No user content. No AI-generated memories. No persistence.

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
Workspace Anticipation
    ↓
Presentation
```

- Intermediate: `resolvePresentationWithPresence`  
- Final: `resolvePresentationWithAnticipation`  
- Live hook: `useResolvedPresentation`  

Inactive anticipation ⇒ presence / evolution / adaptation fallback.

---

## 4. Prediction boundaries

| May influence | Must never |
|---|---|
| Subtle emphasis | Move objects |
| Environmental weighting | Execute actions |
| Pre-attentive focus (informational) | Change navigation |
| Object readiness (informational) | Alter Runtime behaviour |
| Motion preparation (informational) | Modify persistence |

Prediction sources (deterministic):

- Tip evidence hotspot / `topHesitationDestination` → likely next Moment  
- Continue vs save success totals → continuation target  
- Session count, replay divergence, hotspot density → confidence  

---

## 5. Validation

| Check | Failure |
|---|---|
| Evidence present | `no_evidence` |
| Certified contributors | `no_certified_lineage` |
| Active presence | `presence_inactive` |
| Active evolution | `evolution_inactive` |
| Replay session ids | `missing_replay` |
| Architecture snapshots | `missing_architecture_snapshot` |
| Composition / lineage | `composition_regressed` / `lineage_incomplete` |
| Confidence floor | `confidence_unstable` |
| Contributors ⊆ evolution | `prediction_bypass_attempt` |

Invalid anticipation remains `active: false`.  
Every active result references evidence, replay, adaptation lineage, and architecture snapshot.

Deterministic replay: `replayWorkspaceAnticipation(store)`.

---

## 6. Governance linkage

Reuses WorkspacePresence, WorkspaceMemoryEvolution, ExperienceEvidence, Replay history, Certified Packs, AdaptationComposition, governance primitives.

No new governance systems. No new adaptation abstractions. No autonomous actions.

Authority:

```
40 → … → 59 → 60
```
