# 47 — Architectural Integrity

Status: Complete (Sprint 56)  
Date: 2026-08-03  
Branch: `v2-dev`  
Baseline: Sprint 55 commit `2a9230a`  
Authority: `architecture/46_Engineering_Governance.md` · Version 2 docs `40`–`46`  
Implementation: `app/src/dev/architecturalIntegrity.ts` · DEV overlay integrity panel

---

## 1. Objective

Provide a deterministic architectural integrity system that validates Workspace structure against declared authorities as the architecture evolves.

Development tooling only. No production Experience changes, no Runtime Core changes, no AI autonomy. Evidence only — no recommendations.

---

## 2. Graph schema

Type: `ArchitectureGraph` (`schemaVersion: 1`)  
Builder: `buildArchitectureGraph(source)`

| Field | Meaning |
|---|---|
| `nodes` | Sorted `ArchitectureNode[]` |
| `edges` | Sorted `ArchitectureEdge[]` |

Source artefacts (identifier reuse only):

- architecture authority documents (`ARCHITECTURE_AUTHORITY_DOCS`, 40–47)
- `EngineeringChangeRecord`
- `ExperienceChangeProposal`
- `ExperienceOpportunity`
- `ExperienceEvidence`

Edges are created only from explicit fields and the declared `AUTHORITY_CHAIN`. No inferred links.

---

## 3. Node taxonomy

| Kind | Stable id form | Origin |
|---|---|---|
| `architecture_document` | `doc:<filename>` | Authority allowlist |
| `engineering_record` | `eng-…` (existing change id) | Engineering governance |
| `proposal` | `prop-…` | Change proposals |
| `opportunity` | `opp-…` | Opportunity detector |
| `evidence` | `evd-…` | Evidence snapshots |
| `replay_session` | `replay:<sessionId>` | Proposal / opportunity replay refs |
| `commit` | `commit:<sha>` | Engineering record commits |
| `module` | `mod:<path>` | Engineering affected modules |

Evidence and replay nodes are materialised only when explicitly referenced (prevents isolate orphans).

---

## 4. Edge taxonomy

| Type | From → To | Declared by |
|---|---|---|
| `authority_precedes` | doc → doc | `AUTHORITY_CHAIN` constant |
| `references_authority` | engineering → doc | `architectureDocuments` |
| `originates_from_proposal` | engineering → proposal | `proposalIds` |
| `originates_from_opportunity` | proposal → opportunity | `opportunityIds` |
| `supported_by_evidence` | proposal/opportunity → evidence | evidence id lists |
| `replays` | proposal/opportunity → replay | `replaySessionIds` |
| `implemented_by_commit` | engineering → commit | `commits` |
| `affects_module` | engineering → module | `affectedModules` |

Edge id: `edge:<fnv1a(type\|from\|to)>`.

Declared authority chain:

```
40 → 41 → 42 → 43 → 44 → 45 → 46 → 47
```

Authority root: `40_Experience_Refoundation.md`.

---

## 5. Integrity rules

Function: `validateArchitectureIntegrity(graph, source?)`

| Code | Condition |
|---|---|
| `duplicate_node_id` | Same node id appears more than once |
| `dangling_reference` | Edge endpoint missing from node set |
| `orphan_node` | Node degree = 0 |
| `authority_cycle` | Cycle in `authority_precedes` subgraph |
| `invalid_engineering_authority` | Engineering node lacks `references_authority` edge |
| `proposal_missing_evidence_lineage` | Proposal lacks `supported_by_evidence` edge |
| `released_missing_implementation` | Released engineering lacks commit or module edge |

Result: `{ valid, violations, orphanNodeIds, danglingEdgeIds }`.  
Invalid graphs report `valid: false` (rejected for certification / snapshot readiness signalling).

---

## 6. Snapshot format

Type: `ArchitectureSnapshot` (`schemaVersion: 1`)  
Storage: `ws.dev.experience.architecture.v1` (append-only)

| Field | Meaning |
|---|---|
| `snapshotId` | Content-addressed `asnap-<hash>` |
| `graphHash` | FNV-1a of canonical nodes+edges JSON |
| `nodeCount` / `edgeCount` | Graph size |
| `integrity` | Full integrity result copy |
| `t` | Epoch ms (caller-supplied) |
| `authorityRoot` | `40_Experience_Refoundation.md` |
| `releaseLineage` | Sorted released engineering change ids |

Prior snapshots are never mutated (`persistArchitectureSnapshot` append-only; `assertSnapshotsImmutable`).

---

## 7. Validation workflow

1. Collect governance artefacts (evidence, opportunities, proposals, engineering records).
2. `buildArchitectureGraph` → canonical graph.
3. `validateArchitectureIntegrity` → accept / reject.
4. Optional: explore authority successors and outbound dependencies (DEV overlay).
5. `createArchitectureSnapshot` + `persistArchitectureSnapshot` → immutable record.

Production bundles omit the overlay via existing `import.meta.env.DEV` dynamic import. Integrity module loads only when the DEV overlay opens.

---

## 8. Privacy / boundary

| Guarantee | Mechanism |
|---|---|
| Zero user content | Ids, enums, path tokens, numbers only |
| Identifier reuse | No re-minting of proposal/evidence/opportunity/replay ids |
| Local-only | Separate localStorage key; no network APIs in `app/src/dev` |
| DEV-only | Lazy-loaded under Experience Validation gate |
| No Runtime / production Experience changes | Out of scope |
