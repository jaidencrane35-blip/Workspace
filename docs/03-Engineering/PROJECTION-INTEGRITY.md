# Projection Integrity & Immutable Evidence Consumption

| Field | Value |
|-------|-------|
| **Purpose** | Protect consumption of protected lifecycle evidence |
| **Owner** | Platform Kernel |
| **Status** | Active |

## Principle

Domain state remains canonical. Projections and React surfaces may **display**
lifecycle evidence but must not invent, mutate, or erase it.

| Layer | Responsibility |
|-------|----------------|
| Services / domain | Lifecycle authority and continuity writes |
| Repositories | Persistence guards for transitions and immutable identity |
| Summaries / IPC | Project canonical state; actionable surfaces exclude terminals without dropping history |
| React | Projection-only rendering; no lifecycle ownership |

## Shared projection contract (RE / DQ / DE)

All three surfaces share the same consumption contract:

| Channel | Contents | Actionable? |
|---------|----------|-------------|
| Actionable projection | RE `top_candidates`; DQ `items`; DE `top_candidates` / actionable `candidates` | Yes |
| Historical projection | `history` + **`history_count`** | Never |

Rules:

- **`history_count` is evidence authority.** `history.length` is only the returned
  window (especially under `summary(limit)`). Empty actionable ≠ no evidence.
- **Terminal evidence ≠ actionable candidate.** Terminal records must not appear
  beside history in actionable collections.
- **History is a non-actionable DTO.** No execute, mutate, or transition methods.
- **React is projection-only.** No lifecycle ownership and no history action paths.

## Identity vocabulary

| Kind | Meaning | Examples |
|------|---------|----------|
| **Source identity** | Owning subsystem key for a live decision | DQ `source_type` + `source_id`; DE `candidate_key` |
| **Overlay identity** | Lifecycle row for presentation continuity | DQ `DecisionOverlayHistoryEntry`; DE `DecisionArtifactHistoryEntry` |
| **Outcome identity** | Recommendation Engine terminal outcome evidence | `RecommendationHistoryEntry.outcome.outcome_id` |

Do not conflate RE outcome identity with DQ/DE overlay/artifact identity.

## Surface-specific retention

1. **Decision Queue orphans retain overlays.** Missing live sources expire open
   overlays (`Expired`) and keep already-terminal overlays. No DQ overlay
   deletion API.
2. **Decision Engine terminal artifacts project into history.** Outcomes
   `selected` / `dismissed` / `expired` appear only in `history`. Actionable
   `candidates` and `top_candidates` remain `open` / `postponed` only. No DE
   overlay deletion API — retention is transition-based.
3. **Orphan DE evidence may have reduced provenance.** When only an overlay row
   remains, history uses `origin = unknown` and does **not** invent
   `native` / `recommendation_intake`. Live-candidate history preserves known
   origin and durable provenance fields.
4. **Recommendation Engine** carries terminal continuity via
   `history` / `history_count` with outcome identity on entries.

## Frontend responsibilities

- Prefer projected actionable lists (`top_candidates` / DQ `items`).
- Use `history_count` for evidence presence; never infer absence from empty
  actionable lists or truncated `history` arrays.
- History helpers only verify projected non-actionability — they do not invent
  lifecycle or provenance.

## Related

- [Persistence Boundary Governance](./PERSISTENCE-BOUNDARY-GOVERNANCE.md)
- [Lifecycle Governance](./LIFECYCLE-GOVERNANCE.md)
- [Workspace Experience Contract](../05-AI/WORKSPACE-EXPERIENCE-CONTRACT.md)
